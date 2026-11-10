# Manual: criar um componente

Há quatro níveis de personalização, do mais leve ao mais completo:

| Nível | Quando usar | Como |
|---|---|---|
| 1. Tema | mudar cores, raios, fontes de todos os controles | `Theme` / `themes/*.json` |
| 2. Subtipo (variante) | um visual alternativo de um componente existente | estilo `"Classe.nome"` no tema + `ui.register_variant` |
| 3. Pintura por instância | desenhar algo a mais num controle só | `ui.on_paint(id, ...)` |
| 4. Componente novo | comportamento ou desenho novo | `struct` + `impl Component` |

Este manual cobre os níveis 2, 3 e 4.

---

## Nível 2: um subtipo

Um subtipo é o mesmo componente com outra variante de estilo e outros valores iniciais.
Ele aparece na paleta da IDE debaixo do componente.

```rust
use aylis::prelude::*;

// 1) O visual: um estilo "Button.aviso" no tema (ou no JSON do tema).
let mut theme = ui.theme().clone();
theme.set_style(
    "Button.aviso",
    Style::new(StyleProps::new().fill("warning").text("on_primary"))
        .hover(StyleProps::new().opacity(0.9)),
);
ui.set_theme(theme);

// 2) O subtipo na paleta, com valores iniciais (opcional).
ui.register_variant("Button", "aviso", "Aviso", vec![
    ("icon", PropValue::String("warning".into())),
]);

// 3) Usar: pelo nome (forms, IDE, protocolo) ou trocando o style.
let b = ui.create("Button.aviso", Some(form))?;
ui.set_prop(outro_botao, "style", "aviso")?;
```

Só com o tema (sem `register_variant`), o subtipo aparece na paleta com o próprio nome
(`aviso`) e sem valores iniciais.

No JSON do tema:

```json
{ "styles": { "Button.aviso": { "fill": "warning", "text": "on_primary", "hover": { "opacity": 0.9 } } } }
```

---

## Nível 3: pintura por instância

```rust
// Desenha um selo vermelho por cima do botão, depois da pintura normal.
ui.on_paint(btn, |canvas, ctx, default| {
    default(canvas, ctx); // o "inherited Paint"
    let r = ctx.rect;
    canvas.fill_ellipse(RectF::new(r.right() - 10.0, r.y - 2.0, 12.0, 12.0), &Brush::Solid(ctx.theme.color("danger")));
});
```

---

## Nível 4: um componente novo

O exemplo é um `StarRating` (nota de 0 a 5 estrelas). Ele mostra os passos de qualquer
componente.

### 1. A struct e o `Default`

O `Default` é o que a paleta, o JSON e o `ui.create("StarRating")` usam para criar o
componente. Um controle visual guarda um `ControlBase` (posição, tamanho, `align`,
`visible`, `enabled`, `style`, `theme`, `hint`...).

```rust
use aylis::prelude::*;
use aylis::core::properties;

pub struct StarRating {
    pub base: ControlBase,
    /// Nota de 0 a `max`.
    pub value: i64,
    pub max: i64,
    pub read_only: bool,
    hover: Option<i64>,           // estado interno: não é propriedade
}

impl Default for StarRating {
    fn default() -> Self {
        let mut base = ControlBase::new(140.0, 28.0);      // tamanho inicial
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, value: 3, max: 5, read_only: false, hover: None }
    }
}
```

### 2. `impl Component`: nome, propriedades e eventos

```rust
impl Component for StarRating {
    fn class_name(&self) -> &'static str { "StarRating" }          // nome da classe e do estilo
    fn control(&self) -> Option<&ControlBase> { Some(&self.base) }
    fn control_mut(&mut self) -> Option<&mut ControlBase> { Some(&mut self.base) }

    // Publica as propriedades (Object Inspector, JSON, set_prop/get_prop, protocolo
    // remoto) e os eventos. A categoria agrupa no Object Inspector.
    properties! {
        value: i64 => "Data",
        max: i64 => "Behavior",
        read_only: bool => "Behavior";
        events: on_change;
        on_set: after_set                 // chamado depois de cada set_prop (opcional)
    }
    // ...pintura e eventos (abaixo)
}

impl StarRating {
    fn after_set(&mut self, _name: &str) {
        self.value = self.value.clamp(0, self.max.max(0));   // mantém coerente
    }
}
```

