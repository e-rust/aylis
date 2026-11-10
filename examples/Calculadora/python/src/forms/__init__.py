"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import calculadora


def register(aylis):
    aylis.on("frmCalculadora.form_key_down")(lambda event: calculadora.form_key_down(aylis, event))
    aylis.on("frmCalculadora.limpar_historico_click")(lambda event: calculadora.limpar_historico_click(aylis, event))
    aylis.on("frmCalculadora.historico_item_click")(lambda event: calculadora.historico_item_click(aylis, event))
    aylis.on("frmCalculadora.memoria_item_click")(lambda event: calculadora.memoria_item_click(aylis, event))
    aylis.on("frmCalculadora.historico_click")(lambda event: calculadora.historico_click(aylis, event))
    aylis.on("frmCalculadora.tecla_click")(lambda event: calculadora.tecla_click(aylis, event))
