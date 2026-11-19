// Eventos do form frmPlayer: o código é seu (o registro dos handlers fica no AllForms.cs
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
// anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
// toca é o componente VideoPlayer (vpVideo), áudio e vídeo.

using AylisClient;
using static App.Forms.Components;

namespace App.Forms;

public static class FrmPlayer
{
    // O que o form guarda. repetir: 0 não repete, 1 repete a fila, 2 repete a mídia.
    static readonly List<string> fila = new();
    static int atual = -1;
    static bool tocando;
    static bool embaralhar;
    static int repetir;
    static bool mudo;
    static long semente = 12345;

    /// <summary>As extensões de áudio (o resto é vídeo).</summary>
    static readonly string[] Audio = ["mp3", "wav", "m4a", "flac", "ogg", "wma", "aac"];

    /// <summary>O nome do arquivo sem a pasta.</summary>
    static string NomeDoArquivo(string caminho) => caminho.Split('\\', '/')[^1];

    /// <summary>A pasta do arquivo (vazio se não tiver).</summary>
    static string Pasta(string caminho)
    {
        int i = caminho.LastIndexOfAny(['\\', '/']);
        return i >= 0 ? caminho[..i] : "";
    }

    static string SemExtensao(string nome)
    {
        int i = nome.LastIndexOf('.');
        return i > 0 ? nome[..i] : nome;
    }

    static string Tipo(string caminho)
    {
        string nome = NomeDoArquivo(caminho);
        int i = nome.LastIndexOf('.');
        string ext = i >= 0 ? nome[(i + 1)..].ToLowerInvariant() : "";
        return Audio.Contains(ext) ? "Música" : "Vídeo";
    }

    /// <summary>"3:07"</summary>
    static string Tempo(double segundos)
    {
        long s = (long)Math.Floor(Math.Max(segundos, 0.0));
        return $"{s / 60}:{s % 60:00}";
    }

    /// <summary>A fila na lista, com a mídia atual selecionada.</summary>
    static void AtualizarFila(Aylis aylis)
    {
        frmPlayer.lvFila.Items = fila.Select(c => new ListViewItem(NomeDoArquivo(c), Pasta(c))).ToArray();
        frmPlayer.lvFila.ItemIndex = atual;
    }

    /// <summary>O título, o detalhe e o título da janela.</summary>
    static void MostrarAtual(Aylis aylis)
    {
        if (atual < 0)
        {
            frmPlayer.lblTitulo.Text = "Nenhuma mídia";
            frmPlayer.lblDetalhe.Text = "";
            frmPlayer.Caption = "Reprodutor Multimídia";
            return;
        }
        string caminho = fila[atual];
        string titulo = SemExtensao(NomeDoArquivo(caminho));
        frmPlayer.lblTitulo.Text = titulo;
        frmPlayer.lblDetalhe.Text = $"{Tipo(caminho)} · {atual + 1} de {fila.Count}";
        frmPlayer.Caption = $"{titulo} - Reprodutor Multimídia";
    }

    /// <summary>O botão do meio mostra o que ele vai fazer: tocar ou pausar.</summary>
    static void BotaoReproduzir(Aylis aylis)
    {
        frmPlayer.btnReproduzir.Icon = tocando ? "pause" : "play";
        frmPlayer.btnReproduzir.Hint = tocando ? "Pausar (Ctrl+P)" : "Reproduzir (Ctrl+P)";
    }

    /// <summary>Toca a mídia i da fila. Tocar é a última coisa: um erro chega no video_erro.</summary>
    static void Tocar(Aylis aylis, int i)
    {
        atual = i;
        tocando = true;
        frmPlayer.vpVideo.File = fila[i];
        MostrarAtual(aylis);
        AtualizarFila(aylis);
        BotaoReproduzir(aylis);
        frmPlayer.sbPosicao.Position = 0.0;
        frmPlayer.vpVideo.Play();
    }

    /// <summary>A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se
    /// repetir a fila. -1: acabou.</summary>
    static int ProximoIndice()
    {
        int n = fila.Count;
        if (n == 0) return -1;
        if (embaralhar)
        {
            if (n == 1) return 0;
            for (;;)
            {
                semente = (semente * 1103515245 + 12345) % 2147483648;
                int i = (int)(semente % n);
                if (i != atual) return i;
            }
        }
        if (atual + 1 < n) return atual + 1;
        if (repetir == 1) return 0;
        return -1;
    }

    public static void abrir_click(Aylis aylis, AylisComponent sender) => frmPlayer.dlgAbrir.Execute();

    public static void abrir_select(Aylis aylis, AylisUi.OpenDialog sender, string[] value)
    {
        // Os arquivos escolhidos (o evento já separa a lista).
        var novos = value.Where(x => x != "").ToList();
        if (novos.Count == 0) return;
        int primeiro = fila.Count;
        fila.AddRange(novos);
        AtualizarFila(aylis);
        if (atual < 0) Tocar(aylis, primeiro);
        else MostrarAtual(aylis);
    }

