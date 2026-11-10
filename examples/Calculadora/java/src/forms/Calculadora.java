// As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
// modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).
//
// - `2 + 3 = = =` repete a última operação (8, 11...).
// - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
//   (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
// - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
// - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
// - Dividir por zero mostra a mensagem e trava até limpar ou digitar.

package forms;

import java.math.BigDecimal;
import java.math.RoundingMode;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;

public final class Calculadora {
    public enum Op { SOMAR, SUBTRAIR, MULTIPLICAR, DIVIDIR }

    public record Conta(String expressao, double resultado) {}

    static final String DIV_ZERO = "Não é possível dividir por zero";
    static final String INVALIDA = "Entrada inválida";

    public List<Double> memoria = new ArrayList<>();
    public List<Conta> historico = new ArrayList<>();
    private String entrada = "";
    private Double acumulado;
    private Op pendente;
    private Op repetirOp;
    private double repetirB;
    private boolean novo;
    private String funcaoTexto;
    public String expressao = "";
    private String erro;

    static String simbolo(Op op) {
        return switch (op) {
            case SOMAR -> "+";
            case SUBTRAIR -> "−";
            case MULTIPLICAR -> "×";
            case DIVIDIR -> "÷";
        };
    }

    /** O resultado de `a op b`; com erro, `erroDaConta` fica com a mensagem. */
    private String erroDaConta;

    private double aplicar(Op op, double a, double b) {
        erroDaConta = null;
        switch (op) {
            case SOMAR: return a + b;
            case SUBTRAIR: return a - b;
            case MULTIPLICAR: return a * b;
            default:
                if (b == 0.0) {
                    erroDaConta = a == 0.0 ? "Resultado indefinido" : DIV_ZERO;
                    return 0.0;
                }
                return a / b;
        }
    }

    /** `1234567` → `1.234.567`. */
    static String agrupar(String digitos) {
        StringBuilder out = new StringBuilder();
        for (int i = 0; i < digitos.length(); i++) {
            if (i > 0 && (digitos.length() - i) % 3 == 0) out.append('.');
            out.append(digitos.charAt(i));
        }
        return out.toString();
    }

    static String semZerosNoFim(String s) {
        int fim = s.length();
        while (fim > 0 && s.charAt(fim - 1) == '0') fim--;
        if (fim > 0 && s.charAt(fim - 1) == '.') fim--;
        return s.substring(0, fim);
    }

    /** O número como a calculadora mostra: vírgula decimal, ponto nos milhares, até 16
     * dígitos significativos e notação científica nos muito grandes ou pequenos. */
    public static String formatar(double v) {
        if (Double.isNaN(v) || Double.isInfinite(v)) return INVALIDA;
        if (v == 0.0) return "0";
        double a = Math.abs(v);
        if (!(a >= 1e-16 && a < 1e16)) {
            String[] partes = String.format(Locale.ROOT, "%.15e", v).split("e");
            String mantissa = semZerosNoFim(partes[0]).replace('.', ',');
            int exp = Integer.parseInt(partes[1].replace("+", ""));
            return mantissa + "e" + (exp >= 0 ? "+" : "") + exp;
        }
        // A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens).
        int e = 0;
        double t = a;
        while (t >= 10.0) { t /= 10.0; e++; }
        while (t < 1.0) { t *= 10.0; e--; }
        int casas = Math.min(Math.max(15 - e, 0), 16);
        // O valor exato do double arredondado (como o printf do C e o format do Rust).
        String s = new BigDecimal(a).setScale(casas, RoundingMode.HALF_EVEN).toPlainString();
        if (s.contains(".")) s = semZerosNoFim(s);
        int ponto = s.indexOf('.');
        String res = agrupar(ponto < 0 ? s : s.substring(0, ponto));
        if (ponto >= 0) res += "," + s.substring(ponto + 1);
        if (v < 0.0) res = "-" + res;
        return res;
    }

    static double paraNumero(String texto) {
        try {
            return Double.parseDouble(texto.replace(',', '.'));
        } catch (NumberFormatException ex) {
            return 0.0;
        }
    }

    private void zerar() {
        entrada = "";
        acumulado = null;
        pendente = null;
        repetirOp = null;
        novo = false;
        funcaoTexto = null;
        expressao = "";
        erro = null;
    }

    public String visor() {
        if (erro != null) return erro;
        if (entrada.isEmpty()) return "0";
        if (novo) return formatar(valor());
        boolean negativo = entrada.startsWith("-");
        String resto = negativo ? entrada.substring(1) : entrada;
        int virgula = resto.indexOf(',');
        String res = (negativo ? "-" : "") + agrupar(virgula < 0 ? resto : resto.substring(0, virgula));
        if (virgula >= 0) res += "," + resto.substring(virgula + 1);
        return res;
    }

