# ChoroplethMap

![ChoroplethMap](img/ChoroplethMap.png)

Mapa coroplético: as regiões de um GeoJSON (`geojson`: o texto ou um arquivo/asset) pintadas pela escala de cor do valor (`values` = `"SP=10;RJ=5"`, pelo nome em `name_property`). Passar o mouse mostra a região e o valor; clique: `on_region_click`.

**Paleta:** BI · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `low_color` | Cor do menor valor | cor | A cor do menor valor; vazio: a cor primária do tema, clara. |
| `high_color` | Cor do maior valor | cor | A cor do maior valor; vazio: a cor primária do tema. |
| `decimals` | Casas decimais | inteiro | Casas decimais do valor mostrado ao passar o mouse. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `geojson` | Mapa (GeoJSON) | texto | As regiões em GeoJSON: o texto ou o caminho de um arquivo/asset. |
| `name_property` | Propriedade do nome | texto | A propriedade do GeoJSON com o nome de cada região (sem ela, a `name`). |
| `values` | Valores | texto | O valor de cada região pelo nome: `"SP=10;RJ=5"`. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_region_click` | — | — |  |

## Em código

No form, o `ChoroplethMap1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ChoroplethMap", "name": "ChoroplethMap1", "on_region_click": "choropleth_map1_region_click" }
```

E o handler, um método do form (`self.ChoroplethMap1` já é um `ChoroplethMap`):

```rust
impl FrmPrincipal {
    pub fn choropleth_map1_region_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ChoroplethMap1.geojson(ui);
        self.ChoroplethMap1.set_geojson(ui, format!("{atual}!"));
    }
}
```
