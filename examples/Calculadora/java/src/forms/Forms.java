// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms;

import dev.aylis.Aylis;

public final class Forms {
    /** Registra as classes dos forms e os handlers de todos. */
    public static void register(Aylis aylis) {
        Components.registerClasses();
        aylis.on("frmCalculadora.form_key_down", ev -> FrmCalculadora.form_key_down(aylis, ev));
        aylis.on("frmCalculadora.limpar_historico_click", ev -> FrmCalculadora.limpar_historico_click(aylis, ev));
        aylis.on("frmCalculadora.historico_item_click", ev -> FrmCalculadora.historico_item_click(aylis, ev));
        aylis.on("frmCalculadora.memoria_item_click", ev -> FrmCalculadora.memoria_item_click(aylis, ev));
        aylis.on("frmCalculadora.historico_click", ev -> FrmCalculadora.historico_click(aylis, ev));
        aylis.on("frmCalculadora.tecla_click", ev -> FrmCalculadora.tecla_click(aylis, ev));
    }
}
