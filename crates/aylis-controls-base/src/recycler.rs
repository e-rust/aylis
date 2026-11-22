//! [`VirtualList`]: lista de itens desenhados com um modelo seu (como o `RecyclerView` do
//! Android). Só existem as linhas visíveis (e mais duas): ao
//! rolar, as que saem por um lado são reaproveitadas para os itens que entram pelo outro,
//! então uma lista de um milhão de itens custa o mesmo que uma de vinte.
//!
//! O modelo é o primeiro filho da lista (desenhado no próprio designer: um painel com
//! imagem, textos, botões...) ou um form inteiro do projeto (`item_form`). Cada linha é
//! uma cópia dele. Ao mostrar o item `i` numa linha, a lista preenche os componentes de
//! `item_bindings` com os valores da fonte (se houver) e dispara `on_bind` com `i`: o handler
//! acha a linha com [`VirtualList::item`] e preenche o resto. Perto do fim, `on_load_more`
//! avisa (uma vez por tamanho) para a lista infinita: o handler busca mais e aumenta
//! `item_count`.
//!
//! Formatos: lista vertical (o normal) ou horizontal (`orientation`), e grade com `columns`
//! itens por linha (ou por coluna, na horizontal). O tamanho de cada item pode variar: `sizes`
//! (`80;120;80`, o resto com `item_height`) ou, pelo código, [`VirtualList::set_sizer`]; as
//! posições ficam somadas, e achar o primeiro item à vista é uma busca binária.
//!
//! Decidido em 22/11 (o relatório de um player de música portado para o Aylis), sem impor
//! colunas à linha: a seleção (`selection`, `item_index`, `selected`) com a linha marcada pelo
//! estado `checked` do tema, os eventos com o índice (`on_item_click`, `on_item_dbl_click`,
//! `on_item_activate`, `on_change`), o teclado, a barra de rolagem (`scroll_bar`) e vários
//! modelos de linha: cada filho direto é um modelo, e `kinds` (ou `kind_field`, ou
//! [`VirtualList::set_kind`]) diz qual vale em cada item. O cabeçalho é outro componente, o
//! `ListHeader`, que alinha títulos a componentes do modelo pelo nome.

use std::cell::RefCell;
use std::collections::HashMap;

use aylis_core::{ArrangeItem, ChildPointer, Component, ComponentId, ControlBase, EventArgs, EventCtx, Key, KeyEvent, PointerEvent, PropEnum, PropValue, RectF, Ui, WheelEvent, properties};

use crate::leaf_control;

/// De onde vêm os valores dos itens (um dataset, uma lista em memória...).
pub trait RowSource {
    /// Quantos itens.
    fn count(&self, ui: &Ui) -> usize;
    /// O valor do campo `field` no item `row`, como texto.
    fn value(&self, ui: &Ui, row: usize, field: &str) -> Option<String>;
}

/// Linhas a mais além das visíveis (as que estão entrando na tela).
const SPARE: usize = 2;
/// A quantos itens do fim `on_load_more` avisa.
const LOAD_MORE_AHEAD: usize = 5;

/// Para onde a lista corre.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl PropEnum for ListOrientation {
    const NAMES: &'static [&'static str] = &["vertical", "horizontal"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Vertical, Self::Horizontal].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O tamanho de cada item, pelo código (variável).
pub type Sizer = Box<dyn Fn(usize) -> f32>;

/// O modelo de cada item, pelo código: o nome de um dos modelos (vazio: o primeiro).
pub type Kinder = Box<dyn Fn(usize) -> String>;

/// Quantas linhas a lista deixa escolher.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// Nenhuma (como antes): a lista não marca linha nem recebe o foco.
    #[default]
    None,
    Single,
    /// Várias, com Ctrl (uma a uma) e Shift (o intervalo).
    Multiple,
}

impl PropEnum for SelectionMode {
    const NAMES: &'static [&'static str] = &["none", "single", "multiple"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Single, Self::Multiple].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Quando aparece a barra de rolagem da lista.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollBarMode {
    /// Nunca (como antes): rola pela roda e pelo toque.
    #[default]
    None,
    /// Quando os itens passam da área.
    Auto,
    Always,
}

impl PropEnum for ScrollBarMode {
    const NAMES: &'static [&'static str] = &["none", "auto", "always"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Auto, Self::Always].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A largura da barra de rolagem.
const BAR: f32 = 10.0;

/// Uma ligação do `item_bindings`: o componente do modelo, a propriedade (vazia: a principal) e
/// o campo da fonte.
#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
struct ItemBinding {
    component: String,
    property: String,
    field: String,
}

/// O registro das ligações do `VirtualList.item_bindings`.
pub const ITEM_BINDING: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "ItemBinding",
    fields: &[("component", aylis_core::PropKind::String), ("property", aylis_core::PropKind::String), ("field", aylis_core::PropKind::String)],
};

