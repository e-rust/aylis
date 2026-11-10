"""As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).

- `2 + 3 = = =` repete a última operação (8, 11...).
- `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
  (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
- `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
- Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
- Dividir por zero mostra a mensagem e trava até limpar ou digitar.
"""

import math

SIMBOLOS = {"somar": "+", "subtrair": "−", "multiplicar": "×", "dividir": "÷"}
DIV_ZERO = "Não é possível dividir por zero"
INVALIDA = "Entrada inválida"


def aplicar(op, a, b):
    """O resultado de `a op b`, ou (None, erro)."""
    if op == "somar":
        return a + b, None
    if op == "subtrair":
        return a - b, None
    if op == "multiplicar":
        return a * b, None
    if b == 0.0:
        return None, ("Resultado indefinido" if a == 0.0 else DIV_ZERO)
    return a / b, None


def agrupar(digitos):
    """`1234567` → `1.234.567`."""
    out = ""
    for i, c in enumerate(digitos):
        if i > 0 and (len(digitos) - i) % 3 == 0:
            out += "."
        out += c
    return out


def formatar(v):
    """O número como a calculadora mostra: vírgula decimal, ponto nos milhares, até 16
    dígitos significativos e notação científica nos muito grandes ou pequenos."""
    if math.isnan(v) or math.isinf(v):
        return INVALIDA
    if v == 0.0:
        return "0"
    a = abs(v)
    if not (1e-16 <= a < 1e16):
        mantissa, exp = ("%.15e" % v).split("e")
        mantissa = mantissa.rstrip("0").rstrip(".").replace(".", ",")
        exp = int(exp)
        return "%se%s%d" % (mantissa, "+" if exp >= 0 else "", exp)
    # A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens).
    e, t = 0, a
    while t >= 10.0:
        t /= 10.0
        e += 1
    while t < 1.0:
        t *= 10.0
        e -= 1
    casas = min(max(15 - e, 0), 16)
    s = "%.*f" % (casas, a)
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    inteira, _, decimal = s.partition(".")
    out = agrupar(inteira)
    if decimal:
        out += "," + decimal
    if v < 0.0:
        out = "-" + out
    return out


def para_numero(texto):
    try:
        return float(texto.replace(",", "."))
    except ValueError:
        return 0.0


