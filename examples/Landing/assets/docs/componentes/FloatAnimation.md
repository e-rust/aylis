# FloatAnimation

Anima uma propriedade numérica (posição, tamanho, opacidade).

**Paleta:** Animations · **componente não visual**

## Propriedades

### Animação

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `property` | Propriedade | texto |  |
| `start_value` | Valor inicial | número |  |
| `stop_value` | Valor final | número |  |
| `duration` | Duração | duração (ms) |  |
| `delay` | Atraso | duração (ms) |  |
| `easing` | Curva de aceleração | curva de suavização |  |
| `trigger` | Gatilho | um de: `none`, `hover`, `pressed`, `focused`, `checked`, `appear` |  |
| `auto_reverse` | Volta sozinho | lógico |  |
| `looping` | Repetindo | lógico |  |
| `enabled` | Habilitado | lógico |  |
| `keyframes` | Passos (keyframes) | texto |  |

## Em código

No form, o `FloatAnimation1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "FloatAnimation", "name": "FloatAnimation1" }
```

E o handler, um método do form (`self.FloatAnimation1` já é um `FloatAnimation`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.FloatAnimation1.property(ui);
        self.FloatAnimation1.set_property(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
