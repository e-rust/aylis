# Renderizadores no Windows: Skia x Direct2D

O Aylis desenha por um trait (`Canvas`). Os controles não sabem quem está por baixo. No
Windows há três opções:

| Renderizador | Feature de `aylis-platform-winit` | O que é |
|---|---|---|
| **Skia GPU** | `gpu` (padrão) | Skia sobre OpenGL |
| **Skia CPU** | `cpu` (padrão) | Skia rasterizando na CPU; os pixels vão para a janela com `softbuffer` |
| **Direct2D** | `d2d` | Direct2D (acelerado pela GPU) + DirectWrite (texto) + WIC (imagens), que já vêm no Windows. O executável não leva biblioteca de desenho |

```toml
# Só Direct2D: o Skia nem entra no executável.
aylis-platform-winit = { path = "...", default-features = false, features = ["d2d", "clipboard"] }
```

Com as duas famílias compiladas, `Options { renderer: Renderer::D2d, .. }` ou
`AYLIS_RENDERER=d2d` escolhe o Direct2D (e `cpu`/`gpu` escolhem o Skia). Só com `d2d`,
o modo automático já é o Direct2D.

## Resultado

Medido em 25/10/2026: Intel Core i9-9900KS, AMD Radeon RX 9070 XT, Windows 11, release,
janela de 1280 × 820 (escala 150%). Cada número é a média de duas execuções.

### Tamanho do executável

| Exemplo | Skia (cpu + gpu) | Direct2D | Diferença |
|---|---:|---:|---:|
| `modern` (componentes modernos, grafo, conversa, gráficos) | 19,29 MB | **2,22 MB** | −88% |
| `components` (abas, campos, listas, gráficos, diálogos) | 19,36 MB | **2,29 MB** | −88% |
| `grid` (DataGrid com 1 milhão de pedidos) | 19,26 MB | **2,18 MB** | −89% |

O Skia (com o ICU para o texto) é cerca de 17 MB do executável. Com o Direct2D, o
executável leva só o código do app e do Aylis.

### Velocidade (tempo de um quadro inteiro: pintura + apresentação)

| Tela | Skia CPU | Skia GPU (OpenGL) | Direct2D |
|---|---:|---:|---:|
| `modern` | 6,24 ms | **2,19 ms** | 2,91 ms |
| `components` | 2,60 ms | 0,92 ms | **0,59 ms** |
| `grid` (DataGrid) | 2,25 ms | 1,03 ms | **0,48 ms** |
| `catalog` (Material 3 e iOS 26, com vidro) | — | 1,81 ms | 2,74 ms |
| `gallery` (componentes do Vuetify) | — | 7,79 ms | **3,22 ms** |

Quadros por segundo equivalentes, no `modern`: Skia CPU ~160, Direct2D ~345, Skia GPU
~455. (Números do Direct2D e do Skia GPU atualizados com o alvo novo do Direct2D, abaixo.)

### O alvo do Direct2D na janela

O Direct2D desenha na janela por **Direct3D 11 + swap chain "flip" + `ID2D1DeviceContext`**.
Antes, usava o `ID2D1HwndRenderTarget`. Com o alvo novo, ele compõe texturas de outro
desenhista, como o 3D (`docs/3d.md`), sem passar pela CPU. Ele também ficou mais rápido.
Junto, as paradas de gradiente passaram a ficar guardadas, em vez de recriadas a cada
quadro:

| Tela | Alvo antigo | Alvo novo + cache de gradientes |
|---|---:|---:|
| `components` | 0,69 ms | 0,59 ms (−15%) |
| `grid` | 0,59 ms | 0,48 ms (−19%) |
| `modern` | 2,98 ms | 2,91 ms (−2%) |
| `catalog` | 4,63 ms | 2,74 ms (−41%) |
| `gallery` | 4,05 ms | 3,22 ms (−20%) |

`AYLIS_D2D_TARGET=hwnd` volta ao alvo antigo (para comparar); `AYLIS_D2D_DEBUG=1` diz qual
foi usado. Se o Direct3D 11 não abrir, o alvo antigo entra sozinho. Também deixaram de
ficar guardadas as imagens que já vêm em pixels (quadros de vídeo, do 3D), como no Skia:
antes, um vídeo longo enchia a memória. Os três estão muito acima dos 60 quadros por segundo de um monitor comum.

