# OpenDialog

Escolher um arquivo para abrir (diálogo do sistema).

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

No form, o `OpenDialog1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "OpenDialog", "name": "OpenDialog1", "on_select": "open_dialog1_select" }
```

E o handler, um método do form (`self.OpenDialog1` já é um `OpenDialog`):

```rust
impl FrmPrincipal {
    pub fn open_dialog1_select(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.OpenDialog1.title(ui);
        self.OpenDialog1.set_title(ui, format!("{atual}!"));
    }
}
```
