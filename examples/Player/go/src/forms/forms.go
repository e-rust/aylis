// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import "aylis.dev/cliente/aylis"

// Register registra os handlers de todos os forms.
func Register(app *aylis.Aylis) {
	app.On("frmPlayer.abrir_click", func(ev aylis.Event) { FrmPlayer{}.abrir_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.limpar_fila_click", func(ev aylis.Event) { FrmPlayer{}.limpar_fila_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.sair_click", func(ev aylis.Event) { FrmPlayer{}.sair_click(app, frmPlayer.SenderAsMenuItem(ev)) })
	app.On("frmPlayer.reproduzir_click", func(ev aylis.Event) { FrmPlayer{}.reproduzir_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.parar_click", func(ev aylis.Event) { FrmPlayer{}.parar_click(app, frmPlayer.SenderAsMenuItem(ev)) })
	app.On("frmPlayer.anterior_click", func(ev aylis.Event) { FrmPlayer{}.anterior_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.proxima_click", func(ev aylis.Event) { FrmPlayer{}.proxima_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.embaralhar_click", func(ev aylis.Event) { FrmPlayer{}.embaralhar_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.repetir_click", func(ev aylis.Event) { FrmPlayer{}.repetir_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.agora_click", func(ev aylis.Event) { FrmPlayer{}.agora_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.fila_click", func(ev aylis.Event) { FrmPlayer{}.fila_click(app, aylis.Component{App: app, ID: int64(ev.Sender.ID), Name: ev.Sender.Name}) })
	app.On("frmPlayer.posicao_seek", func(ev aylis.Event) { FrmPlayer{}.posicao_seek(app, frmPlayer.SenderAsSeekBar(ev), asNumber(ev.Args)) })
	app.On("frmPlayer.mudo_click", func(ev aylis.Event) { FrmPlayer{}.mudo_click(app, frmPlayer.SenderAsButton(ev)) })
	app.On("frmPlayer.volume_change", func(ev aylis.Event) { FrmPlayer{}.volume_change(app, frmPlayer.SenderAsTrackBar(ev), asNumber(ev.Args)) })
	app.On("frmPlayer.video_fim", func(ev aylis.Event) { FrmPlayer{}.video_fim(app, frmPlayer.SenderAsVideoPlayer(ev)) })
	app.On("frmPlayer.video_erro", func(ev aylis.Event) { FrmPlayer{}.video_erro(app, frmPlayer.SenderAsVideoPlayer(ev), asText(ev.Args)) })
	app.On("frmPlayer.fila_item_click", func(ev aylis.Event) { FrmPlayer{}.fila_item_click(app, frmPlayer.SenderAsListView(ev), int64(asNumber(ev.Args))) })
	app.On("frmPlayer.abrir_select", func(ev aylis.Event) { FrmPlayer{}.abrir_select(app, frmPlayer.SenderAsOpenDialog(ev), asList(ev.Args)) })
	app.On("frmPlayer.posicao_timer", func(ev aylis.Event) { FrmPlayer{}.posicao_timer(app, frmPlayer.SenderAsTimer(ev)) })
}
