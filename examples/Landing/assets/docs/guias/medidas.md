# Quanto cada componente custa: disco e memória

Medido em 25/10/2026, Windows 11 (x86_64, MSVC), release. As ferramentas que geraram estes números (o antigo `examples/medidas`) saíram do
repositório em 27/10/2026; ficam no histórico do git para medir de novo.

## Como foi medido

**Disco.** Um executável release por componente: um form com **uma** instância daquela
classe, registrando só ela (como o app enxuto). A coluna mostra quanto ele fica maior que
o executável só com o form (**18,61 MB**, a base de todo app: Skia, dados de texto do ICU,
janela e o núcleo do Aylis). Com o Direct2D no lugar do Skia, essa base cai para cerca de
2 MB (veja [Renderizadores no Windows](renderizadores.md)).

**RAM.** Para cada classe, num processo separado: cria 1 instância e depois mais 999,
faz o layout e pinta com o Skia de verdade (fora da tela).
- *RAM por instância*: heap do Rust de cada instância a mais, medido exato por um
  alocador que conta bytes. Inclui a parte da instância na árvore do `Ui` (nome,
  filhos, estado visual).
- *RAM da 1ª instância*: o que a primeira acrescenta. É menor que o de cada uma a mais
  quando cabe em espaço já reservado, e maior quando a classe monta um cache na primeira
  vez.
- *Processo por instância*: memória privada do processo por instância a mais. Inclui o
  que o Skia aloca em C++.

As medidas são de uma instância **como vem da paleta**. Conteúdo aumenta a memória: um
`ChatView` com 10 mil mensagens, um `DataGrid` ligado a 100 mil registros ou um `Chart` com
muitos pontos guardam esses dados.

## Resultado

### Básicos

| Componente | Disco (executável) | RAM por instância | RAM da 1ª instância | Processo por instância |
|---|---:|---:|---:|---:|
| Label | 11.0 KB | 688 B | 344 B | 660 B |
| Button | 53.5 KB | 696 B | 352 B | 664 B |
| Edit | 71.5 KB | 824 B | 480 B | 767 B |
| CheckBox | 5.0 KB | 648 B | 304 B | 660 B |
| RadioButton | 16.5 KB | 672 B | 328 B | 767 B |
| Switch | 5.5 KB | 656 B | 312 B | 660 B |
| ProgressBar | 4.5 KB | 640 B | 296 B | 681 B |
| Panel | 6.5 KB | 648 B | 304 B | 820 B |
| Layout | 1.5 KB | 616 B | 272 B | 660 B |
| Rectangle | 4.5 KB | 648 B | 304 B | 685 B |
| ComboBox | 27.0 KB | 664 B | 320 B | 816 B |
| ScrollBox | 4.5 KB | 632 B | 288 B | 566 B |
| Splitter | 5.0 KB | 640 B | 296 B | 820 B |
| Image | 10.5 KB | 696 B | 352 B | 660 B |
| Memo | 46.0 KB | 792 B | 448 B | 836 B |
| NumberBox | 87.0 KB | 865 B | 521 B | 1.0 KB |
| TrackBar | 10.0 KB | 656 B | 312 B | 664 B |
| ListBox | 15.0 KB | 704 B | 360 B | 898 B |
| RadioGroup | 13.0 KB | 688 B | 344 B | 623 B |
| GroupBox | 5.0 KB | 645 B | 301 B | 849 B |

### Adicionais

| Componente | Disco (executável) | RAM por instância | RAM da 1ª instância | Processo por instância |
|---|---:|---:|---:|---:|
| DateEdit | 122.5 KB | 898 B | 554 B | 968 B |
| TimeEdit | 81.0 KB | 856 B | 512 B | 882 B |
| Calendar | 23.0 KB | 680 B | 336 B | 931 B |
| ListView | 20.0 KB | 712 B | 368 B | 771 B |
| TreeView | 32.5 KB | 720 B | 376 B | 771 B |
| Chart | 62.0 KB | 1.4 KB | 1.1 KB | 2.0 KB |
| TabControl | 13.5 KB | 656 B | 312 B | 808 B |
| TabItem | 3.0 KB | 643 B | 299 B | 816 B |
| Expander | 7.0 KB | 656 B | 312 B | 865 B |
| StatusBar | 3.0 KB | 640 B | 296 B | 763 B |
| ToolBar | 2.0 KB | 616 B | 272 B | 664 B |

### Menus, sistema e animações

| Componente | Disco (executável) | RAM por instância | RAM da 1ª instância | Processo por instância |
|---|---:|---:|---:|---:|
| MainMenu | 34.5 KB | 664 B | 320 B | 742 B |
| PopupMenu | 29.0 KB | 376 B | 32 B | 566 B |
| MenuItem | 13.5 KB | 432 B | 88 B | 664 B |
| Timer | 3.0 KB | 408 B | 136 B | 660 B |
| OpenDialog | 3.5 KB | 480 B | 136 B | 570 B |
| SaveDialog | 5.5 KB | 480 B | 136 B | 566 B |
| FloatAnimation | 15.5 KB | 519 B | 175 B | 681 B |
| ColorAnimation | 15.5 KB | 516 B | 172 B | 681 B |

