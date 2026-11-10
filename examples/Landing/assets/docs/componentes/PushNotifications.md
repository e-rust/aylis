# PushNotifications

Notificações push: com `active`, registra quando o form abre (pede a permissão de notificar onde é preciso) e o token deste aparelho chega no `on_token` (fica em `token`, com o serviço em `provider`): o app manda os dois ao servidor dele, que envia por ali. Uma mensagem com o app aberto chega no `on_message`; o toque na notificação (com o app aberto ou fechado), no `on_opened`. Os dois deixam `title`, `body`, `data` (os dados, um objeto JSON; `data_value("chave")` pega um) e `message_id`.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `active` | Ativo | lógico |
| `show_in_foreground` | Mostrar com o app aberto | lógico |
| `sender_id` | ID do remetente (FCM) | texto |
| `project_id` | ID do projeto (FCM) | texto |
| `app_id` | ID do aplicativo | texto |
| `api_key` | Chave da API | texto |
| `vapid_key` | Chave VAPID (Web Push) | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_token` | — | — |  |
| `on_message` | — | — |  |
| `on_opened` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `PushNotifications1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "PushNotifications", "name": "PushNotifications1", "on_token": "push_notifications1_token" }
```

E o handler, um método do form (`self.PushNotifications1` já é um `PushNotifications`):

```rust
impl FrmPrincipal {
    pub fn push_notifications1_token(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.PushNotifications1.active(ui);
        self.PushNotifications1.set_active(ui, !atual);
    }
}
```
