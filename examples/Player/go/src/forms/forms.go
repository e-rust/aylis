// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import "aylis.dev/cliente/aylis"

// Register registra os handlers de todos os forms.
func Register(app *aylis.Aylis) {
	app.On("frmPlayer.abrir_click", func(ev aylis.Event) { FrmPlayer{}.abrir_click(app, ev) })
	app.On("frmPlayer.limpar_fila_click", func(ev aylis.Event) { FrmPlayer{}.limpar_fila_click(app, ev) })
	app.On("frmPlayer.sair_click", func(ev aylis.Event) { FrmPlayer{}.sair_click(app, ev) })
	app.On("frmPlayer.reproduzir_click", func(ev aylis.Event) { FrmPlayer{}.reproduzir_click(app, ev) })
	app.On("frmPlayer.parar_click", func(ev aylis.Event) { FrmPlayer{}.parar_click(app, ev) })
	app.On("frmPlayer.anterior_click", func(ev aylis.Event) { FrmPlayer{}.anterior_click(app, ev) })
	app.On("frmPlayer.proxima_click", func(ev aylis.Event) { FrmPlayer{}.proxima_click(app, ev) })
	app.On("frmPlayer.embaralhar_click", func(ev aylis.Event) { FrmPlayer{}.embaralhar_click(app, ev) })
	app.On("frmPlayer.repetir_click", func(ev aylis.Event) { FrmPlayer{}.repetir_click(app, ev) })
	app.On("frmPlayer.agora_click", func(ev aylis.Event) { FrmPlayer{}.agora_click(app, ev) })
	app.On("frmPlayer.fila_click", func(ev aylis.Event) { FrmPlayer{}.fila_click(app, ev) })
	app.On("frmPlayer.posicao_seek", func(ev aylis.Event) { FrmPlayer{}.posicao_seek(app, ev) })
	app.On("frmPlayer.mudo_click", func(ev aylis.Event) { FrmPlayer{}.mudo_click(app, ev) })
	app.On("frmPlayer.volume_change", func(ev aylis.Event) { FrmPlayer{}.volume_change(app, ev) })
	app.On("frmPlayer.video_fim", func(ev aylis.Event) { FrmPlayer{}.video_fim(app, ev) })
	app.On("frmPlayer.video_erro", func(ev aylis.Event) { FrmPlayer{}.video_erro(app, ev) })
	app.On("frmPlayer.fila_item_click", func(ev aylis.Event) { FrmPlayer{}.fila_item_click(app, ev) })
	app.On("frmPlayer.abrir_select", func(ev aylis.Event) { FrmPlayer{}.abrir_select(app, ev) })
	app.On("frmPlayer.posicao_timer", func(ev aylis.Event) { FrmPlayer{}.posicao_timer(app, ev) })
}
