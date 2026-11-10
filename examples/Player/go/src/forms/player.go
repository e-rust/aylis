// Eventos do form frmPlayer: o código é seu (o registro dos handlers fica no forms.go
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
// anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
// toca é o componente VideoPlayer (vpVideo), áudio e vídeo.

package forms

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"aylis.dev/cliente/aylis"
)

// FrmPlayer reúne os handlers do form frmPlayer.
type FrmPlayer struct{}

// O que o form guarda. repetir: 0 não repete, 1 repete a fila, 2 repete a mídia.
var estado = struct {
	fila       []string
	atual      int
	tocando    bool
	embaralhar bool
	repetir    int
	mudo       bool
	semente    int64
}{atual: -1, semente: 12345}

// As extensões de áudio (o resto é vídeo).
var audio = []string{"mp3", "wav", "m4a", "flac", "ogg", "wma", "aac"}

func separador(caminho string) int {
	return max(strings.LastIndex(caminho, "\\"), strings.LastIndex(caminho, "/"))
}

// nomeDoArquivo é o nome do arquivo sem a pasta.
func nomeDoArquivo(caminho string) string {
	return caminho[separador(caminho)+1:]
}

// pasta é a pasta do arquivo (vazio se não tiver).
func pasta(caminho string) string {
	if i := separador(caminho); i >= 0 {
		return caminho[:i]
	}
	return ""
}

func semExtensao(nome string) string {
	if i := strings.LastIndex(nome, "."); i > 0 {
		return nome[:i]
	}
	return nome
}

func tipo(caminho string) string {
	nome := nomeDoArquivo(caminho)
	ext := ""
	if i := strings.LastIndex(nome, "."); i >= 0 {
		ext = strings.ToLower(nome[i+1:])
	}
	if slices.Contains(audio, ext) {
		return "Música"
	}
	return "Vídeo"
}

// tempo: "3:07"
func tempo(segundos float64) string {
	s := int64(math.Floor(math.Max(segundos, 0)))
	return fmt.Sprintf("%d:%02d", s/60, s%60)
}

// atualizarFila: a fila na lista, com a mídia atual selecionada.
func atualizarFila(app *aylis.Aylis) {
	itens := make([]string, 0, len(estado.fila))
	for _, c := range estado.fila {
		itens = append(itens, nomeDoArquivo(c)+"|"+pasta(c))
	}
	frmPlayer.lvFila.SetItems(strings.Join(itens, ";"))
	frmPlayer.lvFila.SetItemIndex(int64(estado.atual))
}

// mostrarAtual: o título, o detalhe e o título da janela.
func mostrarAtual(app *aylis.Aylis) {
	if estado.atual < 0 {
		frmPlayer.lblTitulo.SetText("Nenhuma mídia")
		frmPlayer.lblDetalhe.SetText("")
		frmPlayer.SetCaption("Reprodutor Multimídia")
		return
	}
	caminho := estado.fila[estado.atual]
	titulo := semExtensao(nomeDoArquivo(caminho))
	frmPlayer.lblTitulo.SetText(titulo)
	frmPlayer.lblDetalhe.SetText(fmt.Sprintf("%s · %d de %d", tipo(caminho), estado.atual+1, len(estado.fila)))
	frmPlayer.SetCaption(titulo + " - Reprodutor Multimídia")
}

// botaoReproduzir: o botão do meio mostra o que ele vai fazer: tocar ou pausar.
func botaoReproduzir(app *aylis.Aylis) {
	icone, dica := "play", "Reproduzir (Ctrl+P)"
	if estado.tocando {
		icone, dica = "pause", "Pausar (Ctrl+P)"
	}
	frmPlayer.btnReproduzir.SetIcon(icone)
	frmPlayer.btnReproduzir.SetHint(dica)
}

