// Eventos do form frmIBGE: o código é seu (o registro dos handlers fica no index.ts
// gerado). Dentro de um handler, o form é a janela do evento.
//
// Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa
// e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem
// o mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou
// município escolhido: as bandeiras, os mapas e os dados dos gráficos.

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import type { Aylis, AylisComponent, AylisEvent, Json } from "../aylis.ts";
import { frmIBGE } from "./components.ts";

/** A pasta dos dados no projeto (para os componentes). */
const DADOS = "assets/dados/";
/** A mesma pasta no disco (o backend roda na pasta da linguagem). */
const PASTA = ["../assets/dados", "assets/dados"].find((p) => existsSync(p)) ?? "../assets/dados";

type Municipio = { id: number; uf_id: number; nome: string; populacao: number | null };
type Registro = { [campo: string]: Json };

/** Os municípios (para os 10 maiores de cada estado). */
let municipios: Municipio[] = [];
/** Código do estado ou do município → [[ano, habitantes], ...]. */
let populacao: { [codigo: string]: [number, number][] } = {};

/** Um arquivo JSON da pasta dos dados (null se faltar). */
function ler(arquivo: string): Json {
    try {
        return JSON.parse(readFileSync(join(PASTA, arquivo), "utf8"));
    } catch {
        return null;
    }
}

/** Põe os registros (uma lista de objetos) num ClientDataSet do form. */
async function carregar(aylis: Aylis, dataset: AylisComponent, linhas: Json) {
    await aylis.call("data", { target: dataset.id, op: "load", rows: linhas });
}

/** O registro atual de um ClientDataSet (campo → valor). */
async function registro(aylis: Aylis, dataset: AylisComponent): Promise<Registro> {
    return ((await aylis.call("data", { target: dataset.id, op: "record" })) ?? {}) as Registro;
}

/** O caminho de uma imagem da pasta dos dados (vazio: não tem). */
function imagem(caminho: Json): string {
    return caminho ? DADOS + caminho : "";
}

export async function form_show(aylis: Aylis, ev: AylisEvent) {
    const estados = ler("estados.json");
    if (estados === null) {
        frmIBGE.lblFonte.text = "Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados";
        return;
    }
    municipios = (ler("municipios.json") ?? []) as Municipio[];
    populacao = (ler("populacao.json") ?? {}) as typeof populacao;
    frmIBGE.svgBrasil.source = DADOS + "mapas/BR.svg";
    // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
    await carregar(aylis, frmIBGE.cdsDistritos, ler("distritos.json") ?? []);
    await carregar(aylis, frmIBGE.cdsMunicipios, municipios as unknown as Json);
    await carregar(aylis, frmIBGE.cdsEstados, estados);
    await estado_mudou(aylis, ev);
}

/** Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios. */
export async function estado_mudou(aylis: Aylis, ev: AylisEvent) {
    const e = await registro(aylis, frmIBGE.cdsEstados);
    if (e.id === null || e.id === undefined) return;
    const uf = e.id as number;
    frmIBGE.lblEstado.text = String(e.nome);
    frmIBGE.lblRegiao.text = `${e.sigla} · Região ${e.regiao}`;
    frmIBGE.imgBandeiraEstado.image = imagem(e.bandeira);
    frmIBGE.lblEstadoEstimativa.text = `Estimativa de ${e.estimativa_ano}`;
    frmIBGE.svgBrasil.highlight = `e${uf}`;
    const mapa = `${DADOS}mapas/${uf}.svg`;
    frmIBGE.svgEstado.source = mapa;
    frmIBGE.svgMunicipio.source = mapa;

    const doEstado = municipios.filter((m) => m.uf_id === uf);
    doEstado.sort((a, b) => (b.populacao ?? 0) - (a.populacao ?? 0) || (a.nome < b.nome ? -1 : a.nome > b.nome ? 1 : 0));
    await carregar(aylis, frmIBGE.cdsMaiores, doEstado.slice(0, 10).map((m) => ({ nome: m.nome, habitantes: m.populacao })));
    await municipio_mudou(aylis, ev);
}

/** Outro município: a bandeira, o destaque nos mapas e a população ano a ano. */
export async function municipio_mudou(aylis: Aylis, ev: AylisEvent) {
    void ev;
    const m = await registro(aylis, frmIBGE.cdsMunicipios);
    if (m.id === null || m.id === undefined) return;
    const codigo = m.id as number;
    frmIBGE.lblMunicipio.text = String(m.nome);
    frmIBGE.lblCodigo.text = `Código IBGE ${codigo}`;
    const bandeira = (m.bandeira as string | null) ?? "";
    frmIBGE.imgBandeiraMunicipio.image = imagem(bandeira);
    frmIBGE.lblSemBandeira.visible = bandeira === "";
    frmIBGE.lblEstimativa.text = `Estimativa de ${m.estimativa_ano}`;
    const alvo = `m${codigo}`;
    frmIBGE.svgEstado.highlight = alvo;
    frmIBGE.svgMunicipio.highlight = alvo;
    frmIBGE.svgMunicipio.zoomTo = alvo;

    const serie = populacao[String(codigo)] ?? [];
    await carregar(aylis, frmIBGE.cdsPopulacao, serie.map(([ano, habitantes]) => ({ ano: String(ano), habitantes })));
}
