// As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
// modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).
//
// - `2 + 3 = = =` repete a última operação (8, 11...).
// - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
//   (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
// - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
// - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
// - Dividir por zero mostra a mensagem e trava até limpar ou digitar.

export type Op = "somar" | "subtrair" | "multiplicar" | "dividir";

const SIMBOLOS: Record<Op, string> = { somar: "+", subtrair: "−", multiplicar: "×", dividir: "÷" };
const DIV_ZERO = "Não é possível dividir por zero";
const INVALIDA = "Entrada inválida";

/** O resultado de `a op b` ou a mensagem de erro. */
function aplicar(op: Op, a: number, b: number): [number, string | null] {
  switch (op) {
    case "somar":
      return [a + b, null];
    case "subtrair":
      return [a - b, null];
    case "multiplicar":
      return [a * b, null];
    default:
      if (b === 0) return [0, a === 0 ? "Resultado indefinido" : DIV_ZERO];
      return [a / b, null];
  }
}

/** `1234567` → `1.234.567`. */
function agrupar(digitos: string): string {
  let out = "";
  for (let i = 0; i < digitos.length; i++) {
    if (i > 0 && (digitos.length - i) % 3 === 0) out += ".";
    out += digitos[i];
  }
  return out;
}

function semZerosNoFim(s: string): string {
  return s.replace(/0+$/, "").replace(/\.$/, "");
}

/** O número como a calculadora mostra: vírgula decimal, ponto nos milhares, até 16
 * dígitos significativos e notação científica nos muito grandes ou pequenos. */
export function formatar(v: number): string {
  if (Number.isNaN(v) || !Number.isFinite(v)) return INVALIDA;
  if (v === 0) return "0";
  const a = Math.abs(v);
  if (!(a >= 1e-16 && a < 1e16)) {
    const [m, e] = v.toExponential(15).split("e");
    const mantissa = semZerosNoFim(m).replace(".", ",");
    const exp = parseInt(e, 10);
    return `${mantissa}e${exp >= 0 ? "+" : ""}${exp}`;
  }
  // A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens).
  let e = 0;
  let t = a;
  while (t >= 10) {
    t /= 10;
    e++;
  }
  while (t < 1) {
    t *= 10;
    e--;
  }
  const casas = Math.min(Math.max(15 - e, 0), 16);
  let s = a.toFixed(casas);
  if (s.includes(".")) s = semZerosNoFim(s);
  const [inteira, decimal] = s.split(".");
  let out = agrupar(inteira);
  if (decimal !== undefined) out += "," + decimal;
  if (v < 0) out = "-" + out;
  return out;
}

function paraNumero(texto: string): number {
  const n = Number(texto.replace(",", "."));
  return Number.isNaN(n) ? 0 : n;
}

export interface Conta {
  expressao: string;
  resultado: number;
}

export class Calculadora {
  memoria: number[] = [];
  historico: Conta[] = [];
  private entrada = "";
  private acumulado: number | null = null;
  private pendente: Op | null = null;
  private repetir: [Op, number] | null = null;
  private novo = false;
  private funcaoTexto: string | null = null;
  expressao = "";
  private erro: string | null = null;

  private zerar() {
    this.entrada = "";
    this.acumulado = null;
    this.pendente = null;
    this.repetir = null;
    this.novo = false;
    this.funcaoTexto = null;
    this.expressao = "";
    this.erro = null;
  }

  visor(): string {
    if (this.erro !== null) return this.erro;
    if (this.entrada === "") return "0";
    if (this.novo) return formatar(this.valor());
    const negativo = this.entrada.startsWith("-");
    const resto = negativo ? this.entrada.slice(1) : this.entrada;
    const virgula = resto.indexOf(",");
    const inteira = virgula < 0 ? resto : resto.slice(0, virgula);
    let out = (negativo ? "-" : "") + agrupar(inteira);
    if (virgula >= 0) out += "," + resto.slice(virgula + 1);
    return out;
  }

  temErro(): boolean {
    return this.erro !== null;
  }

  valor(): number {
    return paraNumero(this.entrada);
  }

  private mostrar(v: number) {
    this.entrada = v === 0 ? "" : formatar(v).split(".").join("");
    if (Math.abs(v) >= 1e16 || (v !== 0 && Math.abs(v) < 1e-16)) {
      // Notação científica: guarda o valor exato (17 dígitos voltam ao mesmo número).
      this.entrada = v.toExponential(16).replace(".", ",");
    }
  }

  private sairDoErro() {
    if (this.erro !== null) this.zerar();
  }

