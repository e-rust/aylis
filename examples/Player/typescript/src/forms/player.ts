// Eventos do form frmPlayer: o código é seu (o registro dos handlers fica no index.ts
// gerado). Dentro de um handler, o form é a janela do evento.
//
// O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
// anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
// toca é o componente VideoPlayer (vpVideo), áudio e vídeo.

import type { Aylis, AylisEvent } from "../aylis.ts";
import { frmPlayer } from "./components.ts";

/** O que o form guarda. repetir: 0 não repete, 1 repete a fila, 2 repete a mídia. */
const estado = { fila: [] as string[], atual: -1, tocando: false, embaralhar: false, repetir: 0, mudo: false, semente: 12345n };

/** As extensões de áudio (o resto é vídeo). */
const AUDIO = ["mp3", "wav", "m4a", "flac", "ogg", "wma", "aac"];

/** O nome do arquivo sem a pasta. */
function nomeDoArquivo(caminho: string): string {
    return caminho.split(/[\\/]/).pop() ?? caminho;
}

/** A pasta do arquivo (vazio se não tiver). */
function pasta(caminho: string): string {
    const i = Math.max(caminho.lastIndexOf("\\"), caminho.lastIndexOf("/"));
    return i >= 0 ? caminho.slice(0, i) : "";
}

function semExtensao(nome: string): string {
    const i = nome.lastIndexOf(".");
    return i > 0 ? nome.slice(0, i) : nome;
}

function tipo(caminho: string): string {
    const nome = nomeDoArquivo(caminho);
    const i = nome.lastIndexOf(".");
    const ext = i >= 0 ? nome.slice(i + 1).toLowerCase() : "";
    return AUDIO.includes(ext) ? "Música" : "Vídeo";
}

/** "3:07" */
function tempo(segundos: number): string {
    const s = Math.floor(Math.max(segundos, 0));
    return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

/** A fila na lista, com a mídia atual selecionada. */
function atualizarFila(aylis: Aylis) {
    frmPlayer.lvFila.items = estado.fila.map((c) => `${nomeDoArquivo(c)}|${pasta(c)}`).join(";");
    frmPlayer.lvFila.itemIndex = estado.atual;
}

/** O título, o detalhe e o título da janela. */
function mostrarAtual(aylis: Aylis) {
    if (estado.atual < 0) {
        frmPlayer.lblTitulo.text = "Nenhuma mídia";
        frmPlayer.lblDetalhe.text = "";
        frmPlayer.caption = "Reprodutor Multimídia";
        return;
    }
    const caminho = estado.fila[estado.atual];
    const titulo = semExtensao(nomeDoArquivo(caminho));
    frmPlayer.lblTitulo.text = titulo;
    frmPlayer.lblDetalhe.text = `${tipo(caminho)} · ${estado.atual + 1} de ${estado.fila.length}`;
    frmPlayer.caption = `${titulo} - Reprodutor Multimídia`;
}

/** O botão do meio mostra o que ele vai fazer: tocar ou pausar. */
function botaoReproduzir(aylis: Aylis) {
    frmPlayer.btnReproduzir.icon = estado.tocando ? "pause" : "play";
    frmPlayer.btnReproduzir.hint = estado.tocando ? "Pausar (Ctrl+P)" : "Reproduzir (Ctrl+P)";
}

/** Toca a mídia i da fila. Tocar é a última coisa: um erro chega no video_erro. */
function tocar(aylis: Aylis, i: number) {
    estado.atual = i;
    estado.tocando = true;
    frmPlayer.vpVideo.file = estado.fila[i];
    mostrarAtual(aylis);
    atualizarFila(aylis);
    botaoReproduzir(aylis);
    frmPlayer.sbPosicao.position = 0;
    frmPlayer.vpVideo.play();
}

/** A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se repetir a fila.
 * -1: acabou. (O sorteio usa BigInt: a conta passa de 2^53.) */
function proximoIndice(): number {
    const n = estado.fila.length;
    if (n === 0) return -1;
    if (estado.embaralhar) {
        if (n === 1) return 0;
        for (;;) {
            estado.semente = (estado.semente * 1103515245n + 12345n) % 2147483648n;
            const i = Number(estado.semente % BigInt(n));
            if (i !== estado.atual) return i;
        }
    }
    if (estado.atual + 1 < n) return estado.atual + 1;
    if (estado.repetir === 1) return 0;
    return -1;
}

export function abrir_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    frmPlayer.dlgAbrir.execute();
}

export function abrir_select(aylis: Aylis, ev: AylisEvent) {
    // Os arquivos escolhidos vêm separados por ";".
    const novos = String(ev.args ?? "").split(";").filter((s) => s);
    if (novos.length === 0) return;
    const primeiro = estado.fila.length;
    estado.fila.push(...novos);
    atualizarFila(aylis);
    if (estado.atual < 0) tocar(aylis, primeiro);
    else mostrarAtual(aylis);
}

