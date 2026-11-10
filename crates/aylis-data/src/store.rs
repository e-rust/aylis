//! Armazenamento em colunas dos registros que chegam do backend.
//!
//! Cada campo vira uma coluna tipada: inteiros (4 bytes enquanto couberem, depois 8) e
//! números em vetores do tipo com os nulos num mapa de bits, decimais e datas em vetores
//! do tipo, lógicos num byte, e textos com **dicionário** (cada texto diferente é guardado
//! uma vez; o registro guarda um número de 4 bytes). Uma lista de clientes com a cidade
//! repetida em milhares de linhas guarda "São Paulo" uma vez só.
//!
//! Só os registros carregados ficam aqui. Um registro alterado ou incluído passa a ter os
//! valores próprios (ver `RowData`), então editar não mexe nas colunas.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

use crate::value::{Date, Decimal, Value};

const NULL: u32 = u32::MAX;

/// Hash rápido para o dicionário dos textos (a carga procura cada texto de cada registro;
/// o SipHash padrão protege contra ataques que aqui não fazem sentido).
#[derive(Default)]
struct FxHasher(u64);

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        const K: u64 = 0x51_7c_c1_b7_27_22_0a_95;
        let mut chunks = bytes.chunks_exact(8);
        for c in &mut chunks {
            self.0 = (self.0.rotate_left(5) ^ u64::from_le_bytes(c.try_into().unwrap_or_default())).wrapping_mul(K);
        }
        for &b in chunks.remainder() {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(K);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

type FastHash = BuildHasherDefault<FxHasher>;

/// Um vetor em blocos de tamanho fixo: crescer não realoca nem copia o que já está
/// guardado (com milhões de registros, dobrar um vetor de 100 colunas trava a carga e
/// deixa metade da memória reservada à toa).
#[derive(Clone, Debug)]
struct Chunked<T> {
    chunks: Vec<Vec<T>>,
    len: usize,
}

const CHUNK: usize = 1 << 16;

impl<T> Default for Chunked<T> {
    fn default() -> Self {
        Self { chunks: Vec::new(), len: 0 }
    }
}

impl<T: Copy> Chunked<T> {
    /// `n` vezes `value`.
    fn filled(value: T, n: usize) -> Self {
        let mut c = Self::default();
        for _ in 0..n {
            c.push(value);
        }
        c
    }

    fn push(&mut self, value: T) {
        // O primeiro bloco cresce aos poucos (tabelas pequenas); os outros já nascem cheios.
        match self.chunks.last_mut() {
            Some(last) if last.len() < CHUNK => last.push(value),
            _ => {
                let mut chunk = Vec::with_capacity(if self.chunks.is_empty() { 0 } else { CHUNK });
                chunk.push(value);
                self.chunks.push(chunk);
            }
        }
        self.len += 1;
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.chunks.get(i / CHUNK)?.get(i % CHUNK)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.chunks.iter().flatten()
    }

    /// Junta `other` no fim. Com este cheio até a borda de um bloco, os blocos de lá
    /// passam para cá sem copiar; senão, valor por valor.
    fn append(&mut self, other: Chunked<T>) {
        if self.len % CHUNK == 0 {
            self.len += other.len;
            self.chunks.extend(other.chunks);
        } else {
            for chunk in other.chunks {
                for v in chunk {
                    self.push(v);
                }
            }
        }
    }
}

impl<T: Copy> FromIterator<T> for Chunked<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = Self::default();
        for v in iter {
            c.push(v);
        }
        c
    }
}

/// Quais posições são nulas (vazio enquanto não aparece nenhum nulo).
#[derive(Clone, Debug, Default)]
struct Nulls(Vec<u64>);

impl Nulls {
    fn set(&mut self, row: usize) {
        let word = row / 64;
        if self.0.len() <= word {
            self.0.resize(word + 1, 0);
        }
        self.0[word] |= 1 << (row % 64);
    }

    fn is(&self, row: usize) -> bool {
        self.0.get(row / 64).is_some_and(|w| (w >> (row % 64)) & 1 == 1)
    }

    /// As `row` primeiras posições nulas (a coluna ganhou tipo depois de só nulos).
    fn first(row: usize) -> Self {
        let mut n = Self::default();
        for r in 0..row {
            n.set(r);
        }
        n
    }

