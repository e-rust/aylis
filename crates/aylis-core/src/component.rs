//! O modelo de componentes: [`Component`] e [`ControlBase`] (os dados
//! comuns de todo controle visual).

use std::any::Any;
use std::fmt;

use crate::align::{Align, Anchors, SelfAlign};
use crate::canvas::Canvas;
use crate::error::{Error, Result};
use crate::event::{EventArgs, EventCtx, KeyEvent, PointerEvent, WheelEvent};
use crate::geometry::{Bounds, PointF, RectF, SizeF};
use crate::paint::PaintCtx;
use crate::platform::CursorShape;
use crate::prop::{PropInfo, PropKind, PropValue};

/// Identificador de um componente dentro do [`Ui`](crate::Ui).
///
/// É "geracional": depois que o componente é liberado, o id antigo nunca mais aponta
/// para nada (as consultas devolvem `None` em vez de acessar memória inválida).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentId {
    pub(crate) index: u32,
    pub(crate) generation: u32,
}

impl ComponentId {
    /// Número estável para identificar o componente fora do Rust (C ABI, protocolo).
    pub fn to_u64(self) -> u64 {
        (self.generation as u64) << 32 | self.index as u64
    }

    /// O inverso de [`to_u64`](Self::to_u64). O `Ui` recusa ids de componentes liberados.
    pub fn from_u64(value: u64) -> Self {
        Self { index: value as u32, generation: (value >> 32) as u32 }
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}v{}", self.index, self.generation)
    }
}

/// Estado visual de um controle, usado pelo tema e pela pintura.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ControlState {
    pub hover: bool,
    pub pressed: bool,
    pub focused: bool,
    pub checked: bool,
    /// Calculado na pintura: o controle ou algum ancestral está com `enabled = false`.
    pub disabled: bool,
}

/// Componente: a base de tudo que vive no [`Ui`](crate::Ui), visual ou não.
///
/// Todos os métodos, exceto [`class_name`](Component::class_name), têm implementação
/// padrão — sobrescreva só o que precisar.
///
/// ## Controle visual
/// Um componente é um **controle** quando devolve um [`ControlBase`] em
/// [`control`](Component::control) / [`control_mut`](Component::control_mut).
///
/// ## "Herança" por composição
/// Para criar um componente a partir de outro, guarde o componente base num campo e
/// devolva-o em [`inner`](Component::inner) / [`inner_mut`](Component::inner_mut).
/// Tudo que você não sobrescrever (pintura, eventos, propriedades, `ControlBase`) é
/// repassado automaticamente para ele. Para chamar o comportamento original dentro de
/// um método sobrescrito (o `inherited`), chame o método no campo:
///
/// ```ignore
/// struct GlowButton { base: Button }
///
/// impl Component for GlowButton {
///     fn class_name(&self) -> &'static str { "GlowButton" }
///     fn inner(&self) -> Option<&dyn Component> { Some(&self.base) }
///     fn inner_mut(&mut self) -> Option<&mut dyn Component> { Some(&mut self.base) }
///
///     fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx) {
///         self.base.paint(canvas, ctx); // inherited Paint
///         // ...desenho extra
///     }
/// }
/// ```
pub trait Component: Any {
    /// Nome da classe ("Button", "ClientDataSet"...). É o nome usado no registro de
    /// classes, nos arquivos de layout e nos estilos do tema.
    fn class_name(&self) -> &'static str;

    /// O componente "pai" na composição, se houver (o equivalente ao ancestral).
    fn inner(&self) -> Option<&dyn Component> {
        None
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        None
    }

    /// Cadeia de classes, da mais derivada para a base. Usada pelo tema: um
    /// `GlowButton` sem estilo próprio usa o estilo de `Button`.
    fn class_chain(&self, out: &mut Vec<&'static str>) {
        out.push(self.class_name());
        if let Some(inner) = self.inner() {
            inner.class_chain(out);
        }
    }

    /// Dados de controle visual. Componentes não visuais devolvem `None`.
    fn control(&self) -> Option<&ControlBase> {
        self.inner().and_then(|i| i.control())
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        self.inner_mut().and_then(|i| i.control_mut())
    }

