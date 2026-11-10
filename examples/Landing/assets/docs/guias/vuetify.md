# Componentes do Vuetify no Aylis

Este documento mostra, para cada componente do Vuetify 3 (lista de `packages/vuetify/src/components`
e `labs` no GitHub), qual é o equivalente no Aylis. Quem já usou Vuetify acha aqui o nome do
componente.

Onde aparece "—", o componente não se aplica a um app nativo (é coisa de página web ou de
Vue).

O exemplo `cargo run -p aylis-platform-winit --example gallery` mostra os componentes numa grade.
Com `-- --theme "Material 3 Dark"` (ou qualquer tema de plataforma), mostra no visual de outra
plataforma.

## Estrutura e layout

| Vuetify | Aylis |
|---|---|
| VApp, VMain, VLayout | `Form` + `align` (Top, Client...) |
| VGrid (`v-row`/`v-col`) | `GridLayout` (colunas fixas ou pela largura) |
| — (flex com quebra) | `FlowLayout` |
| VSheet, VCard | `Panel`, `Panel.card` |
| VDivider | `Divider` (horizontal, vertical, com texto) |
| VAppBar, VToolbar | `ToolBar` |
| VSystemBar, VFooter | `StatusBar` |
| VNavigationDrawer | `NavigationDrawer` (aberta, fechada, só ícones) |
| VBottomNavigation | `BottomNavigation` |
| VBottomSheet | `BottomSheet` |
| VDialog, VOverlay | `Form` modal (`show_modal`), `Panel.overlay` |
| VMenu | `PopupMenu` |
| VTabs, VWindow | `TabControl` / `TabItem` |
| VCarousel | `Carousel` (setas, bolinhas, arrastar, troca automática) |
| VExpansionPanel | `Expander` |
| VResponsive | `aspect_ratio` (em qualquer controle) |
| VVirtualScroll, VInfiniteScroll | `ListView` (virtualizada), `ScrollBox.on_end_reached` |
| VPullToRefresh | `ScrollBox.pull_to_refresh` (`on_refresh`, `refreshing`) |
| VSlideGroup | `ChipGroup.slide` (uma linha, setas), `ScrollBox.horizontal` |
| VLazy, VNoSsr, VParallax | — |
| VDefaultsProvider, VThemeProvider, VLocaleProvider | `theme` por controle, `Theme::mix`, locale do `Ui` |
| VHover | `pointer_enter`/`pointer_leave`, `state.hover` |

## Botões e ações

| Vuetify | Aylis |
|---|---|
| VBtn | `Button` (+ subtipos `secondary`, `outline`, `flat`, `link`, `danger`...) |
| VIconBtn | `Button.icon` |
| VFab | `Button.fab` |
| VSpeedDial | `SpeedDial` |
| VBtnToggle, VBtnGroup | `SegmentedControl`, `Button.pill` |
| VSplitBtn (labs) | `SplitButton` (ação + seta com `popup_menu`) |
| VHotkey, VKbd | `HotkeyEdit`, `Label.kbd` |

## Entradas

| Vuetify | Aylis |
|---|---|
| VTextField, VField, VInput, VLabel, VMessages, VCounter | `Edit` com `label`, `helper`, `error`, `counter`, `prefix`, `suffix` |
| VMaskInput (labs) | `Edit.mask` (`(##) #####-####`) |
| VForm, VValidation | `Edit.rules` + `validate_form(ui, form)` |
| VTextarea | `Memo` |
| VNumberInput | `NumberBox` |
| VSelect | `ComboBox` |
| VAutocomplete, VCombobox | `AutoComplete` (filtra sem acento; `strict`) |
| VCheckbox, VRadio, VRadioGroup, VSwitch | `CheckBox`, `RadioButton`, `RadioGroup`, `Switch` |
| VSelectionControl, VSelectionControlGroup, VItemGroup | `RadioGroup`, `ChipGroup` |
| VSlider | `TrackBar` |
| VRangeSlider | `RangeSlider` |
| VRating | `Rating` (meia estrela) |
| VOtpInput | `OtpInput` |
| VChip, VChipGroup | `Chip` (`filter`, `input`, `outline`, tons), `ChipGroup` |
| VFileInput | `FileInput` |
| VFileUpload | `FileDropZone` (soltar arquivos do sistema ou clicar) |
| VColorPicker | `ColorPicker` |
| VColorInput | `ColorEdit` |
| VDatePicker, VDateInput | `Calendar`, `DateEdit` |
| VDateRangePicker (labs) | `Calendar.range` |
| VMonthPicker (labs) | `Calendar.month` |
| VTimePicker, VTimeInput (labs) | `TimePicker` (relógio: anéis 1–12 e 13–00, depois minutos), `TimeEdit` (abre o relógio) |
| VConfirmEdit | — (`Form` modal com OK/Cancelar) |
| VPicker | — (é a base dos pickers) |

