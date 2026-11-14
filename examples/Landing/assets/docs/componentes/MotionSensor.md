# MotionSensor

O acelerômetro (com a gravidade), em m/s²: `x`, `y` e `z`.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Lê o acelerômetro; desligado, para. |
| `interval` | Intervalo | número | De quanto em quanto tempo lê, em segundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `MotionSensor1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MotionSensor", "name": "MotionSensor1", "on_change": "motion_sensor1_change" }
```

E o handler, um método do form (`self.MotionSensor1` já é um `MotionSensor`):

```rust
impl FrmPrincipal {
    pub fn motion_sensor1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MotionSensor1.active(ui);
        self.MotionSensor1.set_active(ui, !atual);
    }
}
```
