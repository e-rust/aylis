// Eventos do form frmPaint: o código é seu (as declarações e o registro dos handlers
// ficam no forms.hpp gerado). Dentro de um handler, o form é a janela do evento.

#include "../aylis.hpp"
#include "forms.hpp"

namespace frm_paint {

// O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
// ferramentas, as cores, o zoom, o arquivo e a barra de status.

// O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%).
static std::string arquivo;
static bool modificado = false;
static int cor_ativa = 1;
static int zoom = 100;

// Um botão de ferramenta e a ferramenta dele.
struct Ferramenta {
    aylis_components::Button botao;
    const char* nome;
};

// Os botões de ferramenta (os do form do evento) e a ferramenta de cada um.
static std::vector<Ferramenta> ferramentas() {
    return {
        {frmPaint.btnSelecionar, "select"}, {frmPaint.btnLapis, "pencil"}, {frmPaint.btnPincel, "brush"}, {frmPaint.btnBorracha, "eraser"},
        {frmPaint.btnBalde, "fill"}, {frmPaint.btnContaGotas, "picker"}, {frmPaint.btnLinha, "line"}, {frmPaint.btnRetangulo, "rectangle"},
        {frmPaint.btnElipse, "ellipse"},
    };
}

static std::string texto(const aylis::Event& ev) {
    return ev.args.kind() == aylis::Json::Kind::String ? ev.args.str() : std::string();
}

// O nome do arquivo sem a pasta.
static std::string nome_do_arquivo(const std::string& caminho) {
    size_t p = caminho.find_last_of("\\/");
    return p == std::string::npos ? caminho : caminho.substr(p + 1);
}

// O título como o do Paint do Windows: "*Sem título - Paint".
static void titulo(aylis::Aylis& app) {
    std::string nome = arquivo.empty() ? "Sem título" : nome_do_arquivo(arquivo);
    frmPaint.set_caption(std::string(modificado ? "*" : "") + nome + " - Paint");
}

// Desfazer, refazer e o tamanho da imagem acompanham o desenho.
static void atualizar_edicao(aylis::Aylis& app) {
    bool desfazer = frmPaint.pcDesenho.can_undo();
    bool refazer = frmPaint.pcDesenho.can_redo();
    frmPaint.mnuDesfazer.set_enabled(desfazer);
    frmPaint.btnDesfazer.set_enabled(desfazer);
    frmPaint.mnuRefazer.set_enabled(refazer);
    frmPaint.btnRefazer.set_enabled(refazer);
    frmPaint.lblTamanho.set_text(frmPaint.pcDesenho.size() + "px");
}

static void marcar_modificado(aylis::Aylis& app) {
    modificado = true;
    titulo(app);
    atualizar_edicao(app);
}

static void aplicar_zoom(aylis::Aylis& app, int z) {
    zoom = z;
    frmPaint.pcDesenho.set_zoom(z / 100.0);
    frmPaint.lblZoom.set_text(std::to_string(z) + "%");
}

// Só o botão da ferramenta escolhida fica marcado.
static void marcar_ferramenta(aylis::Aylis& app, const aylis_components::Button& escolhido) {
    for (const auto& f : ferramentas()) f.botao.set_checked(f.botao.id() == escolhido.id());
}

// A cor que a paleta troca fica com o anel azul, como no Paint.
static void marcar_cor(aylis::Aylis& app, int cor) {
    cor_ativa = cor;
    frmPaint.btnCor1.set_stroke(cor == 1 ? "#0067C0" : "#9A9A9A");
    frmPaint.btnCor1.set_stroke_width(cor == 1 ? 3.0 : 1.0);
    frmPaint.btnCor2.set_stroke(cor == 2 ? "#0067C0" : "#9A9A9A");
    frmPaint.btnCor2.set_stroke_width(cor == 2 ? 3.0 : 1.0);
}

static void erro(aylis::Aylis& app, const std::string& texto) {
    app.send("message_dlg", {{"text", texto}, {"kind", "error"}, {"buttons", aylis::Json::array({aylis::Json("ok")})}});
}

static void salvar_em(aylis::Aylis& app, const std::string& caminho) {
    try {
        frmPaint.pcDesenho.save(caminho);
    } catch (const aylis::Error& e) {
        erro(app, std::string("Não foi possível salvar a imagem.\n") + e.what());
        return;
    }
    arquivo = caminho;
    modificado = false;
    titulo(app);
}

void novo_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.new_(640, 400);
    arquivo.clear();
    modificado = false;
    titulo(app);
    atualizar_edicao(app);
}