Tipos aceitos pela `properties!`: `bool`, inteiros, `f32`, `f64`, `String`, `Color`,
`Option<f32>`, `Option<Color>`, `Bounds`, `Anchors`, `Option<ComponentId>` (referência a
outro componente) e qualquer `enum` que implemente `PropEnum` (aparece como lista no
Object Inspector):

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Forma { #[default] Estrela, Coracao }

impl PropEnum for Forma {
    const NAMES: &'static [&'static str] = &["estrela", "coracao"];
    fn from_index(i: usize) -> Option<Self> { [Self::Estrela, Self::Coracao].get(i).copied() }
    fn index(self) -> usize { self as usize }
}
```

> Não use os nomes das propriedades do `ControlBase` (`name`, `left`, `top`, `width`,
> `height`, `align`, `style`, `theme`, `hint`, `visible`, `enabled`, `opacity`...). Para
> o nome de uma pessoa, por exemplo, use `display_name`.

### 3. Pintura

`ctx.style` traz o estilo do tema já resolvido para a classe, a variante e o estado.
`ctx.theme` traz os tokens, `ctx.rect` a área (origem 0,0) e `ctx.time` o relógio.

```rust
    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let n = self.max.max(1) as f32;
        let side = (ctx.rect.width / n).min(ctx.rect.height);
        let shown = self.hover.unwrap_or(self.value);
        for i in 0..self.max {
            let r = RectF::new(i as f32 * side, (ctx.rect.height - side) / 2.0, side, side);
            let lit = i < shown;
            let color = if lit { ctx.theme.color("warning") } else { ctx.theme.color("track") };
            ctx.draw_icon(canvas, "star", r.deflate(Bounds::all(2.0)), color);   // ícones do aylis
        }
        if ctx.state.focused {
            canvas.stroke_rect(ctx.rect, 4.0, &Stroke::new(ctx.theme.color("focus_ring"), 2.0));
        }
    }
```

Para desenhar formas livres, use `Path` (com `move_to`, `line_to`, `cubic_to`, `arc` ou
`Path::from_svg("M...")`) e `canvas.fill_path` / `canvas.stroke_path`. Para sombra,
`canvas.draw_shadow(rect, raio, &sombra)`. Para animar, chame `ctx.request_frame()`
(repinta no próximo quadro) ou `ctx.request_frame_at(t)` (num horário).

Regras da pintura:

- `paint` recebe `&self`: estado calculado na pintura (medidas para o mouse, cache) vai em
  `Cell`/`RefCell`;
- não guarde cores fixas: use os tokens do tema (`primary`, `surface`, `text`,
  `text_muted`, `border`, `success`, `warning`, `danger`, os `*_soft`...). Assim o
  componente fica certo no claro, no escuro e em qualquer tema;
- texto: `let mut t = ctx.text_style();` e ajuste `h_align`, `v_align`, `wrap`,
  `trimming`;
- quando o desenho muda, só avise (`ctx.repaint()`): a área a repintar é do núcleo, que
  mede onde cada controle desenhou na pintura anterior (a sombra, um bulbo ou uma lista que
  passa da borda entram sozinhos). Não guarde pixels nem faça a própria cópia de rolagem.
  Desenhar além de onde desenhava antes funciona, mas custa um quadro inteiro.

### 4. Mouse e teclado

```rust
    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        if self.read_only { return; }
        let side = self.base.bounds().width / self.max.max(1) as f32;
        let h = Some(((e.position.x / side).floor() as i64 + 1).clamp(0, self.max));
        if h != self.hover { self.hover = h; ctx.repaint(); }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover = None;
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        if self.read_only || e.button != PointerButton::Primary { return; }
        if let Some(v) = self.hover { self.set(ctx, v); }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, e: &KeyEvent) {
        let v = match e.key {
            Key::Right | Key::Up => self.value + 1,
            Key::Left | Key::Down => self.value - 1,
            _ => return,                       // não tratou: a tecla sobe para o pai
        };
        self.set(ctx, v);
        ctx.set_handled();
    }
