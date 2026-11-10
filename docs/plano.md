# aylis — Plano

> A facilidade de montar telas num designer visual, desenho próprio igual em todas as plataformas e compatibilidade com um backend em qualquer linguagem.

Este é o plano inicial: princípios, arquitetura e fases. O que apareceu depois fica em dois
documentos: os [ajustes pendentes](ajustes-pendentes.md) e o
[histórico de decisões](historico-decisoes.md).

## 1. Princípios

1. **O core não conhece o sistema operacional.** Tudo que depende de plataforma passa por traits. O `aylis-core` depende só da `std` (e de `serde`, opcional).
2. **Fácil de estender.** Comportamento separado da aparência. Personalizar um controle nunca exige reescrevê-lo.
3. **Tudo é criado em runtime.** O arquivo de layout é só um atalho que chama a mesma API.
4. **O frontend não tem lógica de backend.** Não há SQL nem conexão com banco. O frontend envia o delta e o backend decide o que fazer.
5. **Toda operação é expressável como mensagem:** nome de classe, nome de propriedade, `PropValue` e eventos por nome. Isso permite C ABI e protocolo remoto.
6. **Simples.** Não há linguagem intermediária nem expressões. O que tiver código vai para os eventos.
7. **Leve.** Só repinta sob demanda, e o Animator só pede frames enquanto há animação ativa.

## 2. Alvos e renderização

- Plataformas: Windows primeiro, depois Linux, web, Android e iOS.
- Renderização: **Skia (`skia-safe`) na CPU e na GPU**. Na CPU, uma superfície raster; na GPU, GL/Vulkan/Metal/D3D/WebGL. O código de pintura é o mesmo nos dois. O texto também vem do Skia (textlayout).
- Unidades lógicas no core; a escala de DPI é aplicada pelo canvas.
- Riscos: a **web** (o `skia-safe` usa emscripten, o `winit` usa wasm32-unknown-unknown; fazer um teste rápido logo cedo) e o **iOS** (precisa de Mac para compilar e assinar).

## 3. Serviços de plataforma (traits no core)

| Trait | Para que serve |
|---|---|
| `Canvas` | Desenhar: formas, texto, imagens |
| `TextService` | Medir e quebrar texto |
| `WindowService` | Criar e controlar as janelas |
| `TimerService` / frames | Tiques e quadros das animações |
| `ClipboardService` | Área de transferência |
| `CursorService` | Cursor do mouse |
| `ScreenService` | DPI e tema claro/escuro do sistema |
| `TextInputService` | Teclado virtual e IME |
| `LocaleService` | Formatos de data e número do idioma |
| `AppLifecycle` | App em segundo plano e de volta (suspend/resume) |

O input usa tipos próprios: `PointerEvent` (mouse e toque), `KeyEvent` e `WheelEvent`.

## 4. Modelo de componentes

- **`Component`** é a base de tudo (como o `TComponent`): nome, propriedades e notificação de destruição. Os componentes não visuais (`ClientDataSet`, `DataSource`, `Field`, `Timer`, animações, efeitos) derivam dele.
- **`Control`** é a base visual (como o `TControl`). Dados comuns em `ControlBase`:
  - posição e tamanho: `position` (left/top), `width`, `height`;
  - layout: `margins`, `padding`, `align`, `anchors`;
  - estado e pintura: `visible`, `enabled`, `hit_test`, `opacity`, `clip_children`;
  - identificação: `tag`, `name`.
- Os métodos "virtuais" são do trait, com implementação padrão: `paint`, `mouse_*`, `key_*`, `click`, `resize`, `enter_focus`/`exit_focus`.
- Herança por composição: `struct GlowButton { base: Button, .. }` + `delegate_to!(base)`. Chamar `self.base.paint(..)` equivale ao `inherited`.
- Arena `Ui` com `ControlId` geracional (um ID de componente destruído devolve `None`, não um Access Violation).
- API: `ui.add`, `ui.create("Button", parent)`, `ui.free`, `ui.set_parent`, `ui.find`, `ui.set_prop`/`get_prop`, `ui.children`.
- **Sistema de propriedades:** registro de classes (o `RegisterClass`) e propriedades publicadas com metadados: tipo, default, categoria, editor, valores de enum, min/max, readonly, oculta. Há o tipo `ComponentRef`, resolvido por fixup ao carregar.
- **Modo design** no `Ui` (desvia o input para o designer) e `design_paint` opcional.

## 5. Layout

- **Align:** `None`, `Top`, `Bottom`, `Left`, `Right`, `MostTop`, `MostBottom`, `MostLeft`, `MostRight`, `Client`, `Contents`, `Center`, `VertCenter`, `HorzCenter`, `Horizontal`, `Vertical`, `Fit`.
- **Anchors:** `Left`, `Top`, `Right`, `Bottom`.
- **Ordem do realign:**
  1. Área livre = pai menos o padding.
  2. Os `Most*`, depois `Top`/`Bottom`/`Left`/`Right`. Cada um consome o próprio tamanho mais as margens.
  3. `Client` e `Center*` usam o que sobrou.
  4. `Contents` usa o pai inteiro.
  5. Por último, os controles com Anchors.
- O realign é preguiçoso: só roda quando há um flag dirty.

## 6. Eventos

- Hit test do topo para baixo, captura do mouse, enter/leave, click, clique duplo, wheel, teclado, foco e tab order.
- Os handlers são closures `FnMut(&mut Ui, ComponentId)` ou referências por nome (resolvidas para Rust, C ABI ou remoto).

## 7. Temas, estilos, animações e efeitos

- **Tema em JSON:**
  - tokens: cores, fontes, raio, espaçamento, sombras, durações;
  - estilo por classe e por estado: `hover`, `pressed`, `focused`, `disabled`, `checked`;
  - variantes (`Button.danger`) e transições.
- Claro e escuro, podendo seguir o SO. A troca acontece em runtime.
- **Prioridade:** valor da instância > variante > estilo da classe > tokens.
- **`PaintCtx`** traz `rect`, `state`, `style` (estilo já resolvido), `theme` e `time`.
- **Quatro níveis de personalização:**
  1. tema;
  2. estilo/variante;
  3. `on_paint(c, ctx, default)`;
  4. novo componente por composição.
- **Animações e efeitos são componentes filhos**: `FloatAnimation`, `ColorAnimation`, `GlowEffect`, `ShadowEffect`, `BlurEffect`. Usam triggers fixos (`hover`, `pressed`, `focused`, `checked`, `visible`) e funções de easing.

## 8. Camada de dados

- **`ClientDataSet`**, só em memória:
  - cursor e estados `Inactive`/`Browse`/`Edit`/`Insert`;
  - navegação: `first`/`next`/`prior`/`last`, `locate`/`lookup`;
  - edição: `edit`/`insert`/`append`/`post`/`cancel`/`delete`;
  - filtro e índice, bookmark, `disable_controls`/`enable_controls`, master-detail local;
  - **change log (delta)**.
