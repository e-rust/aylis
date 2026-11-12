# ColorAnimation

Anima uma propriedade de cor.

**Paleta:** Animations · **componente não visual**

## Propriedades

### Animação

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `property` | Propriedade | texto |  |
| `start_value` | Valor inicial | cor |  |
| `stop_value` | Valor final | cor |  |
| `duration` | Duração | duração (ms) |  |
| `delay` | Atraso | duração (ms) |  |
| `easing` | Curva de aceleração | curva de suavização |  |
| `trigger` | Gatilho | um de: `none`, `hover`, `pressed`, `focused`, `checked`, `appear` |  |
| `auto_reverse` | Volta sozinho | lógico |  |
| `looping` | Repetindo | lógico |  |
| `enabled` | Habilitado | lógico |  |

## Em código

No form, o `ColorAnimation1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ColorAnimation", "name": "ColorAnimation1" }
```

E o handler, um método do form (`self.ColorAnimation1` já é um `ColorAnimation`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ColorAnimation1.property(ui);
        self.ColorAnimation1.set_property(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
