# KrillaPdfProvider

O provedor de PDF pelo krilla, como componente (para ligar ao `provider` do `Report`; sem provedor ligado, o `Report` já usa este, que o `register` põe como o padrão).

**Paleta:** Providers · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `KrillaPdfProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "KrillaPdfProvider", "name": "KrillaPdfProvider1" }
```

E o handler, um método do form (`self.KrillaPdfProvider1` já é um `KrillaPdfProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.KrillaPdfProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
