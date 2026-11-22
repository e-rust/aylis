# ImageList

A lista de imagens.

**Paleta:** Additional · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | lista de `ImageListItem` | Os itens: a lista de `IMAGE_LIST_ITEM` em JSON. |

## Em código

No form, o `ImageList1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ImageList", "name": "ImageList1" }
```

E o handler, um método do form (`self.ImageList1` já é um `ImageList`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.ImageList1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)

## Mais sobre o ImageList

### Os ícones de uma coluna

Cada item é um ícone de pacote (`icon`, pintado na cor do texto) ou um arquivo do projeto
(`image`), e se acha pela posição ou pelo `name`. O `DataGrid` com `images` neste componente
mostra, nas colunas com `kind: icon`, o item cujo índice (ou nome) vem do campo do registro.

```json
{"class": "ImageList", "name": "imgEstados", "items": [
  {"name": "parada"}, {"name": "tocando", "icon": "fluent:play-fill"}, {"name": "pausada", "icon": "fluent:pause-fill"}]}
```
