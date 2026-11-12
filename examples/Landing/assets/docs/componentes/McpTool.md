# McpTool

Uma ferramenta do app para o agente (filha de um `McpServer`). `tool_name` é o nome que ele chama (vazio: o nome do componente), `description` diz quando usar, e `params` os parâmetros: um JSON Schema (`{"type": "object", "properties": {..}}`), só as propriedades (`{"cliente": {"type": "string"}}`) ou a forma curta `cliente: string, quantidade: integer, obs?: string` (o `?` marca o opcional).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tool_name` | Nome da ferramenta | texto |  |
| `description` | Descrição | texto |  |
| `params` | Parâmetros | texto |  |
| `read_only` | Somente leitura | lógico |  |
| `enabled` | Habilitado | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_execute` | — | — | sim: `result` |

## Em código

No form, o `McpTool1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "McpTool", "name": "McpTool1", "on_execute": "mcp_tool1_execute" }
```

E o handler, um método do form (`self.McpTool1` já é um `McpTool`):

```rust
impl FrmPrincipal {
    pub fn mcp_tool1_execute(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.McpTool1.tool_name(ui);
        self.McpTool1.set_tool_name(ui, format!("{atual}!"));
    }
}
```
