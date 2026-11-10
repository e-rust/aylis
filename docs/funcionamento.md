# Como o Aylis funciona

## Visão geral

```
           seu app (forms/*.json + handlers em Rust ou Python)
                               │
   ┌───────────────┬──────────┴───────────┬────────────────┐
aylis-controls   aylis-data   aylis-dbcontrols   aylis-media     (componentes)
   └───────────────┴──────────┬───────────┴────────────────┘
                          aylis-core        (Ui, Component, layout, eventos, tema, Canvas)
                               │
          ┌────────────────────┼─────────────────────┐
     aylis-skia (CPU/GPU)   aylis-web (canvas 2D)   aylis-platform-winit (janelas, teclado, toque)
```

| Crate | O que tem |
|---|---|
| `aylis-core` | o `Ui` (a árvore de componentes), o trait `Component`, o `ControlBase`, propriedades, alinhamento, eventos, foco, temas, animações, timers, forms, o trait `Canvas`, caminhos e ícones. **Não conhece o sistema operacional nem o Skia.** |
| `aylis-controls` | os controles: Button, Edit, Memo, ComboBox, ListView, Chart, TabControl... e os modernos (Badge, Banner, Avatar, SegmentedControl, LevelMeter, SeekBar, NodeGraph, ChatView...) |
| `aylis-data` | `ClientDataSet`, `Field`, `DataSource`, filtros, índices, delta e reconciliação |
| `aylis-dbcontrols` | `DataGrid`, `DataNavigator`, `DataRepeater`, `DBImage`, `DBChart`... |
| `aylis-media` | `MediaPlayer` (áudio) e `VideoPlayer` |
| `aylis-skia` | o `Canvas` em Skia (CPU ou GPU) e a medição de texto |
| `aylis-web` | o `Canvas` no navegador (canvas 2D) |
| `aylis-d2d` | o `Canvas` no Windows sem Skia: Direct2D, DirectWrite e WIC (veja [Renderizadores](renderizadores.md)) |
| `aylis-platform-winit` | janelas, teclado, mouse, toque, IME, acessibilidade, área de transferência |
| `aylis-json` | forms e temas em JSON, JSON Schema, hot reload |
| `aylis-project` | o projeto (`.aylisproj`), geração de código, traduções |
| `aylis-build` | o `build.rs` dos projetos: embute o projeto e gera o registro enxuto |
| `aylis-remote`, `aylis-host`, `aylis-capi` | protocolo remoto (Python e outras linguagens), C ABI |
| `aylis-ide` | a IDE (feita com o próprio Aylis) |
| `aylis` | a fachada: junta tudo e tem o `Application` |

## O `Ui` e os componentes

O `Ui` guarda **todos** os componentes numa arena. Cada componente tem um
`ComponentId` (como um ponteiro seguro: depois de liberado, o id antigo não aponta para
mais nada), um pai, filhos, um nome e handlers de evento.

```rust
let mut ui = Ui::new();
aylis_controls::register(&mut ui);            // classes criáveis pelo nome
let form = ui.add_root(Form::new("Cadastro", 480.0, 320.0));
let ok = ui.add(Button::new("Salvar"), form);   // filho do form
ui.set_name(ok, "btnSalvar")?;
```

Um componente é uma `struct` que implementa o trait `Component`. Todos os métodos têm
implementação padrão (sobrescreva o que precisar), menos `class_name`:

- `control()` devolve o `ControlBase` (posição, tamanho, `align`, `margins`, `visible`,
  `enabled`, `style`, `theme`, `hint`...). Quem devolve `None` não é visual
  (`ClientDataSet`, `Timer`);
- `props()`, `get_prop()` e `set_prop()` publicam as propriedades (o que o Object
  Inspector, o JSON e o protocolo remoto enxergam). A macro `properties!` escreve os três;
- `paint()` desenha; `pointer_down()`, `key_down()`, `wheel()`, `text_input()`... tratam a
  entrada;
- `inner()` / `inner_mut()` fazem a "herança" por composição: um `NumberBox` guarda um
  `Edit` e tudo que ele não sobrescreve vai para o `Edit` (propriedades, pintura,
  eventos).
- `invoke()` publica **métodos pelo nome**, o que não é propriedade (`undo`, `save`,
  `select_all`): `ui.invoke(id, "save", &["a.png".into()])` em Rust e
  `{"cmd": "invoke", "target": "pcDesenho", "method": "save", "args": ["a.png"]}` pelo
  protocolo, iguais em todas as linguagens.

**PaintCanvas** (paleta "Additional"): uma imagem para desenhar com o mouse, como a área do
Paint. Ferramentas (`tool`): `select`, `pencil`, `brush`, `eraser`, `fill`, `picker`,
`line`, `rectangle`, `ellipse`; `color` e `back_color` (botão esquerdo e direito),
`brush_size`, `filled`, `zoom`. A seleção anda com o mouse e com as setas (Shift: 10
pixels), Delete apaga, Esc/Enter soltam. Métodos: `undo`, `redo`, `clear`, `new`, `open`,
`save` (PNG, JPEG, BMP), `select_all`, `copy`, `cut`, `paste`, `delete`, `size`,
`checksum`. Eventos: `on_change`, `on_pick` (o conta-gotas), `on_cursor` (a posição do
mouse na imagem). Desenha nos próprios pixels: o resultado é igual em qualquer
renderizador. O Paint de `examples` usa.

## Layout

Cada controle tem `align` (`top`, `bottom`, `left`, `right`,
`client`, `center`, `contents`...), `margins`, `padding` e `anchors`. O `Ui` refaz o
layout quando algo muda (tamanho, visibilidade, alinhamento) e avisa o controle com
`resized` (o `OnResize`).

**Da direita para a esquerda (árabe, hebraico, persa, urdu).** `ui.set_right_to_left(true)`
espelha a tela: o que está alinhado à esquerda vai para a
direita (posições, `align` e âncoras), cada controle se desenha espelhado (a caixa do
CheckBox à direita do texto, a seta do ComboBox à esquerda) e o mouse chega espelhado. O
texto e as imagens continuam legíveis, e o alinhamento do texto troca de lado. Campos de
texto (Edit, Memo, editores de código) só mudam de lugar. `keep_left_to_right` num
controle o trava, com os filhos, da esquerda para a direita (um gráfico, um mapa).
`aylis_project::i18n::set_language` liga sozinho quando o idioma é um desses
(`aylis_core::is_right_to_left_language`); a IDE em árabe também espelha, menos a
superfície do designer.

## Eventos

A plataforma entrega ao `Ui` eventos de baixo nível (`pointer_down`, `key_down`, `wheel`,
`text_input`...) em coordenadas do form. O `Ui`:

1. acha o controle sob o mouse (o mais de cima com `hit_test = true`);
2. atualiza o estado visual (`hover`, `pressed`, `focused`), que o tema usa;
3. chama o método do componente (`pointer_down`...) com as coordenadas locais;
4. dispara os eventos publicados (`on_click`, `on_change`...) para os handlers.

Teclas vão para o controle com foco e sobem para os pais até alguém tratar
(`ctx.set_handled()`). Tab e Shift+Tab andam pelo foco (`tab_stop`). Clique direito abre
o `popup_menu`. Toque longo dispara `on_long_press`. Mouse parado sobre um controle com
`hint` abre o balão da dica depois de 0,6 s.

Dois dedos na tela viram gesto (`on_gesture`): o controle sob o
primeiro dedo recebe um `GestureEvent` com a fase (começo, andamento, fim), o centro entre
os dedos, a escala (`scale` desde o começo, `delta_scale` desde o último) e o giro em
radianos (`rotation`, `delta_rotation`), e quanto o centro andou (`pan`). Se o controle não
tratar, sobe para os pais. O toque do primeiro dedo (rolagem, clique) é cancelado, e o
dedo que sobrar depois do gesto não vira clique. O `Model3D` aproxima e gira a câmera e o
`Graph` dá zoom no ponto entre os dedos; no macOS a pinça e o giro do touchpad chegam
como o mesmo gesto.

Arrastar e soltar entre controles: num
controle com `draggable`, andar 5 pontos com o botão pressionado dispara `on_drag_start`;
o handler começa o arrasto com `ui.begin_drag(dados, rótulo)` (sem a chamada, nada
acontece). O rótulo acompanha o mouse; o destino é o controle com `accepts_drops` sob ele
(ou o pai mais próximo que aceite), que recebe `on_drag_over` com os dados ao entrar
(`ui.reject_drop()` recusa) e `on_drag_drop` ao soltar, com o contorno de destaque
enquanto aceita. A origem recebe `on_drag_end` (`true` se foi solto num destino) e não
recebe clique. Esc cancela.

**A imagem junto do cursor** (`DragImage`, aba System): um componente não visual que o
controle aponta em `drag_image`. O que vai, na ordem: a imagem por código
(`set_picture`, a miniatura do item), o arquivo `image`, o desenho do controle `control` ou,
sem nada disso, o desenho do próprio controle arrastado (`ui.snapshot(id, escala)`: o
controle como está na tela, sem sair do lugar). `width`/`height` (0 = 160 × 90 numa imagem,
o tamanho do controle num desenho; com um lado só, o outro segue a proporção), `opacity` e o
ponto preso ao cursor (`hot_x`, `hot_y`; o padrão é o meio, um pouco acima, como no
Explorer). Num arrasto só, o `on_drag_start` troca:
`ui.set_drag_image(Some(DragVisual::new(imagem, tamanho)))`. No Windows a imagem vai numa
janela própria meio transparente (alfa por pixel, `UpdateLayeredWindow`), por cima de tudo e
transparente para o mouse: segue o cursor pela tela inteira, fora da janela também, como o
arrasto do Explorer. Nos outros sistemas o form a pinta junto do mouse, dentro da janela
(no navegador, sem o desenho fora da tela, o controle sem imagem leva só o rótulo). O rótulo
vai embaixo da imagem.

Um handler recebe o `Ui` inteiro, o id de quem disparou e os argumentos:

```rust
ui.on(ok, "on_click", |ui, sender, _args| {
    let nome = ui.get_prop(ui.find("edtNome").unwrap(), "text").unwrap();
    println!("salvando {nome:?}");
});
```

Os eventos disparados dentro de um método de componente são entregues **depois** que o
método termina. Por isso um handler pode alterar o próprio componente que disparou o
evento.

**A roda do mouse** anda `wheel_step` pontos por dente (40 por padrão; navegadores usam
100) e pode deslizar até o destino (`smooth`, `smooth_ms`) em vez de saltar. No projeto:
`"scroll": { "wheel_step": 100, "smooth": true, "smooth_ms": 160 }` no `.aylisproj`; pelo
código, `ui.set_scroll_options(ScrollOptions { .. })`. O passo vale em todo controle que
rola; o `ScrollBox` aceita os seus (`wheel_step` > 0 e `smooth`: `default`, `on`, `off`). O
`WheelEvent` traz `lines` quando veio de uma roda com dentes (o touchpad e o dedo não
deslizam: já são contínuos).

### Teclado do celular sobre os campos

Quando o teclado do celular abre, o form encolhe (a área segura desconta o teclado) e o
campo com foco fica à vista: o Aylis rola os `ScrollBox` em volta e, se ainda não bastar, o
próprio form (`Ui::reveal`, que o app também pode chamar). Com o teclado aberto, o form
rola com o dedo para chegar aos outros campos, e o conteúdo rolado não passa por baixo da
barra de status. Quando o teclado fecha, o form volta ao lugar. Não há nada a configurar;
o mesmo vale para um form maior que a janela no desktop (a roda do mouse rola o form).
Um `ScrollBox` com `align = client` em volta dos campos continua sendo o jeito de ter uma
área rolável fixa, com barra.

### Barra de título própria

`border_style = custom` tira a barra do sistema (a janela continua redimensionável pelas
bordas) e a `TitleBar` desenha a do app: título (o `caption` do form), minimizar,
maximizar/restaurar e fechar (com o vermelho do Windows). Controles postos dentro dela
(menu, busca, abas) ficam à esquerda dos botões. Arrastar a barra move a janela e o duplo
clique maximiza. Por baixo, `ui.window_command(form, WindowCommand::…)` (minimizar,
maximizar, fechar, arrastar, redimensionar por uma borda, sempre por cima) vale para
qualquer controle; o exemplo `titlebar` mostra.

### Ícone na bandeja