// tocar toca a mídia i da fila. Tocar é a última coisa: um erro chega no video_erro.
func tocar(app *aylis.Aylis, i int) {
	estado.atual = i
	estado.tocando = true
	frmPlayer.vpVideo.SetFile(estado.fila[i])
	mostrarAtual(app)
	atualizarFila(app)
	botaoReproduzir(app)
	frmPlayer.sbPosicao.SetPosition(0.0)
	frmPlayer.vpVideo.Play()
}

// proximoIndice: a próxima da fila, na ordem ou embaralhada; no fim, volta ao começo se
// repetir a fila. -1: acabou.
func proximoIndice() int {
	n := len(estado.fila)
	if n == 0 {
		return -1
	}
	if estado.embaralhar {
		if n == 1 {
			return 0
		}
		for {
			estado.semente = (estado.semente*1103515245 + 12345) % 2147483648
			i := int(estado.semente % int64(n))
			if i != estado.atual {
				return i
			}
		}
	}
	if estado.atual+1 < n {
		return estado.atual + 1
	}
	if estado.repetir == 1 {
		return 0
	}
	return -1
}

func (f FrmPlayer) abrir_click(app *aylis.Aylis, ev aylis.Event) {
	frmPlayer.dlgAbrir.Execute()
}

func (f FrmPlayer) abrir_select(app *aylis.Aylis, ev aylis.Event) {
	// Os arquivos escolhidos vêm separados por ";".
	texto, _ := ev.Args.(string)
	var novos []string
	for _, p := range strings.Split(texto, ";") {
		if p != "" {
			novos = append(novos, p)
		}
	}
	if len(novos) == 0 {
		return
	}
	primeiro := len(estado.fila)
	estado.fila = append(estado.fila, novos...)
	atualizarFila(app)
	if estado.atual < 0 {
		tocar(app, primeiro)
	} else {
		mostrarAtual(app)
	}
}

func (f FrmPlayer) limpar_fila_click(app *aylis.Aylis, ev aylis.Event) {
	frmPlayer.vpVideo.Stop()
	estado.fila = nil
	estado.atual = -1
	estado.tocando = false
	frmPlayer.vpVideo.SetFile("")
	atualizarFila(app)
	mostrarAtual(app)
	botaoReproduzir(app)
}

func (f FrmPlayer) sair_click(app *aylis.Aylis, ev aylis.Event) {
	frmPlayer.Close()
}

func (f FrmPlayer) reproduzir_click(app *aylis.Aylis, ev aylis.Event) {
	switch {
	case estado.atual < 0:
		if len(estado.fila) > 0 {
			tocar(app, 0)
		} else {
			f.abrir_click(app, ev)
		}
	case estado.tocando:
		frmPlayer.vpVideo.Pause()
		estado.tocando = false
		botaoReproduzir(app)
	default:
		estado.tocando = true
		botaoReproduzir(app)
		frmPlayer.vpVideo.Play()
	}
}

func (f FrmPlayer) parar_click(app *aylis.Aylis, ev aylis.Event) {
	if estado.atual < 0 {
		return
	}
	frmPlayer.vpVideo.Stop()
	estado.tocando = false
	botaoReproduzir(app)
	frmPlayer.sbPosicao.SetPosition(0.0)
}

func (f FrmPlayer) anterior_click(app *aylis.Aylis, ev aylis.Event) {
	switch atual := estado.atual; {
	case atual > 0:
		tocar(app, atual-1)
	case atual == 0 && estado.repetir == 1:
		tocar(app, len(estado.fila)-1)
	case atual == 0:
		frmPlayer.vpVideo.Seek(0.0)
	}
}

func (f FrmPlayer) proxima_click(app *aylis.Aylis, ev aylis.Event) {
	if i := proximoIndice(); i >= 0 {
		tocar(app, i)
	}
}

