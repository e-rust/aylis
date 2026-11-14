# Icon

![Icon](img/Icon.png)

Um ícone sozinho. `icon` é o nome (os do aylis ou os registrados com `Ui::register_icon`); a cor vem do tema (`text`) ou de `color`.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `icon` | Ícone | texto | O nome do ícone (os do Aylis ou os registrados com `Ui::register_icon`). |
| `color` | Cor | cor | A cor do ícone; vazio: a do texto do tema (ou a da variante). |

## Em código

No form, o `Icon1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Icon", "name": "Icon1" }
```

E o handler, um método do form (`self.Icon1` já é um `Icon`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Icon1.icon(ui);
        self.Icon1.set_icon(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
