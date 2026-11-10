// Eventos do form frmPaint: o código é seu (o registro dos handlers fica no index.ts
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
// ferramentas, as cores, o zoom, o arquivo e a barra de status.

import type { Aylis, AylisEvent } from "../aylis.ts";
import { type Button, type Circle, frmPaint } from "./components.ts";

/** O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%). */
const estado = { arquivo: "", modificado: false, corAtiva: 1, zoom: 100 };

/** Os botões de ferramenta (os do form do evento) e a ferramenta de cada um. */
function ferramentas(): [Button, string][] {
    return [
        [frmPaint.btnSelecionar, "select"], [frmPaint.btnLapis, "pencil"], [frmPaint.btnPincel, "brush"], [frmPaint.btnBorracha, "eraser"],
        [frmPaint.btnBalde, "fill"], [frmPaint.btnContaGotas, "picker"], [frmPaint.btnLinha, "line"], [frmPaint.btnRetangulo, "rectangle"],
        [frmPaint.btnElipse, "ellipse"],
    ];
}

/** O nome do arquivo sem a pasta. */
function nomeDoArquivo(caminho: string): string {
    return caminho.split(/[\\/]/).pop() ?? caminho;
}

/** O título como o do Paint do Windows: "*Sem título - Paint". */
function titulo(aylis: Aylis) {
    const nome = estado.arquivo ? nomeDoArquivo(estado.arquivo) : "Sem título";
    frmPaint.caption = `${estado.modificado ? "*" : ""}${nome} - Paint`;
}

/** Desfazer, refazer e o tamanho da imagem acompanham o desenho. */
async function atualizarEdicao(aylis: Aylis) {
    const desfazer = (await frmPaint.pcDesenho.canUndo()) === true;
    const refazer = (await frmPaint.pcDesenho.canRedo()) === true;
    frmPaint.mnuDesfazer.enabled = desfazer;
    frmPaint.btnDesfazer.enabled = desfazer;
    frmPaint.mnuRefazer.enabled = refazer;
    frmPaint.btnRefazer.enabled = refazer;
    frmPaint.lblTamanho.text = `${await frmPaint.pcDesenho.size()}px`;
}

async function marcarModificado(aylis: Aylis) {
    estado.modificado = true;
    titulo(aylis);
    await atualizarEdicao(aylis);
}

function aplicarZoom(aylis: Aylis, zoom: number) {
    estado.zoom = zoom;
    frmPaint.pcDesenho.zoom = zoom / 100;
    frmPaint.lblZoom.text = `${zoom}%`;
}

async function salvarEm(aylis: Aylis, caminho: string) {
    try {
        await frmPaint.pcDesenho.save(caminho);
    } catch (erro) {
        aylis.send("message_dlg", { text: `Não foi possível salvar a imagem.\n${String(erro).replace(/^Error: /, "")}`, kind: "error", buttons: ["ok"] });
        return;
    }
    estado.arquivo = caminho;
    estado.modificado = false;
    titulo(aylis);
}

function texto(ev: AylisEvent): string {
    return typeof ev.args === "string" ? ev.args : "";
}

export async function novo_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.new(640, 400);
    estado.arquivo = "";
    estado.modificado = false;
    titulo(aylis);
    await atualizarEdicao(aylis);
}

export function abrir_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    frmPaint.dlgAbrir.execute();
}

export async function salvar_click(aylis: Aylis, ev: AylisEvent) {
    if (estado.arquivo) await salvarEm(aylis, estado.arquivo);
    else salvar_como_click(aylis, ev);
}

export function salvar_como_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    frmPaint.dlgSalvar.execute();
}

export function sair_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    frmPaint.close();
}

export async function desfazer_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.undo();
    await marcarModificado(aylis);
}

export async function refazer_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.redo();
    await marcarModificado(aylis);
}

export async function limpar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.clear();
    await marcarModificado(aylis);
}

