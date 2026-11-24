# MessageDialog

Caixa de mensagem com título, texto, tipo e botões; on_close recebe a resposta.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | Título da janela; vazio: o do tipo ("Confirmação", "Aviso"...). |
| `text` | Texto | texto | A mensagem (com `input`, o rótulo do campo). |
| `kind` | Tipo | um de: `information`, `warning`, `error`, `confirmation` | O tipo, que dá o ícone e o título padrão: `information`, `warning`, `error` ou `confirmation`. |
| `icon` | Ícone | texto | O ícone no círculo (um nome de ícone, `"pacote:nome"`); vazio: o do tipo. |
| `icon_color` | Cor do ícone | cor | A cor do círculo do ícone (um token do tema ou uma cor); vazio: a do tipo. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `buttons` | Botões | texto | Os botões, na ordem em que aparecem: `"yes,no"`, `"ok,cancel"`, `"yes,no,cancel"`... |
| `input` | Campo de texto | lógico | Um campo de texto embaixo da mensagem (a pergunta do `prompt`). |
| `required` | Obrigatório | lógico | Com `input`, o OK fica desligado enquanto o campo está vazio (só espaços contam como vazio). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | texto | O texto do campo: o inicial e, depois do OK, o digitado (o Cancelar não muda). |

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
