// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms;

import dev.aylis.Aylis;

public final class Forms {
    /** Registra as classes dos forms e os handlers de todos. */
    public static void register(Aylis aylis) {
        Components.registerClasses();
        aylis.on("frmIBGE.form_show", ev -> FrmIBGE.form_show(aylis, ev));
        aylis.on("frmIBGE.estado_mudou", ev -> FrmIBGE.estado_mudou(aylis, ev));
        aylis.on("frmIBGE.municipio_mudou", ev -> FrmIBGE.municipio_mudou(aylis, ev));
    }
}