O `TrayIcon` (paleta "System", não visual) põe o app na bandeja do sistema, perto do
relógio (Windows e macOS, feature `tray`): `icons` com um arquivo (parado) ou vários
(animação a cada `interval` ms), `hint`, `badge` (um ponto `•` ou um número, desenhado sobre
o ícone, como os apps de mensagem) e `menu_items` (`Abrir;-;Sair`) com `menu_theme`
`light`, `dark` ou uma cor. O menu é um form do Aylis aberto no ponto do clique, por cima de
tudo e fora da barra de tarefas, e fecha ao perder o foco. Eventos: `on_click`,
`on_dbl_click`, `on_menu` (o item). Para isso os forms ganharam `topmost`,
`show_in_taskbar`, `on_activate` e `on_deactivate`. Exemplo `tray`.

### Notificações do sistema

O `Notification` (paleta "System") mostra o toast do Windows, que fica na central de
notificações e a notificação do Android: `title`, `text`, `image` (PNG ao lado do texto,
arquivo ou asset), `app_name` e `app_id` (`empresa.app`). Mais: `notification_id`
(mostrar de novo com o mesmo número atualiza), `channel`/`channel_name` (o canal do
Android, que o usuário configura), `priority` (low, default, high: aparece por cima),
`actions` (`"ver:Ver pedido;ok:Recebi"`; o toque chega em `on_action` com o
identificador), `on_click` (toque no corpo), `progress` (0 a 1), `ongoing` (fixa) e
`Notification::cancel`. Com `form` (o nome de um form do projeto), o form inteiro é
desenhado fora da tela (`Ui::render_png`, `form_width` x `form_height`) e vira a imagem
grande da notificação: um layout personalizado feito no designer. No Android 13+ a
permissão é pedida na primeira notificação. `Notification::show(ui, id)` mostra. Para o Windows mostrar o nome do app,
o AppUserModelID precisa estar registrado para o usuário; o Aylis registra em
`HKCU\Software\Classes\AppUserModelId\<app_id>` na primeira notificação (um instalador
faria o mesmo). Onde o sistema não oferece (navegador, celular), vira o `toast` dentro do
app.

### Caixas de mensagem

Feitas com o próprio Aylis (iguais no desktop, no celular e no navegador; nada da caixa do
sistema). Pelo código, `message_dlg(ui, texto, DlgKind::Confirmation, &[Yes, No], |ui, r| ...)`
e `show_message` (como o `MessageDlg` e o `ShowMessage`). Como componente, o
`MessageDialog` (paleta "System", como o `TTaskDialog`): fica no form com `title` (vazio:
o do tipo), `text`, `kind` (information, warning, error, confirmation) e `buttons` na ordem
em que aparecem (`"yes,no,cancel"`); `MessageDialog::execute(ui, id)` abre e `on_close`
recebe o botão (`"yes"`, `"no"`...). `execute_then` também entrega a resposta a uma
closure. As confirmações da própria IDE usam esse componente.

### Correção e sugestão de texto

**Propriedades do `Edit` e do `Memo`:**
- `spellcheck`: sublinhado ondulado nas palavras erradas; o clique direito numa delas mostra
  as correções e "Adicionar ao dicionário". A palavra ainda sendo digitada só é marcada
  quando termina.
- `autocomplete`: o resto da palavra sugerida aparece apagado depois do cursor; Tab ou seta
  para a direita aceita, Esc dispensa.
- `language`: o idioma (`"pt-BR"`); vazio, o do sistema.
- `keyboard_suggestions`: no celular, a faixa de sugestões e a correção do teclado do
  sistema (ligado por padrão; desligue em códigos e logins).

**O motor:** `TextAssist` (em `aylis_core::textassist`) tem `check`, `suggest`, `complete` e
`add_word`.
- **Windows:** a plataforma instala o motor nativo, com o corretor do sistema
  (`ISpellChecker`, o dicionário do usuário) e a previsão de texto
  (`TextPredictionGenerator`).
- **Linux:** o Enchant 2 (que usa o Hunspell e os dicionários do sistema), aberto em tempo
  de execução (`libenchant-2.so.2`); sem ele instalado, fica sem motor nativo. As
  sugestões de completar saem das palavras do dicionário Hunspell (`.dic`).
- **Outros sistemas (macOS, iOS, Android):** por enquanto, só um motor posto pelo app.
- **Trocar o motor:** `ui.services_mut().text_assist = Some(Box::new(meu_motor))`.
- **Motores prontos:** `WordList`, uma lista de palavras (termos do negócio, nomes de
  produtos), e `Chain`, que junta dois motores (o do app primeiro, o nativo depois).
- Exemplo: `cargo run -p aylis-platform-winit --example spellcheck`.

**Seleção e cursor**, nas propriedades só de execução (não vão para o JSON,
não aparecem no Object Inspector):
- `sel_start`, `sel_length`, `sel_text`: gravar `sel_text` troca a seleção e dispara
  `on_change`, mesmo com `read_only`.
- No `Memo`, também `caret_line` e `caret_column`, a partir de 1.
- Em Rust: `select`, `insert_text`, `go_to`.
- O evento `on_caret_move` avisa quando o cursor ou a seleção mudam.

**A aparência do cursor:** `caret_color` (vazio: a cor do texto), `caret_width`,
`caret_style` (`line`, `block`, `underline`) e `caret_blink` (o ciclo em ms; 0 não pisca).

Uma propriedade calculada entra no `properties!` na seção
`computed: nome: Tipo => getter / setter` e fica marcada como `runtime`.

### Gravação de uso e métricas

Para achar a causa de um problema (no desenvolvimento e na implantação), na paleta
"System":

- **`EventRecorder`**: grava a sessão inteira num arquivo `.aylisrec` (JSON Lines), como
  uma caixa-preta.
  - **O que entra:**
    - cliques, com o componente atingido e o texto dele;
    - arrastos (no máximo um a cada 0,1 s), roda e teclas/atalhos com o controle em foco;
    - texto digitado (campos com `password` gravam `*`; `mask_text` mascara tudo);
    - eventos com handler, forms abertos e fechados (e se eram modais);
    - erros, inclusive pânicos: a última linha antes de o app cair.
  - **Bloqueado:** a entrada que um modal bloqueou também entra, marcada `blocked`.
  - **Começa sozinho** com `active`; também por `EventRecorder::start`/`stop`.
  - **Onde grava:** em `%LOCALAPPDATA%\<app>\ecordings` (ou `folder`), mantendo as `keep`
    mais novas.
- **`Analytics`**: métricas de uso somadas entre as sessões em `analytics.json`.
  - **O que soma:**
    - sessões e tempo de uso;
    - aberturas e tempo por form;
    - cliques por componente, eventos por handler e atalhos;
    - erros (os 10 últimos com a mensagem);
    - métricas do app: `Analytics::track(ui, "order_saved", "1250.00")` conta, guarda o
      último valor e soma se for número.
  - **Ler:** `Analytics::summary` devolve o JSON.

Os dois ouvem os **observadores de atividade** do núcleo (`Ui::add_observer`,
`aylis_core::activity`): toda entrada, evento com handler, form e erro (`Ui::report`) chega a
quem estiver ouvindo, sem o núcleo saber de arquivos. Nada vai para a rede.

**Na IDE:**
- **Abrir:** Arquivo → Abrir gravação… mostra o resumo (por tipo, mais clicados, atalhos,
  erros) e a linha do tempo, com busca, filtro e, ao clicar numa linha, o acontecimento
  completo e os cinco anteriores.
- **A IDE grava o próprio uso** em `%LOCALAPPDATA%\aylis\ecordings`, e os avisos "⚠" dela
  entram como erros.
- **Ligar/desligar:** Ajuda → Gravar o uso da IDE; Ajuda → Abrir pasta das gravações.
- **Em PNG:** `aylis-ide --png saida.png --recording arquivo.aylisrec` desenha o visualizador.

### Arquivos, permissões e biometria

**Arquivos.** `OpenDialog`, `SaveDialog`, `FileInput` e `FileDropZone` funcionam em toda
plataforma: o diálogo do sistema no desktop, o `<input type="file">` no navegador e o
seletor de documentos do Android e do iOS. O caminho escolhido se lê e se grava com
`aylis_core::files::read` e `write` (no lugar de `std::fs`), e o mesmo código vale para todas:

| Plataforma | Abrir | Salvar |
|---|---|---|
| Desktop | o arquivo escolhido | o arquivo escolhido |
| Navegador | o conteúdo fica na memória (`memory:/1/foto.png`) | `download:/nota.txt`: gravar baixa o arquivo |
| Android | uma cópia no cache do app | o documento criado (`content://...`), gravado pelo sistema |
| iOS | uma cópia no app | a pasta Documentos do app (o app Arquivos mostra) |

**Permissões** (`Permissions`, não visual). `permissions` lista as que o app usa
(`camera;microphone;location;notifications;photos;contacts;calendar;bluetooth`) e `reasons`
diz por que (`camera=Para a foto do perfil`). `Permissions::state(ui, Permission::Camera)`
responde `Granted`, `Denied`, `Prompt` (ainda não perguntada) ou `Unavailable`;
`request`/`request_all` mostram as perguntas do sistema, uma por vez, e a resposta chega
no `on_result` (`camera=granted;microphone=denied`). Nas outras linguagens:
`frmX.prmApp.state("camera")`, `frmX.prmApp.request_all()`. A publicação declara no
AndroidManifest as permissões listadas e põe no Info.plist do iOS os textos de `reasons`
(sem texto, um genérico: sem a chave o iOS fecha o app ao perguntar). No desktop tudo vem
liberado; no navegador valem câmera, microfone, localização e notificações (as outras
respondem `unavailable`).

**Biometria** (`Biometrics`, não visual). `Biometrics::availability` diz o tipo
(`fingerprint`, `face`, `iris`, `any` ou `none`); `authenticate` mostra a verificação do
sistema com `reason`, e o resultado chega em `on_success`, `on_cancel` ou `on_fail` (com o
motivo que o sistema deu). Com `allow_device_credential`, o PIN ou o padrão do aparelho
também valem. Por plataforma: `BiometricPrompt` no Android (9 ou mais novo; a publicação
declara `USE_BIOMETRIC`), `LocalAuthentication` no iOS (Touch ID, Face ID; o texto do Face
ID vai no Info.plist), Windows Hello no Windows (rosto, digital ou o PIN do Windows) e o
WebAuthn no navegador, com o autenticador do próprio aparelho: na primeira vez o navegador
oferece salvar uma "chave de acesso" do site; nas seguintes, só confirma. O WebAuthn exige
`https` ou `localhost` (um endereço IP não vale).

Exemplo: `cargo run -p aylis-platform-winit --example device --features d2d`.

### NFC e Bluetooth

**NFC** (`Nfc`, não visual). Com `active`, lê as tags que encostarem (só com o app na
frente): cada uma chega no `on_tag` (o texto é o id, `04:A2:1B:...`), com `tag_id`,
`tag_type`, `writable` e `records`, um registro NDEF por linha:

```text
text:Pedido 42
uri:https://aylis.dev/pedido/42
application/json:{"pedido":42}
```

`write(registros)` (o mesmo texto) grava na próxima tag que encostar, e o resultado vem no
`on_written` ou no `on_error` (só leitura, não cabe, a tag saiu antes). O formato NDEF
(ler e montar a mensagem, a memória das tags do tipo 2) é do núcleo (`aylis_core::ndef`), em
Rust puro. Por plataforma: o `NfcAdapter` em modo leitor no Android (a publicação põe a
permissão `NFC`); um leitor USB pelo PC/SC no Windows, no Linux e no macOS (ACR122U e
parecidos, com as tags do tipo 2: NTAG e Mifare Ultralight; a feature `nfc`, ligada pela IDE
quando um form usa o `Nfc`, e no Linux a `libpcsclite-dev` para compilar); a Web NFC no
navegador (só o Chrome do Android, em `https`). No iOS ainda não.

**Bluetooth** (`Bluetooth`, não visual). BLE e o clássico num componente só:

- `start_scan` procura (filtrando por `service_filter`, uuids separados por `;`, e por parte
  do nome em `name_filter`); cada aparelho chega uma vez no `on_found` (`id;nome;rssi`) e
  `devices` lista os achados;
- `connect(id)`: o `on_connected` traz o id e `services` lista as características
  (`serviço/característica read,notify` por linha);
- `read(id, serviço, característica)`, `write`/`write_hex` (com ou sem resposta) e
  `subscribe` (o `notify`/`indicate`): os valores chegam no `on_value`
  (`característica;hex`), com `last_value` (hex) e `last_text` (UTF-8);
