# PopupMenu

Menu de contexto: abre no clique direito do controle que o usa em popup_menu.

**Paleta:** Menus · **componente não visual**

## Propriedades

Só as comuns.

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_popup` | — | — |  |
| `on_item_click` | — | — |  |
| `on_close` | o form fechou | — |  |

## Em código

No form, o `PopupMenu1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "PopupMenu", "name": "PopupMenu1", "on_popup": "popup_menu1_popup" }
```

E o handler, um método do form (`self.PopupMenu1` já é um `PopupMenu`):

```rust
impl FrmPrincipal {
    pub fn popup_menu1_popup(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.PopupMenu1;
    }
}
```
