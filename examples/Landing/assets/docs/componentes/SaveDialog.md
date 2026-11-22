# SaveDialog

Escolher onde salvar um arquivo (diálogo do sistema).

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O título do diálogo; vazio: o do sistema. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `filter` | Filtro | texto | `Imagens\|*.png;*.jpg\|Todos\|*.*` |
| `initial_dir` | Pasta inicial | texto | A pasta em que o diálogo abre; vazio: a que o sistema escolher. |
| `multiple` | Vários | lógico | Deixa escolher vários arquivos de uma vez. |
| `kind` | Tipo | um de: `file`, `folder` | `file` (o padrão) escolhe arquivos; `folder`, uma pasta (decidido em 21/11). Os filtros não valem no modo pasta, e no navegador e no celular ele volta como cancelado. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file_name` | Nome do arquivo | texto | O arquivo escolhido (com `multiple`, os caminhos separados por `;`); antes de abrir, o nome sugerido. |

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
