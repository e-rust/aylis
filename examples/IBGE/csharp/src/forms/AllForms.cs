// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using AylisClient;

namespace App.Forms;

public static class AllForms
{
    /// <summary>Registra as classes dos forms e os handlers de todos.</summary>
    public static void Register(Aylis aylis)
    {
        Components.RegisterClasses();
        aylis.On("frmIBGE.form_show", ev => FrmIBGE.form_show(aylis, ev));
        aylis.On("frmIBGE.estado_mudou", ev => FrmIBGE.estado_mudou(aylis, ev));
        aylis.On("frmIBGE.municipio_mudou", ev => FrmIBGE.municipio_mudou(aylis, ev));
    }
}
