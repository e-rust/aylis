// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { Aylis } from "../aylis.ts";
import "./components.ts";
import * as player from "./player.ts";

/** Registra os handlers de todos os forms. */
export function registerForms(aylis: Aylis) {
    aylis.on("frmPlayer.abrir_click", (ev) => player.abrir_click(aylis, ev));
    aylis.on("frmPlayer.limpar_fila_click", (ev) => player.limpar_fila_click(aylis, ev));
    aylis.on("frmPlayer.sair_click", (ev) => player.sair_click(aylis, ev));
    aylis.on("frmPlayer.reproduzir_click", (ev) => player.reproduzir_click(aylis, ev));
    aylis.on("frmPlayer.parar_click", (ev) => player.parar_click(aylis, ev));
    aylis.on("frmPlayer.anterior_click", (ev) => player.anterior_click(aylis, ev));
    aylis.on("frmPlayer.proxima_click", (ev) => player.proxima_click(aylis, ev));
    aylis.on("frmPlayer.embaralhar_click", (ev) => player.embaralhar_click(aylis, ev));
    aylis.on("frmPlayer.repetir_click", (ev) => player.repetir_click(aylis, ev));
    aylis.on("frmPlayer.agora_click", (ev) => player.agora_click(aylis, ev));
    aylis.on("frmPlayer.fila_click", (ev) => player.fila_click(aylis, ev));
    aylis.on("frmPlayer.posicao_seek", (ev) => player.posicao_seek(aylis, ev));
    aylis.on("frmPlayer.mudo_click", (ev) => player.mudo_click(aylis, ev));
    aylis.on("frmPlayer.volume_change", (ev) => player.volume_change(aylis, ev));
    aylis.on("frmPlayer.video_fim", (ev) => player.video_fim(aylis, ev));
    aylis.on("frmPlayer.video_erro", (ev) => player.video_erro(aylis, ev));
    aylis.on("frmPlayer.fila_item_click", (ev) => player.fila_item_click(aylis, ev));
    aylis.on("frmPlayer.abrir_select", (ev) => player.abrir_select(aylis, ev));
    aylis.on("frmPlayer.posicao_timer", (ev) => player.posicao_timer(aylis, ev));
}