func (f FrmPlayer) embaralhar_click(app *aylis.Aylis, ev aylis.Event) {
	estado.embaralhar = !estado.embaralhar
	ligado := estado.embaralhar
	dica := "Embaralhar: desativado"
	if ligado {
		dica = "Embaralhar: ativado"
	}
	frmPlayer.btnEmbaralhar.SetChecked(ligado)
	frmPlayer.btnEmbaralhar.SetHint(dica)
	frmPlayer.mnuEmbaralhar.SetChecked(ligado)
}

func (f FrmPlayer) repetir_click(app *aylis.Aylis, ev aylis.Event) {
	estado.repetir = (estado.repetir + 1) % 3
	modo := estado.repetir
	icone := "repeat"
	if modo == 2 {
		icone = "repeat_one"
	}
	frmPlayer.btnRepetir.SetChecked(modo > 0)
	frmPlayer.btnRepetir.SetIcon(icone)
	frmPlayer.btnRepetir.SetHint([]string{"Repetir: desativado", "Repetir: todos", "Repetir: um"}[modo])
	frmPlayer.mnuRepetir.SetChecked(modo > 0)
}

// pagina mostra a página "Reproduzindo agora" ou a "Fila de reprodução".
func pagina(app *aylis.Aylis, agora bool) {
	frmPlayer.pnlAgora.SetVisible(agora)
	frmPlayer.pnlFila.SetVisible(!agora)
	frmPlayer.btnAgora.SetChecked(agora)
	frmPlayer.btnFila.SetChecked(!agora)
}

func (f FrmPlayer) agora_click(app *aylis.Aylis, ev aylis.Event) {
	pagina(app, true)
}

func (f FrmPlayer) fila_click(app *aylis.Aylis, ev aylis.Event) {
	pagina(app, false)
}

func (f FrmPlayer) posicao_seek(app *aylis.Aylis, ev aylis.Event) {
	if s, ok := ev.Args.(float64); ok {
		frmPlayer.vpVideo.Seek(s)
	}
}

func (f FrmPlayer) mudo_click(app *aylis.Aylis, ev aylis.Event) {
	estado.mudo = !estado.mudo
	icone, dica := "volume", "Sem som"
	if estado.mudo {
		icone, dica = "mute", "Com som"
	}
	frmPlayer.vpVideo.SetMuted(estado.mudo)
	frmPlayer.btnMudo.SetIcon(icone)
	frmPlayer.btnMudo.SetHint(dica)
}

func (f FrmPlayer) volume_change(app *aylis.Aylis, ev aylis.Event) {
	if v, ok := ev.Args.(float64); ok {
		frmPlayer.vpVideo.SetVolume(v / 100)
	}
}

func (f FrmPlayer) video_fim(app *aylis.Aylis, ev aylis.Event) {
	if estado.repetir == 2 && estado.atual >= 0 {
		tocar(app, estado.atual)
		return
	}
	if i := proximoIndice(); i >= 0 {
		tocar(app, i)
	} else {
		estado.tocando = false
		botaoReproduzir(app)
	}
}

func (f FrmPlayer) video_erro(app *aylis.Aylis, ev aylis.Event) {
	estado.tocando = false
	botaoReproduzir(app)
	frmPlayer.lblDetalhe.SetText("Não foi possível reproduzir este arquivo")
}

func (f FrmPlayer) fila_item_click(app *aylis.Aylis, ev aylis.Event) {
	if n, ok := ev.Args.(float64); ok {
		if i := int(n); i >= 0 && i < len(estado.fila) {
			tocar(app, i)
		}
	}
}

func (f FrmPlayer) posicao_timer(app *aylis.Aylis, ev aylis.Event) {
	if estado.atual < 0 {
		return
	}
	posicao := frmPlayer.vpVideo.Position()
	duracao := frmPlayer.vpVideo.Duration()
	frmPlayer.sbPosicao.SetDuration(duracao)
	frmPlayer.sbPosicao.SetPosition(posicao)
	frmPlayer.lblTempo.SetText(tempo(posicao))
	frmPlayer.lblDuracao.SetText(tempo(duracao))
}
