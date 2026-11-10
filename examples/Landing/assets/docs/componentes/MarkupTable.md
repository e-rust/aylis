# MarkupTable

![MarkupTable](img/MarkupTable.png)

A tabela simples (`QMarkupTable`): `columns` com os títulos (`Nome;Idade;Cidade`) e `rows` com uma linha por linha de texto (as células separadas por `;`). Os números ficam à direita. `dense` aperta as linhas, `striped` alterna o fundo e `separator` escolhe as linhas de separação. Para dados de banco, use o `DataGrid`.

**Paleta:** Quasar · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `dense` | Compacto | lógico |
| `striped` | Listrado | lógico |
| `separator` | Separadores | um de: `horizontal`, `vertical`, `cell`, `none` |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `columns` | Colunas | texto |
| `rows` | Linhas | texto |

## Em código

No form, o `MarkupTable1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MarkupTable", "name": "MarkupTable1" }
```

E o handler, um método do form (`self.MarkupTable1` já é um `MarkupTable`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MarkupTable1.columns(ui);
        self.MarkupTable1.set_columns(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
