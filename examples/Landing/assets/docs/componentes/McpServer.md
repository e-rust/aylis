# McpServer

Liga o app a agentes de IA pelo MCP. `active` liga (fora do designer); `port` 0 escolhe uma porta livre (o endereço fica em `url`); `token` vazio gera um aleatório ao ligar. Só aceita conexões deste computador, com o token.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Liga o servidor (fora do designer); desligado, ele para. |
| `port` | Porta | inteiro | A porta em `127.0.0.1`; 0: uma porta livre (o endereço fica em `url`). |
| `token` | Token de acesso | texto | O segredo que o agente precisa mandar; vazio: um aleatório gerado ao ligar. |
| `server_name` | Nome do servidor | texto | O nome que o agente vê (vazio: o nome do componente). |
| `instructions` | Instruções | texto | O que o agente lê ao conectar: para que serve o app e como usar as ferramentas. |
| `expose_ui` | Expor a tela | lógico | Dá ao agente as ferramentas genéricas de tela (ler a árvore, mexer em propriedades, clicar, os dados). |
| `read_only` | Somente leitura | lógico | Das ferramentas de tela, só as que leem (nenhuma muda a tela nem os registros). |
| `forms` | Forms liberados | texto | Os forms que o agente alcança, por nome separados por vírgula; vazio: todos. |
| `call_timeout` | Tempo limite da chamada | duração (ms) | Quanto uma chamada pode demorar, em milissegundos (vale ao ligar). |

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