    /// Propriedades publicadas **desta camada**. As do componente base (`inner`) e as do
    /// [`ControlBase`] são incluídas automaticamente pelo `Ui`.
    fn props(&self) -> &'static [PropInfo] {
        &[]
    }

    /// O evento padrão: o duplo clique no designer cria o handler dele e abre o código.
    /// `None`: a IDE escolhe pelo nome (o `on_change` da classe, o
    /// `on_timer`... ou o `on_click`).
    fn default_event(&self) -> Option<&'static str> {
        self.inner().and_then(|c| c.default_event())
    }

    /// Lê uma propriedade desta camada; `None` se não for dela.
    fn get_prop(&self, _name: &str) -> Option<PropValue> {
        None
    }

    /// Grava uma propriedade desta camada. Devolva [`Error::UnknownProperty`] se não for
    /// dela (o `Ui` então tenta a camada base).
    fn set_prop(&mut self, name: &str, _value: PropValue) -> Result<()> {
        Err(Error::UnknownProperty(name.to_string()))
    }

    /// Avisa que um componente foi liberado (como `Notification(AComponent, opRemove)`).
    /// Quem guarda referências para outros componentes deve limpá-las aqui.
    fn notification(&mut self, freed: ComponentId) {
        if let Some(inner) = self.inner_mut() {
            inner.notification(freed);
        }
    }

    /// Pinta o controle. O canvas já está na origem do controle; `ctx.rect` é a área dele.
    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if let Some(inner) = self.inner() {
            inner.paint(canvas, ctx);
        }
    }

    /// O controle não tem fundo (um texto, uma caixa de seleção, um medidor): com o `fill`
    /// próprio, o `Ui` pinta o fundo dele inteiro. Os que têm
    /// fundo usam o `fill` no desenho deles.
    fn transparent_background(&self) -> bool {
        self.inner().is_some_and(|i| i.transparent_background())
    }

    /// O `paint` deste estilo cobre o retângulo inteiro do controle com cor opaca (um painel
    /// de fundo sólido sem cantos arredondados, um form). Com `true`, o que fica inteiro atrás
    /// dele (um irmão pintado antes) não é pintado: um medidor animando embaixo de uma tela que
    /// o cobre não pede quadros. Na dúvida, `false` (o padrão): só se perde a economia.
    fn opaque(&self, style: &crate::theme::ResolvedStyle) -> bool {
        self.inner().is_some_and(|i| i.opaque(style))
    }

    /// Pinta por cima dos filhos (bordas, anel de foco...).
    fn paint_after(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if let Some(inner) = self.inner() {
            inner.paint_after(canvas, ctx);
        }
    }

    /// A pintura (`paint` e `paint_after`) só lê: sem efeito colateral (nada de evento, de
    /// cache que avança, de quadro de vídeo consumido) e o mesmo desenho para o mesmo estado,
    /// estilo e propriedades. Com `true`, o `Ui` pode ensaiar o desenho fora do quadro e não
    /// repintar o controle quando ele não mudou (o fundo de um contêiner quando o mouse entra
    /// nele). Os contêineres simples dizem `true`; o padrão é `false`, mesmo com `inner`
    /// (quem herda de um contêiner e pinta por cima precisa dizer de novo).
    fn paint_is_pure(&self) -> bool {
        false
    }

    /// Um componente provedor (item 25): o que ele oferece aos componentes que o apontam pela
    /// propriedade `provider` (veja [`crate::providers`]). O padrão: nada.
    fn provided(&self) -> Option<crate::providers::Provided<'_>> {
        None
    }

    /// O tamanho real (`bounds`) mudou depois de um layout (como o `Resize`).
    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(inner) = self.inner_mut() {
            inner.resized(ctx);
        }
    }

    /// Clique (como `Click`). O padrão dispara o evento `on_click`.
    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        match self.inner_mut() {
            Some(inner) => inner.click(ctx),
            None => ctx.fire("on_click", EventArgs::None),
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.pointer_down(ctx, event);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.pointer_up(ctx, event);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.pointer_move(ctx, event);
        }
    }

    fn pointer_enter(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(inner) = self.inner_mut() {
            inner.pointer_enter(ctx);
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(inner) = self.inner_mut() {
            inner.pointer_leave(ctx);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.wheel(ctx, event);
        }
    }

    /// Pinça/giro de dois dedos. Marque como tratado (`ctx.set_handled()`) para não subir
    /// para os pais.
    fn gesture(&mut self, ctx: &mut EventCtx<'_>, event: &crate::event::GestureEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.gesture(ctx, event);
        }
    }

    /// Um deslizar do dedo (o `Carousel` troca de item, o `TabControl` de página). Marque
    /// como tratado (`ctx.set_handled()`) para não subir para os pais.
    fn swipe(&mut self, ctx: &mut EventCtx<'_>, event: &crate::event::SwipeEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.swipe(ctx, event);
        }
    }

    /// O dedo que arrastava (rolando) saiu da tela: quem segue o arrasto por dentro (a linha da
    /// lista deslizada para mostrar as ações) fecha ou fica aberto. Vai ao controle tocado e a
    /// todos os pais.
    fn touch_end(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(inner) = self.inner_mut() {
            inner.touch_end(ctx);
        }
    }

    /// Um deslizar que começou na borda do form (`event.start` nas coordenadas do form) e que
    /// ninguém tratou: a `NavigationDrawer` abre. `true` se tratou.
    fn edge_swipe(&mut self, ctx: &mut EventCtx<'_>, event: &crate::event::SwipeEvent) -> bool {
        match self.inner_mut() {
            Some(inner) => inner.edge_swipe(ctx, event),
            None => false,
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.key_down(ctx, event);
        }
    }

    fn key_up(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.key_up(ctx, event);
        }
    }

    /// Tecla de atalho que ninguém tratou (como o atalho de um item de menu): o `Ui`
    /// oferece a todos os componentes do form. Marque como tratado
    /// ([`EventCtx::set_handled`]) se for sua.
    fn shortcut(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if let Some(inner) = self.inner_mut() {
            inner.shortcut(ctx, event);
        }
    }

    /// Os ganchos raros (barra de tarefas, bandeja, vista nativa, conector, arquivos soltos),
    /// se este componente usa: devolva `Some(self)` e implemente [`PlatformHooks`].
    fn platform_hooks(&self) -> Option<&dyn PlatformHooks> {
        self.inner().and_then(|i| i.platform_hooks())
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn PlatformHooks> {
        self.inner_mut().and_then(|i| i.platform_hooks_mut())
    }

    /// Os ganchos do designer (pontos que se arrastam dentro do controle), se este
    /// componente usa: devolva `Some(self)` e implemente [`DesignHooks`].
    fn design_hooks(&self) -> Option<&dyn DesignHooks> {
        self.inner().and_then(|i| i.design_hooks())
    }

    fn design_hooks_mut(&mut self) -> Option<&mut dyn DesignHooks> {
        self.inner_mut().and_then(|i| i.design_hooks_mut())
    }

    /// O ponto (coordenadas do controle, já sem a rotação) acerta o controle? Padrão: o
    /// retângulo inteiro. As formas (círculo, estrela, linha...) respondem só na figura.
    fn contains_point(&self, local: PointF, size: SizeF) -> bool {
        self.inner().map_or(true, |i| i.contains_point(local, size))
    }

    /// Quanto o controle recebe o mouse além do próprio retângulo (as alças de uma moldura
    /// desenhadas do lado de fora). Fora do retângulo, quem decide é o
    /// [`contains_point`](Component::contains_point) (que recebe o ponto fora dele). Padrão: 0.
    fn hit_slop(&self) -> f32 {
        self.inner().map_or(0.0, |i| i.hit_slop())
    }

    /// Em que eixos o conteúdo rola (horizontal, vertical): o [`Ui::reveal`](crate::Ui::reveal)
    /// rola estes contêineres para mostrar um controle de dentro (o campo sob o teclado).
    fn scroll_axes(&self) -> (bool, bool) {
        self.inner().map_or((false, false), |i| i.scroll_axes())
    }

    /// O componente edita texto. No celular, quando ele tem o foco a plataforma abre o
    /// teclado virtual.
    /// O que vai num arrasto que começa neste controle sem `drag_data` nem handler: as listas
    /// e as grades mandam o item (o texto, a chave do registro) e o rótulo. `None`: nada.
    fn drag_item(&self, _ui: &crate::Ui) -> Option<(String, String)> {
        None
    }

    /// O que acompanha o cursor num arrasto, quando este componente é o `drag_image` de um
    /// controle (o `DragImage`); `source` é o controle arrastado. `None`: nada.
    fn drag_visual(&self, _ui: &crate::Ui, _source: ComponentId) -> Option<crate::DragVisual> {
        None
    }

    /// O próprio componente recebe este arrasto (sem `accepts_drops` nem eventos): as abas
    /// que passam de uma faixa para outra. Vale também quando ele é a origem (a aba que volta
    /// para a faixa de onde saiu). Com `true`, recebe [`drop_hover`](Self::drop_hover),
    /// [`drop_leave`](Self::drop_leave) e [`drop_receive`](Self::drop_receive).
    fn drop_accepts(&self, _ui: &crate::Ui, _id: ComponentId, _data: &str) -> bool {
        false
    }

    /// O arrasto aceito por [`drop_accepts`](Self::drop_accepts) está sobre o componente, no
    /// ponto `position` (coordenadas dele); chamado a cada movimento.
    fn drop_hover(&mut self, _ctx: &mut EventCtx<'_>, _data: &str, _position: PointF) {}

    /// O arrasto saiu de cima do componente (ou foi cancelado).
    fn drop_leave(&mut self, _ctx: &mut EventCtx<'_>) {}

    /// Soltou o arrasto aceito sobre o componente, em `position` (coordenadas dele).
    fn drop_receive(&mut self, _ctx: &mut EventCtx<'_>, _data: &str, _position: PointF) {}

    /// O arrasto que começou neste componente acabou (`on_drag_end` também dispara).
    fn drag_ended(&mut self, _ctx: &mut EventCtx<'_>, _data: &str, _end: crate::DragEnd) {}

    fn accepts_text(&self) -> bool {
        self.inner().is_some_and(|i| i.accepts_text())
    }

    /// O controle usa a tecla (o Enter de um Memo, as setas de uma lista): ela não vira o
    /// Tab do form (`tab_keys`).
    fn captures_key(&self, event: &KeyEvent) -> bool {
        self.inner().is_some_and(|i| i.captures_key(event))
    }

    /// A legenda com o `&` da letra do atalho (`"&Salvar"`: Alt+S), para os controles que
    /// aceitam (botões, caixas de marcar, rótulos, grupos).
    fn mnemonic_text(&self) -> Option<&str> {
        self.inner().and_then(|i| i.mnemonic_text())
    }

    /// No Alt+letra, o foco vai para o primeiro filho (um grupo) em vez de clicar.
    fn mnemonic_focuses_child(&self) -> bool {
        self.inner().is_some_and(|i| i.mnemonic_focuses_child())
    }

    /// Texto digitado (já com acentos e IME resolvidos pela plataforma).
    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        if let Some(inner) = self.inner_mut() {
            inner.text_input(ctx, text);
        }
    }

    fn focus_enter(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(inner) = self.inner_mut() {
            inner.focus_enter(ctx);
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(inner) = self.inner_mut() {
            inner.focus_exit(ctx);
        }
    }

    /// Uma propriedade publicada mudou por nome ([`Ui::set_prop`](crate::Ui::set_prop): JSON,
    /// Object Inspector, protocolo). É o lugar dos efeitos colaterais da mudança: aqui
    /// o componente tem o `Ui` (por exemplo, o TabControl mostra a aba nova).
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if let Some(inner) = self.inner_mut() {
            inner.property_changed(ctx, name);
        }
    }

    /// A marca do controle agora (`checked`), lida pelo núcleo na pintura: o tema reflete a
    /// marca mesmo quando o campo muda direto, sem o `set_prop` (C02 do Tabernáculo).
    fn checked_state(&self) -> Option<bool> {
        self.inner().and_then(|i| i.checked_state())
    }

    /// Os argumentos do `on_change` quando a propriedade de valor `name` muda pelo código
    /// ([`Ui::set_prop`](crate::Ui::set_prop): o acesso tipado, o protocolo);
    /// `value` é o valor novo. O padrão é `EventArgs::Value(value)`: o controle que dispara
    /// outro argumento quando o usuário muda (o texto do item, numa ComboBox) devolve o mesmo
    /// aqui, e `None` não dispara. Quais propriedades contam: [`is_value_property`].

    fn change_args(&self, name: &str, value: &PropValue) -> Option<EventArgs> {
        match self.inner() {
            Some(inner) => inner.change_args(name, value),
            None => Some(EventArgs::Value(value.clone())),
        }
    }

    /// A propriedade `name` só muda a pintura (o valor de um medidor, o giro de um 3D): o
    /// `set_prop` dela repinta sem refazer o layout da tela. As do controle (cores, opacidade,
    /// giro, deslocamento, cursor, dica) o `Ui` já conhece; aqui vão as do componente. Na
    /// dúvida, `false` (refazer o layout nunca está errado, só custa).
    fn paint_only_property(&self, name: &str) -> bool {
        self.inner().is_some_and(|inner| inner.paint_only_property(name))
    }

    /// Intervalo do timer deste componente, em segundos (`None` = parado). O `Ui` dispara
    /// `on_timer` a cada intervalo; ver [`Ui::sync_timer`](crate::Ui::sync_timer).
    fn timer_interval(&self) -> Option<f64> {
        self.inner().and_then(|i| i.timer_interval())
    }

    /// Quanto o timer pode atrasar ([`TimerPrecision`](crate::TimerPrecision)).
    fn timer_precision(&self) -> crate::TimerPrecision {
        self.inner().map(|i| i.timer_precision()).unwrap_or_default()
    }

    /// O timer conta numa thread própria (não espera o laço da tela, que pode estar
    /// ocupado); o `on_timer` chega à tela pelo [`Poster`](crate::Poster).
    fn timer_background(&self) -> bool {
        self.inner().is_some_and(|i| i.timer_background())
    }

    /// Quanto o disparo que vem a seguir atrasou, em segundos (antes de [`timer`](Component::timer)).
    fn timer_late(&mut self, late: f64) {
        if let Some(inner) = self.inner_mut() {
            inner.timer_late(late);
        }
    }

    /// Um quadro vai ser pintado (pedido com [`Ui::request_frames`](crate::Ui::request_frames)):
    /// chamado antes da pintura, com o tempo do quadro em `ctx.ui.time()`, para um movimento
    /// que acompanha o monitor (a rolagem com deslize). Devolve se quer o próximo quadro.
    fn frame(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        self.inner_mut().is_some_and(|inner| inner.frame(ctx))
    }

    /// O timer venceu (ver [`timer_interval`](Component::timer_interval)). O padrão dispara o
    /// evento `on_timer`.
    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        match self.inner_mut() {
            Some(inner) => inner.timer(ctx),
            None => ctx.fire("on_timer", EventArgs::None),
        }
    }

    /// Organiza os filhos com `align = None` (grade, fluxo...): recebe a área livre (já
    /// sem o padding e sem o que os filhos encaixados ocuparam) e o tamanho pedido e as
    /// margens de cada filho; devolve o retângulo de cada um, na mesma ordem. `None` (o
    /// padrão) = posição livre.
    fn arrange(&self, area: RectF, children: &[ArrangeItem]) -> Option<Vec<RectF>> {
        self.inner().and_then(|i| i.arrange(area, children))
    }

    /// A linha de base do texto do controle, em pixels do topo dele, na altura `height` (para
    /// o `FlowLayout` com `baseline` alinhar os textos de uma fileira; L12 do Tabernáculo).
    /// `None` (o padrão): o controle não tem texto, e fica pelo fundo.
    fn baseline(&self, ui: &crate::Ui, id: ComponentId, height: f32) -> Option<f32> {
        self.inner().and_then(|i| i.baseline(ui, id, height))
    }

    /// O vão entre os filhos encaixados (Top, Left...), em pixels: o `gap` do `Layout` e do
    /// `Panel` (o `gap` do CSS; L05 do Tabernáculo). Padrão: 0.
    fn dock_gap(&self) -> f32 {
        self.inner().map_or(0.0, |i| i.dock_gap())
    }

    /// A altura que o conteúdo pede na largura `width` (o texto de um Label quebrado nela),
    /// para o `auto_size`. `None` (o padrão): o controle não mede o conteúdo (um contêiner
    /// com `auto_size` fica da altura dos filhos encaixados em cima e embaixo).
    fn auto_height(&self, ui: &crate::Ui, id: ComponentId, width: f32) -> Option<f32> {
        self.inner().and_then(|i| i.auto_height(ui, id, width))
    }

    /// A largura que o conteúdo pede (o texto de um Label sem quebra), para o `auto_size` de
    /// quem não é esticado na largura (`align` left, right ou none). `None` (o padrão): a
    /// largura fica a pedida.
    fn auto_width(&self, ui: &crate::Ui, id: ComponentId) -> Option<f32> {
        self.inner().and_then(|i| i.auto_width(ui, id))
    }

    /// Uma camada aberta por outro (a lista de um ComboBox, as sugestões de um campo), posta
    /// direto no form: o mouse apertado fora dela e fora de quem a abriu ([`dies_with`]) a
    /// libera, mesmo que o clique caia onde não recebe foco (o fundo da tela, um rótulo). O
    /// dono fica sabendo pelo [`notification`](Component::notification). Padrão: `false`.
    ///
    /// [`dies_with`]: Component::dies_with
    fn closes_on_outside_press(&self) -> bool {
        false
    }

    /// O clique fora fechou a camada ([`closes_on_outside_press`]): `true` se ela mesma se
    /// escondeu e deve continuar existindo (o `Popover`, desenhado no form com os filhos);
    /// `false` (o padrão), o núcleo a libera.
    ///
    /// [`closes_on_outside_press`]: Component::closes_on_outside_press
    fn dismissed(&mut self, ctx: &mut EventCtx<'_>) -> bool {
        let _ = ctx;
        false
    }

    /// Este componente só vive enquanto `dies_with` existir: liberar aquele libera este
    /// também. É o que a lista aberta de um ComboBox (uma camada sobre o form) usa: se o
    /// ComboBox some (a tela foi refeita), a lista não fica órfã na tela.
    fn dies_with(&self) -> Option<ComponentId> {
        self.inner().and_then(|i| i.dies_with())
    }

    /// Os itens que o controle desenha sozinho (abas, linhas, itens de menu, células), para
    /// leitores de tela, automação e testes ([`crate::automation`]). `id` é este controle.
    fn access_items(&self, ui: &crate::Ui, id: ComponentId) -> Vec<crate::automation::AccessItem> {
        self.inner().map_or_else(Vec::new, |i| i.access_items(ui, id))
    }

    /// Clica ou seleciona o item `index` de [`access_items`](Self::access_items). Devolve
    /// `false` se não fez nada.
    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, action: crate::automation::ItemAction) -> bool {
        self.inner_mut().is_some_and(|i| i.access_action(ctx, index, action))
    }

    /// Prende o controle a um irmão (mesmo pai): devolve o alvo e o ponto dele onde o
    /// centro deste controle fica (`0..1` na largura e na altura: `(1, 0)` é o canto de
    /// cima à direita). É o que deixa um `Badge` no canto de um botão.
    fn attach(&self) -> Option<(ComponentId, f32, f32)> {
        self.inner().and_then(|i| i.attach())
    }

    /// Dados extras de um evento que está sendo entregue (chave e texto), para quem recebe
    /// os eventos fora do Rust: o protocolo os manda no `info` do evento (o `on_message` do
    /// WebView manda a URL de quem mandou).
    fn event_info(&self, event: &str) -> Vec<(&'static str, String)> {
        self.inner().map(|i| i.event_info(event)).unwrap_or_default()
    }

    /// Mensagem enviada por outro componente com [`Ui::send_message`](crate::Ui::send_message).
    /// O tipo da mensagem é livre: cada camada
    /// (por exemplo a de dados) define as suas e o componente faz `downcast_ref`.
    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(inner) = self.inner_mut() {
            inner.message(ctx, message);
        }
    }

    /// Os métodos que [`invoke`](Component::invoke) atende, com os tipos (o catálogo do
    /// acesso tipado nas linguagens: `frmPaint.pcDesenho.Undo`).
    fn methods(&self) -> &'static [MethodInfo] {
        &[]
    }

    /// Um método pelo nome ([`Ui::invoke`](crate::Ui::invoke), o comando `invoke` do
    /// protocolo): o que não é propriedade, como `undo`, `clear` ou `save`. Funciona igual
    /// em Rust e nas outras linguagens (pelo protocolo). Devolve o resultado ou o erro.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> std::result::Result<PropValue, String> {
        let class = self.class_name();
        match self.inner_mut() {
            Some(inner) => inner.invoke(ctx, method, args),
            None => Err(format!("{class} não tem o método '{method}'")),
        }
    }

}

