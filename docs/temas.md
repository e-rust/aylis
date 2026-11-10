# Temas de plataforma e aparência do projeto

O Aylis traz, além do visual próprio (Aylis Light e Aylis Dark), temas que imitam as
plataformas. Mudam as cores, os raios e as fontes e também a **forma** dos controles:
o Switch do iOS é largo e verde, o CheckBox do iOS é redondo, o botão do Windows 7 tem
gradiente, o campo do Windows 11 tem o traço de baixo...

| Família | Temas | Look |
|---|---|---|
| Aylis | Aylis Light, Aylis Dark | `aylis` |
| Material 3 (Android 12+) | Material 3 Light, Material 3 Dark | `material3` |
| Material 2 (Android 9–11) | Material 2 Light, Material 2 Dark | `material2` |
| Android 7 (Nougat, AppCompat) | Android 7 Light, Android 7 Dark | `material2` |
| iOS 26 (Liquid Glass) | iOS 26 Light, iOS 26 Dark | `ios26` |
| iOS 13–18 | iOS 13 Light, iOS 13 Dark | `ios13` |
| iOS 7–12 | iOS 7 Light (o iOS não tinha modo escuro) | `ios7` |
| WinUI 3 (Windows App SDK) | WinUI 3 Light, WinUI 3 Dark | `win11` |
| Windows 11 (Fluent 2) | Windows 11 Light, Windows 11 Dark | `win11` |
| Windows 10 (Fluent/Metro) | Windows 10 Light, Windows 10 Dark | `win10` |
| Win32 (os controles clássicos do desktop) | Win32 | `win10` |
| Windows 7 (Aero) | Windows 7 | `win7` |
| Windows Phone 7/8 (Metro) | Windows Phone Dark, Windows Phone Light | `winphone` |
| Vuetify 3 (web) | Vuetify 3 Light, Vuetify 3 Dark | `material2` |
| Bootstrap 5 (web) | Bootstrap 5 Light, Bootstrap 5 Dark | `aylis` |
| shadcn/ui (web, Tailwind) | shadcn/ui Light, shadcn/ui Dark | `aylis` |
| Ant Design 5 (web) | Ant Design 5 Light, Ant Design 5 Dark | `aylis` |
| Chakra UI (web) | Chakra UI Light, Chakra UI Dark | `aylis` |

Três temas compartilham a forma com outro e mudam o resto:

