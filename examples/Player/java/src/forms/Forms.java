// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms;

import dev.aylis.Aylis;

public final class Forms {
    /** Registra as classes dos forms e os handlers de todos. */
    public static void register(Aylis aylis) {
        Components.registerClasses();
        aylis.on("frmPlayer.abrir_click", ev -> FrmPlayer.abrir_click(aylis, ev));
        aylis.on("frmPlayer.limpar_fila_click", ev -> FrmPlayer.limpar_fila_click(aylis, ev));
        aylis.on("frmPlayer.sair_click", ev -> FrmPlayer.sair_click(aylis, ev));
        aylis.on("frmPlayer.reproduzir_click", ev -> FrmPlayer.reproduzir_click(aylis, ev));
        aylis.on("frmPlayer.parar_click", ev -> FrmPlayer.parar_click(aylis, ev));
        aylis.on("frmPlayer.anterior_click", ev -> FrmPlayer.anterior_click(aylis, ev));
        aylis.on("frmPlayer.proxima_click", ev -> FrmPlayer.proxima_click(aylis, ev));
        aylis.on("frmPlayer.embaralhar_click", ev -> FrmPlayer.embaralhar_click(aylis, ev));
        aylis.on("frmPlayer.repetir_click", ev -> FrmPlayer.repetir_click(aylis, ev));
        aylis.on("frmPlayer.agora_click", ev -> FrmPlayer.agora_click(aylis, ev));
        aylis.on("frmPlayer.fila_click", ev -> FrmPlayer.fila_click(aylis, ev));
        aylis.on("frmPlayer.posicao_seek", ev -> FrmPlayer.posicao_seek(aylis, ev));
        aylis.on("frmPlayer.mudo_click", ev -> FrmPlayer.mudo_click(aylis, ev));
        aylis.on("frmPlayer.volume_change", ev -> FrmPlayer.volume_change(aylis, ev));
        aylis.on("frmPlayer.video_fim", ev -> FrmPlayer.video_fim(aylis, ev));
        aylis.on("frmPlayer.video_erro", ev -> FrmPlayer.video_erro(aylis, ev));
        aylis.on("frmPlayer.fila_item_click", ev -> FrmPlayer.fila_item_click(aylis, ev));
        aylis.on("frmPlayer.abrir_select", ev -> FrmPlayer.abrir_select(aylis, ev));
        aylis.on("frmPlayer.posicao_timer", ev -> FrmPlayer.posicao_timer(aylis, ev));
    }
}
