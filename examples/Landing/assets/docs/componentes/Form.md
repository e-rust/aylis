# Form

Janela. Pinta o fundo com o estilo `Form` do tema.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caption` | Título | texto | O título da janela (na barra de título e na barra de tarefas). |
| `border_style` | Estilo da borda | um de: `sizeable`, `single`, `none`, `custom` | A borda da janela: redimensionável, de tamanho fixo, sem borda nem título (`none`) ou sem a barra do sistema, para uma `TitleBar` própria (`custom`). |
| `window_corners` | Cantos da janela | um de: `default`, `round`, `small`, `square` | Os cantos da janela (Windows 11 e macOS; nas outras, sem efeito): útil com a barra de título própria (`border_style = custom`), que fica com os cantos de uma janela comum. |
| `transparent` | Transparente | lógico | A janela deixa ver o que está atrás onde o form não é opaco: o fundo com alfa (`fill` `#RRGGBBAA`, a semitransparência), o que fica fora de `window_shape` e os pixels transparentes da pintura. Com o `blur` do `WindowSettings`, o que está atrás sai desfocado (o vidro fosco). Desligado (o padrão), a janela é opaca. Windows com o Direct2D (o padrão), macOS e Linux com compositor; no Windows com o Skia, no celular e no navegador, sem efeito (a janela fica opaca e retangular). |
| `window_shape` | Forma da janela | um de: `rectangle`, `rounded`, `ellipse`, `path` | A forma da janela: o retângulo, os cantos arredondados (`rounded`, com `shape_radius` ou um raio por canto em `shape_radii`), a elipse (`ellipse`: o form quadrado vira um círculo) ou um contorno qualquer (`path`, com `shape_path`). A borda é suavizada (antialiasing), os filhos ficam dentro dela e os cliques fora dela vão para o que está atrás. Fora do retângulo, a janela fica transparente e sem a moldura do sistema (como `border_style = none`, sem redimensionar pela borda), e o form aparece com a forma também no designer. |
| `shape_radius` | Raio da forma | número | O raio dos cantos com `window_shape = rounded` (unidades do form). |
| `shape_radii` | Raios dos cantos | número ou lista de números | Um raio por canto com `window_shape = rounded`, na ordem do CSS: cima-esquerda, cima-direita, baixo-direita, baixo-esquerda (`"24,24,0,0"`: só os de cima). Todos zero (o padrão): vale o `shape_radius`. |
| `shape_path` | Contorno da forma | texto | O contorno com `window_shape = path`, no formato do `d` do SVG (`M 100 0 L 200 100 L 100 200 L 0 100 Z`: um losango). Os limites do contorno viram os do form: as coordenadas podem estar em qualquer escala, e o contorno acompanha quando o form muda de tamanho. Inválido ou vazio: o retângulo. |
| `window_opacity` | Opacidade da janela | número | A transparência da janela inteira, com a moldura (1 = opaca, 0,5 = metade). Windows; nas outras plataformas, sem efeito. Para o fundo transparente com os controles opacos, use `transparent`. |
| `zoom` | Zoom | número | O zoom deste form (1 = normal), vezes o do app (`Ui::set_zoom`): tudo nele fica maior ou menor e o layout se refaz no espaço da janela. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `transition` | Transição | um de: `none`, `fade`, `slide_left`, `slide_right`, `slide_up`, `slide_down` | Celular e navegador (os forms se empilham): como o form entra ao abrir e sai ao fechar (`slide_left` como uma tela nova do Android/iOS, `slide_up` como uma folha, `fade`). No desktop cada form é uma janela e isto não vale. |
| `layer_cache` | Camadas automáticas | um de: `project`, `on`, `off` | As camadas automáticas neste form: as páginas de uma transição e o contêiner que recorta os filhos com o lugar ou a transparência animando viram uma imagem guardada, que anda sem repintar os filhos (mais rápido, e a memória das imagens enquanto andam). `project`: o padrão do projeto (`image_cache.auto_layers`); `on`/`off` trocam só neste form. O `cache_layer` pedido num controle vale sempre. |
| `frame_reuse_kb` | Reaproveitar quadros até (KB) | inteiro | Até quantos KB um quadro de câmera ou de vídeo reaproveita o bitmap do quadro anterior na placa nesta janela, em vez de criar outro. `-1` (o padrão): o do projeto (`image_cache.frame_reuse_kb`, que o `RenderSettings` troca em execução); `0`: sempre cria; acima de 32768, ajustado. Vale no Direct2D (Windows), no quadro seguinte à troca; no Skia e no navegador, sem efeito (eles não reaproveitam). |
| `topmost` | Sempre na frente | lógico | Sempre por cima das outras janelas. |
| `bottommost` | Sempre atrás | lógico | Sempre por baixo das outras janelas, como um fundo de tela (com `topmost` também, vale o `topmost`). |
| `keep_awake` | Manter acordado | um de: `off`, `system`, `display` | Enquanto o form está aberto, o sistema não dorme (`system`) ou nem apaga a tela (`display`). Vale o maior pedido entre os forms abertos. Windows; nas outras plataformas, sem efeito por enquanto. |
| `confine_cursor` | Prender o mouse | um de: `none`, `window`, `monitor` | Prende o mouse na janela (`window`) ou no monitor dela (`monitor`) enquanto ela está ativa; trocar de janela (Alt+Tab) solta. |
| `show_in_taskbar` | Na barra de tarefas | lógico | Aparece na barra de tarefas (desligue para menus e janelas de ferramenta). |
| `active_control` | Controle ativo | componente | O controle que recebe o foco quando o form abre. |
| `tab_order` | Ordem de tabulação | lista de textos (separados por `;`) | A ordem do Tab no form inteiro: nomes separados por `;` (`edtNome;edtEmail;btnSalvar`). Pula entre níveis (de um campo num painel para outro num grupo); os controles fora da lista vêm depois, na ordem natural. Vazio: a ordem natural (a da árvore). |
| `remember_placement` | Lembrar a posição | lógico | Lembra onde a janela estava (o monitor, a posição, o tamanho e se estava maximizada) e reabre igual, num arquivo do app; se aquele monitor sumiu, abre como se não houvesse. |
| `gpu` | Placa de vídeo | texto | A placa de vídeo da janela (vazio: a do projeto): `monitor` troca para a placa do monitor em que ela está (Windows e Linux com Wayland); `alto-desempenho`, `economia` ou parte do nome fixam uma. |
| `present` | Apresentação na tela | um de: `project`, `window`, `composition`, `composed` | Como o quadro chega à janela no Windows (`project`: o do projeto): `window`, `composition` ou `composed`, sempre composta pelo DWM, fora do plano de overlay (o form em tela cheia por cima do resto, onde o cursor congelava em placas com MPO). |
| `activate_on_show` | Ativar ao mostrar | lógico | Mostrar a janela dá o foco a ela (desligado: aparece sem tirar o foco de quem tem, como uma projeção que não deve roubar o teclado do console). |
| `force_foreground` | Forçar a frente ao mostrar | lógico | Mostrar ou trazer para a frente toma o foco mesmo quando o app não recebeu a última entrada (Windows): o form aberto por um atalho global, de um timer ou da bandeja. O Windows só deixa ir para a frente quem recebeu o último clique ou tecla; sem isto, a janela aparece atrás, com o botão piscando, e o teclado (o Esc) vai para o app de trás. Nas outras plataformas, sem efeito (o pedido normal de foco). |
| `close_action` | Ao fechar | um de: `hide`, `free` | O que fechar faz com o form: esconde (pode abrir de novo) ou libera (some da memória, depois de fechar). |
| `frame_rate` | Quadros por segundo | número | Quadros por segundo da janela (0 = a taxa do monitor dela). Um telão de 60 Hz ao lado de um monitor de 144 Hz não precisa acompanhar o mais rápido. |
| `host` | Embutido em | componente | Onde o form fica embutido (um painel, uma faixa de abas...); vazio: na janela dele. Mudar com o app rodando entra ou sai: aberto numa janela, ela some sem fechar o form (`on_dock`); solto, ele volta a ter janela, com o tamanho de antes (`on_undock`). Numa faixa de abas, ele vira uma aba com o título dele. O form principal e um modal não entram. |
| `dockable` | Pode encaixar | lógico | A janela do form entra numa faixa de abas com `accept_forms` quando arrastada pela barra de título até ela (como as do navegador); embutido numa faixa, puxar a aba dele para fora o solta. Windows (nas outras plataformas, só pelo `host`). |