- **Android 7:** os controles têm a forma do Material 2. As cores e os estilos são os do
  AppCompat da época:
  - `primary` é o colorAccent, rosa no claro (#FF4081) e verde-água no escuro, e pinta
    CheckBox, Switch, TrackBar e o foco;
  - `app_bar` é o índigo da barra (use `Panel.appbar`);
  - o botão padrão é cinza, em relevo e em maiúsculas;
  - o campo de texto tem só a linha de baixo.
- **WinUI 3:** a forma dos controles é a do Windows 11. As cores são os recursos de tema
  do próprio WinUI:
  - preenchimentos translúcidos (`ControlFillColorDefault` #FFFFFFB3...) sobre a Mica
    (`background`);
  - traços `ControlStrokeColor...`;
  - texto `TextFillColorPrimary`;
  - fonte Segoe UI Variable.

  O tema Windows 11 usa cores opacas, como os controles Win32/WinForms no Windows 11.
- **Win32:** os controles comuns de um programa desktop clássico, como aparecem no Windows
  10 e 11: fundo cinza `#F0F0F0`, botão cinza claro de borda fina que fica azul no hover,
  campo branco de borda 1 px, caixa marcada branca com o visto preto, barra de progresso
  verde, Segoe UI 9 pt, cantos retos. Usa dois tokens opcionais que outros temas também
  podem definir: `check_mark` (a cor do visto e do ponto do rádio) e `progress` (a cor da
  barra de progresso).
- **Windows Phone:** tem forma própria (`winphone`):
  - bordas de 3 px e nenhum canto arredondado;
  - campo cinza que fica branco com a borda azul no foco;
  - Switch retangular com o "dedo" mais alto que o trilho;
  - TrackBar grosso;
  - títulos finos e grandes.

  O escuro (fundo preto) é o padrão da plataforma, e a cor de destaque é o azul #1BA1E2.
  A Segoe WP não pode ir no app, então vale a Segoe UI.

**Os da web** (aba **Web** da paleta, ao lado de Windows, Apple e Android) usam as cores,
os raios e as fontes padrão de cada framework (os tokens da documentação dele):

- **Vuetify 3:** o Material da biblioteca, azul `#1867C0`, botão elevado de cantos 4, campo
  "filled" (fundo cinza); `Edit.outlined` é o de contorno;
- **Bootstrap 5:** azul `#0d6efd`, cantos de 6, anel de foco claro, `Button.outline` como o
  `.btn-outline-primary` e o Switch como o `.form-switch`;
- **shadcn/ui:** a paleta zinc do Tailwind, botão quase preto (claro) ou quase branco
  (escuro), bordas finas, `Button.ghost`;
- **Ant Design 5:** azul `#1677ff`, texto de 14, o botão "default" (`Button.secondary`) branco
  com borda que fica azul no hover;
- **Chakra UI:** `blue.500`, o botão cinza `gray.100` como secundário, cartão com sombra.

Os estilos comuns a todos: `Button` (o primário), `Button.secondary`, `Button.outline`,
`Button.text`, `Button.danger`, `Button.success`, `Panel.card` e `Panel.outlined`.

Para ver todos lado a lado:

```bash
cargo run -p aylis-platform-winit --example styles
```

## O que muda em cada look

| Controle | O que muda |
|---|---|
| Button | Material 2: maiúsculas e sombra; Material 3: pílula, "tonal" no secundário; iOS 7: só o texto; iOS 13: preenchido arredondado; iOS 26: cápsula de vidro; Windows 7: gradiente do Aero; Windows 10: cinza com cantos retos; Windows 11: claro com o traço de baixo |
| Edit (e ComboBox, Memo...) | Material 3: contorno; Material 2: preenchido com linha embaixo; iOS: arredondado sem borda; Windows 10: borda de 2 px; Windows 11: traço de baixo que fica na cor de destaque com o foco |
| Switch | tamanho, bolinha e cores de cada plataforma (verde no iOS, bolinha que cresce no Material 3, cápsula de vidro no iOS 26, bolinha pequena no Windows 11) |
| CheckBox / RadioButton | quadrado com cantos de 2 px (Material), redondo com ✓ (iOS), quadrado de 20 px (Windows 10/11), 13 px com gradiente (Windows 7); rádio do Material é anel e ponto na cor de destaque |
| TrackBar | bolinha cheia (Material), branca com sombra (iOS), retângulo (Windows 10), bolinha com miolo de destaque (Windows 11) |
| ProgressBar | brilho do Windows 7, trilhos finos do iOS |
| SegmentedControl | contorno com o escolhido preenchido (iOS 7), pílula com ✓ (Material), deslizante (iOS 13/26 e Windows) |

Os outros controles seguem o tema pelas cores, raios e fontes.

**Fontes.** Os temas citam a fonte de cada plataforma: Roboto no Material, a fonte do
sistema no iOS e Segoe UI no Windows. A SF Pro (Apple) e a Segoe UI (Microsoft) não
podem ser embutidas no app: no Windows a Segoe UI já está instalada; nos outros sistemas,
vale a fonte padrão.

## Um mesmo componente com qualquer estilo

O estilo não depende da plataforma em que o app roda. Um app no Windows pode ter um
botão do iOS:

```rust
ui.set_prop(botao, "theme", "iOS 26 Light")?;            // só esse botão
ui.set_prop(painel, "theme", "Windows 11 Dark")?;         // o painel e tudo dentro dele
ui.theme_mut().mix("Switch", "iOS 13 Light");             // todos os Switch do app
ui.set_theme(aylis_core::themes::builtin("Material 3 Light").unwrap().clone());  // tudo
```

## Na IDE: Projeto → Opções do projeto → Aparência (Ctrl+Shift+A)

- **Tema do projeto:** família (Aylis, Material 3, iOS 26, Windows 11...) ou um arquivo
  `themes/*.json`, e o modo (claro ou escuro).
- **Misturar por classe:** "estas classes usam outro tema" (por exemplo, todos os
  `Button` com o iOS 26).
- **Prévia:** os controles no tema escolhido.
- **Quem usa qual tema:** todos os componentes do form aberto, com o tema com que cada
  um é pintado e de onde ele vem:
  - **do projeto**: o tema do projeto;
  - **próprio**: a propriedade `theme` do componente;
  - **herdado de X**: a propriedade `theme` de um componente pai;
  - **mistura da classe Y**: a mistura por classe.

  O combo da linha troca o tema próprio do componente, com desfazer.

O designer desenha os forms com a aparência do projeto. O seletor "Form claro / Form
escuro" da barra troca entre o claro e o escuro da mesma família.

No `.aylisproj`:

```json
"theme": "Material 3 Light",
"theme_mix": { "Button": "iOS 26 Light", "Switch": "iOS 13 Light" }
```

## Tema próprio a partir de um de plataforma

Num tema em JSON, `look` escolhe a forma dos controles e `family`/`dark` agrupam o claro
e o escuro:

```json
{
  "name": "Empresa",
  "family": "Empresa",
  "look": "win11",
  "colors": { "primary": "#C00000" },
  "components": { "Button": "Material 3 Light" }
}
```

Para mudar só algumas cores (ou fontes, ou o estilo de algumas classes) de um tema pronto,
`base` parte dele: o resto (estilos, raios, sombras) vem da base.

```json
{
  "name": "Meu App Dark",
  "base": "Aylis Dark",
  "family": "Meu App",
  "dark": true,
  "colors": { "background": "#0A0A0C", "primary": "#3B82F6" },
  "fonts": { "default": { "family": "Segoe UI", "size": 13 } },
  "styles": { "Panel.column": { "fill": "surface", "radius": 0 } }
}
```

## Cores do projeto e cores com nome

As cores que o app usa ficam num lugar só, por nome, e qualquer propriedade de cor
(`fill`, `text_color`, `accent_color`, `stroke`, `outline`...) e qualquer estilo de tema
pode citá-las em vez do hex. Uma cor pode ser escrita como:

| Escrita | O que é |
|---|---|
| `#10B981`, `#FFFFFFD9` | a cor em si |
| `success`, `text_3`, `brand` | uma cor do tema em uso ou do projeto |
| `text/85`, `primary/20`, `white/12` | a mesma cor com 85 %, 20 %, 12 % de opacidade |
| `white`, `black`, `transparent` | prontas |

As do projeto ficam no `.aylisproj`, em `colors`: uma cor só, uma por modo (escuro/claro)
ou outra cor com opacidade. Uma pode citar a outra (em qualquer ordem) e as do tema:

```json
{
  "aylis_project": 1,
  "name": "Meu App",
  "theme": "dark",
  "colors": {
    "brand": "#10B981",
    "brand_soft": "brand/15",
    "painel": { "dark": "#0F0F0FF5", "light": "#FFFFFFF7" },
    "linha": { "dark": "white/6", "light": "black/8" }
  }
}
```

Elas entram em todo tema que o app usa (por cima das cores do tema, inclusive ao trocar
de tema em tempo de execução), então valem nos estilos (`"Panel.card": { "fill": "painel" }`)
e nos forms (`{ "class": "Panel", "fill": "painel" }`). No `colors` de um tema em JSON também
dá para citar outra cor (`"accent_soft": "accent/15"`).

O texto de exemplo dos campos (`placeholder` do `Edit` e do `Memo`) usa a cor `placeholder` do
tema, quando ele tem uma; sem ela, a `text_muted`. Serve para um tema que imita outro visual
com o exemplo mais apagado que o texto secundário (o NoOBS: `"placeholder": "#71717A"`).

Numa propriedade escrita com nome, o `Ui` guarda o nome: trocar o tema recalcula a cor, o
form salvo continua com o nome e `get_prop` devolve o nome (`ui.color_name(id, prop)` diz
se tem um). Pelo código: `ui.set_prop(id, "fill", "brand/15")`, `ui.set_palette(...)` e
`theme.resolve_color("text/85")`. Na IDE, o campo de cor do inspetor aceita os nomes e a
amostra mostra a cor resolvida.