    public boolean temErro() {
        return erro != null;
    }

    public double valor() {
        return paraNumero(entrada);
    }

    private void mostrar(double v) {
        entrada = v == 0.0 ? "" : formatar(v).replace(".", "");
        if (Math.abs(v) >= 1e16 || (v != 0.0 && Math.abs(v) < 1e-16)) {
            // Notação científica: guarda o valor exato (o toString volta ao mesmo número).
            entrada = Double.toString(v).replace('.', ',');
        }
    }

    private void sairDoErro() {
        if (erro != null) zerar();
    }

    public void digito(int d) {
        sairDoErro();
        if (novo) {
            entrada = "";
            novo = false;
            funcaoTexto = null;
            if (pendente == null) expressao = "";
        }
        if (entrada.chars().filter(c -> c >= '0' && c <= '9').count() >= 16) return;
        if (entrada.equals("0") || entrada.equals("-0")) entrada = entrada.substring(0, entrada.length() - 1);
        entrada += (char) ('0' + d);
    }

    public void virgula() {
        sairDoErro();
        if (novo) {
            entrada = "";
            novo = false;
            funcaoTexto = null;
        }
        if (!entrada.contains(",")) {
            if (entrada.isEmpty() || entrada.equals("-")) entrada += "0";
            entrada += ",";
        }
    }

    public void apagar() {
        if (erro != null) {
            sairDoErro();
            return;
        }
        if (novo) {
            if (pendente == null) expressao = "";
            return;
        }
        if (!entrada.isEmpty()) entrada = entrada.substring(0, entrada.length() - 1);
        if (entrada.equals("-") || entrada.equals("-0")) entrada = "";
    }

    public void inverterSinal() {
        if (erro != null || entrada.isEmpty()) return;
        entrada = entrada.startsWith("-") ? entrada.substring(1) : "-" + entrada;
        if (novo) {
            double v = valor();
            String dentro = funcaoTexto != null ? funcaoTexto : formatar(-v);
            funcaoTexto = "negate(" + dentro + ")";
            expressaoFuncao();
        }
    }

    public void limparEntrada() {
        if (erro != null) {
            sairDoErro();
            return;
        }
        entrada = "";
        funcaoTexto = null;
        if (pendente == null) expressao = "";
        else if (acumulado != null) expressao = formatar(acumulado) + " " + simbolo(pendente);
        novo = false;
    }

    public void limpar() {
        zerar();
    }

    private String textoDoNumero(double v) {
        return funcaoTexto != null ? funcaoTexto : formatar(v);
    }

    public void operacao(Op op) {
        if (erro != null) return;
        double v = valor();
        if (acumulado != null && pendente != null) {
            if (novo && funcaoTexto == null) {
                // Trocou a operação antes de digitar o segundo número.
                pendente = op;
                expressao = formatar(acumulado) + " " + simbolo(op);
                return;
            }
            // Encadeado: 2 + 3 + → mostra 5.
            double r = aplicar(pendente, acumulado, v);
            if (erroDaConta != null) {
                expressao = formatar(acumulado) + " " + simbolo(pendente) + " " + textoDoNumero(v) + " " + simbolo(op);
                erro = erroDaConta;
                return;
            }
            acumulado = r;
            mostrar(r);
        } else {
            acumulado = v;
        }
        pendente = op;
        repetirOp = null;
        novo = true;
        funcaoTexto = null;
        expressao = formatar(acumulado) + " " + simbolo(op);
    }

    public void igual() {
        if (erro != null) {
            sairDoErro();
            return;
        }
        double v = valor();
        double a, b;
        Op op;
        if (acumulado != null && pendente != null) {
            a = acumulado;
            op = pendente;
            b = v;
        } else if (repetirOp != null) {
            a = v;
            op = repetirOp;
            b = repetirB;
        } else {
            // Só um número: `5 =`.
            expressao = textoDoNumero(v) + " =";
            historico.add(new Conta(expressao, v));
            novo = true;
            funcaoTexto = null;
            return;
        }
        String segundo = pendente != null ? textoDoNumero(b) : formatar(b);
        expressao = formatar(a) + " " + simbolo(op) + " " + segundo + " =";
        double r = aplicar(op, a, b);
        if (erroDaConta == null) {
            historico.add(new Conta(expressao, r));
            mostrar(r);
            repetirOp = op;
            repetirB = b;
        } else {
            erro = erroDaConta;
        }
        acumulado = null;
        pendente = null;
        novo = true;
        funcaoTexto = null;
    }