- O **Direct2D** é o mais rápido em telas típicas de sistema (campos, listas, grades:
  muito texto e retângulos): metade do tempo do Skia GPU no DataGrid.
- O **Skia GPU** ganha em telas muito carregadas de formas (o `modern` tem milhares de
  pontos na grade do NodeGraph, muitas sombras e curvas), porque agrupa melhor os
  desenhos na GPU.
- O **Skia CPU** é o mais lento, mas não depende de GPU nem de driver.

### Memória do processo (depois de 600 quadros)

| Tela | Skia CPU | Skia GPU (OpenGL) | Direct2D |
|---|---:|---:|---:|
| `modern` | 16 MB | 349–573 MB | 60–72 MB |
| `components` | 13 MB | 302–476 MB | 47–55 MB |
| `grid` | 12 MB | 297–471 MB | 45–58 MB |

(Memória privada. A faixa é entre as duas execuções: a primeira de cada sessão costuma
ser maior pelos caches do driver.)

- O **Skia GPU** gasta centenas de MB: é o driver OpenGL (contexto, cache de shaders,
  buffers), não o Aylis.
- O **Direct2D** fica em ~45–70 MB: o dispositivo Direct3D por baixo dele.
- O **Skia CPU** é o mais econômico em memória, porque não abre dispositivo gráfico
  nenhum.

### Fidelidade

As duas capturas do `modern` (Skia CPU e Direct2D) diferem em **0,99%** dos pixels (só
**0,11%** com diferença grande), quase tudo no antialiasing e no ajuste das fontes. A
quebra de linha e as reticências usam a mesma regra (`aylis_core::text`), então os textos
quebram e cortam nos mesmos lugares.

Como o Direct2D faz o que não tem pronto:

- **sombra:** a forma é desfocada uma vez com o efeito Gaussian Blur (o mesmo sigma do Skia)
  e guardada pronta; nos quadros seguintes é só uma imagem. Fica igual à do Skia (0,2% dos
  pixels diferem, no antialiasing) e mais rápida que a aproximação antiga (−9% a −17% nas
  telas `gallery`, `catalog` e `modern`);
- **gradiente cônico** (`Brush::Sweep`): pintado numa imagem do tamanho da forma (até 1024
  pixels de lado) e usado como pincel, também guardado;
- as capturas (`AYLIS_SNAPSHOT`) são desenhadas uma segunda vez fora da tela, na CPU,
  porque o alvo da janela não devolve os pixels.

## Escolha no projeto

Cada projeto escolhe o renderizador **por plataforma**. Os projetos novos vêm com o
recomendado: **Direct2D no Windows** e Skia automático (GPU, senão CPU) no Linux, macOS
e Android. No iOS, por enquanto, só existe o Skia CPU.

- **Na IDE:** ao criar o projeto (Novo projeto) e depois em Projeto → Opções. Cada
  escolha mostra o que esperar: tamanho do executável, tempo de quadro e memória
  medidos.
- **No `.aylisproj`:** `"render": { "windows": "d2d", "linux": "auto", "macos": "auto",
  "android": "auto", "ios": "cpu" }`. Valores: `d2d`, `gpu`, `cpu`, `auto`.
- **No `Cargo.toml`:** a IDE escreve uma dependência do `aylis` por plataforma
  (`[target.'cfg(windows)'.dependencies]` com `features = ["d2d"]`, e assim por diante),
  entre os marcadores `# aylis:render` e `# aylis:render-end`. Cada sistema compila só o
  renderizador dele: o executável do Windows não leva o Skia, e o do Linux não leva o
  Direct2D.
- **Ao abrir:** o app usa a escolha da plataforma em que está rodando
  (`Application::options`). `AYLIS_RENDERER` ainda força outro, para testes.

Um projeto novo, com um form de Label, Edit, CheckBox e Button, compilado em release no
Windows: **1,97 MB** com o padrão (Direct2D), contra 19,02 MB com o Skia. Ao abrir, a
medição (`AYLIS_BENCH`) confirma `renderizador=Direct2D`.

