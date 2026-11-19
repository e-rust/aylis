"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import ibge
from .components import *  # noqa: F401,F403
from .components import _enum, _date, _time, _datetime, _list, _numbers, _colors, _date_range, _record  # noqa: F401


def register(aylis):
    aylis.on("frmIBGE.form_show")(lambda event: ibge.form_show(aylis, components.frmIBGE))
    aylis.on("frmIBGE.estado_mudou")(lambda event: ibge.estado_mudou(aylis, components.frmIBGE.sender_as_client_data_set(event)))
    aylis.on("frmIBGE.municipio_mudou")(lambda event: ibge.municipio_mudou(aylis, components.frmIBGE.sender_as_client_data_set(event)))
