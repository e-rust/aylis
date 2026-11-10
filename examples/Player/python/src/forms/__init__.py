"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import player


def register(aylis):
    aylis.on("frmPlayer.abrir_click")(lambda event: player.abrir_click(aylis, event))
    aylis.on("frmPlayer.limpar_fila_click")(lambda event: player.limpar_fila_click(aylis, event))
    aylis.on("frmPlayer.sair_click")(lambda event: player.sair_click(aylis, event))
    aylis.on("frmPlayer.reproduzir_click")(lambda event: player.reproduzir_click(aylis, event))
    aylis.on("frmPlayer.parar_click")(lambda event: player.parar_click(aylis, event))
    aylis.on("frmPlayer.anterior_click")(lambda event: player.anterior_click(aylis, event))
    aylis.on("frmPlayer.proxima_click")(lambda event: player.proxima_click(aylis, event))
    aylis.on("frmPlayer.embaralhar_click")(lambda event: player.embaralhar_click(aylis, event))
    aylis.on("frmPlayer.repetir_click")(lambda event: player.repetir_click(aylis, event))
    aylis.on("frmPlayer.agora_click")(lambda event: player.agora_click(aylis, event))
    aylis.on("frmPlayer.fila_click")(lambda event: player.fila_click(aylis, event))
    aylis.on("frmPlayer.posicao_seek")(lambda event: player.posicao_seek(aylis, event))
    aylis.on("frmPlayer.mudo_click")(lambda event: player.mudo_click(aylis, event))
    aylis.on("frmPlayer.volume_change")(lambda event: player.volume_change(aylis, event))
    aylis.on("frmPlayer.video_fim")(lambda event: player.video_fim(aylis, event))
    aylis.on("frmPlayer.video_erro")(lambda event: player.video_erro(aylis, event))
    aylis.on("frmPlayer.fila_item_click")(lambda event: player.fila_item_click(aylis, event))
    aylis.on("frmPlayer.abrir_select")(lambda event: player.abrir_select(aylis, event))
    aylis.on("frmPlayer.posicao_timer")(lambda event: player.posicao_timer(aylis, event))
