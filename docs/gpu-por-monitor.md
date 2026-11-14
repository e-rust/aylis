# A placa de vídeo de cada janela (seguir o monitor)

Pedido do autor em 11/11/2026. A máquina dele tem seis monitores: quatro na placa de vídeo
dedicada (o limite dela) e dois na saída da placa-mãe (a GPU integrada do processador). Uma
janela criada num monitor de uma placa e arrastada para um monitor da outra trava o sistema,
no navegador e em qualquer app que desenhe na GPU.

## Por que trava

Uma janela tem uma swap chain, e a swap chain é de um dispositivo Direct3D de **uma** placa.
Quando a janela está num monitor ligado à outra placa, o Windows copia cada quadro pronto de
uma placa para a outra (pela memória do sistema). Com janelas grandes, em monitores de alta
resolução e a 60 quadros por segundo, essa cópia satura o barramento e o DWM, e tudo engasga.

Não dá para uma janela ser pintada metade por uma placa e metade pela outra: com a janela entre
dois monitores, a placa dela desenha tudo e o Windows copia a parte que está no outro monitor.
O que dá para fazer é a janela **mudar de placa** quando passa para o outro lado, e aí não há
cópia nenhuma enquanto ela está inteira num monitor.

O navegador não deixa escolher: o Chrome e o Edge abrem a GPU uma vez, para o processo
inteiro. Num app nativo, com o Direct2D, a escolha é nossa.

## O que foi feito

### A regra `monitor`

`aylis_core::gpu::GpuChoice` ganhou `FollowMonitor` (gravado como `monitor`; também aceita
`seguir-monitor` e `follow-monitor`). Vale em dois lugares:

- **No projeto** (`"gpu": "monitor"` no `.aylisproj`, ou Opções do projeto → Placa de vídeo →
  "Seguir o monitor (a placa de cada tela)"): todas as janelas seguem o monitor.
- **No form**, pela propriedade nova `gpu` (categoria Comportamento): vazia, vale a do projeto;
  `monitor`, `alto-desempenho`, `economia` ou parte do nome de uma placa (`radeon`, `intel`)
  valem só para aquele form. `aylis_core::gpu::choice_for` junta as duas.

Com os diagnósticos ligados, `AYLIS_GPU=monitor` também liga a regra para o processo.

### Como a janela troca de placa (Windows, Direct2D)

Em `crates/aylis-d2d/src/lib.rs`:

1. `adapter_for(choice, hwnd)` substitui o antigo `chosen_adapter()`. Com `FollowMonitor`, pega
   o monitor com a maior parte da janela (`MonitorFromWindow` com `MONITOR_DEFAULTTONEAREST`)
   e procura a placa que tem uma saída ligada a ele (`EnumAdapters1` → `EnumOutputs` →
   `DXGI_OUTPUT_DESC::Monitor`). Por isso o Cargo.toml do crate ganhou a feature
   `Win32_Graphics_Gdi` do `windows`.
2. `D2dRenderer::hwnd_target_for(hwnd, largura, altura, vsync, choice)` cria o alvo na placa da
   regra. O `hwnd_target` de antes continua e usa a regra do projeto.
3. O `HwndTarget` guarda a regra e o monitor da última conferência. `follow_monitor()`, no
   mesmo monitor, custa só um `MonitorFromWindow`. Se mudou e a placa do monitor novo tem outro
   LUID, recria o alvo nela. Monitor sem placa conhecida (sessão remota, projetor sem driver):
   fica onde está.
4. `set_choice()` troca a regra (a propriedade `gpu` mudou com o app rodando) e recria se a
   placa pedida não é a atual.
5. `rebuild()` é o caminho único de recriação, usado também pelo dispositivo perdido. Ele
   **solta antes** a swap chain e o alvo do DirectComposition da placa antiga
   (`SetTarget(None)`, `ClearState` e `Flush` no contexto do Direct3D): o Windows aceita uma
   swap chain "flip" e um alvo de composição por janela, e criar os novos com os antigos
   vivos falha. Se a recriação falhar, o alvo fica em `Kind::Released` e o próximo quadro
   tenta de novo, como num dispositivo perdido.
6. `aylis_d2d::adapters()` lista as placas pelo nome, sem o renderizador por software. A
   plataforma só diz como listar (`aylis_core::gpu::set_adapter_list`); a lista sai na primeira
   vez que alguém chama o `gpu_adapters()` e fica guardada (abrir a janela não custa nada).
