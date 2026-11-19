// Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { Aylis } from "../aylis.ts";
import { AylisComponent, Color, Shadow, Easing } from "../aylis.ts";
import * as components from "./components.ts";
void AylisComponent, Color, Shadow, Easing, components;
import * as ibge from "./ibge.ts";

/** Registra os handlers de todos os forms. */
export function registerForms(aylis: Aylis) {
    aylis.on("frmIBGE.form_show", (ev) => ibge.form_show(aylis, components.frmIBGE));
    aylis.on("frmIBGE.estado_mudou", (ev) => ibge.estado_mudou(aylis, components.frmIBGE.senderAsClientDataSet(ev)!));
    aylis.on("frmIBGE.municipio_mudou", (ev) => ibge.municipio_mudou(aylis, components.frmIBGE.senderAsClientDataSet(ev)!));
}
