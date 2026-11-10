// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { Aylis } from "../aylis.ts";
import "./components.ts";
import * as calculadora from "./calculadora.ts";

/** Registra os handlers de todos os forms. */
export function registerForms(aylis: Aylis) {
    aylis.on("frmCalculadora.form_key_down", (ev) => calculadora.form_key_down(aylis, ev));
    aylis.on("frmCalculadora.limpar_historico_click", (ev) => calculadora.limpar_historico_click(aylis, ev));
    aylis.on("frmCalculadora.historico_item_click", (ev) => calculadora.historico_item_click(aylis, ev));
    aylis.on("frmCalculadora.memoria_item_click", (ev) => calculadora.memoria_item_click(aylis, ev));
    aylis.on("frmCalculadora.historico_click", (ev) => calculadora.historico_click(aylis, ev));
    aylis.on("frmCalculadora.tecla_click", (ev) => calculadora.tecla_click(aylis, ev));
}
