# SlideItem

![SlideItem](img/SlideItem.png)

O item que desliza (`QSlideItem`): uma linha de lista (`text`, `detail`, `icon`) que se arrasta para os lados e mostra a ação de baixo (`left_text`/`left_icon` na cor `left_color`, ao puxar para a direita; `right_*` ao puxar para a esquerda). Soltando depois de um terço, dispara `on_left` ou `on_right` e volta.

**Paleta:** Quasar · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `text` | Texto | texto | O texto principal da linha. |
| `detail` | Detalhe | texto | A segunda linha, menor e apagada; vazio: só o `text`, centrado. |
| `icon` | Ícone | texto | O ícone à esquerda da linha (vazio: sem ícone). |
| `left_text` | Texto da esquerda | texto | O texto da ação que aparece ao puxar para a direita. |
| `left_icon` | Ícone da esquerda | texto | O ícone da ação da esquerda; com ele e o `left_text` vazios, não puxa para a direita. |
| `left_color` | Cor da esquerda | cor | O fundo da ação da esquerda; vazio: a cor de sucesso do tema. |
| `right_text` | Texto da direita | texto | O texto da ação que aparece ao puxar para a esquerda. |
| `right_icon` | Ícone da direita | texto | O ícone da ação da direita; com ele e o `right_text` vazios, não puxa para a esquerda. |
| `right_color` | Cor da direita | cor | O fundo da ação da direita; vazio: a cor de perigo do tema. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_left` | — | — |  |
| `on_right` | — | — |  |

## Em código

No form, o `SlideItem1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "SlideItem", "name": "SlideItem1", "on_left": "slide_item1_left" }
```

E o handler, um método do form (`self.SlideItem1` já é um `SlideItem`):

```rust
impl FrmPrincipal {
    pub fn slide_item1_left(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.SlideItem1.text(ui);
        self.SlideItem1.set_text(ui, format!("{atual}!"));
    }
}
```
