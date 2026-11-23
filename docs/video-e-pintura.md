# Vídeo e pintura: o que se configura e como medir

O Aylis vem com padrões medidos numa máquina só. Cada placa de vídeo, monitor e aplicação é
diferente, então tudo o que pesa na pintura e no vídeo pode ser ajustado. São três níveis, do
mais geral ao mais próprio:

- **no projeto** (`.aylisproj`, ou Projeto → Opções na IDE): configura o app inteiro, uma vez;
- **no form e nos componentes** (propriedades): valem para aquele form ou controle, por cima do
  projeto;
- **em execução**: o componente `RenderSettings` (paleta System) lê e troca o valor do app
  inteiro enquanto ele roda (uma tela de desempenho, um modo econômico); vale no quadro
  seguinte. O form com valor próprio continua com o dele.

E, fora dos três, **variáveis de ambiente**: só no desenvolvimento, com o app compilado em debug
pela IDE (a feature `dev`). Servem para testar outro valor sem mexer no projeto e ganham de
todos. Em release o app nunca lê variável de ambiente, mesmo com a `dev`.

O caminho é medir com o padrão, trocar um valor, medir de novo e só então gravar no projeto.

## No projeto

| Chave do `.aylisproj` | Onde na IDE | O que faz | Padrão |
|---|---|---|---|
| `render` (`"windows": "d2d"`, `"linux": "auto"`...) | Projeto → Opções | O renderizador de cada plataforma: `d2d`, `gpu` (Skia na placa), `cpu` (Skia na CPU) ou `auto` ([renderizadores.md](renderizadores.md)) | Direct2D no Windows, `auto` nas outras |
| `gpu` | Projeto → Opções | A placa quando há mais de uma: `alto-desempenho`, `economia`, parte do nome ou `monitor` (segue o monitor da janela; [gpu-por-monitor.md](gpu-por-monitor.md)) | a do sistema |
| `present` | Projeto → Opções | Como o quadro chega à janela no Windows: vazio (a swap chain da janela), `composition` (pelo compositor, como os navegadores) ou `composed` (sempre composto, fora do MPO: forms de overlay em tela cheia) | vazio |
| `image_cache.pixels_mb` | Opções do projeto → Memória | Megabytes de imagens em pixels (vídeo, câmera, prévias) guardadas na placa | 96 |
| `image_cache.files` | Opções do projeto → Memória | Quantas imagens de arquivo ficam prontas | 256 |
| `image_cache.layers_mb` | Opções do projeto → Memória | Megabytes das camadas guardadas (`cache_layer`, transições); `0` desliga | 128 |
| `image_cache.auto_layers` | Opções do projeto → Memória | As camadas automáticas nas transições e animações | ligado |
| `image_cache.frame_reuse_kb` | Opções do projeto → Memória → Reaproveitar quadros | Até quantos KB um quadro de câmera ou de vídeo copia os pixels no bitmap do quadro anterior, em vez de criar outro na placa (Direct2D); `0` desliga | 1024 |

## No form e nos componentes

| Propriedade | De quem | O que faz |
|---|---|---|
| `frame_rate` | Form | Quadros por segundo da janela (`0`: a taxa do monitor). Um telão não precisa de 144 quadros |
| `layer_cache` | Form | As camadas automáticas neste form: `project`, `on` ou `off` |
| `frame_reuse_kb` | Form | Até quantos KB um quadro de vídeo reaproveita o bitmap do anterior nesta janela: `-1` (o padrão) é o do projeto; `0` sempre cria. Só no Direct2D (o Skia e o navegador não reaproveitam) |
| `cache_layer` | todo controle | Guarda o desenho do controle como imagem enquanto ele anda ou esmaece |
| `gpu` | Form | A placa deste form: vazia, a do projeto; `monitor`, `alto-desempenho`, `economia` ou parte do nome ([gpu-por-monitor.md](gpu-por-monitor.md)) |
| `present` | Form | Como o quadro deste form chega à janela no Windows: `project` (o padrão), `window`, `composition` ou `composed` |
| `span_monitors` | Form | No Windows, um form maior que um monitor vira uma janela por monitor |
| `frames_ahead` | VideoPlayer | Quantos quadros o decodificador prepara à frente (`0`: 6) |
| `show_video` | VideoPlayer | Desligado, só o áudio (não decodifica a imagem para a tela) |
| `quality`, `resolution` | CameraView | O tamanho que a câmera entrega; menor é mais leve. A câmera já entrega no tamanho em que aparece |

