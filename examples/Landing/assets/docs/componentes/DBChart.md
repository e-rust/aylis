# DBChart

![DBChart](img/DBChart.png)

Gráfico ligado a um DataSource: um ponto por registro, o rótulo de `label_field` e uma série por campo de `value_fields` (`Vendas;Custos`). Acompanha o dataset (abrir, filtrar, gravar). O tipo, as escalas e o resto são os do `Chart`.

**Paleta:** Data Controls · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `kind` | Tipo | um de: `line`, `bar`, `area`, `pie`, `donut`, `sparkline`, `heatmap`, `scatter`, `stacked_bar`, `waterfall`, `funnel`, `treemap`, `radar`, `horizontal_bar`, `bubble`, `polar_area`, `stacked_line`, `stacked_area`, `mixed`, `sankey`, `waffle` |
| `title` | Título | texto |
| `legend` | Legenda | lógico |
| `stepped` | Em degraus | lógico |
| `tension` | Curvatura | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `animate` | Animar | lógico |
| `zoom` | Zoom | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `data_source` | Fonte de dados | componente |
| `label_field` | Campo dos rótulos | texto |
| `value_fields` | Campos dos valores | texto |
| `labels` | Rótulos | texto |
| `series` | Séries | texto |
| `json` | Dados (JSON) | texto |
| `clicked_series` | Série clicada | inteiro |
| `clicked_index` | Item clicado | inteiro |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `auto_scale` | Escala automática | lógico |
| `y_min` | Mínimo do eixo Y | número |
| `y_max` | Máximo do eixo Y | número |
| `log_scale` | Escala logarítmica | lógico |
| `reverse` | Invertido | lógico |
| `time_axis` | Eixo de tempo | lógico |

## Em código

No form, o `DBChart1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DBChart", "name": "DBChart1" }
```

E o handler, um método do form (`self.DBChart1` já é um `DBChart`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DBChart1.label_field(ui);
        self.DBChart1.set_label_field(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