/// Lista virtual com modelo de item e reciclagem.
pub struct VirtualList {
    pub base: ControlBase,
    /// Quantos itens (com uma fonte, vem dela).
    pub item_count: i64,
    /// A altura de cada item, em pontos (na vertical).
    pub item_height: f32,
    /// Na horizontal: a largura de cada item.
    pub item_width: f32,
    /// Para onde a lista rola: `vertical` (um embaixo do outro) ou `horizontal` (lado a lado).
    pub orientation: ListOrientation,
    /// Grade: quantos itens lado a lado (na horizontal, um embaixo do outro). 1 = lista.
    pub columns: i64,
    /// Tamanhos próprios dos itens, em ordem (`80;120;80`; os que faltam usam `item_height`
    /// ou `item_width`). Só na lista (uma coluna).
    pub sizes: String,
    /// Um form do projeto como modelo (vazio: o primeiro filho da lista).
    pub item_form: String,
    /// O que cada componente do modelo mostra da fonte: a lista de [`ITEM_BINDING`] em JSON
    /// (`[{"component": "lblNome", "field": "nome"}, {"component": "pinEstado", "property":
    /// "fill", "field": "cor"}]`). Sem `property`: o texto, a imagem, o valor ou a marca (a
    /// primeira que o componente tiver).
    pub item_bindings: String,
    /// As ligações lidas de `item_bindings` (refeitas só quando o texto muda).
    parsed_bindings: (String, Vec<ItemBinding>),
    sizer: Option<Sizer>,
    /// Começo de cada linha somado (com tamanhos variáveis), e para quantos itens.
    starts: RefCell<(usize, Vec<f32>)>,
    /// A quantidade de itens na última arrumação (o `arrange` não tem o `Ui`).
    known_count: std::cell::Cell<usize>,
    offset: f32,
    template: Option<ComponentId>,
    /// As linhas: (instância, item mostrado).
    pool: RefCell<Vec<(ComponentId, Option<usize>)>>,
    source: Option<Box<dyn RowSource>>,
    /// `on_load_more` já avisou com esta quantidade.
    load_more_at: Option<usize>,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
    /// As ações que aparecem ao deslizar uma linha para a esquerda com o dedo
    /// (`Arquivar;Excluir`; vazio: sem). O toque numa dispara `on_swipe_action`.
    pub swipe_actions: String,
    /// A linha deslizada agora: (item, quanto andou para a esquerda, negativo).
    swiped: std::cell::Cell<Option<(usize, f32)>>,
    /// O dedo está arrastando a linha (até o `touch_end`).
    row_drag: std::cell::Cell<bool>,
    /// Quantas linhas dá para escolher (`none`: nenhuma, como antes).
    pub selection: SelectionMode,
    /// O item escolhido (-1: nenhum); com `multiple`, o último escolhido.
    pub item_index: i64,
    /// Com `multiple`, os itens escolhidos, em ordem (`2;5;7`).
    pub selected: String,
    /// A barra de rolagem: `none` (como antes), `auto` ou `always`.
    pub scroll_bar: ScrollBarMode,
    /// O modelo de cada item, pelo nome do filho (`pnlMusica;pnlMusica;pnlSecao`; o que falta e
    /// o nome desconhecido: o primeiro). Como o `sizes`, uma lista por item.
    pub kinds: String,
    /// O campo da fonte (o `DataRepeater`) com o nome do modelo de cada item, no lugar de `kinds`.
    pub kind_field: String,
    kinder: Option<Kinder>,
    /// Os modelos: o nome, o componente e a altura desenhada.
    templates: Vec<(String, ComponentId, f32)>,
    /// O modelo de cada linha (a posição em `templates`).
    row_kind: RefCell<HashMap<ComponentId, usize>>,
    /// Onde começou a escolha com Shift.
    anchor: Option<usize>,
    /// Arrastando a barra: a distância do mouse ao começo do cursor dela.
    bar_drag: Option<f32>,
    /// Algum modelo tem animação: só então a linha reciclada recomeça as animações.
    animated: bool,
    /// O item que cada linha mostrou por último com as animações dele (não muda no
    /// `refresh_item`, que volta ao mesmo item sem recomeçar).
    bound: RefCell<HashMap<ComponentId, usize>>,
    /// Os itens cujo `appear` já rodou (decidido em 22/11: uma vez por item). Esquecidos no
    /// `refresh` e quando a quantidade muda.
    appeared_items: RefCell<std::collections::HashSet<usize>>,
}

/// A largura de cada ação revelada ao deslizar a linha.
const SWIPE_ACTION_WIDTH: f32 = 80.0;

impl Default for VirtualList {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 400.0);
        base.set_clip_children(true);
        Self { swipe_actions: String::new(), swiped: std::cell::Cell::new(None), row_drag: std::cell::Cell::new(false), overscroll: aylis_core::Overscroll::Auto,
            base,
            item_count: 0,
            item_height: 64.0,
            item_width: 160.0,
            orientation: ListOrientation::Vertical,
            columns: 1,
            sizes: String::new(),
            item_form: String::new(),
            item_bindings: String::new(),
            parsed_bindings: (String::new(), Vec::new()),
            sizer: None,
            starts: RefCell::new((usize::MAX, Vec::new())),
            known_count: std::cell::Cell::new(0),
            offset: 0.0,
            template: None,
            pool: RefCell::default(),
            source: None,
            load_more_at: None,
            selection: SelectionMode::None,
            item_index: -1,
            selected: String::new(),
            scroll_bar: ScrollBarMode::None,
            kinds: String::new(),
            kind_field: String::new(),
            kinder: None,
            templates: Vec::new(),
            row_kind: RefCell::default(),
            anchor: None,
            bar_drag: None,
            animated: false,
            bound: RefCell::default(),
            appeared_items: RefCell::default(),
        }
    }
}

impl VirtualList {
    /// A linha que mostra o item `index` agora (`None` se ele não está à vista).
    pub fn item(ui: &Ui, list: ComponentId, index: usize) -> Option<ComponentId> {
        ui.get::<VirtualList>(list)?.pool.borrow().iter().find(|(_, i)| *i == Some(index)).map(|(id, _)| *id)
    }

    /// O item da linha onde está `component` (um botão dentro do cartão): num clique dentro
    /// da linha, de qual registro ela é (`None`: fora das linhas).
    pub fn index_of(ui: &Ui, list: ComponentId, component: ComponentId) -> Option<usize> {
        let l = ui.get::<VirtualList>(list)?;
        l.row_index(ui, list, component)
    }

    /// Na linha do item `index`, a cópia de `part`, um componente do modelo dado pelo acesso
    /// tipado (`self.lblNome`): no `on_bind`, mexer na peça da linha sem procurar pelo nome em
    /// texto.
    pub fn part(ui: &Ui, list: ComponentId, index: usize, part: ComponentId) -> Option<ComponentId> {
        ui.get::<VirtualList>(list)?.row_part(ui, index, part)
    }

    fn row_index(&self, ui: &Ui, list: ComponentId, component: ComponentId) -> Option<usize> {
        let pool = self.pool.borrow();
        let mut current = Some(component);
        while let Some(id) = current {
            if let Some((_, index)) = pool.iter().find(|(row, _)| *row == id) {
                return *index;
            }
            if id == list {
                return None;
            }
            current = ui.parent(id);
        }
        None
    }

    /// O caminho (os índices dos filhos) do modelo até `part`, refeito na linha: a cópia
    /// guarda a ordem dos filhos.
    fn row_part(&self, ui: &Ui, index: usize, part: ComponentId) -> Option<ComponentId> {
        let row = self.pool.borrow().iter().find(|(_, i)| *i == Some(index)).map(|(id, _)| *id)?;
        // O modelo de `part` (com vários, o da linha tem de ser o mesmo).
        let mut path = Vec::new();
        let mut current = part;
        let kind = loop {
            if let Some(k) = self.templates.iter().position(|(_, t, _)| *t == current) {
                break k;
            }
            if self.templates.is_empty() && Some(current) == self.template {
                break 0;
            }
            let parent = ui.parent(current)?;
            path.push(ui.children(parent).iter().position(|&c| c == current)?);
            current = parent;
        };
        if self.row_kind.borrow().get(&row).copied().unwrap_or(0) != kind {
            return None;
        }
        path.iter().rev().try_fold(row, |at, &i| ui.children(at).get(i).copied())
    }

    /// Na linha do item `index`, o componente do modelo com o nome `name` (o `ListHeader` acha
    /// as colunas assim).
    pub fn part_named(ui: &Ui, list: ComponentId, index: usize, name: &str) -> Option<ComponentId> {
        let row = ui.get::<VirtualList>(list)?.pool.borrow().iter().find(|(_, i)| *i == Some(index)).map(|(id, _)| *id)?;
        ui.find_in(row, name)
    }

    /// As linhas (cópias dos modelos) e os modelos: quem muda a largura de uma peça em todas
    /// (o `ListHeader`).
    pub fn rows_and_templates(ui: &Ui, list: ComponentId) -> Vec<ComponentId> {
        let Some(l) = ui.get::<VirtualList>(list) else { return Vec::new() };
        let mut out: Vec<ComponentId> = l.pool.borrow().iter().map(|(id, _)| *id).collect();
        out.extend(l.templates.iter().map(|(_, t, _)| *t));
        out
    }

