"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import paint
from .components import *  # noqa: F401,F403
from .components import _enum, _date, _time, _datetime, _list, _numbers, _colors, _date_range, _record  # noqa: F401


def register(aylis):
    aylis.on("frmPaint.novo_click")(lambda event: paint.novo_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.abrir_click")(lambda event: paint.abrir_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.salvar_click")(lambda event: paint.salvar_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPaint.salvar_como_click")(lambda event: paint.salvar_como_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.sair_click")(lambda event: paint.sair_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.desfazer_click")(lambda event: paint.desfazer_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPaint.refazer_click")(lambda event: paint.refazer_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPaint.recortar_click")(lambda event: paint.recortar_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.copiar_click")(lambda event: paint.copiar_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.colar_click")(lambda event: paint.colar_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.excluir_click")(lambda event: paint.excluir_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.selecionar_tudo_click")(lambda event: paint.selecionar_tudo_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.limpar_click")(lambda event: paint.limpar_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.ampliar_click")(lambda event: paint.ampliar_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.reduzir_click")(lambda event: paint.reduzir_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.tamanho_real_click")(lambda event: paint.tamanho_real_click(aylis, components.frmPaint.sender_as_menu_item(event)))
    aylis.on("frmPaint.ferramenta_click")(lambda event: paint.ferramenta_click(aylis, components.frmPaint.sender_as_button(event)))
    aylis.on("frmPaint.preencher_change")(lambda event: paint.preencher_change(aylis, components.frmPaint.sender_as_check_box(event), event.get("args")))
    aylis.on("frmPaint.espessura_change")(lambda event: paint.espessura_change(aylis, components.frmPaint.sender_as_combo_box(event), event.get("args")))
    aylis.on("frmPaint.cor1_click")(lambda event: paint.cor1_click(aylis, components.frmPaint.sender_as_circle(event)))
    aylis.on("frmPaint.cor2_click")(lambda event: paint.cor2_click(aylis, components.frmPaint.sender_as_circle(event)))
    aylis.on("frmPaint.paleta_click")(lambda event: paint.paleta_click(aylis, components.frmPaint.sender_as_circle(event)))
    aylis.on("frmPaint.desenho_change")(lambda event: paint.desenho_change(aylis, components.frmPaint.sender_as_paint_canvas(event)))
    aylis.on("frmPaint.desenho_pick")(lambda event: paint.desenho_pick(aylis, components.frmPaint.sender_as_paint_canvas(event), event.get("args")))
    aylis.on("frmPaint.desenho_cursor")(lambda event: paint.desenho_cursor(aylis, components.frmPaint.sender_as_paint_canvas(event), event.get("args")))
    aylis.on("frmPaint.abrir_select")(lambda event: paint.abrir_select(aylis, components.frmPaint.sender_as_open_dialog(event), _list(event.get("args"))))
    aylis.on("frmPaint.salvar_select")(lambda event: paint.salvar_select(aylis, components.frmPaint.sender_as_save_dialog(event), _list(event.get("args"))))
