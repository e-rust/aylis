# Path

![Path](img/Path.png)

Um caminho SVG qualquer (`data`, o atributo `d`), esticado para o controle (`keep_aspect`: sem distorcer, centralizado). Figuras abertas só levam o traço.

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `stroke` | Cor do traço | cor | A cor do traço; vazio: sem traço nas figuras fechadas e a cor do texto do tema nas abertas. |
| `stroke_width` | Largura do traço | número | A espessura do traço, em pontos; 0: sem traço. |
| `data` | Dados | texto | O caminho no formato SVG (o atributo `d`: `M12 21 C... Z`), esticado para o controle; com `Z` a figura é fechada e leva preenchimento. |
| `keep_aspect` | Mantém a proporção | lógico | Estica sem distorcer, centralizado; desligado, ocupa a largura e a altura do controle. |

## Em código

No form, o `Path1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Path", "name": "Path1" }
```

E o handler, um método do form (`self.Path1` já é um `Path`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Path1.stroke_width(ui);
        self.Path1.set_stroke_width(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