- o clássico (SPP/RFCOMM, impressoras e leitores serial): `paired`, `open(id)`,
  `send`/`send_hex` e `close`; o que chega vem no `on_received`.

Os uuids aceitam a forma curta do Bluetooth SIG (`180D` é a frequência cardíaca). Por
plataforma: Android pelo `BluetoothGatt` e pelo `BluetoothSocket` (as permissões
`BLUETOOTH_SCAN` e `BLUETOOTH_CONNECT`, ou a localização antes do Android 12, pedidas na
primeira vez; a publicação declara); Windows, Linux e macOS pelo `btleplug`, só BLE (a
feature `bluetooth`; no Linux, a `libdbus-1-dev` para compilar); navegador pela Web
Bluetooth (só o Chrome): o `start_scan` abre o seletor de aparelhos do navegador, que exige um
clique do usuário antes, e os serviços usados depois de conectar têm de estar no
`service_filter`. No iOS ainda não.

### Câmera e microfone

Na paleta "Media": o `CameraView` mostra a câmera ao vivo, desenhada pelo Aylis (cantos
arredondados com `radius`, `mirror` espelha, `fit` ajusta), com `active` para ligar e
`camera` para escolher (vazio: a primeira; `aylis_media::cameras()` lista nome e id).
`CameraView::photo(ui, id, caminho)` salva o quadro atual em PNG e `CameraView::frame` dá o
quadro para análise. O `Microphone` mostra o nível do som numa barra, com `device`
(`capture::input_devices()` lista), `Microphone::level`, `record` e `stop_recording`
(WAV). Sem tela, `camera::open_camera` e `capture::AudioInput` leem direto.

| Sistema | Câmera | Microfone |
|---|---|---|
| Windows | Media Foundation | cpal (WASAPI) |
| Linux | GStreamer `v4l2src` | cpal (ALSA/Pulse) |
| Navegador | `getUserMedia` | — |
| Android | Camera2 do NDK | cpal (AAudio) |
| macOS, iOS | ainda não | cpal |

No Android, a primeira vez que o componente liga, o sistema pergunta ao usuário (a
`AylisActivity` faz os pedidos um de cada vez); a publicação só põe as permissões
`CAMERA` e `RECORD_AUDIO` no manifesto quando algum form usa `CameraView` ou
`Microphone` (ou um `AudioMixer` com entradas do sistema).

Exemplo: `cargo run -p aylis-media --example camera`.

### Saídas de áudio, loopback e mixer

- **Saídas.** O `AudioOutput` (não visual) lista as saídas do sistema: `devices` (nomes
  separados por `;`, para o `items` de um ComboBox), `default_device` e
  `AudioOutput::devices` (canais e taxa de cada uma). `on_devices_changed` avisa quando um
  fone ou uma caixa Bluetooth entra ou sai (confere a cada 2 s com `watch`).
- **Tocar em uma ou várias.** `output` no `MediaPlayer`, no `VideoPlayer` e no `Sound`:
  vazio é a padrão; várias, separadas por `|`, tocam o mesmo som sincronizado, cada uma com
  `volume` e `delay` em ms (`Alto-falantes|Fone BT;volume=0.8;delay=180`, para calibrar o
  atraso do Bluetooth). Cada saída tem a sua fila, e a taxa de cada uma é corrigida devagar
  (no máximo 0,5%) para acompanhar a primeira, porque o relógio de cada placa anda um pouco
  diferente. Trocar `output` tocando continua do mesmo ponto. No Android só a primeira da
  lista toca; no navegador, o `<video>` toca na saída padrão.
- **Sound.** Um som curto (efeito, alerta): `Sound::play` toca do começo e, chamado de novo,
  toca por cima, até `polyphony` toques ao mesmo tempo. O arquivo fica na memória depois do
  primeiro toque.
- **Loopback.** A lista de entradas (`capture::input_devices()`) traz também o som de cada
  saída, como `loopback:<saída>`: WASAPI no Windows e o agregado do CoreAudio no macOS
  14.4+. No Linux, o monitor de cada saída do PipeWire/Pulse já aparece como microfone. No
  Android (`AudioPlaybackCapture`, com autorização do usuário), ainda não.
- **Monitor.** `monitor` no `Microphone` manda a entrada para uma ou mais saídas, no mesmo
  formato do `output`. Cuidado com a microfonia: o microfone perto do alto-falante.
- **AudioMixer.** Não visual. `sources` são as fontes, separadas por `|`:
  - o nome de um `Microphone`, `MediaPlayer` ou `VideoPlayer` do form;
  - ou `device:<entrada>` (um microfone ou `device:loopback:<saída>`), aberta pelo próprio
    mixer.

  Cada fonte leva, se quiser, `;volume=0.5;pan=-0.3;mute`. Mudar só volume, panorâmica e
  mudo vale na hora, sem religar. A mistura (48 kHz, estéreo) vai para:
  - as saídas do `output` (vazio: nenhuma);
  - um WAV (`AudioMixer::record` e `stop_recording`);
  - o código (`keep_samples` e `AudioMixer::take_samples`, que guarda até 5 s).

  `AudioMixer::level` é o nível da mistura.

```rust
// A voz do microfone e o som do sistema, gravados juntos.
ui.set_prop(mixer, "sources", "micVoz;volume=1.2|device:loopback:Alto-falantes;volume=0.4")?;
ui.set_prop(mixer, "active", true)?;
aylis_media::AudioMixer::record(&mut ui, mixer)?;
// ...
aylis_media::AudioMixer::stop_recording(&ui, mixer, Path::new("conversa.wav"))?;
```

Nas outras linguagens: `Sound.play()`, `stop()` e `is_playing()`; `AudioMixer.record()`,
`stop_recording(arquivo)` e `level()`.

### Barra de tarefas e mensagens do Windows

O `Taskbar` (paleta "System", não visual) controla o botão do form na barra de tarefas do
Windows: `progress` (`normal`, `paused`, `error`, `indeterminate`) e `value` (0 a 100),
`badge` (o número sobre o ícone, como os apps de mensagem), `buttons` (até 7 botões na
miniatura que aparece ao passar o mouse: `icone.png|Dica;...`, o clique dispara
`on_button` com o índice) e `tasks` (o menu do clique direito: `Nome|argumentos;...`, que
abrem o próprio app com os argumentos).

Qualquer mensagem do Windows da janela de um form pode ser interceptada antes do
processamento normal:

```rust
use aylis::platform::win::{on_message, msg, MessageReply};
on_message(form, |m| {
    // Impede maximizar pelo menu da janela (ou pelo duplo clique na barra).
    if m.message == msg::WM_SYSCOMMAND && m.wparam & 0xFFF0 == msg::SC_MAXIMIZE {
        return MessageReply::Handled(0);
    }
    MessageReply::Default // ou Replace { message, wparam, lparam } para trocar
});
```

O gancho roda na thread da interface, sem o `Ui` na mão (para mexer na tela, use o
`Poster`). Em todos os sistemas, o `on_close_query` cancela o fechar (`ui.cancel_close()`).
O exemplo `taskbar` mostra as duas coisas.

`aylis::platform::win::hwnd(form)` devolve o `HWND` da janela do form (depois que ela existe),
para a API do Windows direto: o tema escuro da barra de título, o selo na barra de tarefas.

**Janela na bandeja.** `ui.window_command(form, WindowCommand::Hide)` esconde a janela sem
fechar o form (o app continua rodando, como um app que vai para a bandeja);
`WindowCommand::Show` traz de volta (desminimiza e dá o foco). Para o app que começa
escondido, `ui.set_start_hidden(form, true)` antes de rodar: a janela nasce invisível.
`WindowCommand::SetFullscreen(true)` põe em tela cheia (sem bordas) no monitor da janela.

**Rodar depois de um tempo.** `ui.set_timeout(ms, |ui| ...)` roda uma função uma vez,
depois de `ms` milissegundos, na thread da interface (como o `setTimeout` do navegador);
`ui.clear_timeout(id)` cancela. Vários vencidos rodam na ordem do vencimento; um criado
dentro de outro espera a volta seguinte do laço.

### Acessibilidade

Leitores de tela veem os controles do Aylis como controles do sistema: papel (botão, campo,
caixa de marcação...), nome (o texto, o `caption`, o `placeholder`), valor, estado e
posição; e conseguem focar, clicar e mudar valores. A árvore vem dos componentes (o papel
pela classe e pelas classes de base, então o `CellEditor` da grade é um campo de texto).

| Onde | Como | Leitor |
|---|---|---|
| Windows, macOS, Linux | AccessKit (`accesskit_winit`) | Narrador, NVDA, VoiceOver, Orca |
| Android | `accesskit_android`: nós virtuais na vista raiz da janela | TalkBack |
| iOS | AccessKit (`accesskit_winit`) | VoiceOver |
| Navegador | elementos HTML com ARIA, invisíveis, sobre o canvas | o do sistema, pelo navegador |

**Automação** (UI Automation no Windows, e o que a automação de cada sistema usa):
- **AutomationId:** o nome do componente (`btnSalvar`) vai como AutomationId (o `author_id`
  do AccessKit). FlaUI, pywinauto, WinAppDriver e Power Automate acham o controle por
  ele, sem depender do texto, que muda com o idioma.
- **Ações:** clicar (Invoke), focar e mudar o valor chegam ao controle sem evento de mouse
  na janela.
- **Itens desenhados:** os itens que o próprio controle desenha também são nós, com papel,
  nome, posição e estado (`Component::access_items` e `access_action`, em
  `aylis_core::automation`). Clicar ou selecionar um deles faz o mesmo que o mouse:

| Controle | Itens | Clicar / selecionar |
|---|---|---|
| `TabControl` | as abas (`Tab`) | troca a aba (`on_change`) |
| `ListBox`, `ComboBox` | as linhas (`ListItem`) | escolhe (`on_change`; o ComboBox sem abrir a lista) |
| `MainMenu` | os itens da barra (`MenuItem`) | abre o submenu |
| menu aberto (`PopupMenu`, submenus) | os itens, sem as divisórias (`MenuItem`, `MenuItemCheck`) | com submenu, abre; senão, ativa (`on_click`, `auto_check`, `on_item_click`) |
| `DataGrid` | cabeçalhos, linhas visíveis e as células delas (com linha e coluna) | cabeçalho ordena; linha ou célula vai ao registro |

**Testes e o inspetor:**
- No mesmo processo, sem janela: `ui.access_items(id)`, `ui.access_action(id, i, ...)` e
  `ui.click_item(id, "Clientes")`.
- Pelo canal de desenvolvimento (o inspetor da IDE, o MCP): os comandos `items` e `click_item`.
- Exemplo: `cargo run -p aylis-platform-winit --example automation`.

`AYLIS_ACCESSIBILITY=0` (no Android, `debug.aylis.accessibility`) desliga. Só custa algo
quando uma tecnologia assistiva está ligada (no navegador, a árvore é refeita no máximo 4
vezes por segundo, e só quando muda).

Tudo isso acontece numa thread só, a da interface. Trabalho demorado vai para outra thread
e volta pela fila (`ui.poster().post(...)`); veja [Threads e a interface](threads.md).

## Pintura

A cada quadro, o `Ui` percorre a árvore e, para cada controle visível:

1. resolve o **estilo** do tema para a classe, a variante (`style`) e o estado (hover,
   pressed...), com transição animada quando o tema pede;
2. desenha a sombra do estilo (se houver);
3. chama `paint(canvas, ctx)`. `ctx.style` traz as cores e medidas já resolvidas,
   `ctx.theme` os tokens, `ctx.rect` a área e `ctx.time` o relógio para animações;
4. pinta os filhos e depois chama `paint_after`.

O `Canvas` é um trait com retângulos, elipses, linhas, **caminhos** (`fill_path`,
`stroke_path`, com SVG `d`), sombras, texto com quebra e reticências, e imagens. O
`aylis-skia` e o `aylis-web` implementam o trait, e a quebra de linha e as reticências usam
a mesma regra (`aylis_core::text`) nos dois. A plataforma só repinta quando algo mudou ou
quando um controle pediu (`ctx.request_frame()` para animação contínua,
`ctx.request_frame_at(t)` para um horário).

