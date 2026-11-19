// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_ibge {
void form_show(aylis::Aylis& app, aylis_components::Form sender);
void estado_mudou(aylis::Aylis& app, aylis_components::ClientDataSet sender);
void municipio_mudou(aylis::Aylis& app, aylis_components::ClientDataSet sender);
}  // namespace frm_ibge

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmIBGE.form_show", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_ibge::form_show(app, frmIBGE);
    });
    app.on("frmIBGE.estado_mudou", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_ibge::estado_mudou(app, frmIBGE.sender_as_client_data_set(ev));
    });
    app.on("frmIBGE.municipio_mudou", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_ibge::municipio_mudou(app, frmIBGE.sender_as_client_data_set(ev));
    });
}
