//! [`DataGrid`]: grade de registros virtualizada.

use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent,
    PaintCtx, PointF, PointerButton, PointerEvent, RectF, Stroke, TextAlign, Ui, WheelEvent, properties,
};
use aylis_core::automation::{AccessItem, ItemAction, ItemRole};
use aylis_data::{DataEvent, DataMessage, DataSource, DataView, Field, FieldLink, FieldType, RowStatus, data_set, view};

use aylis_controls::Edit;

use crate::column::{Column, ColumnKind, ColumnsChanged};
use aylis_core::PropEnum as _;
use aylis_controls::{ImageList, Picture};
use crate::{CellEditor, EditorCommand};

const HEADER: f32 = 32.0;
const INDICATOR: f32 = 22.0;
/// A largura da barra de rolagem sem o estilo `ScrollBar` do tema.
const SCROLLBAR: f32 = 10.0;
const RESIZE_GRIP: f32 = 4.0;
const MIN_COLUMN: f32 = 36.0;
/// Quantas linhas o ajuste da largura da coluna mede (a partir da primeira na tela).
const FIT_ROWS: usize = 1000;
const FILTER_ROW: f32 = 30.0;
const FOOTER: f32 = 30.0;
const CHECK: f32 = 16.0;
/// Depois desta pausa (segundos), a busca incremental começa de novo.
const SEARCH_PAUSE: f64 = 1.5;

/// Uma coluna do grid.
#[derive(Clone, Debug, PartialEq)]
pub struct GridColumn {
    pub field: String,
    pub title: String,
    pub width: f32,
    pub align: TextAlign,
    /// Campo lógico: desenhado como caixa de marcar.
    pub check: bool,
    /// Não edita nesta coluna (o `read_only` da `Column`).
    pub read_only: bool,
    /// O fundo e a cor do texto das células (da `Column`).
    pub color: Option<Color>,
    pub text_color: Option<Color>,
    /// O componente `Column` de onde a coluna veio (sem ele, da lista `columns`).
    pub source: Option<ComponentId>,
    /// O que a coluna mostra: o texto, um ícone do `ImageList` ou a imagem do campo.
    pub kind: ColumnKind,
    /// A dica do título (vazia: nenhuma).
    pub hint: String,
}

/// O registro de uma coluna da lista `columns` (e do `on_column_moved`) no acesso tipado:
/// `[{"field": "sigla", "width": 44}, {"field": "nome", "title": "Estado"}]`.
pub const GRID_COLUMN_RECORD: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "ColumnSpec",
    fields: &[
        ("field", aylis_core::PropKind::String),
        ("title", aylis_core::PropKind::String),
        ("hint", aylis_core::PropKind::String),
        ("width", aylis_core::PropKind::Float),
        ("read_only", aylis_core::PropKind::Bool),
        ("color", aylis_core::PropKind::Color),
        ("text_color", aylis_core::PropKind::Color),
        ("kind", aylis_core::PropKind::Enum(&["text", "icon", "image"])),
    ],
};

/// O registro de um total do rodapé (`footer`): `[{"field": "valor", "function": "sum"}]`.
pub const GRID_TOTAL_RECORD: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "GridTotal",
    fields: &[("field", aylis_core::PropKind::String), ("function", aylis_core::PropKind::Enum(&["sum", "avg", "min", "max", "count"]))],
};

/// Uma coluna da lista `columns` (o [`GRID_COLUMN_RECORD`]): o que não vier sai do `Field`.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnSpec {
    pub field: String,
    /// Vazio: o `display_label` do campo; `None` (`"title": null`): o cabeçalho vazio (uma
    /// coluna de ícone sem título).
    pub title: Option<String>,
    /// A dica do título (o mouse parado sobre ele).
    pub hint: String,
    /// 0: a largura do campo.
    pub width: f32,
    pub read_only: bool,
    pub color: Option<Color>,
    pub text_color: Option<Color>,
    /// `text` (o padrão), `icon` (do `ImageList` da grade, pelo valor do campo) ou `image` (a
    /// imagem guardada no campo).
    pub kind: ColumnKind,
}

impl Default for ColumnSpec {
    fn default() -> Self {
        ColumnSpec {
            field: String::new(),
            title: Some(String::new()),
            hint: String::new(),
            width: 0.0,
            read_only: false,
            color: None,
            text_color: None,
            kind: ColumnKind::Text,
        }
    }
}

impl ColumnSpec {
    pub fn new(field: &str, width: f32) -> Self {
        ColumnSpec { field: field.to_string(), width, ..Default::default() }
    }

    /// A lista `columns` lida do JSON; vazio, `[]` ou JSON inválido: nenhuma.
    pub fn parse_list(text: &str) -> Vec<ColumnSpec> {
        let Ok(serde_json::Value::Array(items)) = serde_json::from_str::<serde_json::Value>(text) else { return Vec::new() };
        items
            .iter()
            .filter_map(|j| {
                let field = j["field"].as_str()?.trim();
                if field.is_empty() {
                    return None;
                }
                let color = |k: &str| j[k].as_str().and_then(Color::from_hex);
                Some(ColumnSpec {
                    field: field.to_string(),
                    title: match j.get("title") {
                        Some(serde_json::Value::Null) => None,
                        t => Some(t.and_then(|t| t.as_str()).unwrap_or_default().to_string()),
                    },
                    hint: j["hint"].as_str().unwrap_or_default().to_string(),
                    width: j["width"].as_f64().unwrap_or(0.0) as f32,
                    read_only: j["read_only"].as_bool().unwrap_or(false),
                    color: color("color"),
                    text_color: color("text_color"),
                    kind: j["kind"].as_str().and_then(|k| ColumnKind::NAMES.iter().position(|n| *n == k.trim())).and_then(ColumnKind::from_index).unwrap_or_default(),
                })
            })
            .collect()
    }

    /// Em JSON, só com o que difere do padrão.
    pub fn to_json(&self) -> serde_json::Value {
        let mut o = serde_json::Map::new();
        o.insert("field".into(), self.field.clone().into());
        match &self.title {
            None => {
                o.insert("title".into(), serde_json::Value::Null);
            }
            Some(t) if !t.is_empty() => {
                o.insert("title".into(), t.clone().into());
            }
            Some(_) => {}
        }
        if !self.hint.is_empty() {
            o.insert("hint".into(), self.hint.clone().into());
        }
        if self.width > 0.0 {
            // Inteira sem o `.0` (o form fica limpo).
            let width = if self.width.fract() == 0.0 { serde_json::json!(self.width as i64) } else { serde_json::json!(self.width) };
            o.insert("width".into(), width);
        }
        if self.read_only {
            o.insert("read_only".into(), true.into());
        }
        if self.kind != ColumnKind::Text {
            o.insert("kind".into(), ColumnKind::NAMES[self.kind.index()].into());
        }
        for (k, c) in [("color", self.color), ("text_color", self.text_color)] {
            if let Some(c) = c {
                o.insert(k.into(), c.to_hex().into());
            }
        }
        serde_json::Value::Object(o)
    }

    /// A lista em JSON (o texto da propriedade `columns`).
    pub fn list_to_json(list: &[ColumnSpec]) -> String {
        serde_json::Value::Array(list.iter().map(ColumnSpec::to_json).collect()).to_string()
    }
}

/// O argumento do `on_column_moved`: as colunas na ordem nova, com as larguras (o mesmo
/// formato da lista `columns`, para o app guardar).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColumnLayout(pub Vec<ColumnSpec>);

impl aylis_core::EventValue for ColumnLayout {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(aylis_core::PropKind::RecordList(&GRID_COLUMN_RECORD));
    fn from_args(args: &EventArgs) -> Option<Self> {
        match args {
            EventArgs::Text(t) | EventArgs::Value(aylis_core::PropValue::String(t)) => Some(ColumnLayout(ColumnSpec::parse_list(t))),
            _ => None,
        }
    }
}

/// Um total do rodapé: o campo e a função (`sum`, `avg`, `min`, `max` ou `count`).
#[derive(Clone, Debug, Default, PartialEq)]
struct FooterTotal {
    field: String,
    function: String,
}

fn parse_footer(text: &str) -> Vec<FooterTotal> {
    let Ok(serde_json::Value::Array(items)) = serde_json::from_str::<serde_json::Value>(text) else { return Vec::new() };
    items
        .iter()
        .filter_map(|j| {
            let field = j["field"].as_str()?.trim().to_string();
            let function = j["function"].as_str()?.trim().to_ascii_lowercase();
            (!field.is_empty() && !function.is_empty()).then_some(FooterTotal { field, function })
        })
        .collect()
}

#[derive(Clone, Debug, Default, PartialEq)]
struct CachedRow {
    texts: Vec<String>,
    /// O ícone ou a imagem de cada coluna `icon`/`image` (as outras: `None`).
    pictures: Vec<Option<Picture>>,
    /// Pintura personalizada (`on_draw_cell`): a da linha e a de cada coluna.
    row_style: CellStyle,
    cell_styles: Vec<CellStyle>,
    /// Valor das colunas lógicas (`None` = nulo ou outra coluna).
    checks: Vec<Option<bool>>,
    /// `None` = registro novo em inclusão.
    status: Option<RowStatus>,
    error: bool,
    current: bool,
    selected: bool,
}

/// O que a pintura personalizada (`DataGrid::on_draw_cell`) recebe: a linha (índice e os
/// valores de todos os campos), a coluna e o valor dela (sem coluna: a linha inteira).
#[derive(Clone, Debug)]
pub struct CellInfo {
    /// A posição da linha entre as visíveis (0 = a primeira do dataset filtrado).
    pub row: usize,
    /// O campo da coluna; `None` na chamada da linha inteira.
    pub column: Option<String>,
    /// Os valores de todos os campos da linha, na ordem do dataset.
    pub values: Vec<(String, aylis_data::Value)>,
    /// O valor e o texto (já formatado) da célula; nulo e vazio na chamada da linha.
    pub value: aylis_data::Value,
    pub text: String,
    pub current: bool,
    pub selected: bool,
}

impl CellInfo {
    /// O valor de um campo da linha.
    pub fn get(&self, field: &str) -> Option<&aylis_data::Value> {
        self.values.iter().find(|(n, _)| n.eq_ignore_ascii_case(field)).map(|(_, v)| v)
    }

    /// A linha como objeto JSON (`{"id": 1, "nome": "Maria", ...}`), para backends em
    /// outras linguagens.
    pub fn json(&self) -> String {
        let fields: Vec<String> = self
            .values
            .iter()
            .map(|(name, v)| {
                format!("{}:{}", aylis_data::json::value_to_json(&aylis_data::Value::String(name.clone())), aylis_data::json::value_to_json(v))
            })
            .collect();
        format!("{{{}}}", fields.join(","))
    }
}

/// Pintura livre de uma célula: recebe o canvas e o retângulo dela.
pub type CellPainter = std::rc::Rc<dyn Fn(&mut dyn Canvas, RectF)>;

/// Como pintar uma célula (ou a linha): o que não vier fica como o tema manda.
#[derive(Clone, Default)]
pub struct CellStyle {
    pub background: Option<Color>,
    pub color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    /// Troca o texto mostrado.
    pub text: Option<String>,
    /// Pinta a célula por conta própria (no lugar do texto): barra, ícone, o que for.
    pub paint: Option<CellPainter>,
}

impl std::fmt::Debug for CellStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellStyle")
            .field("background", &self.background)
            .field("color", &self.color)
            .field("bold", &self.bold)
            .field("italic", &self.italic)
            .field("text", &self.text)
            .field("paint", &self.paint.is_some())
            .finish()
    }
}

