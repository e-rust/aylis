// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_ibge {
void form_show(aylis::Aylis& app, const aylis::Event& ev);
void estado_mudou(aylis::Aylis& app, const aylis::Event& ev);
void municipio_mudou(aylis::Aylis& app, const aylis::Event& ev);
}  // namespace frm_ibge

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmIBGE.form_show", [&app](const aylis::Event& ev) { frm_ibge::form_show(app, ev); });
    app.on("frmIBGE.estado_mudou", [&app](const aylis::Event& ev) { frm_ibge::estado_mudou(app, ev); });
    app.on("frmIBGE.municipio_mudou", [&app](const aylis::Event& ev) { frm_ibge::municipio_mudou(app, ev); });
}
