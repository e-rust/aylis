# Lottie

![Lottie](img/Lottie.png)

Animação Lottie (o JSON do After Effects/Bodymovin). Desenha vetor: nítida em qualquer tamanho, em qualquer renderizador.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `source` | Origem | texto |  |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` |  |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `playing` | Tocando | lógico |  |
| `repeat` | Repetir | lógico |  |
| `speed` | Velocidade | número |  |
| `position` | Posição | número |  |

## Em código

No form, o `Lottie1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Lottie", "name": "Lottie1" }
```

E o handler, um método do form (`self.Lottie1` já é um `Lottie`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Lottie1.source(ui);
        self.Lottie1.set_source(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
