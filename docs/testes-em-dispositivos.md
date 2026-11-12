# Testes que dependem de Mac ou de celular

O que já foi feito no aylis e só pode ser conferido (ou terminado) com um Mac ou um celular
de verdade. O emulador do Android cobre muito, mas não tudo: o que está aqui ou não roda
nele ou foi conferido só nele. Ao testar um item, anote o resultado no
[histórico de decisões](historico-decisoes.md) e risque daqui.

## Mac (macOS e iOS)

Sem um Mac não dá para compilar o código Objective-C/Swift das dependências da Apple nem
rodar no simulador do iOS. Para o iOS, o `build-ios.sh` monta o app.

| O quê | Situação | Como conferir |
|---|---|---|
| Rodar no macOS (janela, renderizadores, teclado, mouse, touchpad) | compila no CI para macOS; nunca rodou | um exemplo qualquer, `cargo run --example components` |
| Vídeo e áudio do AVFoundation depois da revisão de 31/10 (decodificador aberto na thread de vídeo, trilha lida do arquivo aos poucos, `cpal` 0.18, conferência do tamanho da linha) | só revisado; o `avf.rs` não compila fora do Mac | exemplo `Player` com um MP4 e um MP3 |
| Rodar no iOS (janela, toque, teclado na tela) | compila para iOS; nunca rodou | `build-ios.sh` e o simulador |
| Teclado do iOS reduzindo a área segura (`UIKeyboardWillChangeFrameNotification`) | compila | um form com campos embaixo; o campo com foco sobe acima do teclado |
| Ícone, nome e bundle id do app no `build-ios.sh` (`NAME`, `ICON`, `BUNDLE_ID`) | escrito | gerar o app e olhar a tela inicial |
| 3D sem cópia no macOS (IOSurface) e no iOS (Skia Metal); Metal no iOS | escrito | exemplo `model3d`; conferir o FPS e o uso de CPU |
| WebView no macOS e no iOS (wry) | escrito | exemplo `browser`: carregar, mensagem, `eval` |
| Vídeo pela AVFoundation (macOS e iOS) | compila para iOS | um `VideoPlayer` com um MP4 |
| Câmera no macOS e no iOS (AVCaptureSession) | **falta fazer** | `CameraView`: ao vivo, foto, trocar de câmera |
| Acessibilidade no macOS e no iOS (VoiceOver pelo AccessKit) | compila | ligar o VoiceOver e navegar pelos controles; o exemplo `automation` |
| Pinça e giro do touchpad no macOS viram `on_gesture` | escrito | `Model3D` ou `Graph`: pinçar e girar |
| Corretor e sugestão de texto nativos (NSSpellChecker no macOS, UITextChecker no iOS) | **falta fazer** (hoje só com um motor posto pelo app) | `spellcheck` e `autocomplete` no Edit; exemplo `spellcheck` |
| Ícone da bandeja e notificações no macOS | escrito para o desktop | exemplos `tray` e `notification` |
| IDE em `.app` com ícones por tipo de arquivo (`aylis-tool ide-package --target macos`) | gerado e conferido por teste; nunca aberto no Finder | instalar o `.app` e ver o ícone de um `.aylisproj` e de um `.aylisform` |
| A barra de título própria no macOS (`border_style = custom`): a barra do sistema transparente, sem título nem botões, e o conteúdo por baixo, para manter os cantos arredondados e a sombra; com `window_corners = square`, sem moldura | escrito; não compilado para macOS | exemplo `titlebar`: os cantos, arrastar pela barra própria, redimensionar, e que os botões do sistema não aparecem |
| Tempo de CPU e amostragem da pilha no profiler, no macOS | só Windows (e o tempo no Linux) | profiler da IDE num app em Rust |
| `AppLink`, sensores e `BackgroundService` no iOS | **falta fazer** | `TesteAparelho` no simulador |
| `PushNotifications` no iOS e no macOS (APNs) | compila para iOS; o macOS não foi compilado; nunca rodou. A dúvida maior: o toque que abre o app fechado (o delegado da central é posto quando o aylis cria os serviços, talvez depois do fim da partida) | o app com a capacidade Push no App ID; mandar pelo APNs sandbox (ou arrastar um `.apns` no simulador) com o app aberto, em segundo plano e fechado |

## Celular de verdade (Android e iOS)

