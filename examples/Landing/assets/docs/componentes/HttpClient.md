# HttpClient

Um cliente HTTP (componente não visual). `base_url` é o começo dos endereços relativos; `headers` vão em todo pedido (o `Authorization`, o `Accept`); `request_timeout`, `max_response_size` e `max_redirects` são os limites; `allow_http` aceita `http://` para fora deste computador (desligado: só `https://`).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `base_url` | Endereço base | texto | O começo dos endereços relativos (`https://api.exemplo.com/v1`). |
| `headers` | Cabeçalhos | lista de `HttpHeader` | Os cabeçalhos de todo pedido (a lista de `{name, value}`). |
| `request_timeout` | Prazo do pedido | duração (ms) | O prazo de cada pedido, em milissegundos (0: sem prazo). |
| `max_response_size` | Maior resposta | inteiro | A maior resposta aceita, em bytes. |
| `max_redirects` | Máximo de redirecionamentos | inteiro | Quantos redirecionamentos seguir (0: nenhum). |
| `allow_http` | Aceitar HTTP sem TLS | lógico | Aceita `http://` sem TLS para fora deste computador. |
| `provider` | Provedor | componente | O provedor (vazio: o do Aylis). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_response` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `HttpClient1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "HttpClient", "name": "HttpClient1", "on_response": "http_client1_response" }
```

E o handler, um método do form (`self.HttpClient1` já é um `HttpClient`):

```rust
impl FrmPrincipal {
    pub fn http_client1_response(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.HttpClient1.base_url(ui);
        self.HttpClient1.set_base_url(ui, format!("{atual}!"));
    }
}
```
