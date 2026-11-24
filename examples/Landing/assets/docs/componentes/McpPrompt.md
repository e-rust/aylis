# McpPrompt

Um prompt pronto (filho de um `McpServer`): o cliente oferece ao usuário (no Claude Desktop, no menu de prompts) e manda ao agente o texto montado. `arguments` são os argumentos, separados por `;`: `cliente: o código do cliente; periodo?: o mês (AAAA-MM)` (o `?` marca o opcional; depois do `:`, a descrição).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `prompt_name` | Nome do prompt | texto | O nome que o cliente usa; vazio: o nome do componente. |
| `title` | Título | texto | O título que o cliente mostra no menu de prompts (vazio: o nome). |
| `description` | Descrição | texto | Para que serve o prompt, mostrado ao escolher. |
| `arguments` | Argumentos | texto | Os argumentos: `nome: descrição; outro?: descrição` (`?`: opcional). |
| `text` | Texto | texto | O texto do prompt, com `{argumento}` (sem `on_get`). |
| `enabled` | Habilitado | lógico | Oferecido ao cliente; desligado, ele não vê. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_get` | — | — |  |

## Em código

No form, o `McpPrompt1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "McpPrompt", "name": "McpPrompt1", "on_get": "mcp_prompt1_get" }
```

E o handler, um método do form (`self.McpPrompt1` já é um `McpPrompt`):

```rust
impl FrmPrincipal {
    pub fn mcp_prompt1_get(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.McpPrompt1.prompt_name(ui);
        self.McpPrompt1.set_prompt_name(ui, format!("{atual}!"));
    }
}
```
