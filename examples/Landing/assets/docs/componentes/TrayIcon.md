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
| `badge_color` | Cor do selo | cor | A cor do fundo do indicador. |
| `menu_theme` | Tema do menu | texto | `light`, `dark` ou uma cor (`#1E293B`). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `hint` | Dica | texto | A dica que aparece com o mouse parado em cima do ícone. |
| `visible` | Visível | lógico | O ícone aparece na bandeja; desligado, sai dela. |
| `menu_items` | Itens do menu | lista de `TrayMenuItem` | O menu do clique direito, em JSON (o registro `TRAY_MENU_ITEM`): `[{"text": "Abrir"}, {"separator": true}, {"text": "Sair"}]`. |
| `popup_menu` | Menu de contexto | componente | O menu do clique direito por um `PopupMenu`: os `MenuItem` dele, com `enabled`, `checked`, `header`, divisória e submenu (recuado). Escolher um item dispara o `on_click` dele (e o `on_item_click` do menu) e o `on_menu` daqui, com o texto. Vence o `menu_items`. O relatório do Tabernáculo, C26. |
| `guid` | GUID | texto | O GUID fixo do ícone (`{0F8A1B2C-3D4E-5F60-7182-93A4B5C6D7E8}`); vazio = sem. O Windows lembra se o ícone fica na barra ou no "^" por ele, entre execuções e versões do app. Preso ao caminho do exe: se o exe mudar de pasta, o ícone não aparece até usar outro GUID (o exe assinado pelo mesmo editor não tem essa trava). Um por ícone. Só no Windows. |
| `tray_uid` | Número na bandeja | inteiro | O número fixo do ícone no Windows (o `uID`); 0 = o que o Aylis escolher. Sem GUID, o Windows lembra o lugar do ícone pelo caminho do exe mais este número: o mesmo número do programa que o app substitui, no mesmo caminho, herda o lugar dele. Um por ícone. Só no Windows. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |
| `on_dbl_click` | o duplo clique | — |  |
| `on_menu` | — | — |  |
| `on_context` | — | — |  |

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