### Teclado

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tab_keys` | Teclas de avançar campo | texto | As teclas que fazem o papel do Tab (`"Enter"`, `"Enter;Down"`), como o Enter que pula de campo nos sistemas de cadastro. O controle que usa a tecla (o Enter num Memo) fica com ela. |
| `back_tab_keys` | Teclas de voltar campo | texto | As que fazem o papel do Shift+Tab (`"Shift+Enter;Up"`). |
| `allowed_chars` | Caracteres permitidos | texto | Os caracteres liberados, bloqueados e só os liberados, para todos os campos do form (o campo decide primeiro; `chars_override` faz o form mandar). Faixas: `0-9`, `a-z`. |
| `blocked_chars` | Caracteres bloqueados | texto | Os caracteres que os campos do form não aceitam (digitados ou colados); faixas como `a-z`. Vazio: nenhum. |
| `only_allowed` | Só os permitidos | lógico | Só os caracteres de `allowed_chars` entram nos campos do form. |
| `chars_override` | Força os caracteres | lógico | O filtro de caracteres do form manda nos campos de dentro (sem ele, o do campo decide primeiro). |
| `char_map` | Trocas de caracteres | texto | Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a mesma herança dos filtros. |
| `key_preview` | Recebe as teclas antes | lógico | O form recebe as teclas antes do controle com foco (`on_key_down` e `on_key_press`). |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `position` | Posição | um de: `center`, `screen_center`, `designed`, `default`, `top_left`, `top_right`, `bottom_left`, `bottom_right` | Onde a janela abre: no centro do dono ou da tela, em `left`/`top`, onde o sistema escolher ou num canto do monitor. |
| `window_state` | Estado da janela | um de: `normal`, `maximized`, `minimized`, `full_screen` | Normal, maximizada, minimizada ou em tela cheia (`full_screen`). No celular e no navegador o form já ocupa a tela toda. |
| `full_screen_mode` | Modo da tela cheia | um de: `borderless`, `exclusive` | A tela cheia sem bordas (o padrão) ou exclusiva, com o modo de vídeo pedido. |
| `video_width` | Largura do vídeo | inteiro | A largura em pixels do modo de vídeo da tela cheia exclusiva (0: a atual do monitor). Vale o modo do monitor mais perto da largura, da altura e da taxa pedidas. |
| `video_height` | Altura do vídeo | inteiro | A altura em pixels do modo de vídeo da tela cheia exclusiva (0: a atual do monitor). |
| `video_refresh` | Taxa de atualização | número | A taxa de atualização em Hz da tela cheia exclusiva (0: a atual do monitor). |
| `monitor` | Retorno | inteiro | Em que monitor a janela abre (o índice do `Screen`; -1 = o do dono ou o principal). Com `position = designed`, o `left`/`top` contam do canto dele. |
| `span_monitors` | Ocupar vários monitores | lógico | O form maior que um monitor vira uma janela por monitor, cada uma na placa de vídeo do seu (Windows; nas outras, sem efeito). Sem isso, com monitores em placas diferentes, o sistema copia cada quadro de uma placa para a outra. Com `window_state = full_screen`, o form cobre todos os monitores (um telão). |
| `physical_pixels` | Pixels físicos | lógico | 1 unidade do form = 1 pixel físico da tela, sem a escala do monitor nem o zoom, em todas as janelas dele (num form em vários monitores, em todos os pedaços, mesmo com escalas diferentes). Um clique no pixel (x, y) chega como (x, y) do form (a partir do canto dele; com `full_screen` e `span_monitors`, do retângulo de todos os monitores), e uma imagem do tamanho do form em pixels (o print da tela) bate pixel a pixel. Mudar com a janela aberta não a move: o form passa a ter o tamanho dela na escala nova. |
| `orientation` | Orientação | um de: `project`, `any`, `portrait`, `landscape`, `sensor_portrait`, `sensor_landscape` | Celular: a orientação da tela enquanto este form está à frente (`project`: a do projeto). Fechá-lo devolve a do form de baixo; mudar com o app aberto gira na hora. No desktop e no navegador, sem efeito. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_show` | o form abriu | — |  |
| `on_window_created` | — | — |  |
| `on_close` | o form fechou | — |  |
| `on_close_query` | o form vai fechar: `Err` recusa (o X não fecha) | — | sim: `can_close` |
| `on_back` | a tecla Voltar do celular, no form do topo: responda `handled` (ou `Err`) quando o app tratar (voltar a tela de dentro); sem isso, o de sempre | — | sim: `handled` |
| `on_activate` | — | — |  |
| `on_deactivate` | — | — |  |
| `on_minimize` | — | — |  |
| `on_restore` | — | — |  |
| `on_navigate` | — | — |  |
| `on_gpu_change` | — | — |  |
| `on_dock` | — | — |  |
| `on_undock` | — | — |  |
| `on_files_drop` | — | — |  |
| `on_view_change` | — | — |  |

