# Testes que dependem de Mac ou de celular

O que já foi feito no Aylis e só pode ser conferido (ou terminado) com um Mac ou um celular
de verdade. O emulador do Android cobre muito, mas não tudo: o que está aqui ou não roda
nele ou foi conferido só nele. Ao testar um item, anote o resultado no
[histórico de decisões](historico-decisoes.md) e risque daqui.

## Mac (macOS e iOS)

Sem um Mac não dá para compilar o código Objective-C/Swift das dependências da Apple nem
rodar no simulador do iOS. Para o iOS, o `aylis-tool` monta o app: `publish --target ios` (o
simulador) ou `publish --target ios --device` (o iPhone: o `.app` e o `.ipa` assinados), e o
`run --target ios [--device]` instala e abre.

| O quê | Situação | Como conferir |
|---|---|---|
| Rodar no macOS (janela, renderizadores, teclado, mouse, touchpad) | rodou numa VM do macOS 15 Intel (Hyper-V, sem GPU): a IDE e os apps abrem e desenham, as linguagens rodam (22/11); falta um Mac de verdade (GPU, Apple Silicon, teclado, touchpad, Retina) | um exemplo qualquer, `cargo run --example components` |
| Vídeo e áudio do AVFoundation depois da revisão de 31/10 (decodificador aberto na thread de vídeo, trilha lida do arquivo aos poucos, `cpal` 0.18, conferência do tamanho da linha) | só revisado; o `avf.rs` não compila fora do Mac | exemplo `Player` com um MP4 e um MP3 |
| Rodar no iOS (janela, toque, teclado na tela) | compila para iOS; nunca rodou | `aylis-tool run app.aylisproj --target ios` com o simulador aberto |
| Teclado do iOS reduzindo a área segura (`UIKeyboardWillChangeFrameNotification`) | compila | um form com campos embaixo; o campo com foco sobe acima do teclado |
| Ícone, nome e bundle id do app de iOS (o ícone do projeto compilado pelo `actool` para o `Assets.car`, desde 18/11) | escrito; o `actool` nunca rodou | publicar e olhar a tela inicial do simulador e do iPhone |
| O iPhone sem o Xcode no meio (18/11, `aylis_tools::ios_device`): `publish --target ios --device` compila para `aarch64-apple-ios`, acha o perfil de provisionamento nas pastas do Xcode (pelo bundle id, pelo aparelho e pela validade) e o certificado que ele aceita (pelo SHA-1), assina com os direitos tirados do perfil e grava o `.ipa`; `run --target ios --device` instala e abre pelo `xcrun devicectl` | a escolha do perfil e do certificado, os direitos, o `Info.plist` e o `.ipa` conferidos por teste no Windows; o `codesign`, o `actool` e o `devicectl` nunca rodaram | um Mac com o Xcode 15 ou mais novo e um iPhone com o Modo de Desenvolvedor: (1) com a conta gratuita, gerar o perfil por um projeto do Xcode com o mesmo bundle id; (2) `aylis-tool run app.aylisproj --target ios --device`; (3) conferir a mensagem de erro sem perfil, com o perfil vencido e com o iPhone fora do perfil; (4) instalar o `.ipa` pelo Apple Configurator |
| 3D sem cópia no macOS (IOSurface) e no iOS (Skia Metal); Metal no iOS | escrito | exemplo `model3d`; conferir o FPS e o uso de CPU |
| WebView no macOS e no iOS (wry) | escrito | exemplo `browser`: carregar, mensagem, `eval` |
| Vídeo pela AVFoundation (macOS e iOS) | compila para iOS | um `VideoPlayer` com um MP4 |
| Câmera no macOS e no iOS (AVCaptureSession) | **falta fazer** | `CameraView`: ao vivo, foto, trocar de câmera |
| Acessibilidade no macOS e no iOS (VoiceOver pelo AccessKit) | compila | ligar o VoiceOver e navegar pelos controles; o exemplo `automation` |
| Pinça e giro do touchpad no macOS viram `on_gesture` | escrito | `Model3D` ou `Graph`: pinçar e girar |
| Corretor e sugestão de texto nativos (NSSpellChecker no macOS, UITextChecker no iOS) | **falta fazer** (hoje só com um motor posto pelo app) | `spellcheck` e `autocomplete` no Edit; exemplo `spellcheck` |
| Ícone da bandeja e notificações no macOS | escrito para o desktop | exemplos `tray` e `notification` |
| O `guid` e o `tray_uid` do `TrayIcon` (25/11) no macOS e no Linux: sem efeito (o `tray-icon` ignora os dois); o ícone tem de aparecer igual com os dois preenchidos | escrito; não compilado no macOS | `desktop_checks tray_id` (com `--features tray`) num Mac |
| IDE em `.app` com ícones por tipo de arquivo (`aylis-tool ide-package --target macos`) | gerado e conferido por teste; nunca aberto no Finder | instalar o `.app` e ver o ícone de um `.aylisproj` e de um `.aylisform` |
| A barra de título própria no macOS (`border_style = custom`): a barra do sistema transparente, sem título nem botões, e o conteúdo por baixo, para manter os cantos arredondados e a sombra; com `window_corners = square`, sem moldura | escrito; compila no macOS (a IDE na VM Intel, 22/11); nunca visto na tela | exemplo `titlebar`: os cantos, arrastar pela barra própria, redimensionar, e que os botões do sistema não aparecem |
| O form transparente e com forma no macOS (`transparent`, `window_shape`; 18/11): a janela com `with_transparent` e o OpenGL com transparência; o vidro fosco pelo `blur` do `WindowSettings` (o `set_blur` do winit) | escrito; compila no macOS (a IDE na VM Intel, 22/11); nunca visto na tela | exemplo `shaped_forms` com `AYLIS_RENDERER=gpu`: o form redondo sem fundo fora do círculo e o vidro desfocando o que está atrás; a forma do desfoque (no Windows ele segue a forma; no macOS, ver se fica retangular) |
| Tempo de CPU e amostragem da pilha no profiler, no macOS | só Windows (e o tempo no Linux) | profiler da IDE num app em Rust |
| `AppLink`, sensores e `BackgroundService` no iOS | **falta fazer** | `TesteAparelho` no simulador |
| `UrlScheme` no macOS e no iOS (`CFBundleURLTypes` na publicação; o link com o app aberto) | **falta fazer** | um app com `UrlScheme` e o link no Safari |
| `SystemBars` no iOS (14/11): os ícones e o esconder pelo estilo da barra do winit, a cor como faixa pintada pelo núcleo; o fim da rolagem com mola (`overscroll` `bounce`) | escrito; o `ios_system_bars` não foi compilado (precisa de Mac) | um form com `SystemBars` (cor, `icons` `light`, `status_bar_visible` falso) e uma lista rolada até o fim, no simulador |
| A orientação da tela no iOS (14/11): o `UISupportedInterfaceOrientations` (e `~ipad`, com `UIRequiresFullScreen`) do `Info.plist` com a do projeto e as dos forms; em execução, o `set_valid_orientations` do winit ao trocar de form e ao mudar a propriedade | o plist conferido por teste; o `sync_orientation` do iOS não foi compilado (precisa de Mac) | projeto `portrait` com um form `sensor_landscape`: o app em pé, o form deita ao abrir e volta ao fechar; no iPad, a tela inteira; e conferir se uma orientação pedida só pelo código (fora do plist) encerra o app ou só é ignorada |
| Os pacotes de macOS (14/11): `android-sdk` e `dotnet` de ARM (o `android-ndk` do macOS só sai em `.dmg` e ficou de fora) | os de Intel (`macos-x86_64`: rust, jdk, android-sdk, cargo-ndk, wasm-bindgen, node, dotnet, go) instalados na VM do macOS 15 pelo `setup` (22/11; o netcoredbg não existe para Intel: o C# roda sem depurar); os de Apple Silicon nunca instalados; o android-sdk nunca usado para publicar | Ferramentas → Pacotes de ferramentas num Mac com Apple Silicon: instalar e publicar um app para Android e um projeto em C# |
| Área de transferência no macOS (texto, HTML e imagem pelo `arboard`) e no iOS (o `UIPasteboard`) | macOS compilado (22/11), não conferido; iOS **falta fazer** (copia e cola só dentro do app) | o roteiro 119 no Paint e na IDE: imagem para o Preview, HTML para o Pages |
| `PushNotifications` no iOS e no macOS (APNs) | compila para iOS e para macOS (22/11); no macOS só liga dentro de um `.app` (fora dele derrubava o processo); nunca registrou no APNs. A dúvida maior: o toque que abre o app fechado (o delegado da central é posto quando o Aylis cria os serviços, talvez depois do fim da partida) | o app com a capacidade Push no App ID; mandar pelo APNs sandbox (ou arrastar um `.apns` no simulador) com o app aberto, em segundo plano e fechado |

| Janelas e sistema de 23/11 no macOS: `ui.cursor_position()` (o `NSEvent.mouseLocation`, convertido para pixels físicos com a origem no canto de cima), o Caps Lock (`modifierFlags`), o `device_path` (`cg:<CGDirectDisplayID>`), a instância única (`flock` e o socket na pasta do usuário), o `place` (posição e tamanho pelo winit), o menu da bandeja pelo `on_context`; a captura de monitor ainda não (pede a permissão de gravação da tela: `CGPreflightScreenCaptureAccess` e o `CGDisplayCreateImage`, a fazer) | escrito; não compilado (precisa de Mac) | exemplo `desktop_checks` (`cursor`, `keys`, `monitors`, `single`, `tray`) num Mac com dois monitores, um Retina |
| Iniciar com o sistema no macOS (24/11): o agente de login `~/Library/LaunchAgents/<app>.plist` (`RunAtLoad`, o caminho do executável em `ProgramArguments`), e a instância única com o nome do projeto e a soltura (o `flock` e o socket) | escrito; não compilado (precisa de Mac) | `desktop_checks login on`, sair e entrar na conta: o app abre; `login off` tira o arquivo. `desktop_checks quiet`: a segunda abertura sai calada e, depois de 5 s, a próxima vira a primeira. Conferir também com o app dentro de um `.app` (o `launchd` pode pedir o `SMAppService` nas versões novas) |

## Celular de verdade (Android e iOS)

Conferido no ASUS ZenFone 5 (Android 9, Snapdragon 660) em 01/11/2026, com o
`examples/TesteAparelho`: o teclado sem sugestões, a câmera traseira e a frontal (com a troca
e 20 aberturas seguidas), o microfone gravando, o vídeo pelo MediaCodec (10 aberturas), as
notificações (simples, com botões, com progresso, com a imagem de um form), o tema claro e
escuro nas barras do sistema, a área segura, a tela girada e a volta ao app depois de o
Android encerrar o processo em segundo plano. Corrigido no caminho: o app caía
ao abrir no Android 9 (acessibilidade), a câmera frontal saía girada e a troca congelava, o
vídeo saía com as cores erradas nos Snapdragon, `temp_dir` apontava para uma pasta proibida e
a barra de abas não rolava com o dedo.

Também conferido no ZenFone (01/11, retomada): o canal de desenvolvimento pelo cabo (`adb
reverse`; Inspetor do app, mudar propriedade na hora e hot reload pela IDE) e os gestos do
`Model3D` na aba 3D, injetados pelo adb (`input swipe` e `sendevent` com dois dedos em
`/dev/input/event1`): um dedo gira a câmera, pinça aproxima e afasta, dois dedos deslocam a
vista (o modelo acompanha o dedo e fica no lugar ao mudar o zoom) e o toque duplo centraliza.
Corrigido no caminho: o arrasto do dedo virava rolagem e chegava ao `Model3D` como zoom.

Também conferido no ZenFone (01/11, fim da fase 1), medindo pelo SurfaceFlinger
(`dumpsys SurfaceFlinger --latency`):
- **Limite de quadros.** `ui.set_max_fps(30.0)` dá 33,3 ms entre quadros (sem o limite, 55
  quadros/s).
- **Câmera.** Os quadros vêm reduzidos ao tamanho da tela (conversão em inteiros, em blocos e
  em várias threads); no `max` (12 MP) a tela foi de 6,5 para 18 quadros/s e no `medium` de 25
  para 32.
- **Foto.** Sai no tamanho cheio (3024x4032) em 0,7–1,1 s; antes, 20 s com a compressão
  padrão do PNG.
- **Depuração.** `aylis-tool debug-android app.aylisproj` (app compilado com `--debug` ou pelo
  Executar, que o marcam como depurável) anexou o `lldb` ao app aberto. O ponto de parada no
  handler do botão parou em `principal.rs:278` com a pilha até o `ui.rs`, e o app seguiu
  depois do `detach`. O `lldb` do NDK não mostra variáveis de Rust; o CodeLLDB
  ("aylis: anexar (Android)") mostra.

No Windows, a abertura da IDE com o lingote em `aylis_3d::ingot` sai certa. A câmera no navegador
foi conferida com a webcam de verdade (Edge sem janela, a permissão aceita sozinha): 640x480,
1280x960 e 1920x1080 em `low`, `medium` e `max`, com a imagem mudando. Falta a câmera no Linux
(a webcam no WSL pede o `usbipd-win`).

Conferido só no emulador (01/11), com o `examples/TesteAparelho`: o `AppLink` (abrir link,
abrir outro app, a folha de compartilhar, receber link e compartilhar com o app frio ou
aberto), os sensores (valores do emulador mudando ao vivo) e a vibração, o `BackgroundService`
(segue com a tela fechada, volta depois de `kill -9` e depois de reiniciar), o estado da
ligação (tocando e livre) e o loopback do áudio (as amostras chegam, mas em silêncio).

Ainda falta um aparelho para:

| O quê | Por que o aparelho | Como conferir |
|---|---|---|
| Loopback do áudio (`loopback:Android` no `AudioInput`) | no emulador as amostras chegam zeradas | tocar música noutro app e gravar pela aba de áudio do `TesteAparelho` |
| `BackgroundService` com a otimização de bateria da ASUS | o emulador não tem as listas do fabricante | ligar o serviço, fechar o app e deixar a tela apagada por meia hora |
| Papel de telefone (`Phone` com `dialer`) e `LockScreen` | o emulador não toca chamada de verdade nem tem bloqueio com senha | aceitar o papel de telefone, ligar para o aparelho; abrir pela notificação de tela cheia com a tela bloqueada |
| `Nfc` no Android (ler e gravar NDEF) | o emulador não tem NFC | uma tag NTAG no ZenFone: ler uma gravada pelo celular, gravar `text:`/`uri:` e ler de novo; uma tag só de leitura dá `on_error` |
| `Bluetooth` no Android (procurar, conectar, ler, notify; o clássico) | o emulador não tem Bluetooth | uma pulseira ou sensor BLE (frequência cardíaca, `180D`/`2A37` com `subscribe`); uma impressora térmica pareada com `open`/`send` |
| `Nfc` no desktop (leitor USB PC/SC) e na web (Web NFC) | não há leitor nesta máquina; a Web NFC só existe no Chrome do Android | um ACR122U com uma NTAG; o Chrome do ZenFone num endereço `https` |
| `Bluetooth` no Windows (`btleplug`) e na web (Web Bluetooth) | nesta máquina o adaptador é achado, mas nenhum aparelho BLE anunciava por perto (o próprio exemplo do `btleplug` também não viu nenhum) | um aparelho BLE ligado perto do PC; o teste `scans_the_real_adapter` (`--ignored`, feature `bluetooth`) e um form com `Bluetooth` |
| `PushNotifications` no Android (FCM sem o SDK) | o Java compila e o manifesto é conferido por teste; nunca registrou: precisa de um projeto no Firebase (o `google-services.json`) e de um aparelho ou emulador com o Google Play | um form com o componente e os quatro campos do Firebase; o `token` no `on_token`; mandar pela API HTTP v1 com o app aberto, em segundo plano e fechado (o toque dá `on_opened`) |
| `PushTrigger` + `WakeAppAction` no Android (a "chamada" com o app fechado e a tela bloqueada) | o Java compila; nunca rodou. Dúvidas: o toque repetindo (`FLAG_INSISTENT` no canal de toque), o fim do tempo chegar pelo `deleteIntent` e o app abrindo por cima do bloqueio no Android 9 do ZenFone e num Android 14 | o form de chamada do guia (`docs/push.md`); bloquear o aparelho, matar o app e mandar `kind=call`; atender, recusar e deixar o tempo acabar |
| `PushNotifications` no navegador, no Windows e no Linux | compilam e o `sw.js` foi conferido no Node; nunca receberam | Web Push: a página publicada em https com um par VAPID; WNS: o app em MSIX; Linux: o ntfy como distribuidor do UnifiedPush |
| Compartilhar arquivo do `AppLink` para outro app | conferido só o texto | botão "Compartilhar" do `TesteAparelho` com uma foto, para o WhatsApp ou o Gmail |
| Depurar pelo Wi-Fi (`aylis-tool android-wifi`) | o emulador já é rede | com o cabo, `android-wifi`; tirar o cabo e `debug-android` |
| `Gamepad` no Android (botões, direcional, analógico esquerdo, vibração) | o emulador não tem um controle de verdade | um controle Bluetooth ou USB-OTG no ZenFone, um form com `Gamepad` e `Label`s nos eventos |
| Corretor e sugestão do Aylis no celular (`spellcheck`, `autocomplete`) | o motor nativo do Android não foi feito; hoje só com um motor do app | exemplo `spellcheck` com um `WordList` |
| 3D (aquecimento, bateria) | no ASUS ZenFone 5 (Adreno 512, GLES 3.2) o 3D já sai certo; desempenho medido em 01/11 no `examples/Estresse3D` com o corte do brilho em 8 px: 57/57/55/39/26/22 quadros/s para 1/4/9/16/25/36 lingotes (CPU a 42–46 °C; com o corte em 16 px dava 57/56/58/45/31/27). Falta deixar alguns minutos para ver o aquecimento e a bateria | `examples/Estresse3D`, alguns minutos com 1 e com 36 lingotes (ou a aba 3D do `examples/TesteAparelho`) |
| Canal de desenvolvimento sem cabo (o IP da máquina) | pelo cabo, conferido no ZenFone (01/11); sem cabo, só no desktop e por teste | Executar no Android pelo Wi-Fi, com o Inspetor do app e o hot reload |
| Barras do sistema, tecla Voltar, gestos e fim da rolagem (14/11: `SystemBars`, `on_back`, `on_swipe`, a gaveta pela borda, `swipe` do `TabControl`, `zoomable`, `swipe_actions`, `overscroll`) | conferidos por teste sem janela; o Java da `setSystemBars` compilado com o `javac`; nunca rodou num aparelho | o roteiro 140 no emulador `aylis_pixel` (Android 14: esticar) e no ZenFone (Android 9: o brilho), com a gaveta aberta pela borda sem brigar com o gesto de voltar do sistema |
| A orientação da tela no Android (14/11): o `screenOrientation` do manifesto e o `setRequestedOrientation` ao trocar de form, ao mudar a propriedade e pelo `Screen.set_orientation`; o `debug.aylis.orientation` | o manifesto conferido por teste e a assinatura JNI pelo `javap` contra o `android.jar`; nunca rodou num aparelho | o roteiro 155 no emulador `aylis_pixel` e no ZenFone (girando o aparelho, com a trava de rotação ligada e desligada) |
| O menu de texto dos campos e as alças da seleção no toque (14/11: o toque longo no `Edit`/`Memo` seleciona a palavra e abre Recortar, Copiar, Colar, Selecionar tudo; as alças arrastáveis) | conferidos por teste sem janela (toque simulado e área de transferência falsa); nunca rodou num aparelho | o roteiro 156 no Motorola Edge 30 Neo (Android 14, onde foi relatado), no ZenFone (Android 9) e no `aylis_pixel`: colar a senha e o endereço copiados de outro app, arrastar as alças, o tamanho das alças para o dedo |
| Android de 32 bits (`armeabi-v7a`, 13/11: TV box, celular de entrada) | o APK com `lib/armeabi-v7a` sai (14/11, com o Skia compilado pelo Aylis, o pacote `skia-android-armv7`); o Aylis nunca rodou num aparelho de 32 bits | um TV box ou celular de entrada com Android de 32 bits, o app publicado com "Também 32 bits" |
| Publicação: AAB na Play Store (teste interno) e o APK assinado instalado à mão | o AAB foi validado, mas não subiu para a loja | criar o teste interno na Play Console |
| O iPhone sem Mac (18/11): o `.ipa` sem assinatura (`publish --target ios --device --no-sign`) e o workflow do GitHub (`aylis-tool ci-ios`) | o workflow conferido por teste (os passos, os segredos, o YAML válido); nunca rodou no GitHub; o `.ipa` sem assinatura nunca passou pelo Sideloadly. A tela da IDE (Opções do projeto → iPhone (GitHub)) conferida por imagem; o copiar do `.p12` e do perfil, nunca com arquivos de verdade | com a 0.8.0 ou mais nova: `aylis-tool ci-ios` num projeto num repositório do GitHub, Run workflow, baixar o `.ipa` dos Artifacts e instalar pelo Sideloadly no Windows num iPhone com a conta gratuita; depois, com `--signed` e os segredos de uma conta paga |
| iOS no iPhone (tudo da seção Mac que vale para o iOS) | precisa de Mac e de um iPhone | `aylis-tool run app.aylisproj --target ios --device` |
| A Landing publicada no celular (11/11): a rolagem acompanhando o dedo; o 3D da documentação pelo WebGL num navegador sem WebGPU | a rolagem medida no ZenFone (22/11, Edge 138: de 12 para 30 quadros/s; falta o autor conferir e a escala quebrada de 2,625); o WebGL, no Edge sem janela (SwiftShader) com o `navigator.gpu` escondido | o site republicado, no Safari de um iPhone antes do iOS 26 e no Chrome de um Android; a página do `Model3D` na documentação |

## PC com outro hardware ou periférico

Feito e testado por código, mas a máquina do autor não tem o hardware para conferir até o fim:

| O quê | Por que | Como conferir |
|---|---|---|
| O `on_topmost_lost` com o shell no Windows 10 (25/11): o "^" da bandeja lá é o `NotifyIconOverflowWindow` e o Iniciar e a central são do `ShellExperienceHost.exe`/`StartMenuExperienceHost.exe` | as classes e os processos estão na lista do `system`; nunca rodado num Windows 10 | `desktop_checks topmost` num Windows 10: o "^", o Iniciar e a central por cima do form dão `system true` |
| Imprimir pela impressora do sistema (`Report::print`, diálogo do Windows e GDI) | testado só o PDF e a visualização; a impressão real não saiu no papel | exemplo `report`, botão Imprimir, escolher uma impressora |
| O form transparente e com forma no Linux (18/11): `with_transparent` e o OpenGL com transparência, com um compositor (X11 com picom, Wayland); o desfoque só no KDE | escrito; não conferido | exemplo `shaped_forms` com `AYLIS_RENDERER=gpu` num Linux com compositor: fora da forma, o que está atrás |
| Imprimir no Linux (PDF aberto pelo `xdg-open`) | só compilado | o mesmo exemplo no WSL ou num Linux |
| A área de transferência no Wayland sem o XWayland (24/11): o `arboard` com o `wayland-data-control` fala com o compositor direto (o `ext-data-control` ou o `wlr-data-control`, que só o KDE e o wlroots têm); sem o protocolo, cai no X11 como antes | no WSLg o compositor não tem o protocolo: o arboard avisou e caiu no X11, e o texto foi e voltou certo; num Ubuntu 26.04 com GNOME 50 (24/11, por um app), o mesmo: o Mutter não tem o data-control, o aviso saiu pelo logger do app e o X11 funcionou; o caminho Wayland nunca rodou | num Linux com KDE ou sway (Wayland), um app com `Edit` aberto com `env -u DISPLAY` (sem o XWayland): Ctrl+C no app e colar num editor do sistema, e o contrário |
| Janelas e sistema de 23/11 no Linux (X11 pela `libX11` carregada na hora): `ui.cursor_position()` (`XQueryPointer`), o Caps e o Num Lock (a máscara do ponteiro), a captura de monitor (`XGetImage`, só 32 bits por pixel), o `device_path` (o nome da saída), a instância única (`flock` e o socket no `XDG_RUNTIME_DIR`); no Wayland, `None` e erro (o portal de captura fica de fora) | compilado no WSL; não rodado | exemplo `desktop_checks` (`cursor`, `capture`, `keys`, `monitors`, `single`) na VM Fedora com X11 e com Wayland |
| O `KeyboardCapture`, o cursor de imagem e o Scroll Lock no Linux (24/11): o `XGrabKeyboard` na conexão do winit, o inibidor de atalhos do Wayland (`zwp_keyboard_shortcuts_inhibit_manager_v1`), o `CustomCursor` do winit e a luz do Scroll Lock (`XGetKeyboardControl`) | compilado no WSL; não rodado | `desktop_checks grab` na VM Fedora com GNOME no Wayland (o GNOME pergunta se deixa o app pegar os atalhos: aceite; Super e Alt+Tab chegam ao app) e com X11 (Mint, Cinnamon); a mira como cursor nos dois; `scroll true` com o Scroll Lock ligado no X11 |
| A janela de vídeo no Linux (24/11): a ativação pelo token (`xdg_activation_v1.activate` com o `XDG_ACTIVATION_TOKEN`, `bring_to_front`), a escala fracionária do Wayland e a troca entre monitores (`form_scale`, `on_scale_change`), o fim do redimensionamento ao soltar a borda (300 ms sem tamanho novo), o `form_at_screen`/`pointer_over` no X11 com um gerenciador que põe moldura | no WSLg (22.04): pelo X11, o form sob o mouse e o `pointer_over` certos e o `on_resize_end` da largura mudada pelo código; pelo Wayland, o mouse `None` e o `pointer_over` falso; o compositor do WSLg não tem o `xdg-activation` (o `bring_to_front` caiu no pedido de atenção, sem erro) nem escala fracionária | exemplo `window_reports` na VM Fedora (GNOME, Wayland): com a escala em 125%, a primeira linha diz 1.25; arrastar a borda dá um `on_resize_end`; aberto pelo terminal do GNOME (que passa o `XDG_ACTIVATION_TOKEN`), `-- auto` com outra janela clicada antes dos 3 s: a janela volta com o foco (sem o token, só pisca). Na Mint (Cinnamon, X11): passar o mouse sobre a janela e sobre outra por cima dela (`pointer_over` true/false) |
| O tamanho pelo código e o monitor no GNOME pelo Wayland (24/11): o `request_inner_size` que o winit aplica na hora (sem `Resized`) e o Aylis acompanha; o `form_monitor` pelo `wl_surface.enter`, conferido a cada volta enquanto falta; o aviso do `.desktop` que falta para o GNOME lembrar a permissão do `KeyboardCapture` | no WSLg pelo Wayland: a janela foi a 900 × 560 e 520 × 560 com o form, os `on_resize_end` e o `form_monitor` certo com 1 s; no X11, igual a antes. O defeito foi relatado num Ubuntu 26.04 (GNOME 50) | `window_reports -- size` no Fedora com GNOME no Wayland (`env -u DISPLAY`), também com `--features gpu`; um tamanho maior que a área livre do monitor (o form fica com o que coube); `desktop_checks grab` sem instalar (o aviso do `.desktop` no stderr, a pergunta do GNOME volta) e com um `~/.local/share/applications/desktop_checks.desktop` (aceite uma vez; na segunda abertura o GNOME não pergunta) |
| Iniciar com o sistema no Linux (24/11): o `<app>.desktop` em `~/.config/autostart` (o `Exec` com o caminho entre aspas), e a instância única com o nome do projeto e a soltura | compilado; não rodado | `desktop_checks login on` na VM Fedora (GNOME) e na Mint (Cinnamon), sair e entrar: o app abre; `login off` tira o arquivo. `desktop_checks quiet` como no Windows |
| A janela sem o branco da abertura no Linux e no macOS (19/11): nasce escondida e aparece com o primeiro quadro | escrito; conferido só no Windows | `cargo run -p aylis-platform-winit --example window_start --features gpu -- normal --stay`: a janela aparece já escura |
| A tela cheia num segundo monitor (`monitor` do form com `window_state = full_screen`), de nascença e pedida depois (19/11) | a máquina do autor tem um monitor só | o exemplo `window_start full` com `monitor = 1` no form: cobre o segundo monitor inteiro |
| Os pacotes de Linux (14/11): `android-sdk`, `android-ndk` e `dotnet` | o `dotnet` instalado e usado no Ubuntu 26.04 e no Fedora 45 (21/11); o `android-sdk` e o `android-ndk` nunca instalados num Linux | `aylis-tool packages install android-sdk android-ndk dotnet` num Linux; publicar um app para Android e compilar um projeto em C# |
| `UrlScheme` no Linux (o `.desktop` do usuário com o `xdg-mime`, o socket que repassa o link à cópia aberta, o prefixo no `.desktop` do `.deb`) | o módulo conferido com o `rustc` para Linux e o `.deb` por teste; nunca rodou | exemplo `url_scheme` num Linux com desktop: `xdg-open meuapp://teste` com o app fechado e aberto; instalar o `.deb` de um app com `UrlScheme` ligado |
| Vídeo pelo GStreamer depois da revisão de 31/10 (aberto na thread de vídeo, trilha lida aos poucos) e o `cpal` 0.18 (ALSA/PipeWire) | só revisado; não compilado para Linux nesta máquina | exemplo `Player` num Linux |
| A parede de vídeos numa placa integrada (25/11): o `Wait` só com a cerca pendente, a cópia parcial para o buffer de trás com o `Present1` (`FLIP_SEQUENTIAL`) e a `exclusion` sem copiar o fundo, no Direct2D | medido só na AMD RX 9070 XT, onde a cópia da janela e a do fundo já eram baratas (a pintura com `difference` de 21,4 para 9–13 ms; a `exclusion` igual à `difference`); o relato era numa Intel UHD 770 com 99% de 3D | numa máquina com a Intel UHD 770 (de preferência com o monitor ligado nela), `examples/MarcaDagua` com `gpu` = `Intel` no form, `--grid 16` e `--grid 64`, com `exclusion` (o padrão) e `--blend difference`: o 3D da Intel pelo contador `\GPU Engine(*)\Utilization Percentage` do processo e a pintura pelo `profile` (`blend_layers`); comparar com a 0.11.0 |
| Escolher a placa de vídeo com duas GPUs (`gpu` no projeto, `AYLIS_GPU`) | a máquina tem uma GPU só; conferido que a escolha pelo nome acha a placa certa | um notebook com GPU integrada e dedicada; a linha de bench mostra a placa usada |
| Seguir o monitor (`gpu` = `monitor` no projeto ou no form; [gpu-por-monitor.md](gpu-por-monitor.md)) | testado numa placa só, com o WARP no lugar da segunda (11/11); precisa de monitores ligados em placas diferentes | a máquina dos seis monitores: arrastar a janela da dedicada para a integrada, com e sem `AYLIS_PRESENT=composition`, com um `Model3D` e um `VideoPlayer` |
| Form em vários monitores (`span_monitors`, uma janela por monitor; [gpu-por-monitor.md](gpu-por-monitor.md)) | a máquina tem um monitor: conferido por teste (os pedaços, a pintura de cada pedaço igual ao recorte do form no Skia e no Direct2D, a entrada, a lista do ComboBox) e com `AYLIS_SPAN_SPLIT=2` (o monitor contado como dois: duas janelas de verdade, o clique em cada uma, desligar e religar), no Direct2D e no Skia GPU (14/11) | na máquina dos seis monitores, `cargo run -p aylis-platform-winit --example span_monitors --features d2d` (e com `-- --full`, o telão): 1) o form no meio de dois monitores de placas diferentes vira duas janelas sem moldura, uma só na barra de tarefas e no Alt+Tab; 2) `AYLIS_D2D_DEBUG=1`: cada janela na placa do seu monitor, sem travar; 3) passar o mouse de um monitor para o outro sobre o form: o realce não pisca nem some na divisa; apertar o botão da esquerda e soltar já no outro monitor (o botão solta o realce, sem clique); 4) clicar numa janela e depois na outra: o `on_deactivate` não aparece no terminal; 5) abrir o ComboBox perto da divisa de baixo: a lista fica no monitor dele; 6) o form num monitor a 100% e noutro a 150%: o form com a escala da principal nos dois, sem mudar de tamanho; 7) desligar um monitor com o app aberto: as janelas se refazem; 8) com `AYLIS_RENDERER=gpu` (o Skia), repetir 1 a 3 |
| O print sobre todos os monitores em pixels físicos (`physical_pixels` com `full_screen` e `span_monitors`, o pedido do NoOBS de 14/11; [gpu-por-monitor.md](gpu-por-monitor.md)) | a máquina tem um monitor a 150%: conferido por teste (a origem negativa, o pixel da tela no form, cada pedaço igual ao recorte do print pixel a pixel no Skia e no Direct2D) e com `AYLIS_SPAN_SPLIT=2` (o print aberto e liberado 20 vezes, o clique no pixel certo, ~75 ms para abrir) | num PC com dois monitores de escalas diferentes, um deles à esquerda do principal (origem negativa, como (-1920, 0, 5760, 2160)): `cargo run --release -p aylis-platform-winit --example print_overlay --features d2d -- --auto`: 1) o print cobre os dois de uma vez, sem atraso entre eles e sem aparecer antes num só; 2) `clique no pixel certo: sim` (o clique vai ao monitor da direita); 3) o quadriculado de 1 px no canto de cima à esquerda do monitor da esquerda nítido, sem borrão (a imagem bate pixel a pixel); 4) a mediana de abrir perto de 100 ms ou menos; 5) no NoOBS, o atalho com outro app à frente: a seleção feita no monitor a 100% e no a 150% recorta exatamente o que estava sob o mouse |
| Seguir o monitor no Linux (Wayland, `gpu_linux.rs`) | no WSL: a troca de contexto entre o D3D12 e o llvmpipe e o quadro depois dela; o compositor do WSLg não tem o `linux-dmabuf` v4 | um Linux com duas placas no GNOME ou no KDE (Wayland): `AYLIS_DEBUG_GL=1 cargo run -p aylis-platform-winit --example gpu_monitor` e arrastar a janela para um monitor da outra placa |
| Traçado de raios no Vulkan e no Metal | testado no DX12 (Windows) | exemplo `model3d` com "Traçado de raios" num Linux ou Mac com GPU que tenha ray query |
| FBX de verdade (Mixamo, Maya, Blender) | aguardando os arquivos do autor | exemplo `model3d` com o arquivo |
| Pacote da IDE no Linux (`ide-package --target linux`: `.desktop`, MIME, ícones) | instalado pelo `instalar.sh` no Fedora 45 e pelo `.deb` no Ubuntu 26.04 e no Mint 22.3 (20 a 22/11): o atalho e a IDE abrem; falta abrir um `.aylisproj` pelo gerenciador de arquivos | `instalar.sh` num Linux com desktop e abrir um `.aylisproj` pelo gerenciador de arquivos |
| Câmera no Linux | a webcam no WSL pede o `usbipd-win` | `CameraView` num Linux com webcam |
| `Gamepad` com um controle de verdade (Windows, Linux, navegador) | a máquina do autor não tem um controle ligado; o `gilrs` abre e lista vazio | ligar um controle de Xbox ou PlayStation, um form com `Gamepad`; `rumble` no Windows |
| Agente de depuração remota em outro PC (`debug-agent`, `agent-run`, `agent-dap`) | conferido com agente e cliente na mesma máquina | agente num servidor Windows ou Linux, pareado pelo código |

