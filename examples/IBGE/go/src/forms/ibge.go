// Eventos do form frmIBGE: o código é seu (o registro dos handlers fica no forms.go
// gerado). Dentro de um handler, o form é a janela do evento.
//
// Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa
// e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem
// o mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou
// município escolhido: as bandeiras, os mapas e os dados dos gráficos.

package forms

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"

	"aylis.dev/cliente/aylis"
)

// A pasta dos dados no projeto (para os componentes).
const dados = "assets/dados/"

// municipio é o que o backend usa de cada município (os 10 maiores de cada estado).
type municipio struct {
	ID        int64  `json:"id"`
	UFID      int64  `json:"uf_id"`
	Nome      string `json:"nome"`
	Populacao *int64 `json:"populacao"`
}

var (
	// A mesma pasta no disco (o backend roda na pasta da linguagem).
	pasta = acharPasta()
	// Todos os municípios, como vieram do arquivo (vão inteiros para o dataset).
	municipiosJSON json.RawMessage
	municipios     []municipio
	// Código do estado ou do município → [[ano, habitantes], ...].
	populacao map[string][][2]int64
)

func acharPasta() string {
	for _, p := range []string{"../assets/dados", "assets/dados"} {
		if st, err := os.Stat(p); err == nil && st.IsDir() {
			return p
		}
	}
	return "../assets/dados"
}

// FrmIBGE reúne os handlers do form frmIBGE.
type FrmIBGE struct{}

// ler devolve o texto de um arquivo da pasta dos dados (nil se faltar).
func ler(arquivo string) json.RawMessage {
	b, err := os.ReadFile(filepath.Join(pasta, arquivo))
	if err != nil {
		return nil
	}
	return b
}

// carregar põe os registros (uma lista de objetos) num ClientDataSet do form.
func carregar(app *aylis.Aylis, dataset aylis.Component, linhas any) {
	if _, err := app.Call("data", aylis.M{"target": dataset.ID, "op": "load", "rows": linhas}); err != nil {
		aylis.Log(dataset.Name+":", err)
	}
}

// registro devolve o registro atual de um ClientDataSet (campo → valor).
func registro(app *aylis.Aylis, dataset aylis.Component) map[string]any {
	r, _ := app.Call("data", aylis.M{"target": dataset.ID, "op": "record"})
	m, _ := r.(map[string]any)
	return m
}

// texto: o valor de um campo como texto (os números sem casas: 2026, não 2026.0).
func texto(v any) string {
	switch x := v.(type) {
	case nil:
		return ""
	case float64:
		return strconv.FormatFloat(x, 'f', -1, 64)
	default:
		return fmt.Sprint(x)
	}
}

// imagem: o caminho de uma imagem da pasta dos dados (vazio: não tem).
func imagem(caminho string) string {
	if caminho == "" {
		return ""
	}
	return dados + caminho
}

func (f FrmIBGE) form_show(app *aylis.Aylis, ev aylis.Event) {
	estados := ler("estados.json")
	if estados == nil {
		frmIBGE.lblFonte.SetText("Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados")
		return
	}
	municipiosJSON = ler("municipios.json")
	json.Unmarshal(municipiosJSON, &municipios)
	json.Unmarshal(ler("populacao.json"), &populacao)
	frmIBGE.svgBrasil.SetSource(dados + "mapas/BR.svg")
	// Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
	carregar(app, frmIBGE.cdsDistritos.Component, ler("distritos.json"))
	carregar(app, frmIBGE.cdsMunicipios.Component, municipiosJSON)
	carregar(app, frmIBGE.cdsEstados.Component, estados)
	f.estado_mudou(app, ev)
}

// estado_mudou: outro estado na grade (a bandeira, os mapas e os 10 maiores municípios).
func (f FrmIBGE) estado_mudou(app *aylis.Aylis, ev aylis.Event) {
	e := registro(app, frmIBGE.cdsEstados.Component)
	if e == nil || e["id"] == nil {
		return
	}
	uf := int64(e["id"].(float64))
	frmIBGE.lblEstado.SetText(texto(e["nome"]))
	frmIBGE.lblRegiao.SetText(texto(e["sigla"]) + " · Região " + texto(e["regiao"]))
	frmIBGE.imgBandeiraEstado.SetImage(imagem(texto(e["bandeira"])))
	frmIBGE.lblEstadoEstimativa.SetText("Estimativa de " + texto(e["estimativa_ano"]))
	frmIBGE.svgBrasil.SetHighlight(fmt.Sprintf("e%d", uf))
	mapa := fmt.Sprintf("%smapas/%d.svg", dados, uf)
	frmIBGE.svgEstado.SetSource(mapa)
	frmIBGE.svgMunicipio.SetSource(mapa)

	var doEstado []municipio
	for _, m := range municipios {
		if m.UFID == uf {
			doEstado = append(doEstado, m)
		}
	}
	pop := func(m municipio) int64 {
		if m.Populacao == nil {
			return 0
		}
		return *m.Populacao
	}
	sort.SliceStable(doEstado, func(i, j int) bool {
		if pop(doEstado[i]) != pop(doEstado[j]) {
			return pop(doEstado[i]) > pop(doEstado[j])
		}
		return doEstado[i].Nome < doEstado[j].Nome
	})
	maiores := []aylis.M{}
	for i := 0; i < len(doEstado) && i < 10; i++ {
		maiores = append(maiores, aylis.M{"nome": doEstado[i].Nome, "habitantes": doEstado[i].Populacao})
	}
	carregar(app, frmIBGE.cdsMaiores.Component, maiores)
	f.municipio_mudou(app, ev)
}

// municipio_mudou: outro município (a bandeira, o destaque nos mapas e a população ano a ano).
func (f FrmIBGE) municipio_mudou(app *aylis.Aylis, ev aylis.Event) {
	m := registro(app, frmIBGE.cdsMunicipios.Component)
	if m == nil || m["id"] == nil {
		return
	}
	codigo := int64(m["id"].(float64))
	frmIBGE.lblMunicipio.SetText(texto(m["nome"]))
	frmIBGE.lblCodigo.SetText(fmt.Sprintf("Código IBGE %d", codigo))
	bandeira := texto(m["bandeira"])
	frmIBGE.imgBandeiraMunicipio.SetImage(imagem(bandeira))
	frmIBGE.lblSemBandeira.SetVisible(bandeira == "")
	frmIBGE.lblEstimativa.SetText("Estimativa de " + texto(m["estimativa_ano"]))
	alvo := fmt.Sprintf("m%d", codigo)
	frmIBGE.svgEstado.SetHighlight(alvo)
	frmIBGE.svgMunicipio.SetHighlight(alvo)
	frmIBGE.svgMunicipio.SetZoomTo(alvo)

	linhas := []aylis.M{}
	for _, p := range populacao[strconv.FormatInt(codigo, 10)] {
		linhas = append(linhas, aylis.M{"ano": strconv.FormatInt(p[0], 10), "habitantes": p[1]})
	}
	carregar(app, frmIBGE.cdsPopulacao.Component, linhas)
}
