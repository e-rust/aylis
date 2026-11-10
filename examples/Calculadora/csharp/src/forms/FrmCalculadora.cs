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
        string historico = calc.Historico.Count == 0
            ? "Ainda não há histórico|"
            : string.Join(";", Enumerable.Reverse(calc.Historico).Select(c => $"{Calculadora.Formatar(c.Resultado)}|{c.Expressao}"));
        frmCalculadora.lvHistorico.Items = historico;
        string memoria = calc.Memoria.Count == 0
            ? "Não há nada salvo na memória|"
            : string.Join(";", Enumerable.Reverse(calc.Memoria).Select(m => $"{Calculadora.Formatar(m)}|"));
        frmCalculadora.lvMemoria.Items = memoria;
    }

    /// <summary>Aperta um botão pelo texto dele.</summary>
    static void Apertar(Aylis aylis, string texto)
    {
        if (calc.Tecla(texto)) Atualizar(aylis);
    }

    static int? Indice(AylisEvent ev) => ev.Args is JsonValue v && v.TryGetValue<int>(out var i) ? i : null;

    public static void form_key_down(Aylis aylis, AylisEvent ev)
    {
        string tecla = ev.Args?["key"]?.GetValue<string>() ?? "";
        bool ctrl = ev.Args?["ctrl"]?.GetValue<bool>() ?? false;
        bool shift = ev.Args?["shift"]?.GetValue<bool>() ?? false;
        string? botao = Calculadora.BotaoDoTeclado(tecla, ctrl, shift);
        if (botao == "historico") historico_click(aylis, ev);
        else if (botao != null) Apertar(aylis, botao);
    }

    public static void limpar_historico_click(Aylis aylis, AylisEvent ev)
    {
        calc.Historico.Clear();
        Atualizar(aylis);
    }

    public static void historico_item_click(Aylis aylis, AylisEvent ev)
    {
        // A lista mostra a conta mais nova em cima.
        if (Indice(ev) is int i && i >= 0 && i < calc.Historico.Count)
        {
            calc.Usar(calc.Historico[calc.Historico.Count - 1 - i].Resultado);
            Atualizar(aylis);
        }
    }

    public static void memoria_item_click(Aylis aylis, AylisEvent ev)
    {
        if (Indice(ev) is int i && i >= 0 && i < calc.Memoria.Count)
        {
            calc.Usar(calc.Memoria[calc.Memoria.Count - 1 - i]);
            Atualizar(aylis);
        }
    }

    public static void historico_click(Aylis aylis, AylisEvent ev)
    {
        frmCalculadora.pnlLateral.Visible = !frmCalculadora.pnlLateral.Visible;
    }

    public static void tecla_click(Aylis aylis, AylisEvent ev)
    {
        Apertar(aylis, frmCalculadora.SenderAsButton(ev)!.Text);
    }
}