7. `AYLIS_GPU_SWITCH=warp` (diagnóstico): cada troca de monitor alterna entre a placa dele e o
   WARP, o renderizador por software do Windows, que o DXGI lista como outra placa (outro LUID).
   É a troca de verdade numa máquina com uma placa só.

O que já existia e foi aproveitado: os bitmaps da GPU ficam no `DeviceCache` de cada alvo e
as imagens decodificadas ficam na CPU, no renderizador; então, ao recriar em outra placa, o
quadro seguinte sobe tudo de novo sem perder nada. O texto (DirectWrite) não depende de placa.
O 3D (`aylis-3d`) guarda um motor por dispositivo (`with_engine`, pela `GpuDevice` com o LUID):
na placa nova ele abre outro motor e sobe os modelos de novo, sozinho.

### Na plataforma (`crates/aylis-platform-winit`)

- A janela nasce na placa da regra do form (`App::gpu_choice` → `create_backend` →
  `d2d::create`), já no monitor em que abre.
- `WindowEvent::Moved` e `WindowEvent::Resized`/`ScaleFactorChanged` (o `move_to_monitor`,
  maximizar, encaixar com Win+Shift+seta) chamam `App::follow_monitor`, que confere a placa.
- Arrastando com o mouse, a troca espera soltar: a subclass da janela (`win32.rs`) marca o
  `WM_ENTERSIZEMOVE` e, no `WM_EXITSIZEMOVE`, pede a conferência. Assim não há recriação no
  meio do arraste nem vaivém sobre a divisa entre os monitores.
- O `WM_DISPLAYCHANGE` (monitor ligado, desligado ou trocado de placa) confere de novo, mesmo
  com a janela no monitor de antes.
- A janela que fecha sai do `Ui` (`set_form_gpu(form, None)`).
- A propriedade `gpu` mudando com o app rodando vira `WindowCommand::UpdateGpu` (novo), tratado
  por `App::update_gpu`.
- Quando troca: a janela pede um quadro novo, o relatório de uso (`set_platform_info("gpu")`)
  é atualizado e o form recebe o evento.

### O que o programador usa (em todas as linguagens)

Tudo é classe + propriedade + valor, então vale no JSON, no protocolo, na API C e nos 7
clientes:

| Nome | Tipo | O que é |
|---|---|---|
| `gpu` | propriedade (texto) | A regra da placa do form (vazia: a do projeto) |
| `on_gpu_change` | evento | A janela trocou de placa; o argumento (`Text`) é o nome da placa nova |
| `gpu_adapter()` | método → texto | A placa em que a janela desenha agora (vazio: não se sabe) |
| `gpu_adapters()` | método → texto | As placas da máquina, separadas por `;` |
| `span_monitors` | propriedade (bool) | O form maior que um monitor vira uma janela por monitor (veja abaixo) |

No núcleo, `Ui::form_gpu`, `Ui::set_form_gpu`, `Ui::gpu_adapters` e `Ui::set_gpu_adapters`
guardam o que a plataforma informa (o `on_gpu_change` sai do `set_form_gpu` quando a placa
muda, não na primeira vez).

Exemplo em Rust, um form que fica sempre na placa do monitor e avisa a troca:

```rust
ui.set_prop(form, "gpu", PropValue::String("monitor".into()))?;
ui.on(form, "on_gpu_change", |ui, _, args| {
    if let EventArgs::Text(placa) = args {
        aylis_core::stderr_line!("agora desenhando em {placa}");
    }
});
```

### Na IDE

- Opções do projeto → Placa de vídeo ganhou a quarta escolha fixa, "Seguir o monitor (a placa
  de cada tela)", traduzida nas 11 línguas.
- O rótulo da propriedade `gpu` ("Placa de vídeo") entrou no `properties.tsv`.
- O catálogo (`resources/components.json`) ganhou `gpu`, `gpu_adapter()` e `gpu_adapters()` no
  `Form`, escritos à mão na ordem do `properties!`; rode `aylis-tool update-catalog` para
  conferir que sai igual.

## Linux

O problema existe no Linux também, porque é de hardware: o quadro é feito numa placa, e o
monitor na outra recebe uma cópia. Mas quem copia muda:

- **X11:** os monitores da segunda placa funcionam por *reverse PRIME*: o servidor desenha a
  tela inteira na placa principal e copia para a outra, para todos os apps. Não há o que o app
  fazer; ali a regra `monitor` fica na placa do processo.
- **Wayland:** o compositor (GNOME, KDE) recebe o buffer de cada janela e copia para a placa
  do monitor quando o buffer é de outra. Aqui a janela pode, sim, desenhar na placa certa.

