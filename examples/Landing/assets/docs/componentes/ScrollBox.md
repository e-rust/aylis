# ScrollBox

![ScrollBox](img/ScrollBox.png)

Área com rolagem vertical. Os filhos usam o layout normal (por exemplo `align: top`, empilhados); o que passar da altura rola com a roda do mouse ou arrastando a barra. `on_scroll` avisa quando o usuário rola (roda, dedo, barra), com a posição nova; o que o código posiciona não avisa. `scroll_position` lê e grava a posição, `scroll_to(componente)` rola o mínimo para ele ficar inteiro à vista e `visible_components(lista, inteiros)` diz quais filhos da lista estão à vista (os caminhos, separados por `;`; em Rust, `ScrollBox::visible_children`).

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `horizontal` | Horizontal | lógico |
| `pull_to_refresh` | Puxar para atualizar | lógico |
| `refreshing` | Atualizando | lógico |
| `wheel_step` | Passo da roda | número |
| `smooth` | Rolagem suave | um de: `default`, `on`, `off` |

### Teclado

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `allowed_chars` | Caracteres permitidos | texto |
| `blocked_chars` | Caracteres bloqueados | texto |
| `only_allowed` | Só os permitidos | lógico |
| `chars_override` | Força os caracteres | lógico |
| `char_map` | Trocas de caracteres | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_end_reached` | — | — |  |
| `on_refresh` | — | — |  |
| `on_scroll` | — | — |  |

## Em código

No form, o `ScrollBox1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ScrollBox", "name": "ScrollBox1", "on_end_reached": "scroll_box1_end_reached" }
```

E o handler, um método do form (`self.ScrollBox1` já é um `ScrollBox`):

```rust
impl FrmPrincipal {
    pub fn scroll_box1_end_reached(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ScrollBox1.horizontal(ui);
        self.ScrollBox1.set_horizontal(ui, !atual);
    }
}
```
