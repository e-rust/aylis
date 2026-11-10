# TrayIcon

Ícone na bandeja do sistema, com menu e clique.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `icons` | Ícones | texto |
| `interval` | Intervalo | duração (ms) |
| `badge` | Selo | texto |
| `badge_color` | Cor do selo | cor |
| `menu_theme` | Tema do menu | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `hint` | Dica | texto |
| `visible` | Visível | lógico |
| `menu_items` | Itens do menu | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |
| `on_dbl_click` | o duplo clique | — |  |
| `on_menu` | — | — |  |

## Em código

No form, o `TrayIcon1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TrayIcon", "name": "TrayIcon1", "on_click": "tray_icon1_click" }
```

E o handler, um método do form (`self.TrayIcon1` já é um `TrayIcon`):

```rust
impl FrmPrincipal {
    pub fn tray_icon1_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TrayIcon1.icons(ui);
        self.TrayIcon1.set_icons(ui, format!("{atual}!"));
    }
}
```
