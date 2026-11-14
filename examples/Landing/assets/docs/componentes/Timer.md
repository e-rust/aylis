# Timer

Dispara on_timer a cada intervalo.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `interval` | Intervalo | duração (ms) | O intervalo entre os disparos do `on_timer`, em milissegundos. |
| `enabled` | Habilitado | lógico | Contando; desligado, parado e sem gastar nada. |
| `precision` | Precisão | um de: `normal`, `high`, `exact` | Quanto o disparo pode atrasar: `normal` (o relógio do sistema, ~15,6 ms no Windows), `high` (~1 ms) ou `exact` (abaixo de 0,1 ms, gastando CPU no fim da espera). |
| `background` | Fundo | lógico | Conta numa thread própria, sem depender do laço da tela estar livre; o `on_timer` continua chegando na tela. |
| `one_shot` | Disparo único | lógico | Dispara uma vez e se desliga (`enabled` volta a `false`); ligar de novo conta outra vez. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_timer` | a cada `interval` ms, com `enabled` | — |  |

## Em código

No form, o `Timer1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Timer", "name": "Timer1", "on_timer": "timer1_timer" }
```

E o handler, um método do form (`self.Timer1` já é um `Timer`):

```rust
impl FrmPrincipal {
    pub fn timer1_timer(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Timer1.enabled(ui);
        self.Timer1.set_enabled(ui, !atual);
    }
}
```