O glutin abre o OpenGL na placa do processo (o Mesa escolhe uma, pelo `DRI_PRIME`). Para uma
janela desenhar em outra, `crates/aylis-platform-winit/src/gpu_linux.rs`:

1. **Abre o OpenGL na placa escolhida:** um `EGLDisplay` do Wayland naquela placa
   (`eglGetPlatformDisplay` com `EGL_DEVICE_EXT`, a extensão `EGL_EXT_explicit_device` do
   Mesa), o contexto, a janela EGL (`wl_egl_window`) e a superfície. As funções do EGL vêm do
   mesmo `libEGL` do glutin, pelo `eglGetProcAddress`; o display fica aberto (o EGL devolve o
   mesmo para a mesma placa, e outra janela pode usá-lo).
2. **Sabe a placa do monitor:** o feedback do `linux-dmabuf` v4 da superfície da janela
   (`get_surface_feedback`). A placa da primeira faixa (a de maior preferência) é a do
   monitor em que a janela está; o compositor manda de novo quando ela passa para um monitor
   ligado a outra placa. É uma fila própria na conexão do winit (o `wl_display` e o
   `wl_surface` vêm do `raw-window-handle`): os eventos chegam quando o winit lê o socket, e a
   conferência a cada volta do laço só entrega o que já chegou (não custa nada parado).
3. **Liga a placa (`dev_t`) ao EGL:** o nó principal e o de renderização de cada placa do EGL
   (`EGL_EXT_device_query`), comparados pelo `st_rdev`.

No `GpuBackend` (`gpu.rs`), o contexto é o do glutin (a placa do processo) ou o de uma placa
escolhida (`gpu_linux::DeviceGl`). A troca abre o novo primeiro (se falhar, a janela continua
como estava), solta o Skia com o contexto antigo atual, solta o contexto e cria o Skia no
novo. As imagens do Skia são refeitas no quadro seguinte, como na troca do Windows.

Vale para as regras:

- `monitor`: a placa que o compositor prefere para a janela;
- parte do nome (`radeon`, `renderD129`, `llvmpipe`): a placa do EGL com esse nome, só para
  aquele form;
- `alto-desempenho` e `economia`: continuam valendo para o processo inteiro, como antes
  (`DRI_PRIME`, no `mesa_gpu`).

O `gpu_adapters()` lista as placas que o EGL enxerga; o `gpu_adapter()` devolve o
`GL_RENDERER` do contexto da janela; o `on_gpu_change` dispara quando ele muda. No X11, num
compositor sem o `linux-dmabuf` v4 ou com o Mesa antigo (sem o `EGL_EXT_explicit_device`), a
janela fica na placa do processo, sem erro. No macOS, a troca entre a integrada e a dedicada
dos MacBooks já é do sistema (`NSSupportsAutomaticGraphicsSwitching`).

### Testado no WSL (11/11)

No Ubuntu 26.04 do WSL, com o Wayland do WSLg e o Mesa 26: o OpenGL padrão é a Radeon (pelo
D3D12 do Mesa, com `GALLIUM_DRIVER=d3d12`) e o EGL lista o llvmpipe como outra placa.

| Caso | Resultado |
|---|---|
| `--toggle 4` (a `gpu` do form entre `llvmpipe` e vazia) | 4 trocas de contexto: display, contexto e Skia novos a cada vez; o quadro depois da troca sai igual (conferido pela captura) |
| O feedback do `linux-dmabuf` | o compositor do WSLg não tem a versão 4: a janela fica na placa do processo, como esperado |

Limites deste teste: o `GALLIUM_DRIVER` vale para o processo inteiro, então o display do
llvmpipe também desenhou pelo D3D12 (o nome da placa não mudou e o `on_gpu_change` não
disparou); e sem o feedback do compositor, a troca pelo monitor não foi exercitada. Os dois
pedem um Linux de verdade com duas placas e o GNOME ou o KDE no Wayland
([testes em dispositivos](testes-em-dispositivos.md)).

## Um form ocupando vários monitores (`span_monitors`)

Decidido em 14/11. Com seis monitores em duas placas, uma janela gigante cobrindo todos volta a
ter a cópia entre as placas. Por isso, com a propriedade `span_monitors` do form ligada
(categoria Layout, desligada por padrão), o form maior que um monitor vira **uma janela por
monitor**, cada uma na placa do seu, todas com a principal como dona (a barra de tarefas e o
Alt+Tab mostram uma só). Com `window_state = full_screen`, o form cobre todos os monitores (um
telão). Desligada, nada muda.

