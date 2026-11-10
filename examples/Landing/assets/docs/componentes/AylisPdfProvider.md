# AylisPdfProvider

O PDF do próprio Aylis como componente (para ligar explicitamente ao `Report`; sem provedor, ele já usa este).

**Paleta:** Providers · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `AylisPdfProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AylisPdfProvider", "name": "AylisPdfProvider1" }
```

E o handler, um método do form (`self.AylisPdfProvider1` já é um `AylisPdfProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.AylisPdfProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
