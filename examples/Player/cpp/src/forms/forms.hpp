// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_player {
void abrir_click(aylis::Aylis& app, const aylis::Event& ev);
void limpar_fila_click(aylis::Aylis& app, const aylis::Event& ev);
void sair_click(aylis::Aylis& app, const aylis::Event& ev);
void reproduzir_click(aylis::Aylis& app, const aylis::Event& ev);
void parar_click(aylis::Aylis& app, const aylis::Event& ev);
void anterior_click(aylis::Aylis& app, const aylis::Event& ev);
void proxima_click(aylis::Aylis& app, const aylis::Event& ev);
void embaralhar_click(aylis::Aylis& app, const aylis::Event& ev);
void repetir_click(aylis::Aylis& app, const aylis::Event& ev);
void agora_click(aylis::Aylis& app, const aylis::Event& ev);
void fila_click(aylis::Aylis& app, const aylis::Event& ev);
void posicao_seek(aylis::Aylis& app, const aylis::Event& ev);
void mudo_click(aylis::Aylis& app, const aylis::Event& ev);
void volume_change(aylis::Aylis& app, const aylis::Event& ev);
void video_fim(aylis::Aylis& app, const aylis::Event& ev);
void video_erro(aylis::Aylis& app, const aylis::Event& ev);
void fila_item_click(aylis::Aylis& app, const aylis::Event& ev);
void abrir_select(aylis::Aylis& app, const aylis::Event& ev);
void posicao_timer(aylis::Aylis& app, const aylis::Event& ev);
}  // namespace frm_player

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmPlayer.abrir_click", [&app](const aylis::Event& ev) { frm_player::abrir_click(app, ev); });
    app.on("frmPlayer.limpar_fila_click", [&app](const aylis::Event& ev) { frm_player::limpar_fila_click(app, ev); });
    app.on("frmPlayer.sair_click", [&app](const aylis::Event& ev) { frm_player::sair_click(app, ev); });
    app.on("frmPlayer.reproduzir_click", [&app](const aylis::Event& ev) { frm_player::reproduzir_click(app, ev); });
    app.on("frmPlayer.parar_click", [&app](const aylis::Event& ev) { frm_player::parar_click(app, ev); });
    app.on("frmPlayer.anterior_click", [&app](const aylis::Event& ev) { frm_player::anterior_click(app, ev); });
    app.on("frmPlayer.proxima_click", [&app](const aylis::Event& ev) { frm_player::proxima_click(app, ev); });
    app.on("frmPlayer.embaralhar_click", [&app](const aylis::Event& ev) { frm_player::embaralhar_click(app, ev); });
    app.on("frmPlayer.repetir_click", [&app](const aylis::Event& ev) { frm_player::repetir_click(app, ev); });
    app.on("frmPlayer.agora_click", [&app](const aylis::Event& ev) { frm_player::agora_click(app, ev); });
    app.on("frmPlayer.fila_click", [&app](const aylis::Event& ev) { frm_player::fila_click(app, ev); });
    app.on("frmPlayer.posicao_seek", [&app](const aylis::Event& ev) { frm_player::posicao_seek(app, ev); });
    app.on("frmPlayer.mudo_click", [&app](const aylis::Event& ev) { frm_player::mudo_click(app, ev); });
    app.on("frmPlayer.volume_change", [&app](const aylis::Event& ev) { frm_player::volume_change(app, ev); });
    app.on("frmPlayer.video_fim", [&app](const aylis::Event& ev) { frm_player::video_fim(app, ev); });
    app.on("frmPlayer.video_erro", [&app](const aylis::Event& ev) { frm_player::video_erro(app, ev); });
    app.on("frmPlayer.fila_item_click", [&app](const aylis::Event& ev) { frm_player::fila_item_click(app, ev); });
    app.on("frmPlayer.abrir_select", [&app](const aylis::Event& ev) { frm_player::abrir_select(app, ev); });
    app.on("frmPlayer.posicao_timer", [&app](const aylis::Event& ev) { frm_player::posicao_timer(app, ev); });
}
