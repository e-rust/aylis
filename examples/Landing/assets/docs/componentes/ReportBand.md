# ReportBand

Uma banda do relatório: um contêiner com o tipo dela. `group_field`, nos cabeçalhos e rodapés de grupo: o campo que forma o grupo; `new_page`: o grupo começa numa página nova. `data_source`, nas bandas de sub-detalhe: o dataset do detalhe (ligado ao do relatório pelo `master_source`); nos textos delas, um campo que o detalhe não tem vem do mestre.

**Paleta:** Reports · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `kind` | Tipo | um de: `title`, `page_header`, `group_header`, `detail`, `group_footer`, `summary`, `page_footer`, `sub_header`, `sub_detail`, `sub_footer` |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `new_page` | Nova página | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `group_field` | Campo de grupo | texto |
| `data_source` | Fonte de dados | componente |

## Em código

No form, o `ReportBand1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ReportBand", "name": "ReportBand1" }
```

E o handler, um método do form (`self.ReportBand1` já é um `ReportBand`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ReportBand1.group_field(ui);
        self.ReportBand1.set_group_field(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