- **`Field`:**
  - tipos: `string`, `memo`, `integer`, `int64`, `decimal`/`currency`, `date`, `time`, `datetime`, `boolean`, `blob`/`image`, `guid`, `auto_inc`;
  - formatação e entrada: `display_label`, `display_width`, `display_format`, `edit_format`, `edit_mask` (com máscaras nomeadas), `alignment`;
  - restrições: `required`, `read_only`, `visible`, `size`, `precision`/`scale`, `min`/`max`, `default_value`, `display_values`;
  - tipo de campo: dados, calculado ou lookup.
- **Eventos de Field no cliente:** `on_get_text`, `on_set_text`, `on_validate`, `on_change`.
- **Eventos de DataSet no cliente:** `on_new_record`, `before_post`/`after_post`, `before_delete`, `after_scroll`, `on_calc_fields`, `on_reconcile_error`.
- **`DataSource`:** `data_set`, `auto_edit`, `enabled`, `on_data_change`, `on_state_change`.
- **`DataLink` / `FieldLink` / `GridLink`:** notificações e buffer de registros.
- **Controles DB\*:** `DBEdit`, `DBText`, `DBMemo`, `DBCheckBox`, `DBComboBox`, `DBLookupComboBox`, `DBImage`, `DBGrid`, `DBNavigator`. São feitos por composição: o controle comum + um link.
- **`Value`:** `Null`, `Bool`, `Int`, `Float`, `Decimal` (ponto fixo i128), `String`, `Date`, `Time`, `DateTime`, `Blob`, `Guid`.
- **Fluxo com o backend:**
  - backend → frontend: `load` (campos e linhas; Arrow ou JSON, com texto de exibição pré-calculado opcional), `append_rows`, `refresh_record`, `clear`;
  - frontend → backend: `apply_updates` com o delta (`ref`, `op`, `key`, `old`, `new`), e só os eventos que forem assinados;
  - resultado: ok ou erro por registro, com os valores gerados (ids). O `apply_mode` é `on_post` ou `cached`.
- **Grids grandes:** virtualização, cache de páginas, prefetch, placeholder.
- **Handlers remotos** (por exemplo `on_get_text`): cache por valor, chamadas em lote e assíncronas, nunca uma ida por célula.

## 9. Formato de arquivo (JSON)

```json
{
  "aylis": 1,
  "root": {
    "class": "Form",
    "name": "FrmClientes",
    "caption": "Clientes",
    "children": [
      { "class": "Button", "name": "btnSalvar", "align": "left", "text": "Salvar", "on_click": "btn_salvar_click" }
    ]
  }
}
```

- Chaves reservadas: `class`, `name` (opcional), `children`. Todo o resto é propriedade, inclusive os eventos `on_*`.
- Só se grava o que difere do default.
- Atalhos para `margins`/`padding`: `8`, `[h, v]`, `[l, t, r, b]` ou `{ "left": 4 }`.
- Cores em `#RRGGBB[AA]` ou como nome de token do tema.
- Referências a outros componentes pelo nome.
- O mesmo formato vale para layout, tema e protocolo. Um JSON Schema é gerado a partir do registro de classes.

## 10. Modos de integração (a mesma API nos três)

1. **Rust nativo:** chamadas diretas.
2. **C ABI** (`aylis.dll`/`.so`): no mesmo processo, para Pascal, C#, Go ou Python via FFI. Os eventos de cliente são síncronos e rápidos, e funciona no mobile.
3. **Protocolo** (JSON-RPC ou MessagePack, Arrow para dados) sobre stdio, socket ou WebSocket: backend em outro processo ou máquina.

## 11. Workspace

```
crates/
  aylis-core/            Component, Control, Ui, layout, eventos, propriedades, tema, animator, traits de serviço
  aylis-controls/        Layout, Rectangle, Label, Button, CheckBox, Edit, ...
  aylis-data/            Value, Field, ClientDataSet, DataSource, DataLink
  aylis-dbcontrols/      DBEdit, DBText, DBCheckBox, DBGrid, DBNavigator, ...
  aylis-skia/            Canvas + TextService via skia-safe (CPU e GPU)
  aylis-platform-winit/  desktop, Android, iOS e navegador (sobre o winit)
  aylis-capi/            C ABI (aylis.dll + include/aylis.h)
  aylis-remote/          protocolo JSON
  aylis-host/            backend em outro processo (stdio)
  aylis-json/            layout e tema em JSON + JSON Schema
  aylis-web/             Canvas 2D do navegador
  aylis-ide/             a IDE (feita com o próprio aylis)
examples/
  web/                  o cadastro no navegador
  mobile/               tela de celular: APK (Android), .app (iOS) e prévia no desktop
  forms/, python/       layouts JSON e backends de exemplo
```

## 11.1 Resultado do teste rápido de web e Linux (fase 6)

- **Linux:** testado num container Debian 12 (Docker): todos os testes passam, o Skia linka e renderiza, a janela abre via Xvfb nos modos GPU e CPU, e a GPU (Mesa llvmpipe) dá 98,54% dos pixels idênticos à CPU. Falta testar num desktop Linux real (Wayland, GPU de verdade). Atenção: sem a Segoe UI, o texto cai na fonte do sistema (DejaVu Sans, mais larga) e larguras fixas podem cortar texto — embutir uma fonte padrão ou dimensionar pelo texto resolve.
- **Web:** o `skia-safe` **não** compila para `wasm32-unknown-unknown` (não há binários prontos; ele tenta compilar o Skia inteiro). Caminho adotado: crate `aylis-web`, que implementa o `Canvas` sobre o **Canvas 2D** do navegador (no Chrome, Edge e Firefox o Canvas 2D é o próprio Skia). O `aylis-platform-winit` roda no navegador reaproveitando toda a tradução de entrada; o demo `examples/web` (907 KB de wasm sem otimização de tamanho) rodou com hover, clique, foco, Tab e digitação.
- **Alternativa para fidelidade total ao Skia na web:** CanvasKit (Skia oficial em wasm + WebGL), ao custo de ~7 MB de download e de embarcar as fontes. Fica como backend opcional futuro: o trait `Canvas` já permite trocar sem tocar nos controles.

## 11.2 Layout e tema em JSON (fase 7)

- Crate `aylis-json`: `load`/`save` de layouts, `load_theme`/`save_theme` e `schema` (JSON Schema draft 2020-12 gerado do registro de classes; `schema/aylis.schema.json` é gerado por `cargo run -p aylis-json --example export -- schema`).
- Exemplo de layout escrito à mão: `examples/forms/cadastro.json`, aberto por `cargo run -p aylis-platform-winit --example json_form`.
- Limitação conhecida: uma cor de propriedade escrita como token (`"color": "text_muted"`) é resolvida para a cor do tema **no momento do carregamento**; trocar o tema depois não muda essas propriedades (o que vem do estilo do tema muda normalmente). Guardar a referência ao token na propriedade fica para depois.

## 11.3 Camada de dados (fase 8)

- `aylis-data`: `ClientDataSet` (memória, cursor com Bof/Eof, estados, change log), `Field` (tipos, `display_format`/`edit_format` com o idioma, required/size/min/max, `on_get_text`/`on_set_text`/`on_validate`), `DataSource`, `FieldLink`, delta + `apply_updates`/`reconcile` (Rust via `on_apply` ou JSON via `on_apply_updates`/`reconcile_json`), `load_rows`/`load_json`, `disable_controls`.
- `aylis-dbcontrols`: `DBText`, `DBEdit`, `DBCheckBox`, `DBNavigator`.
- Demo: `cargo run -p aylis-platform-winit --example data`.
- Ainda não: máscaras de edição (`edit_mask`), campos calculados e lookup, filtro/índice, master-detail, `DBComboBox`/`DBLookupComboBox`, e eventos de Field por **nome** (hoje `on_get_text`/`on_set_text`/`on_validate` são closures Rust; a versão por nome, em lote e assíncrona, vem com o protocolo remoto).

