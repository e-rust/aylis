# Material 3 e iOS 26 (Liquid Glass)

Os catálogos de componentes do Material 3 (m3.material.io) e da Apple (Human Interface
Guidelines) mapeados para o Aylis.

A ideia é a mesma dos temas de plataforma: **um componente, o visual do tema**. O mesmo
`AppBar` vira a "top app bar" do Material 3 ou a "navigation bar" do iOS, e a mesma
`BottomNavigation` vira a barra do Material ou a tab bar de vidro do iOS 26. Para trocar,
basta mudar o `theme`: do projeto, de uma classe ou de um controle.

O exemplo `cargo run -p aylis-platform-winit --example catalog` mostra dois celulares lado a
lado com os mesmos componentes:
- `-- --dark`: no tema escuro;
- `-- --list`: a segunda tela (roda de opções, chips, cartões, campo preenchido).

## Liquid Glass

A HIG define o Liquid Glass assim: uma camada de controles e navegação que flutua sobre o
conteúdo, e o conteúdo aparece desfocado por baixo.

No Aylis, o vidro é uma propriedade do estilo do tema, `material`:

| `material` | O que é |
|---|---|
| `glass` | Liquid Glass: desfoca o que está atrás e aplica o brilho por cima e a borda de luz |
| `frosted` | Vidro fosco (Acrylic, os materiais "regular" da Apple): desfoque forte, sem brilho |
| `none` | Sem vidro |

Antes de pintar um controle de vidro, o `Ui` chama `Canvas::backdrop_blur`, que desfoca o que
já foi desenhado embaixo. Depois o controle pinta o fundo translúcido (token `glass_fill`) e,
no `glass`, entram o brilho e a borda.

| Renderizador | Como desfoca |
|---|---|
| Skia | `save_layer` com filtro de fundo (blur) |
| Direct2D | copia o pedaço do alvo e aplica o efeito Gaussian Blur |
| Navegador | desenha o próprio canvas sobre si com `filter: blur()`; sem `filter` no Canvas 2D (Safari), reduz o pedaço e amplia de volta com suavização |

No Direct2D, os recortes e camadas abertos são fechados para a cópia e reabertos em seguida
(o Direct2D não copia o alvo com eles abertos). Se o renderizador não souber desfocar, o
fundo fica mais opaco.

Conferido no navegador com o antigo demo `examples/web` (fora do repositório desde 27/10/2026) em `?vidro` (o mesmo catálogo do exemplo
`catalog` do desktop; `&dark` para o escuro): o resultado bate com o Skia. `&aylis_blur=reduzir`
força o caminho do Safari em qualquer navegador, para comparar.

O tema **iOS 26** liga o vidro em botões, cartões, `SegmentedControl`, campo de busca e
`ToolBar.floating`. A tab bar (`BottomNavigation`), a navigation bar (`AppBar`), a sheet
(`BottomSheet`), o `PageControl`, o `UpDown` e o `action_sheet` desenham o vidro por conta
própria. Em qualquer tema dá para usar `Button.glass`, `Button.glass_prominent` e
`Panel.glass`. Num tema JSON próprio, basta `"material": "glass"` no estilo:

```json
"Panel.card": { "fill": "glass_fill", "radius": 26, "material": "glass" }
```

## Material 3