impl PartialEq for CellStyle {
    fn eq(&self, other: &Self) -> bool {
        self.background == other.background
            && self.color == other.color
            && self.bold == other.bold
            && self.italic == other.italic
            && self.text == other.text
            && match (&self.paint, &other.paint) {
                (Some(a), Some(b)) => std::rc::Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

type DrawCellFn = std::rc::Rc<dyn Fn(&CellInfo) -> CellStyle>;

/// Mensagem interna: refazer as linhas preparadas.
struct Refill;

/// Pedido de [`DataGrid::move_column_to`].
struct MoveColumn {
    from: usize,
    to: usize,
}

/// Textos das linhas visíveis, preparados fora da pintura (que não acessa o `Ui`).
#[derive(Clone, Debug, Default, PartialEq)]
struct Cache {
    active: bool,
    first: usize,
    count: usize,
    current: Option<usize>,
    editing: bool,
    has_more: bool,
    /// Quantos registros a consulta tem no backend (`record_total`; 0: não se sabe).
    total: usize,
    /// O aviso do dataset: a ordem, o filtro ou a procura valeu só para os carregados.
    notice: String,
    rows: Vec<CachedRow>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Drag {
    #[default]
    None,
    Resize { column: usize, start_x: f32, start_width: f32 },
    Thumb { offset: f32 },
    /// O polegar da barra horizontal.
    HThumb { offset: f32 },
    /// Botão apertado num título: vira arrasto da coluna depois de alguns pontos; sem
    /// arrastar, é o clique que ordena.
    Title { column: usize, start_x: f32, x: f32, moved: bool, add: bool },
}

/// Quanto o mouse anda num título antes de virar arrasto da coluna.
const MOVE_THRESHOLD: f32 = 5.0;

/// Quando a roda do mouse rola as colunas do [`DataGrid`] (`horizontal_wheel`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HorizontalWheel {
    /// Com o Shift (o padrão dos sistemas).
    #[default]
    Shift,
    /// A roda comum rola as colunas (as linhas, só pelo teclado, pela barra ou pelo código).
    Always,
    /// Só o código rola as colunas (`scroll_left`).
    None,
}

impl aylis_core::prop::PropEnum for HorizontalWheel {
    const NAMES: &'static [&'static str] = &["shift", "always", "none"];
    fn from_index(i: usize) -> Option<Self> {
        [HorizontalWheel::Shift, HorizontalWheel::Always, HorizontalWheel::None].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Grade de registros.
///
/// - **virtualizada**: só as linhas visíveis são lidas e pintadas, então o custo não
///   depende do total de registros;
/// - **busca sob demanda**: perto do fim dos registros carregados, pede o próximo pacote
///   ao backend (`ClientDataSet.on_fetch`);
/// - colunas a partir dos campos visíveis (ou da lista `columns`, `[{"field": "nome"}]`), com título,
///   largura e alinhamento do `Field`; arraste a borda do cabeçalho para redimensionar;
/// - teclado: setas, PageUp/PageDown, Ctrl+Home/End, Home/End (colunas), Insert,
///   Ctrl+Delete, Esc (cancela a edição);
/// - edição na célula: Enter, F2, duplo clique ou começar a digitar; Enter grava, Esc
///   desfaz, Tab vai para a próxima coluna, setas gravam e mudam de linha;
/// - clique no título ordena pela coluna (de novo: decrescente; na terceira vez volta à
///   ordem do dataset), com `sortable`; Ctrl+clique acrescenta a coluna à ordem (ou troca
///   o sentido dela, ou a tira);
/// - `fixed_columns`: as primeiras colunas não rolam na horizontal;
/// - `filter_row`: uma linha embaixo do título para filtrar cada coluna (Enter aplica;
///   vira o `filter` do dataset);
/// - campos lógicos viram caixa de marcar: clique nela ou Espaço alternam;
/// - `multi_select` (o `dgMultiSelect`): clique escolhe uma linha, Ctrl+clique marca e
///   desmarca, Shift+clique e Shift+setas estendem, Ctrl+A seleciona tudo, Ctrl+Espaço
///   alterna a atual. A seleção guarda os registros ([`DataGrid::selected_ids`]), então
///   sobrevive à ordem e ao filtro; `on_selection_change` traz quantos são;
/// - `footer`: rodapé com totais, `[{"field": "valor", "function": "sum"}]` (sum, avg, min,
///   max, count),
///   sobre os registros carregados (com o filtro), no formato do campo;
/// - `incremental_search`: digitar procura na coluna atual o primeiro texto que começa com
///   o que foi digitado (pausa de 1,5 s recomeça; Backspace apaga); Enter e F2 editam;
/// - `cursor_color`: a cor da borda da célula atual (vazia = a cor de destaque do tema).
pub struct DataGrid {
    pub base: ControlBase,
    /// O `DataSource` de onde vêm os registros.
    pub data_source: Option<ComponentId>,
    /// O `ImageList` das colunas de ícone (`kind: icon`): o campo diz qual imagem.
    pub images: Option<ComponentId>,
    /// As imagens dos campos já decodificadas (pelos bytes), para não refazer a cada rolagem.
    blob_images: RefCell<HashMap<u64, aylis_core::Image>>,
    /// As colunas a mostrar, na ordem: a lista de [`GRID_COLUMN_RECORD`] em JSON
    /// (`[{"field": "sigla", "width": 44}, {"field": "nome", "width": 170}, {"field": "regiao"}]`;
    /// no form, a lista de objetos). Vazio = todos os campos visíveis. Lida só quando muda
    /// (as colunas ficam prontas até a próxima mudança).
    pub columns: String,
    /// A altura de cada linha, em pontos.
    pub row_height: f32,
    /// Só leitura: as células não editam (mesmo com o dataset editável).
    pub read_only: bool,
    /// O clique no título ordena pela coluna (de novo: decrescente; na terceira, volta à ordem
    /// do dataset); Ctrl+clique junta colunas à ordem.
    pub sortable: bool,
    /// Arrastar o título muda a ordem das colunas (as fixas trocam só entre si). A nova
    /// ordem chega no `on_column_moved` ([`ColumnLayout`]), no formato de `columns` (com as
    /// larguras).
    pub column_move: bool,
    /// Quantas colunas da esquerda ficam paradas ao rolar para os lados (0: nenhuma).
    pub fixed_columns: u32,
    /// Uma linha embaixo do título para filtrar cada coluna (Enter aplica; vira o `filter` do
    /// dataset).
    pub filter_row: bool,
    /// Seleciona várias linhas: Ctrl+clique marca e desmarca, Shift+clique estende, Ctrl+A
    /// seleciona tudo.
    pub multi_select: bool,
    /// O rodapé com totais: a lista de [`GRID_TOTAL_RECORD`] em JSON
    /// (`[{"field": "valor", "function": "sum"}, {"field": "id", "function": "count"}]`; sum,
    /// avg, min, max, count), sobre os registros carregados. Vazio: sem rodapé.
    pub footer: String,
    /// Digitar procura na coluna atual o primeiro texto que começa com o digitado (em vez de
    /// editar a célula).
    pub incremental_search: bool,
    /// A borda da célula atual; `None` usa a cor de destaque (`primary`) do tema.
    pub cursor_color: Option<Color>,
    /// A coluna da esquerda com a seta do registro atual; sem ela, a grade começa nos dados.
    pub indicator: bool,
    /// O fundo destacado na linha do registro atual; sem ele, só a seleção e o passar do mouse.
    pub highlight_current: bool,
    /// Quando a roda do mouse rola as colunas: `shift` (com o Shift, o padrão dos sistemas),
    /// `always` (a roda comum rola as colunas) ou `none` (só o código, pelo `scroll_left`). O
    /// movimento de lado do touchpad sempre rola as colunas, menos em `none`.
    pub horizontal_wheel: HorizontalWheel,
    /// O texto da grade sem registros (vazio: "Sem registros", traduzido).
    pub empty_text: String,
    /// O fundo e a moldura próprios da grade (o card); desligado, ela pinta só o conteúdo e
    /// fica sem a segunda caixa dentro de um `Panel` com borda.
    pub frame: bool,
    /// A dica da grade (a propriedade `hint`) enquanto o mouse está num título com dica.
    own_hint: Option<String>,
    /// A largura da barra de rolagem: a do estilo `ScrollBar` do tema (ou do `scroll_bar` do
    /// estilo da grade), lida ao preparar as linhas; sem ela, [`SCROLLBAR`].
    bar_width: std::cell::Cell<f32>,
    /// A largura da coluna no começo do arraste do separador no designer.
    design_width: f32,
    /// A coluna que avisou a mudança (ocupada no `Ui` enquanto avisa).
    changing_column: Option<(ComponentId, Column)>,
    /// Registros selecionados (ids) e o de onde a extensão com Shift começa.
    selected: BTreeSet<u64>,
    anchor: Option<u64>,
    /// Totais do rodapé por coluna (texto pronto); recalculados quando os dados mudam.
    totals: Vec<Option<String>>,
    /// O `footer` lido (e o texto de onde veio): a geometria pergunta a cada quadro se há
    /// rodapé, e o JSON só é lido de novo quando o texto muda.
    footer_totals: std::cell::RefCell<(String, std::rc::Rc<Vec<FooterTotal>>)>,
    /// A lista `columns` lida na última montagem das colunas (para regravar com as larguras).
    specs: Vec<ColumnSpec>,
    totals_dirty: bool,
    /// O que foi digitado na busca incremental e quando.
    search: String,
    search_time: f64,
    /// Textos da linha de filtro, por coluna.
    filters: Vec<String>,
    filter_editor: Option<(ComponentId, usize)>,
    /// Ordem atual (a do índice do dataset): cada coluna e se é decrescente.
    sort: Vec<(String, bool)>,
    /// A ordem escolhida nos títulos (vazia = a do dataset).
    user_sort: Vec<(String, bool)>,
    /// A ordem do dataset antes do primeiro clique (`index_name`, `index_field_names`),
    /// para voltar a ela.
    default_index: Option<(String, String)>,
    cols: Vec<GridColumn>,
    top_row: usize,
    scroll_x: f32,
    col: usize,
    cache: Cache,
    /// O id de cada linha preparada.
    cache_ids: Vec<Option<u64>>,
    editor: Option<ComponentId>,
    drag: Drag,
    hovered_row: Option<usize>,
    /// Arrasto do dedo ainda não convertido em linhas.
    touch_rest: f32,
    /// A pintura personalizada do app (`DataGrid::on_draw_cell`).
    draw_cell: Option<DrawCellFn>,
    /// As fontes da última pintura (o texto das células e o do cabeçalho), para medir no
    /// ajuste da largura da coluna.
    text_style: std::cell::RefCell<Option<(aylis_core::TextStyle, aylis_core::TextStyle)>>,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
}

impl Default for DataGrid {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 300.0);
        base.set_can_focus(true).set_clip_children(true);
        Self { overscroll: aylis_core::Overscroll::Auto,
            base,
            data_source: None,
            images: None,
            blob_images: RefCell::default(),
            columns: String::new(),
            row_height: 30.0,
            read_only: false,
            sortable: true,
            column_move: true,
            fixed_columns: 0,
            filter_row: false,
            multi_select: false,
            footer: String::new(),
            incremental_search: false,
            cursor_color: None,
            indicator: true,
            highlight_current: true,
            horizontal_wheel: HorizontalWheel::Shift,
            empty_text: String::new(),
            frame: true,
            own_hint: None,
            bar_width: std::cell::Cell::new(SCROLLBAR),
            design_width: 0.0,
            changing_column: None,
            selected: BTreeSet::new(),
            anchor: None,
            totals: Vec::new(),
            footer_totals: std::cell::RefCell::new((String::new(), std::rc::Rc::new(Vec::new()))),
            specs: Vec::new(),
            totals_dirty: true,
            search: String::new(),
            search_time: f64::NEG_INFINITY,
            filters: Vec::new(),
            filter_editor: None,
            sort: Vec::new(),
            user_sort: Vec::new(),
            default_index: None,
            cols: Vec::new(),
            top_row: 0,
            scroll_x: 0.0,
            col: 0,
            cache: Cache::default(),
            cache_ids: Vec::new(),
            editor: None,
            drag: Drag::None,
            hovered_row: None,
            touch_rest: 0.0,
            draw_cell: None,
            text_style: std::cell::RefCell::new(None),
        }
    }
}

impl DataGrid {
    pub fn new(data_source: ComponentId) -> Self {
        Self { data_source: Some(data_source), ..Default::default() }
    }

    /// A largura da barra de rolagem (a do tema, lida ao preparar as linhas).
    fn bar(&self) -> f32 {
        self.bar_width.get()
    }

    /// A largura da coluna indicadora (0 sem ela).
    fn indicator_w(&self) -> f32 {
        if self.indicator { INDICATOR } else { 0.0 }
    }

    /// Colunas atuais (depois que o dataset abriu).
    pub fn grid_columns(&self) -> &[GridColumn] {
        &self.cols
    }

    /// Primeira linha visível.
    /// Quanto da consulta a tela mostra (o tamanho do polegar da barra de rolagem, de 0 a 1):
    /// com o total do backend (`record_total`), a fração dele.
    pub fn scroll_fraction(&self) -> f32 {
        (self.visible_rows() as f32 / self.scroll_total().max(1) as f32).min(1.0)
    }

    pub fn top_row(&self) -> usize {
        self.top_row
    }

    /// Deslocamento horizontal das colunas (a barra de baixo).
    pub fn scroll_x(&self) -> f32 {
        self.scroll_x
    }

    /// Coluna selecionada.
    pub fn selected_column(&self) -> usize {
        self.col
    }

    /// Quantas linhas estão preparadas para a pintura (as visíveis).
    pub fn cached_rows(&self) -> usize {
        self.cache.rows.len()
    }

    /// Editor de célula aberto, se houver.
    pub fn editor(&self) -> Option<ComponentId> {
        self.editor
    }

    /// Os registros selecionados (`multi_select`), pelo id ([`aylis_data::RowInfo::id`]); vá
    /// até cada um com [`aylis_data::DataSet::goto_id`].
    pub fn selected_ids(&self) -> Vec<u64> {
        self.selected.iter().copied().collect()
    }

    pub fn is_selected(&self, id: u64) -> bool {
        self.selected.contains(&id)
    }

    pub fn clear_selection(&mut self) {
        self.selected.clear();
        self.anchor = None;
        self.fill_selection();
    }

    /// O texto do total do rodapé na coluna `field` (depois de pintar ou de um evento).
    pub fn footer_text(&self, field: &str) -> Option<&str> {
        let i = self.cols.iter().position(|c| c.field.eq_ignore_ascii_case(field))?;
        self.totals.get(i)?.as_deref()
    }

    /// O que foi digitado na busca incremental (vazio depois da pausa).
    pub fn search_text(&self) -> &str {
        &self.search
    }

    fn on_set(&mut self, name: &str) {
        if name == "columns" {
            self.cols.clear();
        }
        if name == "columns" || name == "footer" {
            self.totals_dirty = true;
        }
        if name == "multi_select" && !self.multi_select {
            self.clear_selection();
        }
    }

    /// Marca nas linhas preparadas quais estão selecionadas.
    fn fill_selection(&mut self) {
        let ids = self.cache_ids.clone();
        for (row, id) in self.cache.rows.iter_mut().zip(ids) {
            row.selected = id.is_some_and(|id| self.selected.contains(&id));
        }
    }

    fn data_set(&self, ui: &Ui) -> Option<ComponentId> {
        let source = ui.get::<DataSource>(self.data_source?)?;
        if source.enabled { source.data_set } else { None }
    }

    // ----- geometria -----

    fn width(&self) -> f32 {
        self.base.bounds().width
    }

    fn height(&self) -> f32 {
        self.base.bounds().height
    }

    /// Altura do cabeçalho (título e, se ligada, a linha de filtro).
    fn top(&self) -> f32 {
        HEADER + if self.filter_row { FILTER_ROW } else { 0.0 }
    }

    /// Os totais do `footer` (lidos de novo só quando o texto muda).
    fn footer_list(&self) -> std::rc::Rc<Vec<FooterTotal>> {
        let mut cache = self.footer_totals.borrow_mut();
        if cache.0 != self.footer {
            *cache = (self.footer.clone(), std::rc::Rc::new(parse_footer(&self.footer)));
        }
        cache.1.clone()
    }

    /// Altura do rodapé de totais (0 sem `footer`).
    fn footer_height(&self) -> f32 {
        if self.footer_list().is_empty() { 0.0 } else { FOOTER }
    }

    /// Linhas inteiras que cabem (sem a barra horizontal, embaixo).
    fn visible_rows(&self) -> usize {
        (((self.height() - self.top() - self.footer_height() - self.hbar_height()) / self.row_height).floor().max(1.0)) as usize
    }

    /// As colunas passam da largura: a barra horizontal aparece (e ocupa uma faixa embaixo).
    /// Conta com a barra vertical das linhas que cabem sem ela, para não depender de si.
    fn has_hbar(&self) -> bool {
        let rows = ((self.height() - self.top() - self.footer_height()) / self.row_height).floor().max(1.0) as usize;
        let vbar = self.cache.count > rows || self.cache.has_more;
        self.content_width() > (self.width() - self.indicator_w() - if vbar { self.bar() + 2.0 } else { 0.0 }).max(0.0) + 0.5
    }

    fn hbar_height(&self) -> f32 {
        if self.has_hbar() { self.bar() + 2.0 } else { 0.0 }
    }

    /// O trilho da barra horizontal (embaixo, da largura das colunas).
    fn htrack(&self) -> RectF {
        RectF::new(self.indicator_w(), self.height() - self.bar() - 2.0, self.data_width(), self.bar())
    }

    fn hthumb(&self) -> RectF {
        let track = self.htrack();
        let content = self.content_width().max(1.0);
        let width = (track.width * (self.data_width() / content).min(1.0)).max(24.0).min(track.width);
        let max = (content - self.data_width()).max(1.0);
        RectF::new(track.x + (track.width - width) * (self.scroll_x / max).clamp(0.0, 1.0), track.y, width, track.height)
    }

    /// O texto da célula atual, como aparece na grade (o Ctrl+C).
    fn current_cell_text(&self, ui: &Ui) -> Option<String> {
        let ds = self.data_set(ui)?;
        let v = view(ui, ds);
        let row = v.visible_current()?;
        let column = self.cols.get(self.col)?;
        v.visible_text(row, &column.field).ok()
    }

    /// Ajusta a largura da coluna ao maior texto (o título e as células a partir da
    /// primeira linha na tela, até 1.000: com milhões de registros, medir todos travaria),
    /// como o duplo clique no separador do cabeçalho.
    pub fn fit_column(&mut self, ui: &Ui, column: usize) {
        let Some(c) = self.cols.get(column) else { return };
        let Some((cell_style, header_style)) = self.text_style.borrow().clone() else { return };
        let mut widest = ui.measure_text(&c.title, &header_style).width + 14.0;
        if let Some(ds) = self.data_set(ui) {
            let v = view(ui, ds);
            let count = v.visible_count();
            let first = self.top_row.min(count);
            for i in first..(first + FIT_ROWS).min(count) {
                if let Ok(text) = v.visible_text(i, &c.field) {
                    widest = widest.max(ui.measure_text(&text, &cell_style).width);
                }
            }
        }
        let field = c.field.clone();
        if let Some(c) = self.cols.iter_mut().find(|c| c.field == field) {
            c.width = (widest + 18.0).clamp(MIN_COLUMN, 800.0);
        }
        self.clamp_scroll_x();
    }

    /// Largura das colunas fixas.
    fn fixed_width(&self) -> f32 {
        self.cols.iter().take(self.fixed_columns as usize).map(|c| c.width).sum()
    }

    fn content_width(&self) -> f32 {
        self.cols.iter().map(|c| c.width).sum()
    }

    fn has_vbar(&self) -> bool {
        self.cache.count > self.visible_rows() || self.cache.has_more
    }

    fn data_width(&self) -> f32 {
        (self.width() - self.indicator_w() - if self.has_vbar() { self.bar() + 2.0 } else { 0.0 }).max(0.0)
    }

    fn column_x(&self, index: usize) -> f32 {
        let x = self.indicator_w() + self.cols[..index].iter().map(|c| c.width).sum::<f32>();
        if index < self.fixed_columns as usize { x } else { x - self.scroll_x }
    }

    fn column_at(&self, x: f32) -> Option<usize> {
        let fixed_right = self.indicator_w() + self.fixed_width();
        (0..self.cols.len()).find(|&i| {
            let left = self.column_x(i);
            let fixed = i < self.fixed_columns as usize;
            // Uma coluna que rola fica escondida atrás das fixas.
            x >= left && x < left + self.cols[i].width && (fixed || x >= fixed_right)
        })
    }

    /// As colunas na ordem atual, com a largura de cada uma (o que mais a lista `columns`
    /// dizia de cada coluna fica).
    fn column_layout(&self) -> Vec<ColumnSpec> {
        self.cols
            .iter()
            .map(|c| {
                let mut spec = self.specs.iter().find(|s| s.field.eq_ignore_ascii_case(&c.field)).cloned().unwrap_or_else(|| ColumnSpec::new(&c.field, 0.0));
                spec.width = c.width.round();
                spec
            })
            .collect()
    }

    /// O `columns` com a largura atual de cada coluna (o arraste do separador no designer).
    fn columns_with_widths(&self) -> String {
        ColumnSpec::list_to_json(&self.column_layout())
    }

    /// Onde a coluna arrastada entra, como o espaço antes da coluna `gap` (`len`: no fim).
    /// As fixas trocam só entre si; as que rolam, só entre elas.
    fn drop_gap(&self, column: usize, x: f32) -> usize {
        let fixed = self.fixed_columns as usize;
        let zone = if column < fixed { 0..fixed.min(self.cols.len()) } else { fixed.min(self.cols.len())..self.cols.len() };
        let mut gap = zone.start;
        for i in zone.clone() {
            if x > self.column_x(i) + self.cols[i].width / 2.0 {
                gap = i + 1;
            }
        }
        gap.clamp(zone.start, zone.end)
    }

    /// Leva a coluna `from` para o espaço `gap` (de [`drop_gap`](Self::drop_gap)).
    fn move_column(&mut self, ctx: &mut EventCtx<'_>, from: usize, gap: usize) {
        let to = if gap > from { gap - 1 } else { gap };
        if to == from || from >= self.cols.len() {
            return;
        }
        let column = self.cols.remove(from);
        self.cols.insert(to, column);
        if from < self.filters.len() && to < self.filters.len() {
            let text = self.filters.remove(from);
            self.filters.insert(to, text);
        }
        if from < self.totals.len() && to < self.totals.len() {
            let total = self.totals.remove(from);
            self.totals.insert(to, total);
        }
        // A coluna atual acompanha.
        self.col = match self.col {
            c if c == from => to,
            c if from < c && c <= to => c - 1,
            c if to <= c && c < from => c + 1,
            c => c,
        };
        self.refresh(ctx);
        // A nova ordem (com as larguras), no formato de `columns` (o `ColumnLayout`): o app
        // pode guardar.
        let layout = self.columns_with_widths();
        ctx.fire("on_column_moved", EventArgs::Text(layout));
    }

    /// Muda a ordem das colunas por código: a coluna `from` vai para a posição `to`.
    pub fn move_column_to(ui: &mut Ui, id: ComponentId, from: usize, to: usize) {
        ui.send_message(id, &MoveColumn { from, to });
    }

    fn border_at(&self, x: f32) -> Option<usize> {
        (0..self.cols.len()).find(|&i| (self.column_x(i) + self.cols[i].width - x).abs() <= RESIZE_GRIP)
    }

    fn track(&self) -> RectF {
        RectF::new(self.width() - self.bar() - 2.0, self.top() + 2.0, self.bar(), (self.height() - self.top() - self.footer_height() - self.hbar_height() - 4.0).max(0.0))
    }

    /// Total para a barra de rolagem: com mais registros no backend, o total que ele disse
    /// (`record_total`) ou, sem ele, uma página a mais.
    fn scroll_total(&self) -> usize {
        if !self.cache.has_more {
            return self.cache.count;
        }
        if self.cache.total > self.cache.count {
            return self.cache.total;
        }
        self.cache.count + self.visible_rows()
    }

    fn thumb(&self) -> RectF {
        let track = self.track();
        let total = self.scroll_total().max(1) as f32;
        let visible = self.visible_rows() as f32;
        let height = (track.height * (visible / total).min(1.0)).max(24.0).min(track.height);
        let max_top = (total - visible).max(1.0);
        let y = track.y + (track.height - height) * (self.top_row as f32 / max_top).min(1.0);
        RectF::new(track.x, y, track.width, height)
    }

    // ----- dados -----

    /// As colunas, dos campos do dataset (`fields`: nome e componente `Field`, na ordem).
    /// Serve também com o dataset fechado, como no designer. Com componentes `Column`
    /// filhos da grade (`id`), são eles; senão a lista `columns`; senão os campos visíveis.
    fn build_columns(&mut self, ui: &Ui, id: ComponentId, fields: &[(String, &Field)]) {
        let field = |name: &str| fields.iter().find(|(n, _)| n == name).map(|(_, f)| *f);
        let changing = self.changing_column.take();
        let children: Vec<(ComponentId, &Column)> = ui
            .children(id)
            .iter()
            .filter_map(|&c| match &changing {
                Some((busy, column)) if *busy == c => Some((c, column)),
                _ => Some((c, ui.get::<Column>(c)?)),
            })
            .collect();
        // A lista `columns` é lida aqui, só quando as colunas são refeitas (não a cada quadro).
        self.specs = ColumnSpec::parse_list(&self.columns);
        let listed: Vec<ColumnSpec> = self.specs.clone();
        let specs: Vec<(String, Option<&Column>, Option<ComponentId>, Option<&ColumnSpec>)> = if !children.is_empty() {
            children.into_iter().filter(|(_, c)| c.visible).map(|(id, c)| (c.field_name.clone(), Some(c), Some(id), None)).collect()
        } else if listed.is_empty() {
            fields.iter().filter(|(_, f)| f.visible).map(|(n, _)| (n.clone(), None, None, None)).collect()
        } else {
            listed.iter().map(|s| (s.field.clone(), None, None, Some(s))).collect()
        };
        self.cols = specs
            .into_iter()
            .filter_map(|(name, column, source, spec)| {
                let field = field(&name)?;
                let own_width = spec.map(|s| s.width).filter(|&w| w > 0.0).or(column.map(|c| c.width).filter(|&w| w > 0.0));
                let width = if let Some(w) = own_width {
                    w.max(MIN_COLUMN)
                } else if field.display_width > 0.0 {
                    field.display_width
                } else {
                    match field.field_type {
                        FieldType::String | FieldType::Memo if field.size > 0 => (field.size as f32 * 7.5).clamp(90.0, 280.0),
                        FieldType::String | FieldType::Memo => 180.0,
                        FieldType::Integer | FieldType::AutoInc => 80.0,
                        FieldType::Float | FieldType::Currency => 120.0,
                        FieldType::Boolean => 70.0,
                        FieldType::Date => 105.0,
                        FieldType::Time => 90.0,
                        FieldType::DateTime | FieldType::Timestamp => 160.0,
                        FieldType::Blob => 90.0,
                    }
                };
                let check = field.field_type == FieldType::Boolean;
                let align = column.and_then(|c| c.alignment).unwrap_or(if check { TextAlign::Center } else { field.alignment() });
                // `null`: o cabeçalho vazio; vazio: o rótulo do campo.
                let title = match column.map(|c| c.title.as_deref()).or(spec.map(|s| s.title.as_deref())) {
                    Some(None) => String::new(),
                    Some(Some(t)) if !t.is_empty() => t.to_string(),
                    _ => field.caption().to_string(),
                };
                let hint = column.map(|c| c.hint.clone()).filter(|h| !h.is_empty()).or(spec.map(|s| s.hint.clone())).unwrap_or_default();
                Some(GridColumn {
                    field: name,
                    title,
                    width,
                    align,
                    check,
                    read_only: column.is_some_and(|c| c.read_only) || spec.is_some_and(|s| s.read_only),
                    color: column.and_then(|c| c.color).or(spec.and_then(|s| s.color)),
                    text_color: column.and_then(|c| c.text_color).or(spec.and_then(|s| s.text_color)),
                    source,
                    kind: column.map(|c| c.kind).filter(|k| *k != ColumnKind::Text).or(spec.map(|s| s.kind)).unwrap_or_default(),
                    hint,
                })
            })
            .collect();
        self.col = self.col.min(self.cols.len().saturating_sub(1));
    }

    /// Pintura personalizada da linha e das células:
    /// para cada linha visível, `paint` é chamado uma vez para a linha inteira
    /// (`column = None`) e uma vez por coluna, com os dados da linha; devolve o estilo
    /// (fundo, cor, negrito, texto trocado) ou uma pintura livre. Chamado ao preparar as
    /// linhas (fora da pintura), e de novo quando os dados mudam.
    ///
    /// ```no_run
    /// # use aylis_dbcontrols::{DataGrid, CellStyle};
    /// # let mut ui = aylis_core::Ui::new(); let grid = ui.find("grdPedidos").unwrap();
    /// DataGrid::on_draw_cell(&mut ui, grid, |cell| {
    ///     let atrasado = cell.get("status").and_then(|v| v.as_str()).is_some_and(|s| s == "atrasado");
    ///     match cell.column.as_deref() {
    ///         None if atrasado => CellStyle { background: Some(aylis_core::Color::rgb(0xFF, 0xEB, 0xEE)), ..Default::default() },
    ///         Some("valor") => CellStyle { bold: true, ..Default::default() },
    ///         _ => CellStyle::default(),
    ///     }
    /// });
    /// ```
    pub fn on_draw_cell(ui: &mut Ui, id: ComponentId, paint: impl Fn(&CellInfo) -> CellStyle + 'static) {
        if let Some(g) = ui.get_mut::<DataGrid>(id) {
            g.draw_cell = Some(std::rc::Rc::new(paint));
        }
        ui.send_message(id, &Refill);
    }

    /// Refaz as linhas visíveis (a pintura personalizada dependia de algo que mudou fora do
    /// dataset).
    pub fn refresh_rows(ui: &mut Ui, id: ComponentId) {
        ui.send_message(id, &Refill);
    }

    /// A imagem dos bytes de um campo, decodificada uma vez (pelo conteúdo): rolar não refaz.
    /// Guarda até 512; passando, começa de novo.
    fn blob_image(&self, bytes: &[u8]) -> aylis_core::Image {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut h);
        let key = h.finish();
        let mut cache = self.blob_images.borrow_mut();
        if cache.len() > 512 {
            cache.clear();
        }
        cache.entry(key).or_insert_with(|| aylis_core::Image::encoded(bytes.to_vec())).clone()
    }

    /// Prepara os textos das linhas visíveis.
    fn fill(&mut self, ui: &Ui, id: ComponentId) {
        // A barra do tema muda quantas linhas e colunas cabem: lida antes de contar.
        let bar = aylis_controls::scroll_bar::look_of(ui, id, &*self, false).width.unwrap_or(SCROLLBAR);
        self.bar_width.set(bar);
        let Some(ds) = self.data_set(ui) else {
            self.cache = Cache::default();
            return;
        };
        let v = view(ui, ds);
        if !v.is_active() {
            // Fechado (o designer): as colunas saem dos componentes Field, sem linhas.
            self.cache = Cache::default();
            if self.cols.is_empty() {
                let fields: Vec<(String, &Field)> = ui
                    .children(ds)
                    .iter()
                    .filter_map(|&c| {
                        let f = ui.get::<Field>(c)?;
                        let name = if f.field_name.is_empty() { ui.name(c)?.to_string() } else { f.field_name.clone() };
                        Some((name, f))
                    })
                    .collect();
                self.build_columns(ui, id, &fields);
            }
            return;
        }
        if self.cols.is_empty() {
            let fields: Vec<(String, &Field)> = v.field_names().into_iter().filter_map(|n| Some((n.clone(), v.field(&n).ok()?))).collect();
            self.build_columns(ui, id, &fields);
        }
        let count = v.visible_count();
        let visible = self.visible_rows();
        self.top_row = self.top_row.min(count.saturating_sub(visible));
        let end = (self.top_row + visible + 1).min(count);
        let mut ids = Vec::with_capacity(end.saturating_sub(self.top_row));
        let field_names = v.field_names();
        let rows = (self.top_row..end)
            .map(|i| {
                let info = v.visible_row(i);
                let id = info.and_then(|r| r.id);
                ids.push(id);
                let texts: Vec<String> = self.cols.iter().map(|c| if c.check { String::new() } else { v.visible_text(i, &c.field).unwrap_or_default() }).collect();
                let current = info.is_some_and(|r| r.current);
                let selected = id.is_some_and(|id| self.selected.contains(&id));
                // Pintura personalizada: a linha e cada coluna, com os dados da linha.
                let (row_style, cell_styles) = match &self.draw_cell {
                    Some(paint) => {
                        let values: Vec<(String, aylis_data::Value)> =
                            field_names.iter().map(|n| (n.clone(), v.visible_value(i, n).unwrap_or(aylis_data::Value::Null))).collect();
                        let mut cell = CellInfo { row: i, column: None, values, value: aylis_data::Value::Null, text: String::new(), current, selected };
                        let row_style = paint(&cell);
                        let cells = self
                            .cols
                            .iter()
                            .zip(&texts)
                            .map(|(c, text)| {
                                cell.column = Some(c.field.clone());
                                cell.value = cell.get(&c.field).cloned().unwrap_or(aylis_data::Value::Null);
                                cell.text = text.clone();
                                paint(&cell)
                            })
                            .collect();
                        (row_style, cells)
                    }
                    None => (CellStyle::default(), Vec::new()),
                };
                // As colunas de ícone (o `ImageList`, pelo valor do campo) e de imagem (os bytes
                // do campo, decodificados uma vez).
                let pictures: Vec<Option<Picture>> = self
                    .cols
                    .iter()
                    .map(|c| match c.kind {
                        ColumnKind::Text => None,
                        ColumnKind::Icon => {
                            let key = v.visible_value(i, &c.field).ok().map(|x| x.to_string()).unwrap_or_default();
                            self.images.and_then(|list| ImageList::picture(ui, list, &key))
                        }
                        ColumnKind::Image => match v.visible_value(i, &c.field) {
                            Ok(aylis_data::Value::Blob(bytes)) if !bytes.is_empty() => Some(Picture::Image(self.blob_image(&bytes))),
                            _ => None,
                        },
                    })
                    .collect();
                CachedRow {
                    texts,
                    pictures,
                    row_style,
                    cell_styles,
                    checks: self.cols.iter().map(|c| if c.check { v.visible_value(i, &c.field).ok().and_then(|x| x.as_bool()) } else { None }).collect(),
                    status: info.and_then(|r| r.status),
                    error: info.is_some_and(|r| r.error.is_some()),
                    current,
                    selected,
                }
            })
            .collect();
        self.cache_ids = ids;
        if self.totals_dirty || self.totals.len() != self.cols.len() {
            self.totals = self.compute_totals(&v, &ui.locale());
            self.totals_dirty = false;
        }
        self.sort = ui.get::<aylis_data::ClientDataSet>(ds).and_then(|c| c.effective_index().ok()).map_or_else(Vec::new, |index| parse_sort(&index));
        // O código trocou a ordem por fora: a dos títulos deixa de valer.
        if !self.user_sort.is_empty() && !same_sort(&self.sort, &self.user_sort) {
            self.user_sort.clear();
            self.default_index = None;
        }
        if self.filters.len() != self.cols.len() {
            self.filters.resize(self.cols.len(), String::new());
        }
        self.cache = Cache {
            active: true,
            first: self.top_row,
            count,
            current: v.visible_current(),
            editing: v.state().is_editing(),
            has_more: v.has_more(),
            total: ui.get::<aylis_data::ClientDataSet>(ds).map_or(0, |c| c.record_total.max(0) as usize),
            notice: ui.get::<aylis_data::ClientDataSet>(ds).map(|c| c.partial_notice().to_string()).unwrap_or_default(),
            rows,
        };
    }

    /// Os totais do rodapé (`sum`, `avg`, `min`, `max`, `count` de cada campo), sobre os
    /// registros carregados.
    fn compute_totals(&self, v: &DataView<'_>, locale: &aylis_core::platform::LocaleInfo) -> Vec<Option<String>> {
        let mut totals = vec![None; self.cols.len()];
        for total in self.footer_list().iter() {
            let (name, kind) = (total.field.as_str(), &total.function);
            let Some(col) = self.cols.iter().position(|c| c.field.eq_ignore_ascii_case(name)) else { continue };
            let Ok(field) = v.field(name) else { continue };
            let values: Vec<aylis_data::Value> = (0..v.record_count()).filter_map(|i| v.value_at(i, name).ok()).filter(|x| !x.is_null()).collect();
            let numbers: Vec<f64> = values.iter().filter_map(aylis_data::Value::as_f64).collect();
            let number = |x: f64| -> String {
                // No formato do campo (moeda, casas decimais); inteiros ficam inteiros.
                let value = if matches!(field.field_type, FieldType::Integer | FieldType::AutoInc) && x.fract() == 0.0 {
                    aylis_data::Value::Int(x as i64)
                } else {
                    aylis_data::Value::Float(x)
                };
                field.text_of(&value, true, locale)
            };
            totals[col] = match kind.as_str() {
                "count" => Some(values.len().to_string()),
                "sum" => Some(number(numbers.iter().sum())),
                "avg" if !numbers.is_empty() => Some(number(numbers.iter().sum::<f64>() / numbers.len() as f64)),
                "min" => numbers.iter().copied().reduce(f64::min).map(number),
                "max" => numbers.iter().copied().reduce(f64::max).map(number),
                _ => None,
            };
        }
        totals
    }

    // ----- seleção de várias linhas -----


    /// Seleciona ao ir para a linha `row`: `toggle` (Ctrl) alterna, `extend` (Shift) vai da
    /// âncora até ela, sem nada escolhe só ela.
    fn select_row(&mut self, ctx: &mut EventCtx<'_>, row: usize, toggle: bool, extend: bool) {
        let Some(ds) = self.data_set(ctx.ui) else { return };
        let v = view(ctx.ui, ds);
        let Some(id) = v.visible_row(row).and_then(|r| r.id) else { return };
        if extend {
            let anchor_row = self.anchor.and_then(|a| (0..v.visible_count()).find(|&i| v.visible_row(i).and_then(|r| r.id) == Some(a))).unwrap_or(row);
            if !toggle {
                self.selected.clear();
            }
            for i in anchor_row.min(row)..=anchor_row.max(row) {
                if let Some(id) = v.visible_row(i).and_then(|r| r.id) {
                    self.selected.insert(id);
                }
            }
        } else if toggle {
            if !self.selected.remove(&id) {
                self.selected.insert(id);
            }
            self.anchor = Some(id);
        } else {
            self.selected.clear();
            self.selected.insert(id);
            self.anchor = Some(id);
        }
        self.fill_selection();
        ctx.fire("on_selection_change", EventArgs::Text(self.selected.len().to_string()));
        ctx.repaint();
    }

    fn select_all(&mut self, ctx: &mut EventCtx<'_>) {
        let Some(ds) = self.data_set(ctx.ui) else { return };
        let v = view(ctx.ui, ds);
        self.selected = (0..v.visible_count()).filter_map(|i| v.visible_row(i).and_then(|r| r.id)).collect();
        self.fill_selection();
        ctx.fire("on_selection_change", EventArgs::Text(self.selected.len().to_string()));
        ctx.repaint();
    }

    // ----- caixa de marcar e busca -----

    /// Alterna o campo lógico da coluna `column` no registro atual (entra em edição).
    fn toggle_check(&mut self, ctx: &mut EventCtx<'_>, column: usize) {
        let Some(c) = self.cols.get(column).filter(|c| c.check) else { return };
        let field = c.field.clone();
        let Some(source) = self.data_source else { return };
        if self.read_only || !(FieldLink { data_source: Some(source), data_field: field.clone() }).can_modify(ctx.ui) {
            return;
        }
        self.with_data(ctx, |d| {
            d.edit()?;
            let on = d.value(&field)?.as_bool().unwrap_or(false);
            d.set_value(&field, !on)
        });
    }

    /// O quadrado da caixa de marcar na célula.
    fn check_rect(cell: RectF) -> RectF {
        RectF::new(cell.x + (cell.width - CHECK) / 2.0, cell.y + (cell.height - CHECK) / 2.0, CHECK, CHECK)
    }

    /// Busca incremental: soma `text` ao que foi digitado e vai até o primeiro registro
    /// (a partir do atual) cuja coluna atual começa com isso.
    fn search_typed(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        let now = ctx.ui.time();
        if now - self.search_time > SEARCH_PAUSE {
            self.search.clear();
        }
        self.search_time = now;
        self.search.push_str(text);
        self.search_now(ctx, true);
    }

    fn search_now(&mut self, ctx: &mut EventCtx<'_>, from_current: bool) {
        let Some(ds) = self.data_set(ctx.ui) else { return };
        let Some(c) = self.cols.get(self.col) else { return };
        let wanted = self.search.to_lowercase();
        let v = view(ctx.ui, ds);
        let count = v.visible_count();
        let start = if from_current { v.visible_current().unwrap_or(0) } else { 0 };
        let found = (0..count).map(|k| (start + k) % count.max(1)).find(|&i| v.visible_text(i, &c.field).is_ok_and(|t| t.to_lowercase().starts_with(&wanted)));
        if let Some(row) = found {
            self.with_data(ctx, |d| d.move_to(row));
        }
        ctx.repaint();
    }

    fn ensure_current_visible(&mut self) {
        let Some(current) = self.cache.current else { return };
        let visible = self.visible_rows();
        if current < self.top_row {
            self.top_row = current;
        } else if current >= self.top_row + visible {
            self.top_row = current + 1 - visible;
        }
    }

    fn ensure_column_visible(&mut self) {
        if self.cols.is_empty() || self.col < self.fixed_columns as usize {
            return;
        }
        let fixed = self.fixed_width();
        let left = self.column_x(self.col) + self.scroll_x - self.indicator_w() - fixed;
        let right = left + self.cols[self.col].width;
        let width = self.data_width() - fixed;
        if left < self.scroll_x {
            self.scroll_x = left;
        } else if right > self.scroll_x + width {
            self.scroll_x = (right - width).max(0.0);
        }
    }

    fn clamp_scroll_x(&mut self) {
        self.scroll_x = self.scroll_x.clamp(0.0, (self.content_width() - self.data_width()).max(0.0));
    }

    /// Clique no título: ordena pela coluna (ascendente; de novo, descendente; na terceira,
    /// a ordem do dataset). Com `add` (Ctrl), a coluna entra na ordem depois das outras
    /// (ou troca de sentido, ou sai).
    fn sort_by(&mut self, ctx: &mut EventCtx<'_>, column: usize, add: bool) {
        let Some(c) = self.cols.get(column) else { return };
        let field = c.field.clone();
        // A ordem do dataset agora (o código pode ter mudado sem a grade saber).
        let current = self.data_set(ctx.ui).and_then(|ds| ctx.ui.get::<aylis_data::ClientDataSet>(ds)).and_then(|c| c.effective_index().ok()).map_or_else(Vec::new, |i| parse_sort(&i));
        if !same_sort(&current, &self.user_sort) {
            self.user_sort.clear();
            self.default_index = None;
        }
        let mut next = self.user_sort.clone();
        let at = next.iter().position(|(f, _)| f.eq_ignore_ascii_case(&field));
        if add {
            match at {
                None => next.push((field.clone(), false)),
                Some(i) if !next[i].1 => next[i].1 = true,
                Some(i) => {
                    next.remove(i);
                }
            }
        } else {
            next = match (next.as_slice(), at) {
                ([(_, false)], Some(_)) => vec![(field.clone(), true)],
                ([(_, true)], Some(_)) => Vec::new(),
                // A ordem do dataset já é só esta coluna, crescente: o clique vai direto ao
                // decrescente (senão o primeiro clique não mudaria nada).
                ([], _) if same_sort(&current, &[(field.clone(), false)]) => vec![(field.clone(), true)],
                _ => vec![(field.clone(), false)],
            };
        }
        if self.user_sort.is_empty() && !next.is_empty() {
            self.default_index = self.data_set(ctx.ui).and_then(|ds| ctx.ui.get::<aylis_data::ClientDataSet>(ds)).map(|c| (c.index_name.clone(), c.index_field_names.clone()));
        }
        let spec = sort_spec(&next);
        let restore = if next.is_empty() { self.default_index.take() } else { None };
        self.user_sort = next;
        self.with_data(ctx, |d| match restore {
            Some((name, fields)) => {
                d.set_index(&fields)?;
                if name.is_empty() { Ok(()) } else { d.set_index_name(&name) }
            }
            None => d.set_index(&spec),
        });
        ctx.fire("on_title_click", EventArgs::Text(field));
    }

    /// Abre o campo de filtro da coluna.
    fn start_filter(&mut self, ctx: &mut EventCtx<'_>, column: usize) {
        self.close_filter(ctx, true);
        let Some(c) = self.cols.get(column) else { return };
        let mut edit = Edit::new(self.filters.get(column).cloned().unwrap_or_default());
        edit.placeholder = aylis_core::messages::tf("Filtrar {0}", &[&c.title.to_lowercase()]);
        let editor = ctx.ui.add(edit, ctx.id);
        let x = self.column_x(column).max(self.indicator_w());
        if let Some(cb) = ctx.ui.control_mut(editor) {
            cb.set_rect(x + 2.0, HEADER + 2.0, c.width - 4.0, FILTER_ROW - 4.0);
        }
        ctx.request_focus(editor);
        self.filter_editor = Some((editor, column));
        ctx.repaint();
    }

    /// Fecha o campo de filtro; com `apply`, aplica os filtros de todas as colunas.
    fn close_filter(&mut self, ctx: &mut EventCtx<'_>, apply: bool) {
        let Some((editor, column)) = self.filter_editor.take() else { return };
        let text = ctx.ui.get::<Edit>(editor).map(|e| e.text.clone()).unwrap_or_default();
        ctx.ui.free(editor);
        ctx.request_focus(ctx.id);
        if apply {
            if let Some(slot) = self.filters.get_mut(column) {
                *slot = text.trim().to_string();
            }
            let parts: Vec<String> = self
                .cols
                .iter()
                .zip(&self.filters)
                .filter(|(_, t)| !t.is_empty())
                // Sem acento e sem maiúscula dos dois lados (o `aylis_core::text::fold`): "sao"
                // acha "São Paulo".
                .map(|(c, t)| format!("fold([{}]) like '%{}%'", c.field, aylis_core::text::fold(t).replace('\'', "''")))
                .collect();
            let filter = parts.join(" and ");
            self.with_data(ctx, |d| d.set_filter(&filter));
        }
        ctx.repaint();
    }

    /// Se a tela está perto do fim dos registros carregados, pede mais ao backend.
    fn fetch_if_needed(&mut self, ui: &mut Ui) {
        if !self.cache.has_more {
            return;
        }
        let visible = self.visible_rows();
        if self.top_row + visible * 2 >= self.cache.count {
            if let Some(ds) = self.data_set(ui) {
                let _ = data_set(ui, ds).fetch_more();
            }
        }
    }

    /// Depois de rolar: busca mais se preciso e prepara a pintura.
    fn refresh(&mut self, ctx: &mut EventCtx<'_>) {
        self.fill(ctx.ui, ctx.id);
        self.fetch_if_needed(ctx.ui);
        self.fill(ctx.ui, ctx.id);
        ctx.repaint();
    }

    /// Executa uma operação no dataset; erros vão para o evento `on_error`.
    fn with_data(&mut self, ctx: &mut EventCtx<'_>, f: impl FnOnce(&mut aylis_data::DataSet<'_>) -> aylis_data::DataResult) {
        let Some(ds) = self.data_set(ctx.ui) else { return };
        if let Err(e) = f(&mut data_set(ctx.ui, ds)) {
            ctx.fire("on_error", EventArgs::Text(e.message));
        }
        self.fill(ctx.ui, ctx.id);
        self.ensure_current_visible();
        self.refresh(ctx);
    }