## 11.4 DBGrid (fase 9)

- `DBGrid` virtualizado: só as linhas visíveis são lidas e pintadas (~3 ms por frame na CPU, igual com 2 mil ou 1 milhão de registros, medido no `--release`).
- Busca sob demanda: `ClientDataSet.on_fetch`/evento `on_fetch` + `append_rows`/`append_json` (pacotes, como o `fmOnDemand` do FireDAC).
- Colunas pelos campos (`columns: "a;b"` escolhe e ordena), redimensionar no cabeçalho, edição na célula reaproveitando o `DBEdit` (Enter/F2/duplo clique/digitar; Enter grava, Esc desfaz, Tab próxima coluna).
- Demo: `cargo run --release -p aylis-platform-winit --example grid` (1 milhão sob demanda) ou `-- --all 1000000`.
- Atenção: 1 milhão de registros carregados de uma vez ocupa ~600 MB (cada valor é um `Value` de 48 bytes mais o texto). Para volumes assim, prefira a busca sob demanda; um armazenamento em colunas pode reduzir isso depois.
- Ainda não: ordenar pelo cabeçalho, filtros, colunas fixas, checkbox em campos lógicos dentro do grid, seleção de várias linhas.

## 11.5 C ABI e protocolo remoto (fase 10)

- `aylis-remote`: o protocolo (comandos `load`, `create`, `set`, `get`, `free`, `click`, `focus`, `find`, `save`, `theme`, `locale`, `data`, `batch`, `close`; respostas com `id`; eventos em JSON). Documentado no próprio crate.
- `aylis-host`: backend em outro processo, qualquer linguagem, uma mensagem JSON por linha em stdin/stdout. `--png` para testes sem janela.
- `aylis-capi` (`aylis.dll` + `include/aylis.h`): mesmo protocolo no mesmo processo, com callbacks síncronos (`aylis_ctx_execute`) e `aylis_post` para outras threads.
- Exemplos em Python: `examples/python/cadastro_backend.py` (via aylis-host, dados num SQLite do backend) e `examples/python/capi_demo.py` (via ctypes).
- Ainda não: eventos de Field por nome em lote e assíncronos (`on_get_text` remoto com cache), transporte WebSocket (frontend web ↔ servidor), `aylis_post` sem teste automático, e `aylis_run` consome o aplicativo (como `Application.Run`).

## 11.6 IDE (fase 11)

- `aylis-ide`, feita com os próprios controles do aylis: barra de ferramentas, **paleta** (classes do registro por categoria; um componente registrado por terceiros aparece sozinho), **form designer**, **estrutura** e **Object Inspector** com abas Propriedades/Eventos.
- Designer: colocar da paleta (encaixa na grade de 8 px e entra no container sob o mouse), selecionar, arrastar, 8 alças de redimensionar (respeitando o `align`), setas movem (Shift = 8 px), Ctrl+↑/↓ muda a ordem, Delete, Ctrl+Z/Y, Ctrl+C/V (renomeia: `Button2`...), Esc sobe para o pai, roda do mouse desloca. Não visuais soltos no form (ClientDataSet, DataSource) ficam numa **bandeja** abaixo do form.
- Inspector: editor por tipo (CheckBox para lógico, ComboBox para enum, caixa de texto para o resto com vírgula decimal e `margins` como `8`, `4, 8` ou `1, 2, 3, 4`), amostra de cor, erros na barra de status. Eventos: botão "…" (ou duplo clique no componente) preenche o nome padrão `btn_salvar_click`.
- **Gerar código:** cria `<tela>_handlers.rs` e `<tela>_handlers.py` ao lado do layout. Regerar é seguro: refaz só o bloco de registro e acrescenta funções novas; o que você escreveu fica.
- **Executar:** abre a tela num processo separado (`aylis-ide --preview tela.json`), com os eventos no console.
- Uso: `cargo run -p aylis-ide -- [tela.json]`; `--png saida.png` desenha a IDE sem janela (testes).
- Ainda não: seleção múltipla e alinhar/distribuir, guias de alinhamento ao arrastar, editor de coleções (colunas do grid, itens), editor de cor/fonte em diálogo e abrir o handler no editor de código.

## 11.7 Android e iOS (fase 12)

- **Mesmo código, mesma pintura:** `aylis-platform-winit` roda no Android (NativeActivity, sem Java) com OpenGL ES ou CPU, e compila para iOS (CPU). O demo `examples/mobile` é uma tela de celular com campos, ComboBox, Switch, um DBGrid de 5 mil pedidos e uma lista; no desktop abre no tamanho de um celular (`cargo run -p aylis-mobile-demo`).
- **Android:** `examples/mobile/build-android.ps1 [-Abi x86_64] [-Install]` gera o APK (14,7 MB com x86_64 + arm64) só com aapt2/zipalign/apksigner (sem Gradle). Testado no emulador (Pixel 6, Android 14): desenho na GPU, teclado virtual, digitação pelo teclado da tela, rolagem com o dedo e inércia, toque em itens, tema escuro, voltar do segundo plano (a superfície é recriada e o estado fica), rotação. Parado: 0% de CPU; com cursor piscando, ~1%.
- **Core, toque:** arrastar o dedo além de 8 pontos vira rolagem (roda com `touch = true`, 1:1 com o dedo) no controle tocado, subindo para os pais — o alvo fica travado mesmo que outro conteúdo passe sob o dedo; soltar em movimento continua por inércia (para no fim do conteúdo). O foco no toque vem ao soltar, então começar a rolar sobre um campo não abre o teclado. Controles que precisam do arrasto desligam `pan_on_touch` (Splitter, designer da IDE).
- **Core, teclado e área segura:** `Component::accepts_text` (Edit, DBEdit) → `Ui::wants_text_input` → a plataforma abre/fecha o teclado virtual (e reabre num novo toque, se o usuário fechou com "voltar"). `Ui::set_safe_area` tira dos filhos do form as faixas da barra de status, da barra de navegação e do teclado (o fundo continua na tela toda); um ScrollBox que encolhe mantém à vista o campo com foco.
- **iOS:** `cargo check --target aarch64-apple-ios` passa e `examples/mobile/build-ios.sh` monta o `.app` para o simulador ou o aparelho — **não testado** (precisa de um Mac com Xcode). Área segura pelo `inner`/`outer` do winit; teclado pelos eventos de tecla do winit.
- Ainda não: Metal no iOS (hoje CPU), área de transferência e "voltar" do sistema no celular, IME completo no Android (a NativeActivity recebe teclas simples; sugestões, correção e emoji pedem a GameActivity), teclado do iOS reduzindo a área segura, multitoque (pinça), toque longo, ícone e nome do app configuráveis no script, e a cor das barras do sistema acompanhar o tema.

