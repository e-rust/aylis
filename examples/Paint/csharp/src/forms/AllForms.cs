// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using AylisClient;

namespace App.Forms;

public static class AllForms
{
    /// <summary>Registra as classes dos forms e os handlers de todos.</summary>
    public static void Register(Aylis aylis)
    {
        Components.RegisterClasses();
        aylis.On("frmPaint.novo_click", ev => FrmPaint.novo_click(aylis, ev));
        aylis.On("frmPaint.abrir_click", ev => FrmPaint.abrir_click(aylis, ev));
        aylis.On("frmPaint.salvar_click", ev => FrmPaint.salvar_click(aylis, ev));
        aylis.On("frmPaint.salvar_como_click", ev => FrmPaint.salvar_como_click(aylis, ev));
        aylis.On("frmPaint.sair_click", ev => FrmPaint.sair_click(aylis, ev));
        aylis.On("frmPaint.desfazer_click", ev => FrmPaint.desfazer_click(aylis, ev));
        aylis.On("frmPaint.refazer_click", ev => FrmPaint.refazer_click(aylis, ev));
        aylis.On("frmPaint.recortar_click", ev => FrmPaint.recortar_click(aylis, ev));
        aylis.On("frmPaint.copiar_click", ev => FrmPaint.copiar_click(aylis, ev));
        aylis.On("frmPaint.colar_click", ev => FrmPaint.colar_click(aylis, ev));
        aylis.On("frmPaint.excluir_click", ev => FrmPaint.excluir_click(aylis, ev));
        aylis.On("frmPaint.selecionar_tudo_click", ev => FrmPaint.selecionar_tudo_click(aylis, ev));
        aylis.On("frmPaint.limpar_click", ev => FrmPaint.limpar_click(aylis, ev));
        aylis.On("frmPaint.ampliar_click", ev => FrmPaint.ampliar_click(aylis, ev));
        aylis.On("frmPaint.reduzir_click", ev => FrmPaint.reduzir_click(aylis, ev));
        aylis.On("frmPaint.tamanho_real_click", ev => FrmPaint.tamanho_real_click(aylis, ev));
        aylis.On("frmPaint.ferramenta_click", ev => FrmPaint.ferramenta_click(aylis, ev));
        aylis.On("frmPaint.preencher_change", ev => FrmPaint.preencher_change(aylis, ev));
        aylis.On("frmPaint.espessura_change", ev => FrmPaint.espessura_change(aylis, ev));
        aylis.On("frmPaint.cor1_click", ev => FrmPaint.cor1_click(aylis, ev));
        aylis.On("frmPaint.cor2_click", ev => FrmPaint.cor2_click(aylis, ev));
        aylis.On("frmPaint.paleta_click", ev => FrmPaint.paleta_click(aylis, ev));
        aylis.On("frmPaint.desenho_change", ev => FrmPaint.desenho_change(aylis, ev));
        aylis.On("frmPaint.desenho_pick", ev => FrmPaint.desenho_pick(aylis, ev));
        aylis.On("frmPaint.desenho_cursor", ev => FrmPaint.desenho_cursor(aylis, ev));
        aylis.On("frmPaint.abrir_select", ev => FrmPaint.abrir_select(aylis, ev));
        aylis.On("frmPaint.salvar_select", ev => FrmPaint.salvar_select(aylis, ev));
    }
}
