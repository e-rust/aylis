# Compass

A bússola: `heading` em graus (0 = norte, 90 = leste).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Lê a bússola; desligado, para. |
| `interval` | Intervalo | número | De quanto em quanto tempo lê, em segundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `Compass1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Compass", "name": "Compass1", "on_change": "compass1_change" }
```

E o handler, um método do form (`self.Compass1` já é um `Compass`):

```rust
impl FrmPrincipal {
    pub fn compass1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Compass1.active(ui);
        self.Compass1.set_active(ui, !atual);
    }
}
```
