### Os ícones de uma coluna

Cada item é um ícone de pacote (`icon`, pintado na cor do texto) ou um arquivo do projeto
(`image`), e se acha pela posição ou pelo `name`. O `DataGrid` com `images` neste componente
mostra, nas colunas com `kind: icon`, o item cujo índice (ou nome) vem do campo do registro.

```json
{"class": "ImageList", "name": "imgEstados", "items": [
  {"name": "parada"}, {"name": "tocando", "icon": "fluent:play-fill"}, {"name": "pausada", "icon": "fluent:pause-fill"}]}
```
