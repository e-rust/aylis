# Chart

![Chart](img/Chart.png)

Gráfico de linhas, barras, áreas, pizza e outros tipos, com legenda e eixos; as séries vêm por código ou ligadas aos dados.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `kind` | Tipo | um de: `line`, `bar`, `area`, `pie`, `donut`, `sparkline`, `heatmap`, `scatter`, `stacked_bar`, `waterfall`, `funnel`, `treemap`, `radar`, `horizontal_bar`, `bubble`, `polar_area`, `stacked_line`, `stacked_area`, `mixed`, `sankey`, `waffle` |  |
| `title` | Título | texto |  |
| `legend` | Legenda | lógico |  |
| `stepped` | Em degraus | lógico | Linhas em degraus. |
| `tension` | Curvatura | número | Linhas suavizadas (0 = retas; até 1), sem passar dos pontos (monotônica). |
| `value_format` | Formato dos números | texto | Números dos eixos e das dicas: vazio ou `short` (60k, 1.2M), `full` (60.000, com o separador do idioma) ou `percent` (60%). |
| `label_angle` | Ângulo dos rótulos | número | Ângulo dos rótulos do eixo X, em graus (45: inclinados, para nomes longos). |
| `title_align` | Alinhamento do título | texto | Onde fica o título: `left`, `center` ou `right`. |
| `title_bold` | Título em negrito | lógico |  |
| `area_opacity` | Opacidade da área | número | Opacidade do preenchimento das áreas (0 a 1). |
| `legend_position` | Posição da legenda | texto | Onde fica a legenda: `top_right`, `top_left`, `top` ou `bottom`. |
| `stacked` | Empilhado | lógico | Barras deitadas empilhadas (com o tipo `horizontal_bar`). |
| `colors` | Cores | texto | As cores, em ordem (`#2563EB;#16A34A`): das séries e, na pizza e na rosca, das fatias (sem elas, as fatias usam a paleta do gráfico, não a cor das séries). |
| `center_text` | Texto do centro | texto | O texto no meio da rosca: vazio = o total; `-` = nada; ou um texto (`{total}` vira o total). |
| `slice_labels` | Rótulos das fatias | texto | Os rótulos das fatias: `percent` (dentro), `outside` (nome e percentual fora, com uma linha) ou `none`. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `animate` | Animar | lógico | Animação ao trocar os dados. |
| `zoom` | Zoom | lógico | Roda do mouse aproxima, arrastar anda, duplo clique volta (eixo X). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `labels` | Rótulos | texto |  |
| `series` | Séries | texto |  |
| `json` | Dados (JSON) | texto | Os dados no formato do Chart.js (vazio: `labels` e `series`). |
| `clicked_series` | Série clicada | inteiro | O último ponto clicado (série e índice; -1 = nenhum). |
| `clicked_index` | Item clicado | inteiro |  |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `auto_scale` | Escala automática | lógico | Eixo Y automático; sem ele, de `y_min` a `y_max`. |
| `y_min` | Mínimo do eixo Y | número |  |
| `y_max` | Máximo do eixo Y | número |  |
| `log_scale` | Escala logarítmica | lógico | Escala logarítmica no eixo Y. |
| `reverse` | Invertido | lógico | Eixo Y invertido (o menor em cima). |
| `time_axis` | Eixo de tempo | lógico | Os rótulos são datas (`2026-10-27`, `2026-10-27 14:30`): o eixo X é de tempo. |

## Em código

No form, o `Chart1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Chart", "name": "Chart1" }
```

E o handler, um método do form (`self.Chart1` já é um `Chart`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Chart1.title(ui);
        self.Chart1.set_title(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
