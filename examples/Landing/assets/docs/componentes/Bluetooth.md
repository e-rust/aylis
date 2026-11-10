# Bluetooth

Bluetooth: BLE e o clássico num componente só.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `service_filter` | Filtro de serviços | texto |
| `name_filter` | Filtro de nome | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_found` | — | — |  |
| `on_connected` | — | — |  |
| `on_disconnected` | — | — |  |
| `on_value` | — | — |  |
| `on_written` | — | — |  |
| `on_received` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `Bluetooth1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Bluetooth", "name": "Bluetooth1", "on_found": "bluetooth1_found" }
```

E o handler, um método do form (`self.Bluetooth1` já é um `Bluetooth`):

```rust
impl FrmPrincipal {
    pub fn bluetooth1_found(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Bluetooth1.service_filter(ui);
        self.Bluetooth1.set_service_filter(ui, format!("{atual}!"));
    }
}
```