/// Os ganchos que só uns poucos componentes usam: fora do [`Component`], para a lista de
/// métodos de um controle comum ficar curta. O componente que usa devolve `Some(self)` em
/// [`Component::platform_hooks`] e [`Component::platform_hooks_mut`].
pub trait PlatformHooks {
    /// O que este componente quer no botão da barra de tarefas (o `Taskbar`).
    fn taskbar(&self) -> Option<crate::taskbar::TaskbarSpec> {
        None
    }

    /// Ícone na bandeja do sistema que este componente quer mostrar (o `TrayIcon`).
    fn tray(&self) -> Option<crate::tray::TraySpec> {
        None
    }

    /// Vista nativa que a plataforma põe por cima do controle (um navegador embutido...).
    /// O controle continua sendo pintado por baixo (o que aparece onde a plataforma não
    /// tem a vista, como no designer).
    fn native_view(&self) -> Option<NativeView> {
        None
    }

    /// Comandos pendentes para a vista nativa (a plataforma pega e executa).
    fn take_native_commands(&self) -> Vec<NativeCommand> {
        Vec::new()
    }

    /// Um conector (cano, fio) preso a dois irmãos: os pontos de ligação deles (`0..1` na
    /// largura e na altura). O layout põe o conector em volta do caminho entre os dois; ele
    /// segue os componentes quando eles mudam de lugar.
    fn connection(&self) -> Option<Connection> {
        None
    }

    /// Arquivos soltos sobre o controle (arrastados do sistema). Devolve `true` se aceitou
    /// (senão eles vão para o pai).
    fn files_dropped(&mut self, _ctx: &mut EventCtx<'_>, _paths: &[String]) -> bool {
        false
    }

    /// Arquivos do sistema passando por cima (`Some`, com os caminhos) ou saindo, soltos ou
    /// cancelados (`None`). Devolve `true` se o controle os receberia.
    fn files_hovering(&mut self, _ctx: &mut EventCtx<'_>, _paths: Option<&[String]>) -> bool {
        false
    }
}

/// Os ganchos do designer: pontos que se arrastam dentro do controle (o separador das
/// colunas de uma grade). O componente que usa devolve `Some(self)` em
/// [`Component::design_hooks`] e [`Component::design_hooks_mut`].
pub trait DesignHooks {
    /// Um ponto que se arrasta dentro do controle, em coordenadas do controle. Devolve o
    /// índice do ponto.
    fn design_grip(&self, _local: PointF) -> Option<usize> {
        None
    }

    /// O ponto `grip` foi arrastado `delta` desde o começo (`begin` no primeiro movimento).
    /// Devolve a propriedade a gravar (o designer grava, com o desfazer): o componente
    /// (`None` = este), o nome e o valor.
    fn design_drag(&mut self, _grip: usize, _delta: PointF, _begin: bool) -> Option<(Option<ComponentId>, &'static str, PropValue)> {
        None
    }
}

/// Um filho para [`Component::arrange`]: o tamanho pedido e as margens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrangeItem {
    pub id: ComponentId,
    pub size: SizeF,
    /// Já com a escala de espaços da vista ativa.
    pub margins: Bounds,
    /// Quantas colunas o filho pede numa grade ([`ControlBase::column_span`]).
    pub span: u32,
    /// Quantas linhas ([`ControlBase::row_span`]).
    pub row_span: u32,
    /// O lugar fixo na grade ([`ControlBase::grid_cell`]).
    pub cell: Option<(u32, u32)>,
    /// A parte do espaço livre da linha que ele toma ([`ControlBase::grow`]).
    pub grow: f32,
    /// A linha de base do texto dele, do topo ([`Component::baseline`]); posta pelo layout.
    pub baseline: Option<f32>,
    /// A escala de espaços da vista ativa do form: o contêiner multiplica os espaços dele
    /// (`gap`) por ela.
    pub spacing: f32,
}

impl ArrangeItem {
    /// O item de um controle.
    pub fn of(id: ComponentId, cb: &ControlBase) -> Self {
        ArrangeItem {
            id,
            size: SizeF::new(cb.width(), cb.height()),
            margins: cb.margins(),
            span: cb.column_span(),
            row_span: cb.row_span(),
            cell: cb.grid_cell(),
            grow: cb.grow(),
            baseline: None,
            spacing: cb.spacing_scale(),
        }
    }
}

/// Uma vista nativa pedida por um componente ([`Component::native_view`]).
#[derive(Clone, Debug, PartialEq)]
pub enum NativeView {
    /// Navegador embutido: abre `url` ou, sem ela, mostra `html`. Quando `generation`
    /// muda, a plataforma carrega de novo. `navigation`: os endereços (origens ou prefixos)
    /// para onde a página pode navegar (vazia, todos); `sandbox`: o atributo do `<iframe>`
    /// no navegador.
    ///
    /// O ambiente do navegador (só ao criar a vista; o relatório do Tabernáculo, W07):
    /// `background` é a cor antes da primeira pintura da página; `data_dir`, a pasta de dados
    /// (vazia: a do sistema); `browser_args`, os argumentos do navegador do sistema (o WebView2).
    Web { url: String, html: String, generation: u64, navigation: Vec<String>, sandbox: String, background: Option<crate::Color>, data_dir: String, browser_args: String },
}

/// Mensagem de uma página do navegador embutido para o app: o texto e a URL de quem mandou.
/// A plataforma entrega ao componente com [`Ui::send_message`](crate::Ui::send_message); ele
/// decide se a origem vale e dispara o `on_message`.
#[derive(Clone, Debug, PartialEq)]
pub struct WebMessage {
    pub text: String,
    pub url: String,
}

/// A navegação do navegador embutido para `url` é permitida pela lista (o
/// `allowed_navigation` do WebView)? O conteúdo interno (`about:`, `data:`) sempre. Um item só
/// com a origem (`https://site.com`) libera o site inteiro; com caminho, só o que começa por ele.
pub fn navigation_allowed(allowed: &[String], url: &str) -> bool {
    allowed.is_empty()
        || url.starts_with("about:")
        || url.starts_with("data:")
        || allowed.iter().any(|a| {
            let a = a.trim_end_matches('/');
            url == a || url.starts_with(&format!("{a}/")) || url.starts_with(&format!("{a}?")) || (a.matches('/').count() <= 2 && url_origin(url) == url_origin(a))
        })
}

/// A origem (`https://site.com:8080`) de um endereço, para comparar com uma lista.
pub fn url_origin(url: &str) -> String {
    let Some((scheme, rest)) = url.split_once("://") else { return url.to_string() };
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    format!("{}://{}", scheme.to_ascii_lowercase(), host.to_ascii_lowercase())
}

/// Um método de [`Component::invoke`]: o nome, o tipo de cada argumento e o do resultado
/// (`None`: não devolve nada que interesse).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MethodInfo {
    pub name: &'static str,
    pub args: &'static [PropKind],
    pub returns: Option<PropKind>,
    /// O nome de cada argumento (`path`), que o acesso tipado gerado usa nas assinaturas;
    /// vazio: sem nome (o gerado numera, `a1`, `a2`).
    pub params: &'static [&'static str],
}

impl MethodInfo {
    pub const fn new(name: &'static str, args: &'static [PropKind], returns: Option<PropKind>) -> Self {
        Self { name, args, returns, params: &[] }
    }

    /// Os nomes dos argumentos, na ordem de `args`:
    /// `MethodInfo::new("save_to", &[PropKind::String], None).params(&["path"])`.
    pub const fn params(self, names: &'static [&'static str]) -> Self {
        Self { params: names, ..self }
    }
}

/// Comando para uma vista nativa.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeCommand {
    /// Roda JavaScript na página.
    Eval(String),
    Back,
    Forward,
    Reload,
}

/// Uma vista nativa a mostrar: o controle, onde (relativo ao form) e o que é.
#[derive(Clone, Debug, PartialEq)]
pub struct NativeViewSlot {
    pub id: ComponentId,
    pub rect: crate::RectF,
    /// A parte que aparece (os pais que recortam, como uma caixa rolada, cortam o resto),
    /// nas mesmas coordenadas de `rect`; `None`: inteira.
    pub clip: Option<crate::RectF>,
    pub visible: bool,
    pub view: NativeView,
    pub commands: Vec<NativeCommand>,
}

/// Todas as propriedades publicadas de um componente: as da classe, as das camadas de
/// composição e as do [`ControlBase`] (sem `name` nem os eventos ligados no `Ui`).
///
/// Trabalha direto no componente, sem precisar do [`Ui`](crate::Ui): serve, por exemplo,
/// para ler os valores padrão de uma instância nova da classe.
pub fn prop_infos_of(component: &dyn Component) -> Vec<PropInfo> {
    let mut out: Vec<PropInfo> = Vec::new();
    // Os nomes já vistos num conjunto: procurar na lista a cada propriedade era quadrático
    // (~90 por controle), e isso roda para cada componente no índice de referências.
    let mut seen = std::collections::HashSet::new();
    let mut push = |info: &PropInfo| {
        if seen.insert(info.name) {
            out.push(*info);
        }
    };
    let mut layer = Some(component);
    while let Some(l) = layer {
        l.props().iter().for_each(&mut push);
        layer = l.inner();
    }
    if component.control().is_some() {
        ControlBase::PROPS.iter().for_each(&mut push);
    }
    out
}

/// A descrição de uma propriedade pelo nome, na mesma ordem de [`prop_infos_of`] (a camada
/// de fora vence), sem montar a lista inteira: todo `get_prop`/`set_prop` passa por aqui.
pub fn prop_info_of(component: &dyn Component, name: &str) -> Option<PropInfo> {
    let mut layer = Some(component);
    while let Some(l) = layer {
        if let Some(info) = l.props().iter().find(|p| p.name == name) {
            return Some(*info);
        }
        layer = l.inner();
    }
    if component.control().is_some() {
        return ControlBase::PROPS.iter().find(|p| p.name == name).copied();
    }
    None
}

/// Lê uma propriedade (que não seja evento) direto no componente, procurando em todas as
/// camadas e no [`ControlBase`].
/// As propriedades que são "o valor" de um controle (o que o usuário muda e o `on_change`
/// avisa), quando publicadas em `Data` (ou `Runtime`, como o `value` da ComboBox).
pub const VALUE_PROPERTIES: [&str; 19] = [
    "text", "checked", "value", "item_index", "selected", "date", "end_date", "time", "color", "files", "html", "index", "page", "hotkey",
    "angle", "pressed", "position", "low", "high",
];

