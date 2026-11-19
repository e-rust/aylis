// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import "aylis.dev/cliente/aylis"

// Register registra os handlers de todos os forms.
func Register(app *aylis.Aylis) {
	app.On("frmIBGE.form_show", func(ev aylis.Event) { FrmIBGE{}.form_show(app, frmIBGE.aylisForm) })
	app.On("frmIBGE.estado_mudou", func(ev aylis.Event) { FrmIBGE{}.estado_mudou(app, frmIBGE.SenderAsClientDataSet(ev)) })
	app.On("frmIBGE.municipio_mudou", func(ev aylis.Event) { FrmIBGE{}.municipio_mudou(app, frmIBGE.SenderAsClientDataSet(ev)) })
}
