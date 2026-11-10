# McpServer

Liga o app a agentes de IA pelo MCP. `active` liga (fora do designer); `port` 0 escolhe uma porta livre (o endereço fica em `url`); `token` vazio gera um aleatório ao ligar. Só aceita conexões deste computador, com o token.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `active` | Ativo | lógico |
| `port` | Porta | inteiro |
| `token` | Token de acesso | texto |
| `server_name` | Nome do servidor | texto |
| `instructions` | Instruções | texto |
| `expose_ui` | Expor a tela | lógico |
| `read_only` | Somente leitura | lógico |
| `forms` | Forms liberados | texto |
| `call_timeout` | Tempo limite da chamada | duração (ms) |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_call` | — | — | sim (recusar com a exceção) |
| `on_error` | — | — |  |

## Em código

No form, o `McpServer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "McpServer", "name": "McpServer1", "on_call": "mcp_server1_call" }
```

E o handler, um método do form (`self.McpServer1` já é um `McpServer`):

```rust
impl FrmPrincipal {
    pub fn mcp_server1_call(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.McpServer1.active(ui);
        self.McpServer1.set_active(ui, !atual);
    }
}
```
