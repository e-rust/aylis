# Location

A localização (GPS, rede): latitude, longitude, precisão (m), altitude e velocidade (m/s). Liga com `active`; a primeira vez pede a permissão (no Android, declare `location` no `Permissions` ou ela é declarada pela publicação ao achar este componente).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `active` | Ativo | lógico |
| `interval` | Intervalo | número |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `Location1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Location", "name": "Location1", "on_change": "location1_change" }
```

E o handler, um método do form (`self.Location1` já é um `Location`):

```rust
impl FrmPrincipal {
    pub fn location1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Location1.active(ui);
        self.Location1.set_active(ui, !atual);
    }
}
```
