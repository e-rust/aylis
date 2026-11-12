# Propriedades e eventos comuns a todo controle

Todo controle visual (o que tem `ControlBase`) tem estas, além das próprias.

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `inherit_state` | Herdar o estado do pai | lógico | Pinta com o estado do pai (`hover`, `pressed`, `checked`): o rótulo da linha marcada fica com a cor de marcado do tema sem trocar o estilo filho a filho (C25 do Tabernáculo). |
| `opacity` | Opacidade | número | Transparência de 0 (invisível) a 1, com os filhos; continua no layout e clicável. |
| `cache_layer` | Guardar desenho em camada | lógico | O desenho dele e dos filhos guardado numa imagem, refeita só quando algo lá dentro muda (o `CacheAsBitmap`): um painel pesado que desliza ou some aos poucos. |
| `translate_x` | Deslocar X | número |  |
| `translate_y` | Deslocar Y | número |  |
| `shadow` | Sombra | sombra | Sombra do controle; quando há, vale no lugar da sombra do estilo. |
| `style` | Estilo | texto | A variante do tema (`primary`, `outline`, `flat`, `card`...): o estilo `Classe.variante`. |
| `theme` | Tema | texto | Tema próprio (pelo nome), que vale também para os filhos. Vazio = o do pai. |
| `fill` | Preenchimento | cor |  |
| `border_color` | Cor da borda | cor |  |
| `text_color` | Cor do texto | cor |  |
| `accent_color` | Cor de destaque | cor |  |
| `success_color` | Cor de sucesso | cor |  |
| `warning_color` | Cor de atenção | cor |  |
| `danger_color` | Cor de perigo | cor |  |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `visible` | Visível | lógico | Aparece; escondido não ocupa lugar no layout. |
| `enabled` | Habilitado | lógico | Aceita clique e teclado; desligado aparece apagado, e os filhos também. |
| `hit_test` | Recebe cliques | lógico | Recebe o mouse; desligado, o clique passa para o que está embaixo. |
| `clip_children` | Recorta o conteúdo | lógico | Corta o desenho dos filhos na borda dele. |
| `translate` | Traduzir | lógico | Deslocamento só do desenho (não muda o layout): animações e transições. |
| `captures_wheel` | Segura a roda do mouse | lógico | A roda do mouse em cima dele fica com ele (não rola os contêineres de fora), mesmo sem ele rolar: uma prévia que dá zoom com a roda, um mapa, um gráfico. |
| `can_focus` | Recebe foco | lógico | Pode receber o foco do teclado (clique ou Tab). |
| `draggable` | Arrastável | lógico | Pode ser arrastado (o `drag` diz o que vai junto). |
| `accepts_drops` | Aceita soltar | lógico | Aceita o que é solto em cima dele (`on_drop`). |
| `drag_data` | Dados do arrasto | texto |  |
| `drag_label` | Rótulo do arrasto | texto |  |
| `drag_image` | Imagem do arrasto | componente |  |
| `drop_types` | Tipos aceitos ao soltar | texto |  |
| `tab_stop` | Para no Tab | lógico | Entra na ordem do Tab. |
| `cursor` | Cursor do mouse | um de: `default`, `pointer`, `text`, `wait`, `crosshair`, `move`, `not_allowed`, `resize_horizontal`, `resize_vertical`, `resize_ne_sw`, `resize_nw_se`, `grab`, `grabbing`, `none` | O cursor do mouse em cima dele. |
| `hint` | Dica | texto | Dica mostrada com o mouse parado em cima (o `Hint` do Delphi). |
| `accessible_name` | Nome acessível | texto |  |
| `popup_menu` | Menu de contexto | componente | Menu de contexto (como o `PopupMenu` do `TControl`): abre no clique direito. |

