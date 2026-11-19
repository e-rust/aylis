// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using AylisClient;

namespace App.Forms;

public static class AllForms
{
    /// <summary>Registra as classes dos forms e os handlers de todos.</summary>
    public static void Register(Aylis aylis)
    {
        Components.RegisterClasses();
        aylis.On("frmPaint.novo_click", ev => FrmPaint.novo_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.abrir_click", ev => FrmPaint.abrir_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.salvar_click", ev => FrmPaint.salvar_click(aylis, new AylisComponent(aylis, ev.SenderId, ev.SenderName)));
        aylis.On("frmPaint.salvar_como_click", ev => FrmPaint.salvar_como_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.sair_click", ev => FrmPaint.sair_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.desfazer_click", ev => FrmPaint.desfazer_click(aylis, new AylisComponent(aylis, ev.SenderId, ev.SenderName)));
        aylis.On("frmPaint.refazer_click", ev => FrmPaint.refazer_click(aylis, new AylisComponent(aylis, ev.SenderId, ev.SenderName)));
        aylis.On("frmPaint.recortar_click", ev => FrmPaint.recortar_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.copiar_click", ev => FrmPaint.copiar_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.colar_click", ev => FrmPaint.colar_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.excluir_click", ev => FrmPaint.excluir_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.selecionar_tudo_click", ev => FrmPaint.selecionar_tudo_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.limpar_click", ev => FrmPaint.limpar_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.ampliar_click", ev => FrmPaint.ampliar_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.reduzir_click", ev => FrmPaint.reduzir_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.tamanho_real_click", ev => FrmPaint.tamanho_real_click(aylis, Components.frmPaint.SenderAsMenuItem(ev)!));
        aylis.On("frmPaint.ferramenta_click", ev => FrmPaint.ferramenta_click(aylis, Components.frmPaint.SenderAsButton(ev)!));
        aylis.On("frmPaint.preencher_change", ev => FrmPaint.preencher_change(aylis, Components.frmPaint.SenderAsCheckBox(ev)!, Components.Bool(ev.Args)));
        aylis.On("frmPaint.espessura_change", ev => FrmPaint.espessura_change(aylis, Components.frmPaint.SenderAsComboBox(ev)!, Components.Text(ev.Args)));
        aylis.On("frmPaint.cor1_click", ev => FrmPaint.cor1_click(aylis, Components.frmPaint.SenderAsCircle(ev)!));
        aylis.On("frmPaint.cor2_click", ev => FrmPaint.cor2_click(aylis, Components.frmPaint.SenderAsCircle(ev)!));
        aylis.On("frmPaint.paleta_click", ev => FrmPaint.paleta_click(aylis, Components.frmPaint.SenderAsCircle(ev)!));
        aylis.On("frmPaint.desenho_change", ev => FrmPaint.desenho_change(aylis, Components.frmPaint.SenderAsPaintCanvas(ev)!));
        aylis.On("frmPaint.desenho_pick", ev => FrmPaint.desenho_pick(aylis, Components.frmPaint.SenderAsPaintCanvas(ev)!, Components.Text(ev.Args)));
        aylis.On("frmPaint.desenho_cursor", ev => FrmPaint.desenho_cursor(aylis, Components.frmPaint.SenderAsPaintCanvas(ev)!, Components.Text(ev.Args)));
        aylis.On("frmPaint.abrir_select", ev => FrmPaint.abrir_select(aylis, Components.frmPaint.SenderAsOpenDialog(ev)!, Components.ListOf(ev.Args)));
        aylis.On("frmPaint.salvar_select", ev => FrmPaint.salvar_select(aylis, Components.frmPaint.SenderAsSaveDialog(ev)!, Components.ListOf(ev.Args)));
    }
}