    fn scroll_rows(&mut self, ctx: &mut EventCtx<'_>, rows: isize) {
        if !self.close_editor(ctx, true) {
            return;
        }
        let max = self.cache.count.saturating_sub(self.visible_rows());
        self.top_row = (self.top_row as isize + rows).clamp(0, max as isize) as usize;
        self.refresh(ctx);
    }

    // ----- editor de célula -----

    /// Abre o editor na célula atual. Numa célula que não edita (a grade, a coluna ou o campo
    /// só de leitura), Enter, F2 e o duplo clique abrem o editor só de leitura: o texto fica
    /// selecionável e copiável; digitar não abre.
    fn start_editor(&mut self, ctx: &mut EventCtx<'_>, typed: Option<&str>) {
        if self.cols.is_empty() || self.editor.is_some() {
            return;
        }
        let Some(source) = self.data_source else { return };
        let column = &self.cols[self.col];
        let field = column.field.clone();
        let link = FieldLink { data_source: Some(source), data_field: field.clone() };
        let locked = self.read_only || column.read_only || !link.can_modify(ctx.ui);
        if self.cache.current.is_none() || (locked && (typed.is_some() || column.check || column.kind != ColumnKind::Text)) {
            return;
        }
        self.ensure_current_visible();
        self.ensure_column_visible();
        self.fill(ctx.ui, ctx.id);
        let Some(current) = self.cache.current else { return };
        let x = self.column_x(self.col);
        let y = self.top() + (current - self.top_row) as f32 * self.row_height;
        let editor = ctx.ui.add(CellEditor { locked, ..CellEditor::new(source, &field) }, ctx.id);
        if let Some(cb) = ctx.ui.control_mut(editor) {
            cb.set_rect(x, y, self.cols[self.col].width, self.row_height);
        }
        if let Some(text) = typed {
            ctx.ui.send_message(editor, &EditorCommand::Type(text.to_string()));
        }
        ctx.request_focus(editor);
        self.editor = Some(editor);
        ctx.repaint();
    }

