// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

package forms;

import dev.aylis.Aylis;

public final class Forms {
    /** Registra as classes dos forms e os handlers de todos. */
    public static void register(Aylis aylis) {
        Components.registerClasses();
        aylis.on("frmPaint.novo_click", ev -> FrmPaint.novo_click(aylis, ev));
        aylis.on("frmPaint.abrir_click", ev -> FrmPaint.abrir_click(aylis, ev));
        aylis.on("frmPaint.salvar_click", ev -> FrmPaint.salvar_click(aylis, ev));
        aylis.on("frmPaint.salvar_como_click", ev -> FrmPaint.salvar_como_click(aylis, ev));
        aylis.on("frmPaint.sair_click", ev -> FrmPaint.sair_click(aylis, ev));
        aylis.on("frmPaint.desfazer_click", ev -> FrmPaint.desfazer_click(aylis, ev));
        aylis.on("frmPaint.refazer_click", ev -> FrmPaint.refazer_click(aylis, ev));
        aylis.on("frmPaint.recortar_click", ev -> FrmPaint.recortar_click(aylis, ev));
        aylis.on("frmPaint.copiar_click", ev -> FrmPaint.copiar_click(aylis, ev));
        aylis.on("frmPaint.colar_click", ev -> FrmPaint.colar_click(aylis, ev));
        aylis.on("frmPaint.excluir_click", ev -> FrmPaint.excluir_click(aylis, ev));
        aylis.on("frmPaint.selecionar_tudo_click", ev -> FrmPaint.selecionar_tudo_click(aylis, ev));
        aylis.on("frmPaint.limpar_click", ev -> FrmPaint.limpar_click(aylis, ev));
        aylis.on("frmPaint.ampliar_click", ev -> FrmPaint.ampliar_click(aylis, ev));
        aylis.on("frmPaint.reduzir_click", ev -> FrmPaint.reduzir_click(aylis, ev));
        aylis.on("frmPaint.tamanho_real_click", ev -> FrmPaint.tamanho_real_click(aylis, ev));
        aylis.on("frmPaint.ferramenta_click", ev -> FrmPaint.ferramenta_click(aylis, ev));
        aylis.on("frmPaint.preencher_change", ev -> FrmPaint.preencher_change(aylis, ev));
        aylis.on("frmPaint.espessura_change", ev -> FrmPaint.espessura_change(aylis, ev));
        aylis.on("frmPaint.cor1_click", ev -> FrmPaint.cor1_click(aylis, ev));
        aylis.on("frmPaint.cor2_click", ev -> FrmPaint.cor2_click(aylis, ev));
        aylis.on("frmPaint.paleta_click", ev -> FrmPaint.paleta_click(aylis, ev));
        aylis.on("frmPaint.desenho_change", ev -> FrmPaint.desenho_change(aylis, ev));
        aylis.on("frmPaint.desenho_pick", ev -> FrmPaint.desenho_pick(aylis, ev));
        aylis.on("frmPaint.desenho_cursor", ev -> FrmPaint.desenho_cursor(aylis, ev));
        aylis.on("frmPaint.abrir_select", ev -> FrmPaint.abrir_select(aylis, ev));
        aylis.on("frmPaint.salvar_select", ev -> FrmPaint.salvar_select(aylis, ev));
    }
}
