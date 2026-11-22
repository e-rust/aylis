# SymphoniaTagsProvider

O provedor padrão como componente (o Symphonia): o mesmo que vale sem provedor, para quem quer deixar explícito no form.

**Paleta:** Media · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `SymphoniaTagsProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "SymphoniaTagsProvider", "name": "SymphoniaTagsProvider1" }
```

E o handler, um método do form (`self.SymphoniaTagsProvider1` já é um `SymphoniaTagsProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.SymphoniaTagsProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
