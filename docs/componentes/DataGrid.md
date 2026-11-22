### Ícones e imagens nas colunas

Uma coluna com `kind: icon` mostra um ícone do `ImageList` ligado em `images`: o campo do
registro diz qual (a posição ou o nome). Com `kind: image`, a coluna desenha a imagem guardada
no campo (um `blob` com PNG, JPEG...), inteira na altura da linha: para capas maiores, aumente
o `row_height` (a altura de todas as linhas).

```json
{"class": "DataGrid", "name": "grdMusicas", "data_source": "dsMusicas", "images": "imgEstados", "row_height": 56,
 "columns": [{"field": "estado", "kind": "icon", "width": 40}, {"field": "capa", "kind": "image", "width": 64}, {"field": "titulo"}]}
```

Vale nas 8 linguagens: o app só põe o número (ou o nome) e os bytes nos campos.