O `Ui` continua com um form só. As peças:

| Onde | O que faz |
|---|---|
| `aylis_core::span` | `split(form, monitores)`: o retângulo do form na tela (pixels físicos) dividido pelos monitores, um pedaço por monitor que ele toca, de cima para baixo e da esquerda para a direita; a parte fora de todos não aparece e um monitor espelhado não ganha outra janela. `Piece::to_form` leva um ponto da janela ao form (a origem do pedaço mais o ponto, na escala); `largest` escolhe o pedaço da janela principal |
| `Ui::set_form_pieces` / `form_pieces` | A plataforma diz os pedaços (nas coordenadas do form) |
| `Ui::paint_piece(form, canvas, pedaço, áreas, início)` | Pinta só o pedaço, com o canvas andando `-pedaço.x, -pedaço.y`; só os controles que tocam o pedaço (ou as áreas mudadas nele) são pintados. Os pedidos de quadro (o cursor piscando, uma animação) dos pedaços do mesmo quadro se somam |
| `Ui::popup_area(form, ponto)` | Onde um menu ou uma lista que nasce no ponto pode ficar: o form ou o pedaço que contém o ponto. O `PopupMenu`/`MainMenu`, o `ComboBox`, o `DateEdit`, o `TimeEdit`, o `DateRangeEdit` e o `Popover` usam: abrem dentro do monitor em que nascem, sem ficar partidos entre duas janelas |
| `crates/aylis-platform-winit/src/span.rs` | As janelas: abre as extras (sem moldura, fora da barra de tarefas, com a principal como dona, na placa do monitor delas), refaz os pedaços e volta a uma janela |

Na plataforma (Windows):

- **Abrir:** a janela abre como sempre; na volta seguinte do laço, se o form (o tamanho dele na
  escala do monitor, a partir do canto da janela) passa de um monitor, a principal perde a
  moldura e fica com o pedaço maior, e as outras abrem. O form fica do tamanho de todos os
  pedaços juntos.
- **Placa:** com a regra `gpu` vazia (a do projeto, automática), cada janela segue a placa do
  monitor dela (`monitor`); com uma placa pedida no form, todas ficam nela.
- **Pintura:** um quadro do form pinta todos os pedaços juntos, com um `take_frame` só: cada
  janela pinta a parte das áreas mudadas que cai nela (as cópias da rolagem viram área, porque
  a origem pode estar em outra janela); um pedaço sem nada mudado não pinta. O sistema pedindo
  uma janela (apareceu, mudou de placa) pinta só ela, inteira.
- **Entrada:** o mouse, o toque e a roda de cada janela vão ao form com a origem do pedaço; o
  mouse preso numa janela durante um arrasto, já sobre a vizinha, continua certo (a coordenada
  passa da borda). O mouse passando de um pedaço para o outro não sai do form (o
  `pointer_leave` só quando sai de todos), e o foco passando de um para o outro não dispara o
  `on_deactivate`/`on_activate`. O teclado vai ao form, de qualquer janela.
- **Escala:** todos os pedaços desenham com a escala do monitor da janela principal (num monitor
  com outra escala, o form aparece com a da principal, sem mudar de tamanho no meio). A
  sugestão de tamanho do sistema ao passar para um monitor de outra escala é recusada.
- **Mudanças:** desligar a propriedade com o app aberto volta a uma janela (com a moldura do
  `border_style` e o lugar do form); `width`/`height`, `move_to_monitor`, a posição pelo
  código, a tela cheia, o zoom e os monitores mudando (ligado, desligado, outra resolução)
  refazem os pedaços. Maximizar, arrastar e redimensionar pela borda não valem para um form em
  pedaços.
- **Diagnóstico:** `AYLIS_SPAN_SPLIT=N` conta cada monitor como N, lado a lado, para ver o
  caminho inteiro numa máquina com um monitor só (todas as janelas na mesma placa).

O que não está coberto:

- **Linux e macOS:** a propriedade não tem efeito (o form fica numa janela). Falta conferir as
  janelas sem moldura posicionadas em cada monitor e a dona (no X11, `WM_TRANSIENT_FOR`; no
  Wayland o app nem escolhe onde a janela fica, então o caminho seria o compositor; no macOS, as
  janelas filhas do `NSWindow`). A pintura, a entrada e os menus são do núcleo e já valem.
- **O leitor de tela** só vê a janela principal (a árvore vai com as coordenadas dela), e os
  navegadores embutidos (`WebView`) ficam na principal.
