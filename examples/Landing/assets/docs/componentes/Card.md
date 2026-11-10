# Card

![Card](img/Card.png)

O cartão (`QCard`): um painel com a cara de cartão do tema (fundo, cantos e sombra), que recebe outros componentes. As variantes `flat` (sem sombra) e `bordered` (com borda).

**Paleta:** Quasar · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `fill2` | Segunda cor do degradê | cor |
| `fill_mid` | Cor do meio do degradê | cor |
| `fill_mid_at` | Posição da cor do meio | número |
| `gradient_angle` | Ângulo do degradê | ângulo (graus) |
| `gradient_radial` | Degradê circular | lógico |
| `stroke` | Cor do traço | cor |
| `stroke_width` | Largura do traço | número |
| `radius` | Raio dos cantos | número |

### Teclado

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `allowed_chars` | Caracteres permitidos | texto |
| `blocked_chars` | Caracteres bloqueados | texto |
| `only_allowed` | Só os permitidos | lógico |
| `chars_override` | Força os caracteres | lógico |
| `char_map` | Trocas de caracteres | texto |

## Em código

No form, o `Card1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Card", "name": "Card1" }
```

E o handler, um método do form (`self.Card1` já é um `Card`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Card1.allowed_chars(ui);
        self.Card1.set_allowed_chars(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
