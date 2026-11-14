# Propriedades e eventos comuns a todo controle

Todo controle visual (o que tem `ControlBase`) tem estas, além das próprias.

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `inherit_state` | Herdar o estado do pai | lógico | Pinta com o estado do pai (`hover`, `pressed`, `checked`): o rótulo da linha marcada fica com a cor de marcado do tema sem trocar o estilo filho a filho (C25 do Tabernáculo). |
| `opacity` | Opacidade | número | Transparência de 0 (invisível) a 1, com os filhos; continua no layout e clicável. |
| `blend_mode` | Modo de mistura | um de: `normal`, `multiply`, `screen`, `overlay`, `darken`, `lighten`, `color_dodge`, `color_burn`, `hard_light`, `soft_light`, `difference`, `exclusion`, `hue`, `saturation`, `color`, `luminosity` | Como o controle (com os filhos) se junta ao que está por baixo (o `mix-blend-mode` do CSS): `difference` com branco é o negativo, como a marca d'água sobre um vídeo. |
| `cache_layer` | Guardar desenho em camada | lógico | O desenho dele e dos filhos guardado numa imagem, refeita só quando algo lá dentro muda: um painel pesado que desliza ou some aos poucos. |
| `translate_x` | Deslocar X | número | Deslocamento só do desenho (não muda o layout): animações e transições. |
| `translate_y` | Deslocar Y | número | Deslocamento só do desenho (não muda o layout): animações e transições. |
| `shadow` | Sombra | sombra | Sombra do controle; quando há, vale no lugar da sombra do estilo. |
| `style` | Estilo | texto | A variante do tema (`primary`, `outline`, `flat`, `card`...): o estilo `Classe.variante`. |
| `theme` | Tema | texto | Tema próprio (pelo nome), que vale também para os filhos. Vazio = o do pai. |
| `fill` | Preenchimento | cor | A cor do fundo; vazio: a do tema. |
| `border_color` | Cor da borda | cor | A cor da borda; vazio: a do tema. |
| `text_color` | Cor do texto | cor | A cor do texto; vazio: a do tema. |
| `accent_color` | Cor de destaque | cor | O destaque: a marca da caixa de seleção, a barra do progresso, o trilho preenchido, a seleção, o foco. |
| `success_color` | Cor de sucesso | cor | As cores de sentido: subir/positivo, atenção e cair/erro (candles, variações, avisos). Esta é a de subir/positivo; vazio: a do tema. |
| `warning_color` | Cor de atenção | cor | A cor de sentido de atenção (avisos); vazio: a do tema. |
| `danger_color` | Cor de perigo | cor | A cor de sentido de cair/erro (candles em queda, variações negativas, erros); vazio: a do tema. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `visible` | Visível | lógico | Aparece; escondido não ocupa lugar no layout. |
| `enabled` | Habilitado | lógico | Aceita clique e teclado; desligado aparece apagado, e os filhos também. |
| `hit_test` | Recebe cliques | lógico | Recebe o mouse; desligado, o clique passa para o que está embaixo. |
| `clip_children` | Recorta o conteúdo | lógico | Corta o desenho dos filhos na borda dele. |
| `translate` | Traduzir | lógico | Os textos dele passam pela tradução do app (o padrão). Desligado, ficam como estão e não entram na lista do que traduzir: um nome próprio, um código, um valor de exemplo (G19 do Tabernáculo). |
| `captures_wheel` | Segura a roda do mouse | lógico | A roda do mouse em cima dele fica com ele (não rola os contêineres de fora), mesmo sem ele rolar: uma prévia que dá zoom com a roda, um mapa, um gráfico. |
| `can_focus` | Recebe foco | lógico | Pode receber o foco do teclado (clique ou Tab). |
| `draggable` | Arrastável | lógico | Pode ser arrastado (o `drag` diz o que vai junto). |
| `accepts_drops` | Aceita soltar | lógico | Aceita o que é solto em cima dele (`on_drop`). |
| `drag_data` | Dados do arrasto | texto | O que vai num arrasto que começa nele, sem handler (`"cliente:123"`: o tipo é o que vem antes do primeiro `:`); vazio: o arrasto declarado não leva nada. |
| `drag_label` | Rótulo do arrasto | texto | O texto que acompanha o mouse durante o arrasto; vazio: sem rótulo. |
| `drag_image` | Imagem do arrasto | componente | O `DragImage` que acompanha o cursor no arrasto (a imagem e o tamanho). |
| `drop_types` | Tipos aceitos ao soltar | texto | Os tipos que este destino aceita, separados por `;` (`"cliente;produto"`); vazio: quem decide é o `on_drag_over`. |
| `tab_stop` | Para no Tab | lógico | Entra na ordem do Tab. |
| `cursor` | Cursor do mouse | um de: `default`, `pointer`, `text`, `wait`, `crosshair`, `move`, `not_allowed`, `resize_horizontal`, `resize_vertical`, `resize_ne_sw`, `resize_nw_se`, `grab`, `grabbing`, `none` | O cursor do mouse em cima dele. |
| `hint` | Dica | texto | Dica mostrada com o mouse parado em cima. |
| `accessible_name` | Nome acessível | texto | O nome que o leitor de tela fala, no lugar do texto do controle (um botão só com ícone, um campo sem rótulo); vazio: o texto do próprio controle. |
| `popup_menu` | Menu de contexto | componente | Menu de contexto: abre no clique direito. |

