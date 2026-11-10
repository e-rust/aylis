// Eventos do form frmPlayer: o código é seu (as declarações e o registro dos handlers
// ficam no forms.hpp gerado). Dentro de um handler, o form é a janela do evento.

#include "../aylis.hpp"

#include <algorithm>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <vector>
#include "forms.hpp"

namespace frm_player {

// O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
// anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
// toca é o componente VideoPlayer (vpVideo), áudio e vídeo.

// O que o form guarda. repetir: 0 não repete, 1 repete a fila, 2 repete a mídia.
static std::vector<std::string> fila;
static int atual = -1;
static bool tocando = false;
static bool embaralhar = false;
static int repetir = 0;
static bool mudo = false;
static long long semente = 12345;

// As extensões de áudio (o resto é vídeo).
static const char* AUDIO[] = {"mp3", "wav", "m4a", "flac", "ogg", "wma", "aac"};

static size_t separador(const std::string& caminho) {
    return caminho.find_last_of("\\/");
}

// O nome do arquivo sem a pasta.
static std::string nome_do_arquivo(const std::string& caminho) {
    size_t i = separador(caminho);
    return i == std::string::npos ? caminho : caminho.substr(i + 1);
}

// A pasta do arquivo (vazio se não tiver).
static std::string pasta(const std::string& caminho) {
    size_t i = separador(caminho);
    return i == std::string::npos ? std::string() : caminho.substr(0, i);
}

static std::string sem_extensao(const std::string& nome) {
    size_t i = nome.rfind('.');
    return i != std::string::npos && i > 0 ? nome.substr(0, i) : nome;
}

static std::string tipo(const std::string& caminho) {
    std::string nome = nome_do_arquivo(caminho);
    size_t i = nome.rfind('.');
    std::string ext = i == std::string::npos ? std::string() : nome.substr(i + 1);
    for (char& c : ext) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    for (const char* a : AUDIO) {
        if (ext == a) return "Música";
    }
    return "Vídeo";
}

// "3:07"
static std::string tempo(double segundos) {
    long long s = static_cast<long long>(std::floor(std::max(segundos, 0.0)));
    char buf[32];
    std::snprintf(buf, sizeof buf, "%lld:%02lld", s / 60, s % 60);
    return buf;
}

// A fila na lista, com a mídia atual selecionada.
static void atualizar_fila(aylis::Aylis& app) {
    std::string itens;
    for (size_t i = 0; i < fila.size(); i++) itens += (i ? ";" : "") + nome_do_arquivo(fila[i]) + "|" + pasta(fila[i]);
    frmPlayer.lvFila.set_items(itens);
    frmPlayer.lvFila.set_item_index(atual);
}

// O título, o detalhe e o título da janela.
static void mostrar_atual(aylis::Aylis& app) {
    if (atual < 0) {
        frmPlayer.lblTitulo.set_text("Nenhuma mídia");
        frmPlayer.lblDetalhe.set_text("");
        frmPlayer.set_caption("Reprodutor Multimídia");
        return;
    }
    const std::string& caminho = fila[atual];
    std::string titulo = sem_extensao(nome_do_arquivo(caminho));
    frmPlayer.lblTitulo.set_text(titulo);
    frmPlayer.lblDetalhe.set_text(tipo(caminho) + " · " + std::to_string(atual + 1) + " de " + std::to_string(fila.size()));
    frmPlayer.set_caption(titulo + " - Reprodutor Multimídia");
}

// O botão do meio mostra o que ele vai fazer: tocar ou pausar.
static void botao_reproduzir(aylis::Aylis& app) {
    frmPlayer.btnReproduzir.set_icon(tocando ? "pause" : "play");
    frmPlayer.btnReproduzir.set_hint(tocando ? "Pausar (Ctrl+P)" : "Reproduzir (Ctrl+P)");
}

// Toca a mídia i da fila. Tocar é a última coisa: um erro chega no video_erro.
static void tocar(aylis::Aylis& app, int i) {
    atual = i;
    tocando = true;
    frmPlayer.vpVideo.set_file(fila[i]);
    mostrar_atual(app);
    atualizar_fila(app);
    botao_reproduzir(app);
    frmPlayer.sbPosicao.set_position(0.0);
    frmPlayer.vpVideo.play();
}

// A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se repetir a fila.
// -1: acabou.
static int proximo_indice() {
    int n = static_cast<int>(fila.size());
    if (n == 0) return -1;
    if (embaralhar) {
        if (n == 1) return 0;
        for (;;) {
            semente = (semente * 1103515245LL + 12345LL) % 2147483648LL;
            int i = static_cast<int>(semente % n);
            if (i != atual) return i;
        }
    }
    if (atual + 1 < n) return atual + 1;
    if (repetir == 1) return 0;
    return -1;
}

void abrir_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPlayer.dlgAbrir.execute();
}