class Calculadora:
    def __init__(self):
        self.memoria = []
        self.historico = []  # (expressão, resultado)
        self._zerar()

    def _zerar(self):
        self.entrada = ""
        self.acumulado = None
        self.pendente = None
        self.repetir = None  # (op, segundo número)
        self.novo = False
        self.funcao_texto = None
        self.expressao = ""
        self.erro = None

    def visor(self):
        if self.erro is not None:
            return self.erro
        if self.entrada == "":
            return "0"
        if self.novo:
            return formatar(self.valor())
        sinal, resto = ("-", self.entrada[1:]) if self.entrada.startswith("-") else ("", self.entrada)
        inteira, virgula, decimal = resto.partition(",")
        out = sinal + agrupar(inteira)
        if virgula:
            out += "," + decimal
        return out

    def tem_erro(self):
        return self.erro is not None

    def valor(self):
        return para_numero(self.entrada)

    def _mostrar(self, v):
        self.entrada = "" if v == 0.0 else formatar(v).replace(".", "")
        if abs(v) >= 1e16 or (v != 0.0 and abs(v) < 1e-16):
            # Notação científica: guarda o valor exato (17 dígitos voltam ao mesmo número).
            self.entrada = ("%.16e" % v).replace(".", ",")

    def _sair_do_erro(self):
        if self.erro is not None:
            self._zerar()

    def digito(self, d):
        self._sair_do_erro()
        if self.novo:
            self.entrada = ""
            self.novo = False
            self.funcao_texto = None
            if self.pendente is None:
                self.expressao = ""
        if sum(1 for c in self.entrada if c.isdigit()) >= 16:
            return
        if self.entrada in ("0", "-0"):
            self.entrada = self.entrada[:-1]
        self.entrada += str(d)

    def virgula(self):
        self._sair_do_erro()
        if self.novo:
            self.entrada = ""
            self.novo = False
            self.funcao_texto = None
        if "," not in self.entrada:
            if self.entrada in ("", "-"):
                self.entrada += "0"
            self.entrada += ","

    def apagar(self):
        if self.erro is not None:
            self._sair_do_erro()
            return
        if self.novo:
            if self.pendente is None:
                self.expressao = ""
            return
        self.entrada = self.entrada[:-1]
        if self.entrada in ("-", "-0"):
            self.entrada = ""

    def inverter_sinal(self):
        if self.erro is not None or self.entrada == "":
            return
        if self.entrada.startswith("-"):
            self.entrada = self.entrada[1:]
        else:
            self.entrada = "-" + self.entrada
        if self.novo:
            v = self.valor()
            dentro = self.funcao_texto if self.funcao_texto is not None else formatar(-v)
            self.funcao_texto = "negate(%s)" % dentro
            self._expressao_funcao()

    def limpar_entrada(self):
        if self.erro is not None:
            self._sair_do_erro()
            return
        self.entrada = ""
        self.funcao_texto = None
        if self.pendente is None:
            self.expressao = ""
        elif self.acumulado is not None:
            self.expressao = "%s %s" % (formatar(self.acumulado), SIMBOLOS[self.pendente])
        self.novo = False

    def limpar(self):
        self._zerar()

    def _texto_do_numero(self, v):
        return self.funcao_texto if self.funcao_texto is not None else formatar(v)

    def operacao(self, op):
        if self.erro is not None:
            return
        v = self.valor()
        if self.acumulado is not None and self.pendente is not None:
            if self.novo and self.funcao_texto is None:
                # Trocou a operação antes de digitar o segundo número.
                self.pendente = op
                self.expressao = "%s %s" % (formatar(self.acumulado), SIMBOLOS[op])
                return
            # Encadeado: 2 + 3 + → mostra 5.
            r, erro = aplicar(self.pendente, self.acumulado, v)
            if erro is not None:
                self.expressao = "%s %s %s %s" % (formatar(self.acumulado), SIMBOLOS[self.pendente], self._texto_do_numero(v), SIMBOLOS[op])
                self.erro = erro
                return
            self.acumulado = r
            self._mostrar(r)
        else:
            self.acumulado = v
        self.pendente = op
        self.repetir = None
        self.novo = True
        self.funcao_texto = None
        self.expressao = "%s %s" % (formatar(self.acumulado), SIMBOLOS[op])

    def igual(self):
        if self.erro is not None:
            self._sair_do_erro()
            return
        v = self.valor()
        if self.acumulado is not None and self.pendente is not None:
            a, op, b = self.acumulado, self.pendente, v
        elif self.repetir is not None:
            a, (op, b) = v, self.repetir
        else:
            # Só um número: `5 =`.
            self.expressao = "%s =" % self._texto_do_numero(v)
            self.historico.append((self.expressao, v))
            self.novo = True
            self.funcao_texto = None
            return
        segundo = self._texto_do_numero(b) if self.pendente is not None else formatar(b)
        self.expressao = "%s %s %s =" % (formatar(a), SIMBOLOS[op], segundo)
        r, erro = aplicar(op, a, b)
        if erro is None:
            self.historico.append((self.expressao, r))
            self._mostrar(r)
            self.repetir = (op, b)
        else:
            self.erro = erro
        self.acumulado = None
        self.pendente = None
        self.novo = True
        self.funcao_texto = None

    def porcentagem(self):
        if self.erro is not None:
            return
        v = self.valor()
        if self.acumulado is not None and self.pendente in ("somar", "subtrair"):
            r = self.acumulado * v / 100.0
        elif self.acumulado is not None and self.pendente is not None:
            r = v / 100.0
        else:
            r = 0.0  # sem conta, o Windows zera
        self._mostrar(r)
        self.funcao_texto = formatar(r)
        self.novo = True
        self._expressao_funcao()

    def funcao(self, f):
        if self.erro is not None:
            return
        v = self.valor()
        dentro = self._texto_do_numero(v)
        erro = None
        if f == "inverso":
            texto = "1/(%s)" % dentro
            if v == 0.0:
                erro = DIV_ZERO
            else:
                r = 1.0 / v
        elif f == "quadrado":
            texto = "sqr(%s)" % dentro
            r = v * v
        else:
            texto = "√(%s)" % dentro
            if v < 0.0:
                erro = INVALIDA
            else:
                r = math.sqrt(v)
        self.funcao_texto = texto
        self._expressao_funcao()
        if erro is None:
            self._mostrar(r)
        else:
            self.erro = erro
        self.novo = True

    def _expressao_funcao(self):
        texto = self.funcao_texto or ""
        if self.acumulado is not None and self.pendente is not None:
            self.expressao = "%s %s %s" % (formatar(self.acumulado), SIMBOLOS[self.pendente], texto)
        else:
            self.expressao = texto

    def usar(self, valor):
        """Usa um resultado do histórico ou da memória (clicar na lista)."""
        self._sair_do_erro()
        self._mostrar(valor)
        self.novo = True
        self.funcao_texto = None

    def memoria_guardar(self):
        if self.erro is None:
            self.memoria.append(self.valor())
            self.novo = True

    def memoria_ler(self):
        if self.memoria:
            self.usar(self.memoria[-1])

    def memoria_somar(self, sinal):
        if self.erro is not None:
            return
        v = self.valor() * sinal
        if self.memoria:
            self.memoria[-1] += v
        else:
            self.memoria.append(v)
        self.novo = True

    def memoria_limpar(self):
        self.memoria = []

    def tecla(self, t):
        """Aperta um botão pelo texto dele (o mesmo da tela e do teclado)."""
        if t in "0123456789" and len(t) == 1:
            self.digito(int(t))
        elif t in (",", "."):
            self.virgula()
        elif t == "+":
            self.operacao("somar")
        elif t in ("−", "-"):
            self.operacao("subtrair")
        elif t in ("×", "*"):
            self.operacao("multiplicar")
        elif t in ("÷", "/"):
            self.operacao("dividir")
        elif t == "=":
            self.igual()
        elif t == "%":
            self.porcentagem()
        elif t == "⌫":
            self.apagar()
        elif t == "CE":
            self.limpar_entrada()
        elif t == "C":
            self.limpar()
        elif t == "±":
            self.inverter_sinal()
        elif t == "¹⁄ₓ":
            self.funcao("inverso")
        elif t == "x²":
            self.funcao("quadrado")
        elif t == "²√x":
            self.funcao("raiz")
        elif t == "MC":
            self.memoria_limpar()
        elif t == "MR":
            self.memoria_ler()
        elif t == "M+":
            self.memoria_somar(1.0)
        elif t == "M-":
            self.memoria_somar(-1.0)
        elif t == "MS":
            self.memoria_guardar()
        else:
            return False
        return True


TECLAS_CTRL = {"m": "MS", "r": "MR", "p": "M+", "q": "M-", "l": "MC", "h": "historico"}
TECLAS_SHIFT = {"=": "+", "8": "×", "5": "%", "2": "²√x"}
TECLAS = {
    "+": "+", "-": "−", "*": "×", "/": "÷", ",": ",", ".": ",", "%": "%", "r": "¹⁄ₓ", "q": "x²",
    "@": "²√x", "=": "=", "enter": "=", "backspace": "⌫", "delete": "CE", "escape": "C", "f9": "±",
}


def botao_do_teclado(tecla, ctrl, shift):
    """O botão de uma tecla, como os atalhos da calculadora do Windows; "historico"
    mostra ou esconde o histórico."""
    if ctrl:
        return TECLAS_CTRL.get(tecla)
    if shift and tecla in TECLAS_SHIFT:
        return TECLAS_SHIFT[tecla]
    if len(tecla) == 1 and tecla in "0123456789":
        return tecla
    return TECLAS.get(tecla)
