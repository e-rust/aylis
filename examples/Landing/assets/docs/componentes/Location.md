# Location

A localização (GPS, rede): latitude, longitude, precisão (m), altitude e velocidade (m/s). Liga com `active`; a primeira vez pede a permissão (no Android, declare `location` no `Permissions` ou ela é declarada pela publicação ao achar este componente).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Mede a localização (a primeira vez pede a permissão); desligado, para. |
| `interval` | Intervalo | número | De quanto em quanto tempo mede, em segundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

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