## Dados e listas

| Vuetify | Aylis |
|---|---|
| VDataTable, VTable | `DataGrid` (com `ClientDataSet`), `ListView` |
| VDataIterator | `ListView`, `ChatView`, `GridLayout` com cartões |
| VList | `ListBox`, `ListView` |
| VTreeview | `TreeView` |
| VTimeline | `Timeline` |
| VCalendar | `EventCalendar`: `view` mês, semana ou dia; eventos do dia todo ou com horário (lado a lado quando se sobrepõem); `on_day`, `on_event`, `on_slot` |
| VSparkline | `Chart.sparkline` |
| VPie (labs) | `Chart` `pie`/`donut` |
| VHeatmap (labs) | `Chart.heatmap` |
| VCommandPalette (labs) | `CommandPalette` |
| VHighlight | `Label.highlight` (sem diferenciar maiúsculas e acentos, com quebra de linha) |

## Retorno e progresso

| Vuetify | Aylis |
|---|---|
| VAlert, VBanner | `Banner` |
| VSnackbar, VSnackbarQueue | `Snackbar` (`enqueue`: fila, cada mensagem com a sua ação), `snackbar(ui, ...)`, `toast(ui, ...)` |
| VProgressLinear | `ProgressBar` |
| VProgressCircular | `ProgressCircle` (valor ou girando) |
| VLoadingIndicator (labs) | `ActivityIndicator` |
| VSkeletonLoader | `SkeletonLoader` |
| VEmptyState | `EmptyState` |
| VTooltip | `hint` em qualquer controle |
| VBadge | `Badge` com `target` (preso ao canto de um irmão, acompanha quando ele muda de lugar) e `anchor` |
| VPagination | `Pagination` |
| VStepper, VStepperVertical | `Stepper`, `Stepper.vertical` |
| VBreadcrumbs | `Breadcrumb` |

## Imagens, ícones e mídia

| Vuetify | Aylis |
|---|---|
| VImg | `Image` |
| VIcon | `Icon` + pacotes Material e Fluent |
| VAvatar | `Avatar` |
| VAvatarGroup | `AvatarGroup` |
| VCode | `Label.mono`, `Markdown` |
| VAudio, VVideo (labs) | `MediaPlayer`, `VideoPlayer` |
| VMediaProgressBar, VMediaVolume (labs) | `SeekBar`, `TrackBar` |
| VInfiniteCarousel (labs) | `Carousel` com `cycle` |

Todos os componentes do Vuetify têm equivalente; o que sobra (—) é coisa de página web ou
detalhe sem efeito num app nativo (a proporção fixa do `VResponsive`).

## Por dentro

- `Component::arrange` é o gancho de layout novo. Um contêiner pode organizar os filhos
  soltos (`align = None`); é o que usam o `GridLayout` e o `FlowLayout`.
- `Component::attach` prende um controle ao canto de um irmão: o layout o reposiciona
  sempre que o alvo muda de lugar. É o que usa o `Badge` com `target`.
- `Component::files_dropped` e `Ui::drop_files` tratam os arquivos arrastados do sistema. O
  winit entrega cada arquivo; a plataforma passa para o controle sob o ponteiro ou, se a
  posição não ajudar, para o primeiro que aceita.
- Todos os componentes novos pegam as cores do tema, inclusive dos temas de plataforma
  (Material, iOS, Windows), e aceitam subtipos na paleta.
