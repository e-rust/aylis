# Pagination

![Pagination](img/Pagination.png)

Paginação (o `v-pagination`): `page` de 1 a `length`, com no máximo `total_visible` botões (o resto vira "…"). `on_change` com a página nova.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `total_visible` | Páginas visíveis | inteiro |  |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `page` | Página | inteiro |  |
| `length` | Comprimento | inteiro |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `Pagination1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Pagination", "name": "Pagination1", "on_change": "pagination1_change" }
```

E o handler, um método do form (`self.Pagination1` já é um `Pagination`):

```rust
impl FrmPrincipal {
    pub fn pagination1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Pagination1.page(ui);
        self.Pagination1.set_page(ui, atual + 1);
    }
}
```