## 11.8 Vários forms, projeto e IDE profissional (fase 13)

- **Forms:** `ui.show`, `ui.show_modal(form, |ui, resultado| ...)` (não bloqueia; a janela de baixo fica desabilitada), `ui.close` com `on_close_query` (`ui.cancel_close()`) e `on_close`, `ModalResult` (botões com `modal_result`, o form fecha depois do `on_click`, que ainda pode cancelar), `ui.create_form("frmX")` (como o `Application.CreateForm`). Uma janela por form no desktop; no celular e na web, a tela mostra o form do topo. Forms com `position` (centro do dono, da tela, desenhado, padrão) e `border_style` (redimensionável, fixa, sem borda).
- **Nomes por form:** todo form é um escopo de nomes, inclusive embutido em outro (`ui.set_parent(form, Some(painel))`); dois forms podem ter `btnSalvar`. Caminhos: `ui.find_path("frmClientes.btnSalvar")`, `ui.path_of(id)`. Handlers por form: `frmClientes.btn_salvar_click` tem prioridade sobre `btn_salvar_click`. Referências (`data_source`) procuram no form e nos de fora; com ponto, em outro form (`dmDados.dsClientes`).
- **Protocolo:** `create_form`, `show`, `show_modal` (com `handler` que recebe `on_modal_result`), `close` com alvo, `modal_result`; alvos por caminho; eventos com `form` e `sender.path`. Cliente Python: `aylis.create_form`, `aylis.show`, `aylis.show_modal(form, on_close)`, handlers `"frmX.handler"`.
- **Projeto** (`aylis-project`): pasta com `<nome>.aylisproj`, `forms/`, `assets/{images,audio,video,fonts}`, `themes/`, `src/`; linguagem (Rust/Python) trocável; forms com "criar no início"; `aylis_project::install` (tema, idioma, assets, fábrica de forms). Geração de código por form (`src/forms/<form>.rs|py`), `main` e `Cargo.toml`/cliente Python criados uma vez; regerar preserva o código. Crate fachada `aylis` (`aylis::Application::open(...).run()`, `aylis::prelude`). `aylis-host --project` roda projetos Python. Testado de ponta a ponta: projeto Rust gerado compila e roda (modal, imagem de assets); projeto Python roda pelo host.
- **Componentes novos:** `Image` (`ImageView`: arquivo de `assets/` ou imagem por código, ajuste contain/cover/stretch/center/original, cantos arredondados), `MainMenu`, `PopupMenu`, `MenuItem` (submenus, separador, marcado, atalhos que valem no form inteiro), `popup_menu` em todo controle (clique direito). Core: `Image`/`ImageSource`, `Canvas::draw_image`, serviços `ResourceService` e `FormFactory`, `PaintCtx.ui`/`id`/`design`, modo de design, atalhos (`Component::shortcut`). Label com `font_family` e `bold`.
- **IDE:** menus (Arquivo, Editar, Exibir, Projeto, Executar, Ajuda) com atalhos; gerenciador de projeto (★ principal, ● criado no início, menu de contexto); abas dos forms; designer com seleção múltipla (Ctrl/Shift, área), arrasto em grupo com guias de alinhamento, alinhar/distribuir/mesmo tamanho, trazer para frente/enviar para trás, travar controles, zoom (Ctrl+roda), menu de contexto; Object Inspector com busca, edição de vários selecionados, listas para referências, eventos com ▾ (handlers existentes) e → (abre o código no VS Code ou no editor padrão), seletor de cor, bordas expansíveis, editores de lista (itens, colunas) e de campos do ClientDataSet, imagem copiada para `assets/`; diálogos feitos com o próprio aylis (novo projeto, opções, novo form...); recentes; F9 executa o projeto com a saída em Mensagens; pergunta antes de sair com alterações.
- **Diagnóstico:** `AYLIS_RENDERER=cpu|gpu`, `AYLIS_SNAPSHOT=pasta` (grava o quadro real de cada janela), `AYLIS_DEBUG_GL=1`. Neste Windows com placa AMD, capturas de tela comuns (GDI, PrintWindow) mostram janelas OpenGL em branco, mas o quadro (lido do buffer) está certo.
- Ainda não: editor de fontes em diálogo, lembrar a posição da janela e dos painéis, multitoque nos menus, web sem imagens codificadas (Canvas 2D só desenha o que já está decodificado).

## 11.9 Mídia (fase 14)

- **Crate `aylis-media`** (registrado em `aylis::register_all` e no protocolo; categoria "Media" na paleta).
- **Áudio:** `MediaPlayer` (arquivo de `assets/audio`, `autoplay`, `looping`, `volume`, `muted`, `on_end`, `on_error`). Decodificação com symphonia (MP3, AAC/M4A, FLAC, Vorbis/OGG, WAV) em Rust puro, saída com cpal (WASAPI, CoreAudio, ALSA, AAudio), conversão de taxa e canais. `AudioSource` é um trait: qualquer fonte própria toca no mesmo player.
- **Vídeo em três camadas:**
  1. `FrameSink` — qualquer thread empurra quadros (`Frame::rgba`/`Frame::bgra`) e o player pinta: câmera, gerador, decodificador externo.
  2. `VideoDecoder` (`info`, `seek`, `next_frame`, `audio`, `error`) — o player cuida do relógio, da fila (6 quadros à frente numa thread própria), do áudio sincronizado, de pausa, busca e repetição.
  3. `VideoDecoderFactory` + `register_decoder` — FFmpeg ou codecs próprios entram por extensão, com prioridade sobre o padrão.
- **Padrão no Windows:** Media Foundation (`mf::MfVideo`, `mf::MfAudio`), com aceleração de hardware e conversão para BGRA no próprio leitor; busca precisa; corta o enchimento do H.264 (área visível); `mf::write_mp4` gera MP4 (usado nos testes e no exemplo). O formato garantido é MP4 (H.264 + AAC); o inspector avisa quando o vídeo escolhido não é MP4.
- **VideoPlayer:** `fit`, `radius` (cantos arredondados), controles por cima do vídeo como filhos comuns; pausado não gasta CPU. Exemplo: `cargo run -p aylis-media --example video [-- arquivo.mp4 --muted]`.
- **Core:** `ui.poster()` (o `TThread.Queue`: qualquer thread põe trabalho na fila da interface e acorda a janela), `ImageSource::Bgra`.
- Ainda não: decodificador padrão de vídeo no Android (MediaCodec), iOS/macOS (AVFoundation), Linux (GStreamer) e web (`<video>`); nessas plataformas, por enquanto, o vídeo vem de uma fábrica registrada (ex.: FFmpeg) ou do `FrameSink`. O áudio já funciona em todas.

## 11.10 Executar e publicar (fase 15)

