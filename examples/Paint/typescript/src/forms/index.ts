// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { Aylis } from "../aylis.ts";
import { AylisComponent, Color, Shadow, Easing } from "../aylis.ts";
import * as components from "./components.ts";
void AylisComponent, Color, Shadow, Easing, components;
import * as paint from "./paint.ts";

/** Registra os handlers de todos os forms. */
export function registerForms(aylis: Aylis) {
    aylis.on("frmPaint.novo_click", (ev) => paint.novo_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.abrir_click", (ev) => paint.abrir_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.salvar_click", (ev) => paint.salvar_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPaint.salvar_como_click", (ev) => paint.salvar_como_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.sair_click", (ev) => paint.sair_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.desfazer_click", (ev) => paint.desfazer_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPaint.refazer_click", (ev) => paint.refazer_click(aylis, new AylisComponent(aylis, ev.sender.id, ev.sender.name)));
    aylis.on("frmPaint.recortar_click", (ev) => paint.recortar_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.copiar_click", (ev) => paint.copiar_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.colar_click", (ev) => paint.colar_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.excluir_click", (ev) => paint.excluir_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.selecionar_tudo_click", (ev) => paint.selecionar_tudo_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.limpar_click", (ev) => paint.limpar_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.ampliar_click", (ev) => paint.ampliar_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.reduzir_click", (ev) => paint.reduzir_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.tamanho_real_click", (ev) => paint.tamanho_real_click(aylis, components.frmPaint.senderAsMenuItem(ev)!));
    aylis.on("frmPaint.ferramenta_click", (ev) => paint.ferramenta_click(aylis, components.frmPaint.senderAsButton(ev)!));
    aylis.on("frmPaint.preencher_change", (ev) => paint.preencher_change(aylis, components.frmPaint.senderAsCheckBox(ev)!, ev.args as boolean));
    aylis.on("frmPaint.espessura_change", (ev) => paint.espessura_change(aylis, components.frmPaint.senderAsComboBox(ev)!, String(ev.args ?? "")));
    aylis.on("frmPaint.cor1_click", (ev) => paint.cor1_click(aylis, components.frmPaint.senderAsCircle(ev)!));
    aylis.on("frmPaint.cor2_click", (ev) => paint.cor2_click(aylis, components.frmPaint.senderAsCircle(ev)!));
    aylis.on("frmPaint.paleta_click", (ev) => paint.paleta_click(aylis, components.frmPaint.senderAsCircle(ev)!));
    aylis.on("frmPaint.desenho_change", (ev) => paint.desenho_change(aylis, components.frmPaint.senderAsPaintCanvas(ev)!));
    aylis.on("frmPaint.desenho_pick", (ev) => paint.desenho_pick(aylis, components.frmPaint.senderAsPaintCanvas(ev)!, String(ev.args ?? "")));
    aylis.on("frmPaint.desenho_cursor", (ev) => paint.desenho_cursor(aylis, components.frmPaint.senderAsPaintCanvas(ev)!, String(ev.args ?? "")));
    aylis.on("frmPaint.abrir_select", (ev) => paint.abrir_select(aylis, components.frmPaint.senderAsOpenDialog(ev)!, components.listOf(ev.args)));
    aylis.on("frmPaint.salvar_select", (ev) => paint.salvar_select(aylis, components.frmPaint.senderAsSaveDialog(ev)!, components.listOf(ev.args)));
}