## Leitor de tela e entrada de idiomas (IME)

O Aylis expõe a árvore de acessibilidade pelo AccessKit (no Windows, conferido pelo UI
Automation) e a entrada por IME pelo winit, mas o uso de verdade, com um leitor de tela e um
teclado de outro idioma, ainda não foi conferido em nenhuma plataforma. Software para
governo costuma exigir acessibilidade. Use a `Galeria` e o `IBGE` (grade, mestre-detalhe,
campos).

| O quê | Leitor ou IME | Como conferir |
|---|---|---|
| Ler um form: título, rótulos, botões, o valor de cada campo | Narrador e NVDA (Windows), TalkBack (Android), VoiceOver (Mac e iPhone) | abrir o form e navegar com as teclas do leitor; cada controle diz o nome (o `label` ou o texto ao lado) e o papel |
| Navegar com Tab e Shift+Tab, e o Enter como Tab (`tab_keys`) | os mesmos | a ordem do `tab_order`; o leitor anuncia o controle que ganha o foco |
| Preencher um Edit, um NumberBox e um DateEdit | os mesmos | o leitor repete o que foi digitado; a máscara e o `char_map` não confundem o que é lido |
| A grade (`DataGrid`/`DBGrid`): linha, coluna, cabeçalho e o valor da célula | os mesmos | andar com as setas; o leitor diz "linha 3, Nome, Maria" |
| Botões só com ícone | os mesmos | o leitor diz o `hint` (o ícone sozinho não tem nome) |
| Caixas de mensagem e modais | os mesmos | o foco vai para a caixa e volta ao fechar; o texto é lido |
| Japonês e chinês pelo IME: a palavra em composição aparece no campo, e a escolha do candidato entra | IME do Windows, do macOS e o teclado do celular | digitar num Edit e num Memo; a janela de candidatos fica perto do cursor |
| Coreano (a sílaba montada letra a letra) e o árabe (da direita para a esquerda) | IME do sistema | o mesmo, com o form espelhado no árabe |
| Botão com `accessible_name` (o relatório do Tabernáculo, C29) | Narrador e NVDA, TalkBack, VoiceOver | o leitor diz o `accessible_name`, e não o texto ou o ícone do botão |

