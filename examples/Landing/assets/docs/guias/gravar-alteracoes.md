# Gravar as alterações no banco

O Aylis não fala com banco de dados: a tela guarda as alterações do usuário no
`ClientDataSet` e, no `apply_updates`, entrega ao seu código o
**delta**, a lista do que mudou. Você converte em SQL do jeito que quiser (qualquer banco,
ORM ou API) e devolve o resultado de cada registro.

## O delta

Cada alteração diz o que aconteceu com um registro:

| Campo | O que é |
|---|---|
| `ref` | identifica o registro na resposta (único no processo) |
| `op` | `insert`, `update` ou `delete` |
| `data_set` | o dataset do registro (o mestre ou um detalhe) |
| `key` | os campos-chave com os valores **originais** (`key_fields`; sem ele, todos) |
| `old` | os valores antigos: só os campos alterados (`update`) ou todos (`delete`) |
| `new` | os valores novos: só os campos alterados (`update`) ou todos (`insert`) |
| `master` | no item incluído sob um mestre também incluído: o `ref` do mestre e os campos de ligação |

```json
{ "data_set": "cdsPedidos", "changes": [
  { "ref": 1, "op": "insert", "data_set": "cdsPedidos", "key": { "id": -1 }, "new": { "id": -1, "cliente": "João" } },
  { "ref": 2, "op": "insert", "data_set": "cdsItens", "key": { "id": -2 }, "new": { "id": -2, "pedido_id": -1 },
    "master": { "data_set": "cdsPedidos", "ref": 1, "fields": { "pedido_id": "id" } } },
  { "ref": 3, "op": "update", "data_set": "cdsItens", "key": { "id": 10 }, "old": { "produto": "A" }, "new": { "produto": "B" } },
  { "ref": 4, "op": "delete", "data_set": "cdsPedidos", "key": { "id": 7 }, "old": { "id": 7 } } ] }
```

A resposta é uma lista com um resultado por registro: `{ "ref": 1 }` (gravou),
`{ "ref": 1, "values": { "id": 1043 } }` (gravou, e o banco definiu valores: a chave
gerada, um carimbo de data) ou `{ "ref": 1, "error": "Cliente possui pedidos" }` (não
gravou: a mensagem vai para a tela e o registro continua pendente). Um registro sem resultado
também continua pendente.

## Mestre-detalhe

Um dataset com `master_source` (o detalhe) entra no delta do mestre: o `apply_updates` do
pedido leva os itens junto, **na ordem de gravar**:

1. as inclusões e alterações do mestre;
2. tudo dos detalhes (e dos detalhes deles);
3. as exclusões do mestre (os itens saem antes do pedido).

A chave de um registro novo é provisória e negativa (`-1`, `-2`...) até o banco gravar. O
item incluído sob um pedido novo leva o `master`, dizendo de qual pedido depende. Quando o
resultado do pedido volta com a chave do banco (`values: { "id": 1043 }`), o Aylis troca o
`-1` pelo `1043` nos itens que estão na tela. O ajudante `Delta` faz o mesmo no seu código,
antes de os itens serem gravados (o próximo tópico).

Para gravar um detalhe à parte, no `apply_updates` dele, ligue `apply_separately` no detalhe.

## O ajudante `Delta`

Todas as linguagens têm o mesmo ajudante: separa por tipo e, ao aceitar a inclusão de um
mestre com a chave gerada, troca a chave nos itens que vêm depois.

| | Rust | Pascal | Outras |
|---|---|---|---|
| criar | `Delta::new(changes.to_vec())` | `TAylisDelta.Create(Event)` | `Delta(event)`, `new Delta(ev)`, `NewDelta(ev.Args)` |
| por tipo | `inserted()`, `updated()`, `deleted()` | `Inserted`, `Updated`, `Deleted` | `inserted`/`Inserted`... |
| de um dataset | `of_data_set("cdsItens")` | `OfDataSet('cdsItens')` | `of_data_set`, `ofDataSet`, `OfDataSet` |
| gravou | `accept(ref, valores)` | `Accept(C, TJSONObject.Create(['id', Novo]))` | `accept(change, id=novo)`... |
| não gravou | `reject(ref, "mensagem")` | `Reject(C, 'mensagem')` | `reject(change, "mensagem")` |
| a resposta | `into_results()` | `Aylis.Reply('results', Delta.ResultsJSON)` | `ev.reply["results"] = delta.results` |

### Rust

```rust
data_set(ui, cds_pedidos).on_apply(|changes| {
    let mut delta = Delta::new(changes.to_vec());
    for i in 0..delta.len() {
        let c = delta.change(i).clone(); // os itens já com a chave do pedido gravado
        match c.kind {
            ChangeKind::Insert => {
                let id = inserir(&c.data_set, &c.new); // insert ... returning id
                delta.accept(c.reference, vec![("id".into(), DataValue::Int(id))]);
            }
            ChangeKind::Update => { alterar(&c.data_set, &c.key, &c.new); delta.accept(c.reference, vec![]) }
            ChangeKind::Delete => { excluir(&c.data_set, &c.key); delta.accept(c.reference, vec![]) }
        }
    }
    delta.into_results()
});
```

O exemplo `examples/ERP` grava assim num SQLite, numa transação por `apply_updates`
(`rust/src/erp/banco.rs`).

### Python

