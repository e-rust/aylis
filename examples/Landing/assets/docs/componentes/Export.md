# Export

Exporta os registros de um `DataSource` (na ordem e com o filtro que a tela mostra).

**Paleta:** Data · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `data_source` | Fonte de dados | componente |
| `provider` | Provedor | componente |
| `fields` | Campos | texto |
| `sheet_name` | Nome da aba | texto |

## Em código

No form, o `Export1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Export", "name": "Export1" }
```

E o handler, um método do form (`self.Export1` já é um `Export`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Export1.fields(ui);
        self.Export1.set_fields(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