export function ampliar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    aplicarZoom(aylis, Math.min(estado.zoom * 2, 800));
}

export function reduzir_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    aplicarZoom(aylis, Math.max(Math.floor(estado.zoom / 2), 25));
}

export function tamanho_real_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    aplicarZoom(aylis, 100);
}

export function ferramenta_click(aylis: Aylis, ev: AylisEvent) {
    const escolhida = ferramentas().find(([b]) => b.id === ev.sender.id);
    if (!escolhida) return;
    frmPaint.pcDesenho.tool = escolhida[1];
    marcarFerramenta(aylis, escolhida[0]);
}

/** Só o botão da ferramenta escolhida fica marcado. */
function marcarFerramenta(aylis: Aylis, escolhido: Button) {
    for (const [botao] of ferramentas()) botao.checked = botao.id === escolhido.id;
}

/** A cor que a paleta troca fica com o anel azul, como no Paint. */
function marcarCor(aylis: Aylis, cor: number) {
    estado.corAtiva = cor;
    for (const [circulo, n] of [[frmPaint.btnCor1, 1], [frmPaint.btnCor2, 2]] as [Circle, number][]) {
        circulo.stroke = n === cor ? "#0067C0" : "#9A9A9A";
        circulo.strokeWidth = n === cor ? 3 : 1;
    }
}

export function espessura_change(aylis: Aylis, ev: AylisEvent) {
    // "5 px" → 5
    const n = parseInt(texto(ev).split(" ")[0], 10);
    if (!Number.isNaN(n)) frmPaint.pcDesenho.brushSize = n;
}

export function preencher_change(aylis: Aylis, ev: AylisEvent) {
    if (typeof ev.args === "boolean") frmPaint.pcDesenho.filled = ev.args;
}

export function cor1_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    marcarCor(aylis, 1);
}

export function cor2_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    marcarCor(aylis, 2);
}

export async function paleta_click(aylis: Aylis, ev: AylisEvent) {
    const cor = await frmPaint.senderAsCircle(ev).fill;
    if (estado.corAtiva === 1) {
        frmPaint.pcDesenho.color = cor;
        frmPaint.btnCor1.fill = cor;
    } else {
        frmPaint.pcDesenho.backColor = cor;
        frmPaint.btnCor2.fill = cor;
    }
}

export async function desenho_change(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await marcarModificado(aylis);
}

export function desenho_pick(aylis: Aylis, ev: AylisEvent) {
    // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
    const t = texto(ev);
    if (t.endsWith("2")) frmPaint.btnCor2.fill = t.slice(0, -1);
    else frmPaint.btnCor1.fill = t;
}

export function desenho_cursor(aylis: Aylis, ev: AylisEvent) {
    const t = texto(ev);
    frmPaint.lblPosicao.text = t ? `${t}px` : "";
}

export async function abrir_select(aylis: Aylis, ev: AylisEvent) {
    const caminho = texto(ev);
    try {
        await frmPaint.pcDesenho.open(caminho);
    } catch (erro) {
        aylis.send("message_dlg", { text: `Não foi possível abrir a imagem.\n${String(erro).replace(/^Error: /, "")}`, kind: "error", buttons: ["ok"] });
        return;
    }
    estado.arquivo = caminho;
    estado.modificado = false;
    titulo(aylis);
    await atualizarEdicao(aylis);
}

export async function salvar_select(aylis: Aylis, ev: AylisEvent) {
    await salvarEm(aylis, texto(ev));
}

export async function recortar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.cut();
}

export async function copiar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.copy();
}

export async function colar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
    if ((await frmPaint.pcDesenho.paste()) === true) marcarFerramenta(aylis, frmPaint.btnSelecionar);
}

export async function excluir_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.delete();
}

export async function selecionar_tudo_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    await frmPaint.pcDesenho.selectAll();
    marcarFerramenta(aylis, frmPaint.btnSelecionar);
}
