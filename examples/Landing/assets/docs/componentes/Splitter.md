# Splitter

![Splitter](img/Splitter.png)

Divisória arrastável: redimensiona o controle encaixado antes dela com o mesmo alinhamento (`left`, `right`, `top` ou `bottom`).

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `min_size` | Tamanho mínimo | número |

## Em código

No form, o `Splitter1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Splitter", "name": "Splitter1" }
```

E o handler, um método do form (`self.Splitter1` já é um `Splitter`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Splitter1.min_size(ui);
        self.Splitter1.set_min_size(ui, atual + 10.0);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
