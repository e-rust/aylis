# NotifyAction

Uma notificação montada com os dados: `execute(dados)` mostra `title`, `text` e `image` (modelos: `{chave}` vira o valor nos dados; a imagem é um asset, um arquivo ou, no Android, um endereço `https`), no canal e com a prioridade escolhidos, e os botões (`buttons`, `id:Texto;...`; os de `open_buttons` abrem o app, `*`: todos). Eventos: `on_click` (o toque no corpo, com os dados) e `on_button` (o id do botão). `stop()` tira a notificação.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto |  |
| `text` | Texto | texto |  |
| `image` | Imagem | texto |  |
| `channel` | Canal | texto |  |
| `channel_name` | Nome do canal | texto |  |
| `priority` | Prioridade | um de: `low`, `default`, `high` |  |
| `buttons` | Botões | texto |  |
| `open_buttons` | Botões que abrem o app | texto |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |
| `on_button` | — | — |  |

## Em código

No form, o `NotifyAction1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "NotifyAction", "name": "NotifyAction1", "on_click": "notify_action1_click" }
```

E o handler, um método do form (`self.NotifyAction1` já é um `NotifyAction`):

```rust
impl FrmPrincipal {
    pub fn notify_action1_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.NotifyAction1.title(ui);
        self.NotifyAction1.set_title(ui, format!("{atual}!"));
    }
}
```
