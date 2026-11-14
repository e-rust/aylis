# Lottie

![Lottie](img/Lottie.png)

Animação Lottie (o JSON do After Effects/Bodymovin). Desenha vetor: nítida em qualquer tamanho, em qualquer renderizador.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `source` | Origem | texto | A animação: `"noto:fogo"` (do pacote de animações da IDE) ou um caminho no projeto (`assets/lottie/carregando.json`). |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` | Como a animação ocupa o controle: caber inteira, cobrir, esticar... |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `playing` | Tocando | lógico | A animação anda; desligado, fica parada na `position`. |
| `repeat` | Repetir | lógico | Recomeça ao chegar ao fim; desligado, toca uma vez e para no último quadro. |
| `speed` | Velocidade | número | Velocidade (1 = normal, 2 = o dobro). |
| `position` | Posição | número | O ponto da animação, em segundos: onde ela fica parada e de onde começa a tocar. |

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
