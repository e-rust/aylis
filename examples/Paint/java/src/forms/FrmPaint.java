// Eventos do form frmPaint: o código é seu (o registro dos handlers fica no Forms.java
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
// ferramentas, as cores, o zoom, o arquivo e a barra de status.

package forms;

import dev.aylis.Aylis;

import java.util.List;
import static forms.Components.frmPaint;

public final class FrmPaint {
    // O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%).
    static String arquivo = "";
    static boolean modificado;
    static int corAtiva = 1;
    static int zoom = 100;

    /** Um botão de ferramenta e a ferramenta dele. */
    record Ferramenta(Components.Button botao, String nome) {}

    /** Os botões de ferramenta (os do form do evento) e a ferramenta de cada um. */
    static List<Ferramenta> ferramentas() {
        return List.of(
            new Ferramenta(frmPaint.btnSelecionar, "select"), new Ferramenta(frmPaint.btnLapis, "pencil"), new Ferramenta(frmPaint.btnPincel, "brush"),
            new Ferramenta(frmPaint.btnBorracha, "eraser"), new Ferramenta(frmPaint.btnBalde, "fill"), new Ferramenta(frmPaint.btnContaGotas, "picker"),
            new Ferramenta(frmPaint.btnLinha, "line"), new Ferramenta(frmPaint.btnRetangulo, "rectangle"), new Ferramenta(frmPaint.btnElipse, "ellipse"));
    }

    static String texto(Aylis.Event ev) {
        return ev.args() instanceof String s ? s : "";
    }

    /** O nome do arquivo sem a pasta. */
    static String nomeDoArquivo(String caminho) {
        String[] partes = caminho.split("[\\\\/]");
        return partes[partes.length - 1];
    }

    /** O título como o do Paint do Windows: "*Sem título - Paint". */
    static void titulo(Aylis aylis) {
        String nome = arquivo.isEmpty() ? "Sem título" : nomeDoArquivo(arquivo);
        frmPaint.setCaption((modificado ? "*" : "") + nome + " - Paint");
    }

    /** Desfazer, refazer e o tamanho da imagem acompanham o desenho. */
    static void atualizarEdicao(Aylis aylis) {
        boolean desfazer = frmPaint.pcDesenho.canUndo();
        boolean refazer = frmPaint.pcDesenho.canRedo();
        frmPaint.mnuDesfazer.setEnabled(desfazer);
        frmPaint.btnDesfazer.setEnabled(desfazer);
        frmPaint.mnuRefazer.setEnabled(refazer);
        frmPaint.btnRefazer.setEnabled(refazer);
        frmPaint.lblTamanho.setText(frmPaint.pcDesenho.size() + "px");
    }

    static void marcarModificado(Aylis aylis) {
        modificado = true;
        titulo(aylis);
        atualizarEdicao(aylis);
    }

    static void aplicarZoom(Aylis aylis, int z) {
        zoom = z;
        frmPaint.pcDesenho.setZoom(z / 100.0);
        frmPaint.lblZoom.setText(z + "%");
    }

    /** Só o botão da ferramenta escolhida fica marcado. */
    static void marcarFerramenta(Aylis aylis, Components.Button escolhido) {
        for (Ferramenta f : ferramentas()) f.botao().setChecked(f.botao().id == escolhido.id);
    }

    /** A cor que a paleta troca fica com o anel azul, como no Paint. */
    static void marcarCor(Aylis aylis, int cor) {
        corAtiva = cor;
        frmPaint.btnCor1.setStroke(cor == 1 ? "#0067C0" : "#9A9A9A");
        frmPaint.btnCor1.setStrokeWidth(cor == 1 ? 3.0 : 1.0);
        frmPaint.btnCor2.setStroke(cor == 2 ? "#0067C0" : "#9A9A9A");
        frmPaint.btnCor2.setStrokeWidth(cor == 2 ? 3.0 : 1.0);
    }

    static void erro(Aylis aylis, String texto) {
        aylis.send("message_dlg", Aylis.obj("text", texto, "kind", "error", "buttons", List.of("ok")));
    }

    static void salvarEm(Aylis aylis, String caminho) {
        try {
            frmPaint.pcDesenho.save(caminho);
        } catch (Aylis.AylisException e) {
            erro(aylis, "Não foi possível salvar a imagem.\n" + e.getMessage());
            return;
        }
        arquivo = caminho;
        modificado = false;
        titulo(aylis);
    }