### Fonte

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `font_family` | Fonte | texto |  |
| `font_size` | Tamanho da fonte | número |  |
| `font_weight` | Peso da fonte | um de: `default`, `thin`, `light`, `normal`, `medium`, `semibold`, `bold`, `black` |  |
| `font_italic` | Itálico | lógico |  |
| `font_underline` | Sublinhado | lógico |  |
| `font_strikeout` | Tachado | lógico |  |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `left` | Esquerda | número | Posição a partir do canto do pai; só vale com `align: none`. |
| `top` | Topo | número | Posição a partir do topo do pai; só vale com `align: none`. |
| `width` | Largura | número | Largura pedida; o `align` pode esticar (top, bottom, client). |
| `height` | Altura | número | Altura pedida; o `align` pode esticar (left, right, client). |
| `align` | Alinhamento | um de: `none`, `top`, `bottom`, `left`, `right`, `most_top`, `most_bottom`, `most_left`, `most_right`, `client`, `contents`, `center`, `vert_center`, `horz_center`, `horizontal`, `vertical`, `fit` | Encaixe no pai: `top`/`bottom` esticam na largura, `left`/`right` na altura, `client` ocupa o resto; `none` usa `left`/`top`. |
| `anchors` | Âncoras | vários de: `left`, `top`, `right`, `bottom` | Com `align: none`, os lados que acompanham o pai quando ele muda de tamanho: `left,right` estica na largura, só `right` anda junto com a borda direita. |
| `margins` | Margens | número ou lista de números | Espaço por fora, até os vizinhos e a borda do pai: número, `[h, v]` ou `[esquerda, cima, direita, baixo]`. |
| `padding` | Espaçamento interno | número ou lista de números | Espaço por dentro, até os filhos: número, `[h, v]` ou `[esquerda, cima, direita, baixo]`. |
| `rotation` | Rotação | ângulo (graus) | Giro em graus, em torno do centro (o `RotationAngle` do FMX). |
| `aspect_ratio` | Proporção | número | Proporção fixa (largura ÷ altura; 0 = livre), como o `VResponsive` do Vuetify. |
| `max_width` | Largura máxima | número | Largura máxima (0 = sem limite), como o `max-width` do CSS com margem automática. |
| `min_width` | Largura mínima | número | Largura mínima, altura mínima e máxima (0 = sem limite), como o `Constraints` da VCL. |
| `min_height` | Altura mínima | número |  |
| `max_height` | Altura máxima | número |  |
| `self_align` | Alinhamento próprio | um de: `stretch`, `start`, `center`, `end` | Encaixado: estica na faixa ou fica com o próprio tamanho no começo, meio ou fim. |
| `auto_size` | Tamanho automático | lógico | O tamanho sai do conteúdo: a altura; num Label encaixado à esquerda ou à direita, também a largura. |
| `column_span` | Colunas ocupadas | inteiro | Quantas colunas ocupa num `GridLayout` (o `colspan`; 1 ou 0: uma). |
| `grid_row` | Linha na grade | inteiro | O lugar fixo num `GridLayout` (o `TGridPanel`): linha e coluna (-1: segue a ordem) e quantas linhas ocupa. O relatório do Tabernáculo, G12. |
| `grid_column` | Coluna na grade | inteiro |  |
| `row_span` | Linhas ocupadas | inteiro |  |
| `grow` | Crescer no espaço livre | número | Quanto do espaço que sobra na linha de um `FlowLayout` ele toma (o `flex-grow`): um `Layout` vazio com `grow` 1 é o espaçador que empurra o resto para o fim (L04). |

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro | Um número livre para o código (não muda nada na tela). |
| `tag_string` | Texto livre (TagString) | texto |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |
| `on_dbl_click` | o duplo clique | — |  |
| `on_long_press` | — | — |  |
| `on_pointer_down` | — | — |  |
| `on_pointer_up` | — | — |  |
| `on_pointer_move` | — | — |  |
| `on_pointer_enter` | — | — |  |
| `on_pointer_leave` | — | — |  |
| `on_wheel` | — | — |  |
| `on_gesture` | — | — |  |
| `on_key_down` | uma tecla com o foco no componente | — | sim: `key` |
| `on_key_press` | — | — | sim: `key` |
| `on_key_up` | — | — |  |
| `on_enter` | — | — |  |
| `on_exit` | — | — |  |
| `on_resize` | o tamanho mudou | — |  |
| `on_drag_start` | — | — | sim: `data`, `label` |
| `on_drag_over` | — | — | sim: `accept` |
| `on_drag_move` | — | — |  |
| `on_drag_leave` | — | — |  |
| `on_drag_drop` | — | — |  |
| `on_drag_end` | — | — |  |