/// Mudar a propriedade pelo código dispara o `on_change` do controle (se ele tiver um):
/// as de [`VALUE_PROPERTIES`] (fora das de aparência, como o `text` de um
/// CheckBox, que é a legenda), a aba escolhida e o aberto/fechado.
pub fn is_value_property(info: &PropInfo) -> bool {
    match info.name {
        "tab_index" | "expanded" => true,
        name => matches!(info.category, "Data" | "Runtime") && VALUE_PROPERTIES.contains(&name),
    }
}

pub fn read_prop(component: &dyn Component, name: &str) -> Option<PropValue> {
    let mut layer = Some(component);
    while let Some(l) = layer {
        if let Some(v) = l.get_prop(name) {
            return Some(v);
        }
        layer = l.inner();
    }
    component.control().and_then(|cb| cb.get_prop(name))
}

/// Referência guardada pelas âncoras: tamanho do pai e retângulo do controle no momento
/// em que a posição foi definida.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct AnchorRef {
    pub(crate) parent: SizeF,
    pub(crate) rect: RectF,
}

/// Dados comuns de todo controle visual.
///
/// `left`, `top`, `width` e `height` são os valores **pedidos**. A posição e o tamanho
/// **reais**, calculados pelo layout (Align/Anchors), ficam em [`bounds`](Self::bounds).
#[derive(Clone, Debug)]
pub struct ControlBase {
    /// Posição a partir do canto do pai; só vale com `align: none`.
    left: f32,
    /// Posição a partir do topo do pai; só vale com `align: none`.
    top: f32,
    /// Largura pedida; o `align` pode esticar (top, bottom, client).
    width: f32,
    /// Altura pedida; o `align` pode esticar (left, right, client).
    height: f32,
    /// Espaço por fora, até os vizinhos e a borda do pai: número, `[h, v]` ou `[esquerda, cima, direita, baixo]`.
    margins: Bounds,
    /// Espaço por dentro, até os filhos: número, `[h, v]` ou `[esquerda, cima, direita, baixo]`.
    padding: Bounds,
    /// Encaixe no pai: `top`/`bottom` esticam na largura, `left`/`right` na altura, `client` ocupa o resto; `none` usa `left`/`top`.
    align: Align,
    /// Com `align: none`, os lados que acompanham o pai quando ele muda de tamanho: `left,right` estica na largura, só `right` anda junto com a borda direita.
    anchors: Anchors,
    /// Aparece; escondido não ocupa lugar no layout.
    visible: bool,
    /// Aceita clique e teclado; desligado aparece apagado, e os filhos também.
    enabled: bool,
    /// Recebe o mouse; desligado, o clique passa para o que está embaixo.
    hit_test: bool,
    /// Corta o desenho dos filhos na borda dele.
    clip_children: bool,
    /// Pinta com o estado do pai (`hover`, `pressed`, `checked`): o rótulo da linha marcada
    /// fica com a cor de marcado do tema sem trocar o estilo filho a filho (C25 do Tabernáculo).
    inherit_state: bool,
    /// Os textos dele passam pela tradução do app (o padrão). Desligado, ficam como estão e
    /// não entram na lista do que traduzir: um nome próprio, um código, um valor de exemplo
    /// (G19 do Tabernáculo).
    translatable: bool,
    /// A roda do mouse em cima dele fica com ele (não rola os contêineres de fora), mesmo
    /// sem ele rolar: uma prévia que dá zoom com a roda, um mapa, um gráfico.
    captures_wheel: bool,
    /// Pode receber o foco do teclado (clique ou Tab).
    can_focus: bool,
    /// Pode ser arrastado (o `drag` diz o que vai junto).
    draggable: bool,
    /// Aceita o que é solto em cima dele (`on_drop`).
    accepts_drops: bool,
    /// O arrasto declarado (sem handler): o que vai (`"cliente:123"`, o tipo antes do primeiro
    /// `:`) e o rótulo que acompanha o mouse; os tipos que este destino aceita (`"cliente;produto"`).
    drag: Option<Box<DragDecl>>,
    /// Entra na ordem do Tab.
    tab_stop: bool,
    /// No toque, arrastar rola o contêiner de fora em vez de ir para o controle.
    pan_on_touch: bool,
    /// Menu de contexto: abre no clique direito.
    pub(crate) popup_menu: Option<ComponentId>,
    /// O cursor do mouse em cima dele.
    cursor: CursorShape,
    /// Transparência de 0 (invisível) a 1, com os filhos; continua no layout e clicável.
    opacity: f32,
    /// Como o controle (com os filhos) se junta ao que está por baixo (o `mix-blend-mode`
    /// do CSS): `difference` com branco é o negativo, como a marca d'água sobre um vídeo.
    blend_mode: crate::BlendMode,
    /// Giro em graus, em torno do centro.
    rotation: f32,
    /// A escala do controle e de tudo dentro dele (1 = normal): ele ocupa o
    /// mesmo retângulo no pai, e o desenho dele e o layout dos filhos acontecem num espaço do
    /// tamanho do retângulo dividido pela escala, ampliado na pintura. O clique, o foco e a
    /// área a repintar acompanham ([`ControlBase::inner_size`]).
    scale: f32,
    /// Deslocamento só do desenho (não muda o layout): animações e transições.
    translate: PointF,
    /// O desenho dele e dos filhos guardado numa imagem, refeita só quando algo lá dentro
    /// muda: um painel pesado que desliza ou some aos poucos.
    cache_layer: bool,
    /// Proporção fixa (largura ÷ altura; 0 = livre), como o `VResponsive` do Vuetify.
    aspect_ratio: f32,
    /// Largura máxima (0 = sem limite), como o `max-width` do CSS com margem automática.
    max_width: f32,
    /// Largura mínima, altura mínima e máxima (0 = sem limite).
    min_width: f32,
    /// Altura mínima: o layout e o `auto_size` não o deixam mais baixo (0 = sem limite).
    min_height: f32,
    /// Altura máxima: o layout e o `auto_size` não o deixam mais alto (0 = sem limite; a mínima vence).
    max_height: f32,
    /// Encaixado: estica na faixa ou fica com o próprio tamanho no começo, meio ou fim.
    self_align: SelfAlign,
    /// O tamanho sai do conteúdo: a altura; num Label encaixado à esquerda ou à direita, também a largura.
    auto_size: bool,
    /// Quantas colunas ocupa num `GridLayout` (o `colspan`; 1 ou 0: uma).
    column_span: u32,
    /// O lugar fixo num `GridLayout`: linha e coluna (-1: segue a ordem) e
    /// quantas linhas ocupa. O relatório do Tabernáculo, G12.
    grid_row: i16,
    /// A coluna fixa num `GridLayout`, contando de 0 (-1: segue a ordem); vale junto com o `grid_row`.
    grid_column: i16,
    /// Quantas linhas ocupa num `GridLayout` (1 ou 0: uma).
    row_span: u16,
    /// Quanto do espaço que sobra na linha de um `FlowLayout` ele toma (o `flex-grow`): um
    /// `Layout` vazio com `grow` 1 é o espaçador que empurra o resto para o fim (L04).
    grow: f32,
    /// As escalas de fonte e de espaços da vista ativa do form (postas pelo layout; veja
    /// `views.rs`): os getters de `margins`/`padding` e a fonte da pintura já saem escalados.
    pub(crate) text_scale: f32,
    pub(crate) spacing_scale: f32,
    /// Sombra do controle; quando há, vale no lugar da sombra do estilo.
    shadow: Option<crate::canvas::Shadow>,
    /// Um número livre para o código (não muda nada na tela).
    tag: i64,
    /// Os textos que quase nenhum controle usa (o `tag_string`, o `accessible_name`), numa
    /// caixa só, criada quando um deles tem algo: 8 bytes por controle.
    rare: Option<Box<RareTexts>>,
    /// A variante do tema (`primary`, `outline`, `flat`, `card`...): o estilo `Classe.variante`.
    style: String,
    /// Tema próprio (pelo nome), que vale também para os filhos. Vazio = o do pai.
    theme: String,
    /// Dica mostrada com o mouse parado em cima.
    hint: String,
    state: ControlState,
    pub(crate) bounds: RectF,
    pub(crate) anchor_ref: Option<AnchorRef>,
    /// Controle flutuante preso a outro (ver [`Ui::follow`](crate::Ui::follow)): o alvo e a
    /// distância do canto dele (medida no primeiro layout).
    pub(crate) follow: Option<(ComponentId, Option<PointF>)>,
    /// `visible` desligado pela propriedade no designer: o controle continua no layout e
    /// clicável, pintado translúcido (em execução ele some).
    pub(crate) design_hidden: bool,
    /// Só aparece no designer (o relatório: em execução, só a visualização o mostra).
    pub(crate) design_only: bool,
    /// Fica da esquerda para a direita mesmo com a interface espelhada (árabe), ele e os
    /// filhos: o designer da IDE, um gráfico, um mapa.
    pub(crate) keep_left_to_right: bool,
    /// As cores próprias (vazio = as do tema).
    pub(crate) colors: ColorOverride,
    /// A fonte própria; o que ficar sem valor vem do pai.
    pub(crate) font: FontOverride,
    /// Quanto o conteúdo está rolado.
    scroll_offset: PointF,
    /// A rolagem com que o último layout pôs os filhos: a posição de um filho no conteúdo é a
    /// dele mais esta, mesmo antes do próximo layout.
    pub(crate) laid_out_scroll: PointF,
    /// Faixas da tela cobertas pelo sistema (só no form; ver [`Ui::set_safe_area`](crate::Ui::set_safe_area)).
    pub(crate) safe_area: Bounds,
    /// Tamanho ocupado pelos filhos, calculado no layout (sem a rolagem).
    pub(crate) content_size: SizeF,
}

impl Default for ControlBase {
    fn default() -> Self {
        Self::new(100.0, 100.0)
    }
}