export function limpar_fila_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    frmPlayer.vpVideo.stop();
    estado.fila = [];
    estado.atual = -1;
    estado.tocando = false;
    frmPlayer.vpVideo.file = "";
    atualizarFila(aylis);
    mostrarAtual(aylis);
    botaoReproduzir(aylis);
}

export function sair_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    frmPlayer.close();
}

export function reproduzir_click(aylis: Aylis, ev: AylisEvent) {
    if (estado.atual < 0) {
        if (estado.fila.length > 0) tocar(aylis, 0);
        else abrir_click(aylis, ev);
    } else if (estado.tocando) {
        frmPlayer.vpVideo.pause();
        estado.tocando = false;
        botaoReproduzir(aylis);
    } else {
        estado.tocando = true;
        botaoReproduzir(aylis);
        frmPlayer.vpVideo.play();
    }
}

export function parar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    if (estado.atual < 0) return;
    frmPlayer.vpVideo.stop();
    estado.tocando = false;
    botaoReproduzir(aylis);
    frmPlayer.sbPosicao.position = 0;
}

export function anterior_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    const atual = estado.atual;
    if (atual > 0) tocar(aylis, atual - 1);
    else if (atual === 0 && estado.repetir === 1) tocar(aylis, estado.fila.length - 1);
    else if (atual === 0) frmPlayer.vpVideo.seek(0);
}

export function proxima_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    const i = proximoIndice();
    if (i >= 0) tocar(aylis, i);
}

export function embaralhar_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    estado.embaralhar = !estado.embaralhar;
    const ligado = estado.embaralhar;
    frmPlayer.btnEmbaralhar.checked = ligado;
    frmPlayer.btnEmbaralhar.hint = ligado ? "Embaralhar: ativado" : "Embaralhar: desativado";
    frmPlayer.mnuEmbaralhar.checked = ligado;
}

export function repetir_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    estado.repetir = (estado.repetir + 1) % 3;
    const modo = estado.repetir;
    frmPlayer.btnRepetir.checked = modo > 0;
    frmPlayer.btnRepetir.icon = modo === 2 ? "repeat_one" : "repeat";
    frmPlayer.btnRepetir.hint = ["Repetir: desativado", "Repetir: todos", "Repetir: um"][modo];
    frmPlayer.mnuRepetir.checked = modo > 0;
}

/** Mostra a página "Reproduzindo agora" ou a "Fila de reprodução". */
function pagina(aylis: Aylis, agora: boolean) {
    frmPlayer.pnlAgora.visible = agora;
    frmPlayer.pnlFila.visible = !agora;
    frmPlayer.btnAgora.checked = agora;
    frmPlayer.btnFila.checked = !agora;
}

export function agora_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    pagina(aylis, true);
}

export function fila_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    pagina(aylis, false);
}

export function posicao_seek(aylis: Aylis, ev: AylisEvent) {
    if (typeof ev.args === "number") frmPlayer.vpVideo.seek(ev.args);
}

export function mudo_click(aylis: Aylis, ev: AylisEvent) {
    void ev;
    estado.mudo = !estado.mudo;
    const mudo = estado.mudo;
    frmPlayer.vpVideo.muted = mudo;
    frmPlayer.btnMudo.icon = mudo ? "mute" : "volume";
    frmPlayer.btnMudo.hint = mudo ? "Com som" : "Sem som";
}

export function volume_change(aylis: Aylis, ev: AylisEvent) {
    if (typeof ev.args === "number") frmPlayer.vpVideo.volume = ev.args / 100;
}

export function video_fim(aylis: Aylis, ev: AylisEvent) {
    void ev;
    if (estado.repetir === 2 && estado.atual >= 0) {
        tocar(aylis, estado.atual);
        return;
    }
    const i = proximoIndice();
    if (i >= 0) tocar(aylis, i);
    else {
        estado.tocando = false;
        botaoReproduzir(aylis);
    }
}

export function video_erro(aylis: Aylis, ev: AylisEvent) {
    void ev;
    estado.tocando = false;
    botaoReproduzir(aylis);
    frmPlayer.lblDetalhe.text = "Não foi possível reproduzir este arquivo";
}

export function fila_item_click(aylis: Aylis, ev: AylisEvent) {
    const i = ev.args;
    if (typeof i === "number" && i >= 0 && i < estado.fila.length) tocar(aylis, i);
}

export async function posicao_timer(aylis: Aylis, ev: AylisEvent) {
    void ev;
    if (estado.atual < 0) return;
    const posicao = Number((await frmPlayer.vpVideo.position()) ?? 0);
    const duracao = Number((await frmPlayer.vpVideo.duration()) ?? 0);
    frmPlayer.sbPosicao.duration = duracao;
    frmPlayer.sbPosicao.position = posicao;
    frmPlayer.lblTempo.text = tempo(posicao);
    frmPlayer.lblDuracao.text = tempo(duracao);
}