    public static void limpar_fila_click(Aylis aylis, AylisComponent sender)
    {
        frmPlayer.vpVideo.Stop();
        fila.Clear();
        atual = -1;
        tocando = false;
        frmPlayer.vpVideo.File = "";
        AtualizarFila(aylis);
        MostrarAtual(aylis);
        BotaoReproduzir(aylis);
    }

    public static void sair_click(Aylis aylis, AylisUi.MenuItem sender) => frmPlayer.Close();

    public static void reproduzir_click(Aylis aylis, AylisComponent sender)
    {
        if (atual < 0)
        {
            if (fila.Count > 0) Tocar(aylis, 0);
            else abrir_click(aylis, sender);
        }
        else if (tocando)
        {
            frmPlayer.vpVideo.Pause();
            tocando = false;
            BotaoReproduzir(aylis);
        }
        else
        {
            tocando = true;
            BotaoReproduzir(aylis);
            frmPlayer.vpVideo.Play();
        }
    }

    public static void parar_click(Aylis aylis, AylisUi.MenuItem sender)
    {
        if (atual < 0) return;
        frmPlayer.vpVideo.Stop();
        tocando = false;
        BotaoReproduzir(aylis);
        frmPlayer.sbPosicao.Position = 0.0;
    }

    public static void anterior_click(Aylis aylis, AylisComponent sender)
    {
        if (atual > 0) Tocar(aylis, atual - 1);
        else if (atual == 0 && repetir == 1) Tocar(aylis, fila.Count - 1);
        else if (atual == 0) frmPlayer.vpVideo.Seek(0.0);
    }

    public static void proxima_click(Aylis aylis, AylisComponent sender)
    {
        int i = ProximoIndice();
        if (i >= 0) Tocar(aylis, i);
    }

    public static void embaralhar_click(Aylis aylis, AylisComponent sender)
    {
        embaralhar = !embaralhar;
        frmPlayer.btnEmbaralhar.Checked = embaralhar;
        frmPlayer.btnEmbaralhar.Hint = embaralhar ? "Embaralhar: ativado" : "Embaralhar: desativado";
        frmPlayer.mnuEmbaralhar.Checked = embaralhar;
    }

    public static void repetir_click(Aylis aylis, AylisComponent sender)
    {
        repetir = (repetir + 1) % 3;
        frmPlayer.btnRepetir.Checked = repetir > 0;
        frmPlayer.btnRepetir.Icon = repetir == 2 ? "repeat_one" : "repeat";
        frmPlayer.btnRepetir.Hint = new[] { "Repetir: desativado", "Repetir: todos", "Repetir: um" }[repetir];
        frmPlayer.mnuRepetir.Checked = repetir > 0;
    }

    /// <summary>Mostra a página "Reproduzindo agora" ou a "Fila de reprodução".</summary>
    static void Pagina(Aylis aylis, bool agora)
    {
        frmPlayer.pnlAgora.Visible = agora;
        frmPlayer.pnlFila.Visible = !agora;
        frmPlayer.btnAgora.Checked = agora;
        frmPlayer.btnFila.Checked = !agora;
    }

    public static void agora_click(Aylis aylis, AylisComponent sender) => Pagina(aylis, true);

    public static void fila_click(Aylis aylis, AylisComponent sender) => Pagina(aylis, false);

    public static void posicao_seek(Aylis aylis, AylisUi.SeekBar sender, double value)
    {
        frmPlayer.vpVideo.Seek(value);
    }

    public static void mudo_click(Aylis aylis, AylisUi.Button sender)
    {
        mudo = !mudo;
        frmPlayer.vpVideo.Muted = mudo;
        frmPlayer.btnMudo.Icon = mudo ? "mute" : "volume";
        frmPlayer.btnMudo.Hint = mudo ? "Com som" : "Sem som";
    }

    public static void volume_change(Aylis aylis, AylisUi.TrackBar sender, double value)
    {
        frmPlayer.vpVideo.Volume = value / 100.0;
    }

    public static void video_fim(Aylis aylis, AylisUi.VideoPlayer sender)
    {
        if (repetir == 2 && atual >= 0)
        {
            Tocar(aylis, atual);
            return;
        }
        int i = ProximoIndice();
        if (i >= 0) Tocar(aylis, i);
        else
        {
            tocando = false;
            BotaoReproduzir(aylis);
        }
    }

    public static void video_erro(Aylis aylis, AylisUi.VideoPlayer sender, string value)
    {
        tocando = false;
        BotaoReproduzir(aylis);
        frmPlayer.lblDetalhe.Text = "Não foi possível reproduzir este arquivo";
    }

    public static void fila_item_click(Aylis aylis, AylisUi.ListView sender, long value)
    {
        if (value >= 0 && value < fila.Count) Tocar(aylis, (int)value);
    }

    public static void posicao_timer(Aylis aylis, AylisUi.Timer sender)
    {
        if (atual < 0) return;
        double posicao = frmPlayer.vpVideo.Position();
        double duracao = frmPlayer.vpVideo.Duration();
        frmPlayer.sbPosicao.Duration = duracao;
        frmPlayer.sbPosicao.Position = posicao;
        frmPlayer.lblTempo.Text = Tempo(posicao);
        frmPlayer.lblDuracao.Text = Tempo(duracao);
    }
}
