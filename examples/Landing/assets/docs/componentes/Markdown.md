# Markdown

![Markdown](img/Markdown.png)

Mostra texto em Markdown com as cores e fontes do tema: títulos, listas, tabelas, código realçado, imagens e links, com seleção e cópia.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `editor_look` | Código como no editor | lógico |
| `code_theme` | Tema do código | texto |
| `code_language` | Linguagem do código | texto |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `text` | Texto | texto |
| `source` | Origem | texto |
| `anchor` | Âncora | texto |
| `highlight` | Destacar | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_link` | — | — |  |

## Em código

No form, o `Markdown1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Markdown", "name": "Markdown1", "on_link": "markdown1_link" }
```

E o handler, um método do form (`self.Markdown1` já é um `Markdown`):

```rust
impl FrmPrincipal {
    pub fn markdown1_link(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Markdown1.text(ui);
        self.Markdown1.set_text(ui, format!("{atual}!"));
    }
}
```