### Fonte

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `font_family` | Fonte | texto | A família da fonte (`"Segoe UI"`, `"monospace"`); vazio: a do pai. |
| `font_size` | Tamanho da fonte | número | O tamanho da fonte, em pontos; sem valor: o do pai. |
| `font_weight` | Peso da fonte | um de: `default`, `thin`, `light`, `normal`, `medium`, `semibold`, `bold`, `black` | O peso da fonte (`normal`, `bold`...); sem valor: o do pai. |
| `font_italic` | Itálico | lógico | Itálico; ligado vale também para os filhos. |
| `font_underline` | Sublinhado | lógico | Sublinhado; ligado vale também para os filhos. |
| `font_strikeout` | Tachado | lógico | Tachado (uma linha no meio do texto); ligado vale também para os filhos. |

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
| `rotation` | Rotação | ângulo (graus) | Giro em graus, em torno do centro. |
| `scale` | Escala | número | A escala do controle e de tudo dentro dele (1 = normal): ele ocupa o mesmo retângulo no pai, e o desenho dele e o layout dos filhos acontecem num espaço do tamanho do retângulo dividido pela escala, ampliado na pintura. O clique, o foco e a área a repintar acompanham (`ControlBase::inner_size`). |
| `aspect_ratio` | Proporção | número | Proporção fixa (largura ÷ altura; 0 = livre), como o `VResponsive` do Vuetify. |
| `max_width` | Largura máxima | número | Largura máxima (0 = sem limite), como o `max-width` do CSS com margem automática. |
| `min_width` | Largura mínima | número | Largura mínima, altura mínima e máxima (0 = sem limite). |
| `min_height` | Altura mínima | número | Altura mínima: o layout e o `auto_size` não o deixam mais baixo (0 = sem limite). |
| `max_height` | Altura máxima | número | Altura máxima: o layout e o `auto_size` não o deixam mais alto (0 = sem limite; a mínima vence). |
| `self_align` | Alinhamento próprio | um de: `stretch`, `start`, `center`, `end` | Encaixado: estica na faixa ou fica com o próprio tamanho no começo, meio ou fim. |
| `auto_size` | Tamanho automático | lógico | O tamanho sai do conteúdo: a altura; num Label encaixado à esquerda ou à direita, também a largura. |
| `column_span` | Colunas ocupadas | inteiro | Quantas colunas ocupa num `GridLayout` (o `colspan`; 1 ou 0: uma). |
| `grid_row` | Linha na grade | inteiro | O lugar fixo num `GridLayout`: linha e coluna (-1: segue a ordem) e quantas linhas ocupa. O relatório do Tabernáculo, G12. |
| `grid_column` | Coluna na grade | inteiro | A coluna fixa num `GridLayout`, contando de 0 (-1: segue a ordem); vale junto com o `grid_row`. |
| `row_span` | Linhas ocupadas | inteiro | Quantas linhas ocupa num `GridLayout` (1 ou 0: uma). |
| `grow` | Crescer no espaço livre | número | Quanto do espaço que sobra na linha de um `FlowLayout` ele toma (o `flex-grow`): um `Layout` vazio com `grow` 1 é o espaçador que empurra o resto para o fim (L04). |

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro | Um número livre para o código (não muda nada na tela). |
| `tag_string` | Texto livre (TagString) | texto | Um texto livre para o código (um id, um caminho, um JSON pequeno). |

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
| `on_swipe` | o dedo deslizou rápido para um lado (`direction`: `left`, `right`, `up` ou `down`) sem ter rolado nada nesse eixo; sobe para os pais até um tratar | — |  |
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
