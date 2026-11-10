# ChatBubble

![ChatBubble](img/ChatBubble.png)

O balão de conversa (`QChatMessage`; aqui `ChatBubble`, porque o `ChatMessage` do aylis é a mensagem do `ChatView`): `author` em cima, o `text` (várias linhas), a hora (`stamp`) e as iniciais (ou `avatar`, um arquivo) ao lado. `sent` põe o balão à direita na cor primária (a mensagem de quem usa o app). Para a conversa toda, o `ChatView`.

**Paleta:** Quasar · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `author` | Autor | texto |
| `text` | Texto | texto |
| `stamp` | Hora | texto |
| `sent` | Enviada | lógico |
| `avatar` | Avatar | texto |

## Em código

No form, o `ChatBubble1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ChatBubble", "name": "ChatBubble1" }
```

E o handler, um método do form (`self.ChatBubble1` já é um `ChatBubble`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ChatBubble1.author(ui);
        self.ChatBubble1.set_author(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
