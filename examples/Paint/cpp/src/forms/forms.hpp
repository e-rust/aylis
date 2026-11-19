// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_paint {
void novo_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void abrir_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void salvar_click(aylis::Aylis& app, aylis::Component sender);
void salvar_como_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void sair_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void desfazer_click(aylis::Aylis& app, aylis::Component sender);
void refazer_click(aylis::Aylis& app, aylis::Component sender);
void recortar_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void copiar_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void colar_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void excluir_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void selecionar_tudo_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void limpar_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void ampliar_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void reduzir_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void tamanho_real_click(aylis::Aylis& app, aylis_components::MenuItem sender);
void ferramenta_click(aylis::Aylis& app, aylis_components::Button sender);
void preencher_change(aylis::Aylis& app, aylis_components::CheckBox sender, bool value);
void espessura_change(aylis::Aylis& app, aylis_components::ComboBox sender, std::string value);
void cor1_click(aylis::Aylis& app, aylis_components::Circle sender);
void cor2_click(aylis::Aylis& app, aylis_components::Circle sender);
void paleta_click(aylis::Aylis& app, aylis_components::Circle sender);
void desenho_change(aylis::Aylis& app, aylis_components::PaintCanvas sender);
void desenho_pick(aylis::Aylis& app, aylis_components::PaintCanvas sender, std::string value);
void desenho_cursor(aylis::Aylis& app, aylis_components::PaintCanvas sender, std::string value);
void abrir_select(aylis::Aylis& app, aylis_components::OpenDialog sender, std::vector<std::string> value);
void salvar_select(aylis::Aylis& app, aylis_components::SaveDialog sender, std::vector<std::string> value);
}  // namespace frm_paint

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmPaint.novo_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::novo_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.abrir_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::abrir_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.salvar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::salvar_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPaint.salvar_como_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::salvar_como_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.sair_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::sair_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.desfazer_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::desfazer_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPaint.refazer_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::refazer_click(app, aylis::Component(&app, ev.sender_id, ev.sender_name));
    });
    app.on("frmPaint.recortar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::recortar_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.copiar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::copiar_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.colar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::colar_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.excluir_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::excluir_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.selecionar_tudo_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::selecionar_tudo_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.limpar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::limpar_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.ampliar_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::ampliar_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.reduzir_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::reduzir_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.tamanho_real_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::tamanho_real_click(app, frmPaint.sender_as_menu_item(ev));
    });
    app.on("frmPaint.ferramenta_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::ferramenta_click(app, frmPaint.sender_as_button(ev));
    });
    app.on("frmPaint.preencher_change", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::preencher_change(app, frmPaint.sender_as_check_box(ev), ev.args.boolean());
    });
    app.on("frmPaint.espessura_change", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::espessura_change(app, frmPaint.sender_as_combo_box(ev), ev.args.str());
    });
    app.on("frmPaint.cor1_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::cor1_click(app, frmPaint.sender_as_circle(ev));
    });
    app.on("frmPaint.cor2_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::cor2_click(app, frmPaint.sender_as_circle(ev));
    });
    app.on("frmPaint.paleta_click", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::paleta_click(app, frmPaint.sender_as_circle(ev));
    });
    app.on("frmPaint.desenho_change", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::desenho_change(app, frmPaint.sender_as_paint_canvas(ev));
    });
    app.on("frmPaint.desenho_pick", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::desenho_pick(app, frmPaint.sender_as_paint_canvas(ev), ev.args.str());
    });
    app.on("frmPaint.desenho_cursor", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::desenho_cursor(app, frmPaint.sender_as_paint_canvas(ev), ev.args.str());
    });
    app.on("frmPaint.abrir_select", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::abrir_select(app, frmPaint.sender_as_open_dialog(ev), list_of_(ev.args));
    });
    app.on("frmPaint.salvar_select", [&app](const aylis::Event& ev) {
        using namespace aylis_components;
        frm_paint::salvar_select(app, frmPaint.sender_as_save_dialog(ev), list_of_(ev.args));
    });
}