impl ControlBase {
    /// Propriedades publicadas comuns a todos os controles.
    pub const PROPS: &'static [PropInfo] = &[
        PropInfo::new("left", PropKind::Float, "Layout"),
        PropInfo::new("top", PropKind::Float, "Layout"),
        PropInfo::new("width", PropKind::Float, "Layout"),
        PropInfo::new("height", PropKind::Float, "Layout"),
        PropInfo::new("align", PropKind::Enum(<Align as crate::prop::PropEnum>::NAMES), "Layout"),
        PropInfo::new("anchors", PropKind::Set(&Anchors::NAMES), "Layout"),
        PropInfo::new("margins", PropKind::Bounds, "Layout"),
        PropInfo::new("padding", PropKind::Bounds, "Layout"),
        PropInfo::new("visible", PropKind::Bool, "Behavior"),
        PropInfo::new("enabled", PropKind::Bool, "Behavior"),
        PropInfo::new("hit_test", PropKind::Bool, "Behavior"),
        PropInfo::new("clip_children", PropKind::Bool, "Behavior"),
        PropInfo::new("inherit_state", PropKind::Bool, "Appearance"),
        PropInfo::new("translate", PropKind::Bool, "Behavior"),
        PropInfo::new("captures_wheel", PropKind::Bool, "Behavior"),
        PropInfo::new("can_focus", PropKind::Bool, "Behavior"),
        PropInfo::new("draggable", PropKind::Bool, "Behavior"),
        PropInfo::new("accepts_drops", PropKind::Bool, "Behavior"),
        PropInfo::new("drag_data", PropKind::String, "Behavior"),
        PropInfo::new("drag_label", PropKind::String, "Behavior"),
        PropInfo::new("drag_image", PropKind::Component, "Behavior"),
        PropInfo::new("drop_types", PropKind::String, "Behavior"),
        PropInfo::new("tab_stop", PropKind::Bool, "Behavior"),
        PropInfo::new("cursor", PropKind::Enum(<CursorShape as crate::prop::PropEnum>::NAMES), "Behavior"),
        PropInfo::new("opacity", PropKind::Float, "Appearance"),
        PropInfo::new("blend_mode", PropKind::Enum(<crate::BlendMode as crate::prop::PropEnum>::NAMES), "Appearance"),
        PropInfo::new("cache_layer", PropKind::Bool, "Appearance"),
        PropInfo::new("rotation", PropKind::Angle, "Layout"),
        PropInfo::new("scale", PropKind::Float, "Layout"),
        PropInfo::new("translate_x", PropKind::Float, "Appearance"),
        PropInfo::new("translate_y", PropKind::Float, "Appearance"),
        PropInfo::new("aspect_ratio", PropKind::Float, "Layout"),
        PropInfo::new("max_width", PropKind::Float, "Layout"),
        PropInfo::new("min_width", PropKind::Float, "Layout"),
        PropInfo::new("min_height", PropKind::Float, "Layout"),
        PropInfo::new("max_height", PropKind::Float, "Layout"),
        PropInfo::new("self_align", PropKind::Enum(<SelfAlign as crate::prop::PropEnum>::NAMES), "Layout"),
        PropInfo::new("auto_size", PropKind::Bool, "Layout"),
        PropInfo::new("column_span", PropKind::Int, "Layout"),
        PropInfo::new("grid_row", PropKind::Int, "Layout"),
        PropInfo::new("grid_column", PropKind::Int, "Layout"),
        PropInfo::new("row_span", PropKind::Int, "Layout"),
        PropInfo::new("grow", PropKind::Float, "Layout"),
        PropInfo::new("shadow", PropKind::Shadow, "Appearance"),
        PropInfo::new("style", PropKind::String, "Appearance"),
        PropInfo::new("theme", PropKind::String, "Appearance"),
        PropInfo::new("fill", PropKind::Color, "Appearance"),
        PropInfo::new("border_color", PropKind::Color, "Appearance"),
        PropInfo::new("text_color", PropKind::Color, "Appearance"),
        PropInfo::new("accent_color", PropKind::Color, "Appearance"),
        PropInfo::new("success_color", PropKind::Color, "Appearance"),
        PropInfo::new("warning_color", PropKind::Color, "Appearance"),
        PropInfo::new("danger_color", PropKind::Color, "Appearance"),
        PropInfo::new("font_family", PropKind::String, "Font"),
        PropInfo::new("font_size", PropKind::Float, "Font"),
        PropInfo::new("font_weight", PropKind::Enum(&FONT_WEIGHTS), "Font"),
        PropInfo::new("font_italic", PropKind::Bool, "Font"),
        PropInfo::new("font_underline", PropKind::Bool, "Font"),
        PropInfo::new("font_strikeout", PropKind::Bool, "Font"),
        PropInfo::new("hint", PropKind::String, "Behavior"),
        PropInfo::new("tag", PropKind::Int, "Misc"),
        PropInfo::new("tag_string", PropKind::String, "Misc"),
        PropInfo::new("accessible_name", PropKind::String, "Behavior"),
        PropInfo::new("popup_menu", PropKind::Component, "Behavior"),
        PropInfo::new("on_click", PropKind::Event, "Events"),
        PropInfo::new("on_dbl_click", PropKind::Event, "Events"),
        PropInfo::new("on_long_press", PropKind::Event, "Events"),
        PropInfo::new("on_pointer_down", PropKind::Event, "Events"),
        PropInfo::new("on_pointer_up", PropKind::Event, "Events"),
        PropInfo::new("on_pointer_move", PropKind::Event, "Events"),
        PropInfo::new("on_pointer_enter", PropKind::Event, "Events"),
        PropInfo::new("on_pointer_leave", PropKind::Event, "Events"),
        PropInfo::new("on_wheel", PropKind::Event, "Events"),
        PropInfo::new("on_gesture", PropKind::Event, "Events"),
        PropInfo::new("on_swipe", PropKind::Event, "Events"),
        PropInfo::new("on_key_down", PropKind::Event, "Events"),
        PropInfo::new("on_key_press", PropKind::Event, "Events"),
        PropInfo::new("on_key_up", PropKind::Event, "Events"),
        PropInfo::new("on_enter", PropKind::Event, "Events"),
        PropInfo::new("on_exit", PropKind::Event, "Events"),
        PropInfo::new("on_resize", PropKind::Event, "Events"),
        PropInfo::new("on_drag_start", PropKind::Event, "Events"),
        PropInfo::new("on_drag_over", PropKind::Event, "Events"),
        PropInfo::new("on_drag_move", PropKind::Event, "Events"),
        PropInfo::new("on_drag_leave", PropKind::Event, "Events"),
        PropInfo::new("on_drag_drop", PropKind::Event, "Events"),
        PropInfo::new("on_drag_end", PropKind::Event, "Events"),
    ];

    pub fn new(width: f32, height: f32) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            width,
            height,
            margins: Bounds::ZERO,
            padding: Bounds::ZERO,
            align: Align::None,
            anchors: Anchors::default(),
            visible: true,
            enabled: true,
            hit_test: true,
            clip_children: false,
            inherit_state: false,
            translatable: true,
            captures_wheel: false,
            can_focus: false,
            draggable: false,
            accepts_drops: false,
            drag: None,
            tab_stop: true,
            pan_on_touch: true,
            popup_menu: None,
            cursor: CursorShape::Default,
            opacity: 1.0,
            blend_mode: crate::BlendMode::Normal,
            rotation: 0.0,
            scale: 1.0,
            translate: PointF::ZERO,
            cache_layer: false,
            aspect_ratio: 0.0,
            max_width: 0.0,
            min_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            self_align: SelfAlign::Stretch,
            auto_size: false,
            column_span: 1,
            grid_row: -1,
            grid_column: -1,
            row_span: 1,
            grow: 0.0,
            text_scale: 1.0,
            spacing_scale: 1.0,
            shadow: None,
            tag: 0,
            rare: None,
            style: String::new(),
            theme: String::new(),
            hint: String::new(),
            state: ControlState::default(),
            bounds: RectF::new(0.0, 0.0, width, height),
            anchor_ref: None,
            follow: None,
            design_hidden: false,
            design_only: false,
            keep_left_to_right: false,
            colors: ColorOverride::default(),
            font: FontOverride::default(),
            scroll_offset: PointF::ZERO,
            laid_out_scroll: PointF::ZERO,
            content_size: SizeF::ZERO,
            safe_area: Bounds::default(),
        }
    }

    // ----- leitura -----

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn position(&self) -> PointF {
        PointF::new(self.left, self.top)
    }

    pub fn size(&self) -> SizeF {
        SizeF::new(self.width, self.height)
    }

    /// As margens que valem no layout (com a escala de espaços da vista ativa; a propriedade
    /// guarda a do desenho).
    pub fn margins(&self) -> Bounds {
        if self.spacing_scale == 1.0 { self.margins } else { self.margins.scaled(self.spacing_scale) }
    }

    /// O padding que vale no layout, como as [`margins`](Self::margins).
    pub fn padding(&self) -> Bounds {
        if self.spacing_scale == 1.0 { self.padding } else { self.padding.scaled(self.spacing_scale) }
    }

    pub fn align(&self) -> Align {
        self.align
    }

    pub fn anchors(&self) -> Anchors {
        self.anchors
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Só aparece no designer (em execução não é pintado nem recebe o mouse).
    pub fn design_only(&self) -> bool {
        self.design_only
    }

    /// As cores próprias.
    pub fn colors(&self) -> ColorOverride {
        self.colors
    }

    pub fn set_colors(&mut self, colors: ColorOverride) -> &mut Self {
        self.colors = colors;
        self
    }

    pub fn set_design_only(&mut self, on: bool) -> &mut Self {
        self.design_only = on;
        self
    }

    /// Fica da esquerda para a direita (ele e os filhos) com a interface espelhada.
    pub fn keep_left_to_right(&self) -> bool {
        self.keep_left_to_right
    }

    pub fn set_keep_left_to_right(&mut self, on: bool) -> &mut Self {
        self.keep_left_to_right = on;
        self
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn hit_test(&self) -> bool {
        self.hit_test
    }

    pub fn translatable(&self) -> bool {
        self.translatable
    }

    pub fn set_translatable(&mut self, v: bool) -> &mut Self {
        self.translatable = v;
        self
    }

    pub fn inherit_state(&self) -> bool {
        self.inherit_state
    }

    pub fn set_inherit_state(&mut self, v: bool) -> &mut Self {
        self.inherit_state = v;
        self
    }

    pub fn clip_children(&self) -> bool {
        self.clip_children
    }

    pub fn captures_wheel(&self) -> bool {
        self.captures_wheel
    }

    /// Pode receber o foco do teclado (como `CanFocus`).
    /// Arrastável: passar de alguns pontos com o
    /// botão pressionado dispara `on_drag_start`, e o handler começa o arrasto com
    /// [`Ui::begin_drag`](crate::Ui::begin_drag).
    pub fn draggable(&self) -> bool {
        self.draggable
    }

    pub fn set_draggable(&mut self, v: bool) -> &mut Self {
        self.draggable = v;
        self
    }

    /// Recebe o que é arrastado: `on_drag_over` ao entrar (o handler recusa com
    /// [`Ui::reject_drop`](crate::Ui::reject_drop)) e `on_drag_drop` ao soltar.
    pub fn accepts_drops(&self) -> bool {
        self.accepts_drops
    }

    pub fn set_accepts_drops(&mut self, v: bool) -> &mut Self {
        self.accepts_drops = v;
        self
    }

    /// O que vai no arrasto declarado (vazio: só com o `on_drag_start`).
    pub fn drag_data(&self) -> &str {
        self.drag.as_ref().map_or("", |d| d.data.as_str())
    }

    pub fn drag_label(&self) -> &str {
        self.drag.as_ref().map_or("", |d| d.label.as_str())
    }

    /// O `DragImage` do arrasto (a imagem que acompanha o cursor).
    pub fn drag_image(&self) -> Option<ComponentId> {
        self.drag.as_ref().and_then(|d| d.image)
    }

    /// O `DragImage` foi liberado: a referência sai.
    pub(crate) fn forget_drag_image(&mut self, doomed: &[ComponentId]) {
        if let Some(d) = self.drag.as_mut().filter(|d| d.image.is_some_and(|i| doomed.contains(&i))) {
            d.image = None;
        }
    }

    /// Os tipos aceitos ao soltar (vazio: decide o `on_drag_over`).
    pub fn drop_types(&self) -> &str {
        self.drag.as_ref().map_or("", |d| d.drop_types.as_str())
    }

    fn drag_decl(&mut self) -> &mut DragDecl {
        self.drag.get_or_insert_with(Default::default)
    }

    pub fn can_focus(&self) -> bool {
        self.can_focus
    }

    /// Participa da navegação com Tab (como `TabStop`).
    pub fn tab_stop(&self) -> bool {
        self.tab_stop
    }

    /// Menu de contexto do controle.
    pub fn popup_menu(&self) -> Option<ComponentId> {
        self.popup_menu
    }

    pub fn set_popup_menu(&mut self, menu: Option<ComponentId>) -> &mut Self {
        self.popup_menu = menu;
        self
    }

    /// Ver [`set_pan_on_touch`](Self::set_pan_on_touch).
    pub fn pan_on_touch(&self) -> bool {
        self.pan_on_touch
    }

    /// Cursor do mouse sobre o controle. `Default` herda o do pai.
    pub fn cursor(&self) -> CursorShape {
        self.cursor
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Como o controle se junta ao que está por baixo (veja o campo).
    pub fn blend_mode(&self) -> crate::BlendMode {
        self.blend_mode
    }

    pub fn set_blend_mode(&mut self, mode: crate::BlendMode) -> &mut Self {
        self.blend_mode = mode;
        self
    }

    /// O desenho guardado numa imagem (veja o campo).
    pub fn cache_layer(&self) -> bool {
        self.cache_layer
    }

    pub fn set_cache_layer(&mut self, on: bool) -> &mut Self {
        self.cache_layer = on;
        self
    }

    /// Giro em graus, em torno do centro. Só a pintura gira; o clique usa o retângulo
    /// sem giro.
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// A escala dele e de tudo dentro dele ([`set_scale`](Self::set_scale); 1 = normal).
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// O tamanho do espaço de dentro: o do retângulo dividido pela escala. É nele que o
    /// controle se desenha (`PaintCtx::rect`) e que os filhos são arrumados.
    pub fn inner_size(&self) -> SizeF {
        let s = self.scale;
        if s == 1.0 { self.bounds.size() } else { SizeF::new(self.bounds.width / s, self.bounds.height / s) }
    }

    pub fn shadow(&self) -> Option<crate::canvas::Shadow> {
        self.shadow
    }

    /// Proporção fixa (largura ÷ altura); 0 = livre. No layout, a altura sai da largura
    /// (Top, Bottom, Horizontal, None), a largura sai da altura (Left, Right, Vertical) ou
    /// o controle cabe na área mantendo a proporção (Client, Center).
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) -> &mut Self {
        self.aspect_ratio = if ratio.is_finite() { ratio.max(0.0) } else { 0.0 };
        self
    }

    /// Largura máxima (0 = sem limite). O controle não passa dela; alinhado em cima, embaixo ou
    /// no cliente, fica no meio da área (o conteúdo de uma página que não se espalha numa tela
    /// larga). No form, é a largura máxima da janela.
    pub fn max_width(&self) -> f32 {
        self.max_width
    }

    pub fn set_max_width(&mut self, width: f32) -> &mut Self {
        self.max_width = if width.is_finite() { width.max(0.0) } else { 0.0 };
        self
    }

    /// Largura mínima (0 = sem limite). O layout não deixa o controle mais estreito que ela,
    /// mesmo alinhado (`top`, `client`...): o que passar da área do pai fica cortado (ou rola,
    /// num ScrollBox), e um texto com `wrap` para de quebrar nessa largura. No form, é a
    /// largura mínima da janela.
    pub fn min_width(&self) -> f32 {
        self.min_width
    }

    pub fn set_min_width(&mut self, width: f32) -> &mut Self {
        self.min_width = if width.is_finite() { width.max(0.0) } else { 0.0 };
        self
    }

    /// Altura mínima (0 = sem limite), como a [`min_width`](Self::min_width).
    pub fn min_height(&self) -> f32 {
        self.min_height
    }

    pub fn set_min_height(&mut self, height: f32) -> &mut Self {
        self.min_height = if height.is_finite() { height.max(0.0) } else { 0.0 };
        self
    }

    /// Altura máxima (0 = sem limite). No form, é a altura máxima da janela.
    pub fn max_height(&self) -> f32 {
        self.max_height
    }

    pub fn set_max_height(&mut self, height: f32) -> &mut Self {
        self.max_height = if height.is_finite() { height.max(0.0) } else { 0.0 };
        self
    }

    /// A largura que o controle ocupa numa faixa de `available` (com a `max_width` e a
    /// `min_width`; a mínima vence).
    pub fn clamp_width(&self, available: f32) -> f32 {
        let w = if self.max_width > 0.0 { available.min(self.max_width) } else { available };
        w.max(self.min_width)
    }

    /// A altura dentro da `min_height` e da `max_height` (a mínima vence).
    pub fn clamp_height(&self, height: f32) -> f32 {
        let h = if self.max_height > 0.0 { height.min(self.max_height) } else { height };
        h.max(self.min_height)
    }

    /// Encaixado, estica na faixa (`stretch`) ou fica com o próprio tamanho no começo, no
    /// meio ou no fim: veja [`SelfAlign`].
    pub fn self_align(&self) -> SelfAlign {
        self.self_align
    }

    pub fn set_self_align(&mut self, align: SelfAlign) -> &mut Self {
        self.self_align = align;
        self
    }

    /// A largura numa faixa horizontal de `available` (encaixado em cima ou embaixo): a faixa
    /// toda ou, com `self_align`, a própria `width`; com `self_align` e `max_width`, a faixa até
    /// a máxima (o `width: 100%; max-width` do CSS). Sempre dentro dos limites.
    pub fn band_width(&self, available: f32) -> f32 {
        match self.self_align {
            SelfAlign::Stretch => self.clamp_width(available),
            _ if self.max_width > 0.0 => self.clamp_width(available),
            _ => self.clamp_width(self.width),
        }
    }

    /// A altura numa faixa vertical de `available` (encaixado à esquerda ou à direita), como a
    /// [`band_width`](Self::band_width).
    pub fn band_height(&self, available: f32) -> f32 {
        match self.self_align {
            SelfAlign::Stretch => self.clamp_height(available),
            _ if self.max_height > 0.0 => self.clamp_height(available),
            _ => self.clamp_height(self.height),
        }
    }

    /// Onde começa um tamanho `size` numa faixa de `available`: no começo, no meio (o
    /// `stretch` com a largura máxima sobrando) ou no fim. Passando da faixa, no começo.
    pub fn band_offset(&self, available: f32, size: f32) -> f32 {
        let spare = (available - size).max(0.0);
        match self.self_align {
            SelfAlign::Start => 0.0,
            SelfAlign::End => spare,
            SelfAlign::Stretch | SelfAlign::Center => spare / 2.0,
        }
    }

    /// A altura sai do conteúdo, na largura que o controle tem:
    /// um Label com `wrap` fica da altura do texto quebrado; um contêiner, da
    /// altura dos filhos encaixados em cima e embaixo (mais o padding). Vale com `align`
    /// `top`, `bottom` ou `none`.
    pub fn auto_size(&self) -> bool {
        self.auto_size
    }

    pub fn set_auto_size(&mut self, on: bool) -> &mut Self {
        self.auto_size = on;
        self
    }

    /// Quantas colunas ocupa num `GridLayout` (pelo menos 1; numa grade com menos colunas,
    /// ocupa a linha inteira).
    pub fn column_span(&self) -> u32 {
        self.column_span.max(1)
    }

    /// O lugar fixo na grade: (linha, coluna), ou `None` (segue a ordem dos filhos).
    pub fn grid_cell(&self) -> Option<(u32, u32)> {
        (self.grid_row >= 0 && self.grid_column >= 0).then(|| (self.grid_row as u32, self.grid_column as u32))
    }

    pub fn set_grid_cell(&mut self, cell: Option<(u32, u32)>) -> &mut Self {
        let (r, c) = cell.map_or((-1, -1), |(r, c)| (r.min(i16::MAX as u32) as i16, c.min(i16::MAX as u32) as i16));
        self.grid_row = r;
        self.grid_column = c;
        self
    }

    /// Quantas linhas ocupa num `GridLayout` (o `rowspan`).
    pub fn row_span(&self) -> u32 {
        u32::from(self.row_span.max(1))
    }

    pub fn set_row_span(&mut self, span: u32) -> &mut Self {
        self.row_span = span.clamp(1, 64) as u16;
        self
    }

    pub fn grow(&self) -> f32 {
        self.grow
    }

    pub fn set_grow(&mut self, grow: f32) -> &mut Self {
        self.grow = grow.max(0.0);
        self
    }

    pub fn set_column_span(&mut self, span: u32) -> &mut Self {
        self.column_span = span.max(1);
        self
    }

    /// A escala de fonte da vista ativa do form (1 sem vista).
    pub fn text_scale(&self) -> f32 {
        self.text_scale
    }

    /// A escala de espaços da vista ativa do form (1 sem vista).
    pub fn spacing_scale(&self) -> f32 {
        self.spacing_scale
    }

    pub fn tag(&self) -> i64 {
        self.tag
    }

    /// O nome que o leitor de tela fala (o `aria-label`), no lugar do texto do controle: um
    /// botão só com ícone, um campo sem rótulo. O relatório do Tabernáculo, C29.
    pub fn accessible_name(&self) -> &str {
        self.rare.as_ref().map_or("", |r| &r.accessible_name)
    }

    pub fn set_accessible_name(&mut self, v: &str) -> &mut Self {
        self.rare_mut(|r| r.accessible_name = v.into());
        self
    }

    fn rare_mut(&mut self, change: impl FnOnce(&mut RareTexts)) {
        let rare = self.rare.get_or_insert_with(Default::default);
        change(rare);
        if rare.tag_string.is_empty() && rare.accessible_name.is_empty() {
            self.rare = None;
        }
    }

    pub fn tag_string(&self) -> &str {
        self.rare.as_ref().map_or("", |r| &r.tag_string)
    }

    /// Variante de estilo do tema ("primary", "danger", "flat"...). Vazio = estilo da classe.
    pub fn style(&self) -> &str {
        &self.style
    }

    /// Tema próprio do controle (e dos filhos), pelo nome. Vazio = o do pai.
    pub fn theme_name(&self) -> &str {
        &self.theme
    }

    pub fn set_theme_name(&mut self, v: impl Into<String>) -> &mut Self {
        self.theme = v.into();
        self
    }

    /// Dica (balão) mostrada com o mouse parado sobre o controle. Vazio = a do pai.
    pub fn hint(&self) -> &str {
        &self.hint
    }

    pub fn set_hint(&mut self, v: impl Into<String>) -> &mut Self {
        self.hint = v.into();
        self
    }

    pub fn state(&self) -> ControlState {
        self.state
    }

    /// Posição e tamanho reais, relativos ao pai, calculados pelo layout.
    pub fn bounds(&self) -> RectF {
        self.bounds
    }

    /// Quanto o conteúdo está rolado: os filhos são deslocados por este valor.
    pub fn scroll_offset(&self) -> PointF {
        self.scroll_offset
    }

    /// A rolagem com que o último layout pôs os filhos ([`scroll_offset`](Self::scroll_offset)
    /// muda na hora; esta, no layout): somada à posição de um filho, dá onde ele está no conteúdo.
    pub fn laid_out_scroll(&self) -> PointF {
        self.laid_out_scroll
    }

    /// Rola o conteúdo. Chame `invalidate` (ou mude por `ui.control_mut`) para refazer o
    /// layout.
    pub fn set_scroll_offset(&mut self, offset: PointF) -> &mut Self {
        self.scroll_offset = offset;
        self
    }

    /// Tamanho ocupado pelos filhos visíveis (com margens e padding), calculado no
    /// layout. Serve para saber até onde rolar.
    pub fn content_size(&self) -> SizeF {
        self.content_size
    }

    // ----- escrita (encadeável) -----

    // A posição e o tamanho pedidos mudaram: as âncoras medem de novo a partir daqui. Só quando
    // o valor muda de fato: regravar o mesmo valor a cada arrumação (o `auto_size`) refazia a
    // referência sempre, e o controle ancorado nunca acompanhava o pai (relatório do InsightDB,
    // 11/11: nem a célula do GridLayout, nem a janela em tela cheia).
    pub fn set_left(&mut self, v: f32) -> &mut Self {
        if self.left != v {
            self.left = v;
            self.anchor_ref = None;
        }
        self
    }

    pub fn set_top(&mut self, v: f32) -> &mut Self {
        if self.top != v {
            self.top = v;
            self.anchor_ref = None;
        }
        self
    }

    pub fn set_position(&mut self, left: f32, top: f32) -> &mut Self {
        self.set_left(left).set_top(top)
    }

    pub fn set_width(&mut self, v: f32) -> &mut Self {
        if self.width != v.max(0.0) {
            self.width = v.max(0.0);
            self.anchor_ref = None;
        }
        self
    }

    pub fn set_height(&mut self, v: f32) -> &mut Self {
        if self.height != v.max(0.0) {
            self.height = v.max(0.0);
            self.anchor_ref = None;
        }
        self
    }

    /// A largura que o próprio layout mede (o `auto_size`), como [`set_measured_height`].
    pub(crate) fn set_measured_width(&mut self, v: f32) {
        self.width = v.max(0.0);
        if let Some(r) = self.anchor_ref.as_mut() {
            r.rect.width = self.width;
        }
    }

    /// A altura que o próprio layout mede (o `auto_size`): muda o tamanho sem perder a
    /// referência das âncoras (a altura guardada nela acompanha).
    pub(crate) fn set_measured_height(&mut self, v: f32) {
        self.height = v.max(0.0);
        if let Some(r) = self.anchor_ref.as_mut() {
            r.rect.height = self.height;
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.set_width(width).set_height(height)
    }

    /// Define posição e tamanho de uma vez.
    pub fn set_rect(&mut self, left: f32, top: f32, width: f32, height: f32) -> &mut Self {
        self.set_position(left, top).set_size(width, height)
    }

    pub fn set_margins(&mut self, v: impl Into<Bounds>) -> &mut Self {
        self.margins = v.into();
        self
    }

    pub fn set_padding(&mut self, v: impl Into<Bounds>) -> &mut Self {
        self.padding = v.into();
        self
    }

    pub fn set_align(&mut self, v: Align) -> &mut Self {
        self.align = v;
        self.anchor_ref = None;
        self
    }

    pub fn set_anchors(&mut self, v: Anchors) -> &mut Self {
        self.anchors = v;
        self.anchor_ref = None;
        self
    }

    pub fn set_visible(&mut self, v: bool) -> &mut Self {
        self.visible = v;
        self
    }

    pub fn set_enabled(&mut self, v: bool) -> &mut Self {
        self.enabled = v;
        self
    }

    pub fn set_hit_test(&mut self, v: bool) -> &mut Self {
        self.hit_test = v;
        self
    }

    pub fn set_clip_children(&mut self, v: bool) -> &mut Self {
        self.clip_children = v;
        self
    }

    pub fn set_captures_wheel(&mut self, v: bool) -> &mut Self {
        self.captures_wheel = v;
        self
    }

    pub fn set_can_focus(&mut self, v: bool) -> &mut Self {
        self.can_focus = v;
        self
    }

    /// Arrastar o dedo sobre o controle rola o conteúdo (vira roda para o ScrollBox ou o
    /// DataGrid de baixo). Ligado por padrão; controles que precisam do arrasto em si (um
    /// Splitter, um designer) desligam.
    pub fn set_pan_on_touch(&mut self, v: bool) -> &mut Self {
        self.pan_on_touch = v;
        self
    }

    pub fn set_tab_stop(&mut self, v: bool) -> &mut Self {
        self.tab_stop = v;
        self
    }

    pub fn set_cursor(&mut self, v: CursorShape) -> &mut Self {
        self.cursor = v;
        self
    }

    pub fn set_opacity(&mut self, v: f32) -> &mut Self {
        self.opacity = v.clamp(0.0, 1.0);
        self
    }

    /// Deslocamento do desenho (sem mexer no layout).
    pub fn translate(&self) -> PointF {
        self.translate
    }

    pub fn set_translate(&mut self, offset: PointF) -> &mut Self {
        self.translate = if offset.x.is_finite() && offset.y.is_finite() { offset } else { PointF::ZERO };
        self
    }

    /// A escala dele e dos filhos (1 = normal; de 0,05 a 20).
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = if scale.is_finite() && scale > 0.0 { scale.clamp(0.05, 20.0) } else { 1.0 };
        self
    }

    pub fn set_rotation(&mut self, degrees: f32) -> &mut Self {
        self.rotation = if degrees.is_finite() { degrees % 360.0 } else { 0.0 };
        self
    }

    pub fn set_shadow(&mut self, shadow: Option<crate::canvas::Shadow>) -> &mut Self {
        self.shadow = shadow;
        self
    }

    pub fn set_tag(&mut self, v: i64) -> &mut Self {
        self.tag = v;
        self
    }

    pub fn set_tag_string(&mut self, v: &str) -> &mut Self {
        self.rare_mut(|r| r.tag_string = v.into());
        self
    }

    pub fn set_style(&mut self, v: impl Into<String>) -> &mut Self {
        self.style = v.into();
        self
    }

    /// Estado visual (hover, pressed...). Normalmente quem altera é o despacho de eventos;
    /// controles como CheckBox alteram `checked`.
    pub fn state_mut(&mut self) -> &mut ControlState {
        &mut self.state
    }

    // ----- propriedades por nome -----

    pub fn get_prop(&self, name: &str) -> Option<PropValue> {
        Some(match name {
            "left" => self.left.into(),
            "top" => self.top.into(),
            "width" => self.width.into(),
            "height" => self.height.into(),
            "align" => self.align.into(),
            "anchors" => self.anchors.into(),
            "margins" => self.margins.into(),
            "padding" => self.padding.into(),
            "visible" => self.visible.into(),
            "enabled" => self.enabled.into(),
            "hit_test" => self.hit_test.into(),
            "clip_children" => self.clip_children.into(),
            "inherit_state" => self.inherit_state.into(),
            "translate" => self.translatable.into(),
            "captures_wheel" => self.captures_wheel.into(),
            "can_focus" => self.can_focus.into(),
            "draggable" => self.draggable.into(),
            "accepts_drops" => self.accepts_drops.into(),
            "drag_data" => self.drag_data().into(),
            "drag_label" => self.drag_label().into(),
            "drag_image" => PropValue::Component(self.drag_image()),
            "drop_types" => self.drop_types().into(),
            "tab_stop" => self.tab_stop.into(),
            "cursor" => PropValue::enum_value(self.cursor),
            "opacity" => self.opacity.into(),
            "blend_mode" => PropValue::enum_value(self.blend_mode),
            "cache_layer" => self.cache_layer.into(),
            "rotation" => self.rotation.into(),
            "scale" => self.scale.into(),
            "translate_x" => self.translate.x.into(),
            "translate_y" => self.translate.y.into(),
            "aspect_ratio" => self.aspect_ratio.into(),
            "max_width" => self.max_width.into(),
            "min_width" => self.min_width.into(),
            "min_height" => self.min_height.into(),
            "max_height" => self.max_height.into(),
            "self_align" => PropValue::enum_value(self.self_align),
            "auto_size" => self.auto_size.into(),
            "column_span" => (self.column_span() as i64).into(),
            "grid_row" => i64::from(self.grid_row).into(),
            "grid_column" => i64::from(self.grid_column).into(),
            "row_span" => (self.row_span() as i64).into(),
            "grow" => f64::from(self.grow).into(),
            "shadow" => crate::prop::PropType::to_value(&self.shadow),
            "style" => self.style.clone().into(),
            "fill" => crate::prop::PropType::to_value(&self.colors.fill),
            "border_color" => crate::prop::PropType::to_value(&self.colors.border),
            "text_color" => crate::prop::PropType::to_value(&self.colors.text),
            "accent_color" => crate::prop::PropType::to_value(&self.colors.accent),
            "success_color" => crate::prop::PropType::to_value(&self.colors.success),
            "warning_color" => crate::prop::PropType::to_value(&self.colors.warning),
            "danger_color" => crate::prop::PropType::to_value(&self.colors.danger),
            "font_family" => self.font.family.clone().into(),
            "font_size" => crate::prop::PropType::to_value(&self.font.size),
            "font_weight" => PropValue::Enum(FONT_WEIGHTS[self.font.weight.map_or(0, |w| w as usize + 1)].into()),
            "font_italic" => self.font.italic.into(),
            "font_underline" => self.font.underline.into(),
            "font_strikeout" => self.font.strikeout.into(),
            "theme" => self.theme.clone().into(),
            "hint" => self.hint.clone().into(),
            "tag" => self.tag.into(),
            "tag_string" => self.tag_string().to_string().into(),
            "accessible_name" => self.accessible_name().to_string().into(),
            "popup_menu" => PropValue::Component(self.popup_menu),
            _ => return None,
        })
    }

    pub fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        match name {
            "left" => self.set_left(value.to_f32()?),
            "top" => self.set_top(value.to_f32()?),
            "width" => self.set_width(value.to_f32()?),
            "height" => self.set_height(value.to_f32()?),
            "align" => self.set_align(value.to_enum()?),
            "anchors" => self.set_anchors(value.to_anchors()?),
            "margins" => self.set_margins(value.to_bounds()?),
            "padding" => self.set_padding(value.to_bounds()?),
            "visible" => self.set_visible(value.to_bool()?),
            "enabled" => self.set_enabled(value.to_bool()?),
            "hit_test" => self.set_hit_test(value.to_bool()?),
            "clip_children" => self.set_clip_children(value.to_bool()?),
            "inherit_state" => self.set_inherit_state(value.to_bool()?),
            "translate" => self.set_translatable(value.to_bool()?),
            "captures_wheel" => self.set_captures_wheel(value.to_bool()?),
            "can_focus" => self.set_can_focus(value.to_bool()?),
            "draggable" => self.set_draggable(value.to_bool()?),
            "accepts_drops" => self.set_accepts_drops(value.to_bool()?),
            "drag_data" => {
                self.drag_decl().data = value.to_str()?.to_string();
                self
            }
            "drag_label" => {
                self.drag_decl().label = value.to_str()?.to_string();
                self
            }
            "drag_image" => {
                self.drag_decl().image = value.to_component()?;
                self
            }
            "drop_types" => {
                self.drag_decl().drop_types = value.to_str()?.to_string();
                self
            }
            "tab_stop" => self.set_tab_stop(value.to_bool()?),
            "cursor" => self.set_cursor(value.to_enum()?),
            "opacity" => self.set_opacity(value.to_f32()?),
            "blend_mode" => self.set_blend_mode(value.to_enum()?),
            "cache_layer" => self.set_cache_layer(value.to_bool()?),
            "rotation" => self.set_rotation(value.to_f32()?),
            "scale" => self.set_scale(value.to_f32()?),
            "translate_x" => self.set_translate(PointF::new(value.to_f32()?, self.translate.y)),
            "translate_y" => self.set_translate(PointF::new(self.translate.x, value.to_f32()?)),
            "aspect_ratio" => self.set_aspect_ratio(value.to_f32()?),
            "max_width" => self.set_max_width(value.to_f32()?),
            "min_width" => self.set_min_width(value.to_f32()?),
            "min_height" => self.set_min_height(value.to_f32()?),
            "max_height" => self.set_max_height(value.to_f32()?),
            "self_align" => self.set_self_align(value.to_enum()?),
            "auto_size" => self.set_auto_size(value.to_bool()?),
            "column_span" => self.set_column_span(value.to_f64()?.clamp(1.0, 64.0) as u32),
            "grid_row" => {
                self.grid_row = value.to_f64()?.clamp(-1.0, 4096.0) as i16;
                self
            }
            "grid_column" => {
                self.grid_column = value.to_f64()?.clamp(-1.0, 4096.0) as i16;
                self
            }
            "row_span" => self.set_row_span(value.to_f64()?.clamp(1.0, 64.0) as u32),
            "grow" => self.set_grow(value.to_f32()?),
            "shadow" => self.set_shadow(crate::prop::PropType::from_value(value)?),
            "style" => self.set_style(value.to_str()?),
            "fill" => {
                self.colors.fill = crate::prop::PropType::from_value(value)?;
                self
            }
            "border_color" => {
                self.colors.border = crate::prop::PropType::from_value(value)?;
                self
            }
            "text_color" => {
                self.colors.text = crate::prop::PropType::from_value(value)?;
                self
            }
            "accent_color" => {
                self.colors.accent = crate::prop::PropType::from_value(value)?;
                self
            }
            "success_color" => {
                self.colors.success = crate::prop::PropType::from_value(value)?;
                self
            }
            "warning_color" => {
                self.colors.warning = crate::prop::PropType::from_value(value)?;
                self
            }
            "danger_color" => {
                self.colors.danger = crate::prop::PropType::from_value(value)?;
                self
            }
            "font_family" => {
                self.font.family = value.to_str()?.to_string();
                self
            }
            "font_size" => {
                self.font.size = crate::prop::PropType::from_value(value)?;
                self
            }
            "font_weight" => {
                self.font.weight = font_weight_named(value.to_str()?)?;
                self
            }
            "font_italic" => {
                self.font.italic = value.to_bool()?;
                self
            }
            "font_underline" => {
                self.font.underline = value.to_bool()?;
                self
            }
            "font_strikeout" => {
                self.font.strikeout = value.to_bool()?;
                self
            }
            "theme" => self.set_theme_name(value.to_str()?),
            "hint" => self.set_hint(value.to_str()?),
            "tag" => self.set_tag(value.to_i64()?),
            "tag_string" => self.set_tag_string(value.to_str()?),
            "accessible_name" => self.set_accessible_name(value.to_str()?),
            "popup_menu" => {
                self.popup_menu = value.to_component()?;
                self
            }
            _ => return Err(Error::UnknownProperty(name.to_string())),
        };
        Ok(())
    }
}