```python
@aylis.on("gravar_pedidos")
def gravar(event):
    delta = Delta(event)
    for c in delta:
        if c["op"] == "insert":
            novo = db.execute(f"insert into {tabela(c)} ... returning id", ...).fetchone()[0]
            delta.accept(c, id=novo)
        elif c["op"] == "update":
            ...
            delta.accept(c)
    event["reply"]["results"] = delta.results
```

### Free Pascal

```pascal
procedure TFrmPedidos.GravarPedidos(Sender: TAylisComponent; Event: TJSONObject);
var
  Delta: TAylisDelta;
  C: TJSONObject;
begin
  Delta := TAylisDelta.Create(Event);
  try
    for C in Delta.Inserted('cdsPedidos') do
      Delta.Accept(C, TJSONObject.Create(['id', InserirPedido(C)]));
    for C in Delta.Inserted('cdsItens') do   { o pedido_id já é o do banco }
      Delta.Accept(C, TJSONObject.Create(['id', InserirItem(C)]));
    Aylis.Reply('results', Delta.ResultsJSON);
  finally
    Delta.Free;
  end;
end;
```

Atenção à ordem: aqui o código grava todas as inclusões de pedido antes das de item, o que
também é seguro. Gravar as exclusões de item antes das de pedido fica com você se não usar a
ordem do delta (`for I := 0 to Delta.Count - 1`).

### TypeScript, C#, Java, Go e C++

```ts
const delta = new Delta(ev);
for (const c of delta.changes) { /* ... */ delta.accept(c, { id: novo }); }
ev.reply.results = delta.results;
```

```csharp
var delta = new Delta(ev.Args);
foreach (var c in delta.Changes) { /* ... */ delta.Accept(c, new JsonObject { ["id"] = novo }); }
ev.Reply["results"] = delta.ResultsJson();
```

```java
Aylis.Delta delta = new Aylis.Delta(ev.args());
for (Map<String, Object> c : delta.changes()) { /* ... */ delta.accept(c, Aylis.obj("id", novo)); }
ev.reply().put("results", delta.results());
```

```go
delta := aylis.NewDelta(ev.Args)
for _, c := range delta.Changes { /* ... */ delta.Accept(c, aylis.M{"id": novo}) }
ev.Reply["results"] = delta.Results
```

```cpp
aylis::Delta delta(ev.args);
for (size_t i = 0; i < delta.size(); i++) { /* ... */ delta.accept(delta.change(i), {{"id", novo}}); }
ev.reply.set("results", delta.results_json());
```

## Erros

- Um `error` num resultado: o registro fica pendente com a mensagem, o `on_reconcile_error` do
  dataset dele dispara, e o `apply_errors()` do mestre conta os recusados (detalhes incluídos).
- Uma exceção no handler do `on_apply_updates` recusa a gravação inteira.
- Para tudo ou nada, grave numa transação e, no primeiro erro, desfaça e devolva só aquele
  erro: os outros registros continuam pendentes (é o que o exemplo do ERP faz).

## Consulta grande: ordenar, filtrar e localizar no servidor (`on_server_query`)

Uma consulta grande chega em partes: o backend manda a primeira página (`load_rows`), diz que
há mais (`set_has_more(true)`) e o `on_fetch` traz o resto conforme a grade rola. Com só parte
carregada, ordenar pelo título da coluna, filtrar ou localizar na memória valeria só para o que
já chegou. Então o `ClientDataSet` pergunta ao backend pelo evento `on_server_query`, com um
texto JSON:

| Campo | O que é |
|---|---|
| `kind` | `"sort"` (ordenar), `"filter"` (filtrar) ou `"locate"` (localizar) |
| `index` | a ordem pedida (`cliente;total desc`; vazio: a de chegada) |
| `filter` | o filtro em vigor (vazio: sem filtro) |
| `loaded` | quantos registros estão carregados |
| `field`, `value` | só no `locate`: o campo e o valor procurados |

O handler refaz a consulta no servidor, na ordem e no filtro pedidos, e manda a primeira página
de novo. O que ele faz no dataset durante o evento não dispara outra pergunta. Sem handler, os
dados não mudam e o dataset guarda o aviso em `partial_notice` ("Ordenado só entre os 50
registros carregados: há mais no servidor"), que a grade mostra.

No form:

```json
{ "class": "ClientDataSet", "name": "cdsPedidos", "on_server_query": "cds_pedidos_server_query" }
```

E o handler (`consultar` é o acesso do app ao servidor, que devolve as linhas e se há mais):

```rust
impl FrmPedidos {
    pub fn cds_pedidos_server_query(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let pedido: serde_json::Value = serde_json::from_str(args.text().unwrap_or("{}")).unwrap_or_default();
        let ordem = pedido["index"].as_str().unwrap_or_default();
        let filtro = pedido["filter"].as_str().unwrap_or_default();
        let (linhas, tem_mais) = consultar(ordem, filtro, 200);
        let mut ds = data_set(ui, self.cdsPedidos.id);
        if let Err(e) = ds.load_rows(linhas).and_then(|_| ds.set_has_more(tem_mais)) {
            let _ = message_dlg(ui, &e.message, DlgKind::Error, &[ModalResult::Ok], |_, _| {});
        }
    }
}
```

Num `locate`, o backend também pode achar o registro (`field` = `value`) e mandar a página
em que ele está. Em outra linguagem é o mesmo evento: o texto JSON chega nos argumentos, e o
backend responde mandando as linhas ao dataset (o `cds.load_rows` do cliente).
