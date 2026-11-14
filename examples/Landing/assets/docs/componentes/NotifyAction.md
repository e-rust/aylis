# NotifyAction

Uma notificação montada com os dados: `execute(dados)` mostra `title`, `text` e `image` (modelos: `{chave}` vira o valor nos dados; a imagem é um asset, um arquivo ou, no Android, um endereço `https`), no canal e com a prioridade escolhidos, e os botões (`buttons`, `id:Texto;...`; os de `open_buttons` abrem o app, `*`: todos). Eventos: `on_click` (o toque no corpo, com os dados) e `on_button` (o id do botão). `stop()` tira a notificação.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O título da notificação; `{chave}` vira o valor da chave nos dados. |
| `text` | Texto | texto | O texto da notificação; `{chave}` vira o valor da chave nos dados. |
| `image` | Imagem | texto | A imagem grande: um asset, um arquivo ou, no Android, um endereço `https`; vazio: sem imagem. |
| `channel` | Canal | texto | O canal do Android (o usuário configura som e destaque por canal). |
| `channel_name` | Nome do canal | texto | O nome do canal que o usuário vê nas configurações do Android. |
| `priority` | Prioridade | um de: `low`, `default`, `high` | `low` (silenciosa, só na lista), `default` ou `high` (aparece por cima do app, com som). |
| `buttons` | Botões | texto | Os botões da notificação: `id:Texto;...`; o toque chega em `on_button` com o id. |
| `open_buttons` | Botões que abrem o app | texto | Os botões (pelo id, separados por `;`) que abrem o app; `*`: todos. |

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