    /// Fecha o editor. Com `commit`, grava antes; se o valor for recusado, o editor fica
    /// aberto (com a borda de erro) e devolve `false`.
    fn close_editor(&mut self, ctx: &mut EventCtx<'_>, commit: bool) -> bool {
        let Some(editor) = self.editor else { return true };
        if commit {
            ctx.ui.send_message(editor, &EditorCommand::Commit);
            if ctx.ui.get::<CellEditor>(editor).is_some_and(|e| e.error().is_some()) {
                return false;
            }
        }
        self.editor = None;
        ctx.ui.free(editor);
        ctx.request_focus(ctx.id);
        self.fill(ctx.ui, ctx.id);
        ctx.repaint();
        true
    }

    fn editor_focused(&self, ctx: &EventCtx<'_>) -> bool {
        let root = ctx.ui.root_of(ctx.id);
        self.editor.is_some() && ctx.ui.focused(root) == self.editor
    }

    /// O que o leitor de tela e a automação veem: os cabeçalhos das colunas, cada linha
    /// preparada (as visíveis) e as células dela, com o que cada um é no grid.
    fn access_layout(&self) -> Vec<(AccessItem, GridTarget)> {
        let mut out = Vec::new();
        for (c, col) in self.cols.iter().enumerate() {
            let rect = RectF::new(self.column_x(c), 0.0, col.width, HEADER);
            out.push((AccessItem::new(ItemRole::ColumnHeader, col.title.clone()).rect(rect), GridTarget::Header(c)));
        }
        for (i, row) in self.cache.rows.iter().enumerate() {
            let index = self.cache.first + i;
            let y = self.top() + i as f32 * self.row_height;
            let name = row.texts.iter().filter(|t| !t.is_empty()).cloned().collect::<Vec<_>>().join(" · ");
            let selected = row.current || row.selected;
            let row_item = AccessItem::new(ItemRole::Row, name).rect(RectF::new(0.0, y, self.width(), self.row_height)).selected(selected);
            let parent = out.len();
            out.push((row_item, GridTarget::Row(index)));
            for (c, col) in self.cols.iter().enumerate() {
                let mut cell = AccessItem::new(ItemRole::Cell, row.texts.get(c).cloned().unwrap_or_default())
                    .rect(RectF::new(self.column_x(c), y, col.width, self.row_height))
                    .parent(parent)
                    .cell(index, c)
                    .selected(row.current && c == self.col);
                if let Some(Some(checked)) = row.checks.get(c) {
                    cell = cell.checked(*checked);
                }
                out.push((cell, GridTarget::Cell(index, c)));
            }
        }
        out
    }
}

