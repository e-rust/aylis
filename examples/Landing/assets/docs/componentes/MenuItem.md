# MenuItem

Item de um menu (texto, ícone, atalho, marca); dentro de outro vira submenu.

**Paleta:** Menus · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `text` | Texto | texto |
| `checked` | Marcado | lógico |
| `icon` | Ícone | texto |
| `danger` | Perigoso | lógico |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `shortcut` | Atalho | texto |
| `enabled` | Habilitado | lógico |
| `auto_check` | Marca sozinho | lógico |
| `visible` | Visível | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |

## Em código

No form, o `MenuItem1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MenuItem", "name": "MenuItem1", "on_click": "menu_item1_click" }
```

E o handler, um método do form (`self.MenuItem1` já é um `MenuItem`):

```rust
impl FrmPrincipal {
    pub fn menu_item1_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MenuItem1.text(ui);
        self.MenuItem1.set_text(ui, format!("{atual}!"));
    }
}
```