```

```rust
impl StarRating {
    fn set(&mut self, ctx: &mut EventCtx<'_>, v: i64) {
        let v = v.clamp(0, self.max);
        if v != self.value {
            self.value = v;
            // Entregue aos handlers depois que este método terminar.
            ctx.fire("on_change", EventArgs::Value(v.into()));
        }
        ctx.repaint();
    }
}
```

Dentro dos métodos de evento o componente está "fora" do `Ui` (é assim que o Rust deixa o
método mexer no componente e no `Ui` ao mesmo tempo): `ctx.ui.get(ctx.id)` devolve `None`
ali, e `ctx.ui.get_prop`, `set_prop`, `invoke` e `send_message` nele respondem "está
ocupado". Use os campos do próprio `self`. Os handlers das telas não sofrem com isso: o
`ctx.fire` os entrega depois que o método termina. Os métodos disponíveis são `click`,
`pointer_down/up/move/enter/leave`, `wheel`, `key_down/up`, `text_input` (com
`accepts_text() -> true`), `focus_enter/exit`, `resized`, `timer` (com `timer_interval`),
`property_changed`, `notification` (outro componente foi liberado) e `message`.

### 5. Estilo no tema e subtipos

Dê ao componente um estilo com o nome da classe e, se quiser, variantes:

```rust
theme.set_style("StarRating", Style::new(StyleProps::new().text("text")).disabled(StyleProps::new().opacity(0.5)));
theme.set_style("StarRating.grande", Style::new(StyleProps::new().font("heading")));
ui.register_variant("StarRating", "grande", "Grande", vec![("height", PropValue::Float(40.0))]);
```

### 6. Registro e paleta

```rust
ui.register_class::<StarRating>("Meus componentes");   // aba da paleta
```

Depois do registro, o componente:

- é criado pelo nome (`ui.create("StarRating", Some(pai))`) e pelos forms em JSON;
- aparece na paleta da IDE na aba "Meus componentes", com os subtipos;
- tem as propriedades no Object Inspector e os eventos na aba de eventos;
- é comandado pelo protocolo remoto (Python).

Num projeto, registre as suas classes no `app()` (`src/lib.rs`), antes de os forms serem
carregados.

### 7. "Herdar" de um componente existente

Guarde o componente base num campo e devolva-o em `inner` / `inner_mut`. Tudo que você
não sobrescrever (propriedades, pintura, eventos, `ControlBase`) vai para ele, e o estilo
do tema também cai no da classe base quando o seu não existir:

```rust
pub struct BotaoGlow { pub button: Button, pub glow: Color }

impl Default for BotaoGlow {
    fn default() -> Self { Self { button: Button::new("Glow"), glow: Color::rgb(0x60, 0xA5, 0xFA) } }
}

impl Component for BotaoGlow {
    fn class_name(&self) -> &'static str { "BotaoGlow" }
    fn inner(&self) -> Option<&dyn Component> { Some(&self.button) }
    fn inner_mut(&mut self) -> Option<&mut dyn Component> { Some(&mut self.button) }

    properties! { glow: Color => "Appearance" }   // só as propriedades novas

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if ctx.state.hover {
            canvas.draw_shadow(ctx.rect, ctx.style.radius, &Shadow::new(self.glow, 16.0, 0.0, 0.0));
        }
        self.button.paint(canvas, ctx);           // o "inherited"
    }
}
```

`NumberBox` e `DateEdit` são feitos assim, a partir do `Edit`.

Quando chamar a base (o `inherited`): num método que **acrescenta** algo
(pintar um brilho, tratar uma tecla a mais), chame `self.base.metodo(...)` antes ou depois do
seu código; num que **substitui** (o controle desenha de outro jeito, a tecla não deve chegar
ao base), não chame. O padrão de todo método não sobrescrito já é repassar ao `inner`.
Esquecer de chamar a base num método que só acrescentava é o erro mais comum: o hover ou o
foco do base param de funcionar.

Os ganchos raros ficam fora do `Component`, para a lista de métodos de um controle comum
ficar curta: barra de tarefas, bandeja, vista nativa, conector e arquivos soltos do sistema
em `PlatformHooks`; os pontos que se arrastam no designer em `DesignHooks`. Quem usa
implementa o trait e devolve `Some(self)` em `platform_hooks`/`platform_hooks_mut` (ou
`design_hooks`/`design_hooks_mut`):

```rust
impl Component for MinhaZona {
    // ...
    fn platform_hooks(&self) -> Option<&dyn PlatformHooks> { Some(self) }
    fn platform_hooks_mut(&mut self) -> Option<&mut dyn PlatformHooks> { Some(self) }
}