/// No que um item do grid (para a automação) corresponde.
#[derive(Clone, Copy)]
enum GridTarget {
    Header(usize),
    Row(usize),
    Cell(usize, usize),
}

impl DataGrid {
    /// Arrastar da grade leva a chave do registro atual (os `key_fields`, separados por `;`,
    /// ou o número do registro), sem handler; o rótulo é o texto da primeira coluna.
    fn current_key(&self, ui: &Ui) -> Option<(String, String)> {
        let ds = self.data_set(ui)?;
        let v = view(ui, ds);
        if v.record_count() == 0 {
            return None;
        }
        let keys = match ui.get_prop(ds, "key_fields") {
            Ok(aylis_core::PropValue::String(s)) if !s.trim().is_empty() => s,
            _ => String::new(),
        };
        let data = if keys.is_empty() {
            v.rec_no()?.to_string()
        } else {
            keys.split(';').map(|k| v.text(k.trim()).unwrap_or_default()).collect::<Vec<_>>().join(";")
        };
        let label = v.field_names().first().and_then(|f| v.text(f).ok()).unwrap_or_else(|| data.clone());
        Some((data, label))
    }
}

/// A posição da rolagem pelo código (as propriedades `scroll_left` e `top_row`, só em
/// execução): o app que quer rolar de outro jeito lê e muda.
impl DataGrid {
    /// Quanto as colunas estão roladas, em unidades (só em execução).
    pub fn scroll_left(&self) -> f32 {
        self.scroll_x
    }

    /// Rola as colunas pelo código (ajustado ao que existe).
    pub fn set_scroll_left(&mut self, x: f32) {
        self.scroll_x = x;
        self.clamp_scroll_x();
    }

    /// A primeira linha que aparece (a partir de 0; só em execução).
    pub fn top_row_index(&self) -> i64 {
        self.top_row as i64
    }

    /// Rola as linhas pelo código: a linha `row` vira a primeira que aparece (ajustado ao que
    /// existe).
    pub fn set_top_row_index(&mut self, row: i64) {
        let max = self.cache.count.saturating_sub(self.visible_rows());
        self.top_row = row.clamp(0, max as i64) as usize;
    }

    /// A linha sob a altura `y` (nas coordenadas da grade, como o `position.y` do
    /// `on_pointer_move` dela): a posição entre os registros visíveis (0 = o primeiro do
    /// dataset com o filtro e a ordem), ou `None` no cabeçalho, no rodapé, na barra e abaixo da
    /// última linha. O mesmo número do `CellInfo::row` e do `value_at`.
    pub fn row_at(&self, y: f32) -> Option<usize> {
        let top = self.top();
        let bottom = self.height() - self.footer_height() - self.hbar_height();
        if !self.cache.active || y < top || y >= bottom {
            return None;
        }
        let row = self.top_row + ((y - top) / self.row_height) as usize;
        (row < self.cache.count).then_some(row)
    }

    /// O valor do campo `field` na linha `row` (a do [`row_at`](Self::row_at)), sem mexer no
    /// registro atual; `None` se a linha ou o campo não existem.
    pub fn value_at(&self, ui: &Ui, row: usize, field: &str) -> Option<aylis_data::Value> {
        let ds = self.data_set(ui)?;
        let v = view(ui, ds);
        if row >= v.visible_count() {
            return None;
        }
        v.visible_value(row, field).ok()
    }

    /// Com o mouse num título que tem `hint`, a dica da grade passa a ser a dele (a do
    /// controle volta fora dos títulos): o núcleo mostra a dica do controle sob o mouse.
    fn update_title_hint(&mut self, p: PointF) {
        let column_hint = (p.y >= 0.0 && p.y < HEADER).then(|| self.column_at(p.x)).flatten().map(|c| self.cols[c].hint.clone()).filter(|h| !h.is_empty());
        match column_hint {
            Some(hint) => {
                if self.own_hint.is_none() {
                    self.own_hint = Some(self.base.hint().to_string());
                }
                if self.base.hint() != hint {
                    self.base.set_hint(hint);
                }
            }
            None => {
                if let Some(own) = self.own_hint.take() {
                    self.base.set_hint(own);
                }
            }
        }
    }
}

impl Component for DataGrid {
    fn accessible_items(&self) -> Option<&dyn aylis_core::AccessibleItems> {
        Some(self)
    }

    fn accessible_items_mut(&mut self) -> Option<&mut dyn aylis_core::AccessibleItems> {
        Some(self)
    }

    fn drag_drop(&self) -> Option<&dyn aylis_core::DragDrop> {
        Some(self)
    }

    fn drag_drop_mut(&mut self) -> Option<&mut dyn aylis_core::DragDrop> {
        Some(self)
    }

    /// As linhas na vertical e as colunas de lado.
    fn scroll_axes(&self) -> (bool, bool) {
        (true, true)
    }

