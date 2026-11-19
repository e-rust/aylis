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
    static (Button Botao, PaintTool Ferramenta)[] Ferramentas() =>
    [
        (frmPaint.btnSelecionar, PaintTool.Select), (frmPaint.btnLapis, PaintTool.Pencil), (frmPaint.btnPincel, PaintTool.Brush), (frmPaint.btnBorracha, PaintTool.Eraser),
        (frmPaint.btnBalde, PaintTool.Fill), (frmPaint.btnContaGotas, PaintTool.Picker), (frmPaint.btnLinha, PaintTool.Line), (frmPaint.btnRetangulo, PaintTool.Rectangle),
        (frmPaint.btnElipse, PaintTool.Ellipse),
    ];

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
        var tamanho = frmPaint.pcDesenho.Size();
        frmPaint.lblTamanho.Text = tamanho is null ? "" : $"{tamanho.Width} × {tamanho.Height}px";
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

    public static void novo_click(Aylis aylis, AylisUi.MenuItem sender)
    {
        frmPaint.pcDesenho.New(640, 400);
        arquivo = "";
        modificado = false;
        Titulo(aylis);
        AtualizarEdicao(aylis);
    }

    public static void abrir_click(Aylis aylis, AylisUi.MenuItem sender) => frmPaint.dlgAbrir.Execute();

    public static void salvar_click(Aylis aylis, AylisComponent sender)
    {
        if (arquivo != "") SalvarEm(aylis, arquivo);
        // Sem arquivo ainda: o mesmo do "Salvar como".
        else frmPaint.dlgSalvar.Execute();
    }

    public static void salvar_como_click(Aylis aylis, AylisUi.MenuItem sender) => frmPaint.dlgSalvar.Execute();

    public static void sair_click(Aylis aylis, AylisUi.MenuItem sender) => frmPaint.Close();

    public static void desfazer_click(Aylis aylis, AylisComponent sender)
    {
        frmPaint.pcDesenho.Undo();
        MarcarModificado(aylis);
    }

    public static void refazer_click(Aylis aylis, AylisComponent sender)
    {
        frmPaint.pcDesenho.Redo();
        MarcarModificado(aylis);
    }

    public static void limpar_click(Aylis aylis, AylisUi.MenuItem sender)
    {
        frmPaint.pcDesenho.Clear();
        MarcarModificado(aylis);
    }

    public static void ampliar_click(Aylis aylis, AylisUi.MenuItem sender) => AplicarZoom(aylis, Math.Min(zoom * 2, 800));

    public static void reduzir_click(Aylis aylis, AylisUi.MenuItem sender) => AplicarZoom(aylis, Math.Max(zoom / 2, 25));

    public static void tamanho_real_click(Aylis aylis, AylisUi.MenuItem sender) => AplicarZoom(aylis, 100);

    public static void ferramenta_click(Aylis aylis, AylisUi.Button sender)
    {
        var item = Ferramentas().FirstOrDefault(f => f.Botao.Id == sender.Id);
        if (item.Botao is null) return;
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
            circulo.Stroke = n == cor ? Color.Rgb(0x00, 0x67, 0xC0) : Color.Rgb(0x9A, 0x9A, 0x9A);
            circulo.StrokeWidth = n == cor ? 3.0 : 1.0;
        }
    }

    public static void espessura_change(Aylis aylis, AylisUi.ComboBox sender, string value)
    {
        // "5 px" → 5
        if (int.TryParse(value.Split(' ')[0], out int n)) frmPaint.pcDesenho.BrushSize = n;
    }

    public static void preencher_change(Aylis aylis, AylisUi.CheckBox sender, bool value)
    {
        frmPaint.pcDesenho.Filled = value;
    }

    public static void cor1_click(Aylis aylis, AylisUi.Circle sender) => MarcarCor(aylis, 1);

    public static void cor2_click(Aylis aylis, AylisUi.Circle sender) => MarcarCor(aylis, 2);

    public static void paleta_click(Aylis aylis, AylisUi.Circle sender)
    {
        Color cor = sender.Fill;
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

    public static void desenho_change(Aylis aylis, AylisUi.PaintCanvas sender) => MarcarModificado(aylis);

    public static void desenho_pick(Aylis aylis, AylisUi.PaintCanvas sender, string value)
    {
        // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
        if (value.EndsWith('2')) frmPaint.btnCor2.Fill = Color.Hex(value[..^1]);
        else frmPaint.btnCor1.Fill = Color.Hex(value);
    }

    public static void desenho_cursor(Aylis aylis, AylisUi.PaintCanvas sender, string value)
    {
        frmPaint.lblPosicao.Text = value == "" ? "" : $"{value}px";
    }

    public static void abrir_select(Aylis aylis, AylisUi.OpenDialog sender, string[] value)
    {
        // Um arquivo só: o primeiro da lista.
        string caminho = value.FirstOrDefault() ?? "";
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

    public static void salvar_select(Aylis aylis, AylisUi.SaveDialog sender, string[] value) => SalvarEm(aylis, value.FirstOrDefault() ?? "");

    public static void recortar_click(Aylis aylis, AylisUi.MenuItem sender) => frmPaint.pcDesenho.Cut();

    public static void copiar_click(Aylis aylis, AylisUi.MenuItem sender) => frmPaint.pcDesenho.Copy();

    public static void colar_click(Aylis aylis, AylisUi.MenuItem sender)
    {
        // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
        if (frmPaint.pcDesenho.Paste()) MarcarFerramenta(aylis, frmPaint.btnSelecionar);
    }

    public static void excluir_click(Aylis aylis, AylisUi.MenuItem sender) => frmPaint.pcDesenho.Delete();

    public static void selecionar_tudo_click(Aylis aylis, AylisUi.MenuItem sender)
    {
        frmPaint.pcDesenho.SelectAll();
        MarcarFerramenta(aylis, frmPaint.btnSelecionar);
    }
}