    public static void novo_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.new_(640, 400);
        arquivo = "";
        modificado = false;
        titulo(aylis);
        atualizarEdicao(aylis);
    }

    public static void abrir_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.dlgAbrir.execute();
    }

    public static void salvar_click(Aylis aylis, Aylis.Event ev) {
        if (!arquivo.isEmpty()) salvarEm(aylis, arquivo);
        else salvar_como_click(aylis, ev);
    }

    public static void salvar_como_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.dlgSalvar.execute();
    }

    public static void sair_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.close();
    }

    public static void desfazer_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.undo();
        marcarModificado(aylis);
    }

    public static void refazer_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.redo();
        marcarModificado(aylis);
    }

    public static void recortar_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.cut();
    }

    public static void copiar_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.copy();
    }

    public static void colar_click(Aylis aylis, Aylis.Event ev) {
        // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
        if (frmPaint.pcDesenho.paste()) marcarFerramenta(aylis, frmPaint.btnSelecionar);
    }

    public static void excluir_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.delete();
    }

    public static void selecionar_tudo_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.selectAll();
        marcarFerramenta(aylis, frmPaint.btnSelecionar);
    }

    public static void limpar_click(Aylis aylis, Aylis.Event ev) {
        frmPaint.pcDesenho.clear();
        marcarModificado(aylis);
    }

    public static void ampliar_click(Aylis aylis, Aylis.Event ev) {
        aplicarZoom(aylis, Math.min(zoom * 2, 800));
    }

    public static void reduzir_click(Aylis aylis, Aylis.Event ev) {
        aplicarZoom(aylis, Math.max(zoom / 2, 25));
    }

    public static void tamanho_real_click(Aylis aylis, Aylis.Event ev) {
        aplicarZoom(aylis, 100);
    }

    public static void ferramenta_click(Aylis aylis, Aylis.Event ev) {
        Components.Button clicado = frmPaint.senderAsButton(ev);
        for (Ferramenta f : ferramentas()) {
            if (clicado != null && f.botao().id == clicado.id) {
                frmPaint.pcDesenho.setTool(f.nome());
                marcarFerramenta(aylis, f.botao());
                return;
            }
        }
    }

    public static void preencher_change(Aylis aylis, Aylis.Event ev) {
        if (ev.args() instanceof Boolean b) frmPaint.pcDesenho.setFilled(b);
    }

    public static void espessura_change(Aylis aylis, Aylis.Event ev) {
        // "5 px" → 5
        try {
            frmPaint.pcDesenho.setBrushSize(Integer.parseInt(texto(ev).split(" ")[0]));
        } catch (NumberFormatException e) {
            // texto sem número: deixa como está
        }
    }

    public static void cor1_click(Aylis aylis, Aylis.Event ev) {
        marcarCor(aylis, 1);
    }

    public static void cor2_click(Aylis aylis, Aylis.Event ev) {
        marcarCor(aylis, 2);
    }

    public static void paleta_click(Aylis aylis, Aylis.Event ev) {
        String cor = frmPaint.senderAsCircle(ev).getFill();
        if (corAtiva == 1) {
            frmPaint.pcDesenho.setColor(cor);
            frmPaint.btnCor1.setFill(cor);
        } else {
            frmPaint.pcDesenho.setBackColor(cor);
            frmPaint.btnCor2.setFill(cor);
        }
    }

    public static void desenho_change(Aylis aylis, Aylis.Event ev) {
        marcarModificado(aylis);
    }

    public static void desenho_pick(Aylis aylis, Aylis.Event ev) {
        // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
        String t = texto(ev);
        if (t.endsWith("2")) frmPaint.btnCor2.setFill(t.substring(0, t.length() - 1));
        else frmPaint.btnCor1.setFill(t);
    }

    public static void desenho_cursor(Aylis aylis, Aylis.Event ev) {
        String t = texto(ev);
        frmPaint.lblPosicao.setText(t.isEmpty() ? "" : t + "px");
    }

    public static void abrir_select(Aylis aylis, Aylis.Event ev) {
        String caminho = texto(ev);
        try {
            frmPaint.pcDesenho.open(caminho);
        } catch (Aylis.AylisException e) {
            erro(aylis, "Não foi possível abrir a imagem.\n" + e.getMessage());
            return;
        }
        arquivo = caminho;
        modificado = false;
        titulo(aylis);
        atualizarEdicao(aylis);
    }

    public static void salvar_select(Aylis aylis, Aylis.Event ev) {
        salvarEm(aylis, texto(ev));
    }
}
