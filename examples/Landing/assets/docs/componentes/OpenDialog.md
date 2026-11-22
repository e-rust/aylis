# OpenDialog

Escolher um arquivo para abrir (diálogo do sistema).

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

## Mais sobre o OpenDialog

### Escolher uma pasta

Com `kind` = `folder`, o diálogo escolhe uma pasta (com `multiple`, várias); os filtros e o
`save` não valem nele. No navegador e no celular, a escolha de pasta não existe: o diálogo
volta como cancelado (`on_cancel`).
