# Propriedades e eventos comuns a todo controle

Todo controle visual (o que tem `ControlBase`) tem estas, além das próprias.

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `opacity` | Opacidade | número |
| `cache_layer` | Guardar desenho em camada | lógico |
| `translate_x` | Deslocar X | número |
| `translate_y` | Deslocar Y | número |
| `shadow` | Sombra | sombra |
| `style` | Estilo | texto |
| `theme` | Tema | texto |
| `fill` | Preenchimento | cor |
| `border_color` | Cor da borda | cor |
| `text_color` | Cor do texto | cor |
| `accent_color` | Cor de destaque | cor |
| `success_color` | Cor de sucesso | cor |
| `warning_color` | Cor de atenção | cor |
| `danger_color` | Cor de perigo | cor |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `visible` | Visível | lógico |
| `enabled` | Habilitado | lógico |
| `hit_test` | Recebe cliques | lógico |
| `clip_children` | Recorta o conteúdo | lógico |
| `captures_wheel` | Segura a roda do mouse | lógico |
| `can_focus` | Recebe foco | lógico |
| `draggable` | Arrastável | lógico |
| `accepts_drops` | Aceita soltar | lógico |
| `drag_data` | Dados do arrasto | texto |
| `drag_label` | Rótulo do arrasto | texto |
| `drag_image` | Imagem do arrasto | componente |
| `drop_types` | Tipos aceitos ao soltar | texto |
| `tab_stop` | Para no Tab | lógico |
| `cursor` | Cursor do mouse | um de: `default`, `pointer`, `text`, `wait`, `crosshair`, `move`, `not_allowed`, `resize_horizontal`, `resize_vertical`, `resize_ne_sw`, `resize_nw_se`, `none` |
| `hint` | Dica | texto |
| `popup_menu` | Menu de contexto | componente |

### Fonte

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `font_family` | Fonte | texto |
| `font_size` | Tamanho da fonte | número |
| `font_weight` | Peso da fonte | um de: `default`, `thin`, `light`, `normal`, `medium`, `semibold`, `bold`, `black` |
| `font_italic` | Itálico | lógico |
| `font_underline` | Sublinhado | lógico |
| `font_strikeout` | Tachado | lógico |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `left` | Esquerda | número |
| `top` | Topo | número |
| `width` | Largura | número |
| `height` | Altura | número |
| `align` | Alinhamento | um de: `none`, `top`, `bottom`, `left`, `right`, `most_top`, `most_bottom`, `most_left`, `most_right`, `client`, `contents`, `center`, `vert_center`, `horz_center`, `horizontal`, `vertical`, `fit` |
| `anchors` | Âncoras | vários de: `left`, `top`, `right`, `bottom` |
| `margins` | Margens | posição e tamanho |
| `padding` | Espaçamento interno | posição e tamanho |
| `rotation` | Rotação | ângulo (graus) |
| `aspect_ratio` | Proporção | número |
| `max_width` | Largura máxima | número |
| `min_width` | Largura mínima | número |
| `min_height` | Altura mínima | número |
| `max_height` | Altura máxima | número |
| `self_align` | Alinhamento próprio | um de: `stretch`, `start`, `center`, `end` |
| `auto_size` | Tamanho automático | lógico |
| `column_span` | Colunas ocupadas | inteiro |

### Outros

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `tag` | Marca (número) | inteiro |

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
| `on_drag_drop` | — | — |  |
| `on_drag_end` | — | — |  |