    /// O item escolhido (-1: nenhum) e, com `multiple`, todos.
    pub fn selected(ui: &Ui, list: ComponentId) -> Vec<usize> {
        ui.get::<VirtualList>(list).map(|l| l.selected_items()).unwrap_or_default()
    }

    /// Escolhe o item `index` (com `selection` ligada), rolando até ele.
    pub fn select(ui: &mut Ui, list: ComponentId, index: usize) {
        ui.send_message(list, &Select(index));
    }

    /// Mostra de novo só o item `index` (o `on_bind` dele de novo): a música que passou a tocar.
    pub fn refresh_item(ui: &mut Ui, list: ComponentId, index: usize) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            for (_, i) in l.pool.borrow_mut().iter_mut() {
                if *i == Some(index) {
                    *i = None;
                }
            }
        }
        ui.send_message(list, &Refresh);
    }

    /// O modelo de cada item pelo código: o nome de um dos filhos (vazio: o primeiro).
    pub fn set_kind(ui: &mut Ui, list: ComponentId, kind: impl Fn(usize) -> String + 'static) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            l.kinder = Some(Box::new(kind));
            l.starts.borrow_mut().0 = usize::MAX;
        }
        Self::refresh(ui, list);
    }

    /// Os itens escolhidos (com `single`, o `item_index`).
    fn selected_items(&self) -> Vec<usize> {
        match self.selection {
            SelectionMode::None => Vec::new(),
            SelectionMode::Single => usize::try_from(self.item_index).ok().into_iter().collect(),
            SelectionMode::Multiple => self.selected.split([';', ',']).filter_map(|s| s.trim().parse::<usize>().ok()).collect(),
        }
    }

    fn set_selected_items(&mut self, items: &[usize]) {
        self.selected = items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(";");
    }

    /// Marca as linhas escolhidas pelo estado `checked` (o tema desenha).
    fn mark_rows(&self, ctx: &mut EventCtx<'_>) {
        self.mark_rows_with(ctx, &[]);
    }

    /// Como [`mark_rows`](Self::mark_rows); as linhas de `instant` acabaram de receber outro item
    /// e chegam no estado dele sem animar (a escolhida que volta à tela).
    fn mark_rows_with(&self, ctx: &mut EventCtx<'_>, instant: &[ComponentId]) {
        let chosen = self.selected_items();
        let rows: Vec<(ComponentId, bool)> = self.pool.borrow().iter().map(|(id, i)| (*id, i.is_some_and(|i| chosen.contains(&i)))).collect();
        for (id, on) in rows {
            if instant.contains(&id) {
                ctx.ui.set_checked_state_instant(id, on);
            } else {
                ctx.ui.set_checked_state(id, on);
            }
        }
    }

    /// Escolhe `index` (o clique, o teclado, o `select`): com Ctrl, junta ou tira (só
    /// `multiple`); com Shift, o intervalo desde o último escolhido.
    fn choose(&mut self, ctx: &mut EventCtx<'_>, index: usize, ctrl: bool, shift: bool) {
        if self.selection == SelectionMode::None {
            return;
        }
        let before = (self.item_index, self.selected.clone());
        if self.selection == SelectionMode::Multiple {
            let mut items = self.selected_items();
            if shift {
                let from = self.anchor.unwrap_or(index);
                items = (from.min(index)..=from.max(index)).collect();
            } else if ctrl {
                match items.iter().position(|&i| i == index) {
                    Some(at) => {
                        items.remove(at);
                    }
                    None => items.push(index),
                }
                self.anchor = Some(index);
            } else {
                items = vec![index];
                self.anchor = Some(index);
            }
            self.set_selected_items(&items);
        } else {
            self.anchor = Some(index);
        }
        self.item_index = index as i64;
        self.mark_rows(ctx);
        if before != (self.item_index, self.selected.clone()) {
            ctx.fire("on_change", EventArgs::Value(PropValue::Int(index as i64)));
        }
    }

    /// Rola o mínimo para o item `index` ficar à vista.
    fn ensure_visible(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        let count = self.count(ctx.ui);
        let line = index / self.lanes();
        let (start, size) = (self.line_start(line, count), self.line_size(line));
        let view = self.viewport();
        if start < self.offset {
            self.offset = start;
        } else if start + size > self.offset + view {
            self.offset = start + size - view;
        }
        self.sync(ctx);
    }

    /// O modelo do item `index` (a posição em `templates`).
    fn kind_index(&self, ui: &Ui, index: usize) -> usize {
        if self.templates.len() <= 1 {
            return 0;
        }
        let name = if let Some(k) = &self.kinder {
            k(index)
        } else if let (Some(source), false) = (&self.source, self.kind_field.trim().is_empty()) {
            source.value(ui, index, self.kind_field.trim()).unwrap_or_default()
        } else {
            self.kinds.split(';').nth(index).unwrap_or_default().trim().to_string()
        };
        self.templates.iter().position(|(n, _, _)| n.eq_ignore_ascii_case(name.trim())).unwrap_or(0)
    }

    /// O modelo do item pela lista `kinds` ou pelo código (sem o `Ui`: a altura de cada linha).
    fn kind_without_ui(&self, index: usize) -> Option<usize> {
        if self.templates.len() <= 1 {
            return None;
        }
        let name = match &self.kinder {
            Some(k) => k(index),
            None if !self.kinds.trim().is_empty() => self.kinds.split(';').nth(index).unwrap_or_default().trim().to_string(),
            None => return None,
        };
        Some(self.templates.iter().position(|(n, _, _)| n.eq_ignore_ascii_case(name.trim())).unwrap_or(0))
    }

    /// A barra de rolagem aparece agora.
    fn shows_bar(&self, count: usize) -> bool {
        match self.scroll_bar {
            ScrollBarMode::None => false,
            ScrollBarMode::Always => true,
            ScrollBarMode::Auto => self.line_start(self.lines(count), count) > self.viewport() + 0.5,
        }
    }

    /// O trilho da barra e o cursor dela (nas coordenadas da lista).
    fn bar_rects(&self, count: usize, width: f32, height: f32) -> (RectF, RectF) {
        let total = self.line_start(self.lines(count), count).max(1.0);
        let (track, view) = if self.horizontal() { (RectF::new(0.0, height - BAR, width, BAR), width) } else { (RectF::new(width - BAR, 0.0, BAR, height), height) };
        let length = if self.horizontal() { track.width } else { track.height };
        let thumb_len = (view / total * length).clamp(24.0_f32.min(length), length);
        let max = self.max_offset(count);
        let at = if max > 0.0 { self.offset / max * (length - thumb_len) } else { 0.0 };
        let thumb = if self.horizontal() { RectF::new(track.x + at, track.y + 2.0, thumb_len, BAR - 4.0) } else { RectF::new(track.x + 2.0, track.y + at, BAR - 4.0, thumb_len) };
        (track, thumb)
    }

    /// O mouse na barra (a posição no sentido da lista): leva o cursor até lá.
    fn bar_to(&mut self, ctx: &mut EventCtx<'_>, pos: f32, grab: f32) {
        let count = self.count(ctx.ui);
        let b = self.base.bounds();
        let (track, thumb) = self.bar_rects(count, b.width, b.height);
        let (length, thumb_len) = if self.horizontal() { (track.width, thumb.width) } else { (track.height, thumb.height) };
        let free = (length - thumb_len).max(1.0);
        self.offset = ((pos - grab) / free * self.max_offset(count)).clamp(0.0, self.max_offset(count));
        self.sync(ctx);
        ctx.repaint();
    }

    /// Os itens à vista agora, em ordem.
    pub fn visible(ui: &Ui, list: ComponentId) -> Vec<usize> {
        let Some(l) = ui.get::<VirtualList>(list) else { return Vec::new() };
        let mut out: Vec<usize> = l.pool.borrow().iter().filter_map(|(_, i)| *i).collect();
        out.sort_unstable();
        out
    }

    /// Liga uma fonte de valores (os `item_bindings` e a quantidade vêm dela).
    pub fn set_source(ui: &mut Ui, list: ComponentId, source: Box<dyn RowSource>) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            l.source = Some(source);
        }
        Self::refresh(ui, list);
    }

    /// Mostra de novo todos os itens à vista (os dados mudaram).
    pub fn refresh(ui: &mut Ui, list: ComponentId) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            for (_, index) in l.pool.borrow_mut().iter_mut() {
                *index = None;
            }
            l.forget_items();
        }
        ui.send_message(list, &Refresh);
    }

    /// Rola até o item `index` ficar no topo.
    pub fn scroll_to(ui: &mut Ui, list: ComponentId, index: usize) {
        ui.send_message(list, &ScrollTo(index));
    }

    /// Troca a fonte (para componentes que embrulham a lista, como a `DataRepeater`):
    /// as linhas são preenchidas de novo no próximo [`resync`](Self::resync).
    /// As ações do deslizar (`swipe_actions`), na ordem.
    fn actions(&self) -> Vec<&str> {
        self.swipe_actions.split(';').map(str::trim).filter(|a| !a.is_empty()).collect()
    }

    /// Põe cada linha no deslocamento dela: a deslizada, onde o dedo deixou; as outras (e as
    /// recicladas), no lugar.
    fn apply_swipe(&self, ctx: &mut EventCtx<'_>) {
        let swiped = self.swiped.get();
        // Sem `swipe_actions`, o `translate_x` da linha é do app (uma animação do modelo): a
        // lista não mexe. Com elas, é do deslizar.
        if swiped.is_none() && self.actions().is_empty() {
            return;
        }
        for &(row, index) in self.pool.borrow().iter() {
            let want = match (swiped, index) {
                (Some((i, x)), Some(index)) if i == index => x,
                _ => 0.0,
            };
            if ctx.ui.control(row).is_some_and(|cb| cb.translate().x != want) {
                // Como as animações de lugar: só o lugar muda (o desenho de dentro continua).
                let _ = ctx.ui.set_prop(row, "translate_x", want as f64);
            }
        }
    }

    /// O retângulo da faixa das ações da linha `index` (nas coordenadas da lista).
    fn actions_band(&self, index: usize, count: usize, width: f32) -> RectF {
        let total = SWIPE_ACTION_WIDTH * self.actions().len() as f32;
        let line = index / self.lanes();
        RectF::new(width - total, self.line_start(line, count) - self.offset, total, self.line_size(line))
    }

    pub fn set_row_source(&mut self, source: Option<Box<dyn RowSource>>) {
        self.source = source;
        self.load_more_at = None;
        for (_, index) in self.pool.borrow_mut().iter_mut() {
            *index = None;
        }
    }

    /// Esquece o que cada linha mostra e preenche de novo (os dados mudaram).
    pub fn resync(&mut self, ctx: &mut EventCtx<'_>) {
        for (_, index) in self.pool.borrow_mut().iter_mut() {
            *index = None;
        }
        self.forget_items();
        self.sync(ctx);
    }

    /// Os dados mudaram: os itens já não são os mesmos, então as linhas recomeçam as animações
    /// e o `appear` vale de novo para todos.
    fn forget_items(&self) {
        self.bound.borrow_mut().clear();
        self.appeared_items.borrow_mut().clear();
    }

    /// Tamanho variável por código: `size(i)` é a altura (na horizontal, a largura) do item
    /// `i`. Só na lista (uma coluna).
    pub fn set_sizer(ui: &mut Ui, list: ComponentId, size: impl Fn(usize) -> f32 + 'static) {
        if let Some(l) = ui.get_mut::<VirtualList>(list) {
            l.sizer = Some(Box::new(size));
            l.starts.borrow_mut().0 = usize::MAX;
        }
        Self::refresh(ui, list);
    }

    /// Onde começa o item `index` e o tamanho dele, no sentido da lista (para rolar até ele,
    /// para testes).
    pub fn item_span(ui: &Ui, list: ComponentId, index: usize) -> Option<(f32, f32)> {
        let l = ui.get::<VirtualList>(list)?;
        let count = l.count(ui);
        let line = index / l.lanes();
        Some((l.line_start(line, count), l.line_size(line)))
    }

    /// Os itens lado a lado (as colunas da grade).
    fn lanes(&self) -> usize {
        self.columns.max(1) as usize
    }

    fn horizontal(&self) -> bool {
        self.orientation == ListOrientation::Horizontal
    }

    /// O tamanho de um item sem tamanho próprio.
    fn uniform(&self) -> f32 {
        if self.horizontal() { self.item_width.max(8.0) } else { self.item_height.max(8.0) }
    }

    /// Os tamanhos variam (lista com `sizes`, com o tamanho pelo código ou com modelos de
    /// alturas diferentes).
    fn variable(&self) -> bool {
        self.lanes() == 1 && (self.sizer.is_some() || !self.sizes.trim().is_empty() || self.kind_without_ui(0).is_some())
    }

    /// O tamanho da linha `line` (na grade, todas iguais).
    fn line_size(&self, line: usize) -> f32 {
        if !self.variable() {
            return self.uniform();
        }
        if let Some(s) = &self.sizer {
            return s(line).max(1.0);
        }
        // Com vários modelos (pela lista ou pelo código): a altura desenhada do modelo do item.
        let by_kind = || self.kind_without_ui(line).and_then(|k| self.templates.get(k)).map(|(_, _, h)| *h).filter(|h| *h > 0.0);
        self.sizes.split(';').nth(line).and_then(|s| s.trim().parse::<f32>().ok()).or_else(by_kind).unwrap_or_else(|| self.uniform()).max(1.0)
    }

    fn lines(&self, count: usize) -> usize {
        count.div_ceil(self.lanes())
    }

    /// Os começos das linhas somados (refeitos quando a quantidade muda).
    fn ensure_starts(&self, count: usize) {
        let lines = self.lines(count);
        let mut starts = self.starts.borrow_mut();
        if starts.0 == lines {
            return;
        }
        let mut v = Vec::with_capacity(lines + 1);
        let mut at = 0.0;
        for line in 0..lines {
            v.push(at);
            at += self.line_size(line);
        }
        v.push(at);
        *starts = (lines, v);
    }

    /// Onde começa a linha `line`.
    fn line_start(&self, line: usize, count: usize) -> f32 {
        if !self.variable() {
            return line as f32 * self.uniform();
        }
        self.ensure_starts(count);
        let starts = self.starts.borrow();
        starts.1.get(line).or(starts.1.last()).copied().unwrap_or(0.0)
    }

    /// A linha em `pos` (no sentido da lista).
    fn line_at(&self, pos: f32, count: usize) -> usize {
        if !self.variable() {
            return (pos / self.uniform()).floor().max(0.0) as usize;
        }
        self.ensure_starts(count);
        let starts = self.starts.borrow();
        starts.1.partition_point(|s| *s <= pos).saturating_sub(1)
    }

    /// O tamanho da área à vista, no sentido da lista: o que o layout deu (`bounds`, com
    /// `align: client` maior que o desenhado), não o `width`/`height` do projeto (o relatório
    /// do RTSPlayer de 13/11, item 1: 8 linhas de 13 numa tela de 900).
    fn viewport(&self) -> f32 {
        let (size, p) = (self.base.inner_size(), self.base.padding());
        if self.horizontal() { (size.width - p.horizontal()).max(0.0) } else { (size.height - p.vertical()).max(0.0) }
    }

    fn count(&self, ui: &Ui) -> usize {
        match &self.source {
            Some(s) => s.count(ui),
            None => self.item_count.max(0) as usize,
        }
    }

    fn max_offset(&self, count: usize) -> f32 {
        (self.line_start(self.lines(count), count) - self.viewport()).max(0.0)
    }

    /// Acha (ou cria) os modelos e esconde: as linhas são cópias deles. Os modelos são os
    /// filhos diretos da lista (ou o form de `item_form`); o primeiro é o padrão.
    fn ensure_template(&mut self, ctx: &mut EventCtx<'_>) -> Option<ComponentId> {
        if self.template.is_some_and(|t| ctx.ui.contains(t)) && self.templates.iter().all(|(_, t, _)| ctx.ui.contains(*t)) {
            return self.template;
        }
        let pool: Vec<ComponentId> = self.pool.borrow().iter().map(|(id, _)| *id).collect();
        let list: Vec<ComponentId> = if self.item_form.is_empty() {
            ctx.ui.children(ctx.id).iter().copied().filter(|c| !pool.contains(c)).collect()
        } else {
            let form = ctx.ui.create_form(&self.item_form).ok()?;
            vec![ctx.ui.clone_component(form, Some(ctx.id)).ok()?]
        };
        self.templates.clear();
        for t in &list {
            let height = ctx.ui.control(*t).map_or(0.0, |cb| cb.height());
            self.templates.push((ctx.ui.name(*t).unwrap_or_default().to_string(), *t, height));
            if let Some(cb) = ctx.ui.control_mut(*t) {
                cb.set_visible(false);
            }
        }
        self.template = list.first().copied();
        self.animated = list.iter().any(|t| ctx.ui.has_animations(*t));
        self.starts.borrow_mut().0 = usize::MAX;
        self.template
    }

    /// Põe as linhas nos itens à vista, criando as que faltam e reaproveitando as que
    /// saíram. Os itens que mudaram de linha recebem os valores e o `on_bind`.
    fn sync(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return; // no designer, o modelo aparece como foi desenhado
        }
        let Some(template) = self.ensure_template(ctx) else { return };
        let count = self.count(ctx.ui);
        self.offset = self.offset.clamp(0.0, self.max_offset(count));
        self.known_count.set(count);
        // As linhas à vista (e mais algumas), e os itens delas.
        let first = self.line_at(self.offset, count);
        let last = self.line_at((self.offset + self.viewport() - 0.01).max(self.offset), count) + SPARE;
        let lanes = self.lanes();
        let wanted: Vec<usize> = (first * lanes..((last + 1) * lanes).min(count)).collect();

        let _ = template;
        // O modelo de cada item à vista.
        let kinds: Vec<usize> = wanted.iter().map(|&i| self.kind_index(ctx.ui, i)).collect();
        // As que continuam no lugar (o mesmo item e o mesmo modelo) ficam; as outras ficam livres.
        let mut fresh = Vec::new();
        let mut unplaced = Vec::new();
        {
            let mut pool = self.pool.borrow_mut();
            let row_kind = self.row_kind.borrow();
            for (id, index) in pool.iter_mut() {
                let keep = index.and_then(|i| wanted.iter().position(|&w| w == i)).is_some_and(|at| row_kind.get(id).copied().unwrap_or(0) == kinds[at]);
                if !keep {
                    *index = None;
                }
            }
            let kept: Vec<usize> = pool.iter().filter_map(|(_, i)| *i).collect();
            for (at, &item) in wanted.iter().enumerate().filter(|(_, i)| !kept.contains(i)) {
                let kind = kinds[at];
                match pool.iter_mut().find(|(id, i)| i.is_none() && row_kind.get(id).copied().unwrap_or(0) == kind) {
                    Some((id, index)) => {
                        *index = Some(item);
                        fresh.push((*id, item));
                    }
                    None => unplaced.push((item, kind)),
                }
            }
        }
        // Linhas que faltam: cópias do modelo do item.
        for (item, kind) in unplaced {
            let Some(model) = self.templates.get(kind).map(|(_, t, _)| *t).or(self.template) else { continue };
            let Ok(copy) = ctx.ui.clone_component(model, Some(ctx.id)) else { break };
            if let Some(cb) = ctx.ui.control_mut(copy) {
                // Quem põe a linha no lugar é a lista (`arrange`, que só recebe os soltos): o
                // modelo desenhado com `align: top` (o natural no designer) seria empilhado pelo
                // align, sem `sizes` nem rolagem (RTSPlayer, 13/11, item 2). O modelo fica como
                // foi desenhado.
                cb.set_visible(true).set_align(aylis_core::Align::None);
            }
            self.pool.borrow_mut().push((copy, Some(item)));
            self.row_kind.borrow_mut().insert(copy, kind);
            fresh.push((copy, item));
        }
        for (id, _) in self.pool.borrow().iter() {
            let shown = self.pool.borrow().iter().any(|(p, i)| p == id && i.is_some());
            if let Some(cb) = ctx.ui.control_mut(*id) {
                cb.set_visible(shown);
            }
        }
        let instant = self.restart_animations(ctx, &fresh);
        self.mark_rows_with(ctx, &instant);
        for (id, index) in fresh {
            self.bind(ctx, id, index);
            ctx.fire("on_bind", EventArgs::Value(PropValue::Int(index as i64)));
        }
        // Lista infinita: perto do fim, pede mais (uma vez por quantidade).
        if count > 0 && wanted.last().is_some_and(|&last| last + LOAD_MORE_AHEAD >= count) && self.load_more_at != Some(count) {
            self.load_more_at = Some(count);
            ctx.fire("on_load_more", EventArgs::Value(PropValue::Int(count as i64)));
        }
        // A linha deslizada segue o item quando as linhas são recicladas.
        self.apply_swipe(ctx);
        ctx.ui.invalidate();
    }

    /// As linhas que foram para outro item recomeçam as animações (decidido em 22/11): a do item
    /// anterior para e volta ao valor de antes, o `appear` roda só para o item que ainda não
    /// apareceu, e o estado do item novo chega sem animar. Devolve essas linhas. Sem animação
    /// nos modelos, não faz nada.
    fn restart_animations(&self, ctx: &mut EventCtx<'_>, fresh: &[(ComponentId, usize)]) -> Vec<ComponentId> {
        if !self.animated {
            return Vec::new();
        }
        let mut moved = Vec::new();
        for &(row, item) in fresh {
            let previous = self.bound.borrow_mut().insert(row, item);
            if previous == Some(item) {
                continue; // o `refresh_item`: o mesmo item, as animações seguem
            }
            if ctx.ui.reset_animations(row) {
                if let Some(p) = previous {
                    self.appeared_items.borrow_mut().insert(p);
                }
            }
            if self.appeared_items.borrow().contains(&item) {
                ctx.ui.finish_appear(row);
            }
            moved.push(row);
        }
        moved
    }

    /// As ligações de `item_bindings`, lidas de novo só quando o texto muda.
    fn bindings(&mut self) -> &[ItemBinding] {
        if self.parsed_bindings.0 != self.item_bindings {
            self.parsed_bindings = (self.item_bindings.clone(), crate::records::parse_list(&self.item_bindings));
        }
        &self.parsed_bindings.1
    }

    /// Preenche os componentes de `item_bindings` da linha com os valores do item: a
    /// propriedade dada (`fill`, `visible`...) ou, sem ela, o texto, a imagem, o valor ou a
    /// marca (a primeira que o componente tiver); o valor do campo, como texto, convertido para
    /// o tipo da propriedade.
    fn bind(&mut self, ctx: &mut EventCtx<'_>, row: ComponentId, index: usize) {
        let bindings = self.bindings().to_vec();
        let Some(source) = &self.source else { return };
        for b in &bindings {
            let Some(target) = ctx.ui.find_in(row, b.component.trim()) else { continue };
            let value = source.value(ctx.ui, index, b.field.trim()).unwrap_or_default();
            let property = Some(b.property.trim()).filter(|p| !p.is_empty()).or_else(|| ["text", "source", "value", "checked"].into_iter().find(|p| ctx.ui.prop_info(target, p).is_some()));
            let Some(property) = property else { continue };
            let typed = match ctx.ui.prop_info(target, property).map(|i| i.kind) {
                Some(aylis_core::PropKind::Bool) => PropValue::Bool(matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "1" | "sim" | "yes")),
                Some(aylis_core::PropKind::Int) => PropValue::Int(value.trim().parse::<f64>().unwrap_or(0.0) as i64),
                Some(aylis_core::PropKind::Float | aylis_core::PropKind::Angle | aylis_core::PropKind::Duration) => PropValue::Float(value.trim().parse().unwrap_or(0.0)),
                Some(aylis_core::PropKind::Color) => match aylis_core::Color::from_hex(&value) {
                    Some(c) => PropValue::Color(c),
                    None => PropValue::String(value),
                },
                _ => PropValue::String(value),
            };
            let _ = ctx.ui.set_prop(target, property, typed);
        }
    }
}