**Quadros por segundo.** Uma animação contínua sai, no máximo, na taxa do monitor (o
renderizador de CPU não espera o vsync; sem esse teto, uma animação passava de 5.000
quadros por segundo à toa). `ui.set_max_fps(30.0)` baixa o teto para economizar bateria
(no celular, no notebook). `ui.fps()` diz quantos quadros saíram no último segundo, e o
componente `FpsMeter` mostra isso onde o app quiser (com `live`, ele se atualiza duas vezes
por segundo). O exemplo `fps` mostra os dois.

**Desenho guardado.** Um desenho caro e parado (um mapa com centenas de áreas, um gráfico
vetorial) pode ficar guardado como imagem: `canvas.begin_cached(chave, rect)` devolve
`true` quando a imagem guardada já foi desenhada; com `false`, o componente desenha e chama
`canvas.end_cached()`, que guarda e compõe. A chave muda com tudo o que muda o desenho
(cores, destaque, área); o tamanho em pixels já entra nela. O Direct2D guarda até 16; os
outros renderizadores desenham sempre. O `Svg` usa isso: o mapa de Minas Gerais (853
municípios, desenhado duas vezes no exemplo do IBGE) custava 34 ms a cada repintura da
janela e passou a 1,3 ms. O `Svg` também não desenha as formas fora da área visível (o
município com zoom) e divide o arquivo lido entre os `Svg` que mostram o mesmo conteúdo.

**Imagens prontas.** Os renderizadores guardam o que já fizeram de cada `Image` (decodificada,
enviada à placa de vídeo) para não refazer a cada pintura (`aylis_core::ImageCache`). Uma
entrada sai sozinha quando o app solta a imagem: o cache guarda uma referência fraca
(`Image::downgrade`), e um id só volta a ser pedido por quem ainda tem a imagem. Antes, uma
prévia trocada a cada meio segundo (480×270) deixava as antigas na placa até o teto: o NoOBS
ia de 154 a ~340 MB de memória comprometida; agora fica parado em 129 MB. Os limites contam
só para as imagens vivas e, passando deles, sai a usada há mais tempo (que é refeita quando
voltar: a tela não erra, só a pintura fica mais lenta). Ficam no `.aylisproj`, em Projeto →
Opções do projeto → Memória:

```json
"image_cache": { "pixels_mb": 96, "files": 256 }
```

- `pixels_mb` (8 a 4096): imagens em pixels na placa — quadros de vídeo e de câmera, prévias,
  os blocos de um `PaintCanvas`. Precisa caber o que aparece junto na tela (um desenho 4K
  inteiro visível ocupa ~33 MB).
- `files` (16 a 65.536): imagens de arquivo (PNG, JPEG, SVG...) decodificadas e na placa.
  Precisa caber as que aparecem juntas numa tela (ícones, miniaturas, uma galeria).

Pelo código: `aylis_core::image_cache::set_limits(ImageCacheLimits { .. })`. Valem para o
processo todo (Direct2D, Skia e o navegador).

**Painel de memória.** No app em depuração (compilado com `dev`, o que a IDE roda), um quadro
no canto de cima à direita do form principal mostra, a cada segundo:

- os quadros desenhados no último segundo (`ui.fps()`) e o teto do app, se houver
  (`set_max_fps`); com a tela parada, conta só a repintura do próprio painel (1);
- o processo: a memória comprometida (o "commit", privada) e o working set (no Linux e no
  Android, a memória anônima residente e a residente toda; no macOS, no iOS e no navegador,
  "—");
- o heap do Rust (o que o alocador de depuração conta);
- as imagens no cache: o total e cada cache com nome (`Direct2D · pixels na placa`,
  `Direct2D · arquivos na placa`, `Direct2D · arquivos decodificados`, `Skia · ...`,
  `Navegador · ...`), com quantas imagens e quanto ocupam.

Ctrl+Shift+M mostra e esconde. A publicação nunca o liga, e as capturas (`AYLIS_SNAPSHOT`)
saem sem ele. Num app próprio: `ui.set_memory_panel(Some(true))`. Os mesmos números das
imagens vão para o Profiler da IDE (Executar → Profiler), embaixo da memória.

### Zoom

Como o zoom do navegador: tudo fica maior ou menor (textos, controles, imagens, bordas) e o
layout se refaz no espaço da janela; o mouse, o toque, o cursor do texto e as vistas
nativas (`WebView`) acompanham.

- **O app inteiro:** `ui.set_zoom(1.25)`, ou no `.aylisproj`:
  `"zoom": { "value": 1.25, "keys": true }`. Com `keys` (ou `ui.set_zoom_keys(true)`), o
  usuário muda com Ctrl + `+`/`-`/`0` e Ctrl + roda, de 10 em 10%.
- **Um form:** a propriedade `zoom` do `Form` (1 = normal) multiplica a do app; um form
  com zoom 2 num app com 1,25 aparece com 2,5. Ao abrir, a janela já nasce do tamanho do
  form vezes o zoom.

O zoom vai de 0,25 a 5. `ui.form_zoom(form)` diz o zoom em que um form aparece. Exemplo:
`cargo run -p aylis-platform-winit --example zoom --features d2d`.

### Texto com efeitos (AnimatedText)

Como num programa de apresentação: o texto entra, sai ou fica se mexendo, por letra,
palavra ou linha. A propriedade `effects` lista os efeitos separados por `;`, cada um com o
nome e os parâmetros `chave=valor`:

```text
fade by=letter duration=500 stagger=45; slide dy=40 by=letter duration=700 stagger=45 easing=back
typewriter speed=55 cursor
fade by=letter duration=300 stagger=25 order=center; wave amplitude=6 period=1400 after
```

- **Efeitos:** `fade`, `slide` (`dx`, `dy`), `scale` (`from`), `rotate` (`from`, em graus),
  `typewriter` (`speed`, `cursor`), `color` (`from=#22c55e`), `tracking` (`from`, as letras
  chegam se juntando) e os contínuos `wave` (`amplitude`, `period`), `shake` e `blink`.
- **Comuns:** `by=letter|word|line|all`, `duration` e `delay` (ms), `stagger` (ms entre
  uma unidade e a próxima), `order=normal|reverse|center|random`,
  `easing=linear|in|out|in_out|back|bounce|elastic` e `out` (saída em vez de entrada).
- **Simultâneos e encadeados:** sem nada, todos começam juntos (cada um com o seu
  `delay`); com `after`, o efeito começa quando o anterior termina (mais o `delay`).
  Antes de começar, cada efeito segura o estado inicial dele sem esconder a letra: para
  aparecer deslizando, junte `fade` e `slide`.
- **Tocar:** `autoplay` (padrão), `looping` com `loop_delay`; `AnimatedText::play`,
  `stop`, `seek(ms)` e `is_playing` (nas outras linguagens, `frmX.txtTitulo.play()`); o
  `on_finish` avisa o fim. No designer aparece o estado final.

A fonte, a cor e o alinhamento são os do controle (`font_size`, `text_color`, `h_align`,
`v_align`, `wrap`). Exemplo:
`cargo run -p aylis-platform-winit --example text_effects --features d2d`.

### Lista com modelo de item (VirtualList)

Como o `RecyclerView` do Android: cada item é desenhado com
um modelo seu, e só existem as linhas que cabem na tela (mais duas). Ao rolar, as que
saem são reaproveitadas para os itens que entram, então um milhão de itens custa o
mesmo que vinte.

- O modelo é o primeiro filho da lista, desenhado no próprio designer (um painel com
  imagem, textos, botões), ou um form inteiro do projeto em `item_form`. Cada linha é uma
  cópia (`Ui::clone_component`), com os nomes de dentro repetidos sem conflito.
- `item_count` e `item_height` dizem quantos itens e a altura de cada um.
- `on_bind` chega com o índice: `VirtualList::item(ui, lista, i)` dá a linha, e
  `ui.find_in(linha, "lblNome")` os componentes dela.
- `on_load_more` avisa perto do fim (uma vez por quantidade): o handler busca a próxima
  página e aumenta `item_count`. É a lista infinita.
- `bindings` (`"lblNome=nome;imgFoto=foto"`) preenche os componentes sozinho a partir de
  uma fonte (`RowSource`). A `DataRepeater` (paleta "Data Controls") já vem ligada a um
  `DataSource`: um item por registro, e ela acompanha o dataset.
- `VirtualList::scroll_to` e `refresh` (dados mudaram) completam.
- **Tamanho por item:** `sizes` (`"80;120;80"`; os que faltam usam `item_height`) ou
  `VirtualList::set_sizer` (uma função do índice). A lista guarda as somas e acha o
  primeiro item à vista por busca binária.
- **Horizontal e em grade:** `orientation` = `horizontal` (rola de lado, `item_width`) e
  `columns` > 1 (uma grade: `columns` itens por linha; na horizontal, por coluna).

Exemplo: `cargo run -p aylis-platform-winit --example infinite_list --features cpu`.

### BI e trader

Paleta "BI": `KpiCard` (o número grande, a variação contra `previous` em verde ou
vermelho conforme `higher_is_better`, e a tendência), `PivotTable` (`data` com cabeçalho e
campos separados por `;`, agrupada por `rows_field` × `columns_field` com `aggregate`;
`on_cell_click` para detalhar), `Slicer` (botões de filtro; `Slicer::accepts`) e
`ChoroplethMap` (regiões de um GeoJSON coloridas por `values`).

**O `Chart`** tem os tipos do Chart.js e mais alguns: linha, barra, barra horizontal, área,
pizza, rosca, área polar, dispersão, bolhas, radar, barras, linhas e áreas empilhadas,
misto (cada série com o seu tipo), cascata, funil, treemap, mapa de calor, sparkline,
Sankey e waffle.
- **Sankey** (`kind: sankey`): cada rótulo é um fluxo `origem>destino` e a 1ª série traz o
  valor de cada um (`labels: "Salário>Receita;Receita>Poupança"`, `series:
  "Fluxo=5000,2500"`). Os nós ficam em colunas pelo caminho desde as origens, e a largura da
  faixa é o valor.
- **Waffle** (`kind: waffle`): 10x10 quadrados, cada um 1% do total da 1ª série, uma cor por
  rótulo (as sobras do arredondamento vão para os itens de maior resto, para somar 100).
- **Dados:** `series`, uma série por linha:
  `Nome[bar;right;#e11d48;width 3;dash 6 3;triangle;fill origin]=10|20|x:y|x:y:r`. As
  opções entre colchetes são o tipo da série, o eixo Y da direita, a cor, a espessura, o
  tracejado, o estilo do ponto e o preenchimento (até a origem ou outra série). Também
  aceita o JSON do Chart.js (`{"labels": [...], "datasets": [...]}`).
- **Escalas:** `y_min`/`y_max` (ou `auto_scale`), `log_scale`, `reverse`, `time_axis`
  (rótulos com datas viram um eixo de tempo), o eixo da direita.
- **Linhas:** `stepped` (degraus) e `tension` (suavizada, monotônica: não passa dos pontos).
- **Interação:** clicar na legenda esconde ou mostra a série; dica com o valor em todos os
  tipos; `on_click` com `"série;índice"` (`Chart::clicked_series`, `clicked_index`); com
  `zoom`, a roda aproxima, arrastar anda pelo eixo X e o duplo clique volta ao todo; os
  dados novos entram com animação.
- **DBChart** (paleta "Data Controls"): um `DataSource`, o campo dos rótulos
  (`label_field`) e os campos de valor (`value_fields` = `"Vendas;Custos"`), como o
  `TDBChart`.
- Exemplo: `cargo run -p aylis-platform-winit --example charts --features cpu`.

**Relatório de bandas** (paleta "Reports"), como o QuickReport/FastReport:
- `Report` é a página (`page_width`, `page_height`, `landscape`, `margin`; A4 por padrão)
  ligada a um `DataSource`; dentro dele, `ReportBand`s com `kind` = `title`,
  `page_header`, `group_header` (com `group_field`), `detail` (uma por registro),
  `group_footer`, `summary` e `page_footer`.
- Mestre-detalhe: `sub_header`, `sub_detail` e `sub_footer` com o `data_source` do detalhe
  (ligado ao mestre pelo `master_source`). Depois de cada registro do mestre saem os
  registros do detalhe dele (o cursor do mestre anda e volta no fim); os totais do
  `sub_footer` são do detalhe daquele registro, e um campo que o detalhe não tem vem do
  mestre (`[numero]`, `[valor + numero]`). Mais de um `sub_detail` (itens e parcelas) sai
  um depois do outro; o cabeçalho e o rodapé ficam com o do mesmo `data_source`. Detalhe
  sem registros não imprime cabeçalho nem rodapé.
