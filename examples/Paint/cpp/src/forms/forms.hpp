// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#pragma once

#include "../aylis.hpp"
#include "components.hpp"

namespace frm_paint {
void novo_click(aylis::Aylis& app, const aylis::Event& ev);
void abrir_click(aylis::Aylis& app, const aylis::Event& ev);
void salvar_click(aylis::Aylis& app, const aylis::Event& ev);
void salvar_como_click(aylis::Aylis& app, const aylis::Event& ev);
void sair_click(aylis::Aylis& app, const aylis::Event& ev);
void desfazer_click(aylis::Aylis& app, const aylis::Event& ev);
void refazer_click(aylis::Aylis& app, const aylis::Event& ev);
void recortar_click(aylis::Aylis& app, const aylis::Event& ev);
void copiar_click(aylis::Aylis& app, const aylis::Event& ev);
void colar_click(aylis::Aylis& app, const aylis::Event& ev);
void excluir_click(aylis::Aylis& app, const aylis::Event& ev);
void selecionar_tudo_click(aylis::Aylis& app, const aylis::Event& ev);
void limpar_click(aylis::Aylis& app, const aylis::Event& ev);
void ampliar_click(aylis::Aylis& app, const aylis::Event& ev);
void reduzir_click(aylis::Aylis& app, const aylis::Event& ev);
void tamanho_real_click(aylis::Aylis& app, const aylis::Event& ev);
void ferramenta_click(aylis::Aylis& app, const aylis::Event& ev);
void preencher_change(aylis::Aylis& app, const aylis::Event& ev);
void espessura_change(aylis::Aylis& app, const aylis::Event& ev);
void cor1_click(aylis::Aylis& app, const aylis::Event& ev);
void cor2_click(aylis::Aylis& app, const aylis::Event& ev);
void paleta_click(aylis::Aylis& app, const aylis::Event& ev);
void desenho_change(aylis::Aylis& app, const aylis::Event& ev);
void desenho_pick(aylis::Aylis& app, const aylis::Event& ev);
void desenho_cursor(aylis::Aylis& app, const aylis::Event& ev);
void abrir_select(aylis::Aylis& app, const aylis::Event& ev);
void salvar_select(aylis::Aylis& app, const aylis::Event& ev);
}  // namespace frm_paint

// Registra as classes dos forms e os handlers de todos.
inline void register_forms(aylis::Aylis& app) {
    aylis_components::register_classes();
    app.on("frmPaint.novo_click", [&app](const aylis::Event& ev) { frm_paint::novo_click(app, ev); });
    app.on("frmPaint.abrir_click", [&app](const aylis::Event& ev) { frm_paint::abrir_click(app, ev); });
    app.on("frmPaint.salvar_click", [&app](const aylis::Event& ev) { frm_paint::salvar_click(app, ev); });
    app.on("frmPaint.salvar_como_click", [&app](const aylis::Event& ev) { frm_paint::salvar_como_click(app, ev); });
    app.on("frmPaint.sair_click", [&app](const aylis::Event& ev) { frm_paint::sair_click(app, ev); });
    app.on("frmPaint.desfazer_click", [&app](const aylis::Event& ev) { frm_paint::desfazer_click(app, ev); });
    app.on("frmPaint.refazer_click", [&app](const aylis::Event& ev) { frm_paint::refazer_click(app, ev); });
    app.on("frmPaint.recortar_click", [&app](const aylis::Event& ev) { frm_paint::recortar_click(app, ev); });
    app.on("frmPaint.copiar_click", [&app](const aylis::Event& ev) { frm_paint::copiar_click(app, ev); });
    app.on("frmPaint.colar_click", [&app](const aylis::Event& ev) { frm_paint::colar_click(app, ev); });
    app.on("frmPaint.excluir_click", [&app](const aylis::Event& ev) { frm_paint::excluir_click(app, ev); });
    app.on("frmPaint.selecionar_tudo_click", [&app](const aylis::Event& ev) { frm_paint::selecionar_tudo_click(app, ev); });
    app.on("frmPaint.limpar_click", [&app](const aylis::Event& ev) { frm_paint::limpar_click(app, ev); });
    app.on("frmPaint.ampliar_click", [&app](const aylis::Event& ev) { frm_paint::ampliar_click(app, ev); });
    app.on("frmPaint.reduzir_click", [&app](const aylis::Event& ev) { frm_paint::reduzir_click(app, ev); });
    app.on("frmPaint.tamanho_real_click", [&app](const aylis::Event& ev) { frm_paint::tamanho_real_click(app, ev); });
    app.on("frmPaint.ferramenta_click", [&app](const aylis::Event& ev) { frm_paint::ferramenta_click(app, ev); });
    app.on("frmPaint.preencher_change", [&app](const aylis::Event& ev) { frm_paint::preencher_change(app, ev); });
    app.on("frmPaint.espessura_change", [&app](const aylis::Event& ev) { frm_paint::espessura_change(app, ev); });
    app.on("frmPaint.cor1_click", [&app](const aylis::Event& ev) { frm_paint::cor1_click(app, ev); });
    app.on("frmPaint.cor2_click", [&app](const aylis::Event& ev) { frm_paint::cor2_click(app, ev); });
    app.on("frmPaint.paleta_click", [&app](const aylis::Event& ev) { frm_paint::paleta_click(app, ev); });
    app.on("frmPaint.desenho_change", [&app](const aylis::Event& ev) { frm_paint::desenho_change(app, ev); });
    app.on("frmPaint.desenho_pick", [&app](const aylis::Event& ev) { frm_paint::desenho_pick(app, ev); });
    app.on("frmPaint.desenho_cursor", [&app](const aylis::Event& ev) { frm_paint::desenho_cursor(app, ev); });
    app.on("frmPaint.abrir_select", [&app](const aylis::Event& ev) { frm_paint::abrir_select(app, ev); });
    app.on("frmPaint.salvar_select", [&app](const aylis::Event& ev) { frm_paint::salvar_select(app, ev); });
}
