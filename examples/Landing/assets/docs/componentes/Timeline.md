# Timeline

![Timeline](img/Timeline.png)

Linha do tempo (o `v-timeline`): `items` é a lista de registros `TIMELINE_ITEM` (`{"title": "Pedido entregue", "when": "hoje, 14:20", "text": "Recebido por Ana"}`; o `when` e o `text` são opcionais). Pontos na cor da variante; o primeiro em destaque.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `item_height` | Altura do item | número | A altura de cada evento na linha, em pontos. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | lista de `TimelineItem` | Os eventos, em JSON: `[{"title": "Pedido entregue", "when": "hoje, 14:20", "text": "..."}]`. |

## Em código

No form, o `Timeline1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Timeline", "name": "Timeline1" }
```

E o handler, um método do form (`self.Timeline1` já é um `Timeline`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Timeline1.item_height(ui);
        self.Timeline1.set_item_height(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
