# Apps fora do alcance do Aylis: o que cada um pediria

Análise pedida pelo autor em 26/11 ([histórico, seção 23](historico-decisoes.md)): cinco tipos de
app que esbarram no Aylis de hoje. Para cada um, o que o app precisa, o que já existe, o que
falta e onde falta (no framework, no app ou numa diretriz). Nada aqui foi decidido: o que muda
uma diretriz está nas perguntas do fim de cada caso e só entra com a resposta do autor.

A régua é a de [De quem é cada coisa](../CLAUDE.md): o framework faz o que todo app refaria
igual e o que o app não alcança (o pixel, o sistema); o app faz a regra dele.

## Resumo

| App | Cabe hoje? | O que falta no framework | Esbarra em diretriz? |
|---|---|---|---|
| Imagem médica (DICOM) | O 2D quase: a janela/nível cabe na CPU | Imagem de 16 bits com tabela (LUT), volume 3D no `aylis-3d`, saída de 10 bits | Só a saída de 10 bits |
| Editor de vídeo com correção de cor | A linha do tempo e o player cabem | Busca quadro a quadro, a cor na placa (LUT 3D), os escopos, HDR | Sim: a cor na placa e o HDR |
| Diagramação profissional | Não | Desenhar glifos posicionados (`draw_glyph_run`) | Não (no navegador vira caminho) |
| DAW com VST3/CLAP | A tela cabe | Hospedar a janela nativa do plugin, a forma de onda em pirâmide | Não, com um componente; no Wayland, no navegador e no celular, não |
| Quadro infinito colaborativo | Não | Uma cena de objetos leves com índice espacial | Não |

## 1. Imagem médica (visualizador DICOM, radiologia)

**O que o app precisa:**
- ler DICOM (CT, RM, raio X; 12 a 16 bits por pixel, tons de cinza);
- janela/nível (*window/level*) ao arrastar o mouse, a 60 quadros por segundo;
- séries de 500 a 2000 fatias (512 × 512 × 2 bytes × 2000 = 1 GB);
- MPR (os cortes nos outros dois planos) e volume 3D (*ray marching*);
- medidas, anotações e o PACS (DICOMweb, DIMSE).

**O que já existe:** imagens, `PaintCanvas`, o `aylis-3d` (wgpu na mesma placa, composto
com os controles em todos os renderizadores), o provedor para bibliotecas de terceiros.

**Correção do que eu disse antes:** a janela/nível **não precisa de shader**. É uma tabela de
65.536 entradas (16 bits → 8 bits) aplicada pixel a pixel: em 512 × 512 são 262 mil consultas
por quadro, menos de 1 ms na CPU; numa mamografia de 4096 × 3328, uns 5 a 10 ms. Sai igual nos
três renderizadores, porque o resultado é uma imagem RGBA comum.

**O que falta:**
- **Framework:** um componente `DicomImage` (ou uma `Image` de 16 bits com LUT) ligado a um
  provedor de leitura (o `dicom-rs`, por exemplo, em `aylis-providers`), com a janela/nível
  pelo mouse (o componente cuida do ciclo) e o evento quando muda;
- **Framework:** um `VolumeView` no `aylis-3d`: textura 3D de 16 bits e o *ray marching*
  num shader **do Aylis** (não do app), como o `rays.wgsl` de hoje. No navegador vai pelo
  WebGPU, que o 3D já usa; a memória do wasm32 (4 GB) limita o tamanho do volume;
- **App:** medidas, laudo, o PACS (no backend: nada de rede médica na tela).

**Onde esbarra:** a saída de **10 bits** dos monitores de diagnóstico (calibrados pela curva
DICOM GSDF). A swap chain do Direct2D é de 8 bits (B8G8R8A8), o Canvas 2D também. No Windows
dá para pedir uma de 10 bits (R10G10B10A2) e no macOS também, mas não no navegador.

**Pergunta ao autor:** a saída de 10 bits pode existir só onde o sistema oferece (Windows,
macOS), degradando para 8 bits nos outros, como os recursos que já vão para o
`CLASS_PLATFORMS`? A regra "o desenho sai igual nos três" continuaria valendo para o que é
desenhado; mudaria só a profundidade da saída.

## 2. Editor de vídeo profissional com correção de cor

**O que o app precisa:**
- 4 trilhas 4K na linha do tempo, com o arrasto quadro a quadro (*scrubbing*) e as
  miniaturas;
