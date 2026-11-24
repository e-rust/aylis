# ToastStack

![ToastStack](img/ToastStack.png)

Avisos que não bloqueiam, empilhados de cima para baixo, encostados à direita da pilha (o "toast"): título, mensagem e a cor do tipo. Cada um some no prazo (ou no clique); um aviso com o mesmo título de um que está na tela só renova o prazo dele. Entra deslizando e esmaecendo.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `card_width` | Largura do cartão | número | A largura máxima de um cartão (ele encolhe até o texto, e nunca passa da pilha). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_dismiss` | — | — |  |

## Em código

No form, o `ToastStack1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ToastStack", "name": "ToastStack1", "on_dismiss": "toast_stack1_dismiss" }
```

E o handler, um método do form (`self.ToastStack1` já é um `ToastStack`):

```rust
impl FrmPrincipal {
    pub fn toast_stack1_dismiss(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ToastStack1.card_width(ui);
        self.ToastStack1.set_card_width(ui, atual + 10.0);
    }
}
```
