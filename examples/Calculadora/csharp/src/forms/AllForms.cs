// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using AylisClient;

namespace App.Forms;

public static class AllForms
{
    /// <summary>Registra as classes dos forms e os handlers de todos.</summary>
    public static void Register(Aylis aylis)
    {
        Components.RegisterClasses();
        aylis.On("frmCalculadora.form_key_down", ev => FrmCalculadora.form_key_down(aylis, ev));
        aylis.On("frmCalculadora.limpar_historico_click", ev => FrmCalculadora.limpar_historico_click(aylis, ev));
        aylis.On("frmCalculadora.historico_item_click", ev => FrmCalculadora.historico_item_click(aylis, ev));
        aylis.On("frmCalculadora.memoria_item_click", ev => FrmCalculadora.memoria_item_click(aylis, ev));
        aylis.On("frmCalculadora.historico_click", ev => FrmCalculadora.historico_click(aylis, ev));
        aylis.On("frmCalculadora.tecla_click", ev => FrmCalculadora.tecla_click(aylis, ev));
    }
}
