# SpeedDial

![SpeedDial](img/SpeedDial.png)

Botão flutuante que abre ações (o `v-speed-dial`): o botão principal fica embaixo; aberto (`open`), as ações (`icons`, com `labels` ao lado) sobem acima dele. `on_item` com o índice da ação. O controle ocupa a altura toda das ações.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `icon` | Ícone | texto |
| `icons` | Ícones | texto |
| `labels` | Rótulos | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `open` | Aberto | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_item` | — | — |  |

## Em código

No form, o `SpeedDial1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "SpeedDial", "name": "SpeedDial1", "on_item": "speed_dial1_item" }
```

E o handler, um método do form (`self.SpeedDial1` já é um `SpeedDial`):

```rust
impl FrmPrincipal {
    pub fn speed_dial1_item(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.SpeedDial1.icon(ui);
        self.SpeedDial1.set_icon(ui, format!("{atual}!"));
    }
}
```
