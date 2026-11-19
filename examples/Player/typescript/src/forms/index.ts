// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { Aylis } from "../aylis.ts";
import { AylisComponent, Color, Shadow, Easing } from "../aylis.ts";
import * as components from "./components.ts";
void AylisComponent, Color, Shadow, Easing, components;
import * as player from "./player.ts";

/** Registra os handlers de todos os forms. */
export function registerForms(aylis: Aylis) {
    aylis.on("frmPlayer.abrir_click", (ev) => player.abrir_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.limpar_fila_click", (ev) => player.limpar_fila_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.sair_click", (ev) => player.sair_click(aylis, components.frmPlayer.senderAsMenuItem(ev)!));
    aylis.on("frmPlayer.reproduzir_click", (ev) => player.reproduzir_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.parar_click", (ev) => player.parar_click(aylis, components.frmPlayer.senderAsMenuItem(ev)!));
    aylis.on("frmPlayer.anterior_click", (ev) => player.anterior_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.proxima_click", (ev) => player.proxima_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.embaralhar_click", (ev) => player.embaralhar_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.repetir_click", (ev) => player.repetir_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.agora_click", (ev) => player.agora_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.fila_click", (ev) => player.fila_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPlayer.posicao_seek", (ev) => player.posicao_seek(aylis, components.frmPlayer.senderAsSeekBar(ev)!, Number(ev.args ?? 0)));
    aylis.on("frmPlayer.mudo_click", (ev) => player.mudo_click(aylis, components.frmPlayer.senderAsButton(ev)!));
    aylis.on("frmPlayer.volume_change", (ev) => player.volume_change(aylis, components.frmPlayer.senderAsTrackBar(ev)!, Number(ev.args ?? 0)));
    aylis.on("frmPlayer.video_fim", (ev) => player.video_fim(aylis, components.frmPlayer.senderAsVideoPlayer(ev)!));
    aylis.on("frmPlayer.video_erro", (ev) => player.video_erro(aylis, components.frmPlayer.senderAsVideoPlayer(ev)!, String(ev.args ?? "")));
    aylis.on("frmPlayer.fila_item_click", (ev) => player.fila_item_click(aylis, components.frmPlayer.senderAsListView(ev)!, Number(ev.args ?? 0)));
    aylis.on("frmPlayer.abrir_select", (ev) => player.abrir_select(aylis, components.frmPlayer.senderAsOpenDialog(ev)!, components.listOf(ev.args)));
    aylis.on("frmPlayer.posicao_timer", (ev) => player.posicao_timer(aylis, components.frmPlayer.senderAsTimer(ev)!));
}
