// Eventos do form frmPaint: o código é seu (o registro dos handlers fica no AllForms.cs
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
// ferramentas, as cores, o zoom, o arquivo e a barra de status.

using System.Text.Json.Nodes;
using AylisClient;
using static App.Forms.Components;
using Button = App.Forms.AylisUi.Button;

namespace App.Forms;

public static class FrmPaint
{
    // O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%).
    static string arquivo = "";
    static bool modificado;
    static int corAtiva = 1;
    static int zoom = 100;

    /// <summary>Os botões de ferramenta (os do form do evento) e a ferramenta de cada um.</summary>
    static (Button Botao, string Ferramenta)[] Ferramentas() =>
    [
        (frmPaint.btnSelecionar, "select"), (frmPaint.btnLapis, "pencil"), (frmPaint.btnPincel, "brush"), (frmPaint.btnBorracha, "eraser"),
        (frmPaint.btnBalde, "fill"), (frmPaint.btnContaGotas, "picker"), (frmPaint.btnLinha, "line"), (frmPaint.btnRetangulo, "rectangle"),
        (frmPaint.btnElipse, "ellipse"),
    ];

    static string Texto(AylisEvent ev) => ev.Args is JsonValue v && v.TryGetValue<string>(out var s) ? s : "";

    /// <summary>O nome do arquivo sem a pasta.</summary>
    static string NomeDoArquivo(string caminho) => caminho.Split('\\', '/')[^1];

    /// <summary>O título como o do Paint do Windows: "*Sem título - Paint".</summary>
    static void Titulo(Aylis aylis)
    {
        string nome = arquivo == "" ? "Sem título" : NomeDoArquivo(arquivo);
        frmPaint.Caption = $"{(modificado ? "*" : "")}{nome} - Paint";
    }

    /// <summary>Desfazer, refazer e o tamanho da imagem acompanham o desenho.</summary>
    static void AtualizarEdicao(Aylis aylis)
    {
        bool desfazer = frmPaint.pcDesenho.CanUndo();
        bool refazer = frmPaint.pcDesenho.CanRedo();
        frmPaint.mnuDesfazer.Enabled = desfazer;
        frmPaint.btnDesfazer.Enabled = desfazer;
        frmPaint.mnuRefazer.Enabled = refazer;
        frmPaint.btnRefazer.Enabled = refazer;
        frmPaint.lblTamanho.Text = $"{frmPaint.pcDesenho.Size()}px";
    }

    static void MarcarModificado(Aylis aylis)
    {
        modificado = true;
        Titulo(aylis);
        AtualizarEdicao(aylis);
    }

    static void AplicarZoom(Aylis aylis, int z)
    {
        zoom = z;
        frmPaint.pcDesenho.Zoom = z / 100.0;
        frmPaint.lblZoom.Text = $"{z}%";
    }

    static void Erro(Aylis aylis, string texto) =>
        aylis.Send("message_dlg", new JsonObject { ["text"] = texto, ["kind"] = "error", ["buttons"] = new JsonArray("ok") });

    static void SalvarEm(Aylis aylis, string caminho)
    {
        try
        {
            frmPaint.pcDesenho.Save(caminho);
        }
        catch (AylisException e)
        {
            Erro(aylis, $"Não foi possível salvar a imagem.\n{e.Message}");
            return;
        }
        arquivo = caminho;
        modificado = false;
        Titulo(aylis);
    }

    public static void novo_click(Aylis aylis, AylisEvent ev)
    {
        frmPaint.pcDesenho.New(640, 400);
        arquivo = "";
        modificado = false;
        Titulo(aylis);
        AtualizarEdicao(aylis);
    }

    public static void abrir_click(Aylis aylis, AylisEvent ev) => frmPaint.dlgAbrir.Execute();

    public static void salvar_click(Aylis aylis, AylisEvent ev)
    {
        if (arquivo != "") SalvarEm(aylis, arquivo);
        else salvar_como_click(aylis, ev);
    }

    public static void salvar_como_click(Aylis aylis, AylisEvent ev) => frmPaint.dlgSalvar.Execute();

    public static void sair_click(Aylis aylis, AylisEvent ev) => frmPaint.Close();

    public static void desfazer_click(Aylis aylis, AylisEvent ev)
    {
        frmPaint.pcDesenho.Undo();
        MarcarModificado(aylis);
    }

    public static void refazer_click(Aylis aylis, AylisEvent ev)
    {
        frmPaint.pcDesenho.Redo();
        MarcarModificado(aylis);
    }

    public static void limpar_click(Aylis aylis, AylisEvent ev)
    {
        frmPaint.pcDesenho.Clear();
        MarcarModificado(aylis);
    }

