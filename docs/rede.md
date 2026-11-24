# Rede: HTTP e WebSocket

Dois componentes não visuais da paleta **System** falam com servidores pela rede, com a
resposta sempre na thread da tela (nada de `post` à mão):

| Componente | O que faz |
|---|---|
| `HttpClient` | Pedidos HTTP com método, cabeçalhos, corpo e JSON; a resposta no `on_response` |
| `WebSocketClient` | Uma conexão WebSocket com reconexão sozinha; as mensagens no `on_message` |

Os dois valem no JSON do form, no protocolo e nas outras linguagens (propriedades, métodos e
eventos por nome), e funcionam em todas as plataformas: no desktop e no celular pelo `ureq` e
pelo `tungstenite` (com rustls, sem OpenSSL); no navegador, pelo `fetch` e pelo `WebSocket` do
próprio navegador. O Socket.IO não faz parte: um servidor Socket.IO fala WebSocket por baixo,
e o protocolo dele (os prefixos `40`, `42`...) fica com o app.

A biblioteca vem por provedor (como o `Barcode` e o `Export`): sem nada ligado em `provider`,
vale o do Aylis (`AylisHttpProvider`, `AylisWebSocketProvider`); um provedor seu, com outra
biblioteca, é um componente que devolve `Provided::Http` ou `Provided::WebSocket` (os contratos
`HttpProvider` e `WebSocketProvider` de `aylis_core::providers`).

## HttpClient

```json
{ "class": "HttpClient", "name": "httpApi", "base_url": "https://api.exemplo.com/v1",
  "headers": [{ "name": "Accept", "value": "application/json" }],
  "on_response": "http_api_response", "on_error": "http_api_error" }
```

| Propriedade | O que é |
|---|---|
| `base_url` | O começo dos endereços relativos (`login` vira `https://api.exemplo.com/v1/login`) |
| `headers` | Os cabeçalhos de todo pedido (os do pedido vencem) |
| `request_timeout` | O prazo de cada pedido, em ms (30 s; 0: sem prazo) |
| `max_response_size` | A maior resposta aceita, em bytes (16 MB) |
| `max_redirects` | Quantos redirecionamentos seguir (5; 0: a resposta 3xx volta como está) |
| `allow_http` | Aceita `http://` sem TLS para fora deste computador (desligado) |
| `provider` | Outro provedor (vazio: o do Aylis) |
| `busy` | Só leitura: há pedidos esperando resposta |

Os métodos devolvem o número do pedido, que volta no evento:

| Método | O que faz |
|---|---|
| `request(method, url, body, headers)` | Qualquer método; `headers` em JSON (`{"Authorization": "Bearer x"}`) ou linhas `Nome: valor` |
| `get(url)` | Um `GET` |
| `post_json(url, json)` | Um `POST` com `Content-Type: application/json` (o JSON é conferido antes) |
| `cancel(request)`, `cancel_all()` | Esquece a resposta (o pedido pode ter chegado ao servidor) |

O `on_response` chega com qualquer status (um 404 também é resposta) e o `on_error` só com
falha de transporte (endereço, TLS, prazo, limite). Os dois trazem um `HttpResult`: `request`,
`method`, `url` (o final, depois dos redirecionamentos), `status`, `ok` (2xx), `body` (texto),
`content_type`, `headers`, `error` e `elapsed_ms`.

```rust
pub fn btn_entrar_click(&mut self, ui: &mut Ui, _: Button) -> Result<(), String> {
    let corpo = json!({ "usuario": self.edtUsuario.text(ui), "senha": self.edtSenha.text(ui) });
    self.httpApi.post_json(ui, "auth/login", corpo.to_string())?;
    Ok(())
}

pub fn http_api_response(&mut self, ui: &mut Ui, _: HttpClient, r: HttpResult) -> Result<(), String> {
    if !r.ok {
        return Err(format!("o servidor respondeu {}", r.status));
    }
    let token = r.json()["token"].as_str().unwrap_or_default().to_string();
    self.httpApi.set_headers(ui, &[HttpHeader { name: "Authorization".into(), value: format!("Bearer {token}") }]);
    Ok(())
}
```

No Rust, também há o jeito `async`, que não dispara os eventos (dentro de um
`ui.spawn_local`, ligado a um dono):