### Modernos

| Componente | Disco (executável) | RAM por instância | RAM da 1ª instância | Processo por instância |
|---|---:|---:|---:|---:|
| Icon | 45.5 KB | 652 B | 43.6 KB | 672 B |
| Badge | 47.5 KB | 676 B | 332 B | 828 B |
| Banner | 50.0 KB | 717 B | 43.6 KB | 923 B |
| Avatar | 16.0 KB | 724 B | 380 B | 742 B |
| SegmentedControl | 52.5 KB | 709 B | 365 B | 923 B |
| ActivityIndicator | 6.0 KB | 624 B | 280 B | 590 B |
| Breadcrumb | 49.0 KB | 771 B | 43.7 KB | 1.0 KB |
| HotkeyEdit | 15.0 KB | 683 B | 339 B | 742 B |
| DirectionPad | 52.0 KB | 668 B | 43.6 KB | 816 B |
| LevelMeter | 9.5 KB | 656 B | 312 B | 566 B |
| SeekBar | 34.5 KB | 680 B | 336 B | 750 B |
| NodeGraph | 82.5 KB | 776 B | 43.7 KB | 1.0 KB |
| ChatView | 68.5 KB | 696 B | 352 B | 746 B |
| Waveform | 6.5 KB | 856 B | 512 B | 947 B |

### Dados e controles de dados

| Componente | Disco (executável) | RAM por instância | RAM da 1ª instância | Processo por instância |
|---|---:|---:|---:|---:|
| ClientDataSet | 290.0 KB | 840 B | 496 B | 951 B |
| Field | 37.0 KB | 856 B | 512 B | 890 B |
| DataSource | 3.0 KB | 392 B | 48 B | 566 B |
| DataNavigator | 319.0 KB | 672 B | 328 B | 841 B |
| DataGrid | 491.5 KB | 912 B | 568 B | 1.1 KB |
| DBImage | 115.0 KB | 736 B | 392 B | 816 B |

### Mídia

| Componente | Disco (executável) | RAM por instância | RAM da 1ª instância | Processo por instância |
|---|---:|---:|---:|---:|
| MediaPlayer | 9.0 KB | 480 B | 136 B | 681 B |
| VideoPlayer | 27.0 KB | 984 B | 640 B | 1.2 KB |

## Como ler

**A soma não é exata: componentes compartilham código.** O número de cada um é o que ele
traz sozinho. Quando dois usam a mesma peça, ela entra uma vez só:

| Peça compartilhada | Tamanho aproximado | Quem traz |
|---|---:|---|
| Ícones do Aylis e leitor de SVG | ~40 KB | Button, Icon, Badge, Banner, Breadcrumb, SegmentedControl, DirectionPad, NodeGraph, Edit (busca)... |
| `Edit` (texto de uma linha, seleção, área de transferência) | ~70 KB | NumberBox, DateEdit, TimeEdit |
| Camada de dados (`ClientDataSet`: cursor, filtros, índices, delta, formatos) | ~290 KB | todos os DB* |
| Animações de propriedade | ~15 KB | FloatAnimation, ColorAnimation |

Por isso um `Button` (53,5 KB) e um `Badge` (47,5 KB) juntos não somam 101 KB: os ~40 KB
dos ícones entram uma vez.

**RAM.** Cada controle custa menos de 1,5 KB de heap e de 0,6 a 2 KB no processo. Uma tela
com 300 controles fica perto de 0,5 MB. O que pesa de verdade em memória é o conteúdo
(registros, imagens, mensagens) e o próprio Skia: fontes, glifos já desenhados e a
superfície da janela. Esse custo é fixo e não cresce com o número de componentes. Os
saltos de 50 a 330 KB na primeira instância de algumas classes são o Skia preparando
glifos e tamanhos de fonte que ainda não tinha usado. Os 43,6 KB da primeira instância de
`Icon`, `Banner`, `Breadcrumb`, `DirectionPad` e `NodeGraph` são o conjunto de ícones,
montado uma vez no primeiro uso.

**Mídia.** `MediaPlayer` (9 KB) e `VideoPlayer` (27 KB) aparecem pequenos porque a
decodificação (áudio com symphonia e cpal, vídeo pelo sistema) só entra no executável
quando o app chama a reprodução. Um app que toca mídia de verdade leva esses
decodificadores.

**App de projeto.** Um app criado pela IDE usa o `Application`: abrir o pacote embutido,
ler forms e temas em JSON, traduções e assets. Isso soma cerca de 0,4 MB à base. Um form
com Label, Edit, CheckBox e Button publicado pela IDE tem **19,02 MB** com o Skia e
**1,97 MB** com o Direct2D (veja [Carregamento e app enxuto](carregamento.md)).

(MB aqui é 1024 × 1024 bytes, como no Explorer do Windows.)