    public static void ampliar_click(Aylis aylis, AylisEvent ev) => AplicarZoom(aylis, Math.Min(zoom * 2, 800));

    public static void reduzir_click(Aylis aylis, AylisEvent ev) => AplicarZoom(aylis, Math.Max(zoom / 2, 25));

    public static void tamanho_real_click(Aylis aylis, AylisEvent ev) => AplicarZoom(aylis, 100);

    public static void ferramenta_click(Aylis aylis, AylisEvent ev)
    {
        var clicado = frmPaint.SenderAsButton(ev);
        var item = Ferramentas().FirstOrDefault(f => f.Botao.Id == clicado?.Id);
        if (item.Ferramenta == null) return;
        frmPaint.pcDesenho.Tool = item.Ferramenta;
        MarcarFerramenta(aylis, item.Botao);
    }

    /// <summary>Só o botão da ferramenta escolhida fica marcado.</summary>
    static void MarcarFerramenta(Aylis aylis, Button escolhido)
    {
        foreach (var (botao, _) in Ferramentas()) botao.Checked = botao.Id == escolhido.Id;
    }

    /// <summary>A cor que a paleta troca fica com o anel azul, como no Paint.</summary>
    static void MarcarCor(Aylis aylis, int cor)
    {
        corAtiva = cor;
        foreach (var (circulo, n) in new[] { (frmPaint.btnCor1, 1), (frmPaint.btnCor2, 2) })
        {
            circulo.Stroke = n == cor ? "#0067C0" : "#9A9A9A";
            circulo.StrokeWidth = n == cor ? 3.0 : 1.0;
        }
    }

    public static void espessura_change(Aylis aylis, AylisEvent ev)
    {
        // "5 px" → 5
        if (int.TryParse(Texto(ev).Split(' ')[0], out int n)) frmPaint.pcDesenho.BrushSize = n;
    }

    public static void preencher_change(Aylis aylis, AylisEvent ev)
    {
        if (ev.Args is JsonValue v && v.TryGetValue<bool>(out bool b)) frmPaint.pcDesenho.Filled = b;
    }

    public static void cor1_click(Aylis aylis, AylisEvent ev) => MarcarCor(aylis, 1);

    public static void cor2_click(Aylis aylis, AylisEvent ev) => MarcarCor(aylis, 2);

    public static void paleta_click(Aylis aylis, AylisEvent ev)
    {
        string cor = frmPaint.SenderAsCircle(ev)!.Fill;
        if (corAtiva == 1)
        {
            frmPaint.pcDesenho.Color = cor;
            frmPaint.btnCor1.Fill = cor;
        }
        else
        {
            frmPaint.pcDesenho.BackColor = cor;
            frmPaint.btnCor2.Fill = cor;
        }
    }

    public static void desenho_change(Aylis aylis, AylisEvent ev) => MarcarModificado(aylis);

    public static void desenho_pick(Aylis aylis, AylisEvent ev)
    {
        // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
        string t = Texto(ev);
        if (t.EndsWith('2')) frmPaint.btnCor2.Fill = t[..^1];
        else frmPaint.btnCor1.Fill = t;
    }

    public static void desenho_cursor(Aylis aylis, AylisEvent ev)
    {
        string t = Texto(ev);
        frmPaint.lblPosicao.Text = t == "" ? "" : $"{t}px";
    }

    public static void abrir_select(Aylis aylis, AylisEvent ev)
    {
        string caminho = Texto(ev);
        try
        {
            frmPaint.pcDesenho.Open(caminho);
        }
        catch (AylisException e)
        {
            Erro(aylis, $"Não foi possível abrir a imagem.\n{e.Message}");
            return;
        }
        arquivo = caminho;
        modificado = false;
        Titulo(aylis);
        AtualizarEdicao(aylis);
    }

    public static void salvar_select(Aylis aylis, AylisEvent ev) => SalvarEm(aylis, Texto(ev));

    public static void recortar_click(Aylis aylis, AylisEvent ev) => frmPaint.pcDesenho.Cut();

    public static void copiar_click(Aylis aylis, AylisEvent ev) => frmPaint.pcDesenho.Copy();

    public static void colar_click(Aylis aylis, AylisEvent ev)
    {
        // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
        if (frmPaint.pcDesenho.Paste()) MarcarFerramenta(aylis, frmPaint.btnSelecionar);
    }

    public static void excluir_click(Aylis aylis, AylisEvent ev) => frmPaint.pcDesenho.Delete();

    public static void selecionar_tudo_click(Aylis aylis, AylisEvent ev)
    {
        frmPaint.pcDesenho.SelectAll();
        MarcarFerramenta(aylis, frmPaint.btnSelecionar);
    }
}