    fn bytes(&self) -> usize {
        self.0.len() * 8
    }

    /// Os nulos de outro pedaço que começa na posição `base`.
    fn append(&mut self, other: &Nulls, base: usize, len: usize) {
        if other.0.is_empty() {
            return;
        }
        if base % 64 == 0 {
            // Alinhado: as palavras de lá vêm inteiras.
            self.0.resize(base / 64, 0);
            self.0.extend_from_slice(&other.0);
        } else {
            for r in 0..len {
                if other.is(r) {
                    self.set(base + r);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Column {
    /// Só nulos até agora (ou campos calculados).
    Empty,
    Bool(Vec<Option<bool>>),
    /// Inteiros que cabem em 32 bits (passa a `Int` no primeiro que não cabe).
    Int32 { values: Chunked<i32>, nulls: Nulls },
    Int { values: Chunked<i64>, nulls: Nulls },
    Float { values: Chunked<f64>, nulls: Nulls },
    Decimal(Vec<Option<Decimal>>),
    Date(Vec<Option<Date>>),
    /// Texto com dicionário: `codes[i]` aponta para `words` (`NULL` = nulo).
    Text { words: Vec<Box<str>>, index: HashMap<Box<str>, u32, FastHash>, codes: Chunked<u32> },
    /// Qualquer outro tipo (hora, data e hora, blob) ou colunas com tipos misturados.
    Any(Vec<Value>),
}

/// As colunas de todos os registros carregados.
#[derive(Clone, Debug, Default)]
pub(crate) struct ColumnStore {
    columns: Vec<Column>,
    len: usize,
}

impl ColumnStore {
    pub(crate) fn new(width: usize) -> Self {
        Self { columns: vec![Column::Empty; width], len: 0 }
    }

    pub(crate) fn width(&self) -> usize {
        self.columns.len()
    }

    /// Guarda um registro; devolve a posição dele.
    pub(crate) fn push(&mut self, values: Vec<Value>) -> u32 {
        let row = self.len;
        for (col, value) in values.into_iter().enumerate().take(self.columns.len()) {
            push_value(&mut self.columns[col], row, value);
        }
        // Colunas com menos valores (registro curto) recebem nulo.
        for column in &mut self.columns {
            if column_len(column) < row + 1 {
                push_value(column, row, Value::Null);
            }
        }
        self.len += 1;
        row as u32
    }

    /// Um valor.
    pub(crate) fn get(&self, row: u32, col: usize) -> Value {
        let r = row as usize;
        match self.columns.get(col) {
            None | Some(Column::Empty) => Value::Null,
            Some(Column::Bool(v)) => v.get(r).copied().flatten().map_or(Value::Null, Value::Bool),
            Some(Column::Int32 { values, nulls }) => match values.get(r) {
                Some(&v) if !nulls.is(r) => Value::Int(v as i64),
                _ => Value::Null,
            },
            Some(Column::Int { values, nulls }) => match values.get(r) {
                Some(&v) if !nulls.is(r) => Value::Int(v),
                _ => Value::Null,
            },
            Some(Column::Float { values, nulls }) => match values.get(r) {
                Some(&v) if !nulls.is(r) => Value::Float(v),
                _ => Value::Null,
            },
            Some(Column::Decimal(v)) => v.get(r).copied().flatten().map_or(Value::Null, Value::Decimal),
            Some(Column::Date(v)) => v.get(r).copied().flatten().map_or(Value::Null, Value::Date),
            Some(Column::Text { words, codes, .. }) => match codes.get(r) {
                Some(&c) if c != NULL => Value::String(words[c as usize].to_string()),
                _ => Value::Null,
            },
            Some(Column::Any(v)) => v.get(r).cloned().unwrap_or_default(),
        }
    }

    /// A ordem de cada texto do dicionário da coluna `col`, sem diferença de maiúsculas
    /// (`None` se a coluna não é de texto). Ordenar milhões de registros por um texto vira
    /// ordenar números.
    pub(crate) fn text_ranks(&self, col: usize) -> Option<Vec<u32>> {
        let Some(Column::Text { words, .. }) = self.columns.get(col) else { return None };
        let lower: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
        let mut order: Vec<u32> = (0..words.len() as u32).collect();
        order.sort_by(|&a, &b| lower[a as usize].cmp(&lower[b as usize]));
        let mut ranks = vec![0; words.len()];
        let mut rank = 0;
        for (i, &code) in order.iter().enumerate() {
            if i > 0 && lower[code as usize] != lower[order[i - 1] as usize] {
                rank = i as u32;
            }
            ranks[code as usize] = rank;
        }
        Some(ranks)
    }

    /// A chave de ordenação de um valor guardado (`ranks`: de [`text_ranks`](Self::text_ranks)).
    pub(crate) fn sort_key(&self, row: u32, col: usize, ranks: Option<&[u32]>) -> SortKey {
        let r = row as usize;
        match (self.columns.get(col), ranks) {
            (Some(Column::Text { codes, .. }), Some(ranks)) => match codes.get(r) {
                Some(&code) if code != NULL => SortKey::Text { rank: ranks[code as usize], code },
                _ => SortKey::Null,
            },
            (Some(Column::Int32 { values, nulls }), _) if !nulls.is(r) => values.get(r).map_or(SortKey::Null, |&v| SortKey::Int(v as i64)),
            (Some(Column::Int { values, nulls }), _) if !nulls.is(r) => values.get(r).map_or(SortKey::Null, |&v| SortKey::Int(v)),
            (Some(Column::Float { values, nulls }), _) if !nulls.is(r) => values.get(r).map_or(SortKey::Null, |&v| SortKey::Float(v)),
            _ => SortKey::of(self.get(row, col)),
        }
    }

    /// O texto do dicionário da coluna `col` (para comparar com valores fora dele).
    pub(crate) fn word(&self, col: usize, code: u32) -> &str {
        match self.columns.get(col) {
            Some(Column::Text { words, .. }) => words.get(code as usize).map_or("", |w| w),
            _ => "",
        }
    }

    /// O registro inteiro.
    pub(crate) fn row(&self, row: u32) -> Vec<Value> {
        (0..self.columns.len()).map(|c| self.get(row, c)).collect()
    }

    /// Memória aproximada das colunas, em bytes (para conferir a economia).
    pub fn approx_bytes(&self) -> usize {
        self.columns
            .iter()
            .map(|c| match c {
                Column::Empty => 0,
                Column::Bool(v) => v.len() * 2,
                Column::Int32 { values, nulls } => values.len() * 4 + nulls.bytes(),
                Column::Int { values, nulls } => values.len() * 8 + nulls.bytes(),
                Column::Float { values, nulls } => values.len() * 8 + nulls.bytes(),
                Column::Decimal(v) => v.len() * std::mem::size_of::<Option<Decimal>>(),
                Column::Date(v) => v.len() * std::mem::size_of::<Option<Date>>(),
                Column::Text { words, codes, .. } => codes.len() * 4 + words.iter().map(|w| w.len() * 2 + 32).sum::<usize>(),
                Column::Any(v) => v.len() * std::mem::size_of::<Value>(),
            })
            .sum()
    }
}

impl ColumnStore {
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    /// As colunas de dados nas posições dos campos (`data[i]`: o campo `i` é de dados);
    /// os outros (calculados, lookup) ficam vazios.
    pub(crate) fn spread(self, data: &[bool]) -> Self {
        let mut given = self.columns.into_iter();
        let columns = data.iter().map(|&d| if d { given.next().unwrap_or(Column::Empty) } else { Column::Empty }).collect();
        Self { columns, len: self.len }
    }

    /// Acrescenta os registros de `other` (as mesmas colunas). Números e textos vêm sem
    /// passar por `Value`; com este cheio até a borda de um bloco ([`RowBatch::PART`]), os
    /// blocos de lá passam para cá sem copiar (os textos só trocam o código para o do
    /// dicionário daqui).
    pub(crate) fn append(&mut self, other: ColumnStore) {
        let base = self.len;
        let len = other.len;
        for (mine, theirs) in self.columns.iter_mut().zip(other.columns) {
            match (&mut *mine, theirs) {
                (_, Column::Empty) => {
                    // Só nulos lá: a coluna daqui (se já tem tipo) recebe os nulos.
                    if !matches!(mine, Column::Empty) {
                        for r in 0..len {
                            push_value(mine, base + r, Value::Null);
                        }
                    }
                }
                (Column::Int32 { values, nulls }, Column::Int32 { values: v, nulls: n }) => {
                    nulls.append(&n, base, len);
                    values.append(v);
                }
                (Column::Int { values, nulls }, Column::Int { values: v, nulls: n }) => {
                    nulls.append(&n, base, len);
                    values.append(v);
                }
                (Column::Float { values, nulls }, Column::Float { values: v, nulls: n }) => {
                    nulls.append(&n, base, len);
                    values.append(v);
                }
                (Column::Text { words, index, codes }, Column::Text { words: w, codes: mut k, .. }) => {
                    // O código de cada texto de lá no dicionário daqui.
                    let map: Vec<u32> = w
                        .into_iter()
                        .map(|word| match index.get(&*word) {
                            Some(&c) => c,
                            None => {
                                let c = words.len() as u32;
                                index.insert(word.clone(), c);
                                words.push(word);
                                c
                            }
                        })
                        .collect();
                    for chunk in &mut k.chunks {
                        for code in chunk.iter_mut() {
                            if *code != NULL {
                                *code = map[*code as usize];
                            }
                        }
                    }
                    codes.append(k);
                }
                // Tipos diferentes (ou a coluna daqui ainda vazia): valor por valor.
                (column, theirs) => {
                    let single = ColumnStore { columns: vec![theirs], len };
                    for r in 0..len {
                        push_value(column, base + r, single.get(r as u32, 0));
                    }
                }
            }
        }
        self.len += len;
    }
}

/// Registros montados fora da interface (numa thread), para entrar num `ClientDataSet`
/// de uma vez com [`DataSet::load_batch`](crate::DataSet::load_batch): milhões de linhas
/// sem passar valor por valor pela tela nem refazer a ordem a cada pacote.
///
/// Cada linha é escrita célula a célula, na ordem dos campos de dados, e termina com
/// [`end_row`](Self::end_row). Os valores entram já no tipo do campo (não há conversão:
/// um texto num campo inteiro vira uma coluna de valores misturados). Lotes montados em
/// paralelo se juntam com [`append`](Self::append).
///
/// ```
/// use aylis_data::RowBatch;
/// let mut lote = RowBatch::new(2);
/// for i in 0..3 {
///     lote.int(i).text("São Paulo").end_row();
/// }
/// assert_eq!(lote.len(), 3);
/// ```
#[derive(Clone, Debug, Default)]
pub struct RowBatch {
    store: ColumnStore,
    /// A próxima célula da linha em andamento.
    col: usize,
}

impl RowBatch {
    /// Lotes montados em paralelo com um múltiplo deste número de registros (menos o
    /// último) se juntam sem copiar: os blocos de memória passam de um para o outro.
    pub const PART: usize = CHUNK;

    /// Um lote vazio com `width` colunas (os campos de dados, na ordem).
    pub fn new(width: usize) -> Self {
        Self { store: ColumnStore::new(width), col: 0 }
    }

    /// Quantos registros completos.
    pub fn len(&self) -> usize {
        self.store.len
    }

    pub fn is_empty(&self) -> bool {
        self.store.len == 0
    }

    pub fn width(&self) -> usize {
        self.store.columns.len()
    }

    fn put(&mut self, value: Value) -> &mut Self {
        if let Some(column) = self.store.columns.get_mut(self.col) {
            push_value(column, self.store.len, value);
            self.col += 1;
        }
        self
    }

    /// Um inteiro: direto na coluna (sem passar por `Value`) quando ela já é de inteiros.
    pub fn int(&mut self, v: i64) -> &mut Self {
        match self.store.columns.get_mut(self.col) {
            Some(Column::Int32 { values, .. }) if i32::try_from(v).is_ok() => values.push(v as i32),
            Some(Column::Int { values, .. }) => values.push(v),
            _ => return self.put(Value::Int(v)),
        }
        self.col += 1;
        self
    }

    /// Um número: direto na coluna quando ela já é de números.
    pub fn float(&mut self, v: f64) -> &mut Self {
        match self.store.columns.get_mut(self.col) {
            Some(Column::Float { values, .. }) => values.push(v),
            _ => return self.put(Value::Float(v)),
        }
        self.col += 1;
        self
    }

    pub fn bool(&mut self, v: bool) -> &mut Self {
        self.put(Value::Bool(v))
    }

    /// Um texto: se ele já está no dicionário da coluna, não copia nada.
    pub fn text(&mut self, s: &str) -> &mut Self {
        let row = self.store.len;
        let Some(column) = self.store.columns.get_mut(self.col) else { return self };
        match column {
            Column::Text { words, index, codes } => {
                let code = match index.get(s) {
                    Some(&c) => c,
                    None => {
                        let c = words.len() as u32;
                        let boxed: Box<str> = s.into();
                        index.insert(boxed.clone(), c);
                        words.push(boxed);
                        c
                    }
                };
                codes.push(code);
            }
            other => push_value(other, row, Value::String(s.to_string())),
        }
        self.col += 1;
        self
    }

    /// Qualquer valor (data, decimal...).
    pub fn value(&mut self, v: Value) -> &mut Self {
        self.put(v)
    }

    pub fn null(&mut self) -> &mut Self {
        self.put(Value::Null)
    }

    /// Fecha a linha (as células que faltaram ficam nulas).
    pub fn end_row(&mut self) {
        while self.col < self.store.columns.len() {
            self.null();
        }
        self.col = 0;
        self.store.len += 1;
    }

    /// Uma linha inteira de uma vez.
    pub fn push(&mut self, values: Vec<Value>) {
        for v in values {
            self.put(v);
        }
        self.end_row();
    }

    /// Junta os registros de outro lote (das mesmas colunas) no fim deste.
    pub fn append(&mut self, other: RowBatch) {
        self.store.append(other.store);
    }

    pub(crate) fn into_store(self) -> ColumnStore {
        self.store
    }
}

/// Chave de ordenação de um valor: números e posições no dicionário comparam sem copiar
/// textos; o resto, pelo valor.
#[derive(Clone, Debug)]
pub(crate) enum SortKey {
    Null,
    Int(i64),
    Float(f64),
    /// Texto do dicionário: a ordem dele e o código (para comparar com textos de fora).
    Text { rank: u32, code: u32 },
    Value(Value),
}

impl SortKey {
    pub(crate) fn of(value: Value) -> Self {
        match value {
            Value::Null => SortKey::Null,
            Value::Int(i) => SortKey::Int(i),
            Value::Float(f) => SortKey::Float(f),
            v => SortKey::Value(v),
        }
    }

    /// A ordem do índice (nulos antes), igual à de [`crate::expr::sort_order`].
    pub(crate) fn order(&self, other: &SortKey, store: &ColumnStore, col: usize) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        use SortKey::*;
        match (self, other) {
            (Null, Null) => Ordering::Equal,
            (Null, _) => Ordering::Less,
            (_, Null) => Ordering::Greater,
            (Int(a), Int(b)) => a.cmp(b),
            (Float(a), Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Int(a), Float(b)) => (*a as f64).partial_cmp(b).unwrap_or(Ordering::Equal),
            (Float(a), Int(b)) => a.partial_cmp(&(*b as f64)).unwrap_or(Ordering::Equal),
            (Text { rank: a, .. }, Text { rank: b, .. }) => a.cmp(b),
            (a, b) => crate::expr::sort_order(&a.value(store, col), &b.value(store, col)),
        }
    }

    /// A chave como um número que compara na mesma ordem, com a família (1 inteiro, 2
    /// número, 3 texto do dicionário; 0 nulo): chaves da mesma família ordenam só pelo
    /// número. `None` quando precisa da comparação completa.
    pub(crate) fn packed(&self) -> Option<(u8, u64)> {
        match self {
            SortKey::Null => Some((0, 0)),
            SortKey::Int(i) => Some((1, (*i as u64) ^ (1 << 63))),
            SortKey::Float(f) if !f.is_nan() => {
                let bits = f.to_bits();
                Some((2, if bits >> 63 == 1 { !bits } else { bits | (1 << 63) }))
            }
            SortKey::Text { rank, .. } => Some((3, *rank as u64)),
            _ => None,
        }
    }

    fn value(&self, store: &ColumnStore, col: usize) -> Value {
        match self {
            SortKey::Null => Value::Null,
            SortKey::Int(i) => Value::Int(*i),
            SortKey::Float(f) => Value::Float(*f),
            SortKey::Text { code, .. } => Value::String(store.word(col, *code).to_string()),
            SortKey::Value(v) => v.clone(),
        }
    }
}

fn column_len(column: &Column) -> usize {
    match column {
        Column::Empty => usize::MAX, // aceita qualquer posição
        Column::Bool(v) => v.len(),
        Column::Int32 { values, .. } => values.len(),
        Column::Int { values, .. } => values.len(),
        Column::Float { values, .. } => values.len(),
        Column::Decimal(v) => v.len(),
        Column::Date(v) => v.len(),
        Column::Text { codes, .. } => codes.len(),
        Column::Any(v) => v.len(),
    }
}

/// Acrescenta `value` na posição `row` (as anteriores já existem). Na primeira vez que
/// aparece um valor, a coluna escolhe o tipo; um valor de outro tipo depois converte a
/// coluna para `Any`.
fn push_value(column: &mut Column, row: usize, value: Value) {
    if let Column::Empty = column {
        if value.is_null() {
            return;
        }
        *column = match &value {
            Value::Bool(_) => Column::Bool(vec![None; row]),
            Value::Int(_) => Column::Int32 { values: Chunked::filled(0, row), nulls: Nulls::first(row) },
            Value::Float(_) => Column::Float { values: Chunked::filled(0.0, row), nulls: Nulls::first(row) },
            Value::Decimal(_) => Column::Decimal(vec![None; row]),
            Value::Date(_) => Column::Date(vec![None; row]),
            Value::String(_) => Column::Text { words: Vec::new(), index: HashMap::default(), codes: Chunked::filled(NULL, row) },
            _ => Column::Any(vec![Value::Null; row]),
        };
    }
    // Devolve o valor que não coube no tipo da coluna.
    let rejected: Option<Value> = match (&mut *column, value) {
        (Column::Bool(v), Value::Bool(b)) => {
            v.push(Some(b));
            None
        }
        (Column::Int32 { values, .. }, Value::Int(i)) if i32::try_from(i).is_ok() => {
            values.push(i as i32);
            None
        }
        (Column::Int32 { values, nulls }, Value::Int(i)) => {
            // Não cabe em 32 bits: a coluna passa a 64.
            let mut wide: Chunked<i64> = values.iter().map(|&v| v as i64).collect();
            wide.push(i);
            *column = Column::Int { values: wide, nulls: std::mem::take(nulls) };
            None
        }
        (Column::Int { values, .. }, Value::Int(i)) => {
            values.push(i);
            None
        }
        (Column::Float { values, .. }, Value::Float(f)) => {
            values.push(f);
            None
        }
        (Column::Decimal(v), Value::Decimal(d)) => {
            v.push(Some(d));
            None
        }
        (Column::Date(v), Value::Date(d)) => {
            v.push(Some(d));
            None
        }
        (Column::Text { words, index, codes }, Value::String(s)) => {
            let code = match index.get(s.as_str()) {
                Some(&c) => c,
                None => {
                    let c = words.len() as u32;
                    let boxed: Box<str> = s.into_boxed_str();
                    index.insert(boxed.clone(), c);
                    words.push(boxed);
                    c
                }
            };
            codes.push(code);
            None
        }
        (Column::Text { codes, .. }, Value::Null) => {
            codes.push(NULL);
            None
        }
        (Column::Any(v), value) => {
            v.push(value);
            None
        }
        (Column::Bool(v), Value::Null) => {
            v.push(None);
            None
        }
        (Column::Int32 { values, nulls }, Value::Null) => {
            nulls.set(values.len());
            values.push(0);
            None
        }
        (Column::Int { values, nulls }, Value::Null) => {
            nulls.set(values.len());
            values.push(0);
            None
        }
        (Column::Float { values, nulls }, Value::Null) => {
            nulls.set(values.len());
            values.push(0.0);
            None
        }
        (Column::Decimal(v), Value::Null) => {
            v.push(None);
            None
        }
        (Column::Date(v), Value::Null) => {
            v.push(None);
            None
        }
        (Column::Empty, _) => None,
        (_, other) => Some(other),
    };
    // Tipo diferente do da coluna: passa a guardar valores quaisquer.
    if let Some(other) = rejected {
        let len = column_len(column);
        let snapshot = ColumnStore { columns: vec![column.clone()], len };
        let mut all: Vec<Value> = (0..len).map(|r| snapshot.get(r as u32, 0)).collect();
        all.push(other);
        *column = Column::Any(all);
    }
}
