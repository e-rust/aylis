# Lottie, SVG, Markdown e WebView

Quatro componentes novos. Cada um vem numa feature da fachada `aylis`, então o app só leva
os que os forms usam:

| Componente | Paleta | Crate (feature) |
|---|---|---|
| `Lottie` | Media | `aylis-vector` (`vector`) |
| `Svg` | Additional | `aylis-vector` (`vector`) |
| `Markdown` | Additional | `aylis-markdown` (`markdown`) |
| `WebView` | Additional | `aylis-webview` (`webview`) |

## Lottie

O `Lottie` toca animações Lottie, o JSON que o After Effects exporta pelo Bodymovin. O
player é do próprio Aylis e desenha pelo `Canvas`, então sai igual no Skia, no Direct2D e
no navegador, e fica nítido em qualquer tamanho.

```rust
let anim = ui.add(Lottie::new("noto:fogo"), form);   // do pacote padrão
ui.set_prop(anim, "repeat", false)?;
ui.set_prop(anim, "speed", 1.5)?;
// Uma animação lida de qualquer lugar (por exemplo, baixada):
ui.get_mut::<Lottie>(anim).unwrap().load_json(&bytes)?;
```

Propriedades:

- `source`: `"pacote:nome"` ou um caminho no projeto (`assets/lottie/carregando.json`).
- `playing`, `repeat` e `speed`.
- `position`: em segundos; é onde a animação fica parada.
- `fit`: contain, cover, stretch...

**Pacote padrão: os emojis animados Noto do Google**, os mesmos 55 das figurinhas do
conversa-web.

- O `aylis-tool resources --lottie` baixa o pacote para `resources/lottie/noto/`.
- **Projeto → Animações…** mostra todas tocando. Com um `Lottie` selecionado, o OK põe a
  animação nele.
- Ao salvar e ao gerar código, o projeto recebe em `assets/lottie/noto/` só as animações
  citadas, junto com a licença.
- A licença é CC BY 4.0, que exige crédito no app: *"Animated Noto Emoji © Google, CC BY
  4.0"* (por exemplo, na tela Sobre).

**O que o player cobre:**

- camadas de forma, sólidas, nulas e pré-composições (com tempo remapeado);
- parentesco;
- transformações animadas, com curvas de velocidade e caminho de movimento;
- grupos, caminhos, elipses e retângulos;
- preenchimentos e contornos sólidos ou em gradiente (linear e radial), com "Trim Paths";
- máscaras;
- **track mattes** (alfa e alfa invertido).

Fica de fora: texto, imagens, efeitos e expressões.

Os mattes usam uma máscara de alfa de verdade no Skia e no Direct2D. No Direct2D, o
matte invertido usa um recorte pelo contorno. No navegador, os dois tipos usam o recorte
pelo contorno.

**Conferido:** as 55 animações foram comparadas com o lottie-web oficial, quadro a quadro,
no meio da animação, e saem iguais no Skia e no Direct2D (testes em
`crates/aylis-vector/tests/noto.rs`).

## Svg

O `Svg` desenha uma imagem SVG como vetor.

- `source`: o arquivo no projeto, por exemplo `assets/images/logo.svg`.
- `color`: o `currentColor`. Sem ela, vale a cor de texto do tema, então ícones em SVG
  acompanham o tema claro e o escuro.
- `fit`: como a imagem se encaixa no controle.
- `highlight`: os `id` (separados por `;`) pintados com `highlight_color` (sem ela, a cor
  de destaque do tema). Num mapa, o estado ou o município escolhido.
- `zoom_to`: mostra só a área do elemento com esse `id` (com uma folga). Com o mapa do
  estado como `source`, o município sozinho, com os vizinhos em volta.
- Pelo código: `set_svg(texto)`.

**O que lê:**

- elementos `path`, `rect` (com cantos), `circle`, `ellipse`, `line`, `polyline` e
  `polygon`;
- `g` com `transform`, `use` e `svg` aninhado;
- `fill` e `stroke`: cor, `currentColor` e gradientes linear e radial (com `href`,
  `gradientUnits` e `gradientTransform`);
