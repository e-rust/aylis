// Eventos do form frmPlayer: o código é seu (o registro dos handlers fica no Forms.java
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
// anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
// toca é o componente VideoPlayer (vpVideo), áudio e vídeo.

package forms;

import dev.aylis.Aylis;

import java.util.ArrayList;
import java.util.List;
import static forms.Components.frmPlayer;

public final class FrmPlayer {
    // O que o form guarda. repetir: 0 não repete, 1 repete a fila, 2 repete a mídia.
    static final List<String> fila = new ArrayList<>();
    static int atual = -1;
    static boolean tocando;
    static boolean embaralhar;
    static int repetir;
    static boolean mudo;
    static long semente = 12345;

    /** As extensões de áudio (o resto é vídeo). */
    static final List<String> AUDIO = List.of("mp3", "wav", "m4a", "flac", "ogg", "wma", "aac");

    /** O nome do arquivo sem a pasta. */
    static String nomeDoArquivo(String caminho) {
        int i = Math.max(caminho.lastIndexOf('\\'), caminho.lastIndexOf('/'));
        return caminho.substring(i + 1);
    }

    /** A pasta do arquivo (vazio se não tiver). */
    static String pasta(String caminho) {
        int i = Math.max(caminho.lastIndexOf('\\'), caminho.lastIndexOf('/'));
        return i >= 0 ? caminho.substring(0, i) : "";
    }

    static String semExtensao(String nome) {
        int i = nome.lastIndexOf('.');
        return i > 0 ? nome.substring(0, i) : nome;
    }

    static String tipo(String caminho) {
        String nome = nomeDoArquivo(caminho);
        int i = nome.lastIndexOf('.');
        String ext = i >= 0 ? nome.substring(i + 1).toLowerCase() : "";
        return AUDIO.contains(ext) ? "Música" : "Vídeo";
    }

    /** "3:07" */
    static String tempo(double segundos) {
        long s = (long) Math.floor(Math.max(segundos, 0.0));
        return (s / 60) + ":" + String.format("%02d", s % 60);
    }

    /** A fila na lista, com a mídia atual selecionada. */
    static void atualizarFila(Aylis aylis) {
        List<String> itens = new ArrayList<>();
        for (String c : fila) itens.add(nomeDoArquivo(c) + "|" + pasta(c));
        frmPlayer.lvFila.setItems(String.join(";", itens));
        frmPlayer.lvFila.setItemIndex(atual);
    }

    /** O título, o detalhe e o título da janela. */
    static void mostrarAtual(Aylis aylis) {
        if (atual < 0) {
            frmPlayer.lblTitulo.setText("Nenhuma mídia");
            frmPlayer.lblDetalhe.setText("");
            frmPlayer.setCaption("Reprodutor Multimídia");
            return;
        }
        String caminho = fila.get(atual);
        String titulo = semExtensao(nomeDoArquivo(caminho));
        frmPlayer.lblTitulo.setText(titulo);
        frmPlayer.lblDetalhe.setText(tipo(caminho) + " · " + (atual + 1) + " de " + fila.size());
        frmPlayer.setCaption(titulo + " - Reprodutor Multimídia");
    }

    /** O botão do meio mostra o que ele vai fazer: tocar ou pausar. */
    static void botaoReproduzir(Aylis aylis) {
        frmPlayer.btnReproduzir.setIcon(tocando ? "pause" : "play");
        frmPlayer.btnReproduzir.setHint(tocando ? "Pausar (Ctrl+P)" : "Reproduzir (Ctrl+P)");
    }

    /** Toca a mídia i da fila. Tocar é a última coisa: um erro chega no video_erro. */
    static void tocar(Aylis aylis, int i) {
        atual = i;
        tocando = true;
        frmPlayer.vpVideo.setFile(fila.get(i));
        mostrarAtual(aylis);
        atualizarFila(aylis);
        botaoReproduzir(aylis);
        frmPlayer.sbPosicao.setPosition(0.0);
        frmPlayer.vpVideo.play();
    }

    /** A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se repetir a
     * fila. -1: acabou. */
    static int proximoIndice() {
        int n = fila.size();
        if (n == 0) return -1;
        if (embaralhar) {
            if (n == 1) return 0;
            for (;;) {
                semente = (semente * 1103515245L + 12345L) % 2147483648L;
                int i = (int) (semente % n);
                if (i != atual) return i;
            }
        }
        if (atual + 1 < n) return atual + 1;
        if (repetir == 1) return 0;
        return -1;
    }

    public static void abrir_click(Aylis aylis, Aylis.Event ev) {
        frmPlayer.dlgAbrir.execute();
    }

    public static void abrir_select(Aylis aylis, Aylis.Event ev) {
        // Os arquivos escolhidos vêm separados por ";".
        String texto = ev.args() instanceof String s ? s : "";
        List<String> novos = new ArrayList<>();
        for (String p : texto.split(";")) if (!p.isEmpty()) novos.add(p);
        if (novos.isEmpty()) return;
        int primeiro = fila.size();
        fila.addAll(novos);
        atualizarFila(aylis);
        if (atual < 0) tocar(aylis, primeiro);
        else mostrarAtual(aylis);
    }