## Em código

No form, o `Form1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Form", "name": "Form1", "on_show": "form1_show" }
```

E o handler, um método do form (`self.Form1` já é um `Form`):

```rust
impl FrmPrincipal {
    pub fn form1_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Form1.caption(ui);
        self.Form1.set_caption(ui, format!("{atual}!"));
    }
}
```

## Mais sobre o Form

### Vistas: o mesmo form em outras larguras

Um form muda com o tamanho da janela pelas vistas (`views`): cada vista diz quando vale (até
uma largura, uma altura, uma orientação ou uma plataforma) e só o que muda nela. Saindo da
faixa, tudo volta ao desenho base. No designer, escolha a vista na barra de cima e mexa nos
componentes: a mudança fica na vista.

```json
"views": [
  { "name": "compact", "max_width": 699, "font_scale": 0.9,
    "changes": {
      "pnlLateral": { "visible": false },
      "pnlVolume": { "parent": "pnlMusica", "align": "bottom" },
      "lblTitulo": { "font_size": 28 }
    } }
]
```

Uma vista troca propriedades, muda um componente de contêiner (`parent`) e de posição
(`index`) e escala as fontes e os espaços. O que não é propriedade vai no `on_view_change`, que
chega com o nome da vista que entrou (vazio: o base) só quando ela muda, e não a cada pixel do
arrasto da janela:

```rust
pub fn frmPlayer_view_change(&mut self, ui: &mut Ui, _sender: Form, view: String) {
    self.data.compacto = view == "compact";
    self.lstMusicas.set_item_height(ui, if self.data.compacto { 48.0 } else { 56.0 });
}
```
