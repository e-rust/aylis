// Eventos do form frmCalculadora: o código é seu (o registro dos handlers fica no AllForms.cs
// gerado). Dentro de um handler, o form é a janela do evento.

using System.Text.Json.Nodes;
using AylisClient;
using static App.Forms.Components;

namespace App.Forms;

public static class FrmCalculadora
{
    static readonly Calculadora calc = new();

    /// <summary>Os botões que só funcionam sem erro no visor (os da janela do evento).</summary>
    static AylisUi.Button[] SemErro => [
        frmCalculadora.btnMMais, frmCalculadora.btnMMenos, frmCalculadora.btnMS, frmCalculadora.btnPorcento,
        frmCalculadora.btnInverso, frmCalculadora.btnQuadrado, frmCalculadora.btnRaiz, frmCalculadora.btnDividir,
        frmCalculadora.btnMultiplicar, frmCalculadora.btnSubtrair, frmCalculadora.btnSomar, frmCalculadora.btnSinal,
        frmCalculadora.btnVirgula,
    ];

    /// <summary>Põe o estado da calculadora na tela.</summary>
    static void Atualizar(Aylis aylis)
    {
        string visor = calc.Visor();
        // O número encolhe para caber, como no Windows.
        int n = visor.Length;
        int tamanho = calc.TemErro ? 26 : n <= 11 ? 46 : Math.Max(46 * 11 / n, 22);
        frmCalculadora.lblVisor.Text = visor;
        frmCalculadora.lblVisor.FontSize = tamanho;
        frmCalculadora.lblExpressao.Text = calc.Expressao;
        bool erro = calc.TemErro;
        foreach (var botao in SemErro) botao.Enabled = !erro;
        foreach (var botao in new[] { frmCalculadora.btnMC, frmCalculadora.btnMR }) botao.Enabled = !erro && calc.Memoria.Count > 0;
        frmCalculadora.lvHistorico.Items = calc.Historico.Count == 0
            ? [new ListViewItem("Ainda não há histórico", "")]
            : Enumerable.Reverse(calc.Historico).Select(c => new ListViewItem(Calculadora.Formatar(c.Resultado), c.Expressao)).ToArray();
        frmCalculadora.lvMemoria.Items = calc.Memoria.Count == 0
            ? [new ListViewItem("Não há nada salvo na memória", "")]
            : Enumerable.Reverse(calc.Memoria).Select(m => new ListViewItem(Calculadora.Formatar(m), "")).ToArray();
    }

    /// <summary>Aperta um botão pelo texto dele.</summary>
    static void Apertar(Aylis aylis, string texto)
    {
        if (calc.Tecla(texto)) Atualizar(aylis);
    }

    public static void form_key_down(Aylis aylis, AylisUi.Form sender, global::App.Forms.KeyEvent value)
    {
        // aylis.Reply("key", "") engoliria a tecla.
        string? botao = Calculadora.BotaoDoTeclado(value.Key, value.Ctrl, value.Shift);
        if (botao == "historico") historico_click(aylis, frmCalculadora.btnHistorico);
        else if (botao != null) Apertar(aylis, botao);
    }

    public static void limpar_historico_click(Aylis aylis, AylisUi.Button sender)
    {
        calc.Historico.Clear();
        Atualizar(aylis);
    }

    public static void historico_item_click(Aylis aylis, AylisUi.ListView sender, long value)
    {
        // A lista mostra a conta mais nova em cima.
        if (value >= 0 && value < calc.Historico.Count)
        {
            int i = (int)value;
            calc.Usar(calc.Historico[calc.Historico.Count - 1 - i].Resultado);
            Atualizar(aylis);
        }
    }

    public static void memoria_item_click(Aylis aylis, AylisUi.ListView sender, long value)
    {
        if (value >= 0 && value < calc.Memoria.Count)
        {
            int i = (int)value;
            calc.Usar(calc.Memoria[calc.Memoria.Count - 1 - i]);
            Atualizar(aylis);
        }
    }

    public static void historico_click(Aylis aylis, AylisUi.Button sender)
    {
        frmCalculadora.pnlLateral.Visible = !frmCalculadora.pnlLateral.Visible;
    }

    public static void tecla_click(Aylis aylis, AylisUi.Button sender)
    {
        Apertar(aylis, sender.Text);
    }
}
