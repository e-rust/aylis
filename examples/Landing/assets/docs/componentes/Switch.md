# Switch

![Switch](img/Switch.png)

Chave liga/desliga. A bolinha desliza quando muda; o trilho muda de cor pela transição do tema.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `text` | Texto | texto |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `checked` | Marcado | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `Switch1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Switch", "name": "Switch1", "on_change": "switch1_change" }
```

E o handler, um método do form (`self.Switch1` já é um `Switch`):

```rust
impl FrmPrincipal {
    pub fn switch1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Switch1.text(ui);
        self.Switch1.set_text(ui, format!("{atual}!"));
    }
}
```