- `opacity`, `fill-opacity` e `stroke-opacity`;
- `fill-rule="evenodd"`, pontas, junções e tracejado;
- `vector-effect="non-scaling-stroke"` (o traço não engrossa com o `zoom_to`);
- os atributos ou o `style="..."`.

Fica de fora: texto, filtros, máscaras, `clipPath`, imagens e folhas `<style>`.

`SvgImage` e `Animation` também servem sozinhos, num `paint` próprio:
`svg.render(canvas, rect, cor)` e `anim.render(canvas, quadro, rect)`.

## Markdown

O `Markdown` mostra texto em Markdown com as cores e as fontes do tema.

- Lê CommonMark mais tabelas, riscado e listas de tarefas.
- Mostra títulos (os níveis 1 e 2 têm uma linha embaixo), negrito, itálico, código em
  linha e em bloco, citações, listas (numeradas, com marcadores e de tarefas, aninhadas),
  tabelas com alinhamento por coluna, linha horizontal e imagens do projeto.
- O texto quebra por palavra, medido com a fonte real do renderizador.
- Rola com a roda do mouse.
- O clique num link dispara o `on_link`.

```rust
let md = ui.add(Markdown::new("# Ajuda\n\nVeja o [site](https://aylis.dev)."), form);
ui.on(md, "on_link", |_, _, args| {
    if let EventArgs::Text(url) = args { abrir_no_navegador(url) }
});
```

Propriedades:

- `text`, ou `source` (um `.md` do projeto);
- `font_size`: 0 usa o tamanho do tema.

`content_height()` informa a altura do texto, para quem quiser ajustar o tamanho do
controle.

## WebView

O `WebView` mostra uma página da web dentro do form.

O controle não desenha a página. Ele pede uma vista nativa (`Component::native_view`), e a
plataforma põe um navegador de verdade sobre ele, acompanhando a posição, o tamanho, a
visibilidade e a rolagem dos contêineres:

| Onde | Navegador | Situação |
|---|---|---|
| Windows | WebView2 (Edge, já vem no Windows 10 e 11) | testado |
| macOS | WKWebView | compila; não testado |
| Navegador (wasm) | `<iframe>` sobre o canvas | testado |
| Linux, Android, iOS | — | por enquanto o controle mostra um aviso |

No designer da IDE não abre navegador: o controle mostra uma barra de endereço com a URL.

```rust
let web = ui.add(WebView::new("https://aylis.dev"), form);
ui.on(web, "on_message", |ui, id, args| {
    // A página mandou: window.ipc.postMessage("...") (desktop)
    // ou parent.postMessage("...", "*") (navegador).
    if let EventArgs::Text(t) = args {
        ui.get::<WebView>(id).unwrap().eval(format!("mostrar('{t}')"));
    }
});
```

**Propriedades:** `url`, ou `html` para mostrar um HTML dado.

**Eventos:**

- `on_load`: a página terminou de carregar.
- `on_navigate`: o navegador vai abrir um endereço.
- `on_message`: a página mandou uma mensagem.

**Métodos:** `navigate`, `load_html`, `eval`, `back`, `forward` e `reload`.

No navegador (wasm), o `eval` e o histórico só funcionam com páginas do mesmo site, por
política do próprio navegador.

O exemplo `cargo run -p aylis-platform-winit --example browser --features webview` mostra
a página e o app conversando. No demo web, a URL `?novos` mostra WebView, Markdown e Svg
juntos.

## Por dentro

- **Canvas:** o `Path` ganhou `even_odd` (regra par-ímpar). O `Canvas` ganhou
  `clip_path(path, inverse)` e a máscara de alfa (`begin_masked`, `begin_mask` e
  `end_mask`), implementados no Skia, no Direct2D e no navegador. No navegador a máscara
  ainda usa o recorte pelo contorno.
- **Vistas nativas:**
  - `Component::native_view` e `take_native_commands` são o gancho do componente;
  - `Ui::native_views(form)` lista onde cada vista fica;
  - o módulo `native` do `aylis-platform-winit` (feature `webview`) cria as vistas e entrega
    os eventos.
