# RadioButton

![RadioButton](img/RadioButton.png)

Opção exclusiva. Ao ser clicado, desmarca os irmãos (mesmo pai) que tenham o mesmo `group`.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `text` | Texto | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `group` | Grupo | texto |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `checked` | Marcado | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `RadioButton1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "RadioButton", "name": "RadioButton1", "on_change": "radio_button1_change" }
```

E o handler, um método do form (`self.RadioButton1` já é um `RadioButton`):

```rust
impl FrmPrincipal {
    pub fn radio_button1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.RadioButton1.text(ui);
        self.RadioButton1.set_text(ui, format!("{atual}!"));
    }
}
```