void abrir_select(aylis::Aylis& app, const aylis::Event& ev) {
    // Os arquivos escolhidos vêm separados por ";".
    std::string texto = ev.args.kind() == aylis::Json::Kind::String ? ev.args.str() : std::string();
    std::vector<std::string> novos;
    size_t inicio = 0;
    while (inicio <= texto.size()) {
        size_t fim = texto.find(';', inicio);
        if (fim == std::string::npos) fim = texto.size();
        if (fim > inicio) novos.push_back(texto.substr(inicio, fim - inicio));
        inicio = fim + 1;
    }
    if (novos.empty()) return;
    int primeiro = static_cast<int>(fila.size());
    fila.insert(fila.end(), novos.begin(), novos.end());
    atualizar_fila(app);
    if (atual < 0) tocar(app, primeiro);
    else mostrar_atual(app);
}

void limpar_fila_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPlayer.vpVideo.stop();
    fila.clear();
    atual = -1;
    tocando = false;
    frmPlayer.vpVideo.set_file("");
    atualizar_fila(app);
    mostrar_atual(app);
    botao_reproduzir(app);
}

void sair_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmPlayer.close();
}

void reproduzir_click(aylis::Aylis& app, const aylis::Event& ev) {
    if (atual < 0) {
        if (!fila.empty()) tocar(app, 0);
        else abrir_click(app, ev);
    } else if (tocando) {
        frmPlayer.vpVideo.pause();
        tocando = false;
        botao_reproduzir(app);
    } else {
        tocando = true;
        botao_reproduzir(app);
        frmPlayer.vpVideo.play();
    }
}

void parar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    if (atual < 0) return;
    frmPlayer.vpVideo.stop();
    tocando = false;
    botao_reproduzir(app);
    frmPlayer.sbPosicao.set_position(0.0);
}

void anterior_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    if (atual > 0) tocar(app, atual - 1);
    else if (atual == 0 && repetir == 1) tocar(app, static_cast<int>(fila.size()) - 1);
    else if (atual == 0) frmPlayer.vpVideo.seek(0.0);
}

void proxima_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    int i = proximo_indice();
    if (i >= 0) tocar(app, i);
}

void embaralhar_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    embaralhar = !embaralhar;
    frmPlayer.btnEmbaralhar.set_checked(embaralhar);
    frmPlayer.btnEmbaralhar.set_hint(embaralhar ? "Embaralhar: ativado" : "Embaralhar: desativado");
    frmPlayer.mnuEmbaralhar.set_checked(embaralhar);
}

void repetir_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    static const char* DICAS[] = {"Repetir: desativado", "Repetir: todos", "Repetir: um"};
    repetir = (repetir + 1) % 3;
    frmPlayer.btnRepetir.set_checked(repetir > 0);
    frmPlayer.btnRepetir.set_icon(repetir == 2 ? "repeat_one" : "repeat");
    frmPlayer.btnRepetir.set_hint(DICAS[repetir]);
    frmPlayer.mnuRepetir.set_checked(repetir > 0);
}

// Mostra a página "Reproduzindo agora" ou a "Fila de reprodução".
static void pagina(aylis::Aylis& app, bool agora) {
    frmPlayer.pnlAgora.set_visible(agora);
    frmPlayer.pnlFila.set_visible(!agora);
    frmPlayer.btnAgora.set_checked(agora);
    frmPlayer.btnFila.set_checked(!agora);
}

void agora_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    pagina(app, true);
}

void fila_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    pagina(app, false);
}

void posicao_seek(aylis::Aylis& app, const aylis::Event& ev) {
    if (ev.args.kind() == aylis::Json::Kind::Number) frmPlayer.vpVideo.seek(ev.args.number());
}

void mudo_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    mudo = !mudo;
    frmPlayer.vpVideo.set_muted(mudo);
    frmPlayer.btnMudo.set_icon(mudo ? "mute" : "volume");
    frmPlayer.btnMudo.set_hint(mudo ? "Com som" : "Sem som");
}

void volume_change(aylis::Aylis& app, const aylis::Event& ev) {
    if (ev.args.kind() == aylis::Json::Kind::Number) frmPlayer.vpVideo.set_volume(ev.args.number() / 100.0);
}

void video_fim(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    if (repetir == 2 && atual >= 0) {
        tocar(app, atual);
        return;
    }
    int i = proximo_indice();
    if (i >= 0) {
        tocar(app, i);
    } else {
        tocando = false;
        botao_reproduzir(app);
    }
}

void video_erro(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    tocando = false;
    botao_reproduzir(app);
    frmPlayer.lblDetalhe.set_text("Não foi possível reproduzir este arquivo");
}

void fila_item_click(aylis::Aylis& app, const aylis::Event& ev) {
    if (ev.args.kind() != aylis::Json::Kind::Number) return;
    int i = static_cast<int>(ev.args.number());
    if (i >= 0 && i < static_cast<int>(fila.size())) tocar(app, i);
}

void posicao_timer(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    if (atual < 0) return;
    double posicao = frmPlayer.vpVideo.position();
    double duracao = frmPlayer.vpVideo.duration();
    frmPlayer.sbPosicao.set_duration(duracao);
    frmPlayer.sbPosicao.set_position(posicao);
    frmPlayer.lblTempo.set_text(tempo(posicao));
    frmPlayer.lblDuracao.set_text(tempo(duracao));
}

}  // namespace frm_player