**Do relatório do Tabernáculo (12/11), fora do Windows**

- O vídeo por endereço (`file` com `https://`), W11: no Linux pelo `uridecodebin` do GStreamer, no
  macOS pelo `NSURL` do AVFoundation. No Windows foi conferido pelo Media Foundation.
- A foto do `snapshot` com o WebView (W10) é só do Windows (o `PrintWindow`); no Linux e no macOS a
  foto continua sem o que o navegador pinta.
- O `auto_size` do form (L07) pede o tamanho à janela: conferir que no macOS e no Linux a janela
  acompanha.

- **Arrastar para uma janela do app por cima de outra (macOS e Linux):** no Windows, o destino é
  a janela à vista no ponto (17/11). Nos outros sistemas, o Aylis só procura outra janela quando
  o mouse sai da janela de origem; conferir se uma janela pequena por cima de uma grande recebe
  o soltar e, se não, achar a ordem das janelas em cada sistema.
- **`MediaSession` (21/11):** no Windows, com um app tocando, o quadro do volume (as teclas de
  volume ou de mídia) mostra o título, o artista e a capa, e play, pausa, próxima e anterior
  controlam o app (o SMTC compila, mas não foi visto na tela). No Linux (a VM com o Fedora), o
  painel do GNOME e o `playerctl metadata` mostram o mesmo (o MPRIS passou no D-Bus do WSL, sem
  painel). No navegador, o controle de mídia do Chrome. No macOS, no iOS e no Android ainda não
  há implementação.