/// Os pesos da propriedade `font_weight`; `default` é o do pai (ou do tema).
pub const FONT_WEIGHTS: [&str; 8] = ["default", "thin", "light", "normal", "medium", "semibold", "bold", "black"];

fn font_weight_named(name: &str) -> Result<Option<crate::canvas::FontWeight>> {
    use crate::canvas::FontWeight::*;
    let all = [Thin, Light, Normal, Medium, SemiBold, Bold, Black];
    match FONT_WEIGHTS.iter().position(|n| n.eq_ignore_ascii_case(name)) {
        Some(0) => Ok(None),
        Some(i) => Ok(Some(all[i - 1])),
        None => Err(Error::invalid(format!("valor '{name}' inválido; use um de: {}", FONT_WEIGHTS.join(", ")))),
    }
}

/// A fonte de um controle por cima da do tema: família, tamanho,
/// peso, itálico, sublinhado e tachado. O que não tem valor vem do pai:
/// a fonte posta no form vale para tudo dentro dele.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontOverride {
    /// A família da fonte (`"Segoe UI"`, `"monospace"`); vazio: a do pai.
    pub family: String,
    /// O tamanho da fonte, em pontos; sem valor: o do pai.
    pub size: Option<f32>,
    /// O peso da fonte (`normal`, `bold`...); sem valor: o do pai.
    pub weight: Option<crate::canvas::FontWeight>,
    /// Itálico; ligado vale também para os filhos.
    pub italic: bool,
    /// Sublinhado; ligado vale também para os filhos.
    pub underline: bool,
    /// Tachado (uma linha no meio do texto); ligado vale também para os filhos.
    pub strikeout: bool,
}