- busca exata no quadro (os players param no quadro-chave mais próximo);
- correção de cor: LUT 3D (`.cube`), *lift/gamma/gain*, curvas, em ponto flutuante;
- escopos (forma de onda, vetorscópio, histograma) calculados a cada quadro;
- HDR (PQ, HLG) na tela e na exportação;
- o áudio sincronizado no quadro, e exportar (codificar).

**O que já existe:** o `aylis-media` decodifica na placa (Media Foundation, GStreamer, AVF,
MediaCodec) e o quadro chega ao Direct2D como textura sem cópia (a parede de 16 vídeos);
o player, o áudio e a câmera.

**O que falta:**
- **Framework:** busca exata e decodificar um quadro pedido (hoje o `aylis-media` é feito
  para tocar, não para editar), e as miniaturas em segundo plano;
- **Framework:** a cor na placa. Uma LUT 3D com interpolação trilinear em 4K na CPU custa
  dezenas de ms por quadro: precisa de shader, e os escopos de *compute*;
- **App ou provedor:** exportar (o codificador), o projeto de edição, os efeitos dele.

**Onde esbarra:**
- **A cor na placa:** o Canvas 2D do navegador não faz shader, então "nada que só um
  renderizador saiba fazer" impede, a não ser que a cor seja um nó do `aylis-media` (como o
  vídeo já é um caminho próprio da placa), com o navegador mostrando sem a correção ou pelo
  WebGL/WebGPU;
- **HDR:** a tela inteira teria de ser em ponto flutuante (scRGB, R16G16B16A16_FLOAT no
  Windows), com os controles no branco do SDR e o vídeo acima dele. Isso muda a swap chain e a
  composição dos três renderizadores.

**Perguntas ao autor:**
1. A correção de cor pode ser um caminho próprio do vídeo (como o quadro na placa), fora do
   desenho comum dos controles, e degradar onde não houver placa?
2. O HDR entra no escopo do Aylis, ou fica como limitação documentada?

## 3. Diagramação profissional (estilo InDesign)

**O que o app precisa:**
- texto vertical (chinês, japonês), ruby (furigana), justificação Knuth-Plass, hifenização
  por idioma, recursos OpenType finos (versaletes, ligaduras opcionais, numerais);
- caixas de texto ligadas, fluxo em volta de imagens, estilos de parágrafo;
- exportar PDF/X com CMYK e perfis de cor.

**O que já existe:** o `Canvas` tem `draw_text` (um texto num retângulo, com o estilo), que
cada renderizador entrega ao DirectWrite, ao Skia ou ao Canvas 2D; caminhos (`fill_path`); o PDF
pelo provedor (o krilla).

**O que falta:** o `Canvas` não sabe desenhar **glifos posicionados**: o app que montasse o
próprio layout (com o `rustybuzz` para formar e um algoritmo de quebra) não teria como
desenhar o resultado, a não ser como caminhos.

- **Framework:** um `draw_glyph_run` no `Canvas` (a fonte, os glifos, as posições).
  - O Direct2D tem o `DrawGlyphRun`, e o Skia, o `TextBlob`.
  - O Canvas 2D não tem desenho por glifo. A implementação padrão do trait desenharia os
    contornos como caminhos, que saem iguais (sem *hinting*, com cache por glifo).
  - Assim a regra de "sair igual nos três" continua valendo.
- **Provedor:** a formação do texto (`rustybuzz`) e a hifenização, ligadas por provedor;
- **App:** o motor de diagramação (o fluxo, a justificação), que é a regra do app;
- **PDF:** CMYK e perfis no provedor de PDF (o krilla tem espaços de cor), na exportação, não
  na tela.

**Onde esbarra:** em nada. É trabalho grande, mas cabe nas diretrizes.

## 4. Estação de áudio (DAW) com plugins VST3 ou CLAP

**O que o app precisa:**
- áudio em tempo real: *buffers* de 64 a 256 amostras (1 a 5 ms), sem alocar nem travar na
  thread de áudio;
- o grafo de áudio, MIDI, a automação;
- hospedar plugins **com a janela deles** (cada plugin desenha a própria tela numa janela
  nativa: HWND, NSView, X11);
- a forma de onda de 3 horas com zoom até a amostra.

**O que já existe:** o `cpal` no `aylis-media` (saídas e microfone), o `aylis-webview`, que já
põe uma janela nativa de outro (o WebView) dentro de um form, e agora a redução por pixel do
`Chart` (o mínimo e o máximo por coluna).