```rust
let pedido = HttpRequest::post("auth/login").json_text(corpo.to_string());
let futuro = HttpClient::fetch(ui, self.httpApi.id(), pedido);
ui.spawn_local(form, futuro, |ui, resposta| { /* Result<HttpResponse, String> */ });
// Sem componente, pelo provedor do Aylis: aylis::providers::net::http(HttpRequest::get(url))
```

## WebSocketClient

```json
{ "class": "WebSocketClient", "name": "wsAlertas", "url": "wss://api.exemplo.com/alertas",
  "active": true, "on_message": "ws_alertas_message", "on_close": "ws_alertas_close" }
```

| Propriedade | O que é |
|---|---|
| `url` | O endereço `wss://` (o `ws://` só para este computador, ou com `allow_insecure`) |
| `protocols` | Os subprotocolos oferecidos (separados por `;`) |
| `headers` | Os cabeçalhos da abertura (no navegador não valem: o `WebSocket` dele não deixa) |
| `active` | Conecta (fora do designer); desligado, fecha |
| `auto_reconnect` | Caiu sem o app pedir: tenta de novo (ligado) |
| `reconnect_delay`, `reconnect_max_delay` | A espera da reconexão: começa em 1 s e dobra a cada falha seguida, até 30 s |
| `connect_timeout` | O prazo para abrir, em ms (10 s) |
| `max_message_size` | A maior mensagem aceita (16 MB); acima disso a conexão fecha com 1009 |
| `connected` | Só leitura: está aberta |

Métodos: `connect()`, `disconnect()`, `send(texto)` e `send_binary(base64)` (no Rust,
`send_text` e `send_bytes`). Eventos: `on_open`, `on_message` (o texto), `on_binary` (os bytes
em base64), `on_error` e `on_close`, com um `WebSocketClose` (`code`, `reason`,
`reconnecting` e `retry_ms`).

```rust
pub fn ws_alertas_message(&mut self, ui: &mut Ui, _: WebSocketClient, texto: String) -> Result<(), String> {
    let alerta: Value = serde_json::from_str(&texto).map_err(|e| e.to_string())?;
    self.lstAlertas.add(ui, alerta["titulo"].as_str().unwrap_or_default());
    Ok(())
}
```

## Segurança

- **TLS sempre verificado**, com as raízes do Mozilla embutidas (as mesmas em todo sistema).
  Não há opção para aceitar certificado inválido.
- **Sem TLS só para este computador:** `http://` e `ws://` valem para `localhost`, `127.x` e
  `::1`; para fora, só com `allow_http` / `allow_insecure` ligados (a decisão é do app).
- **Prazos** em todo pedido e na abertura do WebSocket, e **limite do tamanho** da resposta e
  da mensagem: um servidor que não para de mandar não esgota a memória.
- **Redirecionamentos contados**, sem descer de `https` para `http` e sem levar
  `Authorization`, `Cookie` e `Proxy-Authorization` para outro host. Um `303` (e o `301`/`302`
  de um `POST`) segue com `GET`, sem corpo, como o navegador.
- **Cabeçalho não injeta cabeçalho:** nome inválido ou valor com quebra de linha é recusado.
- No navegador, o que é dele continua dele: o CORS, o conteúdo misto, os cookies (só para o
  mesmo site), os redirecionamentos (até 20, ou nenhum com `max_redirects: 0`) e o limite de
  tamanho, conferido pelo `Content-Length` antes e pelo tamanho lido depois.

## Desempenho

O HTTP usa quatro threads de trabalho, criadas no primeiro pedido e paradas esperando depois.
Cada conexão WebSocket tem uma thread que olha a cada 50 ms se a tela mandou algo (o
`tungstenite` não lê e escreve ao mesmo tempo com TLS): uma mensagem do app sai em até 50 ms, e
uma conexão parada acorda a thread 20 vezes por segundo, sem tocar na tela. Nada repinta nem
roda timer enquanto não chega nada; a reconexão é um `set_timeout` só.

## Onde funciona

Em todas as plataformas. No celular, o app precisa da permissão de internet (o Android já pede
no pacote gerado). Os dois ficam atrás das features `http` e `websocket` do crate `aylis`, que
a IDE liga quando um form usa a classe.
