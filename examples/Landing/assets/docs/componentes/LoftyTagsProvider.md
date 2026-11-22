# LoftyTagsProvider

O provedor de tags pelo lofty. Não visual, sem propriedades: ligue no `provider` do `MediaInfo`.

**Paleta:** Providers · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `LoftyTagsProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "LoftyTagsProvider", "name": "LoftyTagsProvider1" }
```

E o handler, um método do form (`self.LoftyTagsProvider1` já é um `LoftyTagsProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.LoftyTagsProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
