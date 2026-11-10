"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import ibge


def register(aylis):
    aylis.on("frmIBGE.form_show")(lambda event: ibge.form_show(aylis, event))
    aylis.on("frmIBGE.estado_mudou")(lambda event: ibge.estado_mudou(aylis, event))
    aylis.on("frmIBGE.municipio_mudou")(lambda event: ibge.municipio_mudou(aylis, event))
