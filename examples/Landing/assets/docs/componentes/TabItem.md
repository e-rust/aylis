# TabItem

Uma página do `TabControl`: um contêiner com o título da aba.

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caption` | Título | texto | O título da aba. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_show` | o form abriu | — |  |

## Em código

No form, o `TabItem1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TabItem", "name": "TabItem1", "on_show": "tab_item1_show" }
```

E o handler, um método do form (`self.TabItem1` já é um `TabItem`):

```rust
impl FrmPrincipal {
    pub fn tab_item1_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TabItem1.caption(ui);
        self.TabItem1.set_caption(ui, format!("{atual}!"));
    }
}
```
