// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_calculadora {
void form_key_down(aylis::Aylis& app, const aylis::Event& ev);
void limpar_historico_click(aylis::Aylis& app, const aylis::Event& ev);
void historico_item_click(aylis::Aylis& app, const aylis::Event& ev);
void memoria_item_click(aylis::Aylis& app, const aylis::Event& ev);
void historico_click(aylis::Aylis& app, const aylis::Event& ev);
void tecla_click(aylis::Aylis& app, const aylis::Event& ev);
}  // namespace frm_calculadora

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmCalculadora.form_key_down", [&app](const aylis::Event& ev) { frm_calculadora::form_key_down(app, ev); });
    app.on("frmCalculadora.limpar_historico_click", [&app](const aylis::Event& ev) { frm_calculadora::limpar_historico_click(app, ev); });
    app.on("frmCalculadora.historico_item_click", [&app](const aylis::Event& ev) { frm_calculadora::historico_item_click(app, ev); });
    app.on("frmCalculadora.memoria_item_click", [&app](const aylis::Event& ev) { frm_calculadora::memoria_item_click(app, ev); });
    app.on("frmCalculadora.historico_click", [&app](const aylis::Event& ev) { frm_calculadora::historico_click(app, ev); });
    app.on("frmCalculadora.tecla_click", [&app](const aylis::Event& ev) { frm_calculadora::tecla_click(app, ev); });
}