- Nas bandas vão Labels, imagens, linhas, formas e gráficos, desenhados no designer como num
  form. O texto aceita, entre colchetes, um campo (`[nome]`), uma expressão
  (`[preco * qtd]`), totais (`[SUM(valor)]`, `COUNT`, `AVG`, `MIN`, `MAX`: no rodapé de
  grupo, do grupo; no sumário, de tudo; no rodapé de página, da página), `[PAGE]`,
  `[PAGES]`, `[DATE]`, `[TIME]` e `[LINE]` (o número do registro).
- O `Report` e as bandas só aparecem no designer (`design_only`); em execução, o
  `ReportPreview` mostra as páginas (zoom, primeira, anterior, próxima, última, imprimir,
  PDF).
- Pelo código: `Report::prepare` monta as páginas (`aylis_core::pages::PageSet`),
  `Report::to_pdf` grava o PDF (escritor próprio, sem dependências) e `Report::print` usa
  a impressora do sistema (o diálogo de impressão do Windows; nos outros, o PDF aberto no
  visualizador).
- Exemplo: `cargo run -p aylis-platform-winit --example report --features cpu`. O projeto
  `examples/Relatorios` tem dois relatórios mestre-detalhe (pedidos com itens; vendas por
  vendedor com itens e parcelas).

Paleta "Trader": `CandleChart` (`data` com `tempo;abertura;máxima;mínima;fechamento;volume`
ou `set_candles`/`push`; `indicators` = `"sma:9;ema:21;bollinger:20"`; `lower` = volume,
rsi ou macd; `volume_profile`; roda = zoom, arrastar = tempo, `draw_mode` = trend ou
fibonacci para desenhar), `OrderBook` e `DepthChart` (`bids`/`asks` = `"preço:qtd;..."`),
`TickerTape`, `Watchlist` (`Watchlist::update` faz a célula piscar) e `TimeAndSales`
(`TimeAndSales::add`). Os indicadores (`sma`, `ema`, `bollinger`, `rsi`, `macd`) também
servem direto pelo código. Os números seguem o idioma do app.

Exemplo: `cargo run -p aylis-platform-winit --example dashboard --features cpu`.

### Formas geométricas

Na paleta "Shapes": `Rectangle`, `Ellipse`, `Circle`, `Line`
(`direction`: down, up, horizontal, vertical), `Polygon` (`sides`, ou `points` como
`"0.5,0 1,1 0,1"` em frações do controle), `Star` (`points`, `inner`), `Pie` e `Arc`
(`start_angle`/`end_angle` em graus, 0 em cima e no sentido do relógio) e `Path` (`data`
com o `d` do SVG, esticado para o controle; `keep_aspect` não distorce). Todas têm
`fill`, `stroke` e `stroke_width`, giram com `rotation` e respondem a clique, hover e
animações como qualquer controle.

O clique acerta a figura, não o retângulo em volta: o canto vazio de um círculo deixa o
clique passar para o que está embaixo, e uma linha ou um arco respondem perto do traço.
Qualquer componente pode fazer o mesmo implementando `Component::contains_point`; a
regra de preenchimento é a do desenho (`Path::contains`).

Exemplo: `cargo run -p aylis-platform-winit --example shapes --features cpu`.

### Ícones

O Aylis traz um conjunto próprio de ícones vetoriais: traço arredondado numa grade de
24×24, mais de 80 nomes como `play`, `mic`, `settings`, `search`, `trash`, `folder`,
`chevron-down`... (`aylis_core::icons::builtin_names()` lista todos). Eles mudam de cor com o
tema. O app pode acrescentar os seus:

```rust
ui.register_icon_svg("logo", "M4 4h16v16H4Z", "", 24.0)?;   // traço, preenchimento, grade
```

`Button.icon`, `Edit.icon`, `Badge.icon`, `Icon.icon`, `SegmentedControl.icons` e
`ctx.draw_icon(canvas, "nome", rect, cor)` usam esses nomes.

## Temas, subtipos e mistura de temas

Um `Theme` tem:

- **tokens**: cores (`primary`, `surface`, `text`, `danger_soft`, `meter_low`...), fontes,
  raios, espaçamentos, durações e sombras (`sm`, `md`, `lg`);
- **estilos por classe** (`"Button"`), com ajustes por estado (`hover`, `pressed`,
  `focused`, `checked`, `disabled`);
- **variantes** (`"Button.danger"`), escolhidas no controle pela propriedade `style`.

A ordem de prioridade: valor da instância > variante > estilo da classe
> estilo da classe base > tokens. "Aylis Light" e "Aylis Dark" são os temas nativos, com
cores próprias do Aylis.

Também há temas que imitam as plataformas (Material 2 e 3, iOS 7, 13 e 26, Windows 7, 10
e 11), que mudam até a forma dos controles: veja [Temas de plataforma](temas.md).

### Abas do navegador (TabStrip)

O `TabStrip` (paleta Layouts) é a faixa de abas de um navegador, como a do Edge: a aba
ativa tem os cantos de cima arredondados e se junta ao conteúdo de baixo; cada aba tem
ícone, título e o × (na ativa e na que está sob o mouse); o "+" no fim pede uma nova; as
abas encolhem para caber; arrastar troca a ordem; o clique do meio fecha. É só a faixa: o
`on_change` diz qual conteúdo mostrar. No layout, `tabs: "Início|home;Relatório"` (título
e ícone). Eventos: `on_change` (o índice), `on_close` (o índice; com `auto_close` a aba
sai sozinha, sem ele o código decide e chama `remove`), `on_add` e `on_reorder`
(`"de,para"`). Por código: `add`, `remove`, `set_title`, `set_modified` (o ponto de não
salvo no lugar do ×) e `move_tab`.

### Gravadores de tela e de áudio

Para apps de gravação (o NoOBS usa todos):

- **`RecordButton`** (paleta Media): o botão redondo de gravar. Vermelho com o ponto
  branco; `recording` vira o quadrado de parar com o brilho pulsando; `paused` para o
  pulso; `finalizing` gira um arco verde em volta e ignora o clique (o arquivo ainda está
  fechando); `countdown` > 0 mostra os segundos no meio e um anel âmbar que esvazia
  (`countdown_fraction` de 1 a 0); desabilitado fica cinza; `glow: false` tira o brilho
  (com uma camada translúcida por cima). `on_click` no clique.
- **`ScreenLayout`** (Media): as telas na posição real, escaladas para caber, cada uma com
  a prévia ao vivo e a marca no canto; o clique marca/desmarca e dispara `on_change` com
  o id. Por código: `set_screens(vec![ScreenItem { .. }])` (monitores e câmeras, `camera`
  pinta o fundo azul com o ícone), `set_image(id, imagem)`, `is_enabled(id)`; `locked`
  trava (durante a gravação).
- **`LevelMeter`**: `stereo` desenha duas barras (a de baixo com `value_right`);
  `gradient` pinta o degradê pela posição (verde, âmbar perto de `high`, vermelho no fim)
  que aparece só até o nível; `muted` enche a barra de `meter_high`; `dim` apaga as cores.
  O subtipo "Estéreo com degradê" já vem assim. `smoothing` (ms) faz a barra deslizar
  até o nível novo (níveis que chegam a cada 100 ms ficam contínuos com uns 90).
- **Avisos empilhados**: `notify(ui, título, mensagem, ToastKind::Warning, ms)` mostra um
  cartão no canto de cima à direita do form (título em negrito, a mensagem embaixo, a cor
  do tipo: `Info`, `Success`, `Warning`, `Error`). Entra deslizando, some no prazo ou no
  clique; um aviso com o mesmo título de um que está na tela só renova o prazo. O
  `toast(ui, texto)` continua sendo o aviso curto de rodapé.

- **`MediaCard`** (Media): o cartão de um vídeo, foto ou pasta numa biblioteca. A
  miniatura (`set_image`) com a `duration` no canto, o `title`, o `subtitle` e os `badges`
  (`;` entre eles, `*` destaca, `icon:nome` é um ícone); `compact` é a linha de uma pasta
  (ícone, título e subtítulo). A caixa do canto dispara `on_select` (`selected`, `order`
  mostra o número da ordem, `selection_mode` deixa a caixa sempre à vista); o resto,
  `on_click`; o botão direito, `on_context`. `dimmed` apaga (recortado), `progress` ≥ 0
  desenha a barra com o `progress_text`. As medidas são as de uma biblioteca web: nome
  em 12 px, tamanho em 11, selos de 15 px (fonte 9) e o tamanho nunca encolhe por causa
  deles.
- **`Waveform`**: `seek_bar` vira a barra de busca de um player (trilho de 4 px, polegar
  de 14 com halo sob o mouse e a onda discreta embaixo); `paused` pinta trilho e polegar
  na cor do texto em vez do destaque.
- **`confirm(ui, título, texto, "Excluir", true, |ui, sim| ...)`**: a pergunta com os
  textos do app; o último argumento antes da função pinta o botão de vermelho.

- **`FrameView`** (Media): o palco de um player que decodifica por conta própria: fundo
  preto e o último quadro entregue com `set_frame(Some((imagem, retângulo)))`, no lugar
  dito; `set_shift(dx, dy)` desloca enquanto o próximo não chega. Os gestos chegam pelos
  eventos comuns (`on_wheel`, `on_pointer_down`, `on_dbl_click`). O cursor `none`
  (`CursorShape::None`) esconde o mouse (o vídeo tocando com o mouse parado). Entregue o
  quadro como `Image::bgra` (opaco ou pré-multiplicado): o Direct2D o usa sem converter
  pixel a pixel e guarda os últimos quadros, então repintar a tela não reenvia o vídeo.
  `Image::rgba` passa pela conversão (cara num 4K a cada quadro).
- **`CropBox`** (Media): o recorte por cima de uma imagem ou vídeo.
  A seleção (`sel_x`, `sel_y`, `sel_width`, `sel_height`) fica em pixels da origem
  (`source_width` × `source_height`), com `min_size` e `snap` (2 = pares); fora dela
  escurece com `shade`. Miolo move, cantos e bordas redimensionam; `on_change` no
  arrasto, `on_end` ao soltar, `on_tap` num clique parado (o `on_click` comum vem em
  todo clique, até depois de arrastar). `framed` é o visual de editor web: terços sempre
  à vista, alças de 8 px brancas por fora da moldura e o tamanho em cima, à esquerda
  (deixe uns 10 px de folga em volta).

**Controles deslizantes e menus.** O `TrackBar` tem `thumb_size`, `track_size` e
`solid_thumb` (o `input[type=range]` da web: polegar cheio e trilho de ponta a ponta) e
`limit`: acima dele o trilho e o polegar ficam na cor de perigo, com uma marca no ponto
(volume amplificado acima de 100 %). O `PopupMenu` segue o estilo `PopupMenu` do tema
quando ele existe (fundo, borda, raio, fonte, a cor sob o mouse e o recuo do texto em
`padding`; sem ícones no painel, o texto começa no recuo); o `MenuItem` com `danger` sai
na cor `danger_text` (ou `danger`) e, sob o mouse, em fundo `danger`.

**Texto com marcas e contorno.** O `Label` com `markup` interpreta `<b>`, `<code>` e
`<br>` no texto; `outline` + `outline_width` desenham o contorno das letras (a legenda
sobre vídeo).

**Degradês.** O `Rectangle` (e o `Panel`) faz degradê de `fill` até `fill2`, com uma cor
do meio opcional (`fill_mid` em `fill_mid_at`, de 0 a 1) e `gradient_radial` (do centro
para a borda).

**Títulos espaçados.** O `Label` tem `letter_spacing` (pixels a mais entre as letras) e
`text_case` (`upper`, `lower`): os títulos de seção em caixa-alta espaçada, sem mudar o
texto guardado.

### Fonte e cores próprias

**Sem borda.** Os componentes de texto (`Edit`, `Memo`, `CodeEditor` e os que partem
deles: `NumberBox`, `AutoComplete`, os campos de data) têm a
propriedade `border`: desligada, some o contorno e o traço do foco (o campo dentro de um
painel ou de uma tabela).

**A fonte de qualquer controle**, como o `Font` do `TControl`, fica no `ControlBase`
(`FontOverride`):
- `font_family`, `font_size` e `font_weight` (`default`, `thin` ... `bold`, `black`).
- `font_italic`, `font_underline` e `font_strikeout`.
- **Herança, como o `ParentFont`:** o que não tem valor vem do pai, então a fonte posta no
  form vale para tudo dentro dele. A pintura passa a fonte combinada para os filhos.
