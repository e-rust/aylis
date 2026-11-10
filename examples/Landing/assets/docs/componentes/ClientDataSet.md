# ClientDataSet

Tabela de dados em memória (como o TClientDataSet): campos, registros, filtros.

**Paleta:** Data · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `key_fields` | Campos chave | texto |
| `apply_mode` | Modo de aplicar | um de: `cached`, `on_post` |
| `read_only` | Somente leitura | lógico |
| `active` | Ativo | lógico |
| `filter` | Filtro | texto |
| `filtered` | Filtrado | lógico |
| `index_field_names` | Campos do índice | texto |
| `indexes` | Índices | texto |
| `index_name` | Nome do índice | texto |
| `params` | Parâmetros | texto |
| `status_filter` | Filtro de situação | texto |
| `master_source` | Fonte mestre | componente |
| `master_fields` | Campos mestre | texto |
| `apply_separately` | Gravar à parte do mestre | lógico |
| `record_total` | Total de registros | inteiro |
| `max_records` | Máximo de registros | inteiro |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_master_change` | — | — |  |
| `after_scroll` | o registro atual mudou | — |  |
| `before_post` | o registro vai ser gravado: `Err` recusa, com a mensagem | — | sim (recusar com a exceção) |
| `after_post` | o registro foi gravado | — |  |
| `before_delete` | o registro vai ser apagado: `Err` recusa | — | sim (recusar com a exceção) |
| `on_state_change` | — | — |  |
| `on_apply_updates` | — | — | sim: `results` |
| `on_reconcile_error` | — | — |  |
| `on_error` | — | — |  |
| `on_fetch` | — | — |  |
| `on_server_query` | o dataset pede ao backend os registros (o filtro, a ordem e a página que a tela quer), em vez de filtrar na tela: o handler responde com as linhas (veja o guia [Gravar as alterações](../guias/gravar-alteracoes.md)) | — |  |

**Veja também:** [Gravar as alterações no banco](../guias/gravar-alteracoes.md)

## Em código

No form, o `ClientDataSet1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ClientDataSet", "name": "ClientDataSet1", "on_master_change": "client_data_set1_master_change" }
```

E o handler, um método do form (`self.ClientDataSet1` já é um `ClientDataSet`):

```rust
impl FrmPrincipal {
    pub fn client_data_set1_master_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ClientDataSet1.key_fields(ui);
        self.ClientDataSet1.set_key_fields(ui, format!("{atual}!"));
    }
}
```
