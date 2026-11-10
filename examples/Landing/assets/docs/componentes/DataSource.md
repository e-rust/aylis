# DataSource

Liga um ClientDataSet aos controles de dados (DataGrid, "Ligar a campo…"...).

**Paleta:** Data · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `data_set` | Conjunto de dados | componente |
| `auto_edit` | Edita sozinho | lógico |
| `enabled` | Habilitado | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_data_change` | — | — |  |
| `on_state_change` | — | — |  |

**Veja também:** [Gravar as alterações no banco](../guias/gravar-alteracoes.md)

## Em código

No form, o `DataSource1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DataSource", "name": "DataSource1", "on_data_change": "data_source1_data_change" }
```

E o handler, um método do form (`self.DataSource1` já é um `DataSource`):

```rust
impl FrmPrincipal {
    pub fn data_source1_data_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DataSource1.auto_edit(ui);
        self.DataSource1.set_auto_edit(ui, !atual);
    }
}
```
