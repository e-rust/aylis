# Notification

Notificação do sistema (Windows, Android), com ações e imagem.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `image` | Imagem | texto |
| `form` | Form | texto |
| `form_width` | Largura do form | número |
| `form_height` | Altura do form | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `title` | Título | texto |
| `text` | Texto | texto |
| `app_name` | Nome do aplicativo | texto |
| `app_id` | ID do aplicativo | texto |
| `notification_id` | ID da notificação | inteiro |
| `channel` | Canal | texto |
| `channel_name` | Nome do canal | texto |
| `priority` | Prioridade | um de: `low`, `default`, `high` |
| `actions` | Ações | texto |
| `ongoing` | Em andamento | lógico |
| `progress` | Progresso | número |
| `full_screen` | Tela cheia | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |
| `on_action` | — | — |  |

## Em código

No form, o `Notification1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Notification", "name": "Notification1", "on_click": "notification1_click" }
```

E o handler, um método do form (`self.Notification1` já é um `Notification`):

```rust
impl FrmPrincipal {
    pub fn notification1_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Notification1.title(ui);
        self.Notification1.set_title(ui, format!("{atual}!"));
    }
}
```