- **O `remember_placement`** não guarda um form em pedaços (a posição guardada seria a de um
  pedaço).
- **Uma janela de arrasto (a aba que vira janela) solta sobre outro form** continua certa, mas
  um arrasto de um form para outro form em pedaços usa a janela sob o mouse com a origem dela.
- **Monitores que se sobrepõem em parte** (raro) dão janelas sobrepostas, com o mesmo desenho.

### Como conferir

Sem vários monitores (testes):

- `cargo test -p aylis-core --lib span` e `--test span`: a divisão (dois monitores, o da
  esquerda com `x` negativo, o telão 2×3 com um sétimo e o vão, o espelhado), o ponto da janela
  no form, o menu no pedaço e a pintura de um pedaço só com os controles dele.
- `cargo test -p aylis-controls --test span_monitors`: cada pedaço pintado com o deslocamento
  (`Ui::paint_piece`) sai igual ao recorte da pintura do form inteiro, inteiro e por parte (o
  mouse em cima e um deslocamento de cada controle registrado), no Skia e no Direct2D, a 100%
  e a 150% (as divisas em pixel quebrado no form); o clique dado na janela da direita chega ao
  botão do form; a lista do `ComboBox` no fim do monitor de cima abre para cima, dentro dele.
  O Direct2D sai igual até a borda; no Skia, um desenho cortado pela borda da janela passa por
  outro caminho de rasterização (como na pintura por parte, mas sem a margem), e a beirada
  anti-aliasing de um arco cortado pela divisa muda até 65 tons: o teste aceita 96 no Skia
  (resto de verdade muda mais de 200).
- `AYLIS_SPAN_SPLIT=2 cargo run -p aylis-platform-winit --example span_monitors --features d2d
  -- --auto` (com `AYLIS_RENDERER=gpu`, o Skia na GPU; `--full`, o telão): duas janelas de
  verdade, o clique mandado a cada uma chega ao botão dela, e desligar e religar a propriedade
  volta a uma janela e refaz os pedaços. Conferido em 14/11 num monitor 3840 × 2160 a 150%, no
  Direct2D e no Skia GPU; com `AYLIS_SNAPSHOT`, as imagens de cada janela saem com o pedaço
  certo.

Com vários monitores, à mão: [testes em dispositivos](testes-em-dispositivos.md).

## Como testar

O exemplo `gpu_monitor` abre uma janela com `gpu = monitor`, leva-a do monitor 0 para o 1 e
de volta, e imprime cada `on_gpu_change`:

```text
set AYLIS_GPU_SWITCH=warp
cargo run -p aylis-platform-winit --example gpu_monitor --features d2d -- --auto 3
```

Conferido em 11/11 numa placa só (Radeon RX 9070 XT) com dois monitores:

| Caso | Resultado |
|---|---|
| Sem o `AYLIS_GPU_SWITCH` | nenhuma troca (os dois monitores são da mesma placa) |
| `AYLIS_GPU_SWITCH=warp`, swap chain | 5 trocas em 6 movimentos (o primeiro já estava no monitor), alternando Radeon e WARP, cada uma com o `on_gpu_change` e o nome certo |
| O mesmo com `AYLIS_PRESENT=composition` | 5 trocas, sem cair para o alvo antigo (`ID2D1HwndRenderTarget`) |

Falta conferir à mão: arrastar a janela com o mouse (a troca ao soltar) e, na máquina com as
duas placas:
   - abrir o app num monitor da dedicada e arrastar a janela para um da integrada: não deve
     travar, e a janela pisca uma vez ao passar da metade (a recriação);
   - `AYLIS_D2D_DEBUG=1` mostra o alvo criado; o relatório de uso (`gpu`) mostra a placa nova;
   - um handler no `on_gpu_change` recebe o nome da placa;
   - com `AYLIS_PRESENT=composition` (DirectComposition), repetir: é o caso em que soltar o
     alvo antigo antes importa;
   - um `Model3D` e um `VideoPlayer` na janela, arrastando: o 3D deve continuar (abre outro
     motor). O vídeo precisa ser conferido: se o decodificador entrega texturas compartilhadas
     de uma placa, a janela na outra não consegue abri-las (`OpenSharedResource1` falha) e o
     quadro do vídeo some até o decodificador acompanhar.
- Antes de tudo, se trava mesmo pela cópia entre as placas: arrastar um app desenhado na CPU
  (o Aylis com `AYLIS_RENDERER=cpu`, o Bloco de Notas) para um monitor da outra placa. Se
  travar também, a causa é do driver ou do DWM, e trocar de placa não resolve.
