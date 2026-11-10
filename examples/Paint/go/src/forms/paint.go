// Eventos do form frmPaint: o código é seu (o registro dos handlers fica no forms.go
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
// ferramentas, as cores, o zoom, o arquivo e a barra de status.

package forms

import (
	"fmt"
	"strconv"
	"strings"

	"aylis.dev/cliente/aylis"
)

// FrmPaint reúne os handlers do form frmPaint.
type FrmPaint struct{}

// O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%).
var estado = struct {
	arquivo    string
	modificado bool
	corAtiva   int
	zoom       int
}{corAtiva: 1, zoom: 100}

// ferramenta: um botão de ferramenta e a ferramenta dele.
type ferramenta struct {
	botao aylisButton
	nome  string
}

// ferramentas: os botões de ferramenta (os do form do evento) e a ferramenta de cada um.
func ferramentas() []ferramenta {
	return []ferramenta{
		{frmPaint.btnSelecionar, "select"}, {frmPaint.btnLapis, "pencil"}, {frmPaint.btnPincel, "brush"}, {frmPaint.btnBorracha, "eraser"},
		{frmPaint.btnBalde, "fill"}, {frmPaint.btnContaGotas, "picker"}, {frmPaint.btnLinha, "line"}, {frmPaint.btnRetangulo, "rectangle"},
		{frmPaint.btnElipse, "ellipse"},
	}
}

// invocar: abrir e salvar pelo invoke direto, que devolve o erro para a mensagem.
func invocar(app *aylis.Aylis, metodo string, args ...any) (any, error) {
	if args == nil {
		args = []any{}
	}
	return app.Call("invoke", aylis.M{"target": frmPaint.pcDesenho.ID, "method": metodo, "args": args})
}

func texto(ev aylis.Event) string {
	s, _ := ev.Args.(string)
	return s
}

// nomeDoArquivo é o nome do arquivo sem a pasta.
func nomeDoArquivo(caminho string) string {
	partes := strings.FieldsFunc(caminho, func(r rune) bool { return r == '\\' || r == '/' })
	if len(partes) == 0 {
		return caminho
	}
	return partes[len(partes)-1]
}

// titulo é o título como o do Paint do Windows: "*Sem título - Paint".
func titulo(app *aylis.Aylis) {
	nome := "Sem título"
	if estado.arquivo != "" {
		nome = nomeDoArquivo(estado.arquivo)
	}
	marca := ""
	if estado.modificado {
		marca = "*"
	}
	frmPaint.SetCaption(marca + nome + " - Paint")
}

// atualizarEdicao: desfazer, refazer e o tamanho da imagem acompanham o desenho.
func atualizarEdicao(app *aylis.Aylis) {
	desfazer, refazer := frmPaint.pcDesenho.CanUndo(), frmPaint.pcDesenho.CanRedo()
	frmPaint.mnuDesfazer.SetEnabled(desfazer)
	frmPaint.btnDesfazer.SetEnabled(desfazer)
	frmPaint.mnuRefazer.SetEnabled(refazer)
	frmPaint.btnRefazer.SetEnabled(refazer)
	frmPaint.lblTamanho.SetText(frmPaint.pcDesenho.Size() + "px")
}

func marcarModificado(app *aylis.Aylis) {
	estado.modificado = true
	titulo(app)
	atualizarEdicao(app)
}

func aplicarZoom(app *aylis.Aylis, zoom int) {
	estado.zoom = zoom
	frmPaint.pcDesenho.SetZoom(float64(zoom) / 100)
	frmPaint.lblZoom.SetText(fmt.Sprintf("%d%%", zoom))
}

// marcarFerramenta: só o botão da ferramenta escolhida fica marcado.
func marcarFerramenta(app *aylis.Aylis, escolhido aylisButton) {
	for _, f := range ferramentas() {
		f.botao.SetChecked(f.botao.ID == escolhido.ID)
	}
}

// marcarCor: a cor que a paleta troca fica com o anel azul, como no Paint.
func marcarCor(app *aylis.Aylis, cor int) {
	estado.corAtiva = cor
	for n, circulo := range []aylisCircle{frmPaint.btnCor1, frmPaint.btnCor2} {
		if n+1 == cor {
			circulo.SetStroke("#0067C0")
			circulo.SetStrokeWidth(3.0)
		} else {
			circulo.SetStroke("#9A9A9A")
			circulo.SetStrokeWidth(1.0)
		}
	}
}

func erro(app *aylis.Aylis, texto string) {
	app.Send("message_dlg", aylis.M{"text": texto, "kind": "error", "buttons": []string{"ok"}})
}

func salvarEm(app *aylis.Aylis, caminho string) {
	if _, err := invocar(app, "save", caminho); err != nil {
		erro(app, "Não foi possível salvar a imagem.\n"+err.Error())
		return
	}
	estado.arquivo = caminho
	estado.modificado = false
	titulo(app)
}

