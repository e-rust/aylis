// Eventos do form frmCalculadora: o código é seu (o registro dos handlers fica no forms.go
// gerado). Dentro de um handler, o form é a janela do evento.

package forms

import (
	"strings"
	"unicode/utf8"

	"aylis.dev/cliente/aylis"
)

// FrmCalculadora reúne os handlers do form frmCalculadora.
type FrmCalculadora struct{}

var calc = &Calculadora{}

// semErro são os botões que só funcionam sem erro no visor (os da janela do evento).
func semErro() []aylisButton {
	return []aylisButton{
		frmCalculadora.btnMMais, frmCalculadora.btnMMenos, frmCalculadora.btnMS, frmCalculadora.btnPorcento,
		frmCalculadora.btnInverso, frmCalculadora.btnQuadrado, frmCalculadora.btnRaiz, frmCalculadora.btnDividir,
		frmCalculadora.btnMultiplicar, frmCalculadora.btnSubtrair, frmCalculadora.btnSomar, frmCalculadora.btnSinal,
		frmCalculadora.btnVirgula,
	}
}

// atualizar põe o estado da calculadora na tela.
func atualizar(app *aylis.Aylis) {
	visor := calc.Visor()
	// O número encolhe para caber, como no Windows.
	n := utf8.RuneCountInString(visor)
	tamanho := 46
	if calc.TemErro() {
		tamanho = 26
	} else if n > 11 {
		tamanho = max(46*11/n, 22)
	}
	frmCalculadora.lblVisor.SetText(visor)
	frmCalculadora.lblVisor.SetFontSize(float64(tamanho))
	frmCalculadora.lblExpressao.SetText(calc.Expressao)
	erro := calc.TemErro()
	for _, botao := range semErro() {
		botao.SetEnabled(!erro)
	}
	for _, botao := range []aylisButton{frmCalculadora.btnMC, frmCalculadora.btnMR} {
		botao.SetEnabled(!erro && len(calc.Memoria) > 0)
	}
	itens := []string{}
	for i := len(calc.Historico) - 1; i >= 0; i-- {
		itens = append(itens, Formatar(calc.Historico[i].Resultado)+"|"+calc.Historico[i].Expressao)
	}
	historico := "Ainda não há histórico|"
	if len(itens) > 0 {
		historico = strings.Join(itens, ";")
	}
	frmCalculadora.lvHistorico.SetItems(historico)
	itens = itens[:0]
	for i := len(calc.Memoria) - 1; i >= 0; i-- {
		itens = append(itens, Formatar(calc.Memoria[i])+"|")
	}
	memoria := "Não há nada salvo na memória|"
	if len(itens) > 0 {
		memoria = strings.Join(itens, ";")
	}
	frmCalculadora.lvMemoria.SetItems(memoria)
}

// apertar aperta um botão pelo texto dele.
func apertar(app *aylis.Aylis, texto string) {
	if calc.Tecla(texto) {
		atualizar(app)
	}
}

func indice(ev aylis.Event) int {
	if n, ok := ev.Args.(float64); ok {
		return int(n)
	}
	return -1
}

func (f FrmCalculadora) form_key_down(app *aylis.Aylis, ev aylis.Event) {
	args, _ := ev.Args.(map[string]any)
	tecla, _ := args["key"].(string)
	ctrl, _ := args["ctrl"].(bool)
	shift, _ := args["shift"].(bool)
	switch botao := BotaoDoTeclado(tecla, ctrl, shift); botao {
	case "historico":
		f.historico_click(app, ev)
	case "":
	default:
		apertar(app, botao)
	}
}

func (f FrmCalculadora) limpar_historico_click(app *aylis.Aylis, ev aylis.Event) {
	calc.Historico = nil
	atualizar(app)
}

func (f FrmCalculadora) historico_item_click(app *aylis.Aylis, ev aylis.Event) {
	// A lista mostra a conta mais nova em cima.
	if i := indice(ev); i >= 0 && i < len(calc.Historico) {
		calc.Usar(calc.Historico[len(calc.Historico)-1-i].Resultado)
		atualizar(app)
	}
}

func (f FrmCalculadora) memoria_item_click(app *aylis.Aylis, ev aylis.Event) {
	if i := indice(ev); i >= 0 && i < len(calc.Memoria) {
		calc.Usar(calc.Memoria[len(calc.Memoria)-1-i])
		atualizar(app)
	}
}

func (f FrmCalculadora) historico_click(app *aylis.Aylis, ev aylis.Event) {
	frmCalculadora.pnlLateral.SetVisible(!frmCalculadora.pnlLateral.Visible())
}

func (f FrmCalculadora) tecla_click(app *aylis.Aylis, ev aylis.Event) {
	apertar(app, frmCalculadora.SenderAsButton(ev).Text())
}