  digito(d: number) {
    this.sairDoErro();
    if (this.novo) {
      this.entrada = "";
      this.novo = false;
      this.funcaoTexto = null;
      if (this.pendente === null) this.expressao = "";
    }
    if ([...this.entrada].filter((c) => c >= "0" && c <= "9").length >= 16) return;
    if (this.entrada === "0" || this.entrada === "-0") this.entrada = this.entrada.slice(0, -1);
    this.entrada += String(d);
  }

  virgula() {
    this.sairDoErro();
    if (this.novo) {
      this.entrada = "";
      this.novo = false;
      this.funcaoTexto = null;
    }
    if (!this.entrada.includes(",")) {
      if (this.entrada === "" || this.entrada === "-") this.entrada += "0";
      this.entrada += ",";
    }
  }

  apagar() {
    if (this.erro !== null) {
      this.sairDoErro();
      return;
    }
    if (this.novo) {
      if (this.pendente === null) this.expressao = "";
      return;
    }
    this.entrada = this.entrada.slice(0, -1);
    if (this.entrada === "-" || this.entrada === "-0") this.entrada = "";
  }

  inverterSinal() {
    if (this.erro !== null || this.entrada === "") return;
    this.entrada = this.entrada.startsWith("-") ? this.entrada.slice(1) : "-" + this.entrada;
    if (this.novo) {
      const v = this.valor();
      const dentro = this.funcaoTexto ?? formatar(-v);
      this.funcaoTexto = `negate(${dentro})`;
      this.expressaoFuncao();
    }
  }

  limparEntrada() {
    if (this.erro !== null) {
      this.sairDoErro();
      return;
    }
    this.entrada = "";
    this.funcaoTexto = null;
    if (this.pendente === null) this.expressao = "";
    else if (this.acumulado !== null) this.expressao = `${formatar(this.acumulado)} ${SIMBOLOS[this.pendente]}`;
    this.novo = false;
  }

  limpar() {
    this.zerar();
  }

  private textoDoNumero(v: number): string {
    return this.funcaoTexto ?? formatar(v);
  }

  operacao(op: Op) {
    if (this.erro !== null) return;
    const v = this.valor();
    if (this.acumulado !== null && this.pendente !== null) {
      if (this.novo && this.funcaoTexto === null) {
        // Trocou a operação antes de digitar o segundo número.
        this.pendente = op;
        this.expressao = `${formatar(this.acumulado)} ${SIMBOLOS[op]}`;
        return;
      }
      // Encadeado: 2 + 3 + → mostra 5.
      const [r, erro] = aplicar(this.pendente, this.acumulado, v);
      if (erro !== null) {
        this.expressao = `${formatar(this.acumulado)} ${SIMBOLOS[this.pendente]} ${this.textoDoNumero(v)} ${SIMBOLOS[op]}`;
        this.erro = erro;
        return;
      }
      this.acumulado = r;
      this.mostrar(r);
    } else {
      this.acumulado = v;
    }
    this.pendente = op;
    this.repetir = null;
    this.novo = true;
    this.funcaoTexto = null;
    this.expressao = `${formatar(this.acumulado)} ${SIMBOLOS[op]}`;
  }

  igual() {
    if (this.erro !== null) {
      this.sairDoErro();
      return;
    }
    const v = this.valor();
    let a: number, op: Op, b: number;
    if (this.acumulado !== null && this.pendente !== null) {
      [a, op, b] = [this.acumulado, this.pendente, v];
    } else if (this.repetir !== null) {
      [a, op, b] = [v, this.repetir[0], this.repetir[1]];
    } else {
      // Só um número: `5 =`.
      this.expressao = `${this.textoDoNumero(v)} =`;
      this.historico.push({ expressao: this.expressao, resultado: v });
      this.novo = true;
      this.funcaoTexto = null;
      return;
    }
    const segundo = this.pendente !== null ? this.textoDoNumero(b) : formatar(b);
    this.expressao = `${formatar(a)} ${SIMBOLOS[op]} ${segundo} =`;
    const [r, erro] = aplicar(op, a, b);
    if (erro === null) {
      this.historico.push({ expressao: this.expressao, resultado: r });
      this.mostrar(r);
      this.repetir = [op, b];
    } else {
      this.erro = erro;
    }
    this.acumulado = null;
    this.pendente = null;
    this.novo = true;
    this.funcaoTexto = null;
  }

