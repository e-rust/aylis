# Export

Exporta os registros de um `DataSource` (na ordem e com o filtro que a tela mostra).

**Paleta:** Data · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data_source` | Fonte de dados | componente | O `DataSource` dos registros exportados (na ordem e com o filtro da tela). |
| `provider` | Provedor | componente | Quem grava o arquivo (um provedor de exportação); sem ele, CSV. |
| `fields` | Campos | texto | Os campos, separados por `;` (vazio: os visíveis, na ordem do dataset). |
| `sheet_name` | Nome da aba | texto | O nome da aba (onde o formato tem abas). |

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