- **Projeto Rust embutido:** o `build.rs` gerado (`aylis_build::project()`, crate `aylis-build`) empacota `.aylisproj`, `forms/`, `themes/` e `assets/` dentro do executável; `aylis::Application::bundled(aylis::include_project!())`. Em desenvolvimento (debug) lê da pasta (alterar um form ou imagem não exige recompilar; `AYLIS_BUNDLE=1` força o pacote). O código gerado agora tem `src/lib.rs` (`app()`, `android_main`, `aylis_start` da web) e `src/main.rs`; projetos antigos com os arquivos gerados intocados são atualizados sozinhos.
- **Ícone:** `publish.icon` (PNG até 256×256) vai para o .exe do Windows (recurso escrito pelo próprio `aylis-build`, sem `rc.exe`), para a janela, para o APK e para a página web. Opções de publicação no `.aylisproj`: `app_name`, `version`, `icon`, `identifier`.
- **Crate `aylis-tools`** (usado pela IDE e pela linha de comando `aylis-tool check | run | publish`):
  - `env`: verifica Rust (versão), linker (MSVC no Windows, Xcode no Mac, cc no Linux), Python e aylis-host, Android (SDK, plataforma, build tools, adb, NDK, JDK 17+, cargo-ndk, alvos, aparelho/emulador), web (alvo wasm32, wasm-bindgen na versão do aylis), iOS (só no Mac). Cada falta traz o comando que corrige (winget, rustup, sdkmanager, cargo install...); em 0,5 s.
  - `run`: desktop (compila com `--message-format=json`, roda o .exe com a saída nas mensagens), Python (aylis-host), Android (APK da arquitetura do aparelho, instala, abre e mostra o logcat do Rust), navegador (servidor local + navegador padrão), iOS (simulador). `Control::cancel` mata o processo.
  - `publish`: desktop Rust = um executável só (20 MB, com forms e assets); desktop Python = pasta com aylis-host, projeto e atalho; Android = APK assinado (arm64 + x86_64); web = pasta com `index.html` e o `.wasm` (o form ocupa a janela do navegador); iOS = `.app` do simulador (num Mac).
- **IDE:** plataforma do Executar na barra de ferramentas; F9 salva, gera o código, verifica o ambiente (se faltar algo, abre o "Verificar ambiente") e executa; erros do compilador e linhas de panic/traceback com arquivo:linha são clicáveis (abrem no VS Code na linha e coluna); Parar (Ctrl+F2) mata o programa; "Verificar ambiente…" com ✓/!/✗ por plataforma e "Corrigir…" (mostra o comando e pergunta antes); "Publicar…" (Ctrl+Shift+F9) com plataforma, nome, versão, identificador, ícone e pasta; a linha "Publicado" abre a pasta.
- **Web:** imagens (PNG, JPEG, WebP, SVG decodificados pelo navegador; RGBA/BGRA dos vídeos) e `clip_round_rect` no Canvas 2D. Temas com os tokens `success` e `warning`.
- Testado: Vendas publicado para Windows (ícone no .exe, roda de outra pasta só com o pacote), Android (APK de 17 MB; F9 instala e abre no emulador) e web (modal e imagem); F9 com erro proposital mostra `src\forms\principal.rs:15:22`.
- Ainda não: assinatura do APK com a chave do desenvolvedor (hoje a de depuração) e AAB para a Play Store; instalador (.msi) — hoje o .exe já é um arquivo só; iOS testado só no papel (falta um Mac); Python só no desktop.

## 11.11 Mais componentes (fase 16)

- **Campos:** `Memo` (várias linhas, seleção, rolagem, Ctrl+A/C/X/V), `NumberBox` (setas, roda, ↑/↓, mínimo/máximo, casas decimais no formato do idioma), `TrackBar`, `DateEdit` (digita no formato do idioma ou escolhe no calendário suspenso; F4/Alt+↓), `TimeEdit`, `Calendar`. Datas são texto ISO nas propriedades (`2026-10-24`), horas `14:30`; `Day` faz a aritmética (dia da semana, somar meses, ler/escrever no formato do idioma). `LocaleInfo` ganhou nomes dos meses e dias.
- **Listas (virtualizadas):** `ListBox`, `RadioGroup` (com colunas), `ListView` (título, detalhe, imagem, seta; 5 mil itens sem custo), `TreeView` (`items` com `>` por nível: `Brasil;>São Paulo;>>Campinas`; ←/→ abre e fecha).
- **Contêineres:** `TabControl` + `TabItem`, `GroupBox`, `Expander`, `StatusBar`, `ToolBar`.
- **Sistema:** `Timer` (`on_timer`; o core agenda e a plataforma dorme até o próximo tique), `OpenDialog`/`SaveDialog` (assíncronos; filtro no formato `Descrição|*.ext`; diálogo do sistema no desktop), `message_dlg`/`show_message` (feitos com o aylis: iguais no desktop, celular e web), `toast`.
- **Gráficos:** `Chart` de linhas, barras, área e pizza (rosca), com legenda, grade com escala "bonita" e o valor sob o mouse; `labels`/`series` em texto (`Vendas=10,20|Custos=5,8`) ou `Chart::set_data`.
- **Core:** timers (`Component::timer_interval`, `Component::timer`, `Ui::run_timers`, `Ui::sync_timer`), toque longo (`on_long_press`; no toque, abre o menu de contexto), `Component::property_changed` (setter com acesso ao `Ui`), serviço `FileDialogService`, tokens de tema `success`/`warning` e estilos dos novos controles.
- **Celular:** o botão Voltar do Android fecha o form de cima (num modal, responde Cancelar); no principal, encerra. **Protocolo:** `toast`, `message_dlg` (com handler), `execute` (Open/SaveDialog); cliente Python com `toast`, `message_dlg`, `execute`.
- Exemplo: `cargo run -p aylis-platform-winit --example components` (três abas com tudo isso; parado, ~0% de CPU).
- Ainda não: Memo com quebra automática de linha; ListBox com várias seleções; área de transferência e IME do teclado virtual no Android (GameActivity); "hoje" do calendário em UTC (sem fuso).

## 11.12 Dados completos (fase 17)