**O que falta:**
- **App:** a thread de áudio e o grafo. A diretriz já diz que thread é escolha do
  programador, e o Aylis não deve entrar no caminho de tempo real;
- **Framework:** um componente `NativeHost` que reserva um retângulo e põe ali uma janela
  nativa que o app recebeu de fora (a do plugin), como o WebView já faz: posição, recorte,
  foco, esconder com a aba. É a mesma mecânica, generalizada;
- **Framework:** um `WaveformView` com a pirâmide de mínimos e máximos (os arquivos de picos
  que toda DAW grava), para não ler as amostras a cada quadro. É a mesma ideia da redução do
  `Chart`, em níveis guardados.

**Onde esbarra:** a regra 7 ("o que não é do Aylis não se conserta por dentro") continua
valendo: o Aylis hospeda a janela do plugin, mas não conserta o que o plugin desenha.
- No **Wayland** não há como pôr a janela de outro dentro da sua: fica como limitação
  documentada (os plugins rodam pelo XWayland ou em janela solta);
- no navegador e no celular não há plugins nativos.

## 5. Quadro infinito colaborativo (estilo Figma ou Miro)

**O que o app precisa:** 1 milhão de objetos vetoriais, zoom de 1% a 25.600%, várias pessoas
editando ao mesmo tempo, com cursores, seleção, alças, desfazer, texto editável dentro dos
objetos.

**Por que não cabe hoje:**
- um componente por objeto não escala: a `giant_screen` mede 18 mil, e cada componente tem
  propriedades, eventos e layout;
- com 1 milhão de objetos, o desenho tem de cortar o que está fora da vista sem percorrer
  todos, o clique tem de achar o objeto sem testar todos, e o zoom afastado precisa de menos
  detalhe (LOD) ou de ladrilhos guardados;
- a precisão: o `Canvas` usa `f32`. Num mundo de 10⁶ unidades com zoom de 256×, o `f32` perde
  os pixels: as coordenadas do mundo precisam ser `f64`, relativas à câmera na hora de
  desenhar.

**O que precisaria para funcionar (um mecanismo do framework):**
1. **Uma `SceneView`:** um controle com **objetos leves** que não são componentes.
   - Cada objeto tem um id, o tipo (retângulo, caminho, texto, imagem, conector), a geometria
     em `f64` e o estilo por tokens do tema.
   - Os objetos ficam guardados num **índice espacial** (uma R-tree): pintar percorre só o que
     cruza a vista, e clicar acha o objeto pela árvore.
   - O custo segue o que aparece, como o DBGrid.
2. **Os dados como no `ClientDataSet`:** os objetos ficam num conjunto com o **delta**.
   - O backend manda e recebe em lote (`add`/`update`/`remove` de muitos de uma vez, nunca um
     por objeto) pelo protocolo, que já tem a fila com limite.
   - A colaboração (CRDT, Yjs ou Automerge) fica no backend, que é regra do app, e a tela só
     aplica o delta.
3. **O ciclo no componente:**
   - a seleção, as alças, mover, girar e redimensionar (o designer da IDE já faz isso, e o
     código pode ser reaproveitado), o desfazer local e o texto editado por cima com o editor
     que já existe;
   - o app recebe os eventos `on_item_click`, `on_selection_change` e `on_items_changed`, com
     o delta, e decide (aceitar, recusar, gravar).
4. **O desenho por níveis:**
   - longe, o objeto vira um retângulo ou some abaixo de um tamanho;
   - perto, desenha inteiro;
   - opcionalmente, ladrilhos guardados por nível de zoom (o núcleo já guarda camadas e copia
     na rolagem).
5. **A acessibilidade:** os objetos visíveis viram itens (`AccessibleItems`), não o milhão
   inteiro, como as linhas do grid.
6. **Os cursores dos outros:** uma camada por cima com os cursores e as seleções que o backend
   manda, num ritmo limitado (a mesma ideia do `max_paint_rate`).

**Onde esbarra:** em nada.
- Os objetos não são componentes, e isso não fere o "ancestral comum": a `SceneView` é um
  componente, e os objetos são dados dela, como as linhas são dados do grid;
- a regra de negócio (o que cada objeto significa, as permissões, a colaboração) continua no
  app.

**O tamanho:** é o maior dos cinco, pelo índice espacial, pela edição e pelo protocolo em lote.
Vale como um pacote próprio (`aylis-scene`), atrás de uma feature.
