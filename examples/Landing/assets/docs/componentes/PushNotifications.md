# PushNotifications

Notificações push: com `active`, registra quando o form abre (pede a permissão de notificar onde é preciso) e o token deste aparelho chega no `on_token` (fica em `token`, com o serviço em `provider`): o app manda os dois ao servidor dele, que envia por ali. Uma mensagem com o app aberto chega no `on_message`; o toque na notificação (com o app aberto ou fechado), no `on_opened`. Os dois deixam `title`, `body`, `data` (os dados, um objeto JSON; `data_value("chave")` pega um) e `message_id`.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Registrar quando o form abrir. |
| `show_in_foreground` | Mostrar com o app aberto | lógico | Com o app aberto, a mensagem com título também vira notificação do sistema. |
| `sender_id` | ID do remetente (FCM) | texto | Android (FCM): o número do remetente do projeto Firebase (`project_number` do `google-services.json`). |
| `project_id` | ID do projeto (FCM) | texto | Android (FCM): o id do projeto Firebase (`project_id` do `google-services.json`). |
| `app_id` | ID do aplicativo | texto | Android (FCM): o id do app no Firebase (`mobilesdk_app_id` do `google-services.json`). |
| `api_key` | Chave da API | texto | Android (FCM): a chave da API (`current_key` do `google-services.json`). |
| `vapid_key` | Chave VAPID (Web Push) | texto | Navegador (Web Push): a chave pública VAPID do servidor do app. |

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