- **Expressões** (`aylis_data::expr`): `cidade = 'São Paulo' and limite >= 1000`, `like` com `%`/`_`, `in (...)`, `is null`, contas (decimais exatos), `+` juntando texto, funções (`upper`, `substr`, `round`, `iif`, `coalesce`, `year`...), campos com espaço entre colchetes. Texto compara sem diferenciar maiúsculas.
- **Campos calculados e lookup:** `Field.kind` = `data`, `calculated` (`expression: "qtd * preco"` ou `Field::on_calculate` em Rust) ou `lookup` (`lookup_data_set`, `key_fields`, `lookup_key_fields`, `lookup_result_field`). Calculados na leitura (sempre atuais, inclusive durante a edição); não aceitam escrita e não vão no delta; o backend manda só os campos de dados.
- **Filtro e índice:** `filter` + `filtered`, `DataSet::on_filter_record` (Rust), `index_field_names` (`cidade;limite desc`, inclusive por calculado). Mudar pela propriedade (JSON, Object Inspector, protocolo) refaz a visão sozinho; o registro atual é mantido; um registro editado para fora do filtro some da visão mas continua no delta.
- **Master-detail:** `master_source` (o DataSource do mestre) + `master_fields` (`cliente_id=id`). O detalhe mostra só os registros do mestre atual, inclusões já vêm com a chave, e `on_master_change` avisa o backend (com os valores em JSON) para carregar sob demanda.
- **Máscaras** (`aylis_data::mask`, `Field.edit_mask`): `0 9 L l A a C c > < \`, `;0`/`;1` (gravar sem/com literais); prontas `CPF`, `CNPJ`, `CEP`, `PHONE`. O DBEdit encaixa enquanto digita; o `post` recusa incompleta.
- **Controles:** `DBMemo`, `DBImage` (blob ou caminho de asset), `DBComboBox`, `DBLookupComboBox`; `DBGrid` com ordenação pelo título (▲/▼), `fixed_columns` e `filter_row` (vira o `filter` do dataset).
- **Protocolo:** `data` ganhou `set_values` (vários campos numa ida e volta), `locate`, `set_filter`, `set_index`, `lookup`.
- Exemplo: `cargo run -p aylis-platform-winit --example data`.
- **DBGrid (completo):** campos lógicos viram caixa de marcar (clique ou Espaço); `multi_select` (Ctrl/Shift+clique, Shift+setas, Ctrl+A; a seleção guarda os registros por id e sobrevive à ordem e ao filtro; `selected_ids()` + `DataSet::goto_id`); `footer: "valor=sum;id=count"` (sum, avg, min, max, count, no formato do campo, recalculado só quando os dados mudam); `incremental_search` (digitar procura na coluna atual; pausa de 1,5 s recomeça).
- **Índices nomeados, parâmetros e situação:** `indexes: "porNome=nome|porLimite=limite desc"` + `index_name` (como `IndexDefs`/`IndexName`); filtro com `:parametro` (`set_param` ou `params: "minimo=100"`); `update_status()`, `status_filter` (`StatusFilter`), `revert_record()` (`RevertRecord`) e `restore_deleted(id)`. No protocolo: `set_index_name`, `set_param`, `revert_record` e `status` no `info`.

## 11.13 Produtividade (fase 19)

- **Hot reload:** `aylis_json::reload(ui, form, json_novo, Some(json_anterior))` aplica o layout novo num form aberto sem recriá-lo: só o que mudou entre as duas versões do arquivo é aplicado (o texto digitado, a aba escolhida e os handlers ligados por código ficam), componentes novos são criados, os que saíram do layout são liberados (os criados pelo código ficam) e a ordem dos filhos segue o layout. `aylis_project::hot_reload` observa `forms/*.json` e o tema (a cada 400 ms) e aplica na thread da interface. Ligado sozinho no app de desenvolvimento lendo da pasta (`Application::run`) e no `aylis-host` (Python); `AYLIS_HOT_RELOAD=0` desliga. Testado: salvar o form com o Vendas aberto troca o texto do botão na hora.
- **Inspetor do app em execução:** o F9 (desktop) passa `AYLIS_INSPECT=<porta>`; o app abre um servidor só em 127.0.0.1 que fala o protocolo do aylis (mais `tree`, `props`, `highlight`). Na IDE, Executar → Inspetor do app em execução (Ctrl+Shift+I): árvore de componentes, propriedades agrupadas por categoria, alterar um valor muda no app na hora, "Destacar no app" pisca uma moldura no controle.
- **Depuração:** rastro de eventos (Executar → Rastro de eventos, `AYLIS_TRACE=1`: cada evento com handler aparece em Mensagens, `evento frmPrincipal.btnClientes.on_click → abrir_clientes`); "Depurar no VS Code" (Shift+F5) gera `.vscode/launch.json` e `tasks.json` (mantendo as configurações do usuário) e abre o VS Code: Rust com `cppvsdbg` (Windows) ou `lldb`, compilando com o `target/` do aylis; Python com `debugpy` (a IDE executa esperando o VS Code anexar; o "Verificar ambiente" sugere instalar o debugpy).
- **Core:** `Ui::set_event_trace`.
- Ainda não: inspetor e hot reload no Android/web (o app roda em outro aparelho), editar propriedades com editores por tipo (hoje texto), pontos de parada dentro da IDE (ficam no VS Code).

## 11.14 Profissionalismo (fase 20)

- **Acessibilidade (AccessKit):** no desktop, cada janela expõe a árvore de controles para leitores de tela (Narrador, NVDA, VoiceOver, Orca): papel pela classe (inclusive herdada: `DBEdit` é campo de texto), nome pelo texto, valor dos campos, estado das caixas de marcação, limites do TrackBar, desabilitado, posição. Ações do leitor (focar, clicar, mudar valor) chegam ao `Ui`. Só é montada quando uma tecnologia assistiva pede. Verificado pela UI Automation do Windows (29 elementos no exemplo de componentes; "Invoke" no botão funciona). `AYLIS_ACCESSIBILITY=0` desliga.
- **Tradução (i18n):** o texto escrito no designer é a chave; `locales/<idioma>.json` (`{"Salvar": "Save"}`) traduz as propriedades de texto ao carregar (`text`, `caption`, `placeholder`, `items`...) e o código usa `ui.tr("...")` (Python: `aylis.tr`). Trocar de idioma com o app aberto (`aylis_project::i18n::set_language`, protocolo `set_language`) refaz formatos e textos — menos os que o código mudou. IDE: Projeto → Traduções… junta os textos dos forms e os `tr("...")` do código (Rust e Python) e edita lado a lado; o idioma do projeto (Opções) lista os arquivos de `locales/`. `AYLIS_LANG=en-US` testa sem mudar o projeto. Hot reload também das traduções. O pacote embutido inclui `locales/`.
- **WebSocket:** `aylis-host --connect ws://servidor:porta` (o backend é servidor) ou `--listen endereço` (o backend conecta); o mesmo protocolo, uma mensagem por mensagem de texto. Cliente Python sem dependências: `Aylis(url="ws://...")` ou `Aylis(listen="host:porta")`. No navegador, `aylis::run_remote_web(url)`: a página (o demo web com `?ws=ws://...`) é montada e comandada por um backend remoto em qualquer linguagem. Testado: backend Python pela rede com o host sem janela (teste automatizado) e no navegador (clique → evento no Python → resposta na tela).
- **Armazenamento em colunas:** os registros carregados do backend ficam em colunas tipadas (inteiros, números, decimais, datas, lógicos; textos com dicionário — cada texto diferente guardado uma vez); só os registros alterados ou incluídos têm valores próprios. 100 mil registros ocupam menos da metade da memória; filtro e índice continuam iguais (`ClientDataSet::row_values`, `storage_bytes`).
- **Correção:** `SystemTime::now()` entra em pânico no navegador; datas "de hoje" agora usam `web-time` (o Calendar quebrava qualquer app web).
- Ainda não: acessibilidade no Android/iOS/web, tradução de textos de imagens, `wss://` direto (use um proxy com TLS), colunas para registros alterados.

## 11.15 Componentes dos projetos, subtipos e app enxuto (fase 21)

Componentes levantados nos apps que vão usar o aylis (NoOBS, RTSPlayer, audioroute, conversa-web, fii-postgres), com as cores próprias do aylis:

- **Base de desenho:** caminhos vetoriais (`Path`: retas, curvas, arcos, leitura do `d` do SVG) com `fill_path`/`stroke_path`, pontas e junções arredondadas, sombra (`draw_shadow`, tokens `sm`/`md`/`lg` e `shadow` nos estilos). Skia e navegador.
- **Ícones:** conjunto próprio (traço arredondado, grade 24×24, mais de 80 nomes) e ícones do app em SVG (`Ui::register_icon_svg`). `Button` com `icon`, `icon_position`, `checkable`/`checked`/`checked_icon`; `Edit` com `icon` e `clear_button`.
- **Componentes novos:** Icon, Badge, Banner, Avatar (iniciais e presença), SegmentedControl, ActivityIndicator, LevelMeter (VU com pico), Breadcrumb, SeekBar (carregado, marcas, horário sob o mouse), HotkeyEdit, DirectionPad (PTZ), NodeGraph (nós, cabos, zoom, ligar arrastando), ChatView (bolhas virtualizadas, respostas, leitura) e Waveform; Chart `donut` e fatias de verdade na pizza; dica (`hint`) em qualquer controle.
- **Subtipos:** `Classe.variante` é um subtipo: aparece na paleta debaixo da classe (seta para abrir), cria com `style` e valores iniciais (`Button.icon` já vem quadrado e sem texto). Registrados pelo aylis, pelo app (`register_variant`) ou só criando o estilo no tema. `ui.create("Button.icon")`.
- **Mistura de temas:** biblioteca de temas no `Ui` (`register_theme`); propriedade `theme` por controle (vale para os filhos); `Theme::mix(classe, tema)` e `"components"` no JSON (uma classe com o visual de outro tema).
- **App enxuto:** o `build.rs` lê os forms e gera o registro só das classes usadas (`aylis::classes::Button`...); a fachada `aylis` tem as features `data`, `dbcontrols`, `media` e `remote`, e o `Cargo.toml` gerado liga só as que os forms pedem (a IDE atualiza a cada "Gerar código"; o inspetor fica na feature `inspect`, ligada só ao executar). Form com 4 tipos de componente: 22,09 MB → 19,02 MB com Skia (−13,9%) e 1,97 MB com Direct2D.
- **Documentação:** `docs/` com funcionamento, manual de componente (compilado como teste), tempo de execução carregamento e o custo de cada componente (`docs/medidas.md`).
- **Exemplo com banco (`examples/fii`):** carteira de FIIs sobre o PostgreSQL do fii-postgres. O backend é o único que conhece o banco: lê os fundos, cotações, proventos e lançamentos e devolve registros ao `ClientDataSet`; grava o delta numa transação, com um ponto de salvamento por registro e id gerado pelo banco. Testado contra o banco real com a transação desfeita no fim.
- **Renderizador sem Skia no Windows (`aylis-d2d`, feature `d2d`):** Direct2D + DirectWrite + WIC, com cache de layouts de texto e de geometrias. Executável de 19,3 MB → 2,2 MB (−88%); quadro do DBGrid 0,57 ms (Skia GPU 1,07 ms, Skia CPU 2,25 ms); memória ~45–70 MB (Skia GPU ~300–570 MB, Skia CPU ~12–16 MB). `AYLIS_BENCH=N` mede quadros e memória. Escolha por plataforma no projeto (`"render"` no `.aylisproj`; Novo projeto e Projeto → Opções na IDE, com os números medidos): vira features por `[target.'cfg(...)'.dependencies]` no `Cargo.toml` gerado e o app abre com ela. Padrão dos projetos novos: Direct2D no Windows, Skia nas outras. Detalhes em `docs/renderizadores.md`.
- **Temas de plataforma (`aylis_core::themes`):** Material 2 e 3, Android 7 (AppCompat), iOS 7, 13 e 26 (Liquid Glass), Windows 7, 10 e 11, WinUI 3 (recursos de tema do Fluent, translúcidos sobre a Mica) e Windows Phone (Metro, `Look::WinPhone`), claro e escuro. Mudam cores, raios, fontes e a forma dos controles (`Look`: Switch, CheckBox, RadioButton, TrackBar, Button, campos, ProgressBar, SegmentedControl). Qualquer componente com qualquer estilo (`theme` por controle, `mix` por classe). Projeto: `theme` aceita o nome do tema e `theme_mix` mistura por classe. IDE: Projeto → Aparência (Ctrl+Shift+A) com prévia e a lista de quem usa qual tema e por quê (`Ui::theme_source`). Detalhes em `docs/temas.md`.
- **Medidas (`examples/medidas`):** por componente, quanto acrescenta ao executável (base 18,61 MB; controles simples de 1,5 a 20 KB; ícones ~40 KB e camada de dados ~290 KB compartilhados) e à RAM (0,4 a 1,4 KB de heap por instância).
- **Fontes e ícones offline (`resources/`, `scripts/recursos.py`):** 32 famílias do Google Fonts (OFL/Apache/UFL) e os pacotes Material Symbols (outlined, rounded, sharp; Apache-2.0) e Fluent (MIT), com variantes `-fill`. Registro de fontes de arquivo (`aylis_core::fonts`) usado por Skia, DirectWrite e navegador; pacotes de ícones `pacote:nome` (`Ui::register_icon_pack`). IDE: Projeto → Fontes (catálogo, prévia, baixar outra do Google Fonts, fonte do projeto, copia arquivos e licença para `assets/fonts/`) e Projeto → Ícones (busca, preenchido, aplicar no componente, copiar nome, importar SVG para `app:`). Ao salvar e gerar código, `assets/icons/` recebe só os ícones citados (1 ícone Material: ~12 KB, quase tudo licença; o pacote inteiro tem 2,3–5,0 MB). SF Pro/SF Symbols e Segoe UI não vão (licença). Detalhes em `docs/fontes-e-icones.md`.
- **Lottie, SVG, Markdown e WebView:** `aylis-vector` (player Lottie próprio pelo Canvas: formas, gradientes, pré-composições, parentesco, máscaras, track mattes com máscara de alfa no Skia/Direct2D, Trim Paths; pacote padrão Noto do Google com as 55 animações do conversa-web, conferidas com o lottie-web; SVG com gradientes, `use`, transformações, par-ímpar e tracejado), `aylis-markdown` (CommonMark com tabelas e tarefas, quebra por palavra, rolagem, `on_link`) e `aylis-webview` (vista nativa: WebView2/WKWebView pelo wry, `<iframe>` no navegador; `on_load`, `on_navigate`, `on_message`, `eval`). Canvas: `Path::even_odd`, `clip_path`, máscara de alfa. IDE: Projeto → Animações; ao salvar, `assets/lottie/` recebe só as citadas. Detalhes em `docs/lottie-svg-markdown-webview.md`.
- **Paridade com o Vuetify (`docs/vuetify.md`):** 29 componentes novos — GridLayout, FlowLayout, Divider, NavigationDrawer, BottomNavigation, BottomSheet, Carousel, Chip, ChipGroup, Rating, RangeSlider, OtpInput, SplitButton, AutoComplete, ColorPicker, ColorEdit, FileInput, FileDropZone (arrastar do sistema), Snackbar, SkeletonLoader, ProgressCircle, EmptyState, Pagination, Stepper, Timeline, SpeedDial, AvatarGroup, EventCalendar, CommandPalette — e acréscimos: Edit com `label`, `helper`, `error`, `counter`, `prefix`/`suffix`, `mask`, `rules` + `validate_form`; Calendar com intervalo, mês e eventos; Chart `sparkline` e `heatmap`; ScrollBox com `on_end_reached` e puxar para atualizar; `Label.kbd`. Core: `Component::arrange` (layout de contêiner) e `files_dropped`/`Ui::drop_files`. Fechados também: TimePicker (relógio) aberto pelo TimeEdit, Badge preso a outro controle (`Component::attach`), fila do Snackbar, EventCalendar com semana e dia (eventos com horário), `Label.highlight`, `ScrollBox.horizontal` e `ChipGroup` em uma linha. Galeria: `--example gallery` (`--fim` rola até os últimos).
- **Material 3 e iOS 26 (`docs/material3-ios.md`):** Liquid Glass de verdade — `Canvas::backdrop_blur` (Skia com filtro de fundo, Direct2D com cópia do alvo + Gaussian Blur, navegador com `filter: blur()`) e a propriedade de estilo `material` (`glass`, `frosted`), ligada pelo tema iOS 26 em botões, cartões, campo de busca, segmentado e barras. Componentes novos: AppBar (top app bar / navigation bar), NavigationRail, SideSheet, PageControl, UpDown (stepper), WheelPicker, ButtonGroup, Gauge, ActivityRings, LoadingIndicator (Expressive) e `action_sheet`. Visual por look: tab bar flutuante de vidro, FAB menu e split button do M3, chips de cantos 8, slider Expressive, sheet com detents. Subtipos: Button elevated/tonal/text/fab_extended/icon_*/glass/glass_prominent, Panel filled/outlined/glass, Edit filled/outlined, ToolBar.floating, AppBar center/medium/large. Exemplo: `--example catalog`.
- **Editores visuais de propriedades (`docs/editores-visuais.md`):** como as ferramentas do DevTools, escolhidos no Object Inspector pelo tipo da propriedade. Tipos novos `PropKind::Angle`, `Duration`, `Shadow` e `Easing` (no `properties!`, `campo: f32 as Angle`). Relógio de ângulo (`AngleDial`, também na própria linha), editor de sombra (`ShadowEditor`: quadro do deslocamento, desfoque, espalhamento, cor, prontas), editor da curva (`EasingEditor`: pontos de controle, prontas do aylis/CSS/Material 3/Fluent, bolinha de prévia), linha do tempo (`TimingEditor`: atraso, duração, ida e volta, repetir, curva do valor e agulha) e seletor de cores com o quadro HSV ao lado dos tokens do tema. Core: `Easing::CubicBezier` + `parse`/`to_spec`, `delay` nas animações, `Shadow::parse`/`to_spec`, `rotation` e `shadow` em todo controle, gradiente com ângulo no `Rectangle` (`fill2`, `gradient_angle`).
- **3D (`docs/3d.md`):** crate `aylis-3d` (feature `3d`) com o componente `Model3D`: glTF 2.0 / GLB (malhas, PBR, esqueleto na GPU, morph targets, clipes com transição), câmera orbital, desenhado com `wgpu` e composto com os controles na GPU, sem cópia: no Windows, textura e cercas compartilhadas do Direct3D 12 abertas pelo Direct2D, que passou a desenhar por Direct3D 11 + swap chain flip (mais rápido: −2% a −41% nas telas medidas). Medido: 3D sozinho 0,27 ms, controles 0,48 ms, os dois juntos 0,65 ms. Composição sem cópia também no Skia GPU (o `wgpu` no mesmo contexto OpenGL/OpenGL ES do Skia: Windows, Linux, Android) e no navegador (WebGPU copiado na GPU com `drawImage`), testados no Windows, no emulador do Android e no Chrome. Reserva por cópia na CPU no macOS e no iOS (próximos: IOSurface no macOS, Skia Metal no iOS, precisam de um Mac). Importar FBX na IDE: o `source` do `Model3D` aceita `.fbx`, convertido com o ufbx para `assets/models/<nome>.glb` (malhas, esqueleto, blend shapes, materiais, texturas, um clipe por take); `aylis_3d::glb::to_glb` grava qualquer modelo. `scripts/medir.py` mede e compara os exemplos.
- Ainda não: editor de código com realce, mapa, visualizador de PDF/Office, seletor de emoji, atualizar automaticamente o `Cargo.toml` de projetos antigos.

