// As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
// modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).
//
// - `2 + 3 = = =` repete a última operação (8, 11...).
// - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
//   (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
// - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
// - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
// - Dividir por zero mostra a mensagem e trava até limpar ou digitar.

package forms

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

// Op é uma operação de dois números.
type Op int

const (
	Somar Op = iota
	Subtrair
	Multiplicar
	Dividir
)

const (
	divZero  = "Não é possível dividir por zero"
	invalida = "Entrada inválida"
)

func (op Op) simbolo() string {
	return [...]string{"+", "−", "×", "÷"}[op]
}

// aplicar devolve `a op b`, ou a mensagem de erro.
func aplicar(op Op, a, b float64) (float64, string) {
	switch op {
	case Somar:
		return a + b, ""
	case Subtrair:
		return a - b, ""
	case Multiplicar:
		return a * b, ""
	}
	if b == 0 {
		if a == 0 {
			return 0, "Resultado indefinido"
		}
		return 0, divZero
	}
	return a / b, ""
}

// Conta é uma linha do histórico.
type Conta struct {
	Expressao string
	Resultado float64
}

// agrupar: `1234567` → `1.234.567`.
func agrupar(digitos string) string {
	var out strings.Builder
	for i, c := range digitos {
		if i > 0 && (len(digitos)-i)%3 == 0 {
			out.WriteByte('.')
		}
		out.WriteRune(c)
	}
	return out.String()
}

func semZerosNoFim(s string) string {
	return strings.TrimSuffix(strings.TrimRight(s, "0"), ".")
}

// Formatar mostra o número como a calculadora: vírgula decimal, ponto nos milhares, até
// 16 dígitos significativos e notação científica nos muito grandes ou pequenos.
func Formatar(v float64) string {
	if math.IsNaN(v) || math.IsInf(v, 0) {
		return invalida
	}
	if v == 0 {
		return "0"
	}
	a := math.Abs(v)
	if !(a >= 1e-16 && a < 1e16) {
		partes := strings.SplitN(strconv.FormatFloat(v, 'e', 15, 64), "e", 2)
		mantissa := strings.ReplaceAll(semZerosNoFim(partes[0]), ".", ",")
		exp, _ := strconv.Atoi(partes[1])
		sinal := ""
		if exp >= 0 {
			sinal = "+"
		}
		return fmt.Sprintf("%se%s%d", mantissa, sinal, exp)
	}
	// A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens).
	e, t := 0, a
	for t >= 10 {
		t /= 10
		e++
	}
	for t < 1 {
		t *= 10
		e--
	}
	casas := min(max(15-e, 0), 16)
	s := strconv.FormatFloat(a, 'f', casas, 64)
	if strings.Contains(s, ".") {
		s = semZerosNoFim(s)
	}
	inteira, decimal, temDecimal := strings.Cut(s, ".")
	out := agrupar(inteira)
	if temDecimal {
		out += "," + decimal
	}
	if v < 0 {
		out = "-" + out
	}
	return out
}

func paraNumero(texto string) float64 {
	v, err := strconv.ParseFloat(strings.Replace(texto, ",", ".", 1), 64)
	if err != nil {
		return 0
	}
	return v
}

// Calculadora é o estado da calculadora.
type Calculadora struct {
	Memoria      []float64
	Historico    []Conta
	entrada      string
	acumulado    *float64
	pendente     *Op
	repetir      *Op
	repetirB     float64
	novo         bool
	funcaoTexto  *string
	Expressao    string
	erro         string
	temErroAtivo bool
}

func (c *Calculadora) zerar() {
	c.entrada = ""
	c.acumulado = nil
	c.pendente = nil
	c.repetir = nil
	c.novo = false
	c.funcaoTexto = nil
	c.Expressao = ""
	c.erro = ""
	c.temErroAtivo = false
}

func (c *Calculadora) falhar(e string) {
	c.erro = e
	c.temErroAtivo = true
}

// Visor é o texto do visor grande.
func (c *Calculadora) Visor() string {
	if c.temErroAtivo {
		return c.erro
	}
	if c.entrada == "" {
		return "0"
	}
	if c.novo {
		return Formatar(c.Valor())
	}
	sinal, resto := "", c.entrada
	if strings.HasPrefix(resto, "-") {
		sinal, resto = "-", resto[1:]
	}
	inteira, decimal, temVirgula := strings.Cut(resto, ",")
	out := sinal + agrupar(inteira)
	if temVirgula {
		out += "," + decimal
	}
	return out
}

func (c *Calculadora) TemErro() bool { return c.temErroAtivo }

func (c *Calculadora) Valor() float64 { return paraNumero(c.entrada) }

func (c *Calculadora) mostrar(v float64) {
	if v == 0 {
		c.entrada = ""
	} else {
		c.entrada = strings.ReplaceAll(Formatar(v), ".", "")
	}
	if math.Abs(v) >= 1e16 || (v != 0 && math.Abs(v) < 1e-16) {
		// Notação científica: guarda o valor exato (o menor texto que volta ao número).
		c.entrada = strings.Replace(strconv.FormatFloat(v, 'e', -1, 64), ".", ",", 1)
	}
}

