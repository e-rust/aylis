// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_player {
void abrir_click(aylis::Aylis& app, aylis::Component sender);
void limpar_fila_click(aylis::Aylis& app, aylis::Component sender);
void sair_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void reproduzir_click(aylis::Aylis& app, aylis::Component sender);
void parar_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void anterior_click(aylis::Aylis& app, aylis::Component sender);
void proxima_click(aylis::Aylis& app, aylis::Component sender);
void embaralhar_click(aylis::Aylis& app, aylis::Component sender);
void repetir_click(aylis::Aylis& app, aylis::Component sender);
void agora_click(aylis::Aylis& app, aylis::Component sender);
void fila_click(aylis::Aylis& app, aylis::Component sender);
void posicao_seek(aylis::Aylis& app, aylis_components::SeekBar sender, double value);
void mudo_click(aylis::Aylis& app, aylis_components::Button sender);
void volume_change(aylis::Aylis& app, aylis_components::TrackBar sender, double value);
void video_fim(aylis::Aylis& app, aylis_components::VideoPlayer sender);
void video_erro(aylis::Aylis& app, aylis_components::VideoPlayer sender, std::string value);
void fila_item_click(aylis::Aylis& app, aylis_components::ListView sender, long long value);
void abrir_select(aylis::Aylis& app, aylis_components::OpenDialog sender, std::vector<std::string> value);
void posicao_timer(aylis::Aylis& app, aylis_components::Timer sender);
}  // namespace frm_player

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmPlayer.abrir_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::abrir_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.limpar_fila_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::limpar_fila_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.sair_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::sair_click(app, frmPlayer.sender_as_menu_item(ev));
    });
    app.on("frmPlayer.reproduzir_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::reproduzir_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.parar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::parar_click(app, frmPlayer.sender_as_menu_item(ev));
    });
    app.on("frmPlayer.anterior_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::anterior_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.proxima_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::proxima_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.embaralhar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::embaralhar_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.repetir_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::repetir_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.agora_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::agora_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.fila_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::fila_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPlayer.posicao_seek", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::posicao_seek(app, frmPlayer.sender_as_seek_bar(ev), ev.args.number());
    });
    app.on("frmPlayer.mudo_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::mudo_click(app, frmPlayer.sender_as_button(ev));
    });
    app.on("frmPlayer.volume_change", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::volume_change(app, frmPlayer.sender_as_track_bar(ev), ev.args.number());
    });
    app.on("frmPlayer.video_fim", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::video_fim(app, frmPlayer.sender_as_video_player(ev));
    });
    app.on("frmPlayer.video_erro", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::video_erro(app, frmPlayer.sender_as_video_player(ev), ev.args.str());
    });
    app.on("frmPlayer.fila_item_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::fila_item_click(app, frmPlayer.sender_as_list_view(ev), static_cast<long long>(ev.args.number()));
    });
    app.on("frmPlayer.abrir_select", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::abrir_select(app, frmPlayer.sender_as_open_dialog(ev), list_of_(ev.args));
    });
    app.on("frmPlayer.posicao_timer", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_player::posicao_timer(app, frmPlayer.sender_as_timer(ev));
    });
}
