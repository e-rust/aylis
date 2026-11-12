# Path

![Path](img/Path.png)

Um caminho SVG qualquer (`data`, o atributo `d`), esticado para o controle (`keep_aspect`: sem distorcer, centralizado). Figuras abertas só levam o traço.

**Paleta:** Shapes · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `stroke` | Cor do traço | cor |  |
| `stroke_width` | Largura do traço | número |  |
| `data` | Dados | texto |  |
| `keep_aspect` | Mantém a proporção | lógico |  |

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