- **No Object Inspector:** ficam na categoria Font. O "…" do `font_family` abre o editor,
  com as fontes do projeto primeiro, depois as instaladas no sistema
  (`aylis_skia::system_font_families`), o tamanho, o peso, os estilos e uma prévia.
- **Sublinhado e tachado:** o Skia faz sozinho (decoração do parágrafo); o Direct2D e a
  web traçam por linha (`aylis_core::decorate_line`).

**Todo controle aceita cores próprias**, por cima das do tema (vazio = a do tema), na
categoria Colors do Object Inspector (`ColorOverride` no `ControlBase`):
- `fill` (o fundo), `border_color` (a borda) e `text_color` (o texto e o ícone);
- `accent_color` (a cor de destaque: a marca do CheckBox, a barra do ProgressBar, o
  trilho do Switch, a seleção);
- `success_color`, `warning_color` e `danger_color` (os estados: o Gauge, o Banner, o
  Rating, os botões de Perigo e Sucesso).
- A pintura recolore os tokens do tema só para aquele controle (hover, pressionado, as
  versões claras e o texto por cima são derivados da cor), e guarda o tema recolorido.
  Os filhos não herdam: a cor é do controle.
- O desenho da plataforma, como o degradê do Windows 7, dá lugar à cor.
- **Num componente novo:** pinte com `ctx.colors` e `ctx.font(tamanho)` (ou
  `ctx.monospace_font`) em vez do tema direto, e as propriedades valem sozinhas. Quem não
  pinta fundo (um Label, um ícone) marca `transparent_background()`, e o núcleo pinta o
  `fill` atrás dele.

### Subtipos na paleta

Cada variante é um **subtipo** do componente. Na paleta da IDE, a seta ao lado de
`Button` abre a lista: Secundário, Contorno, Plano, Perigo, Sucesso, Link, Pílula, Ícone e
Flutuante (FAB). Escolher um subtipo cria o componente com `style` = variante e com os
valores iniciais do subtipo: o "Ícone" já vem quadrado, sem texto e com ícone.

- o app registra os seus com `ui.register_variant("Edit", "cpf", "CPF", vec![...])`;
- uma variante criada só no tema JSON (`"styles": { "Button.minha": {...} }`) aparece
  sozinha como subtipo;
- pelo nome, `ui.create("Button.icon", Some(pai))` cria o subtipo (o JSON e o protocolo
  remoto também aceitam).

### Trocar e misturar temas

- **Trocar tudo:** `ui.set_theme(Theme::dark())`. Todos os controles repintam com o tema
  novo.
- **Tema por controle:** a propriedade `theme` (nome de um tema da biblioteca) vale para o
  controle e para os filhos, como uma barra lateral escura num form claro:
  `ui.set_prop(painel, "theme", "Aylis Dark")`.
- **Tema por classe:** `ui.theme_mut().mix("Button", "Marca")` faz todos os botões usarem o
  visual do tema "Marca", com as variantes e as classes derivadas. No JSON do tema:
  `"components": { "Button": "Marca" }`.
- **Biblioteca:** `ui.register_theme(tema)` acrescenta um tema que pode ser citado pelo
  nome; `ui.theme_names()` lista os disponíveis; `ui.theme_of(id)` diz com qual tema um
  controle é pintado.

### Transições de página e de form

`transition` (`none`, `fade`, `slide_left`, `slide_right`, `slide_up`, `slide_down`) diz como
uma página entra enquanto a outra sai, desenhadas ao mesmo tempo:

- **`TabControl.transition`:** ao trocar de aba. Deslizando, a direção segue a ordem das
  abas (avançar entra pela direita, voltar pela esquerda).
- **`Form.transition`:** no celular e no navegador, onde os forms se empilham numa tela só.
  `show` entra com ela, `close` (e o "voltar") sai com a inversa e só então fecha. No
  desktop cada form é uma janela e isto não vale.
- **Por código:** `ui.transition(Some(de), Some(para), Transition::Fade, 250.0)` troca
  quaisquer dois controles (o de saída fica invisível no fim).

Por baixo, a transição anima `opacity` e as propriedades novas `translate_x`/`translate_y`,
que deslocam só o desenho, sem mexer no layout; elas
também servem para animações próprias (`ui.animate(id, "translate_y", 40.0, ...)`).

### Teclado e colunas do DataGrid

Com as colunas mais largas que a grade, aparece a barra de rolagem horizontal embaixo
(arrastar o polegar ou clicar no trilho; a roda com Shift também rola). Ctrl+C (ou
Ctrl+Insert) copia o texto da célula atual, formatado como aparece na grade. Arrastar o
separador do cabeçalho muda a largura da coluna; o duplo clique nele ajusta a largura ao
maior texto da coluna (o título e os registros a partir da primeira linha na tela, até
1.000: com milhões de registros, medir todos travaria). Por código: `fit_column`.
Com `column_move` (ligado por padrão), arrastar o título muda a ordem das colunas: o título
segue o mouse e uma barra marca onde a coluna entra; as colunas fixas (`fixed_columns`)
trocam só entre si. A nova ordem chega no `on_column_moved`, no formato de `columns` (com as
larguras), para o app guardar; por código, `DataGrid::move_column_to(ui, grade, de, para)`.
Com `sortable`, o clique no título (ao soltar, sem arrastar) ordena pela coluna; o segundo inverte; o terceiro volta à
ordem do dataset (`index_field_names` ou `index_name`). Ctrl+clique acrescenta a coluna à
ordem, depois das que já estão (de novo inverte, na terceira sai); com mais de uma, o
cabeçalho mostra o número da posição ao lado da seta.
A célula atual tem a borda na cor de destaque do tema (`primary`); a propriedade
`cursor_color` troca essa cor.

### Colunas do DataGrid

As colunas vêm, nesta ordem de preferência:

- dos componentes `Column` filhos da grade (como as colunas persistentes): cada um com
  `field_name`, `title`, `width`, `alignment`, `color`, `text_color`, `visible` e
  `read_only`; o que fica vazio vem do `Field` (o rótulo, a largura, o alinhamento);
- da lista `columns`, `"sigla:44;nome:170;regiao"` (a largura depois de `:` é opcional);
- dos campos visíveis do dataset.

No designer a grade já mostra as colunas, mesmo com o dataset fechado. O duplo clique no
DataGrid (ou "Editor de colunas…" no Object Inspector) abre a janela de colunas: a lista na
ordem; clicar numa leva as propriedades dela ao Object Inspector; "+ Adicionar" cria a do
próximo campo, "Todos os campos" cria uma para cada campo visível (a lista `columns`, se
houver, vira as primeiras colunas, com as larguras); Excluir, Subir e Descer. Arrastar o
separador do cabeçalho no designer muda a largura (na `Column` ou na lista `columns`), com
Ctrl+Z.

### Pintura personalizada do DataGrid

`DataGrid::on_draw_cell(ui, grid, |cell| ...)` desenha a célula por código: para cada
linha visível, a função é chamada uma vez para a linha inteira (`cell.column == None`) e uma
vez por coluna, com o índice da linha (`cell.row`), os valores de todos os campos
(`cell.get("status")`, ou `cell.json()` para um objeto JSON), o valor e o texto da célula e
se a linha é a atual ou está selecionada. Devolve um `CellStyle`: fundo, cor do texto,
negrito, itálico, texto trocado, ou `paint` (uma função que pinta a célula à mão, com o
canvas e o retângulo dela: barras, ícones, o que for). A chamada acontece quando as linhas
são preparadas (não a cada quadro); `DataGrid::refresh_rows` refaz se a regra depender de algo
fora do dataset.

## Dados

