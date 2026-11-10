# Notificações push (`PushNotifications`)

O componente `PushNotifications` (categoria "Device") registra o aparelho no serviço de push
do sistema e entrega ao app o que o servidor dele mandar. É não visual, como o `Timer`.

## No form

| Propriedade | Para quê |
|---|---|
| `active` | registra sozinho quando o form abre (padrão: ligado) |
| `show_in_foreground` | com o app aberto, a mensagem com título também aparece como notificação do sistema (padrão: ligado); sem isso, só o `on_message` |
| `sender_id`, `project_id`, `app_id`, `api_key` | o Firebase, para o Android (do `google-services.json`: `project_info.project_number`, `project_info.project_id`, `client[].client_info.mobilesdk_app_id`, `client[].api_key[].current_key`) |
| `vapid_key` | a chave pública VAPID do servidor, para o navegador |

Somente leitura: `token`, `provider` (`fcm`, `apns`, `webpush`, `wns`, `unifiedpush`),
`available`, `registered` e, da última mensagem, `title`, `body`, `data` (objeto JSON) e
`message_id`.

| Evento | Quando |
|---|---|
| `on_token` | o registro deu certo (ou o token mudou): mande `token` e `provider` ao seu servidor |
| `on_message` | uma mensagem chegou com o app aberto (o texto do evento são os dados) |
| `on_opened` | o usuário tocou na notificação, com o app aberto ou abrindo por ela |
| `on_error` | o registro falhou (o texto diz por quê) |

Métodos: `register()`, `unregister()` e `data_value("chave")` (um valor dos dados).

```rust
// rust/src/forms/principal.rs (o componente se chama `push` no form)
impl FrmPrincipal {
    pub fn push_token(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        // Mande ao servidor: ele envia para este aparelho por aqui.
        let token = args.text().unwrap_or_default();
        api::save_device(&token, &self.push.provider(ui));
    }

    pub fn push_opened(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let order = self.push.data_value(ui, "order");
        self.open_order(ui, &order);
    }
}
```

## Cada plataforma

| Plataforma | Serviço | O que precisa | Com o app fechado |
|---|---|---|---|
| Android | FCM | Google Play Services no aparelho; o projeto no Firebase com o app Android (o pacote e o SHA-1 da chave de assinatura) | a notificação aparece; o toque abre o app (`on_opened`) |
| iOS | APNs | a capacidade Push Notifications no App ID e o certificado do desenvolvedor (a publicação põe `aps-environment` e o `remote-notification`) | o sistema mostra; o toque abre o app |
| macOS | APNs | o app assinado com o direito `com.apple.developer.aps-environment` | o sistema mostra |
| Navegador | Web Push (VAPID) | a página publicada (`aylis-tool publish --target web`, com o `sw.js`) servida em https | o service worker mostra; o toque abre a página |
| Windows | WNS | o app empacotado em MSIX (sem a identidade de pacote o Windows recusa o canal) | as toasts do WNS aparecem; as raw só com o app aberto |
| Linux | UnifiedPush | um distribuidor instalado (ntfy, KUnifiedPush, NextPush...) | só com o app aberto |

No Android o Aylis não usa o SDK do Firebase (o build do Aylis não passa pelo Gradle): faz o
que o SDK faz, a instalação no Firebase Installations e o pedido do token ao Play Services.
O app publicado ganha o receiver das mensagens e as permissões sozinho, quando um form tem o
componente.

## Regras e ações: montar um fluxo (um app de chamadas, um alarme)

Três peças a mais, cada uma com os seus eventos; quem liga uma à outra é o app.

| Componente | O que faz |
|---|---|
| `PushTrigger` | olha as mensagens de um `PushNotifications` (`source`) e, quando os dados casam com o `filter`, executa a `action` |
| `WakeAppAction` | abre o app por cima da tela bloqueada e acende a tela, tocando, como uma chamada chegando |
| `NotifyAction` | uma notificação montada com os dados (título, texto, imagem, canal, prioridade, botões) |

**`PushTrigger`**: `filter` são condições sobre os dados separadas por `;` (`kind=call`,
`kind!=chat`, `caller` = a chave existe e não é vazia; vazio aceita tudo). Os dados incluem
também `title`, `body` e `message_id` da mensagem. `action` é qualquer componente com o
método `execute(dados)`. `replace_default` (padrão: ligado) não deixa aparecer a notificação
comum da mensagem. Evento: `on_trigger` (os dados).

