# Layout

![Layout](img/Layout.png)

Contêiner sem desenho próprio: agrupa os controles e os arruma pelo alinhamento (`align`), margens e espaçamento.

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Teclado

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `allowed_chars` | Caracteres permitidos | texto |
| `blocked_chars` | Caracteres bloqueados | texto |
| `only_allowed` | Só os permitidos | lógico |
| `chars_override` | Força os caracteres | lógico |
| `char_map` | Trocas de caracteres | texto |

## Em código

No form, o `Layout1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Layout", "name": "Layout1" }
```

E o handler, um método do form (`self.Layout1` já é um `Layout`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Layout1.allowed_chars(ui);
        self.Layout1.set_allowed_chars(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
