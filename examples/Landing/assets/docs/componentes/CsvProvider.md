# CsvProvider

CSV (o padrão do `Export`): `separator` entre as colunas (`;`, o que o Excel em português abre direto), a vírgula decimal (`decimal_comma`) e o BOM do UTF-8 (para o Excel mostrar os acentos).

**Paleta:** Providers · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `separator` | Separadores | texto | O que vai entre as colunas (`;`, o que o Excel em português abre direto; `,`; `\t`). |
| `decimal_comma` | Vírgula decimal | lógico | Os números com vírgula decimal (`1234,5`); desligado, com ponto. |
| `bom` | Marca BOM (UTF-8) | lógico | Começa o arquivo com o BOM do UTF-8, para o Excel mostrar os acentos. |
| `header` | Linha de títulos | lógico | A primeira linha com os títulos. |

## Em código

No form, o `CsvProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "CsvProvider", "name": "CsvProvider1" }
```

E o handler, um método do form (`self.CsvProvider1` já é um `CsvProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.CsvProvider1.separator(ui);
        self.CsvProvider1.set_separator(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