impl FontOverride {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// Esta fonte com o que faltar vindo de `parent`.
    pub fn over(&self, parent: &FontOverride) -> FontOverride {
        FontOverride {
            family: if self.family.is_empty() { parent.family.clone() } else { self.family.clone() },
            size: self.size.or(parent.size),
            weight: self.weight.or(parent.weight),
            italic: self.italic || parent.italic,
            underline: self.underline || parent.underline,
            strikeout: self.strikeout || parent.strikeout,
        }
    }

    /// Aplica na fonte do estilo (a do tema).
    pub fn apply(&self, font: &mut crate::canvas::FontSpec) {
        if !self.family.is_empty() {
            font.family = self.family.clone();
        }
        if let Some(size) = self.size.filter(|s| *s > 0.0) {
            font.size = size;
        }
        if let Some(weight) = self.weight {
            font.weight = weight;
        }
        font.italic |= self.italic;
        font.underline |= self.underline;
        font.strikeout |= self.strikeout;
    }
}

impl ControlBase {
    /// A fonte própria do controle (sem a herdada).
    pub fn font(&self) -> &FontOverride {
        &self.font
    }

    pub fn set_font(&mut self, font: FontOverride) -> &mut Self {
        self.font = font;
        self
    }
}

/// As cores próprias de um controle (vazio = as do tema):
/// valem para qualquer controle (`fill`, `border_color`, `text_color`, `accent_color` e as de
/// sentido: `success_color`, `warning_color`, `danger_color`).
/// O `Ui` as põe no estilo resolvido e no tema que o controle vê (`primary` e as variações,
/// `text`, `border`), então até os controles que leem o tema direto as seguem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorOverride {
    /// A cor do fundo; vazio: a do tema.
    pub fill: Option<crate::Color>,
    /// A cor da borda; vazio: a do tema.
    pub border: Option<crate::Color>,
    /// A cor do texto; vazio: a do tema.
    pub text: Option<crate::Color>,
    /// O destaque: a marca da caixa de seleção, a barra do progresso, o trilho preenchido,
    /// a seleção, o foco.
    pub accent: Option<crate::Color>,
    /// As cores de sentido: subir/positivo, atenção e cair/erro (candles, variações, avisos).
    /// Esta é a de subir/positivo; vazio: a do tema.
    pub success: Option<crate::Color>,
    /// A cor de sentido de atenção (avisos); vazio: a do tema.
    pub warning: Option<crate::Color>,
    /// A cor de sentido de cair/erro (candles em queda, variações negativas, erros); vazio: a do tema.
    pub danger: Option<crate::Color>,
}