func (c *Calculadora) sairDoErro() {
	if c.temErroAtivo {
		c.zerar()
	}
}

func (c *Calculadora) Digito(d int) {
	c.sairDoErro()
	if c.novo {
		c.entrada = ""
		c.novo = false
		c.funcaoTexto = nil
		if c.pendente == nil {
			c.Expressao = ""
		}
	}
	n := 0
	for _, r := range c.entrada {
		if r >= '0' && r <= '9' {
			n++
		}
	}
	if n >= 16 {
		return
	}
	if c.entrada == "0" || c.entrada == "-0" {
		c.entrada = c.entrada[:len(c.entrada)-1]
	}
	c.entrada += strconv.Itoa(d)
}

func (c *Calculadora) Virgula() {
	c.sairDoErro()
	if c.novo {
		c.entrada = ""
		c.novo = false
		c.funcaoTexto = nil
	}
	if !strings.Contains(c.entrada, ",") {
		if c.entrada == "" || c.entrada == "-" {
			c.entrada += "0"
		}
		c.entrada += ","
	}
}

func (c *Calculadora) Apagar() {
	if c.temErroAtivo {
		c.sairDoErro()
		return
	}
	if c.novo {
		if c.pendente == nil {
			c.Expressao = ""
		}
		return
	}
	if len(c.entrada) > 0 {
		c.entrada = c.entrada[:len(c.entrada)-1]
	}
	if c.entrada == "-" || c.entrada == "-0" {
		c.entrada = ""
	}
}

func (c *Calculadora) InverterSinal() {
	if c.temErroAtivo || c.entrada == "" {
		return
	}
	if strings.HasPrefix(c.entrada, "-") {
		c.entrada = c.entrada[1:]
	} else {
		c.entrada = "-" + c.entrada
	}
	if c.novo {
		v := c.Valor()
		dentro := Formatar(-v)
		if c.funcaoTexto != nil {
			dentro = *c.funcaoTexto
		}
		texto := "negate(" + dentro + ")"
		c.funcaoTexto = &texto
		c.expressaoFuncao()
	}
}

func (c *Calculadora) LimparEntrada() {
	if c.temErroAtivo {
		c.sairDoErro()
		return
	}
	c.entrada = ""
	c.funcaoTexto = nil
	if c.pendente == nil {
		c.Expressao = ""
	} else if c.acumulado != nil {
		c.Expressao = Formatar(*c.acumulado) + " " + c.pendente.simbolo()
	}
	c.novo = false
}

func (c *Calculadora) Limpar() { c.zerar() }

func (c *Calculadora) textoDoNumero(v float64) string {
	if c.funcaoTexto != nil {
		return *c.funcaoTexto
	}
	return Formatar(v)
}

func (c *Calculadora) Operacao(op Op) {
	if c.temErroAtivo {
		return
	}
	v := c.Valor()
	if c.acumulado != nil && c.pendente != nil {
		if c.novo && c.funcaoTexto == nil {
			// Trocou a operação antes de digitar o segundo número.
			c.pendente = &op
			c.Expressao = Formatar(*c.acumulado) + " " + op.simbolo()
			return
		}
		// Encadeado: 2 + 3 + → mostra 5.
		r, e := aplicar(*c.pendente, *c.acumulado, v)
		if e != "" {
			c.Expressao = Formatar(*c.acumulado) + " " + c.pendente.simbolo() + " " + c.textoDoNumero(v) + " " + op.simbolo()
			c.falhar(e)
			return
		}
		c.acumulado = &r
		c.mostrar(r)
	} else {
		c.acumulado = &v
	}
	c.pendente = &op
	c.repetir = nil
	c.novo = true
	c.funcaoTexto = nil
	c.Expressao = Formatar(*c.acumulado) + " " + op.simbolo()
}

func (c *Calculadora) Igual() {
	if c.temErroAtivo {
		c.sairDoErro()
		return
	}
	v := c.Valor()
	var a, b float64
	var op Op
	switch {
	case c.acumulado != nil && c.pendente != nil:
		a, op, b = *c.acumulado, *c.pendente, v
	case c.repetir != nil:
		a, op, b = v, *c.repetir, c.repetirB
	default:
		// Só um número: `5 =`.
		c.Expressao = c.textoDoNumero(v) + " ="
		c.Historico = append(c.Historico, Conta{c.Expressao, v})
		c.novo = true
		c.funcaoTexto = nil
		return
	}
	segundo := Formatar(b)
	if c.pendente != nil {
		segundo = c.textoDoNumero(b)
	}
	c.Expressao = Formatar(a) + " " + op.simbolo() + " " + segundo + " ="
	r, e := aplicar(op, a, b)
	if e == "" {
		c.Historico = append(c.Historico, Conta{c.Expressao, r})
		c.mostrar(r)
		c.repetir = &op
		c.repetirB = b
	} else {
		c.falhar(e)
	}
	c.acumulado = nil
	c.pendente = nil
	c.novo = true
	c.funcaoTexto = nil
}