void abrir_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.dlgAbrir.execute();
}

void salvar_click(aylis::Aylis& app, const aylis::Event& ev) {
    if (!arquivo.empty()) salvar_em(app, arquivo);
    else salvar_como_click(app, ev);
}

void salvar_como_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.dlgSalvar.execute();
}

void sair_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.close();
}

void desfazer_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.undo();
    marcar_modificado(app);
}

void refazer_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.redo();
    marcar_modificado(app);
}

void recortar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.cut();
}

void copiar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.copy();
}

void colar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
    if (frmPaint.pcDesenho.paste()) marcar_ferramenta(app, frmPaint.btnSelecionar);
}

void excluir_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.delete_();
}

void selecionar_tudo_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.select_all();
    marcar_ferramenta(app, frmPaint.btnSelecionar);
}

void limpar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPaint.pcDesenho.clear();
    marcar_modificado(app);
}

void ampliar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    aplicar_zoom(app, std::min(zoom * 2, 800));
}

void reduzir_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    aplicar_zoom(app, std::max(zoom / 2, 25));
}

void tamanho_real_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    aplicar_zoom(app, 100);
}

void ferramenta_click(aylis::Aylis& app, const aylis::Event& ev) {
    const auto clicado = frmPaint.sender_as_button(ev);
    for (const auto& f : ferramentas()) {
        if (clicado && f.botao.id() == clicado.id()) {
            frmPaint.pcDesenho.set_tool(f.nome);
            marcar_ferramenta(app, f.botao);
            return;
        }
    }
}

void preencher_change(aylis::Aylis& app, const aylis::Event& ev) {
    if (ev.args.kind() == aylis::Json::Kind::Bool) frmPaint.pcDesenho.set_filled(ev.args.boolean());
}

void espessura_change(aylis::Aylis& app, const aylis::Event& ev) {
    // "5 px" → 5
    std::string t = texto(ev);
    if (!t.empty() && t[0] >= '0' && t[0] <= '9') frmPaint.pcDesenho.set_brush_size(std::atoi(t.c_str()));
}

void cor1_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    marcar_cor(app, 1);
}

void cor2_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    marcar_cor(app, 2);
}

void paleta_click(aylis::Aylis& app, const aylis::Event& ev) {
    std::string cor = frmPaint.sender_as_circle(ev).fill();
    if (cor_ativa == 1) {
        frmPaint.pcDesenho.set_color(cor);
        frmPaint.btnCor1.set_fill(cor);
    } else {
        frmPaint.pcDesenho.set_back_color(cor);
        frmPaint.btnCor2.set_fill(cor);
    }
}

void desenho_change(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    marcar_modificado(app);
}

void desenho_pick(aylis::Aylis& app, const aylis::Event& ev) {
    // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
    std::string t = texto(ev);
    if (!t.empty() && t.back() == '2') frmPaint.btnCor2.set_fill(t.substr(0, t.size() - 1));
    else frmPaint.btnCor1.set_fill(t);
}

void desenho_cursor(aylis::Aylis& app, const aylis::Event& ev) {
    std::string t = texto(ev);
    frmPaint.lblPosicao.set_text(t.empty() ? t : t + "px");
}

void abrir_select(aylis::Aylis& app, const aylis::Event& ev) {
    std::string caminho = texto(ev);
    try {
        frmPaint.pcDesenho.open(caminho);
    } catch (const aylis::Error& e) {
        erro(app, std::string("Não foi possível abrir a imagem.\n") + e.what());
        return;
    }
    arquivo = caminho;
    modificado = false;
    titulo(app);
    atualizar_edicao(app);
}

void salvar_select(aylis::Aylis& app, const aylis::Event& ev) {
    salvar_em(app, texto(ev));
}

}  // namespace frm_paint