    public void porcentagem() {
        if (erro != null) return;
        double v = valor();
        double r;
        if (acumulado != null && (pendente == Op.SOMAR || pendente == Op.SUBTRAIR)) r = acumulado * v / 100.0;
        else if (acumulado != null && pendente != null) r = v / 100.0;
        else r = 0.0; // sem conta, o Windows zera
        mostrar(r);
        funcaoTexto = formatar(r);
        novo = true;
        expressaoFuncao();
    }

    public void funcao(String f) {
        if (erro != null) return;
        double v = valor();
        String dentro = textoDoNumero(v);
        double r = 0.0;
        String e = null;
        String texto;
        if (f.equals("inverso")) {
            texto = "1/(" + dentro + ")";
            if (v == 0.0) e = DIV_ZERO; else r = 1.0 / v;
        } else if (f.equals("quadrado")) {
            texto = "sqr(" + dentro + ")";
            r = v * v;
        } else {
            texto = "√(" + dentro + ")";
            if (v < 0.0) e = INVALIDA; else r = Math.sqrt(v);
        }
        funcaoTexto = texto;
        expressaoFuncao();
        if (e == null) mostrar(r); else erro = e;
        novo = true;
    }

    private void expressaoFuncao() {
        String texto = funcaoTexto != null ? funcaoTexto : "";
        expressao = acumulado != null && pendente != null ? formatar(acumulado) + " " + simbolo(pendente) + " " + texto : texto;
    }

    /** Usa um resultado do histórico ou da memória (clicar na lista). */
    public void usar(double valor) {
        sairDoErro();
        mostrar(valor);
        novo = true;
        funcaoTexto = null;
    }

    public void memoriaGuardar() {
        if (erro == null) {
            memoria.add(valor());
            novo = true;
        }
    }

    public void memoriaLer() {
        if (!memoria.isEmpty()) usar(memoria.get(memoria.size() - 1));
    }

    public void memoriaSomar(double sinal) {
        if (erro != null) return;
        double v = valor() * sinal;
        if (!memoria.isEmpty()) memoria.set(memoria.size() - 1, memoria.get(memoria.size() - 1) + v);
        else memoria.add(v);
        novo = true;
    }

    /** Aperta um botão pelo texto dele (o mesmo da tela e do teclado). */
    public boolean tecla(String t) {
        switch (t) {
            case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9" -> digito(t.charAt(0) - '0');
            case ",", "." -> virgula();
            case "+" -> operacao(Op.SOMAR);
            case "−", "-" -> operacao(Op.SUBTRAIR);
            case "×", "*" -> operacao(Op.MULTIPLICAR);
            case "÷", "/" -> operacao(Op.DIVIDIR);
            case "=" -> igual();
            case "%" -> porcentagem();
            case "⌫" -> apagar();
            case "CE" -> limparEntrada();
            case "C" -> limpar();
            case "±" -> inverterSinal();
            case "¹⁄ₓ" -> funcao("inverso");
            case "x²" -> funcao("quadrado");
            case "²√x" -> funcao("raiz");
            case "MC" -> memoria.clear();
            case "MR" -> memoriaLer();
            case "M+" -> memoriaSomar(1.0);
            case "M-" -> memoriaSomar(-1.0);
            case "MS" -> memoriaGuardar();
            default -> {
                return false;
            }
        }
        return true;
    }

    /** O botão de uma tecla, como os atalhos da calculadora do Windows; "historico"
     * mostra ou esconde o histórico. */
    public static String botaoDoTeclado(String tecla, boolean ctrl, boolean shift) {
        if (ctrl) {
            return switch (tecla) {
                case "m" -> "MS";
                case "r" -> "MR";
                case "p" -> "M+";
                case "q" -> "M-";
                case "l" -> "MC";
                case "h" -> "historico";
                default -> null;
            };
        }
        if (shift) {
            switch (tecla) {
                case "=": return "+";
                case "8": return "×";
                case "5": return "%";
                case "2": return "²√x";
                default: break;
            }
        }
        if (tecla.length() == 1 && tecla.charAt(0) >= '0' && tecla.charAt(0) <= '9') return tecla;
        return switch (tecla) {
            case "+" -> "+";
            case "-" -> "−";
            case "*" -> "×";
            case "/" -> "÷";
            case ",", "." -> ",";
            case "%" -> "%";
            case "r" -> "¹⁄ₓ";
            case "q" -> "x²";
            case "@" -> "²√x";
            case "=", "enter" -> "=";
            case "backspace" -> "⌫";
            case "delete" -> "CE";
            case "escape" -> "C";
            case "f9" -> "±";
            default -> null;
        };
    }
}
