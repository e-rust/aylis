# ChartSync

Liga gráficos pelo eixo X: o cursor e a janela (veja o módulo).

**Paleta:** Additional · **componente não visual**

## Propriedades

Só as comuns.

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_sync` | — | — |  |

## Em código

No form, o `ChartSync1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ChartSync", "name": "ChartSync1", "on_sync": "chart_sync1_sync" }
```

E o handler, um método do form (`self.ChartSync1` já é um `ChartSync`):

```rust
impl FrmPrincipal {
    pub fn chart_sync1_sync(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.ChartSync1;
    }
}
```