  porcentagem() {
    if (this.erro !== null) return;
    const v = this.valor();
    let r: number;
    if (this.acumulado !== null && (this.pendente === "somar" || this.pendente === "subtrair")) r = (this.acumulado * v) / 100;
    else if (this.acumulado !== null && this.pendente !== null) r = v / 100;
    else r = 0; // sem conta, o Windows zera
    this.mostrar(r);
    this.funcaoTexto = formatar(r);
    this.novo = true;
    this.expressaoFuncao();
  }

  funcao(f: "inverso" | "quadrado" | "raiz") {
    if (this.erro !== null) return;
    const v = this.valor();
    const dentro = this.textoDoNumero(v);
    let r = 0;
    let erro: string | null = null;
    let texto: string;
    if (f === "inverso") {
      texto = `1/(${dentro})`;
      if (v === 0) erro = DIV_ZERO;
      else r = 1 / v;
    } else if (f === "quadrado") {
      texto = `sqr(${dentro})`;
      r = v * v;
    } else {
      texto = `√(${dentro})`;
      if (v < 0) erro = INVALIDA;
      else r = Math.sqrt(v);
    }
    this.funcaoTexto = texto;
    this.expressaoFuncao();
    if (erro === null) this.mostrar(r);
    else this.erro = erro;
    this.novo = true;
  }

  private expressaoFuncao() {
    const texto = this.funcaoTexto ?? "";
    this.expressao = this.acumulado !== null && this.pendente !== null ? `${formatar(this.acumulado)} ${SIMBOLOS[this.pendente]} ${texto}` : texto;
  }

  /** Usa um resultado do histórico ou da memória (clicar na lista). */
  usar(valor: number) {
    this.sairDoErro();
    this.mostrar(valor);
    this.novo = true;
    this.funcaoTexto = null;
  }

  memoriaGuardar() {
    if (this.erro === null) {
      this.memoria.push(this.valor());
      this.novo = true;
    }
  }

  memoriaLer() {
    if (this.memoria.length > 0) this.usar(this.memoria[this.memoria.length - 1]);
  }

  memoriaSomar(sinal: number) {
    if (this.erro !== null) return;
    const v = this.valor() * sinal;
    if (this.memoria.length > 0) this.memoria[this.memoria.length - 1] += v;
    else this.memoria.push(v);
    this.novo = true;
  }

  /** Aperta um botão pelo texto dele (o mesmo da tela e do teclado). */
  tecla(t: string): boolean {
    if (t.length === 1 && t >= "0" && t <= "9") this.digito(Number(t));
    else if (t === "," || t === ".") this.virgula();
    else if (t === "+") this.operacao("somar");
    else if (t === "−" || t === "-") this.operacao("subtrair");
    else if (t === "×" || t === "*") this.operacao("multiplicar");
    else if (t === "÷" || t === "/") this.operacao("dividir");
    else if (t === "=") this.igual();
    else if (t === "%") this.porcentagem();
    else if (t === "⌫") this.apagar();
    else if (t === "CE") this.limparEntrada();
    else if (t === "C") this.limpar();
    else if (t === "±") this.inverterSinal();
    else if (t === "¹⁄ₓ") this.funcao("inverso");
    else if (t === "x²") this.funcao("quadrado");
    else if (t === "²√x") this.funcao("raiz");
    else if (t === "MC") this.memoria = [];
    else if (t === "MR") this.memoriaLer();
    else if (t === "M+") this.memoriaSomar(1);
    else if (t === "M-") this.memoriaSomar(-1);
    else if (t === "MS") this.memoriaGuardar();
    else return false;
    return true;
  }
}

const TECLAS_CTRL: Record<string, string> = { m: "MS", r: "MR", p: "M+", q: "M-", l: "MC", h: "historico" };
const TECLAS_SHIFT: Record<string, string> = { "=": "+", "8": "×", "5": "%", "2": "²√x" };
const TECLAS: Record<string, string> = {
  "+": "+", "-": "−", "*": "×", "/": "÷", ",": ",", ".": ",", "%": "%", r: "¹⁄ₓ", q: "x²",
  "@": "²√x", "=": "=", enter: "=", backspace: "⌫", delete: "CE", escape: "C", f9: "±",
};

/** O botão de uma tecla, como os atalhos da calculadora do Windows; "historico" mostra ou
 * esconde o histórico. */
export function botaoDoTeclado(tecla: string, ctrl: boolean, shift: boolean): string | null {
  if (ctrl) return TECLAS_CTRL[tecla] ?? null;
  if (shift && tecla in TECLAS_SHIFT) return TECLAS_SHIFT[tecla];
  if (tecla.length === 1 && tecla >= "0" && tecla <= "9") return tecla;
  return TECLAS[tecla] ?? null;
}
