// IBGE: o ponto de entrada. Gerado pela IDE do aylis (pode editar).
// aylis-host --project ../IBGE.aylisproj -- bun src/main.ts

import { Aylis } from "./aylis.ts";
import { registerForms } from "./forms/index.ts";

const aylis = new Aylis();
registerForms(aylis);
await aylis.run(); // abre o form principal do projeto
