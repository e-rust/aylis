# WebSocketClient

Uma conexão WebSocket (componente não visual) com reconexão. `active` liga (fora do designer) e desligado fecha; `url` é o `wss://` (o `ws://` sem TLS só para este computador, ou com `allow_insecure`); `protocols` os subprotocolos; `headers` os cabeçalhos da abertura (no navegador não valem).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `url` | Endereço (URL) | texto | O endereço (`wss://`; `ws://` só para este computador, salvo `allow_insecure`). |
| `protocols` | Subprotocolos | lista de textos (separados por `;`) | Os subprotocolos oferecidos. |
| `headers` | Cabeçalhos | lista de `HttpHeader` | Os cabeçalhos da abertura (a lista de `{name, value}`). |
| `active` | Ativo | lógico | Conecta (fora do designer); desligado, fecha. |
| `auto_reconnect` | Reconectar sozinho | lógico | Reconecta sozinho quando a conexão cai, esperando de `reconnect_delay` até `reconnect_max_delay` (a espera dobra a cada tentativa). |
| `reconnect_delay` | Espera para reconectar | duração (ms) | A primeira espera para reconectar, em milissegundos. |
| `reconnect_max_delay` | Maior espera para reconectar | duração (ms) | A maior espera para reconectar, em milissegundos. |
| `connect_timeout` | Tempo limite para conectar | duração (ms) | O prazo para abrir, em milissegundos. |
| `max_message_size` | Maior mensagem | inteiro | A maior mensagem aceita, em bytes (acima disso a conexão fecha com 1009). |
| `allow_insecure` | Aceitar sem TLS | lógico | Aceita `ws://` sem TLS para fora deste computador. |
| `provider` | Provedor | componente | O provedor (vazio: o do Aylis). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_open` | — | — |  |
| `on_message` | — | — |  |
| `on_binary` | — | — |  |
| `on_close` | o form fechou | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `WebSocketClient1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "WebSocketClient", "name": "WebSocketClient1", "on_open": "web_socket_client1_open" }
```

E o handler, um método do form (`self.WebSocketClient1` já é um `WebSocketClient`):

```rust
impl FrmPrincipal {
    pub fn web_socket_client1_open(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.WebSocketClient1.url(ui);
        self.WebSocketClient1.set_url(ui, format!("{atual}!"));
    }
}
```