**`WakeAppAction`**: `form` (o form que abre; vazio, o do componente), `title` e `text`
(modelos: `{caller}` vira o valor da chave nos dados; `{{` e `}}` são as chaves), `buttons`
(`answer:Atender;reject:Recusar`), `open_buttons` (os que abrem o app; `*`, todos; os outros
respondem sem abrir), `ringtone` (`default`, `notification`, `alarm`, `none`), `vibrate`,
`timeout` (segundos sem resposta; 0, sem limite), `call_style` (o estilo de chamada do
Android 12+, com os botões `answer` e `reject`/`decline`) e `turn_screen_on`. Eventos:
`on_open` (o app abriu pela ação), `on_button` (o id do botão) e `on_timeout`. Métodos:
`execute(dados)` e `stop()`, que para o toque, tira a notificação e deixa de mostrar o app por
cima do bloqueio (chame ao atender ou ao encerrar). `running` diz se está tocando.

**`NotifyAction`**: `title`, `text`, `image` (modelos; a imagem é um asset, um arquivo ou, no
Android, um endereço `https`), `channel`, `channel_name`, `priority`, `buttons` e
`open_buttons` (padrão `*`). Eventos: `on_click` e `on_button`. Métodos: `execute(dados)` e
`stop()`.

Um app de chamadas, por exemplo: `push1` (`PushNotifications`) → `regraChamada`
(`PushTrigger`, `source = push1`, `filter = kind=call`, `action = tocar`) → `tocar`
(`WakeAppAction`, `form = frmChamada`, `title = {caller}`, `text = Chamada de voz`,
`buttons = answer:Atender;reject:Recusar`, `call_style`). O servidor manda uma mensagem só de
dados, com prioridade alta:

```json
{ "message": { "token": "...", "android": { "priority": "high" },
  "data": { "kind": "call", "caller": "Ana", "call_id": "77" } } }
```

No `on_button` do `tocar`, `answer` liga a mídia do app e chama `stop()`; `reject` avisa o
servidor. O `on_timeout` registra a chamada perdida.

| Plataforma | Com o app fechado | Com o app aberto |
|---|---|---|
| Android | as regras ficam gravadas no aparelho e o próprio Android executa a ação: o app abre por cima do bloqueio (notificação de tela cheia) e toca até a resposta; os eventos chegam quando o app roda | igual |
| Outras | a notificação comum do sistema | o `PushTrigger` casa a mensagem; o `WakeAppAction` mostra o form, traz a janela para a frente e avisa por uma notificação (sem tocar); o `NotifyAction` usa a notificação do sistema |

No Android 14, a Play Store só deixa um app usar a tela cheia (`USE_FULL_SCREEN_INTENT`) se ele
for de chamadas ou de alarme; nos outros, o usuário libera nas configurações e, sem isso, a
chamada aparece como uma faixa no alto. No iOS, abrir por cima do bloqueio só existe pelo
CallKit com push de VoIP (ainda não feito).

## O que o servidor manda

O texto da mensagem segue um formato só, em todos os serviços que entregam o conteúdo cru
(Web Push, WNS raw, UnifiedPush):

```json
{ "title": "Pedido novo", "body": "Chegou o pedido 42", "id": "m-42", "data": { "order": "42" } }
```

Também vale o formato do FCM (`{"notification": {"title", "body"}, "data": {...}}`); um
texto que não é JSON vira o `body`.

- **FCM** (Android): a API HTTP v1, `https://fcm.googleapis.com/v1/projects/<projeto>/messages:send`,
  com `message.token`, `message.notification` (título e texto) e `message.data` (texto → texto).
- **APNs** (iOS, macOS): `https://api.push.apple.com/3/device/<token>` (ou o `api.sandbox` para
  o app de desenvolvimento), com o JWT da chave `.p8`; o corpo é `{"aps": {"alert": {"title",
  "body"}}, ...dados}`: os dados são as chaves de fora do `aps`.
- **Web Push**: o `token` é a assinatura em JSON (`endpoint` e `keys`); qualquer biblioteca de
  Web Push (com as chaves VAPID) envia o corpo acima, cifrado.
- **WNS**: um POST no endereço do canal (o `token`) com o token OAuth do app, `X-WNS-Type:
  wns/raw` e o corpo acima (ou `wns/toast` com o XML da toast; o `launch` em JSON vira os dados).
- **UnifiedPush**: um POST no endpoint (o `token`) com o corpo acima.
