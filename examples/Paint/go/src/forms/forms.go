// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms

import "aylis.dev/cliente/aylis"

// Register registra os handlers de todos os forms.
func Register(app *aylis.Aylis) {
	app.On("frmPaint.novo_click", func(ev aylis.Event) { FrmPaint{}.novo_click(app, ev) })
	app.On("frmPaint.abrir_click", func(ev aylis.Event) { FrmPaint{}.abrir_click(app, ev) })
	app.On("frmPaint.salvar_click", func(ev aylis.Event) { FrmPaint{}.salvar_click(app, ev) })
	app.On("frmPaint.salvar_como_click", func(ev aylis.Event) { FrmPaint{}.salvar_como_click(app, ev) })
	app.On("frmPaint.sair_click", func(ev aylis.Event) { FrmPaint{}.sair_click(app, ev) })
	app.On("frmPaint.desfazer_click", func(ev aylis.Event) { FrmPaint{}.desfazer_click(app, ev) })
	app.On("frmPaint.refazer_click", func(ev aylis.Event) { FrmPaint{}.refazer_click(app, ev) })
	app.On("frmPaint.recortar_click", func(ev aylis.Event) { FrmPaint{}.recortar_click(app, ev) })
	app.On("frmPaint.copiar_click", func(ev aylis.Event) { FrmPaint{}.copiar_click(app, ev) })
	app.On("frmPaint.colar_click", func(ev aylis.Event) { FrmPaint{}.colar_click(app, ev) })
	app.On("frmPaint.excluir_click", func(ev aylis.Event) { FrmPaint{}.excluir_click(app, ev) })
	app.On("frmPaint.selecionar_tudo_click", func(ev aylis.Event) { FrmPaint{}.selecionar_tudo_click(app, ev) })
	app.On("frmPaint.limpar_click", func(ev aylis.Event) { FrmPaint{}.limpar_click(app, ev) })
	app.On("frmPaint.ampliar_click", func(ev aylis.Event) { FrmPaint{}.ampliar_click(app, ev) })
	app.On("frmPaint.reduzir_click", func(ev aylis.Event) { FrmPaint{}.reduzir_click(app, ev) })
	app.On("frmPaint.tamanho_real_click", func(ev aylis.Event) { FrmPaint{}.tamanho_real_click(app, ev) })
	app.On("frmPaint.ferramenta_click", func(ev aylis.Event) { FrmPaint{}.ferramenta_click(app, ev) })
	app.On("frmPaint.preencher_change", func(ev aylis.Event) { FrmPaint{}.preencher_change(app, ev) })
	app.On("frmPaint.espessura_change", func(ev aylis.Event) { FrmPaint{}.espessura_change(app, ev) })
	app.On("frmPaint.cor1_click", func(ev aylis.Event) { FrmPaint{}.cor1_click(app, ev) })
	app.On("frmPaint.cor2_click", func(ev aylis.Event) { FrmPaint{}.cor2_click(app, ev) })
	app.On("frmPaint.paleta_click", func(ev aylis.Event) { FrmPaint{}.paleta_click(app, ev) })
	app.On("frmPaint.desenho_change", func(ev aylis.Event) { FrmPaint{}.desenho_change(app, ev) })
	app.On("frmPaint.desenho_pick", func(ev aylis.Event) { FrmPaint{}.desenho_pick(app, ev) })
	app.On("frmPaint.desenho_cursor", func(ev aylis.Event) { FrmPaint{}.desenho_cursor(app, ev) })
	app.On("frmPaint.abrir_select", func(ev aylis.Event) { FrmPaint{}.abrir_select(app, ev) })
	app.On("frmPaint.salvar_select", func(ev aylis.Event) { FrmPaint{}.salvar_select(app, ev) })
}
