# ClientDataSet

Tabela de dados em memória (como o TClientDataSet): campos, registros, filtros.

**Paleta:** Data · **componente não visual**

## Propriedades

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `key_fields` | Campos chave | texto | Campos-chave separados por `;` (identificam o registro no delta). |
| `apply_mode` | Modo de aplicar | um de: `cached`, `on_post` |  |
| `read_only` | Somente leitura | lógico |  |
| `active` | Ativo | lógico |  |
| `filter` | Filtro | texto | Expressão de filtro (`cidade = 'São Paulo' and limite > 1000`, ver `crate::expr`); vale com `filtered` ligado. |
| `filtered` | Filtrado | lógico |  |
| `index_field_names` | Campos do índice | texto | Ordem dos registros: campos separados por `;`, com `desc` opcional (`cidade;limite desc`), como o `IndexFieldNames`. |
| `indexes` | Índices | texto | Índices com nome (como o `IndexDefs`): `porNome=nome\|porValor=valor desc;nome`. |
| `index_name` | Nome do índice | texto | O índice de `indexes` em uso (como o `IndexName`); vale no lugar de `index_field_names`. Ordenar por campos (`set_index`) volta a ser sem nome. |
| `params` | Parâmetros | texto | Valores dos parâmetros do filtro (`:minimo`): `minimo=100;cidade=São Paulo`. Por código, `DataSet::set_param`(crate::DataSet::set_param) aceita qualquer valor. |
| `status_filter` | Filtro de situação | texto | Só os registros nestas situações: `unmodified;modified;inserted` (vazio = todos), como o `StatusFilter`. |
| `master_source` | Fonte mestre | componente | Master-detail: o `DataSource` do dataset mestre. |
| `master_fields` | Campos mestre | texto | Master-detail: `campo_daqui=campo_do_mestre` separados por `;` (só o nome se forem iguais): `cliente_id=id`. |
| `apply_separately` | Gravar à parte do mestre | lógico | Master-detail: as alterações deste detalhe vão à parte, no `apply_updates` dele. Desligado (o padrão), vão junto no delta do mestre, na ordem de gravar. |
| `record_total` | Total de registros | inteiro | Quantos registros a consulta tem no backend (0: não se sabe). Com os dados em partes, a barra de rolagem da grade usa este total. |
| `max_records` | Máximo de registros | inteiro | Limite de registros na memória (0: sem limite): a busca sob demanda para nele, e a grade avisa que mostra só os primeiros. |

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