/// Mensagem interna: refazer as linhas (dados novos).
struct Refresh;
/// Mensagem interna: rolar até um item.
struct ScrollTo(usize);
/// Mensagem interna: escolher um item.
struct Select(usize);

impl Component for VirtualList {
    fn layout_hooks(&self) -> Option<&dyn aylis_core::LayoutHooks> {
        Some(self)
    }

    leaf_control!("VirtualList");

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        item_count: i64 => "Behavior",
        item_height: f32 => "Appearance",
        item_width: f32 => "Appearance",
        orientation: ListOrientation => "Layout",
        columns: i64 => "Layout",
        sizes: String as NumberList => "Appearance",
        item_form: String as FormName => "Behavior",
        item_bindings: String as RecordList(&ITEM_BINDING) => "Behavior",
        swipe_actions: String => "Behavior",
        selection: SelectionMode => "Behavior",
        item_index: i64 => "Data",
        selected: String as NumberList => "Data",
        scroll_bar: ScrollBarMode => "Appearance",
        kinds: String as List => "Behavior",
        kind_field: String => "Behavior";
        events: on_bind: i64, on_load_more: i64, on_swipe_action: SwipeAction, on_change: i64, on_item_click: i64, on_item_dbl_click: i64, on_item_activate: i64
    }

    fn child_pointer_mut(&mut self) -> Option<&mut dyn ChildPointer> {
        Some(self)
    }

    fn captures_key(&self, event: &KeyEvent) -> bool {
        self.selection != SelectionMode::None && matches!(event.key, Key::Up | Key::Down | Key::PageUp | Key::PageDown | Key::Home | Key::End | Key::Enter)
    }

    /// O teclado com `selection`: as setas, Page Up/Down, Home/End andam (com Shift, o
    /// intervalo em `multiple`) e rolam até o item; Enter dispara o `on_item_activate`.
    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if self.selection == SelectionMode::None {
            return;
        }
        let count = self.count(ctx.ui);
        if count == 0 {
            return;
        }
        let current = usize::try_from(self.item_index).ok();
        let page = ((self.viewport() / self.uniform()).floor() as usize).max(1) * self.lanes();
        let lanes = self.lanes();
        let last = count - 1;
        let target = match (event.key, current) {
            (Key::Enter, Some(i)) => {
                ctx.fire("on_item_activate", EventArgs::Value(PropValue::Int(i as i64)));
                ctx.set_handled();
                return;
            }
            (Key::Up, Some(i)) => i.saturating_sub(lanes),
            (Key::Down, Some(i)) => (i + lanes).min(last),
            (Key::PageUp, Some(i)) => i.saturating_sub(page),
            (Key::PageDown, Some(i)) => (i + page).min(last),
            (Key::Home, _) => 0,
            (Key::End, _) => last,
            (Key::Up | Key::Down | Key::PageUp | Key::PageDown, None) => 0,
            _ => return,
        };
        self.choose(ctx, target, false, event.modifiers.shift);
        self.ensure_visible(ctx, target);
        ctx.set_handled();
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("index_of", &[PropKind::Component], Some(PropKind::Int)).params(&["component"]),
            MethodInfo::new("part", &[PropKind::Int, PropKind::Component], Some(PropKind::Component)).params(&["index", "component"]),
            MethodInfo::new("select", &[PropKind::Int], None).params(&["index"]),
            MethodInfo::new("scroll_to", &[PropKind::Int], None).params(&["index"]),
            MethodInfo::new("refresh_item", &[PropKind::Int], None).params(&["index"]),
            MethodInfo::new("refresh", &[], None),
        ];
        METHODS
    }

    /// `index_of(componente)`: o item da linha onde ele está (-1: fora das linhas);
    /// `part(índice, componente do modelo)`: a cópia dele na linha do item (veja
    /// [`VirtualList::index_of`] e [`VirtualList::part`]).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let component = |i: usize| match args.get(i) {
            Some(PropValue::Component(Some(id))) => Ok(*id),
            _ => Err(aylis_core::messages::t("informe o componente")),
        };
        match method {
            "index_of" => Ok(PropValue::Int(self.row_index(ctx.ui, ctx.id, component(0)?).map_or(-1, |i| i as i64))),
            "part" => {
                let index = match args.first() {
                    Some(PropValue::Int(i)) if *i >= 0 => *i as usize,
                    _ => return Err(aylis_core::messages::t("informe o índice do item")),
                };
                Ok(PropValue::Component(self.row_part(ctx.ui, index, component(1)?)))
            }
            "select" | "scroll_to" | "refresh_item" => {
                let index = match args.first() {
                    Some(PropValue::Int(i)) if *i >= 0 => *i as usize,
                    _ => return Err(aylis_core::messages::t("informe o índice do item")),
                };
                match method {
                    "select" => {
                        self.choose(ctx, index, false, false);
                        self.ensure_visible(ctx, index);
                    }
                    "scroll_to" => {
                        let count = self.count(ctx.ui);
                        self.offset = self.line_start(index / self.lanes(), count);
                        self.sync(ctx);
                    }
                    _ => {
                        for (_, i) in self.pool.borrow_mut().iter_mut() {
                            if *i == Some(index) {
                                *i = None;
                            }
                        }
                        self.sync(ctx);
                    }
                }
                Ok(PropValue::Null)
            }
            "refresh" => {
                self.resync(ctx);
                Ok(PropValue::Null)
            }
            _ => Err(format!("VirtualList não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            "item_form" => {
                // Outro modelo: as linhas são refeitas.
                for (id, _) in self.pool.take() {
                    ctx.ui.free(id);
                }
                if let Some(t) = self.template.take().filter(|_| !self.item_form.is_empty()) {
                    ctx.ui.free(t);
                }
                self.sync(ctx);
            }
            "selection" => {
                // Escolher pede o foco (o teclado); sem escolha, como antes.
                let on = self.selection != SelectionMode::None;
                self.base.set_can_focus(on);
                self.mark_rows(ctx);
            }
            "item_index" | "selected" => self.mark_rows(ctx),
            "scroll_bar" => ctx.ui.invalidate(),
            "item_count" | "item_height" | "item_width" | "orientation" | "columns" | "sizes" | "kinds" | "kind_field" => {
                // As posições somadas são refeitas.
                self.starts.borrow_mut().0 = usize::MAX;
                if name == "item_count" && self.count(ctx.ui) < self.load_more_at.unwrap_or(0) {
                    self.load_more_at = None; // a lista diminuiu (filtro, recarga)
                }
                // Menos itens (filtro, recarga): os itens já não são os mesmos. Mais (a lista
                // infinita carregando): os que já apareceram continuam.
                if name == "item_count" && self.count(ctx.ui) < self.known_count.get() {
                    self.forget_items();
                }
                // Antes da primeira medida (carregando o layout) as linhas esperam: os
                // handlers ainda estão sendo ligados.
                if !self.pool.borrow().is_empty() {
                    self.sync(ctx);
                }
            }
            _ => {}
        }
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        self.sync(ctx);
    }

    fn scroll_axes(&self) -> (bool, bool) {
        (self.horizontal(), !self.horizontal())
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        // O dedo para os lados numa lista vertical com `swipe_actions`: arrasta a linha tocada
        // (decidido em 14/11); o `touch_end` deixa aberta ou fecha.
        let actions = self.actions().len();
        if actions > 0 && event.touch && !self.horizontal() && self.lanes() == 1 {
            let sideways = event.delta.x != 0.0 && event.delta.x.abs() > event.delta.y.abs() * 1.5;
            if self.row_drag.get() || sideways {
                let count = self.count(ctx.ui);
                let index = match (self.row_drag.get(), self.swiped.get()) {
                    (true, Some((i, _))) => i,
                    _ => self.line_at(event.position.y + self.offset, count),
                };
                if index < count {
                    let total = SWIPE_ACTION_WIDTH * actions as f32;
                    let before = self.swiped.get().filter(|(i, _)| *i == index).map_or(0.0, |(_, x)| x);
                    self.swiped.set(Some((index, (before + event.delta.x).clamp(-total, 0.0))));
                    self.row_drag.set(true);
                    self.apply_swipe(ctx);
                    ctx.repaint();
                    ctx.set_handled();
                    return;
                }
            }
        }
        // Rolar fecha a linha aberta.
        if self.swiped.get().is_some() && !self.row_drag.get() && event.delta.y != 0.0 {
            self.swiped.set(None);
            self.apply_swipe(ctx);
            ctx.repaint();
        }
        let max = self.max_offset(self.count(ctx.ui));
        // Na horizontal, a roda comum (vertical) também anda.
        let delta = if self.horizontal() && event.delta.x != 0.0 { event.delta.x } else { event.delta.y };
        let target = (self.offset - delta).clamp(0.0, max);
        if (target - self.offset).abs() < 0.01 {
            return; // no limite: a rolagem passa para os pais
        }
        self.offset = target;
        self.sync(ctx);
        ctx.set_handled();
    }

    fn touch_end(&mut self, ctx: &mut EventCtx<'_>) {
        if !self.row_drag.replace(false) {
            return;
        }
        // Passou da metade das ações: fica aberta; senão, fecha.
        let total = SWIPE_ACTION_WIDTH * self.actions().len() as f32;
        self.swiped.set(self.swiped.get().filter(|(_, x)| *x < -total / 2.0).map(|(i, _)| (i, -total)));
        self.apply_swipe(ctx);
        ctx.repaint();
    }

    // O toque numa ação revelada (a faixa à direita da linha deslizada, que a linha descobriu):
    // `on_swipe_action` com o item e a ação; qualquer toque na lista fecha a linha.
    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::PointerEvent) {
        if self.bar_drag.take().is_some() {
            return;
        }
        let Some((index, x)) = self.swiped.get() else { return };
        let actions: Vec<String> = self.actions().iter().map(|a| a.to_string()).collect();
        let band = self.actions_band(index, self.count(ctx.ui), self.base.bounds().width);
        self.swiped.set(None);
        self.apply_swipe(ctx);
        ctx.repaint();
        if x >= 0.0 || !band.contains(event.position) {
            return;
        }
        let hit = ((event.position.x - band.x) / SWIPE_ACTION_WIDTH) as usize;
        if let Some(action) = actions.get(hit) {
            let args = crate::records::to_json(&SwipeAction { index: index as i64, action: action.clone() });
            ctx.fire("on_swipe_action", EventArgs::Text(args));
        }
        ctx.set_handled();
    }

    /// A barra de rolagem: apertar no cursor arrasta; no trilho, leva até o ponto.
    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let count = self.count(ctx.ui);
        if !self.shows_bar(count) {
            return;
        }
        let b = self.base.bounds();
        let (track, thumb) = self.bar_rects(count, b.width, b.height);
        if !track.contains(event.position) {
            return;
        }
        let pos = if self.horizontal() { event.position.x } else { event.position.y };
        let start = if self.horizontal() { thumb.x - track.x } else { thumb.y - track.y };
        let len = if self.horizontal() { thumb.width } else { thumb.height };
        let grab = if thumb.contains(event.position) { pos - start } else { len / 2.0 };
        self.bar_drag = Some(grab);
        self.bar_to(ctx, pos, grab);
        ctx.set_handled();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(grab) = self.bar_drag {
            let pos = if self.horizontal() { event.position.x } else { event.position.y };
            self.bar_to(ctx, pos, grab);
        }
    }

    // As ações por baixo da linha deslizada (a linha anda para a esquerda e as descobre) e a
    // barra de rolagem.
    fn paint(&self, canvas: &mut dyn aylis_core::Canvas, ctx: &aylis_core::PaintCtx<'_>) {
        let count = self.known_count.get();
        if self.shows_bar(count) {
            let (track, thumb) = self.bar_rects(count, ctx.rect.width, ctx.rect.height);
            let at = |r: RectF| r.offset(ctx.rect.x, ctx.rect.y);
            canvas.fill_rect(at(track), 0.0, &aylis_core::Brush::Solid(ctx.theme.color("text_muted").with_opacity(0.08)));
            let alpha = if self.bar_drag.is_some() { 0.7 } else { 0.45 };
            canvas.fill_rect(at(thumb), (BAR - 4.0) / 2.0, &aylis_core::Brush::Solid(ctx.theme.color("text_muted").with_opacity(alpha)));
        }
        let Some((index, x)) = self.swiped.get().filter(|(_, x)| *x < 0.0) else { return };
        let _ = x;
        let actions = self.actions();
        let band = self.actions_band(index, self.known_count.get(), ctx.rect.width);
        let mut style = ctx.text_style();
        style.color = ctx.theme.color("on_primary");
        style.h_align = aylis_core::TextAlign::Center;
        style.v_align = aylis_core::TextAlign::Center;
        for (k, action) in actions.iter().enumerate() {
            let r = RectF::new(band.x + k as f32 * SWIPE_ACTION_WIDTH, band.y, SWIPE_ACTION_WIDTH, band.height);
            // A última (a de mais efeito, como excluir) na cor de erro; as outras, na principal.
            let color = if k + 1 == actions.len() { ctx.theme.color("error") } else { ctx.theme.color("primary") };
            canvas.fill_rect(r, 0.0, &aylis_core::Brush::Solid(color));
            canvas.draw_text(action, r, &style);
        }
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn std::any::Any) {
        if message.is::<Refresh>() {
            self.sync(ctx);
        } else if let Some(ScrollTo(index)) = message.downcast_ref::<ScrollTo>() {
            let count = self.count(ctx.ui);
            self.offset = self.line_start(*index / self.lanes(), count);
            self.sync(ctx);
        } else if let Some(Select(index)) = message.downcast_ref::<Select>() {
            self.choose(ctx, *index, false, false);
            self.ensure_visible(ctx, *index);
        }
    }

}