## 12. Fases

1. **Core:** geometria, `Component`/`Control`, arena, propriedades e registro, motor de alinhamento com testes, tema e `PaintCtx`, traits de serviço.
2. **Skia + janela no Windows (CPU).**
3. **GPU (OpenGL)** com troca por feature.
4. **Eventos:** pointer, hover, captura, click, teclado, foco. Animator.
5. **Controles básicos** + tema padrão claro e escuro + demo de alinhamento + `GlowButton` (prova de extensibilidade).
6. **Teste rápido da web e do Linux.**
7. **Carregar e salvar em JSON** + JSON Schema.
8. **`aylis-data` + controles DB\*** (demo de cadastro).
9. **`DBGrid`** virtualizado.
10. **C ABI e protocolo remoto.**
11. **IDE:** paleta, form designer, inspector, estrutura, geração de código dos eventos.
12. **Android e iOS.**
13. **Vários formulários e projeto:** `show`, `show_modal` (com função de retorno e `ModalResult`), `close`, `on_close_query`/`on_close`, uma janela por formulário, nomes únicos só dentro de cada formulário; projeto em pasta (`.aylisproj`, `forms/`, `assets/`, `themes/`, `src/`) com a linguagem como opção trocável; IDE com gerenciador de projeto, abas, menus, seleção múltipla, alinhar/distribuir, guias, editores de coleções e de campos; componente `Image`; `MainMenu`/`PopupMenu`.
14. **Mídia:** `MediaPlayer` (áudio, symphonia + cpal) e `VideoPlayer` em três camadas (quadros empurrados de qualquer thread, `VideoDecoder` plugável, fábricas por extensão). Padrão: decodificador do sistema; formato garantido MP4 (H.264 + AAC); FFmpeg ou codecs próprios plugáveis.
15. **Executar e publicar:** verificar ambiente (Rust, Python, Android, iOS, web) com correções sugeridas; F9 compila e executa com lista de erros clicável, saída e "Parar"; publicar (.exe, APK, .app, pacote web).
16. **Mais componentes:** Memo, NumberBox, DateEdit/calendário, TimeEdit, TrackBar, ListBox, RadioGroup, TabControl, GroupBox, Expander, StatusBar, ToolBar, ListView, TreeView, Timer, OpenDialog/SaveDialog/MessageDlg, gestos e Toast no celular, Chart.
17. **Dados completos:** máscaras, campos calculados e lookup, filtro e índice, master-detail, DBLookupComboBox, DBMemo, DBImage, grid com ordenação/filtro/colunas fixas, eventos de Field remotos em lote.
18. **Relatórios e impressão** — adiada.
19. **Produtividade:** hot reload do layout no app em execução, inspetor do app em execução, depuração.
20. **Profissionalismo:** acessibilidade (AccessKit), tradução (i18n), transporte WebSocket, armazenamento em colunas.
21. **Componentes dos projetos:** caminhos, sombra e ícones; componentes modernos (Badge, Banner, Avatar, SegmentedControl, LevelMeter, SeekBar, NodeGraph, ChatView...); subtipos na paleta; mistura de temas; app enxuto (só as classes e os crates usados); documentação.