    fn class_name(&self) -> &'static str {
        "DataGrid"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        data_source: Option<ComponentId> => "Data",
        images: Option<ComponentId> => "Appearance",
        columns: String as RecordList(&GRID_COLUMN_RECORD) => "Data",
        row_height: f32 => "Appearance",
        read_only: bool => "Behavior",
        sortable: bool => "Behavior",
        column_move: bool => "Behavior",
        fixed_columns: u32 => "Appearance",
        filter_row: bool => "Behavior",
        multi_select: bool => "Behavior",
        footer: String as RecordList(&GRID_TOTAL_RECORD) => "Data",
        incremental_search: bool => "Behavior",
        cursor_color: Option<Color> => "Appearance",
        indicator: bool => "Appearance",
        highlight_current: bool => "Appearance",
        horizontal_wheel: HorizontalWheel => "Behavior",
        empty_text: String => "Appearance",
        frame: bool => "Appearance";
        events: on_error: String, on_title_click: String, on_selection_change: String, on_column_moved: ColumnLayout;
        computed: scroll_left: f32 => scroll_left / set_scroll_left,
            top_row: i64 => top_row_index / set_top_row_index;
        on_set: on_set
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("row_at", &[PropKind::Float], Some(PropKind::Int)).params(&["y"]),
            MethodInfo::new("value_at", &[PropKind::Int, PropKind::String], Some(PropKind::String)).params(&["row", "field"]),
        ];
        METHODS
    }

    /// Para as outras linguagens: `row_at(y)` devolve a linha (`Null` fora das linhas) e
    /// `value_at(row, field)` o valor como texto (o do JSON: número, `true`, data ISO; `Null`
    /// quando nulo ou fora).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        use aylis_core::PropValue;
        match method {
            "row_at" => {
                let y = args.first().and_then(|v| v.to_f32().ok()).ok_or("row_at(y): falta a altura")?;
                Ok(self.row_at(y).map_or(PropValue::Null, |r| PropValue::Int(r as i64)))
            }
            "value_at" => {
                let row = args.first().and_then(|v| v.to_i64().ok()).ok_or("value_at(row, field): falta a linha")?;
                let field = args.get(1).and_then(|v| v.to_str().ok()).ok_or("value_at(row, field): falta o campo")?;
                let value = usize::try_from(row).ok().and_then(|r| self.value_at(ctx.ui, r, field));
                Ok(match value {
                    None | Some(aylis_data::Value::Null) => PropValue::Null,
                    Some(aylis_data::Value::String(s)) => PropValue::String(s),
                    Some(v) => PropValue::String(match aylis_data::json::value_to_json(&v) {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    }),
                })
            }
            _ => Err(format!("DataGrid não tem o método {method}")),
        }
    }

    /// Colunas, rodapé e linha de filtro mudam o que é preparado (e quantas linhas cabem).
    fn notification(&mut self, freed: ComponentId) {
        if self.images == Some(freed) {
            self.images = None;
        }
        if self.data_source == Some(freed) {
            self.data_source = None;
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "columns" | "footer" | "filter_row" | "row_height" | "multi_select" | "data_source" | "indicator" | "top_row" | "scroll_left" | "images") {
            if name == "data_source" {
                self.cols.clear();
            }
            self.refresh(ctx);
        }
    }

    fn design_hooks(&self) -> Option<&dyn aylis_core::DesignHooks> {
        Some(self)
    }

    fn design_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::DesignHooks> {
        Some(self)
    }


    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        // Uma Column mudou, ou o designer mexeu no documento: refaz as colunas.
        let changed = message.downcast_ref::<ColumnsChanged>();
        if changed.is_some() || message.downcast_ref::<aylis_core::DesignChanged>().is_some() {
            self.changing_column = changed.map(|c| (c.0, c.1.clone()));
            self.cols.clear();
            self.fill(ctx.ui, ctx.id);
            self.changing_column = None;
            ctx.repaint();
            return;
        }
        if let Some(m) = message.downcast_ref::<MoveColumn>() {
            let gap = if m.to > m.from { m.to + 1 } else { m.to };
            self.move_column(ctx, m.from, gap);
            return;
        }
        if message.downcast_ref::<Refill>().is_some() {
            self.fill(ctx.ui, ctx.id);
            ctx.repaint();
            return;
        }
        let Some(m) = message.downcast_ref::<DataMessage>() else { return };
        if self.data_set(ctx.ui) != Some(m.data_set) || m.event == DataEvent::UpdateData {
            return;
        }
        if m.event == DataEvent::ActiveChanged {
            self.cols.clear();
            self.top_row = 0;
            self.selected.clear();
            self.anchor = None;
        }
        // Mudou como um campo aparece (`display_width` em execução): as colunas são refeitas dos
        // campos; os dados e o cursor ficam.
        if let DataEvent::FieldChanged(_) = m.event {
            self.cols.clear();
            self.totals_dirty = true;
            self.fill(ctx.ui, ctx.id);
            self.clamp_scroll_x();
            ctx.repaint();
            return;
        }
        // Os totais mudam com os dados (carga, post, exclusão, filtro), não com o cursor.
        if !matches!(m.event, DataEvent::Scrolled | DataEvent::StateChanged) {
            self.totals_dirty = true;
        }
        self.fill(ctx.ui, ctx.id);
        if matches!(m.event, DataEvent::Scrolled | DataEvent::DataSetChanged | DataEvent::StateChanged) {
            self.ensure_current_visible();
            self.fill(ctx.ui, ctx.id);
        }
        ctx.repaint();
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        self.clamp_scroll_x();
        self.refresh(ctx);
    }

    /// A roda rola as linhas; com o Shift (ou o touchpad de lado), as colunas
    /// (`horizontal_wheel`). Só fica com a roda quando rolou: na ponta (a última coluna, a
    /// última linha, ou sem nada para rolar), ela segue para o contêiner de fora, como nas
    /// listas e no navegador (pedido do autor em 16/11; antes a grade engolia a roda e a
    /// `ScrollBox` em volta parava). Quem quer que a grade segure sempre: `captures_wheel`.
    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let sideways = event.delta.x.abs() > event.delta.y.abs();
        let horizontal = match self.horizontal_wheel {
            HorizontalWheel::Shift => event.modifiers.shift || sideways,
            HorizontalWheel::Always => !event.touch || sideways,
            HorizontalWheel::None => false,
        };
        if horizontal {
            let delta = if event.delta.x != 0.0 { event.delta.x } else { event.delta.y };
            let before = self.scroll_x;
            self.scroll_x -= delta;
            self.clamp_scroll_x();
            if (self.scroll_x - before).abs() < 0.5 {
                self.scroll_x = before;
                return;
            }
            if !self.close_editor(ctx, true) {
                self.scroll_x = before;
                return;
            }
            self.refresh(ctx);
        } else if self.horizontal_wheel == HorizontalWheel::None && sideways {
            // O de lado é só do código (`scroll_left`): segue para fora.
            return;
        } else if event.lines {
            // Roda com dentes: o delta já vem multiplicado pelo passo do app
            // (`ScrollOptions::wheel_step`), então um passo = um dente = 3 linhas, qualquer que
            // seja o passo (antes, `delta/40*3`: com o passo 100, ~8 linhas por dente).
            let step = ctx.ui.scroll_options().wheel_step.max(1.0);
            let notches = -event.delta.y / step;
            let rows = (notches * 3.0).round();
            let rows = if rows == 0.0 && notches != 0.0 { notches.signum() } else { rows };
            let before = self.top_row;
            self.scroll_rows(ctx, rows as isize);
            if self.top_row == before {
                return;
            }
        } else {
            // Dedo e touchpad (deslocamento em pontos): as linhas acompanham o deslocamento,
            // uma a cada row_height, como as outras listas rolam os mesmos pontos.
            self.touch_rest -= event.delta.y;
            let rows = (self.touch_rest / self.row_height).trunc();
            self.touch_rest -= rows * self.row_height;
            let before = self.top_row;
            self.scroll_rows(ctx, rows as isize);
            if rows != 0.0 && self.top_row == before {
                // Chegou ao fim: não trata (a inércia para).
                self.touch_rest = 0.0;
                return;
            }
        }
        ctx.set_handled();
    }

    // Cabeçalho: ordena (se `sortable`); linha ou célula: vai para o registro (e a coluna),
    // como o clique.

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let p = event.position;
        if !self.close_editor(ctx, true) {
            return;
        }
        if self.filter_editor.is_some() {
            self.close_filter(ctx, true);
        }
        // cabeçalho: redimensionar coluna (duplo clique no separador: o tamanho do maior
        // texto) ou ordenar
        if p.y < HEADER {
            if let Some(column) = self.border_at(p.x) {
                if event.click_count == 2 {
                    self.fit_column(ctx.ui, column);
                    ctx.repaint();
                    return;
                }
                self.drag = Drag::Resize { column, start_x: p.x, start_width: self.cols[column].width };
            } else if let Some(column) = self.column_at(p.x) {
                // Ordena ao soltar (se não arrastou a coluna).
                self.drag = Drag::Title { column, start_x: p.x, x: p.x, moved: false, add: event.modifiers.ctrl || event.modifiers.meta };
            }
            return;
        }
        // linha de filtro
        if p.y < self.top() {
            if let Some(column) = self.column_at(p.x) {
                self.start_filter(ctx, column);
            }
            return;
        }
        // barra de rolagem
        if self.has_vbar() && self.track().contains(p) {
            let thumb = self.thumb();
            if thumb.contains(p) {
                self.drag = Drag::Thumb { offset: p.y - thumb.y };
            } else {
                let page = self.visible_rows() as isize;
                self.scroll_rows(ctx, if p.y < thumb.y { -page } else { page });
            }
            return;
        }
        // barra horizontal
        if self.has_hbar() && self.htrack().contains(p) {
            let thumb = self.hthumb();
            if thumb.contains(p) {
                self.drag = Drag::HThumb { offset: p.x - thumb.x };
            } else {
                let page = self.data_width() * 0.9;
                self.scroll_x += if p.x < thumb.x { -page } else { page };
                self.clamp_scroll_x();
                ctx.repaint();
            }
            return;
        }
        // rodapé
        if p.y >= self.height() - self.footer_height() - self.hbar_height() {
            return;
        }
        // célula
        let row = self.top_row + ((p.y - self.top()) / self.row_height) as usize;
        if row >= self.cache.count {
            return;
        }
        let column = self.column_at(p.x);
        if let Some(col) = column {
            self.col = col;
        }
        self.with_data(ctx, |d| d.move_to(row));
        if self.multi_select {
            self.select_row(ctx, row, event.modifiers.ctrl, event.modifiers.shift);
        }
        // Na caixa de marcar: alterna (só se o cursor chegou à linha).
        if let Some(col) = column.filter(|&c| self.cols[c].check) {
            let y = self.top() + (row - self.top_row) as f32 * self.row_height;
            let cell = RectF::new(self.column_x(col), y, self.cols[col].width, self.row_height);
            if Self::check_rect(cell).inflate(3.0).contains(p) && self.cache.current == Some(row) {
                self.toggle_check(ctx, col);
            }
            return;
        }
        if event.click_count == 2 {
            self.start_editor(ctx, None);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        match self.drag {
            Drag::Resize { column, start_x, start_width } => {
                if let Some(c) = self.cols.get_mut(column) {
                    c.width = (start_width + p.x - start_x).max(MIN_COLUMN);
                }
                self.clamp_scroll_x();
                ctx.repaint();
            }
            Drag::HThumb { offset } => {
                let track = self.htrack();
                let thumb = self.hthumb();
                let room = (track.width - thumb.width).max(1.0);
                let fraction = ((p.x - offset - track.x) / room).clamp(0.0, 1.0);
                self.scroll_x = (self.content_width() - self.data_width()).max(0.0) * fraction;
                ctx.repaint();
            }
            Drag::Thumb { offset } => {
                let track = self.track();
                let thumb = self.thumb();
                let room = (track.height - thumb.height).max(1.0);
                let fraction = ((p.y - offset - track.y) / room).clamp(0.0, 1.0);
                let max_top = self.scroll_total().saturating_sub(self.visible_rows());
                self.top_row = ((max_top as f32) * fraction).round() as usize;
                self.refresh(ctx);
            }
            Drag::Title { column, start_x, moved, add, .. } => {
                let moved = moved || (self.column_move && (p.x - start_x).abs() > MOVE_THRESHOLD);
                self.drag = Drag::Title { column, start_x, x: p.x, moved, add };
                if moved {
                    ctx.repaint();
                }
            }
            Drag::None => {
                let cursor = if p.y < HEADER && self.border_at(p.x).is_some() {
                    CursorShape::ResizeHorizontal
                } else {
                    CursorShape::Default
                };
                if cursor != self.base.cursor() {
                    self.base.set_cursor(cursor);
                }
                self.update_title_hint(p);
                let top = self.top();
                let hovered = (p.y >= top).then(|| self.top_row + ((p.y - top) / self.row_height) as usize);
                if hovered != self.hovered_row {
                    self.hovered_row = hovered;
                    ctx.repaint();
                }
            }
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        let drag = std::mem::take(&mut self.drag);
        if let Drag::Title { column, x, moved, add, .. } = drag {
            if moved {
                let gap = self.drop_gap(column, x);
                self.move_column(ctx, column, gap);
            } else if self.sortable {
                self.sort_by(ctx, column, add);
            } else if let Some(c) = self.cols.get(column) {
                // Sem `sortable`, o clique só avisa (o app que ordena por conta própria).
                ctx.fire("on_title_click", EventArgs::Text(c.field.clone()));
            }
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(own) = self.own_hint.take() {
            self.base.set_hint(own);
        }
        self.hovered_row = None;
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let ctrl = event.modifiers.ctrl;
        // Teclas que sobem do campo de filtro.
        if let Some((editor, _)) = self.filter_editor {
            let root = ctx.ui.root_of(ctx.id);
            if ctx.ui.focused(root) == Some(editor) {
                match event.key {
                    Key::Enter => self.close_filter(ctx, true),
                    Key::Escape => self.close_filter(ctx, false),
                    _ => return,
                }
                ctx.set_handled();
                return;
            }
        }
        // Teclas que sobem do editor de célula.
        if self.editor_focused(ctx) {
            match event.key {
                Key::Enter => {
                    self.close_editor(ctx, true);
                }
                Key::Escape => {
                    self.close_editor(ctx, false);
                    self.with_data(ctx, |d| d.cancel());
                }
                Key::Tab => {
                    if self.close_editor(ctx, true) {
                        let last = self.cols.len().saturating_sub(1);
                        self.col = if event.modifiers.shift { self.col.saturating_sub(1) } else { (self.col + 1).min(last) };
                        self.start_editor(ctx, None);
                    }
                }
                Key::Up | Key::Down => {
                    if self.close_editor(ctx, true) {
                        let down = event.key == Key::Down;
                        self.with_data(ctx, |d| if down { d.next() } else { d.prior() });
                    }
                }
                _ => return,
            }
            ctx.set_handled();
            return;
        }

        let page = self.visible_rows();
        let current = self.cache.current.unwrap_or(0);
        let last_col = self.cols.len().saturating_sub(1);
        let shift = event.modifiers.shift;
        // Seleção de várias linhas pelo teclado.
        if self.multi_select {
            match event.key {
                Key::Char('a') | Key::Char('A') if ctrl => {
                    self.select_all(ctx);
                    ctx.set_handled();
                    return;
                }
                Key::Space | Key::Char(' ') if ctrl => {
                    self.select_row(ctx, current, true, false);
                    ctx.set_handled();
                    return;
                }
                Key::Up | Key::Down | Key::PageUp | Key::PageDown => {
                    let target = match event.key {
                        Key::Up => current.saturating_sub(1),
                        Key::Down => current + 1,
                        Key::PageUp => current.saturating_sub(page),
                        _ => current + page,
                    }
                    .min(self.cache.count.saturating_sub(1));
                    self.with_data(ctx, |d| d.move_to(target));
                    self.select_row(ctx, target, false, shift);
                    ctx.set_handled();
                    return;
                }
                _ => {}
            }
        }
        // Busca incremental: Backspace apaga, Esc limpa.
        if self.incremental_search && !self.search.is_empty() && ctx.ui.time() - self.search_time <= SEARCH_PAUSE {
            match event.key {
                Key::Backspace => {
                    self.search.pop();
                    self.search_time = ctx.ui.time();
                    if !self.search.is_empty() {
                        self.search_now(ctx, false);
                    }
                    ctx.repaint();
                    ctx.set_handled();
                    return;
                }
                Key::Escape => {
                    self.search.clear();
                    ctx.repaint();
                    ctx.set_handled();
                    return;
                }
                _ => {}
            }
        }
        // Ctrl+C: o texto da célula atual (como aparece na grade).
        if ctrl && matches!(event.key, Key::Char('c') | Key::Char('C') | Key::Insert) {
            if let Some(text) = self.current_cell_text(ctx.ui) {
                aylis_core::clipboard::set_text(ctx.ui, &text);
            }
            ctx.set_handled();
            return;
        }
        // Espaço numa coluna lógica alterna a caixa.
        if matches!(event.key, Key::Space | Key::Char(' ')) && !ctrl && self.cols.get(self.col).is_some_and(|c| c.check) {
            self.toggle_check(ctx, self.col);
            ctx.set_handled();
            return;
        }
        match event.key {
            Key::Up => self.with_data(ctx, |d| d.prior()),
            Key::Down => self.with_data(ctx, |d| d.next()),
            Key::PageUp => self.with_data(ctx, |d| d.move_to(current.saturating_sub(page))),
            Key::PageDown => self.with_data(ctx, |d| d.move_to(current + page)),
            Key::Home if ctrl => self.with_data(ctx, |d| d.first()),
            Key::End if ctrl => self.with_data(ctx, |d| d.last()),
            Key::Home | Key::Left | Key::Right | Key::End => {
                self.col = match event.key {
                    Key::Home => 0,
                    Key::End => last_col,
                    Key::Left => self.col.saturating_sub(1),
                    _ => (self.col + 1).min(last_col),
                };
                self.ensure_column_visible();
                ctx.repaint();
            }
            Key::Enter | Key::F(2) => self.start_editor(ctx, None),
            Key::Insert if !self.read_only => self.with_data(ctx, |d| d.insert()),
            Key::Delete if ctrl && !self.read_only => self.with_data(ctx, |d| d.delete()),
            Key::Escape if self.cache.editing => self.with_data(ctx, |d| d.cancel()),
            _ => return,
        }
        ctx.set_handled();
    }

    /// Começar a digitar abre o editor com a tecla digitada, ou, com
    /// `incremental_search`, procura na coluna atual.
    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        if self.editor.is_some() || self.filter_editor.is_some() {
            return;
        }
        // Espaço numa coluna lógica é a tecla da caixa (tratada no key_down).
        if text == " " && self.cols.get(self.col).is_some_and(|c| c.check) {
            ctx.set_handled();
            return;
        }
        if self.incremental_search {
            let continuing = !self.search.is_empty() && ctx.ui.time() - self.search_time <= SEARCH_PAUSE;
            if !text.trim().is_empty() || continuing {
                self.search_typed(ctx, text);
                ctx.set_handled();
            }
        } else if !text.trim().is_empty() {
            self.start_editor(ctx, Some(text));
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let border = theme.color("border");
        let text_color = theme.color("text");
        let muted = theme.color("text_muted");
        let look = GridLook::of(ctx);
        if self.frame {
            canvas.fill_rect(r, 6.0, &Brush::Solid(theme.color("surface")));
        }
        {
            let mut header = ctx.text_style();
            header.font.weight = aylis_core::FontWeight::SemiBold;
            *self.text_style.borrow_mut() = Some((ctx.text_style(), header));
        }

        canvas.save();
        canvas.clip_rect(if self.frame { r.inflate(-1.0) } else { r });
        let data_right = self.indicator_w() + self.data_width();

        // ----- cabeçalho -----
        let top = self.top();
        canvas.fill_rect(RectF::new(0.0, 0.0, r.width, top), 0.0, &Brush::Solid(look.header_fill.unwrap_or(theme.color("background"))));
        let header_line = look.header_line.unwrap_or(border);
        let mut header_style = ctx.text_style();
        header_style.color = look.header_text.unwrap_or(muted);
        header_style.font.weight = aylis_core::FontWeight::SemiBold;
        let fixed_right = self.indicator_w() + self.fixed_width();
        let fixed = self.fixed_columns as usize;
        // Primeiro as colunas que rolam (recortadas depois das fixas), depois as fixas.
        for pass in [false, true] {
            canvas.save();
            if pass {
                canvas.clip_rect(RectF::new(self.indicator_w(), 0.0, (fixed_right - self.indicator_w()).max(0.0), top));
            } else {
                canvas.clip_rect(RectF::new(fixed_right, 0.0, (data_right - fixed_right).max(0.0), top));
            }
            for (i, c) in self.cols.iter().enumerate() {
                if (i < fixed) != pass {
                    continue;
                }
                let x = self.column_x(i);
                if x > data_right || x + c.width < self.indicator_w() {
                    continue;
                }
                header_style.h_align = c.align;
                let position = self.sort.iter().position(|(f, _)| f.eq_ignore_ascii_case(&c.field));
                let sorted = position.map(|i| self.sort[i].1);
                // Com mais de uma coluna na ordem, o número da posição ao lado da seta.
                let number = position.filter(|_| self.sort.len() > 1).map(|i| (i + 1).to_string());
                let arrow_room = if sorted.is_some() { 14.0 } else { 0.0 } + if number.is_some() { 10.0 } else { 0.0 };
                canvas.draw_text(&c.title, RectF::new(x + 8.0, 0.0, c.width - 16.0 - arrow_room, HEADER), &header_style);
                if let Some(n) = &number {
                    let mut ns = header_style.clone();
                    ns.font.size = (ns.font.size - 3.0).max(8.0);
                    ns.color = theme.color("primary");
                    ns.h_align = TextAlign::Center;
                    canvas.draw_text(n, RectF::new(x + c.width - 30.0, 0.0, 10.0, HEADER), &ns);
                }
                if let Some(desc) = sorted {
                    let ax = x + c.width - 14.0;
                    let ay = HEADER / 2.0;
                    let s = Stroke::new(theme.color("primary"), 1.6);
                    let d = if desc { 2.5 } else { -2.5 };
                    canvas.draw_line(PointF::new(ax - 4.0, ay - d), PointF::new(ax, ay + d), &s);
                    canvas.draw_line(PointF::new(ax, ay + d), PointF::new(ax + 4.0, ay - d), &s);
                }
                let edge = (x + c.width).round() - 0.5;
                canvas.draw_line(PointF::new(edge, 8.0), PointF::new(edge, HEADER - 8.0), &Stroke::new(header_line, 1.0));
                if self.filter_row {
                    let cell = RectF::new(x + 2.0, HEADER + 2.0, c.width - 4.0, FILTER_ROW - 4.0);
                    canvas.fill_rect(cell, 4.0, &Brush::Solid(theme.color("surface")));
                    canvas.stroke_rect(cell, 4.0, &Stroke::new(border, 1.0));
                    let text = self.filters.get(i).map_or("", String::as_str);
                    let mut fs = ctx.text_style();
                    fs.h_align = TextAlign::Leading;
                    fs.font.size = (fs.font.size - 1.0).max(9.0);
                    if text.is_empty() {
                        fs.color = muted.with_opacity(0.7);
                        canvas.draw_text(&ctx.ui.tr("filtrar…"), cell.inflate(-6.0), &fs);
                    } else {
                        canvas.draw_text(text, cell.inflate(-6.0), &fs);
                    }
                }
            }
            canvas.restore();
        }
        canvas.draw_line(PointF::new(0.0, top - 0.5), PointF::new(r.width, top - 0.5), &Stroke::new(header_line, 1.0));

        // ----- coluna sendo arrastada: o título segue o mouse e uma barra marca onde entra -----
        if let Drag::Title { column, start_x, x, moved: true, .. } = self.drag {
            if let Some(c) = self.cols.get(column) {
                let gap = self.drop_gap(column, x);
                let gx = if gap < self.cols.len() { self.column_x(gap) } else { self.column_x(self.cols.len() - 1) + self.cols[self.cols.len() - 1].width };
                let primary = theme.color("primary");
                canvas.fill_rect(RectF::new(gx - 1.5, 0.0, 3.0, r.height), 1.5, &Brush::Solid(primary));
                let ghost = RectF::new(self.column_x(column) + x - start_x, 2.0, c.width, HEADER - 4.0);
                canvas.fill_rect(ghost, 6.0, &Brush::Solid(theme.color("surface")));
                canvas.fill_rect(ghost, 6.0, &Brush::Solid(primary.with_opacity(0.16)));
                canvas.stroke_rect(ghost, 6.0, &Stroke::new(primary, 1.0));
                header_style.h_align = c.align;
                header_style.color = text_color;
                canvas.draw_text(&c.title, RectF::new(ghost.x + 8.0, 0.0, c.width - 16.0, HEADER), &header_style);
            }
        }

        // ----- linhas -----
        if self.cache.active && self.cache.count == 0 {
            let mut empty = ctx.text_style();
            empty.color = muted;
            empty.h_align = TextAlign::Center;
            let text = if self.empty_text.is_empty() { ctx.ui.tr("Sem registros") } else { self.empty_text.clone() };
            canvas.draw_text(&text, RectF::new(0.0, top, r.width, self.row_height * 2.0), &empty);
        }
        // As cores das linhas: o estilo `DataGrid.row` (o `focused` é a linha atual, o `checked`
        // a selecionada, o `hover` a do mouse) e o `DataGrid.stripe` (a zebra); sem eles, as de
        // sempre.
        let selection = look.current_fill.unwrap_or(theme.color("primary").with_opacity(0.12));
        let selected_fill = look.selected_fill.unwrap_or(theme.color("primary").with_opacity(0.22));
        let stripe = look.stripe.unwrap_or(theme.color("text").with_opacity(0.025));
        let hover = look.hover_fill.unwrap_or(theme.color("text").with_opacity(0.05));
        let surface = look.row_fill.unwrap_or(theme.color("surface"));
        for (k, row) in self.cache.rows.iter().enumerate() {
            let index = self.cache.first + k;
            let y = top + k as f32 * self.row_height;
            if y >= r.height {
                break;
            }
            let line = RectF::new(0.0, y, data_right, self.row_height);
            // Fundo opaco: as colunas fixas cobrem as que rolam por baixo delas.
            let fill = row.row_style.background.unwrap_or(surface);
            if row.row_style.background.is_some() || look.row_fill.is_some() {
                canvas.fill_rect(line, 0.0, &Brush::Solid(fill));
            }
            let mut overlays = Vec::new();
            if index % 2 == 1 {
                overlays.push(stripe);
            }
            if row.selected {
                overlays.push(selected_fill);
            }
            if row.current && self.highlight_current {
                overlays.push(selection);
            } else if self.hovered_row == Some(index) {
                overlays.push(hover);
            }
            for o in &overlays {
                canvas.fill_rect(line, 0.0, &Brush::Solid(*o));
            }
            let mut row_text = ctx.text_style();
            // O texto da linha: o do app (`on_draw_cell`), senão o do estado no estilo.
            let state_text = if row.current && self.highlight_current { look.current_text } else { None }
                .or(if row.selected { look.selected_text } else { None })
                .or(look.row_text);
            row_text.color = if row.error { theme.color("danger") } else { row.row_style.color.or(state_text).unwrap_or(text_color) };
            if row.row_style.bold {
                row_text.font.weight = aylis_core::FontWeight::Bold;
            }
            row_text.font.italic = row.row_style.italic;
            for pass in [false, true] {
                canvas.save();
                if pass {
                    if fixed == 0 {
                        canvas.restore();
                        continue;
                    }
                    let area = RectF::new(self.indicator_w(), y, (fixed_right - self.indicator_w()).max(0.0), self.row_height);
                    canvas.clip_rect(area);
                    canvas.fill_rect(area, 0.0, &Brush::Solid(fill));
                    for o in &overlays {
                        canvas.fill_rect(area, 0.0, &Brush::Solid(*o));
                    }
                } else {
                    canvas.clip_rect(RectF::new(fixed_right, y, (data_right - fixed_right).max(0.0), self.row_height));
                }
                for (i, c) in self.cols.iter().enumerate() {
                    if (i < fixed) != pass {
                        continue;
                    }
                    let x = self.column_x(i);
                    if x > data_right || x + c.width < self.indicator_w() {
                        continue;
                    }
                    let mut style = row_text.clone();
                    style.h_align = c.align;
                    let cell_style = row.cell_styles.get(i);
                    let cell = RectF::new(x, y, c.width, self.row_height);
                    if let Some(bg) = c.color {
                        canvas.fill_rect(cell, 0.0, &Brush::Solid(bg));
                    }
                    if let Some(color) = c.text_color {
                        style.color = color;
                    }
                    if let Some(bg) = cell_style.and_then(|s| s.background) {
                        canvas.fill_rect(cell, 0.0, &Brush::Solid(bg));
                    }
                    if let Some(s) = cell_style {
                        if let Some(color) = s.color {
                            style.color = color;
                        }
                        if s.bold {
                            style.font.weight = aylis_core::FontWeight::Bold;
                        }
                        if s.italic {
                            style.font.italic = true;
                        }
                    }
                    if let Some(paint) = cell_style.and_then(|s| s.paint.clone()) {
                        canvas.save();
                        canvas.clip_rect(cell);
                        paint(canvas, cell);
                        canvas.restore();
                    } else if c.check {
                        let on = row.checks.get(i).copied().flatten();
                        draw_check(canvas, Self::check_rect(cell), on, ctx);
                    } else if let Some(picture) = row.pictures.get(i).cloned().flatten() {
                        // O ícone (na cor do texto da célula) ou a imagem, no alinhamento da coluna.
                        let inner = cell.inflate(-4.0);
                        match picture {
                            Picture::Icon(icon) => {
                                let side = inner.width.min(inner.height).min(24.0);
                                let x = match c.align {
                                    TextAlign::Center => inner.x + (inner.width - side) / 2.0,
                                    TextAlign::Trailing => inner.right() - side,
                                    _ => inner.x + 4.0,
                                };
                                icon.draw(canvas, RectF::new(x, inner.y + (inner.height - side) / 2.0, side, side), style.color);
                            }
                            Picture::Image(image) => {
                                if let Some(size) = canvas.image_size(&image) {
                                    let (src, dest) = aylis_core::ImageFit::Contain.place(size, inner);
                                    canvas.save();
                                    canvas.clip_rect(cell);
                                    canvas.draw_image(&image, src, dest, 1.0);
                                    canvas.restore();
                                }
                            }
                        }
                    } else if c.kind != ColumnKind::Text {
                        // Coluna de ícone ou de imagem sem figura (o item sem ícone, o campo
                        // vazio): fica vazia, sem o número ou os bytes como texto.
                    } else {
                        let text = cell_style.and_then(|s| s.text.as_deref()).unwrap_or_else(|| row.texts.get(i).map_or("", String::as_str));
                        canvas.draw_text(text, RectF::new(x + 8.0, y, c.width - 16.0, self.row_height), &style);
                    }
                    // A borda da célula atual é o cursor da grade, não o anel de foco: aparece
                    // também com o foco vindo do clique (que tira o `focused` do `ctx.state`).
                    let focused = ctx.ui.control(ctx.id).is_some_and(|c| c.state().focused);
                    if row.current && i == self.col && focused && self.editor.is_none() {
                        // A moldura da célula atual: o `cursor_color` da grade, senão o estilo
                        // `DataGrid.cursor` (cor, largura e raio), senão 2 na cor de destaque.
                        let color = self.cursor_color.or(look.cursor_color).unwrap_or(theme.color("primary"));
                        let width = look.cursor_width.unwrap_or(2.0);
                        if width > 0.0 {
                            let inset = width / 2.0;
                            let cell = RectF::new(x + inset, y + inset, c.width - width, self.row_height - width);
                            canvas.stroke_rect(cell, look.cursor_radius.unwrap_or(3.0), &Stroke::new(color, width));
                        }
                    }
                }
                canvas.restore();
            }
            if fixed > 0 {
                let edge = fixed_right.round() - 0.5;
                canvas.draw_line(PointF::new(edge, y), PointF::new(edge, y + self.row_height), &Stroke::new(border, 1.0));
            }

            if self.indicator {
                draw_indicator(canvas, row, self.cache.editing, PointF::new(self.indicator_w() / 2.0, y + self.row_height / 2.0), ctx);
            }
            let base = (y + self.row_height).round() - 0.5;
            canvas.draw_line(PointF::new(0.0, base), PointF::new(data_right, base), &Stroke::new(border.with_opacity(0.6), 1.0));
        }

        // ----- rodapé com totais -----
        let footer = self.footer_height();
        let hbar = self.hbar_height();
        if footer > 0.0 {
            let fy = r.height - hbar - footer;
            canvas.fill_rect(RectF::new(0.0, fy, r.width, footer), 0.0, &Brush::Solid(theme.color("background")));
            canvas.draw_line(PointF::new(0.0, fy + 0.5), PointF::new(r.width, fy + 0.5), &Stroke::new(border, 1.0));
            let mut fs = ctx.text_style();
            fs.font.weight = aylis_core::FontWeight::SemiBold;
            for pass in [false, true] {
                canvas.save();
                if pass {
                    canvas.clip_rect(RectF::new(self.indicator_w(), fy, (fixed_right - self.indicator_w()).max(0.0), footer));
                } else {
                    canvas.clip_rect(RectF::new(fixed_right, fy, (data_right - fixed_right).max(0.0), footer));
                }
                for (i, c) in self.cols.iter().enumerate() {
                    if (i < fixed) != pass {
                        continue;
                    }
                    let Some(text) = self.totals.get(i).and_then(Option::as_deref) else { continue };
                    fs.h_align = if c.check { TextAlign::Center } else { c.align };
                    canvas.draw_text(text, RectF::new(self.column_x(i) + 8.0, fy, c.width - 16.0, footer), &fs);
                }
                canvas.restore();
            }
            let mut sigma = ctx.text_style();
            sigma.color = muted;
            sigma.h_align = TextAlign::Center;
            canvas.draw_text("Σ", RectF::new(0.0, fy, self.indicator_w(), footer), &sigma);
        }

        // ----- busca incremental -----
        let searching = !self.search.is_empty() && ctx.time - self.search_time <= SEARCH_PAUSE;
        if searching {
            // Some sozinha depois da pausa.
            ctx.request_frame_at(self.search_time + SEARCH_PAUSE + 0.01);
            let mut ts = ctx.text_style();
            ts.h_align = TextAlign::Leading;
            let label = aylis_core::messages::tf("Buscar: {0}", &[&self.search]);
            let w = canvas.measure_text(&label, &ts).width + 20.0;
            let chip = RectF::new(r.width - w - self.bar() - 12.0, r.height - hbar - footer - self.row_height - 8.0, w, self.row_height);
            canvas.fill_rect(chip, chip.height / 2.0, &Brush::Solid(theme.color("text").with_opacity(0.85)));
            ts.color = theme.color("surface");
            canvas.draw_text(&label, chip.inflate(-10.0), &ts);
        }

        // ----- aviso de dados parciais: a ordem, o filtro ou a procura valeu só para os
        // registros carregados (há mais no backend e ninguém tratou o `on_server_query`) -----
        if !self.cache.notice.is_empty() {
            let mut ts = ctx.text_style();
            ts.h_align = TextAlign::Leading;
            ts.font.size = (ts.font.size - 1.0).max(9.0);
            let warning = theme.color("warning");
            let band = RectF::new(self.indicator_w(), r.height - hbar - footer - self.row_height, (r.width - self.indicator_w() - self.bar() - 4.0).max(0.0), self.row_height);
            canvas.fill_rect(band, 0.0, &Brush::Solid(theme.color("surface")));
            canvas.fill_rect(band, 0.0, &Brush::Solid(warning.with_opacity(0.16)));
            canvas.fill_rect(RectF::new(band.x, band.y, 3.0, band.height), 0.0, &Brush::Solid(warning));
            ts.color = theme.color("text");
            canvas.draw_text(&format!("⚠  {}", self.cache.notice), RectF::new(band.x + 10.0, band.y, band.width - 14.0, band.height), &ts);
        }

        // ----- barra de rolagem -----
        // Pelo estilo `ScrollBar` do tema (o `hover` dele vale arrastando); o que ele não diz,
        // como era: 10 de largura, o trilho quase invisível, o polegar apagado, raio de meia
        // largura.
        let paint_bar = |canvas: &mut dyn Canvas, track: RectF, thumb: RectF, dragging: bool| {
            let bar = aylis_controls::scroll_bar::resolve(
                ctx,
                dragging,
                aylis_controls::scroll_bar::Defaults {
                    width: SCROLLBAR,
                    track: Some(theme.color("text").with_opacity(0.04)),
                    thumb: muted.with_opacity(if dragging { 0.45 } else { 0.28 }),
                },
            );
            let radius = bar.radius.unwrap_or(self.bar() / 2.0);
            if let Some(color) = bar.track.filter(|c| !c.is_transparent()) {
                canvas.fill_rect(track, radius, &Brush::Solid(color));
            }
            canvas.fill_rect(thumb.inflate(-1.0), radius, &Brush::Solid(bar.thumb));
        };
        if self.has_vbar() {
            paint_bar(canvas, self.track(), self.thumb(), matches!(self.drag, Drag::Thumb { .. }));
        }
        if hbar > 0.0 {
            let track = self.htrack();
            canvas.fill_rect(RectF::new(0.0, track.y - 2.0, r.width, hbar + 2.0), 0.0, &Brush::Solid(theme.color("surface")));
            paint_bar(canvas, track, self.hthumb(), matches!(self.drag, Drag::HThumb { .. }));
        }
        canvas.restore();
        if self.frame {
            canvas.stroke_rect(r, 6.0, &Stroke::new(if ctx.state.focused { theme.color("focus_ring") } else { border }, 1.0));
        }
    }
}

impl aylis_core::AccessibleItems for DataGrid {
    fn access_items(&self, _: &Ui, _: ComponentId) -> Vec<AccessItem> {
        self.access_layout().into_iter().map(|(item, _)| item).collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, action: ItemAction) -> bool {
        let Some((_, target)) = self.access_layout().into_iter().nth(index) else { return false };
        if !self.close_editor(ctx, true) {
            return false;
        }
        match target {
            GridTarget::Header(c) => {
                if action == ItemAction::Select {
                    return false;
                }
                if self.sortable {
                    self.sort_by(ctx, c, false);
                } else if let Some(col) = self.cols.get(c) {
                    ctx.fire("on_title_click", EventArgs::Text(col.field.clone()));
                }
            }
            GridTarget::Row(row) | GridTarget::Cell(row, _) => {
                if let GridTarget::Cell(_, c) = target {
                    self.col = c;
                }
                self.with_data(ctx, |d| d.move_to(row));
                if self.multi_select {
                    self.select_row(ctx, row, false, false);
                }
                ctx.repaint();
            }
        }
        true
    }
}

impl aylis_core::DragDrop for DataGrid {
    fn drag_item(&self, ui: &Ui) -> Option<(String, String)> {
        self.current_key(ui)
    }
}

/// As partes da grade pelo tema: os sub-estilos `DataGrid.header` (o fundo,
/// o texto e a `border_color` das divisões), `DataGrid.row` (o fundo e o texto das linhas; os
/// estados `hover`, `checked` = selecionada e `focused` = a atual), `DataGrid.stripe` (o
/// `fill` da zebra; `transparent` tira) e `DataGrid.cursor` (a `border_color`, a
/// `border_width` e o `radius` da moldura da célula atual), cada um também por variante
/// (`DataGrid.row.compacta`). O que o tema não diz fica como sempre foi.
#[derive(Default)]
struct GridLook {
    header_fill: Option<Color>,
    header_text: Option<Color>,
    header_line: Option<Color>,
    row_fill: Option<Color>,
    row_text: Option<Color>,
    hover_fill: Option<Color>,
    selected_fill: Option<Color>,
    selected_text: Option<Color>,
    current_fill: Option<Color>,
    current_text: Option<Color>,
    stripe: Option<Color>,
    cursor_color: Option<Color>,
    cursor_width: Option<f32>,
    cursor_radius: Option<f32>,
}

impl GridLook {
    fn of(ctx: &PaintCtx<'_>) -> GridLook {
        let theme = ctx.theme;
        let variant = ctx.variant();
        // O sub-estilo e, por cima, o da variante (sem nenhum dos dois: `None`).
        let part = |name: &str| -> Option<aylis_core::Style> {
            let base = theme.style(&format!("DataGrid.{name}"));
            let own = (!variant.is_empty()).then(|| theme.style(&format!("DataGrid.{name}.{variant}"))).flatten();
            match (base, own) {
                (None, None) => None,
                (base, own) => {
                    let mut style = base.cloned().unwrap_or_default();
                    if let Some(own) = own {
                        style.merge(own);
                    }
                    Some(style)
                }
            }
        };
        let color = |c: &Option<aylis_core::ColorRef>| c.as_ref().map(|c| theme.color_of(c));
        let mut look = GridLook::default();
        if let Some(h) = part("header") {
            look.header_fill = color(&h.base.fill);
            look.header_text = color(&h.base.text);
            look.header_line = color(&h.base.border_color);
        }
        if let Some(row) = part("row") {
            look.row_fill = color(&row.base.fill);
            look.row_text = color(&row.base.text);
            look.hover_fill = color(&row.hover.fill);
            look.selected_fill = color(&row.checked.fill);
            look.selected_text = color(&row.checked.text);
            look.current_fill = color(&row.focused.fill);
            look.current_text = color(&row.focused.text);
        }
        if let Some(stripe) = part("stripe") {
            look.stripe = color(&stripe.base.fill);
        }
        if let Some(cursor) = part("cursor") {
            look.cursor_color = color(&cursor.base.border_color);
            look.cursor_width = cursor.base.border_width;
            look.cursor_radius = cursor.base.radius.as_ref().map(|m| match m {
                aylis_core::Metric::Value(v) => *v,
                aylis_core::Metric::Token(t) => theme.radius.get(t).copied().unwrap_or(0.0),
            });
        }
        look
    }
}

/// Caixa de marcar de um campo lógico (`None` = nulo: caixa vazia e apagada).
fn draw_check(canvas: &mut dyn Canvas, r: RectF, on: Option<bool>, ctx: &PaintCtx<'_>) {
    let theme = ctx.theme;
    match on {
        Some(true) => {
            canvas.fill_rect(r, 4.0, &Brush::Solid(theme.color("primary")));
            let s = Stroke::new(theme.color("on_primary"), 2.0);
            let (x, y) = (r.x, r.y);
            canvas.draw_line(PointF::new(x + 4.0, y + 8.5), PointF::new(x + 7.0, y + 11.5), &s);
            canvas.draw_line(PointF::new(x + 7.0, y + 11.5), PointF::new(x + 12.5, y + 5.0), &s);
        }
        Some(false) => canvas.stroke_rect(r.inflate(-0.5), 4.0, &Stroke::new(theme.color("text_muted"), 1.5)),
        None => canvas.stroke_rect(r.inflate(-0.5), 4.0, &Stroke::new(theme.color("border"), 1.0)),
    }
}

/// Indicador à esquerda: registro atual, em edição/inclusão, alterado ou com erro.
fn draw_indicator(canvas: &mut dyn Canvas, row: &CachedRow, editing: bool, c: PointF, ctx: &PaintCtx<'_>) {
    let theme = ctx.theme;
    let dot = |canvas: &mut dyn Canvas, color: Color| {
        canvas.fill_ellipse(RectF::new(c.x - 3.0, c.y - 3.0, 6.0, 6.0), &Brush::Solid(color));
    };
    if row.current {
        let color = theme.color("primary");
        let stroke = Stroke::new(color, 1.8);
        if row.status.is_none() {
            // inclusão: "+"
            canvas.draw_line(PointF::new(c.x - 4.0, c.y), PointF::new(c.x + 4.0, c.y), &stroke);
            canvas.draw_line(PointF::new(c.x, c.y - 4.0), PointF::new(c.x, c.y + 4.0), &stroke);
        } else if editing {
            // edição: lápis
            canvas.draw_line(PointF::new(c.x - 4.0, c.y + 4.0), PointF::new(c.x + 3.0, c.y - 3.0), &stroke);
            canvas.draw_line(PointF::new(c.x - 4.0, c.y + 4.0), PointF::new(c.x - 4.0, c.y + 1.5), &stroke);
        } else {
            // atual: seta
            canvas.draw_line(PointF::new(c.x - 2.0, c.y - 4.0), PointF::new(c.x + 2.5, c.y), &stroke);
            canvas.draw_line(PointF::new(c.x + 2.5, c.y), PointF::new(c.x - 2.0, c.y + 4.0), &stroke);
        }
        return;
    }
    if row.error {
        dot(canvas, theme.color("danger"));
    } else if row.status == Some(RowStatus::Inserted) {
        dot(canvas, theme.color("primary"));
    } else if row.status == Some(RowStatus::Modified) {
        dot(canvas, theme.color("accent"));
    }
}

/// Lê `nome;cidade desc` como [(nome, false), (cidade, true)].
fn parse_sort(index: &str) -> Vec<(String, bool)> {
    index
        .split(';')
        .filter_map(|part| {
            let mut words = part.split_whitespace();
            let name = words.next()?.to_string();
            Some((name, words.next().is_some_and(|w| w.eq_ignore_ascii_case("desc"))))
        })
        .collect()
}

fn sort_spec(sort: &[(String, bool)]) -> String {
    sort.iter().map(|(f, desc)| if *desc { format!("{f} desc") } else { f.clone() }).collect::<Vec<_>>().join(";")
}

fn same_sort(a: &[(String, bool)], b: &[(String, bool)]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|((fa, da), (fb, db))| fa.eq_ignore_ascii_case(fb) && da == db)
}

impl aylis_core::DesignHooks for DataGrid {
    /// No designer, o separador das colunas no cabeçalho arrasta a largura.
    fn design_grip(&self, local: PointF) -> Option<usize> {
        if local.y < HEADER { self.border_at(local.x) } else { None }
    }
    fn design_drag(&mut self, grip: usize, delta: PointF, begin: bool) -> Option<(Option<ComponentId>, &'static str, aylis_core::PropValue)> {
        let c = self.cols.get_mut(grip)?;
        if begin {
            self.design_width = c.width;
        }
        let width = (self.design_width + delta.x).max(MIN_COLUMN).round();
        // Coluna como componente: a largura dela; senão a lista `columns` com as larguras.
        if let Some(source) = c.source {
            return Some((Some(source), "width", aylis_core::PropValue::Float(width as f64)));
        }
        c.width = width;
        Some((None, "columns", aylis_core::PropValue::String(self.columns_with_widths())))
    }
}