## Em execução

O `RenderSettings` (não visual, paleta System) tem as mesmas chaves do `image_cache` do projeto,
como propriedades de execução: `pixels_mb`, `files`, `layers_mb`, `auto_layers` e
`frame_reuse_kb`. Ler diz o que vale agora (o que o projeto aplicou ou a última troca); trocar
vale no app inteiro já no quadro seguinte: baixar um limite solta o excedente no próximo uso, e
`auto_layers` repinta. Fora da faixa, o valor é ajustado (como o projeto faz ao abrir).

As propriedades não são gravadas no form: um form que gravasse os valores passaria por cima do
projeto toda vez que abrisse. No designer, trocar não mexe na IDE.

```rust
// Um modo econômico, escolhido pelo usuário numa tela de opções do app.
self.RenderSettings1.set_layers_mb(ui, 0);
self.RenderSettings1.set_auto_layers(ui, false);
self.RenderSettings1.set_frame_reuse_kb(ui, 0);
```

No Direct2D, o limite do reaproveitamento de quadros de uma janela sai, nesta ordem, da
variável `AYLIS_FRAME_REUSE_KB` (só em debug), do `frame_reuse_kb` do form (se não for `-1`) e
do valor do app (o do projeto ou o que o `RenderSettings` pôs).

## Variáveis de ambiente (só em debug, no desenvolvimento)

| Variável | O que faz |
|---|---|
| `AYLIS_RENDERER=d2d`, `gpu` ou `cpu` | Força o renderizador |
| `AYLIS_GPU=alto-desempenho` (ou `economia`, um nome) | Força a placa |
| `AYLIS_PRESENT=window`, `composition` ou `composed` | Força o jeito de apresentar no Windows |
| `AYLIS_VSYNC=0` | Não espera o monitor (para medir o custo do quadro) |
| `AYLIS_FRAME_REUSE_KB=N` | Força o limite do reaproveitamento de quadros (`0`: sempre cria) |
| `AYLIS_BENCH=N` | Redesenha N quadros sem parar e escreve o tempo médio, o p50, o p95 e a memória |
| `AYLIS_SNAPSHOT=pasta` | Grava a imagem do form (com `AYLIS_BENCH`) |
| `AYLIS_D2D_DEBUG=1` | Diz em que placa e com que alvo o Direct2D desenha |
| `AYLIS_D2D_TARGET=hwnd` | O alvo antigo do Direct2D (para comparar) |
| `AYLIS_DEBUG_GL=1` | O que o Skia na placa escolheu (Linux, Android) |
| `AYLIS_SPAN_SPLIT=N` | Conta cada monitor como N, para testar o `span_monitors` com um monitor só |
| `AYLIS_WAKE_TRACE=1` | Mostra o que acordou a janela (para achar repintura sem motivo) |

## Como medir

- **O seu app inteiro:** rode em release com `AYLIS_BENCH=600` (e o renderizador que quiser) e
  compare a linha `AYLIS_BENCH` antes e depois de trocar um valor. Na IDE, o Profiler (Executar
  → Profiler) mostra o tempo de cada quadro e de cada handler enquanto você usa o app.
- **Quadros de câmera e de vídeo no Direct2D:** `aylis-tool bench-frames` mede, na sua placa,
  1, 4, 9, 16 e 36 câmeras numa tela 1920 × 1080, criando um bitmap a cada quadro e
  reaproveitando, e sugere o `frame_reuse_kb`:

  ```text
  aylis-tool bench-frames
  aylis-tool bench-frames --cameras 4,16 --width 3840 --height 2160 --json quadros.json
  ```

  Feche as compilações antes: a medida é de tempo, e a máquina ocupada muda os números.
- **Os exemplos do Aylis:** `aylis-tool bench saída.json` mede os exemplos em cada renderizador
  e compara com uma medição anterior (`--compare`).

O que o Aylis mediu fica em `medicoes/` (por exemplo `medicoes/d2d_frames.json`, de onde veio o
padrão de 1024 KB: 36 câmeras de 320 × 180 caíram de 7,3 para 1,7 ms por quadro, e uma de
1920 × 1080 subiu de 0,9 para 1,4 ms, porque copiar num bitmap grande custa mais que criar).
