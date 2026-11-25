// Eventos do form frmCalculadora: o código é seu (o registro dos handlers fica no index.ts
// gerado). Dentro de um handler, o form é a janela do evento.

import type { Aylis } from "../aylis.ts";
import { Calculadora, botaoDoTeclado, formatar } from "../motor.ts";
import { frmCalculadora } from "./components.ts";

const calc = new Calculadora();

/** Os botões que só funcionam sem erro no visor (os da janela do evento). */
function semErro() {
    return [
        frmCalculadora.btnMMais, frmCalculadora.btnMMenos, frmCalculadora.btnMS, frmCalculadora.btnPorcento,
        frmCalculadora.btnInverso, frmCalculadora.btnQuadrado, frmCalculadora.btnRaiz, frmCalculadora.btnDividir,
        frmCalculadora.btnMultiplicar, frmCalculadora.btnSubtrair, frmCalculadora.btnSomar, frmCalculadora.btnSinal,
        frmCalculadora.btnVirgula,
    ];
}

/** Põe o estado da calculadora na tela. */
function atualizar(aylis: Aylis) {
    const visor = calc.visor();
    // O número encolhe para caber, como no Windows.
    const n = [...visor].length;
    const tamanho = calc.temErro() ? 26 : n <= 11 ? 46 : Math.max(Math.floor((46 * 11) / n), 22);
    frmCalculadora.lblVisor.text = visor;
    frmCalculadora.lblVisor.fontSize = tamanho;
    frmCalculadora.lblExpressao.text = calc.expressao;
    const erro = calc.temErro();
    for (const botao of semErro()) botao.enabled = !erro;
    for (const botao of [frmCalculadora.btnMC, frmCalculadora.btnMR]) botao.enabled = !erro && calc.memoria.length > 0;
    frmCalculadora.lvHistorico.items = calc.historico.length === 0
        ? [{ title: "Ainda não há histórico", detail: "" }]
        : [...calc.historico].reverse().map((c) => ({ title: formatar(c.resultado), detail: c.expressao }));
    frmCalculadora.lvMemoria.items = calc.memoria.length === 0
        ? [{ title: "Não há nada salvo na memória", detail: "" }]
        : [...calc.memoria].reverse().map((m) => ({ title: formatar(m), detail: "" }));
}

/** Aperta um botão pelo texto dele. */
function apertar(aylis: Aylis, texto: string) {
    if (calc.tecla(texto)) atualizar(aylis);
}

export async function form_key_down(aylis: Aylis, sender: import("./components.ts").Form, value: import("./components.ts").KeyEvent) {
    // aylis.reply("key", "") engoliria a tecla.
    void sender;
    const botao = botaoDoTeclado(value.key, value.ctrl, value.shift);
    if (botao === "historico") await historico_click(aylis, frmCalculadora.btnHistorico);
    else if (botao !== null) apertar(aylis, botao);
}

export function limpar_historico_click(aylis: Aylis, sender: import("./components.ts").Button) {
    void sender;
    calc.historico = [];
    atualizar(aylis);
}

export function historico_item_click(aylis: Aylis, sender: import("./components.ts").ListView, value: number) {
    void sender;
    // A lista mostra a conta mais nova em cima.
    const i = value;
    if (i >= 0 && i < calc.historico.length) {
        calc.usar(calc.historico[calc.historico.length - 1 - i].resultado);
        atualizar(aylis);
    }
}

export function memoria_item_click(aylis: Aylis, sender: import("./components.ts").ListView, value: number) {
    void sender;
    const i = value;
    if (i >= 0 && i < calc.memoria.length) {
        calc.usar(calc.memoria[calc.memoria.length - 1 - i]);
        atualizar(aylis);
    }
}

export async function historico_click(aylis: Aylis, sender: import("./components.ts").Button) {
    void sender;
    frmCalculadora.pnlLateral.visible = !(await frmCalculadora.pnlLateral.visible);
}

export async function tecla_click(aylis: Aylis, sender: import("./components.ts").Button) {
    apertar(aylis, await sender.text);
}
