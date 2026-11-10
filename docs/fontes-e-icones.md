# Fontes e ícones offline

A IDE traz um catálogo de fontes do Google Fonts e pacotes de ícones prontos, sem precisar
de internet. O app final leva só o que usa: as famílias de fonte marcadas no projeto e os
ícones que os forms e o código citam.

Sem isso, seria pôr os `.ttf` junto do app e montar uma lista de imagens à mão. Aqui a IDE
faz as duas coisas.

## O que vem com a IDE (`resources/`)

O `aylis-tool resources` monta a pasta `resources/` na raiz do Aylis. Ele roda uma vez
e baixa tudo (HTTPS, sem precisar do `curl` nem de Python).

| Recurso | O que é | Licença | Tamanho |
|---|---|---|---|
| `resources/fonts/` | 32 famílias do Google Fonts: Inter, Roboto, Roboto Mono, Open Sans, Lato, Montserrat, Poppins, Noto Sans, JetBrains Mono, Fira Code, Noto Color Emoji e outras. `catalog.json` traz família, pasta, licença, categoria e arquivos | SIL OFL 1.1 (algumas Apache-2.0 ou Ubuntu Font Licence) | 86 MB |
| `resources/icons/material.txt` (`-rounded`, `-sharp`) | Material Symbols, peso 400: cerca de 3.900 ícones, cada um com a variante `-fill` | Apache-2.0 (Google) | 3,0 MB (rounded 5,0 MB, sharp 2,5 MB) |
| `resources/icons/fluent.txt` | Fluent System Icons, 24 px: cerca de 2.500 ícones, cada um com a variante `-fill` | MIT (Microsoft) | 2,3 MB |

**Licenças.** As licenças do Google Fonts (OFL, Apache e UFL) deixam embutir a fonte no
app e distribuí-la junto, inclusive em app comercial. A condição é levar o texto da
licença, e a IDE copia esse texto junto com a fonte. Os Material Symbols (Apache-2.0) e os
Fluent (MIT) também podem ir no app, desde que levem a licença: cada pacote gerado já traz
o texto dela no cabeçalho.

A IDE não traz as fontes e os símbolos da Apple (SF Pro, SF Symbols) nem a Segoe UI e os
ícones da Microsoft que vêm com o Windows. As licenças deles só permitem usar no próprio
sistema, não embutir no app. Os temas do iOS e do Windows pedem essas fontes pelo nome:
quando o sistema tem a fonte, ela aparece; quando não tem, o sistema usa a fonte padrão
dele.

## Fontes

**Projeto → Opções do projeto → Fontes** abre o catálogo:

- **Marcar** uma família copia os arquivos e a licença dela para
  `assets/fonts/<pasta>/` (por exemplo `assets/fonts/roboto/OFL.txt`).
- **Desmarcar** apaga a pasta.
- **Clicar no nome** mostra a prévia em vários tamanhos e em negrito.
- **Fonte do projeto** troca a fonte do tema em todos os estilos. As fontes
  monoespaçadas ficam como estão. O valor vai para o campo `"font"` do `.aylisproj`.
- **Baixar** traz outra família do Google Fonts (pelo nome, por exemplo "Fira Sans") direto
  do repositório `google/fonts` e põe no catálogo.
- **Em uso no projeto** lista cada família que o projeto usa (a fonte do projeto, o
  `font_family` dos componentes nos forms e o `family` dos temas próprios em `themes/`),
  com onde aparece. O **trocar por** de cada uma troca a família em todos esses lugares
  de uma vez, inclusive nos forms abertos no designer. Por código:
  `aylis_project::assets::fonts_in_use` e `replace_font`.
- **Fontes nomeadas** funcionam como as cores do projeto: um nome (`titulo`, `corpo`...)
  aponta para uma família, e os componentes usam o nome no `font_family`. Trocar a família
  do nome troca em todos de uma vez. Ficam no `.aylisproj`:

  ```json
  "fonts": { "titulo": "Montserrat", "corpo": "Inter" }
  ```

  O nome também vale no `family` dos estilos de um tema, e um nome pode apontar para outro.
  No Object Inspector, o editor de fonte lista as nomeadas primeiro. Por código:
  `ui.set_font_names(vec![("titulo".into(), "Montserrat".into())])` (o `install` do
  projeto já faz isso) e `ui.resolve_font("titulo")`.

Ao abrir o app, as fontes de `assets/fonts/` (`.ttf`, `.otf`, `.ttc`) são registradas
antes do tema:

```rust
// O que install() faz; num app sem projeto dá para chamar direto:
let family = aylis_core::fonts::register(std::fs::read("Roboto.ttf")?).unwrap(); // "Roboto"
let _ = ui.set_prop(label, "font_family", family);
```

Os três renderizadores usam as fontes registradas:

- **Skia:** `TypefaceFontProvider`, procurado antes das fontes do sistema.
- **Direct2D:** coleção do DirectWrite com as fontes do sistema mais as registradas. Precisa
  do Windows 10 1703 ou mais novo.
- **Navegador:** `FontFace`. O texto aparece assim que a fonte carrega.

### O mesmo texto em todas as plataformas

Cada plataforma mede e quebra o texto com o próprio motor (Skia, DirectWrite, o navegador),
como no FMX. A maior diferença vem da fonte do sistema, que muda de uma plataforma para outra:
o mesmo "Salvar alterações" ocupa uma largura no Windows (Segoe UI), outra no Android (Roboto)
e outra no Mac (SF). Por isso:

