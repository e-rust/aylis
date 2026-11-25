// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import "aylis.dev/cliente/aylis"

// Register registra os handlers de todos os forms.
func Register(app *aylis.Aylis) {
	app.On("frmCalculadora.form_key_down", func(ev aylis.Event) { FrmCalculadora{}.form_key_down(app, frmCalculadora.aylisForm, recordOfKeyEvent(ev.Args)) })
	app.On("frmCalculadora.limpar_historico_click", func(ev aylis.Event) { FrmCalculadora{}.limpar_historico_click(app, frmCalculadora.SenderAsButton(ev)) })
	app.On("frmCalculadora.historico_item_click", func(ev aylis.Event) { FrmCalculadora{}.historico_item_click(app, frmCalculadora.SenderAsListView(ev), int64(asNumber(ev.Args))) })
	app.On("frmCalculadora.memoria_item_click", func(ev aylis.Event) { FrmCalculadora{}.memoria_item_click(app, frmCalculadora.SenderAsListView(ev), int64(asNumber(ev.Args))) })
	app.On("frmCalculadora.historico_click", func(ev aylis.Event) { FrmCalculadora{}.historico_click(app, frmCalculadora.SenderAsButton(ev)) })
	app.On("frmCalculadora.tecla_click", func(ev aylis.Event) { FrmCalculadora{}.tecla_click(app, frmCalculadora.SenderAsButton(ev)) })
}
