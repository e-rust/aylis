package aylis

import (
	"encoding/json"
	"fmt"
)

// Delta são as alterações de um apply_updates, para o backend gravar do jeito dele, na ordem
// de gravar (o mestre incluído antes dos itens; as exclusões do mestre por último). Cada
// alteração é um M: "ref", "op" ("insert", "update", "delete"), "data_set", "key", "old",
// "new" e, no item incluído sob um mestre também incluído, "master". Ao aceitar o mestre com a
// chave gerada, os itens dele já recebem a chave antes de serem gravados:
//
//	delta := aylis.NewDelta(ev.Args)
//	for _, c := range delta.Changes {
//		if c["op"] == "insert" && c["data_set"] == "cdsPedidos" {
//			delta.Accept(c, aylis.M{"id": inserirPedido(c)})
//		}
//	}
//	ev.Reply["results"] = delta.Results
type Delta struct {
	DataSet string
	Changes []M
	Results []any
}

// NewDelta lê o delta dos argumentos do evento (o objeto ou o texto JSON) ou da lista.
func NewDelta(source any) *Delta {
	if text, ok := source.(string); ok {
		var v any
		if err := json.Unmarshal([]byte(text), &v); err == nil {
			source = v
		}
	}
	d := &Delta{}
	var list []any
	switch v := source.(type) {
	case map[string]any:
		d.DataSet, _ = v["data_set"].(string)
		list, _ = v["changes"].([]any)
	case []any:
		list = v
	}
	for _, item := range list {
		c, ok := item.(map[string]any)
		if !ok {
			continue
		}
		copied := M{}
		for k, v := range c {
			copied[k] = v
		}
		for _, part := range []string{"key", "new"} {
			m := M{}
			if src, ok := c[part].(map[string]any); ok {
				for k, v := range src {
					m[k] = v
				}
			}
			copied[part] = m
		}
		d.Changes = append(d.Changes, copied)
	}
	return d
}

func (d *Delta) of(op, dataSet string) []M {
	var out []M
	for _, c := range d.Changes {
		if c["op"] == op && (dataSet == "" || c["data_set"] == dataSet) {
			out = append(out, c)
		}
	}
	return out
}

// Inserted são as inclusões (de um dataset, ou de todos com "").
func (d *Delta) Inserted(dataSet string) []M { return d.of("insert", dataSet) }

// Updated são as alterações (de um dataset, ou de todos com "").
func (d *Delta) Updated(dataSet string) []M { return d.of("update", dataSet) }

// Deleted são as exclusões (de um dataset, ou de todos com "").
func (d *Delta) Deleted(dataSet string) []M { return d.of("delete", dataSet) }

// OfDataSet são as alterações de um dataset (o mestre ou um detalhe).
func (d *Delta) OfDataSet(dataSet string) []M {
	var out []M
	for _, c := range d.Changes {
		if c["data_set"] == dataSet {
			out = append(out, c)
		}
	}
	return out
}

func refOf(v any) int64 {
	switch n := v.(type) {
	case float64:
		return int64(n)
	case int64:
		return n
	case int:
		return int64(n)
	case json.Number:
		i, _ := n.Int64()
		return i
	}
	return -1
}

// Accept registra que gravou; values são os que o banco definiu (a chave gerada). Os itens
// que dependem desta inclusão recebem a chave nos campos de ligação.
func (d *Delta) Accept(change M, values M) {
	ref := refOf(change["ref"])
	for _, other := range d.Changes {
		link, ok := other["master"].(map[string]any)
		if !ok || refOf(link["ref"]) != ref {
			continue
		}
		fields, _ := link["fields"].(map[string]any)
		for detail, master := range fields {
			value, ok := values[fmt.Sprint(master)]
			if !ok {
				continue
			}
			for _, part := range []string{"new", "key"} {
				if m, ok := other[part].(M); ok {
					if _, has := m[detail]; has {
						m[detail] = value
					}
				}
			}
		}
	}
	if len(values) == 0 {
		d.Results = append(d.Results, M{"ref": ref})
	} else {
		d.Results = append(d.Results, M{"ref": ref, "values": values})
	}
}

// Reject registra que não gravou: a mensagem vai para a tela e o registro continua pendente.
func (d *Delta) Reject(change M, message string) {
	d.Results = append(d.Results, M{"ref": refOf(change["ref"]), "error": message})
}

// SampleDelta é o delta de exemplo dos selftests dos clientes (o mesmo em todas as linguagens).
const SampleDelta = `{"data_set":"cdsPedidos","changes":[` +
	`{"ref":1,"op":"insert","data_set":"cdsPedidos","key":{"id":-1},"new":{"id":-1,"cliente":"Joao"}},` +
	`{"ref":2,"op":"insert","data_set":"cdsItens","key":{"id":-2},"new":{"id":-2,"pedido_id":-1},` +
	`"master":{"data_set":"cdsPedidos","ref":1,"fields":{"pedido_id":"id"}}},` +
	`{"ref":3,"op":"update","data_set":"cdsItens","key":{"id":10},"old":{"produto":"A"},"new":{"produto":"B"}},` +
	`{"ref":4,"op":"delete","data_set":"cdsPedidos","key":{"id":7},"old":{"id":7}}]}`

// DeltaSelftest é o texto que todos os clientes escrevem ao conferir o Delta com o SampleDelta.
func DeltaSelftest() string {
	d := NewDelta(SampleDelta)
	d.Accept(d.Inserted("cdsPedidos")[0], M{"id": 501})
	item := d.Inserted("cdsItens")[0]["new"].(M)
	return fmt.Sprintf("delta %d %d %d %v %d", len(d.Inserted("")), len(d.Updated("")), len(d.Deleted("")), item["pedido_id"], len(d.Results))
}
