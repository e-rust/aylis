# McpTool

Uma ferramenta do app para o agente (filha de um `McpServer`). `tool_name` é o nome que ele chama (vazio: o nome do componente), `description` diz quando usar, e `params` os parâmetros: um JSON Schema (`{"type": "object", "properties": {..}}`), só as propriedades (`{"cliente": {"type": "string"}}`) ou a forma curta `cliente: string, quantidade: integer, obs?: string` (o `?` marca o opcional).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tool_name` | Nome da ferramenta | texto | O nome que o agente chama; vazio: o nome do componente. |
| `description` | Descrição | texto | O que a ferramenta faz e quando usar (o agente lê para escolher). |
| `params` | Parâmetros | texto | Os parâmetros: um JSON Schema, só as propriedades ou a forma curta `cliente: string, quantidade: integer, obs?: string` (`?`: opcional); vazio: nenhum. |
| `read_only` | Somente leitura | lógico | Avisa o agente que a ferramenta só lê, não muda nada. |
| `enabled` | Habilitado | lógico | Oferecida ao agente; desligada, ele não a vê. |

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