func (c *Calculadora) Porcentagem() {
	if c.temErroAtivo {
		return
	}
	v := c.Valor()
	var r float64
	switch {
	case c.acumulado != nil && c.pendente != nil && (*c.pendente == Somar || *c.pendente == Subtrair):
		r = *c.acumulado * v / 100
	case c.acumulado != nil && c.pendente != nil:
		r = v / 100
	default:
		r = 0 // sem conta, o Windows zera
	}
	c.mostrar(r)
	texto := Formatar(r)
	c.funcaoTexto = &texto
	c.novo = true
	c.expressaoFuncao()
}

func (c *Calculadora) Funcao(f string) {
	if c.temErroAtivo {
		return
	}
	v := c.Valor()
	dentro := c.textoDoNumero(v)
	var r float64
	e := ""
	var texto string
	switch f {
	case "inverso":
		texto = "1/(" + dentro + ")"
		if v == 0 {
			e = divZero
		} else {
			r = 1 / v
		}
	case "quadrado":
		texto = "sqr(" + dentro + ")"
		r = v * v
	default:
		texto = "√(" + dentro + ")"
		if v < 0 {
			e = invalida
		} else {
			r = math.Sqrt(v)
		}
	}
	c.funcaoTexto = &texto
	c.expressaoFuncao()
	if e == "" {
		c.mostrar(r)
	} else {
		c.falhar(e)
	}
	c.novo = true
}

func (c *Calculadora) expressaoFuncao() {
	texto := ""
	if c.funcaoTexto != nil {
		texto = *c.funcaoTexto
	}
	if c.acumulado != nil && c.pendente != nil {
		c.Expressao = Formatar(*c.acumulado) + " " + c.pendente.simbolo() + " " + texto
	} else {
		c.Expressao = texto
	}
}

// Usar põe um resultado do histórico ou da memória no visor (clicar na lista).
func (c *Calculadora) Usar(valor float64) {
	c.sairDoErro()
	c.mostrar(valor)
	c.novo = true
	c.funcaoTexto = nil
}

func (c *Calculadora) MemoriaGuardar() {
	if !c.temErroAtivo {
		c.Memoria = append(c.Memoria, c.Valor())
		c.novo = true
	}
}

func (c *Calculadora) MemoriaLer() {
	if len(c.Memoria) > 0 {
		c.Usar(c.Memoria[len(c.Memoria)-1])
	}
}

func (c *Calculadora) MemoriaSomar(sinal float64) {
	if c.temErroAtivo {
		return
	}
	v := c.Valor() * sinal
	if len(c.Memoria) > 0 {
		c.Memoria[len(c.Memoria)-1] += v
	} else {
		c.Memoria = append(c.Memoria, v)
	}
	c.novo = true
}

// Tecla aperta um botão pelo texto dele (o mesmo da tela e do teclado).
func (c *Calculadora) Tecla(t string) bool {
	switch t {
	case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
		c.Digito(int(t[0] - '0'))
	case ",", ".":
		c.Virgula()
	case "+":
		c.Operacao(Somar)
	case "−", "-":
		c.Operacao(Subtrair)
	case "×", "*":
		c.Operacao(Multiplicar)
	case "÷", "/":
		c.Operacao(Dividir)
	case "=":
		c.Igual()
	case "%":
		c.Porcentagem()
	case "⌫":
		c.Apagar()
	case "CE":
		c.LimparEntrada()
	case "C":
		c.Limpar()
	case "±":
		c.InverterSinal()
	case "¹⁄ₓ":
		c.Funcao("inverso")
	case "x²":
		c.Funcao("quadrado")
	case "²√x":
		c.Funcao("raiz")
	case "MC":
		c.Memoria = nil
	case "MR":
		c.MemoriaLer()
	case "M+":
		c.MemoriaSomar(1)
	case "M-":
		c.MemoriaSomar(-1)
	case "MS":
		c.MemoriaGuardar()
	default:
		return false
	}
	return true
}

// BotaoDoTeclado é o botão de uma tecla, como os atalhos da calculadora do Windows;
// "historico" mostra ou esconde o histórico.
func BotaoDoTeclado(tecla string, ctrl, shift bool) string {
	if ctrl {
		return map[string]string{"m": "MS", "r": "MR", "p": "M+", "q": "M-", "l": "MC", "h": "historico"}[tecla]
	}
	if shift {
		if b, ok := map[string]string{"=": "+", "8": "×", "5": "%", "2": "²√x"}[tecla]; ok {
			return b
		}
	}
	if len(tecla) == 1 && tecla[0] >= '0' && tecla[0] <= '9' {
		return tecla
	}
	return map[string]string{
		"+": "+", "-": "−", "*": "×", "/": "÷", ",": ",", ".": ",", "%": "%", "r": "¹⁄ₓ", "q": "x²",
		"@": "²√x", "=": "=", "enter": "=", "backspace": "⌫", "delete": "CE", "escape": "C", "f9": "±",
	}[tecla]
}
