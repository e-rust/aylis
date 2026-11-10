# MessageDialog

Caixa de mensagem com título, texto, tipo e botões; on_close recebe a resposta.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `title` | Título | texto |
| `text` | Texto | texto |
| `kind` | Tipo | um de: `information`, `warning`, `error`, `confirmation` |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `buttons` | Botões | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_close` | o form fechou | — |  |

## Em código

No form, o `MessageDialog1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MessageDialog", "name": "MessageDialog1", "on_close": "message_dialog1_close" }
```

E o handler, um método do form (`self.MessageDialog1` já é um `MessageDialog`):

```rust
impl FrmPrincipal {
    pub fn message_dialog1_close(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MessageDialog1.title(ui);
        self.MessageDialog1.set_title(ui, format!("{atual}!"));
    }
}
```
