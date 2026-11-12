# TrayIcon

Ícone na bandeja do sistema, com menu e clique.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `icons` | Ícones | texto | Arquivos dos quadros (PNG, ICO), separados por `;`: um = parado; vários = animação. |
| `interval` | Intervalo | duração (ms) | Tempo de cada quadro da animação (ms). |
| `badge` | Selo | texto | Indicador: vazio, `•` ou um texto curto (`3`, `9+`). |
| `badge_color` | Cor do selo | cor |  |
| `menu_theme` | Tema do menu | texto | `light`, `dark` ou uma cor (`#1E293B`). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `hint` | Dica | texto |  |
| `visible` | Visível | lógico |  |
| `menu_items` | Itens do menu | texto | O menu do clique direito: itens separados por `;` (`-` é uma linha). |
| `popup_menu` | Menu de contexto | componente | O menu do clique direito por um `PopupMenu` (o `PopupMenu` do `TTrayIcon`): os `MenuItem` dele, com `enabled`, `checked`, `header`, divisória e submenu (recuado). Escolher um item dispara o `on_click` dele (e o `on_item_click` do menu) e o `on_menu` daqui, com o texto. Vence o `menu_items`. O relatório do Tabernáculo, C26. |

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
