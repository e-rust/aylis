# McpResource

Um recurso que o agente lê (filho de um `McpServer`): um conteúdo do app pelo endereço (`uri`: `app://tabelas/clientes`), com nome, título, descrição e tipo (`mime_type`). Um `uri` com `{nome}` é um modelo (`app://tabelas/{tabela}`): vale para todo endereço que bate com ele, e o `on_read` recebe os pedaços.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `uri` | Endereço (URI) | texto | O endereço (ou o modelo, com `{nome}`). |
| `resource_name` | Nome do recurso | texto | O nome que o agente vê; vazio: o nome do componente. |
| `title` | Título | texto | O título que o cliente mostra (vazio: o nome). |
| `description` | Descrição | texto | O que o recurso tem, para o agente decidir se o lê. |
| `mime_type` | Tipo do conteúdo | texto | O tipo do conteúdo (`text/plain`, `application/json`, `text/markdown`...). |
| `text` | Texto | texto | O conteúdo fixo (sem `on_read`). |
| `enabled` | Habilitado | lógico | Oferecido ao agente; desligado, ele não vê. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_read` | — | — |  |

## Em código

No form, o `McpResource1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "McpResource", "name": "McpResource1", "on_read": "mcp_resource1_read" }
```

E o handler, um método do form (`self.McpResource1` já é um `McpResource`):

```rust
impl FrmPrincipal {
    pub fn mcp_resource1_read(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.McpResource1.uri(ui);
        self.McpResource1.set_uri(ui, format!("{atual}!"));
    }
}
```
