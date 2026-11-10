# AylisBarcodeProvider

O provedor de códigos de barras do Aylis como componente (para ligar explicitamente; sem provedor ligado, o `Barcode` já usa este).

**Paleta:** Providers · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `AylisBarcodeProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AylisBarcodeProvider", "name": "AylisBarcodeProvider1" }
```

E o handler, um método do form (`self.AylisBarcodeProvider1` já é um `AylisBarcodeProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.AylisBarcodeProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