impl PlatformHooks for MinhaZona {
    fn files_dropped(&mut self, ctx: &mut EventCtx<'_>, paths: &[String]) -> bool {
        ctx.fire("on_files", EventArgs::Text(paths.join("\n")));
        true
    }
}
```

### 8. Componente com filhos

Um componente pode criar filhos no `Ui` como qualquer código (por exemplo, quando é
adicionado ou numa propriedade). Para um contêiner (como o `Panel`), basta não pintar por
cima dos filhos e, se precisar, `base.set_clip_children(true)`.

Num componente composto (uma grade com um editor na célula), a conversa entre o pai e os
filhos tem uma regra, por causa do "ocupado" acima:

- **o pai pergunta aos filhos**, na hora: lê e grava neles pelo `ctx.ui` (eles estão no
  lugar; só o pai saiu);
- **o filho avisa o pai por evento** (`ctx.fire`), que chega depois que o filho termina;
- **o filho nunca chama o pai no meio de um método do pai** (grade → editor → grade): o
  pai está ocupado, e a chamada falha.

```rust
fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
    if event.key == Key::Down {
        // A grade lê o texto do editor (ele está no lugar) e grava ela mesma, no self.
        if let Ok(texto) = ctx.ui.get_prop(self.editor, "text") {
            self.celulas[self.linha][self.coluna] = texto.to_string();
        }
        self.linha += 1;
    }
}
```

Pedir ao editor "grave na grade" (`ctx.ui.invoke(self.editor, "gravar", &[])`, e o editor
chamando a grade de volta) dá "grdItens está ocupado". Adiar o recado também não serve: ele
chegaria depois de a grade descer de linha, e o valor iria para a linha errada.

### 9. Não visual

Não devolva `control()`. Ele aparece na faixa de componentes não visuais da IDE, como o
`Timer` e o `ClientDataSet`.

### 10. Teste

```rust
#[test]
fn estrelas() {
    let mut ui = Ui::new();
    ui.register_class::<StarRating>("Teste");
    let f = ui.add_root(aylis::controls::Form::new("t", 300.0, 100.0));
    let s = ui.create("StarRating", Some(f)).unwrap();
    ui.set_prop(s, "value", 4i64).unwrap();
    assert_eq!(ui.get::<StarRating>(s).unwrap().value, 4);
    ui.set_prop(s, "value", 99i64).unwrap();
    assert_eq!(ui.get::<StarRating>(s).unwrap().value, 5, "limitado pelo max");
}
```

Os testes do próprio Aylis usam um `Canvas` falso que só mede e conta o que foi
desenhado (veja `crates/aylis-controls/tests/modern.rs`).

---

## Componente dentro do próprio Aylis

Os controles do Aylis moram em crates por família, que compilam em paralelo:
`aylis-controls-base` (forms, botões, edição, listas, menus, sistema...), `aylis-controls-charts`
(gráficos, BI, trader, grafo, mapa), `aylis-controls-industrial`, `aylis-controls-device`
(sensores, permissões) e `aylis-controls-editors` (editores visuais, pintura, texto animado).
O `aylis-controls` junta todos e registra as classes. Para um componente novo:

1. crie a struct e o `impl Component` num módulo do crate da família
   (`crates/aylis-controls-<família>/src/`), usando a macro `leaf_control!("Nome")` (do
   `aylis_core`; ela escreve `class_name`, `control` e `control_mut`). Um crate de família
   depende só do `aylis-core`: um controle que usa outro controle fica no `aylis-controls-base`;
2. exporte no `lib.rs` do crate da família e em `crates/aylis-controls/src/lib.rs`
   (`pub use ...`) e registre em `aylis_controls::register`;
3. ponha o estilo padrão em `Theme::add_standard_styles` (ou em `add_variant_styles`)
   e os subtipos em `register_variants`;
4. acrescente a classe em `aylis_build::CLASSES` e em `aylis::classes` (o teste
   `crates/aylis/tests/classes.rs` falha se esquecer): é o que permite ao app publicado
   levar só os componentes que usa (veja [Carregamento e app enxuto](carregamento.md));
5. escreva os testes em `crates/aylis-controls/tests/`.