    public static void limpar_fila_click(Aylis aylis, Aylis.Event ev) {
        frmPlayer.vpVideo.stop();
        fila.clear();
        atual = -1;
        tocando = false;
        frmPlayer.vpVideo.setFile("");
        atualizarFila(aylis);
        mostrarAtual(aylis);
        botaoReproduzir(aylis);
    }

    public static void sair_click(Aylis aylis, Aylis.Event ev) {
        frmPlayer.close();
    }

    public static void reproduzir_click(Aylis aylis, Aylis.Event ev) {
        if (atual < 0) {
            if (!fila.isEmpty()) tocar(aylis, 0);
            else abrir_click(aylis, ev);
        } else if (tocando) {
            frmPlayer.vpVideo.pause();
            tocando = false;
            botaoReproduzir(aylis);
        } else {
            tocando = true;
            botaoReproduzir(aylis);
            frmPlayer.vpVideo.play();
        }
    }

    public static void parar_click(Aylis aylis, Aylis.Event ev) {
        if (atual < 0) return;
        frmPlayer.vpVideo.stop();
        tocando = false;
        botaoReproduzir(aylis);
        frmPlayer.sbPosicao.setPosition(0.0);
    }

    public static void anterior_click(Aylis aylis, Aylis.Event ev) {
        if (atual > 0) tocar(aylis, atual - 1);
        else if (atual == 0 && repetir == 1) tocar(aylis, fila.size() - 1);
        else if (atual == 0) frmPlayer.vpVideo.seek(0.0);
    }

    public static void proxima_click(Aylis aylis, Aylis.Event ev) {
        int i = proximoIndice();
        if (i >= 0) tocar(aylis, i);
    }

    public static void embaralhar_click(Aylis aylis, Aylis.Event ev) {
        embaralhar = !embaralhar;
        frmPlayer.btnEmbaralhar.setChecked(embaralhar);
        frmPlayer.btnEmbaralhar.setHint(embaralhar ? "Embaralhar: ativado" : "Embaralhar: desativado");
        frmPlayer.mnuEmbaralhar.setChecked(embaralhar);
    }

    public static void repetir_click(Aylis aylis, Aylis.Event ev) {
        repetir = (repetir + 1) % 3;
        frmPlayer.btnRepetir.setChecked(repetir > 0);
        frmPlayer.btnRepetir.setIcon(repetir == 2 ? "repeat_one" : "repeat");
        frmPlayer.btnRepetir.setHint(new String[] {"Repetir: desativado", "Repetir: todos", "Repetir: um"}[repetir]);
        frmPlayer.mnuRepetir.setChecked(repetir > 0);
    }

    /** Mostra a página "Reproduzindo agora" ou a "Fila de reprodução". */
    static void pagina(Aylis aylis, boolean agora) {
        frmPlayer.pnlAgora.setVisible(agora);
        frmPlayer.pnlFila.setVisible(!agora);
        frmPlayer.btnAgora.setChecked(agora);
        frmPlayer.btnFila.setChecked(!agora);
    }

    public static void agora_click(Aylis aylis, Aylis.Event ev) {
        pagina(aylis, true);
    }

    public static void fila_click(Aylis aylis, Aylis.Event ev) {
        pagina(aylis, false);
    }

    public static void posicao_seek(Aylis aylis, Aylis.Event ev) {
        if (ev.args() instanceof Number n) frmPlayer.vpVideo.seek(n.doubleValue());
    }

    public static void mudo_click(Aylis aylis, Aylis.Event ev) {
        mudo = !mudo;
        frmPlayer.vpVideo.setMuted(mudo);
        frmPlayer.btnMudo.setIcon(mudo ? "mute" : "volume");
        frmPlayer.btnMudo.setHint(mudo ? "Com som" : "Sem som");
    }

    public static void volume_change(Aylis aylis, Aylis.Event ev) {
        if (ev.args() instanceof Number n) frmPlayer.vpVideo.setVolume(n.doubleValue() / 100.0);
    }

    public static void video_fim(Aylis aylis, Aylis.Event ev) {
        if (repetir == 2 && atual >= 0) {
            tocar(aylis, atual);
            return;
        }
        int i = proximoIndice();
        if (i >= 0) tocar(aylis, i);
        else {
            tocando = false;
            botaoReproduzir(aylis);
        }
    }

    public static void video_erro(Aylis aylis, Aylis.Event ev) {
        tocando = false;
        botaoReproduzir(aylis);
        frmPlayer.lblDetalhe.setText("Não foi possível reproduzir este arquivo");
    }

    public static void fila_item_click(Aylis aylis, Aylis.Event ev) {
        if (ev.args() instanceof Number n) {
            int i = n.intValue();
            if (i >= 0 && i < fila.size()) tocar(aylis, i);
        }
    }

    public static void posicao_timer(Aylis aylis, Aylis.Event ev) {
        if (atual < 0) return;
        double posicao = frmPlayer.vpVideo.position();
        double duracao = frmPlayer.vpVideo.duration();
        frmPlayer.sbPosicao.setDuration(duracao);
        frmPlayer.sbPosicao.setPosition(posicao);
        frmPlayer.lblTempo.setText(tempo(posicao));
        frmPlayer.lblDuracao.setText(tempo(duracao));
    }
}
