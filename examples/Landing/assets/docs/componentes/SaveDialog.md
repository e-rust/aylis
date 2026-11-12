# SaveDialog

Escolher onde salvar um arquivo (diálogo do sistema).

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto |  |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `filter` | Filtro | texto | `Imagens\|*.png;*.jpg\|Todos\|*.*` |
| `initial_dir` | Pasta inicial | texto |  |
| `multiple` | Vários | lógico |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file_name` | Nome do arquivo | texto |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_select` | — | — |  |
| `on_cancel` | — | — |  |

## Em código

No form, o `SaveDialog1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "SaveDialog", "name": "SaveDialog1", "on_select": "save_dialog1_select" }
```

E o handler, um método do form (`self.SaveDialog1` já é um `SaveDialog`):

```rust
impl FrmPrincipal {
    pub fn save_dialog1_select(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.SaveDialog1.title(ui);
        self.SaveDialog1.set_title(ui, format!("{atual}!"));
    }
}
```
