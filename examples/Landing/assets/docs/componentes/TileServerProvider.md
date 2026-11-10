# TileServerProvider

Um servidor de blocos de mapa (o único caminho para o `MapView` trocar de servidor): `url` com `{z}`, `{x}` e `{y}` (e `{s}`, trocado por `a`, `b` ou `c` para dividir os pedidos), e o crédito que a licença dele pede. O padrão é o OpenStreetMap (o mesmo do `MapView` sem provedor); um servidor pago ou próprio é só outro endereço.

**Paleta:** Providers · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `url` | Endereço (URL) | texto |
| `attribution` | Atribuição | texto |

## Em código

No form, o `TileServerProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TileServerProvider", "name": "TileServerProvider1" }
```

E o handler, um método do form (`self.TileServerProvider1` já é um `TileServerProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TileServerProvider1.url(ui);
        self.TileServerProvider1.set_url(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
