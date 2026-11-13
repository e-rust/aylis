# Clipboard

Copiar e colar texto, HTML, imagem e os formatos do app pela área de transferência.

**Paleta:** System · **componente não visual**

## Propriedades

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro |  |

## Em código

No form, o `Clipboard1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Clipboard", "name": "Clipboard1" }
```

E o handler, um método do form (`self.Clipboard1` já é um `Clipboard`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Clipboard1.tag(ui);
        self.Clipboard1.set_tag(ui, atual + 1);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
