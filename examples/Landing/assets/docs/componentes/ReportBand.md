# ReportBand

Uma banda do relatório: um contêiner com o tipo dela. `group_field`, nos cabeçalhos e rodapés de grupo: o campo que forma o grupo; `new_page`: o grupo começa numa página nova. `data_source`, nas bandas de sub-detalhe: o dataset do detalhe (ligado ao do relatório pelo `master_source`); nos textos delas, um campo que o detalhe não tem vem do mestre.

**Paleta:** Reports · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `kind` | Tipo | um de: `title`, `page_header`, `group_header`, `detail`, `group_footer`, `summary`, `page_footer`, `sub_header`, `sub_detail`, `sub_footer` | O papel da banda: título, cabeçalho e rodapé de página ou de grupo, detalhe, sumário ou as do sub-detalhe. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `new_page` | Nova página | lógico | O grupo começa numa página nova. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `group_field` | Campo de grupo | texto | Nos cabeçalhos e rodapés de grupo: o campo que forma o grupo (um grupo novo quando o valor muda). |
| `data_source` | Fonte de dados | componente | Nas bandas de sub-detalhe: o `DataSource` do detalhe (ligado ao do relatório pelo `master_source`). |

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