- o projeto novo já vem com uma fonte embutida (a Inter, em `assets/fonts/inter/`) como fonte
  do projeto. Com a mesma fonte, o Skia e o DirectWrite dão a mesma largura e as mesmas
  quebras de linha (o teste `text_metrics` do `aylis-controls` confere);
- a verificação do projeto avisa quando ele usa a fonte do sistema, ou quando um componente
  pede uma família que não está em `assets/fonts/` (`aylis_project::check::fonts`);
- caracteres que a fonte embutida não tem (árabe, japonês, emoji) caem na fonte do sistema, e
  ali a altura da linha ainda muda um pouco de uma plataforma para outra;
- deixe folga nos textos (um botão um pouco mais largo que o texto, não justo) e use
  `auto_size` onde o texto varia (traduções, valores, nomes): o controle cresce com o texto em
  vez de cortar.

#### O modo de texto (`"text"` no `.aylisproj`)

Para o texto sair igual mesmo entre motores diferentes, o projeto escolhe, por plataforma,
quem mede e desenha:

```json
"text": { "windows": "core", "linux": "core", "macos": "core", "android": "core", "ios": "core", "web": "layout" }
```

- `native`: cada renderizador mede e desenha (o de antes; um projeto sem o campo fica assim);
- `layout`: o núcleo mede com as fontes do projeto (`ttf-parser` e `rustybuzz`, em Rust puro)
  e decide as quebras e os tamanhos; o renderizador só desenha cada linha no lugar dado. As
  medidas são as mesmas em todo lugar (o teste `text_modes` do `aylis-controls` mede com o
  Skia, com o DirectWrite e sem renderizador nenhum, e dá igual);
- `core`: o núcleo também desenha as letras (o contorno de cada uma vira um caminho): entre o
  Skia e o Direct2D, 0,01% de pixels diferentes num form de rótulos, contra 1% no `native`.

O projeto novo vem com `core` no desktop e no celular e `layout` na web (no navegador, o
`core` levaria o motor e as fontes no download). Não há opção por form: o mesmo texto seria
medido de dois jeitos na mesma janela. O núcleo só conhece as fontes do projeto: um texto com
um caractere que nenhuma delas tem (um emoji, o japonês sem uma fonte japonesa) fica com o
renderizador, como no `native`. Muda em Projeto → Opções do projeto ("Texto por plataforma").

## Ícones

**Projeto → Ícones…** (Ctrl+Alt+I) abre os pacotes:

- **Pacote:** `material`, `material-rounded`, `material-sharp`, `fluent`, os embutidos do
  Aylis e `app` (os ícones que você importou).
- **Preenchido:** mostra as variantes `-fill`.
- **Busca:** filtra pelo nome em inglês (home, person, arrow…). A grade mostra até 240
  ícones de cada vez.
- **OK com um componente que tem `icon` selecionado** (Button, Edit, Icon…): o ícone vai
  para ele.
- **Copiar nome:** copia o nome para usar no código.
- **Importar SVG…:** traz um `.svg` para `assets/icons/app.txt` como `app:<nome>`. O
  desenho é ajustado para a grade 24×24 de acordo com o `viewBox`. Entram as formas
  preenchidas: `<path>`, `<rect>`, `<circle>`, `<ellipse>`, `<polygon>` e `<polyline>`, com
  `<g transform>` e `<use>`. Ícones só de traço (estilo Feather/Lucide) precisam ter os
  traços convertidos em formas no editor ("Stroke to path"): os ícones do Aylis são
  preenchidos.

O nome do ícone é `pacote:nome`:

```rust
let b = ui.add(Button::with_icon("material:home"), form);
ui.set_prop(b, "icon", "fluent:person-fill")?;
```

### Só os ícones usados

Ao **salvar** um form e ao **Gerar código**, a IDE procura nos forms e em `src/` os textos
`material:...`, `fluent:...` etc. Para cada pacote citado, ela grava em
`assets/icons/<pacote>.txt` um pacote só com esses ícones e a licença. Os pacotes que
deixam de ser citados saem da pasta. O `app.txt` (os seus ícones) fica sempre.

Com um ícone, o arquivo tem cerca de 12 KB no Material (quase tudo o texto da licença
Apache) e 1 KB no Fluent. O pacote inteiro tem de 2,3 a 5,0 MB.

Para um ícone montado em tempo de execução (`format!("material:{nome}")`), cite o nome
completo em algum lugar do código, por exemplo num comentário, para ele entrar no pacote.

### Formato do pacote

```text
# aylis-icons 1
# title: Material Symbols (Outlined)
# license: Apache-2.0 (Google)
# view: 0 -960 960 960
#| (texto da licença, linha a linha)
home<TAB>M220-180h150v-250h220v250h150v-390L480-765 220-570...
home-fill<TAB>M160-120v-480l320-240 320 240v480H560v-280H400...
```

- Cada linha de ícone tem o nome, uma tabulação e o `d` do SVG.
- O desenho é preenchido.
- `view` é o `viewBox` do pacote.

Na API:

- `Ui::register_icon_pack(prefixo, texto)` registra um pacote.
- `IconPack::subset(&["home"])` gera um pacote menor com os ícones escolhidos.