impl ColorOverride {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// Muda alguma cor do tema (não só o estilo).
    pub(crate) fn recolors_theme(&self) -> bool {
        !self.is_empty()
    }

    /// O tema com as cores próprias por cima.
    pub(crate) fn apply_theme(&self, theme: &crate::Theme) -> crate::Theme {
        use crate::Color;
        let mut t = theme.clone();
        let mut set = |name: &str, c: Color| {
            t.colors.insert(name.to_string(), c);
        };
        if let Some(a) = self.accent {
            let dark = luminance(a) < 0.5;
            set("primary", a);
            set("primary_hover", a.lerp(if dark { Color::WHITE } else { Color::BLACK }, 0.12));
            set("primary_pressed", a.lerp(if dark { Color::BLACK } else { Color::BLACK }, 0.15));
            set("on_primary", if luminance(a) < 0.6 { Color::WHITE } else { Color::BLACK });
            set("focus_ring", a);
            set("selection", a.with_opacity(0.25));
            set("primary_soft", a.with_opacity(0.1));
            set("primary_container", a.lerp(Color::WHITE, 0.82));
        }
        if let Some(c) = self.fill {
            // O fundo: o da superfície e o das áreas de dentro (um pouco mais escuro ou claro).
            let toward = if luminance(c) < 0.5 { Color::WHITE } else { Color::BLACK };
            set("surface", c);
            set("background", c);
            set("surface_2", c.lerp(toward.with_opacity(c.a as f32 / 255.0), 0.06));
        }
        if let Some(c) = self.text {
            set("text", c);
            set("text_muted", c.with_opacity(0.65));
            for on in ["on_primary", "on_danger", "on_success", "on_primary_container"] {
                set(on, c);
            }
        }
        if let Some(c) = self.border {
            set("border", c);
            set("track", c);
        }
        for (token, color) in [("success", self.success), ("warning", self.warning), ("danger", self.danger)] {
            if let Some(c) = color {
                set(token, c);
                set(&format!("{token}_soft"), c.with_opacity(0.1));
                set(&format!("{token}_hover"), c.lerp(Color::WHITE, 0.12));
                set(&format!("{token}_pressed"), c.lerp(Color::BLACK, 0.15));
                if token == "success" {
                    set("meter_low", c);
                } else if token == "danger" {
                    set("meter_high", c);
                } else {
                    set("meter_mid", c);
                }
            }
        }
        t
    }
}

/// Luminância relativa (0 escuro, 1 claro).
pub(crate) fn luminance(c: crate::Color) -> f32 {
    (0.299 * c.r as f32 + 0.587 * c.g as f32 + 0.114 * c.b as f32) / 255.0
}

/// As duas pontas de um conector ([`Component::connection`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Connection {
    pub from: ComponentId,
    /// O ponto de ligação em `from` (`(1, 0.5)`: o meio do lado direito).
    pub from_at: (f32, f32),
    pub to: ComponentId,
    pub to_at: (f32, f32),
}

impl Connection {
    /// Quanto a ponta sai do componente antes da primeira dobra.
    pub const STUB: f32 = 16.0;

    /// A direção para fora do lado onde está o ponto (`(1, 0.5)` → direita).
    fn normal(at: (f32, f32)) -> PointF {
        let (x, y) = at;
        let d = [(x, PointF::new(-1.0, 0.0)), (1.0 - x, PointF::new(1.0, 0.0)), (y, PointF::new(0.0, -1.0)), (1.0 - y, PointF::new(0.0, 1.0))];
        d.iter().min_by(|a, b| a.0.total_cmp(&b.0)).map_or(PointF::new(1.0, 0.0), |d| d.1)
    }

    /// O caminho em ângulos retos entre os dois pontos (nas coordenadas do pai), dados os
    /// retângulos dos dois componentes.
    pub fn route(&self, from: RectF, to: RectF) -> Vec<PointF> {
        let p0 = PointF::new(from.x + from.width * self.from_at.0, from.y + from.height * self.from_at.1);
        let p1 = PointF::new(to.x + to.width * self.to_at.0, to.y + to.height * self.to_at.1);
        let (n0, n1) = (Self::normal(self.from_at), Self::normal(self.to_at));
        let s0 = PointF::new(p0.x + n0.x * Self::STUB, p0.y + n0.y * Self::STUB);
        let s1 = PointF::new(p1.x + n1.x * Self::STUB, p1.y + n1.y * Self::STUB);
        let mut points = vec![p0, s0];
        if n0.x != 0.0 {
            // Sai na horizontal: anda até o meio em x, desce/sobe, segue.
            let mx = (s0.x + s1.x) / 2.0;
            points.push(PointF::new(mx, s0.y));
            points.push(PointF::new(mx, s1.y));
        } else {
            let my = (s0.y + s1.y) / 2.0;
            points.push(PointF::new(s0.x, my));
            points.push(PointF::new(s1.x, my));
        }
        points.push(s1);
        points.push(p1);
        points.dedup_by(|a, b| (a.x - b.x).abs() < 0.01 && (a.y - b.y).abs() < 0.01);
        points
    }
}

/// O arrasto declarado de um controle (fica fora do `ControlBase` comum: quase nenhum tem).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DragDecl {
    /// O que vai num arrasto que começa nele, sem handler (`"cliente:123"`: o tipo é o que vem
    /// antes do primeiro `:`); vazio: o arrasto declarado não leva nada.
    pub data: String,
    /// O texto que acompanha o mouse durante o arrasto; vazio: sem rótulo.
    pub label: String,
    /// Os tipos que este destino aceita, separados por `;` (`"cliente;produto"`); vazio: quem
    /// decide é o `on_drag_over`.
    pub drop_types: String,
    /// O `DragImage` que acompanha o cursor no arrasto (a imagem e o tamanho).
    pub image: Option<ComponentId>,
}

/// Os textos raros do [`ControlBase`].
#[derive(Clone, Debug, Default)]
struct RareTexts {
    /// Um texto livre para o código (um id, um caminho, um JSON pequeno).
    tag_string: Box<str>,
    /// O nome que o leitor de tela fala, no lugar do texto do controle (um botão só com
    /// ícone, um campo sem rótulo); vazio: o texto do próprio controle.
    accessible_name: Box<str>,
}
