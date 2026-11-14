# FloatAnimation

Anima uma propriedade numérica (posição, tamanho, opacidade).

**Paleta:** Animations · **componente não visual**

## Propriedades

### Animação

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `property` | Propriedade | texto | A propriedade do pai que a animação muda (`opacity`, `fill`, `left`...). |
| `start_value` | Valor inicial | número | Valor inicial; `Null` = o valor atual da propriedade. |
| `stop_value` | Valor final | número | O valor a que a propriedade chega no fim da animação. |
| `duration` | Duração | duração (ms) | Quanto dura a ida, em milissegundos. |
| `delay` | Atraso | duração (ms) | Espera antes de começar, em milissegundos (só na ida). |
| `easing` | Curva de aceleração | curva de suavização | A curva de velocidade da animação (`linear`, `out_quad`...): como o valor anda no tempo. |
| `trigger` | Gatilho | um de: `none`, `hover`, `pressed`, `focused`, `checked`, `appear` | O estado do pai que dispara a animação (`hover`, `pressed`, `focused`, `checked`, `appear`); `none`: só por código. |
| `auto_reverse` | Volta sozinho | lógico | Chegando ao fim, volta ao valor inicial pelo mesmo caminho. |
| `looping` | Repetindo | lógico | Repete sem parar até ser desligada. |
| `enabled` | Habilitado | lógico | Desligada, o gatilho não a dispara. |
| `keyframes` | Passos (keyframes) | texto | Sequência de passos (`0:0; 0.6:1.15; 1:1`: fração do tempo e valor), no lugar de ir de `start_value` a `stop_value` (só números). |

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
