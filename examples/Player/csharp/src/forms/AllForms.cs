// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

using AylisClient;

namespace App.Forms;

public static class AllForms
{
    /// <summary>Registra as classes dos forms e os handlers de todos.</summary>
    public static void Register(Aylis aylis)
    {
        Components.RegisterClasses();
        aylis.On("frmPlayer.abrir_click", ev => FrmPlayer.abrir_click(aylis, ev));
        aylis.On("frmPlayer.limpar_fila_click", ev => FrmPlayer.limpar_fila_click(aylis, ev));
        aylis.On("frmPlayer.sair_click", ev => FrmPlayer.sair_click(aylis, ev));
        aylis.On("frmPlayer.reproduzir_click", ev => FrmPlayer.reproduzir_click(aylis, ev));
        aylis.On("frmPlayer.parar_click", ev => FrmPlayer.parar_click(aylis, ev));
        aylis.On("frmPlayer.anterior_click", ev => FrmPlayer.anterior_click(aylis, ev));
        aylis.On("frmPlayer.proxima_click", ev => FrmPlayer.proxima_click(aylis, ev));
        aylis.On("frmPlayer.embaralhar_click", ev => FrmPlayer.embaralhar_click(aylis, ev));
        aylis.On("frmPlayer.repetir_click", ev => FrmPlayer.repetir_click(aylis, ev));
        aylis.On("frmPlayer.agora_click", ev => FrmPlayer.agora_click(aylis, ev));
        aylis.On("frmPlayer.fila_click", ev => FrmPlayer.fila_click(aylis, ev));
        aylis.On("frmPlayer.posicao_seek", ev => FrmPlayer.posicao_seek(aylis, ev));
        aylis.On("frmPlayer.mudo_click", ev => FrmPlayer.mudo_click(aylis, ev));
        aylis.On("frmPlayer.volume_change", ev => FrmPlayer.volume_change(aylis, ev));
        aylis.On("frmPlayer.video_fim", ev => FrmPlayer.video_fim(aylis, ev));
        aylis.On("frmPlayer.video_erro", ev => FrmPlayer.video_erro(aylis, ev));
        aylis.On("frmPlayer.fila_item_click", ev => FrmPlayer.fila_item_click(aylis, ev));
        aylis.On("frmPlayer.abrir_select", ev => FrmPlayer.abrir_select(aylis, ev));
        aylis.On("frmPlayer.posicao_timer", ev => FrmPlayer.posicao_timer(aylis, ev));
    }
}