A camada de dados não sabe nada de banco: os registros chegam do backend
(`load_rows`, `load_json`), o usuário edita, e as alterações voltam como **delta**
(`delta()`, `apply_updates()`). O backend responde registro a registro (`reconcile`). Os
controles DB* ligam-se a um `DataSource` e a um campo (`data_source`, `data_field`). Veja
[Em tempo de execução](tempo-de-execucao.md#dados).

**Milhões de registros.** O `ClientDataSet` guarda os registros em colunas: inteiros em 4
bytes enquanto couberem (depois 8), números em 8, os nulos num mapa de bits, e os textos num
dicionário (cada texto diferente uma vez; o registro guarda um número de 4 bytes). As colunas
crescem em blocos de 64 mil posições, sem realocar nem copiar o que já está guardado. O
`DataGrid` só lê as linhas na tela. Ordenar calcula a chave de cada registro uma vez (um texto
vira a posição dele no dicionário) e divide o trabalho entre os núcleos; o filtro também,
quando só usa campos de dados e não há `on_filter_record`. Medido com 10 milhões de
registros e 100 colunas (50 de texto, 50 de números), em release, numa máquina de 16
núcleos (`cargo test --release -p aylis-dbcontrols --test huge_grid -- --ignored`):

| Operação | Tempo |
|---|---|
| carga de uma vez (`RowBatch`, montada em 16 threads) | 10,7 s (montar 8,7 s, juntar 0,4 s, carregar com a ordem 1,5 s) |
| carga em pacotes de 50 mil (`append_rows`) | 120 s no total; 0,5 s o pacote mais lento |
| memória dos dados | 5,8 GB |
| pintar, ir ao último, Page Down, roda | menos de 5 ms cada |
| ordenar por texto ou por número | 1,5 s |
| filtrar (`n0 > 30000`) | 0,5 s |
| localizar o último pelo código | 0,1 s |

**Carga de uma vez** (`RowBatch`). Os registros são montados fora da interface, já nas
colunas do dataset: `lote.int(1).text("Recife").float(2.5).end_row()` (um texto que já está
no dicionário não é copiado). Vários lotes montados em paralelo se juntam com `append` (com
múltiplos de `RowBatch::PART` registros, os blocos passam de um lote para o outro sem
copiar) e entram com `data_set(ui, cds).load_batch(lote)`: uma troca e uma só reconstrução
da ordem e do filtro, sem conferir valor por valor (os valores já vêm no tipo de cada
campo). Em pacotes, cada `append_rows` refaz a ordem de tudo o que já está carregado. O
exemplo `GradeGigante` monta uma parte por núcleo, mostra o andamento e carrega de uma vez.

## Projeto, IDE e forms

Um projeto é uma pasta:

```
Vendas/
  Vendas.aylisproj      nome, form principal, tema, idioma, opções de publicação
  forms/*.json         os forms (como os .dfm)
  themes/*.json        temas
  assets/              imagens, sons, vídeos, fontes
  locales/*.json       traduções
  code-sync.json       o que cada linguagem já tem (veja abaixo)
  rust/                o código em Rust: Cargo.toml, build.rs, src/lib.rs, src/forms/<form>.rs
  python/ ... pascal/  o código nas outras linguagens que o projeto tiver
```

O visual (forms, temas, assets, traduções) é um só; o código dos eventos fica numa pasta
por linguagem. A linguagem ativa é a do `.aylisproj` (Projeto → Opções → Linguagem): F9,
depurar e publicar usam a pasta dela, e as outras ficam intactas. Trocar para uma linguagem
nova cria a pasta com os handlers vazios (a lógica não é convertida); voltar para uma que já
existe traz o código dela de volta.

**Sincronia entre as linguagens** (`aylis_project::sync`):
- "Gerar código" acrescenta os handlers novos do form, vazios, em **todas** as pastas, e um
  handler renomeado no form (o mesmo componente e evento) é renomeado no código de todas,
  com o corpo. Um handler que saiu do form fica no código, marcado como sem uso.
- A lógica de cada handler tem uma revisão: mudou numa linguagem em dia, as outras ficam
  pendentes até o corpo delas mudar também (foi portado) ou alguém marcar como portado.
- Projeto → Pendências entre linguagens lista o que falta na linguagem ativa (abre sozinha
  ao trocar de linguagem), com "Lado a lado" (o código das duas) e "Portado". Também
  aparecem os componentes que saíram do form e o código ainda cita (`"edtNome"`).
- O estado fica em `code-sync.json` (vai para o controle de versão com o projeto).
- Projetos do layout antigo (o código na raiz) são movidos para a pasta da linguagem na
  primeira geração.
- `aylis-tool run`/`script` aceitam `--language go` para rodar outra pasta sem mudar a ativa.

**Acesso tipado aos componentes** (`aylis_project::bindings`): cada form é
uma instância global com o nome dele (`frmCalculadora`, como o `Form1`) e um campo por
componente, com as propriedades no tipo certo. O "Gerar código" escreve, em cada linguagem,
o arquivo `components` só com as classes que os forms usam; o registro dos forms liga o
cliente e o arquivo de eventos de cada form ganha o `import`/`uses` da instância.

| Linguagem | Ler e gravar |
|---|---|
| Free Pascal | `frmCalculadora.pnlLateral.Visible := not frmCalculadora.pnlLateral.Visible;` |
| C# | `frmCalculadora.pnlLateral.Visible = !frmCalculadora.pnlLateral.Visible;` |
| Python | `frmCalculadora.pnlLateral.visible = not frmCalculadora.pnlLateral.visible` |
| TypeScript | `frmCalculadora.pnlLateral.visible = !(await frmCalculadora.pnlLateral.visible);` |
| Java | `frmCalculadora.pnlLateral.setVisible(!frmCalculadora.pnlLateral.isVisible());` |
| Go | `frmCalculadora.pnlLateral.SetVisible(!frmCalculadora.pnlLateral.Visible())` |
| C++ | `frmCalculadora.pnlLateral.set_visible(!frmCalculadora.pnlLateral.visible());` |
| Rust | `frmCalculadora.pnlLateral.set_visible(ui, !frmCalculadora.pnlLateral.visible(ui));` |

- As classes e propriedades vêm de `resources/components.json` (o catálogo do registro,
  `aylis_json::catalog`); mudou uma classe, `aylis-tool update-catalog` regrava (o teste
  `cargo test -p aylis-remote --test catalog` confere que está em dia).
- No Rust, dentro de um handler os componentes são `self.btnSalvar`; fora dele (um módulo
  do app, um timer, a bandeja), `FrmX::components(ui)` dá uma cópia deles
  (`if let Some(f) = FrmPedido::components(ui) { f.lblTotal.set_text(ui, "…") }`), que vale
  até enquanto um handler do próprio form roda — quando o `FrmX::get(ui)`, que traz também o
  estado (`data`), dá `None` porque a instância está emprestada ao handler.
- Peça repetida (uma linha de lista, um cartão) é um frame no JSON, não código que monta
  componentes: `let linha = FraLinha::create_in(ui, lyLista)?;` cria mais uma instância dentro
  do pai, com nome livre (`fraLinha`, `fraLinha2`...) e os handlers do `fraLinha`, e devolve os
  componentes tipados (`linha.lblNome.set_text(ui, "…")`); `FraLinha::components_of(ui, raiz)`
  acha os de uma instância.
- Ler é um `get` na tela (espera a resposta; no TypeScript, `await`); gravar é um `set`.
  Cores, enums e referências são texto (`"#0067C0"`, `"client"`, o nome do componente).
- Métodos também são tipados: os do componente (`Component::methods`, como os do
  `PaintCanvas`: `pcDesenho.Undo`, `pcDesenho.Save(Caminho)`), o `Close`/`Show` do form, o
  `Execute` dos diálogos e o `Play`/`Pause`/`Stop`/`Seek`/`Position`/`Duration` da mídia.
  No catálogo eles entram com os parênteses: `"save(string)": "invoke:bool"`.
- Nome que só se sabe na hora (o componente pelo nome, ou quem disparou o evento): `FindButton(Nome)`
  e `SenderAsButton(Event)` (um par por classe usada no form):

| Linguagem | O botão que disparou o evento |
|---|---|
| Free Pascal | `Texto := frmCalculadora.SenderAsButton(Event).Text;` |
| C# | `frmCalculadora.SenderAsButton(ev).Text` |
| Python | `frmCalculadora.sender_as_button(event).text` |
| TypeScript | `await frmCalculadora.senderAsButton(ev).text` |
| Java | `frmCalculadora.senderAsButton(ev).getText()` |
| Go | `frmCalculadora.SenderAsButton(ev).Text()` |
| C++ | `frmCalculadora.sender_as_button(ev).text()` |
| Rust | `frmCalculadora.sender_as_button(ui, sender)` (um `Option`) |

- O `Aylis.Call('get', ...)` de antes continua funcionando.

A IDE edita os forms (paleta, designer, Object Inspector), gera os handlers de evento,
compila e executa (F9) e publica para cada plataforma. Os handlers ficam ligados pelo
nome: o form diz `"on_click": "btn_salvar_click"` e o código registra a função com esse
nome. O app também pode ser feito em Python, comandando o Aylis pelo protocolo remoto.

Além de Rust e Python, o projeto pode ser em TypeScript (Node 22.6+ ou Bun), C#, Java,
Go, C++ ou Free Pascal (Projeto → Opções → Linguagem). O backend roda pelo `aylis-host`,
como o Python: "Gerar código" cria um arquivo de eventos por form (o bloco de registro é
refeito, as funções que você escreveu ficam), o registro de todos e o ponto de entrada; o
F9 compila (`dotnet build`, `javac`, `go build`, `cl`/`g++`, `fpc`) em `.aylis/build` e
executa; Publicar monta uma pasta com o aylis-host, o projeto, o backend compilado e um
atalho; Depurar na IDE anexa o depurador da linguagem. O Verificar ambiente diz se o
compilador foi achado. Os clientes ficam em `clients/` (`clients/README.md`).

**Configurar linguagens.** Ferramentas → Configurar linguagens… marca as linguagens e a IDE
instala tudo de cada uma, sem passo manual: o compilador ou o interpretador, o depurador e o
servidor de linguagem do editor, conferindo de novo até não sobrar nada que dê para
instalar. Numa instalação nova (nenhuma linguagem instalada) o assistente abre sozinho.

| Linguagem | Compilador (Windows, pelo winget) | Depurador | Servidor de linguagem |
|---|---|---|---|
| Rust | rustup + toolchain stable, Build Tools do Visual Studio (C++) | CodeLLDB | rust-analyzer |
| Python | Python 3.12 (do usuário, no PATH) | debugpy | python-lsp-server |
| TypeScript | Node LTS | js-debug | typescript-language-server + TypeScript 5 |
| C# | .NET SDK 8 | netcoredbg | csharp-ls |
| Java | Eclipse Temurin 17 | jdb (do JDK) | Eclipse JDT LS 1.38 |
| Go | Go | Delve | gopls |
| C++ | Build Tools do Visual Studio (C++) | CodeLLDB | clangd |
| Free Pascal | Lazarus (traz o Free Pascal de 64 bits) | CodeLLDB | pasls (compilado com o Lazarus; o git vem junto se faltar) |

O winget roda sem perguntas (`--silent` e os termos aceitos); o Windows ainda pede a
permissão de administrador nos pacotes que instalam para a máquina. No macOS vem pelo
Homebrew e no Linux pelo apt (o `pkexec` pede a senha). Depois de cada instalação a IDE
relê o PATH do registro: o que acabou de entrar já vale sem reiniciar. Pela linha de
comando: `aylis-tool setup go csharp` ou `aylis-tool setup --all`.

Como o projeto vira executável, e por que ele só leva os componentes usados, está em
[Carregamento e app enxuto](carregamento.md).

**Agentes de IA (MCP).** Executar → Servidor MCP liga a IDE como servidor do Model
Context Protocol em `http://127.0.0.1:7341/mcp` (só este computador; pedidos com
`Origin` de outro site são recusados). No Claude Code:

```bash
claude mcp add --transport http aylis http://127.0.0.1:7341/mcp
```

O agente vê o projeto (`project_info`), abre forms (`open_form`, inclusive variantes),
lista e lê componentes (`list_components`, `get_component`, `get_layout`), muda
(`set_property`, `add_component`, `delete_component`, `set_layout`), salva, gera o
código, abre projetos e o código (`open_project`, `open_code`), executa e lê a saída
(`run`, que é o F9: com ponto de parada, no depurador; `stop`, `output`, `status`),
depura como o usuário (`breakpoint`, `debug`, `debug_command` com continue/step_over/
step_in/step_out/pause/stop/run_to, e `debug_state`: onde parou, a pilha, as variáveis e o
"Avaliar") e comanda o app aberto (`app_command`, o mesmo protocolo do inspetor: `tree`,
`props`, `set`, `click`...; no desktop pela porta de inspeção). Tudo
passa pelas ações da IDE: o que o agente muda aparece no designer e o Ctrl+Z desfaz. A
escolha fica guardada; `AYLIS_IDE_MCP=porta` liga só naquela execução.

Na paleta, um componente que não funciona em todas as plataformas mostra à direita os
ícones das que ele atende (Windows, macOS, Linux, Android, iOS, web), e a dica da linha
lista os nomes: o `TrayIcon` mostra os três desktops, o `CameraView` Windows, Linux,
Android e web. A tabela fica em `aylis_build::CLASS_PLATFORMS`; o que não está nela
funciona em todas.

### Paleta, designer e editores

**A paleta:**
- **Abas Windows / Apple / Android / aylis.** Em cada plataforma, as versões (WinUI 3,
  Windows 11..., iOS 26..., Material 3...) são grupos com todos os componentes visuais.
  O escolhido ali entra no form com `theme` = a família (`"iOS 26"`). Um nome de família
  vale como tema e segue o claro ou o escuro do app (`Ui::theme_named`). A aba aylis é a
  paleta por categoria, sem tema próprio.
- **Não visuais:** ficam numa seção própria no fim, cada um com um ícone e a descrição
  na dica, e não entram na galeria. O botão com o plugue abre a lista com as
  descrições. Os ícones e os textos estão em `aylis-ide/src/nonvisual.rs`.

**No designer:**
- **Fora do form:** o que passa da borda do form aparece apagado (`Ui::paint_overflow`)
  e dá para pegar de volta com o mouse.
- **`visible` desligado:** o controle fica translúcido e clicável, mas o JSON continua
  `"visible": false`. Só a propriedade faz isso (o `design_hidden` do `ControlBase`); a
  visibilidade que o próprio controle muda (as páginas de um TabControl, uma gaveta)
  continua valendo.
- **Estados:** sem transição; o estado final (desabilitado...) aparece direto, porque o
  relógio do designer não anda.
- **Duplo clique:** abre o editor do componente (os campos do ClientDataSet, os itens do
  PopupMenu e do MainMenu) ou cria e abre o evento padrão (`Component::default_event`).
- **Bandeja dos não visuais:** cada chip tem um X para excluir o componente.

**A galeria** (o botão de grade ao lado da busca da paleta):
- Tem as mesmas abas Windows / Apple / Android / aylis e, numa plataforma, as versões.
- Os componentes vivos aparecem no tema da versão, e o "Usar" escolhe `Classe@Família`,
  inclusive os subtipos.
- Abre na aba em que a paleta está.

**Ícones:**
- **Seletor:** toda propriedade de ícone (`icon`, `checked_icon`, as terminadas em
  `_icon`) tem o "…" que abre o seletor (`actions::pick_icon`). Ele traz os pacotes do
  Aylis, o Material, o Fluent e o pacote do app, com busca. Começa no ícone atual, e o
  "Sem ícone" limpa.
- **Sem projeto:** funciona também, só sem o pacote do app e sem importar SVG.
- **Prévia (F9 sem projeto, Shift+F9):** registra os ícones de pacote citados no form
  (`codegen::icons_cited_in`) e, com projeto, as fontes e os ícones de `assets/`.

**Menus e barra de status:**
- **Sem projeto aberto:** os comandos que dependem dele ficam desabilitados
  (`State::project_only`).
- **Compilando:** a barra de status mostra o tempo decorrido. Quando o programa abre,
  "✔ Compilado em …" vai para as mensagens.

**Editor de menu:** itens, divisórias, ícone, marca (`checked`, `auto_check`), atalho e o
`on_click` de cada item. O menu inteiro também tem `on_item_click`, que recebe o item
clicado em `EventArgs::Value(Component)`.

**Ordem de tabulação** (Editar → Ordem de tabulação…):
- A lista de todos os controles focáveis do form, de qualquer nível, na ordem do Tab.
  Fica no `tab_order` do form (nomes separados por `;`); os que não estão na lista vêm
  depois, na ordem natural.
- O Tab nunca passa para outro form.
- `active_control` é o focado ao abrir o form, antes do `on_show`.
- **Outras teclas no lugar do Tab:** `tab_keys` e `back_tab_keys` do form (`"Enter"`,
  `"Shift+Enter;Up"`), como o Enter que pula de campo nos sistemas de cadastro. O controle
  que usa a tecla fica com ela: o Enter de um Memo continua sendo nova linha, as setas de
  um campo andam no texto (`Component::captures_key`).
- **Pelo código**, em todas as linguagens: `frmX.edtNome.SetFocus`, `frmX.Focused` (o nome
  do controle com foco, como o `ActiveControl`), `frmX.FocusNext` e `frmX.FocusPrev` (o
  `SelectNext`). No protocolo: `focus`, `focused`, `focus_next`, `focus_prev`, e `key`
  (`{"cmd":"key","target":"frmX","key":"Shift+Tab"}`) e `type` (`{"cmd":"type",...,"text":"abc"}`)
  simulam o teclado.

**Caracteres de cada campo** (Edit, Memo e os contêineres: Form, Panel, Layout, GroupBox,
ScrollBox):
- `allowed_chars` (liberados), `blocked_chars` (bloqueados) e `only_allowed` (só os
  liberados: um campo só de números é `allowed_chars = "0-9"` com `only_allowed`). Faixas
  como `a-z`, `0-9`; `\s` é o espaço, `\-` o hífen.
- Para cada caractere decide o primeiro, a partir do campo, que diz algo sobre ele: o form
  bloqueia `@` e um campo libera → o campo aceita. Um contêiner com `chars_override` manda
  nos de dentro.
- Vale para o digitado e o colado; o texto posto pelo código não é filtrado. No Object
  Inspector, o "…" de `allowed_chars`/`blocked_chars` abre a lista com
  os modelos prontos (só números, e-mail, telefone...), a grade dos símbolos e um campo de
  teste.

**Atalhos com Alt+letra** (Button, CheckBox, RadioButton, Switch, Label, GroupBox): o `&`
antes da letra no texto (`"&Salvar"`: Alt+S clica; `&&` é o próprio `&`).
A letra aparece sublinhada. Num Label, Alt+letra leva o foco ao `focus_control` (o
`FocusControl`); num GroupBox, ao primeiro campo do grupo. Vale em todas as plataformas. A
IDE avisa nas mensagens quando a mesma letra aparece duas vezes no form.

**Listas suspensas** (combo, autocomplete, cor, data, hora):
- Ficam presas ao controle que as abriu (`Ui::follow`): a rolagem leva junto, e elas
  somem enquanto o controle está fora da parte visível.
- Saem junto com ele quando ele é liberado (`Component::dies_with`).

**Cada janela pinta em separado:** o pedido de "mais um quadro" (uma transição, a bolinha
do editor de suavização, o cursor piscando) fica guardado por form. A pintura de uma
janela não apaga o pedido da outra.

### Projetos pela linha de comando e roteiros de teste

- `aylis-tool new <pasta> <nome> [linguagem]` cria um projeto, como Arquivo → Novo projeto
  (Rust, Python, TypeScript, C#, Java, Go, C++ ou Pascal).
- `aylis-tool generate <projeto.aylisproj>` é o "Gerar código": refaz o registro dos eventos a
  partir dos forms e acrescenta os handlers novos.
- `aylis-tool script <projeto.aylisproj> <roteiro.json> [--output estado.json]` toca um
  **roteiro de teste** sem janela e mostra o estado final. O roteiro
  (`aylis_remote::script`) tem passos (`click`, `key`, `item`, `drag`, `event`,
  `set`, `wait` e `locate`, que leva um dataset a um registro) e o
  `state` a conferir (propriedades, `invoke` um método ou `"data": "record"`/`"rows"`, o
  registro atual ou todos os de um dataset). Funciona em
  qualquer linguagem: o roteiro vai numa linha JSON no stdin do app de desenvolvimento
  (`{"script": ..., "output": ...}`, sem variável de ambiente); os apps em Rust tocam no
  próprio processo (`Application::run`, só com a feature `dev`); nas outras, o
  `aylis-host --dev` toca e, depois de cada passo, manda
  `{"event": "ping"}` e espera o `{"cmd": "pong"}` da lógica (os clientes das linguagens
  respondem sozinhos), repetindo enquanto chegarem eventos novos. Nas linguagens do
  aylis-host a tela também sai num PNG ao lado do estado.
- Os apps de demonstração (`examples`, veja o README de lá) usam isso para garantir o
  mesmo resultado nas 8 linguagens.
- `aylis-tool ibge <pasta> [--offline]` monta os dados do exemplo do IBGE
  (`examples/IBGE/assets/dados`): baixa o que falta da API do IBGE e as bandeiras do
  Wikimedia Commons, guarda as respostas em `ibge-cache` e gera os JSON e os mapas em SVG.

### Variantes de form por plataforma

Um form pode ter um visual diferente por plataforma: ao lado de `forms/principal.json`
(o padrão) ficam `forms/principal.android.json`, `principal.desktop.json` e assim por
diante (`windows`, `macos`, `linux`, `android`, `ios`, `desktop`, `mobile`, `web`). Na
execução vale a variante do sistema, depois a do grupo (`android` antes de `mobile`,
`windows` antes de `desktop`), depois o padrão. Cada variante é livre, mas os handlers
são os mesmos: o código liga os eventos pelo nome, então `btnSalvar` em qualquer variante
chama o mesmo `salvar`.

Quando o form tem variantes, o código gerado traz `Componentes::de(ui)`: um campo por
componente, `ComponentId` nos que existem em todas e `Option<ComponentId>` (com o
comentário dizendo onde falta) nos que só existem em algumas, para o handler conferir.
Não há herança de form: cada variante é um layout completo.

Na IDE, Projeto → Nova variante do form copia o layout padrão para a plataforma escolhida;
as variantes aparecem embaixo do form no painel do projeto, e cada uma abre na sua aba.

### DataModule

Um arquivo de layout cuja raiz é `DataModule`, com só
componentes não visuais (conexões, `ClientDataSet`, `Timer`, `TrayIcon`, diálogos...) e,
no código dele (`src/forms/<nome>.rs`), a regra de negócio. Ele nunca abre janela. O
form que usa um DataModule declara isso no layout e se refere aos componentes pelo
caminho:

```json
{ "class": "Form", "name": "frmClientes", "uses": ["dmDados"],
  "children": [ { "class": "DataGrid", "data_source": "dmDados.dsClientes" } ] }
```

O aplicativo cria os DataModules marcados como "criar no início" antes dos forms, e
`ui.create_form("frmClientes")` cria antes os do `uses` que ainda não existem; o
`on_create` do DataModule dispara depois de carregado. Os forms continuam aceitando
componentes não visuais: o programador escolhe onde pôr cada um. Não há herança de forms
nem de DataModules.

Na IDE: Projeto → Novo DataModule cria o arquivo (com "criar no início"); o designer de
um DataModule é a bandeja de não visuais ocupando a área toda, e a paleta recusa os
visuais. Projeto → Opções do projeto → DataModules usados marca os do form aberto; o Object Inspector passa a
listar os componentes deles (`dmDados.pmAcoes`) nas propriedades de referência. No
painel do projeto, ⛁ marca os DataModules e a dica da linha diz quem usa cada um (e, num
form, quais ele usa).

### Frame

Uma parte da tela no seu próprio arquivo: o `Frame` reserva o lugar num form e mostra
outro form do projeto (`form`). Cada tela ou aba fica num form à parte, desenhado e
editado sozinho, e o form que usa só tem o frame:

```json
{ "class": "Frame", "name": "frGeneral", "form": "fraGeneral", "align": "client" }
```

Ao criar o form, a fábrica de forms do projeto cria o form de cada frame e o põe dentro,
ocupando a área toda; sem `fill` próprio, o fundo dele fica transparente. Um frame pode
ter frames. Os componentes de um frame se acham pelo nome do form dele
(`ui.find_path("fraGeneral.edtPasta")`, e no acesso tipado `fraGeneral.edtPasta`), e os
eventos vão para os handlers desse form (`fraGeneral.salvar`, no `src/forms/` dele).
Cada form entra uma vez só, e um frame que usa o próprio form (direto ou por outro) dá
erro ao carregar. Os forms dos frames ficam no projeto sem "criar no início".

Na IDE, o designer mostra o conteúdo do frame, mas clicar nele seleciona o frame, a
estrutura não lista o que é do frame e salvar não grava esse conteúdo (ele é do arquivo
do form do frame). O `form` no Object Inspector lista os forms do projeto; salvar o form
de um frame atualiza os forms abertos que o usam.

### Pacotes de código (tipo B)

O pacote composto (tipo A) é um form. O de código é um crate Rust que registra uma classe,
compilado no app de quem usa. Ele também entra no form como um frame com
`form = "pkg:Nome"` e `values`:

- **No `package.json`**: `"kind": "code"`, `"crate": "medidor-aylis"` (o nome no `Cargo.toml`)
  e `"component": "PressureDial"` (a classe que o `register` do crate registra). As propriedades
  expostas apontam direto para as do componente (`"target": "value"`), e os eventos também
  (`"target": "on_alarm"`).
- **Criar**:
  ```bash
  aylis-tool pack-code examples/pacotes/Medidor --name Medidor --component PressureDial --out Medidor.aylispkg
  ```
  O crate precisa de `pub fn register(ui: &mut aylis_core::Ui)`. O `target/` não vai. O pacote
  é assinado com a chave do autor, como o composto.
- **Usar** (Ferramentas → Pacotes de componentes → Pacotes…): só em projetos Rust. A instalação avisa que o pacote é código
  (ele roda ao compilar e no app). O crate vai para `packages/<Nome>/crate`, e os caminhos do
  Aylis no `Cargo.toml` dele passam a apontar para o Aylis desta máquina. Depois a IDE:
  - põe a dependência no `Cargo.toml` do projeto, marcada com `# pacote do aylis` (ela tira a
    linha quando o pacote sai);
  - põe `medidor_aylis::register(ui);` no `forms::register` gerado;
  - compila na hora.
- **No app**: o frame cria a classe e aplica os valores. O código-fonte do crate não vai para
  dentro do executável. No designer, a classe não existe (a IDE não carrega código novo), e o
  frame aparece vazio.

O exemplo `examples/pacotes/Medidor` é um mostrador com alarme, e serve de modelo.

### Idiomas

O texto do form é a chave da tradução: `locales/en-US.json` diz `{"Salvar": "Save"}` e o
`Ui::tr` devolve o texto no idioma escolhido (`aylis_project::i18n::set_language`). Junto
com o idioma muda o `LocaleInfo` (separador decimal, formato de data, nomes dos meses):
`en` usa o americano, `es` o espanhol, os outros o brasileiro.

As mensagens do próprio Aylis também acompanham o idioma: títulos e botões do `message_dlg` (Sim/Não/Cancelar...), as mensagens de validação
do `Edit`, os erros dos campos (`'x' não é um número inteiro válido para Idade`, `Nome é
obrigatório`), os textos do grid (`Sem registros`, `Buscar:`), os placeholders padrão e
os erros de mídia. O Aylis traz inglês e espanhol (`aylis_core::messages`); em outro idioma
elas ficam em português. O app troca qualquer uma pondo a frase em português no próprio
`locales/<idioma>.json`: o dicionário do app vem antes do catálogo do Aylis.

**A própria IDE** fala português, inglês, chinês, hindi, espanhol, francês, árabe, bengali,
russo, japonês, alemão e indonésio. Ela abre no idioma do sistema (português se o do
sistema não estiver na lista); Exibir → Idioma troca na hora, e a escolha fica guardada
na configuração da IDE. O mecanismo é o mesmo dos apps: os textos em português são a chave e
`Ui::translate_untranslated` traduz o que os painéis montam pelo código (campos de edição
ficam de fora). Os catálogos (`crates/aylis-ide/resources/locales/<código>.json`) são a fonte:
`aylis-tool translations` lista os textos da IDE que ainda
não estão no catálogo (`--templates` para os com valores) e `aylis-tool translations --apply
tabela.tsv` acrescenta as traduções nos 11 catálogos (uma linha por texto: o português e
as 11 traduções, separados por tabulação).

**Mensagens montadas com valores** (`Form X criado`) também traduzem: a chave do catálogo é
o modelo, com `{}` no lugar de cada valor (`"Form {} criado": "Form {} created"`), e o
`Ui::tr` acha o modelo que casa com o texto e põe os valores no lugar. Quando a tradução
muda a ordem, `{0}`, `{1}` dizem qual valor vai onde.

O idioma das mensagens vale para o processo inteiro (um erro de campo nasce longe do
`Ui`). Fora de um projeto, `aylis_core::messages::set_language("en")` escolhe direto.
Mensagens para o desenvolvedor (erro de expressão, de configuração, da IDE) continuam em
português.