func (f FrmPaint) novo_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.New(640, 400)
	estado.arquivo = ""
	estado.modificado = false
	titulo(app)
	atualizarEdicao(app)
}

func (f FrmPaint) abrir_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.dlgAbrir.Execute()
}

func (f FrmPaint) salvar_click(app *aylis.Aylis, ev aylis.Event) {
	if estado.arquivo != "" {
		salvarEm(app, estado.arquivo)
	} else {
		f.salvar_como_click(app, ev)
	}
}

func (f FrmPaint) salvar_como_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.dlgSalvar.Execute()
}

func (f FrmPaint) sair_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.Close()
}

func (f FrmPaint) desfazer_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.Undo()
	marcarModificado(app)
}

func (f FrmPaint) refazer_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.Redo()
	marcarModificado(app)
}

func (f FrmPaint) recortar_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.Cut()
}

func (f FrmPaint) copiar_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.Copy()
}

func (f FrmPaint) colar_click(app *aylis.Aylis, ev aylis.Event) {
	// O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
	if frmPaint.pcDesenho.Paste() {
		marcarFerramenta(app, frmPaint.btnSelecionar)
	}
}

func (f FrmPaint) excluir_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.Delete()
}

func (f FrmPaint) selecionar_tudo_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.SelectAll()
	marcarFerramenta(app, frmPaint.btnSelecionar)
}

func (f FrmPaint) limpar_click(app *aylis.Aylis, ev aylis.Event) {
	frmPaint.pcDesenho.Clear()
	marcarModificado(app)
}

func (f FrmPaint) ampliar_click(app *aylis.Aylis, ev aylis.Event) {
	aplicarZoom(app, min(estado.zoom*2, 800))
}

func (f FrmPaint) reduzir_click(app *aylis.Aylis, ev aylis.Event) {
	aplicarZoom(app, max(estado.zoom/2, 25))
}

func (f FrmPaint) tamanho_real_click(app *aylis.Aylis, ev aylis.Event) {
	aplicarZoom(app, 100)
}

func (f FrmPaint) ferramenta_click(app *aylis.Aylis, ev aylis.Event) {
	clicado := frmPaint.SenderAsButton(ev)
	for _, fe := range ferramentas() {
		if clicado.ID != 0 && fe.botao.ID == clicado.ID {
			frmPaint.pcDesenho.SetTool(fe.nome)
			marcarFerramenta(app, fe.botao)
			return
		}
	}
}

func (f FrmPaint) preencher_change(app *aylis.Aylis, ev aylis.Event) {
	if b, ok := ev.Args.(bool); ok {
		frmPaint.pcDesenho.SetFilled(b)
	}
}

func (f FrmPaint) espessura_change(app *aylis.Aylis, ev aylis.Event) {
	// "5 px" → 5
	if n, err := strconv.Atoi(strings.Split(texto(ev), " ")[0]); err == nil {
		frmPaint.pcDesenho.SetBrushSize(int64(n))
	}
}

func (f FrmPaint) cor1_click(app *aylis.Aylis, ev aylis.Event) {
	marcarCor(app, 1)
}

func (f FrmPaint) cor2_click(app *aylis.Aylis, ev aylis.Event) {
	marcarCor(app, 2)
}

func (f FrmPaint) paleta_click(app *aylis.Aylis, ev aylis.Event) {
	cor := frmPaint.SenderAsCircle(ev).Fill()
	if estado.corAtiva == 1 {
		frmPaint.pcDesenho.SetColor(cor)
		frmPaint.btnCor1.SetFill(cor)
	} else {
		frmPaint.pcDesenho.SetBackColor(cor)
		frmPaint.btnCor2.SetFill(cor)
	}
}

func (f FrmPaint) desenho_change(app *aylis.Aylis, ev aylis.Event) {
	marcarModificado(app)
}

func (f FrmPaint) desenho_pick(app *aylis.Aylis, ev aylis.Event) {
	// "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
	t := texto(ev)
	if strings.HasSuffix(t, "2") {
		frmPaint.btnCor2.SetFill(t[:len(t)-1])
	} else {
		frmPaint.btnCor1.SetFill(t)
	}
}

func (f FrmPaint) desenho_cursor(app *aylis.Aylis, ev aylis.Event) {
	t := texto(ev)
	if t != "" {
		t += "px"
	}
	frmPaint.lblPosicao.SetText(t)
}

func (f FrmPaint) abrir_select(app *aylis.Aylis, ev aylis.Event) {
	caminho := texto(ev)
	if _, err := invocar(app, "open", caminho); err != nil {
		erro(app, "Não foi possível abrir a imagem.\n"+err.Error())
		return
	}
	estado.arquivo = caminho
	estado.modificado = false
	titulo(app)
	atualizarEdicao(app)
}

func (f FrmPaint) salvar_select(app *aylis.Aylis, ev aylis.Event) {
	salvarEm(app, texto(ev))
}