impl ChildPointer for VirtualList {
    /// O apertar numa linha: escolhe o item (com Ctrl e Shift em `multiple`) e, no duplo
    /// clique, o `on_item_dbl_click` e o `on_item_activate`.
    fn child_pressed(&mut self, ctx: &mut EventCtx<'_>, child: ComponentId, event: &PointerEvent) {
        let Some(index) = self.row_index(ctx.ui, ctx.id, child) else { return };
        if self.selection != SelectionMode::None {
            if self.base.can_focus() {
                ctx.ui.set_focus(ctx.id);
            }
            self.choose(ctx, index, event.modifiers.ctrl || event.modifiers.meta, event.modifiers.shift);
        }
        if event.click_count == 2 {
            ctx.fire("on_item_dbl_click", EventArgs::Value(PropValue::Int(index as i64)));
            ctx.fire("on_item_activate", EventArgs::Value(PropValue::Int(index as i64)));
        }
    }

    fn child_clicked(&mut self, ctx: &mut EventCtx<'_>, child: ComponentId) {
        if let Some(index) = self.row_index(ctx.ui, ctx.id, child) {
            ctx.fire("on_item_click", EventArgs::Value(PropValue::Int(index as i64)));
        }
    }
}

impl aylis_core::LayoutHooks for VirtualList {
    /// Cada item no lugar dele (a linha e, na grade, a coluna); o modelo e as linhas sem
    /// item ficam fora.
    fn arrange(&self, area: RectF, children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        let pool = self.pool.borrow();
        if pool.is_empty() {
            return None; // ainda sem linhas (designer, antes do primeiro sync): o normal
        }
        let lanes = self.lanes();
        let count = self.known_count.get();
        // A barra de rolagem fica fora das linhas (o mouse nela é da lista).
        let bar = if self.shows_bar(count) { BAR } else { 0.0 };
        let cross_total = if self.horizontal() { area.height - bar } else { area.width - bar };
        let lane_size = cross_total / lanes as f32;
        Some(
            children
                .iter()
                .map(|c| match pool.iter().find(|(id, _)| *id == c.id).and_then(|(_, i)| *i) {
                    Some(i) => {
                        let (line, lane) = (i / lanes, i % lanes);
                        let main = self.line_start(line, count) - self.offset;
                        let size = self.line_size(line);
                        let cross = lane as f32 * lane_size;
                        let m = c.margins;
                        // O lugar inteiro do item, menos as margens do modelo.
                        if self.horizontal() {
                            RectF::new(area.x + main + m.left, area.y + cross + m.top, (size - m.left - m.right).max(0.0), (lane_size - m.top - m.bottom).max(0.0))
                        } else {
                            RectF::new(area.x + cross + m.left, area.y + main + m.top, (lane_size - m.left - m.right).max(0.0), (size - m.top - m.bottom).max(0.0))
                        }
                    }
                    None => RectF::new(area.x, area.y, 0.0, 0.0),
                })
                .collect(),
        )
    }
}

/// O toque numa ação da linha deslizada (o `on_swipe_action` da `VirtualList`): o item e o
/// texto da ação (um dos `swipe_actions`).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SwipeAction {
    pub index: i64,
    pub action: String,
}

/// O registro do [`SwipeAction`] no acesso tipado.
pub const SWIPE_ACTION: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "SwipeAction",
    fields: &[("index", aylis_core::PropKind::Int), ("action", aylis_core::PropKind::String)],
};

impl aylis_core::EventValue for SwipeAction {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(aylis_core::PropKind::Record(&SWIPE_ACTION));
    fn from_args(args: &EventArgs) -> Option<Self> {
        crate::records::from_event(args)
    }
}