Projetos criados antes dessa opção continuam com o Skia em todas as plataformas, como
eram. Para mudar, escolha em Projeto → Opções: se o `Cargo.toml` usa
`default-features = false`, a IDE acrescenta o bloco por plataforma.

## Qual escolher

| Situação | Recomendação |
|---|---|
| App de sistema para Windows (cadastros, grades, relatórios) | **Direct2D**: executável 8× menor, o mais rápido nessas telas, memória moderada |
| Visual muito carregado (gráficos densos, editores de nós, animações) | Skia GPU, se os ~300 MB de memória do driver não forem problema |
| Máquina sem GPU, área de trabalho remota, memória curta | Skia CPU |
| Multiplataforma (Linux, macOS, Android, iOS) | Skia: o Direct2D só existe no Windows |

O app pode levar o Skia e o Direct2D juntos e escolher na hora (`AYLIS_RENDERER`), mas o
tamanho do executável só cai quando o Skia fica de fora.

## Como medir de novo

A plataforma tem um modo de medição: `AYLIS_BENCH=N` redesenha N quadros sem parar e sem
esperar o monitor, depois escreve a média, p50 e p95 do tempo de cada quadro e a
memória do processo, e fecha.

```bash
cargo build --release -p aylis-platform-winit --features d2d --example modern
```

```bash
AYLIS_RENDERER=d2d AYLIS_BENCH=600 target/release/examples/modern.exe
```

Para medir todos os exemplos de uma vez e comparar com uma medição anterior:

```bash
aylis-tool bench medicoes/depois.json --compare medicoes/base.json
```

Troque `d2d` por `cpu` ou `gpu` para os outros. Para comparar o tamanho, compile uma vez
com as features padrão e outra com `--no-default-features --features d2d,clipboard`.

Observação sobre a GPU: o tempo medido é o da CPU para montar e enviar o quadro. A GPU
trabalha em paralelo. Sem vsync, o envio continua só enquanto a GPU acompanha, então o
número é um bom indicador de quanto sobra por quadro.

## Escolher a placa de vídeo

Num PC com mais de uma placa (a integrada e a dedicada de um notebook), o projeto escolhe
qual usar em Opções do projeto → **Placa de vídeo** (`gpu` no `.aylisproj`):
- `auto` (o padrão): a que o sistema dá;
- `alto-desempenho` ou `economia`: a dedicada ou a integrada;
- o nome (ou parte dele) de uma placa: `Radeon`, `RTX 4060`.

A variável `AYLIS_GPU` (mesmos valores) passa por cima do projeto, para testar. A escolha
vale para o Direct2D (o adaptador do DXGI), para o 3D (`wgpu`, com `aylis_3d::adapters()`
listando as placas) e, no Linux, para o Mesa (`DRI_PRIME`). A linha da medição
(`AYLIS_BENCH`) mostra a placa usada. O código fica em `aylis_core::gpu` (`GpuChoice`).

## Como o quadro chega à janela (Windows)

Com o Direct2D, o projeto escolhe em Opções do projeto → **Apresentação (Windows)**
(`present` no `.aylisproj`) como o quadro desenhado chega à janela:
- vazio (o padrão), **Janela**: a swap chain ligada à janela, como nos jogos. Em tela cheia
  o Windows manda o quadro direto ao monitor ("independent flip"), com a menor latência, e o
  quadro pode trocar sem esperar o monitor (tearing). Ao virar tela cheia, o Windows e o
  driver trocam o modo de exibição: medido no NoOBS (04/11/2026, Radeon com FreeSync), o
  mouse parava por quase 1 segundo, com o app recebendo os eventos normalmente;
- `composition`, **DirectComposition**: a swap chain vira uma camada que o compositor do
  Windows (DWM) junta à janela, como nos navegadores, no Electron e no WebView2. Sempre
  composto, sem tearing; a tela cheia não troca o modo de exibição (no NoOBS, o mouse deixou
  de parar). Não tira o aviso do Radeon Software em tela cheia.

Se o DirectComposition não abrir, o Aylis cai para a swap chain da janela. Com os
diagnósticos ligados (o app com `dev`), `AYLIS_PRESENT=composition` ou `window` passa por
cima do projeto. O código fica em `aylis_core::gpu` (`PresentMode`) e no `aylis-d2d`
(`compose_into`).
