"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import paint


def register(aylis):
    aylis.on("frmPaint.novo_click")(lambda event: paint.novo_click(aylis, event))
    aylis.on("frmPaint.abrir_click")(lambda event: paint.abrir_click(aylis, event))
    aylis.on("frmPaint.salvar_click")(lambda event: paint.salvar_click(aylis, event))
    aylis.on("frmPaint.salvar_como_click")(lambda event: paint.salvar_como_click(aylis, event))
    aylis.on("frmPaint.sair_click")(lambda event: paint.sair_click(aylis, event))
    aylis.on("frmPaint.desfazer_click")(lambda event: paint.desfazer_click(aylis, event))
    aylis.on("frmPaint.refazer_click")(lambda event: paint.refazer_click(aylis, event))
    aylis.on("frmPaint.recortar_click")(lambda event: paint.recortar_click(aylis, event))
    aylis.on("frmPaint.copiar_click")(lambda event: paint.copiar_click(aylis, event))
    aylis.on("frmPaint.colar_click")(lambda event: paint.colar_click(aylis, event))
    aylis.on("frmPaint.excluir_click")(lambda event: paint.excluir_click(aylis, event))
    aylis.on("frmPaint.selecionar_tudo_click")(lambda event: paint.selecionar_tudo_click(aylis, event))
    aylis.on("frmPaint.limpar_click")(lambda event: paint.limpar_click(aylis, event))
    aylis.on("frmPaint.ampliar_click")(lambda event: paint.ampliar_click(aylis, event))
    aylis.on("frmPaint.reduzir_click")(lambda event: paint.reduzir_click(aylis, event))
    aylis.on("frmPaint.tamanho_real_click")(lambda event: paint.tamanho_real_click(aylis, event))
    aylis.on("frmPaint.ferramenta_click")(lambda event: paint.ferramenta_click(aylis, event))
    aylis.on("frmPaint.preencher_change")(lambda event: paint.preencher_change(aylis, event))
    aylis.on("frmPaint.espessura_change")(lambda event: paint.espessura_change(aylis, event))
    aylis.on("frmPaint.cor1_click")(lambda event: paint.cor1_click(aylis, event))
    aylis.on("frmPaint.cor2_click")(lambda event: paint.cor2_click(aylis, event))
    aylis.on("frmPaint.paleta_click")(lambda event: paint.paleta_click(aylis, event))
    aylis.on("frmPaint.desenho_change")(lambda event: paint.desenho_change(aylis, event))
    aylis.on("frmPaint.desenho_pick")(lambda event: paint.desenho_pick(aylis, event))
    aylis.on("frmPaint.desenho_cursor")(lambda event: paint.desenho_cursor(aylis, event))
    aylis.on("frmPaint.abrir_select")(lambda event: paint.abrir_select(aylis, event))
    aylis.on("frmPaint.salvar_select")(lambda event: paint.salvar_select(aylis, event))
