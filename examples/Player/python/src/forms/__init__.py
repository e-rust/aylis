"""Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

from . import components  # noqa: F401
from . import player
from .components import *  # noqa: F401,F403
from .components import _enum, _date, _time, _datetime, _list, _numbers, _colors, _date_range, _record  # noqa: F401


def register(aylis):
    aylis.on("frmPlayer.abrir_click")(lambda event: player.abrir_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.limpar_fila_click")(lambda event: player.limpar_fila_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.sair_click")(lambda event: player.sair_click(aylis, components.frmPlayer.sender_as_menu_item(event)))
    aylis.on("frmPlayer.reproduzir_click")(lambda event: player.reproduzir_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.parar_click")(lambda event: player.parar_click(aylis, components.frmPlayer.sender_as_menu_item(event)))
    aylis.on("frmPlayer.anterior_click")(lambda event: player.anterior_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.proxima_click")(lambda event: player.proxima_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.embaralhar_click")(lambda event: player.embaralhar_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.repetir_click")(lambda event: player.repetir_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.agora_click")(lambda event: player.agora_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.fila_click")(lambda event: player.fila_click(aylis, AylisComponent(aylis, event["sender"]["id"], event["sender"]["name"])))
    aylis.on("frmPlayer.posicao_seek")(lambda event: player.posicao_seek(aylis, components.frmPlayer.sender_as_seek_bar(event), event.get("args")))
    aylis.on("frmPlayer.mudo_click")(lambda event: player.mudo_click(aylis, components.frmPlayer.sender_as_button(event)))
    aylis.on("frmPlayer.volume_change")(lambda event: player.volume_change(aylis, components.frmPlayer.sender_as_track_bar(event), event.get("args")))
    aylis.on("frmPlayer.video_fim")(lambda event: player.video_fim(aylis, components.frmPlayer.sender_as_video_player(event)))
    aylis.on("frmPlayer.video_erro")(lambda event: player.video_erro(aylis, components.frmPlayer.sender_as_video_player(event), event.get("args")))
    aylis.on("frmPlayer.fila_item_click")(lambda event: player.fila_item_click(aylis, components.frmPlayer.sender_as_list_view(event), event.get("args")))
    aylis.on("frmPlayer.abrir_select")(lambda event: player.abrir_select(aylis, components.frmPlayer.sender_as_open_dialog(event), _list(event.get("args"))))
    aylis.on("frmPlayer.posicao_timer")(lambda event: player.posicao_timer(aylis, components.frmPlayer.sender_as_timer(event)))