Conferido no ASUS ZenFone 5 (Android 9, Snapdragon 636) em 01/11/2026, com o
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
| Corretor e sugestão do aylis no celular (`spellcheck`, `autocomplete`) | o motor nativo do Android não foi feito; hoje só com um motor do app | exemplo `spellcheck` com um `WordList` |
| 3D (aquecimento, bateria) | no ASUS ZenFone 5 (Adreno 512, GLES 3.2) o 3D já sai certo; desempenho medido em 01/11 no `examples/Estresse3D` com o corte do brilho em 8 px: 57/57/55/39/26/22 quadros/s para 1/4/9/16/25/36 lingotes (CPU a 42–46 °C; com o corte em 16 px dava 57/56/58/45/31/27). Falta deixar alguns minutos para ver o aquecimento e a bateria | `examples/Estresse3D`, alguns minutos com 1 e com 36 lingotes | aba 3D do `examples/TesteAparelho` |
| Canal de desenvolvimento no celular (`adb reverse` num endereço local; IP da máquina sem cabo) | conferido no desktop e por teste | Executar no Android com o Inspetor do app e o hot reload |
| Publicação: AAB na Play Store (teste interno) e o APK assinado instalado à mão | o AAB foi validado, mas não subiu para a loja | criar o teste interno na Play Console |
| iOS no iPhone (tudo da seção Mac que vale para o iOS) | precisa de Mac e de um iPhone | o app do `build-ios.sh` instalado pelo Xcode |
| A Landing publicada no celular (11/11): a rolagem acompanhando o dedo; o 3D da documentação pelo WebGL num navegador sem WebGPU | a rolagem sem layout por passo foi medida só sem janela; o WebGL, no Edge sem janela (SwiftShader) com o `navigator.gpu` escondido | o site republicado, no Safari de um iPhone antes do iOS 26 e no Chrome de um Android; a página do `Model3D` na documentação |

## PC com outro hardware ou periférico

Feito e testado por código, mas a máquina do autor não tem o hardware para conferir até o fim:

| O quê | Por que | Como conferir |
|---|---|---|
| Imprimir pela impressora do sistema (`Report::print`, diálogo do Windows e GDI) | testado só o PDF e a visualização; a impressão real não saiu no papel | exemplo `report`, botão Imprimir, escolher uma impressora |
| Imprimir no Linux (PDF aberto pelo `xdg-open`) | só compilado | o mesmo exemplo no WSL ou num Linux |
| Vídeo pelo GStreamer depois da revisão de 31/10 (aberto na thread de vídeo, trilha lida aos poucos) e o `cpal` 0.18 (ALSA/PipeWire) | só revisado; não compilado para Linux nesta máquina | exemplo `Player` num Linux |
| Escolher a placa de vídeo com duas GPUs (`gpu` no projeto, `AYLIS_GPU`) | a máquina tem uma GPU só; conferido que a escolha pelo nome acha a placa certa | um notebook com GPU integrada e dedicada; a linha de bench mostra a placa usada |
| Seguir o monitor (`gpu` = `monitor` no projeto ou no form; [gpu-por-monitor.md](gpu-por-monitor.md)) | testado numa placa só, com o WARP no lugar da segunda (11/11); precisa de monitores ligados em placas diferentes | a máquina dos seis monitores: arrastar a janela da dedicada para a integrada, com e sem `AYLIS_PRESENT=composition`, com um `Model3D` e um `VideoPlayer` |
| Seguir o monitor no Linux (Wayland, `gpu_linux.rs`) | no WSL: a troca de contexto entre o D3D12 e o llvmpipe e o quadro depois dela; o compositor do WSLg não tem o `linux-dmabuf` v4 | um Linux com duas placas no GNOME ou no KDE (Wayland): `AYLIS_DEBUG_GL=1 cargo run -p aylis-platform-winit --example gpu_monitor` e arrastar a janela para um monitor da outra placa |
| Traçado de raios no Vulkan e no Metal | testado no DX12 (Windows) | exemplo `model3d` com "Traçado de raios" num Linux ou Mac com GPU que tenha ray query |
| FBX de verdade (Mixamo, Maya, Blender) | aguardando os arquivos do autor | exemplo `model3d` com o arquivo |
| Pacote da IDE no Linux (`ide-package --target linux`: `.desktop`, MIME, ícones) | gerado e conferido por teste | `instalar.sh` num Linux com desktop e abrir um `.aylisproj` pelo gerenciador de arquivos |
| Câmera no Linux | a webcam no WSL pede o `usbipd-win` | `CameraView` num Linux com webcam |
| `Gamepad` com um controle de verdade (Windows, Linux, navegador) | a máquina do autor não tem um controle ligado; o `gilrs` abre e lista vazio | ligar um controle de Xbox ou PlayStation, um form com `Gamepad`; `rumble` no Windows |
| Agente de depuração remota em outro PC (`debug-agent`, `agent-run`, `agent-dap`) | conferido com agente e cliente na mesma máquina | agente num servidor Windows ou Linux, pareado pelo código |

## Leitor de tela e entrada de idiomas (IME)

O aylis expõe a árvore de acessibilidade pelo AccessKit (no Windows, conferido pelo UI
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
