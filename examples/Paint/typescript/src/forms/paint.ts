// Eventos do form frmPaint: o código é seu (o registro dos handlers fica no index.ts
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
// ferramentas, as cores, o zoom, o arquivo e a barra de status.

import type { Aylis, AylisComponent } from "../aylis.ts";
import { Color } from "../aylis.ts";
import {
    type Button, type CheckBox, type Circle, type ComboBox, type MenuItem, type OpenDialog, type PaintCanvas, type PaintTool, type SaveDialog,
    frmPaint,
} from "./components.ts";

/** O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%). */
const estado = { arquivo: "", modificado: false, corAtiva: 1, zoom: 100 };

/** Os botões de ferramenta (os do form do evento) e a ferramenta de cada um. */
function ferramentas(): [Button, PaintTool][] {
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
    const tamanho = await frmPaint.pcDesenho.size();
    frmPaint.lblTamanho.text = tamanho ? `${tamanho.width} × ${tamanho.height}px` : "";
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

export async function novo_click(aylis: Aylis, sender: MenuItem) {
    await frmPaint.pcDesenho.new(640, 400);
    estado.arquivo = "";
    estado.modificado = false;
    titulo(aylis);
    await atualizarEdicao(aylis);
}

export function abrir_click(aylis: Aylis, sender: MenuItem) {
    frmPaint.dlgAbrir.execute();
}

export async function salvar_click(aylis: Aylis, sender: AylisComponent) {
    if (estado.arquivo) await salvarEm(aylis, estado.arquivo);
    // Sem arquivo ainda: o mesmo do "Salvar como".
    else frmPaint.dlgSalvar.execute();
}

export function salvar_como_click(aylis: Aylis, sender: MenuItem) {
    frmPaint.dlgSalvar.execute();
}

export function sair_click(aylis: Aylis, sender: MenuItem) {
    frmPaint.close();
}

export async function desfazer_click(aylis: Aylis, sender: AylisComponent) {
    await frmPaint.pcDesenho.undo();
    await marcarModificado(aylis);
}

export async function refazer_click(aylis: Aylis, sender: AylisComponent) {
    await frmPaint.pcDesenho.redo();
    await marcarModificado(aylis);
}

export async function limpar_click(aylis: Aylis, sender: MenuItem) {
    await frmPaint.pcDesenho.clear();
    await marcarModificado(aylis);
}

export function ampliar_click(aylis: Aylis, sender: MenuItem) {
    aplicarZoom(aylis, Math.min(estado.zoom * 2, 800));
}

export function reduzir_click(aylis: Aylis, sender: MenuItem) {
    aplicarZoom(aylis, Math.max(Math.floor(estado.zoom / 2), 25));
}

export function tamanho_real_click(aylis: Aylis, sender: MenuItem) {
    aplicarZoom(aylis, 100);
}

export function ferramenta_click(aylis: Aylis, sender: Button) {
    const escolhida = ferramentas().find(([b]) => b.id === sender.id);
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
        circulo.stroke = n === cor ? Color.rgb(0x00, 0x67, 0xc0) : Color.rgb(0x9a, 0x9a, 0x9a);
        circulo.strokeWidth = n === cor ? 3 : 1;
    }
}

export function espessura_change(aylis: Aylis, sender: ComboBox, value: string) {
    // "5 px" → 5
    const n = parseInt(value.split(" ")[0], 10);
    if (!Number.isNaN(n)) frmPaint.pcDesenho.brushSize = n;
}

export function preencher_change(aylis: Aylis, sender: CheckBox, value: boolean) {
    frmPaint.pcDesenho.filled = value;
}

export function cor1_click(aylis: Aylis, sender: Circle) {
    marcarCor(aylis, 1);
}

export function cor2_click(aylis: Aylis, sender: Circle) {
    marcarCor(aylis, 2);
}

export async function paleta_click(aylis: Aylis, sender: Circle) {
    const cor = await sender.fill;
    if (estado.corAtiva === 1) {
        frmPaint.pcDesenho.color = cor;
        frmPaint.btnCor1.fill = cor;
    } else {
        frmPaint.pcDesenho.backColor = cor;
        frmPaint.btnCor2.fill = cor;
    }
}

export async function desenho_change(aylis: Aylis, sender: PaintCanvas) {
    await marcarModificado(aylis);
}

export function desenho_pick(aylis: Aylis, sender: PaintCanvas, value: string) {
    // "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
    if (value.endsWith("2")) frmPaint.btnCor2.fill = Color.hex(value.slice(0, -1));
    else frmPaint.btnCor1.fill = Color.hex(value);
}

export function desenho_cursor(aylis: Aylis, sender: PaintCanvas, value: string) {
    frmPaint.lblPosicao.text = value ? `${value}px` : "";
}

export async function abrir_select(aylis: Aylis, sender: OpenDialog, value: string[]) {
    // Um arquivo só: o primeiro da lista.
    const caminho = value[0] ?? "";
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

export async function salvar_select(aylis: Aylis, sender: SaveDialog, value: string[]) {
    await salvarEm(aylis, value[0] ?? "");
}

export async function recortar_click(aylis: Aylis, sender: MenuItem) {
    await frmPaint.pcDesenho.cut();
}

export async function copiar_click(aylis: Aylis, sender: MenuItem) {
    await frmPaint.pcDesenho.copy();
}

export async function colar_click(aylis: Aylis, sender: MenuItem) {
    // O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
    if ((await frmPaint.pcDesenho.paste()) === true) marcarFerramenta(aylis, frmPaint.btnSelecionar);
}

export async function excluir_click(aylis: Aylis, sender: MenuItem) {
    await frmPaint.pcDesenho.delete();
}

export async function selecionar_tudo_click(aylis: Aylis, sender: MenuItem) {
    await frmPaint.pcDesenho.selectAll();
    marcarFerramenta(aylis, frmPaint.btnSelecionar);
}
