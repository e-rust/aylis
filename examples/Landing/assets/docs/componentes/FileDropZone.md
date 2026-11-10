# FileDropZone

![FileDropZone](img/FileDropZone.png)

Área para soltar arquivos (ou clicar e escolher): borda tracejada, ícone de envio e a lista do que foi escolhido, cada um com "×".

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `title` | Título | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `filter` | Filtro | texto |
| `multiple` | Vários | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `files` | Arquivos | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `FileDropZone1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "FileDropZone", "name": "FileDropZone1", "on_change": "file_drop_zone1_change" }
```

E o handler, um método do form (`self.FileDropZone1` já é um `FileDropZone`):

```rust
impl FrmPrincipal {
    pub fn file_drop_zone1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.FileDropZone1.files(ui);
        self.FileDropZone1.set_files(ui, format!("{atual}!"));
    }
}
```
