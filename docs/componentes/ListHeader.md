### Alinhar às linhas sem impor colunas

Ligue o `list` num `VirtualList` (ou `DataRepeater`) e diga, em `columns`, o título e o
componente do modelo de cada coluna (`part`). O título fica onde o componente está na primeira
linha à vista; a linha que não tem o componente, ou o esconde, não participa.

```json
{"class": "ListHeader", "name": "hdrMusicas", "list": "lstMusicas", "align": "top", "columns": [
  {"title": "#", "part": "lblNumero", "width": 40},
  {"title": "Título", "part": "lblTitulo", "width": 260, "sortable": true, "resizable": true},
  {"title": "Duração", "part": "lblDuracao", "width": 72, "align": "right"}]}
```

Arrastar a divisa muda a largura do componente em todas as linhas (`on_column_resize` avisa).
O clique num título com `sortable` troca a ordem e avisa o `on_sort` com o `part`: o app
ordena os dados e chama o `refresh` da lista (`sort_part` e `sort_descending` mostram a seta).