| Material 3 | Aylis |
|---|---|
| App bars (small, center-aligned, medium, large) | `AppBar` com `size` (`AppBar.center`, `.medium`, `.large`) |
| Badges | `Badge` (preso: `target`) |
| Buttons (filled, tonal, elevated, outlined, text) | `Button`, `Button.tonal`, `.elevated`, `.outline`, `.text` |
| Button groups (standard, connected) | `ButtonGroup` (`connected`; `ButtonGroup.standard`) |
| Extended FAB, FAB | `Button.fab_extended`, `Button.fab` (cantos 16) |
| FAB menu | `SpeedDial`: no Material 3, as ações viram pílulas com ícone e nome |
| Icon buttons (standard, filled, tonal, outlined) | `Button.icon`, `.icon_filled`, `.icon_tonal`, `.icon_outlined` |
| Segmented buttons | `SegmentedControl` |
| Split button | `SplitButton`: no Material 3, duas peças com um vão |
| Cards (elevated, filled, outlined) | `Panel.card`, `Panel.filled`, `Panel.outlined` |
| Carousel | `Carousel` |
| Checkbox, Radio button, Switch | `CheckBox`, `RadioButton`, `Switch` |
| Chips (assist, filter, input, suggestion) | `Chip` (`filter`, `input`), `ChipGroup`; cantos 8 no Material 3 |
| Date pickers, Time pickers | `Calendar` / `DateEdit`, `TimePicker` / `TimeEdit` |
| Dialogs | `message_dlg`, `Form` modal |
| Divider, Lists, Menus | `Divider`, `ListBox`/`ListView`, `PopupMenu` |
| Loading indicator | `LoadingIndicator` (formas que giram e se transformam; `LoadingIndicator.contained`) |
| Progress indicators | `ProgressBar`, `ProgressCircle` |
| Navigation bar | `BottomNavigation` |
| Navigation drawer | `NavigationDrawer` |
| Navigation rail | `NavigationRail` (com FAB em cima) |
| Search | `Edit.search` |
| Bottom sheets, Side sheets | `BottomSheet`, `SideSheet` |
| Sliders | `TrackBar` e `RangeSlider` no estilo Expressive (trilho grosso, controle em barra) |
| Snackbar | `Snackbar` (com fila) |
| Tabs | `TabControl` |
| Text fields (filled, outlined) | `Edit.filled` (rótulo dentro e linha embaixo), `Edit.outlined` (padrão) |
| Toolbars | `ToolBar`, `ToolBar.floating` |
| Tooltips | `hint` |

## iOS (Human Interface Guidelines)

| Apple | Aylis |
|---|---|
| Navigation bar (título grande, botão voltar) | `AppBar` (`size: large` é o "large title"; `back_text`); no iOS 26, botões redondos de vidro |
| Tab bar | `BottomNavigation`: cápsula de vidro flutuante no iOS 26, barra translúcida antes |
| Toolbar | `ToolBar.floating` (vidro) |
| Search field | `Edit.search` (cápsula de vidro) |
| Sidebar | `NavigationDrawer` |
| Sheets | `BottomSheet` com `detents` (`0.4;0.9`): arrastar a alça encaixa na altura mais perto, e puxar para baixo fecha |
| Action sheets | `action_sheet(ui, título, ações, destrutiva, "Cancelar", ...)` |
| Alerts | `message_dlg` |
| Page controls | `PageControl` (cápsula de vidro no iOS 26) |
| Pickers | `WheelPicker` (a roda), `Calendar`, `TimePicker` |
| Segmented controls, Sliders, Toggles | `SegmentedControl`, `TrackBar`, `Switch` (com a forma do iOS) |
| Steppers | `UpDown` (− e +, segurar repete) |
| Text fields, Combo boxes | `Edit`, `AutoComplete` / `ComboBox` |
| Color wells, Digit entry | `ColorEdit`, `OtpInput` |
| Context menus, Menus, Pull-down buttons | `PopupMenu`, `SplitButton` |
| Activity rings | `ActivityRings` (Mover, Exercício, Em pé; passando de 1 dá outra volta) |
| Gauges | `Gauge` (arco de 270°, com ou sem gradiente) |
| Progress indicators, Rating indicators | `ProgressBar` / `ActivityIndicator`, `Rating` |
| Charts, Image views, Web views, Text views | `Chart`, `Image`, `WebView`, `Markdown` / `Memo` |
| Lists and tables, Collections, Disclosure | `ListView`, `GridLayout`, `Expander` |
| Labels, Split views, Tab views | `Label`, `Splitter`, `TabControl` |

Ficam de fora o que é do sistema e não de um app:
- Dock menus, Menu bar, Home Screen quick actions;
- Widgets, Complications, Live Activities, Watch faces;
- Virtual keyboards, Activity views (a folha de compartilhar do sistema).
