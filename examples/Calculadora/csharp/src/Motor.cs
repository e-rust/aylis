// As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
// modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).
//
// - `2 + 3 = = =` repete a última operação (8, 11...).
// - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
//   (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
// - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
// - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
// - Dividir por zero mostra a mensagem e trava até limpar ou digitar.

using System.Globalization;
using System.Text;

namespace App;

public enum Op { Somar, Subtrair, Multiplicar, Dividir }

public sealed record Conta(string Expressao, double Resultado);

public sealed class Calculadora
{
    const string DivZero = "Não é possível dividir por zero";
    const string Invalida = "Entrada inválida";
    static readonly CultureInfo Inv = CultureInfo.InvariantCulture;

    public List<double> Memoria { get; set; } = new();
    public List<Conta> Historico { get; set; } = new();
    string entrada = "";
    double? acumulado;
    Op? pendente;
    (Op op, double b)? repetir;
    bool novo;
    string? funcaoTexto;
    public string Expressao { get; private set; } = "";
    string? erro;

    static string Simbolo(Op op) => op switch
    {
        Op.Somar => "+",
        Op.Subtrair => "−",
        Op.Multiplicar => "×",
        _ => "÷",
    };

    /// <summary>O resultado de `a op b`, ou a mensagem de erro.</summary>
    static (double r, string? erro) Aplicar(Op op, double a, double b) => op switch
    {
        Op.Somar => (a + b, null),
        Op.Subtrair => (a - b, null),
        Op.Multiplicar => (a * b, null),
        _ when b == 0.0 => (0, a == 0.0 ? "Resultado indefinido" : DivZero),
        _ => (a / b, null),
    };

    /// <summary>`1234567` → `1.234.567`.</summary>
    static string Agrupar(string digitos)
    {
        var out_ = new StringBuilder();
        for (int i = 0; i < digitos.Length; i++)
        {
            if (i > 0 && (digitos.Length - i) % 3 == 0) out_.Append('.');
            out_.Append(digitos[i]);
        }
        return out_.ToString();
    }

    static string SemZerosNoFim(string s) => s.TrimEnd('0').TrimEnd('.');

    /// <summary>O número como a calculadora mostra: vírgula decimal, ponto nos milhares,
    /// até 16 dígitos significativos e notação científica nos muito grandes ou pequenos.</summary>
    public static string Formatar(double v)
    {
        if (double.IsNaN(v) || double.IsInfinity(v)) return Invalida;
        if (v == 0.0) return "0";
        double a = Math.Abs(v);
        if (!(a >= 1e-16 && a < 1e16))
        {
            var partes = v.ToString("E15", Inv).Split('E');
            var mantissa = SemZerosNoFim(partes[0]).Replace('.', ',');
            int exp = int.Parse(partes[1], Inv);
            return $"{mantissa}e{(exp >= 0 ? "+" : "")}{exp}";
        }
        // A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens).
        int e = 0;
        double t = a;
        while (t >= 10.0) { t /= 10.0; e++; }
        while (t < 1.0) { t *= 10.0; e--; }
        int casas = Math.Clamp(15 - e, 0, 16);
        string s = a.ToString("F" + casas, Inv);
        if (s.Contains('.')) s = SemZerosNoFim(s);
        int ponto = s.IndexOf('.');
        string res = Agrupar(ponto < 0 ? s : s[..ponto]);
        if (ponto >= 0) res += "," + s[(ponto + 1)..];
        if (v < 0.0) res = "-" + res;
        return res;
    }

    static double ParaNumero(string texto) =>
        double.TryParse(texto.Replace(',', '.'), NumberStyles.Float, Inv, out var v) ? v : 0.0;

    void Zerar()
    {
        entrada = "";
        acumulado = null;
        pendente = null;
        repetir = null;
        novo = false;
        funcaoTexto = null;
        Expressao = "";
        erro = null;
    }

    public string Visor()
    {
        if (erro != null) return erro;
        if (entrada == "") return "0";
        if (novo) return Formatar(Valor());
        bool negativo = entrada.StartsWith('-');
        string resto = negativo ? entrada[1..] : entrada;
        int virgula = resto.IndexOf(',');
        string res = (negativo ? "-" : "") + Agrupar(virgula < 0 ? resto : resto[..virgula]);
        if (virgula >= 0) res += "," + resto[(virgula + 1)..];
        return res;
    }

    public bool TemErro => erro != null;

    public double Valor() => ParaNumero(entrada);

    void Mostrar(double v)
    {
        entrada = v == 0.0 ? "" : Formatar(v).Replace(".", "");
        if (Math.Abs(v) >= 1e16 || (v != 0.0 && Math.Abs(v) < 1e-16))
        {
            // Notação científica: guarda o valor exato (17 dígitos voltam ao mesmo número).
            entrada = v.ToString("E16", Inv).Replace('.', ',');
        }
    }

    void SairDoErro()
    {
        if (erro != null) Zerar();
    }

    public void Digito(int d)
    {
        SairDoErro();
        if (novo)
        {
            entrada = "";
            novo = false;
            funcaoTexto = null;
            if (pendente == null) Expressao = "";
        }
        if (entrada.Count(char.IsAsciiDigit) >= 16) return;
        if (entrada == "0" || entrada == "-0") entrada = entrada[..^1];
        entrada += (char)('0' + d);
    }

    public void Virgula()
    {
        SairDoErro();
        if (novo)
        {
            entrada = "";
            novo = false;
            funcaoTexto = null;
        }
        if (!entrada.Contains(','))
        {
            if (entrada == "" || entrada == "-") entrada += "0";
            entrada += ",";
        }
    }

    public void Apagar()
    {
        if (erro != null) { SairDoErro(); return; }
        if (novo)
        {
            if (pendente == null) Expressao = "";
            return;
        }
        if (entrada.Length > 0) entrada = entrada[..^1];
        if (entrada == "-" || entrada == "-0") entrada = "";
    }

    public void InverterSinal()
    {
        if (erro != null || entrada == "") return;
        entrada = entrada.StartsWith('-') ? entrada[1..] : "-" + entrada;
        if (novo)
        {
            double v = Valor();
            string dentro = funcaoTexto ?? Formatar(-v);
            funcaoTexto = $"negate({dentro})";
            ExpressaoFuncao();
        }
    }

    public void LimparEntrada()
    {
        if (erro != null) { SairDoErro(); return; }
        entrada = "";
        funcaoTexto = null;
        if (pendente == null) Expressao = "";
        else if (acumulado != null) Expressao = $"{Formatar(acumulado.Value)} {Simbolo(pendente.Value)}";
        novo = false;
    }

    public void Limpar() => Zerar();

    string TextoDoNumero(double v) => funcaoTexto ?? Formatar(v);

    public void Operacao(Op op)
    {
        if (erro != null) return;
        double v = Valor();
        if (acumulado != null && pendente != null)
        {
            if (novo && funcaoTexto == null)
            {
                // Trocou a operação antes de digitar o segundo número.
                pendente = op;
                Expressao = $"{Formatar(acumulado.Value)} {Simbolo(op)}";
                return;
            }
            // Encadeado: 2 + 3 + → mostra 5.
            var (r, e) = Aplicar(pendente.Value, acumulado.Value, v);
            if (e != null)
            {
                Expressao = $"{Formatar(acumulado.Value)} {Simbolo(pendente.Value)} {TextoDoNumero(v)} {Simbolo(op)}";
                erro = e;
                return;
            }
            acumulado = r;
            Mostrar(r);
        }
        else
        {
            acumulado = v;
        }
        pendente = op;
        repetir = null;
        novo = true;
        funcaoTexto = null;
        Expressao = $"{Formatar(acumulado ?? 0.0)} {Simbolo(op)}";
    }

    public void Igual()
    {
        if (erro != null) { SairDoErro(); return; }
        double v = Valor();
        double a, b;
        Op op;
        if (acumulado != null && pendente != null)
        {
            (a, op, b) = (acumulado.Value, pendente.Value, v);
        }
        else if (repetir != null)
        {
            (a, op, b) = (v, repetir.Value.op, repetir.Value.b);
        }
        else
        {
            // Só um número: `5 =`.
            Expressao = $"{TextoDoNumero(v)} =";
            Historico.Add(new Conta(Expressao, v));
            novo = true;
            funcaoTexto = null;
            return;
        }
        string segundo = pendente != null ? TextoDoNumero(b) : Formatar(b);
        Expressao = $"{Formatar(a)} {Simbolo(op)} {segundo} =";
        var (r, e) = Aplicar(op, a, b);
        if (e == null)
        {
            Historico.Add(new Conta(Expressao, r));
            Mostrar(r);
            repetir = (op, b);
        }
        else
        {
            erro = e;
        }
        acumulado = null;
        pendente = null;
        novo = true;
        funcaoTexto = null;
    }

    public void Porcentagem()
    {
        if (erro != null) return;
        double v = Valor();
        double r;
        if (acumulado != null && (pendente == Op.Somar || pendente == Op.Subtrair)) r = acumulado.Value * v / 100.0;
        else if (acumulado != null && pendente != null) r = v / 100.0;
        else r = 0.0; // sem conta, o Windows zera
        Mostrar(r);
        funcaoTexto = Formatar(r);
        novo = true;
        ExpressaoFuncao();
    }

    public void Funcao(string f)
    {
        if (erro != null) return;
        double v = Valor();
        string dentro = TextoDoNumero(v);
        double r = 0.0;
        string? e = null;
        string texto;
        if (f == "inverso")
        {
            texto = $"1/({dentro})";
            if (v == 0.0) e = DivZero; else r = 1.0 / v;
        }
        else if (f == "quadrado")
        {
            texto = $"sqr({dentro})";
            r = v * v;
        }
        else
        {
            texto = $"√({dentro})";
            if (v < 0.0) e = Invalida; else r = Math.Sqrt(v);
        }
        funcaoTexto = texto;
        ExpressaoFuncao();
        if (e == null) Mostrar(r); else erro = e;
        novo = true;
    }

    void ExpressaoFuncao()
    {
        string texto = funcaoTexto ?? "";
        Expressao = acumulado != null && pendente != null ? $"{Formatar(acumulado.Value)} {Simbolo(pendente.Value)} {texto}" : texto;
    }

    /// <summary>Usa um resultado do histórico ou da memória (clicar na lista).</summary>
    public void Usar(double valor)
    {
        SairDoErro();
        Mostrar(valor);
        novo = true;
        funcaoTexto = null;
    }

    public void MemoriaGuardar()
    {
        if (erro == null)
        {
            Memoria.Add(Valor());
            novo = true;
        }
    }

    public void MemoriaLer()
    {
        if (Memoria.Count > 0) Usar(Memoria[^1]);
    }

    public void MemoriaSomar(double sinal)
    {
        if (erro != null) return;
        double v = Valor() * sinal;
        if (Memoria.Count > 0) Memoria[^1] += v; else Memoria.Add(v);
        novo = true;
    }

    /// <summary>Aperta um botão pelo texto dele (o mesmo da tela e do teclado).</summary>
    public bool Tecla(string t)
    {
        switch (t)
        {
            case "0": case "1": case "2": case "3": case "4": case "5": case "6": case "7": case "8": case "9":
                Digito(t[0] - '0'); break;
            case ",": case ".": Virgula(); break;
            case "+": Operacao(Op.Somar); break;
            case "−": case "-": Operacao(Op.Subtrair); break;
            case "×": case "*": Operacao(Op.Multiplicar); break;
            case "÷": case "/": Operacao(Op.Dividir); break;
            case "=": Igual(); break;
            case "%": Porcentagem(); break;
            case "⌫": Apagar(); break;
            case "CE": LimparEntrada(); break;
            case "C": Limpar(); break;
            case "±": InverterSinal(); break;
            case "¹⁄ₓ": Funcao("inverso"); break;
            case "x²": Funcao("quadrado"); break;
            case "²√x": Funcao("raiz"); break;
            case "MC": Memoria.Clear(); break;
            case "MR": MemoriaLer(); break;
            case "M+": MemoriaSomar(1.0); break;
            case "M-": MemoriaSomar(-1.0); break;
            case "MS": MemoriaGuardar(); break;
            default: return false;
        }
        return true;
    }

    /// <summary>O botão de uma tecla, como os atalhos da calculadora do Windows;
    /// "historico" mostra ou esconde o histórico.</summary>
    public static string? BotaoDoTeclado(string tecla, bool ctrl, bool shift)
    {
        if (ctrl)
        {
            return tecla switch
            {
                "m" => "MS", "r" => "MR", "p" => "M+", "q" => "M-", "l" => "MC", "h" => "historico",
                _ => null,
            };
        }
        if (shift)
        {
            switch (tecla)
            {
                case "=": return "+";
                case "8": return "×";
                case "5": return "%";
                case "2": return "²√x";
            }
        }
        if (tecla.Length == 1 && tecla[0] >= '0' && tecla[0] <= '9') return tecla;
        return tecla switch
        {
            "+" => "+", "-" => "−", "*" => "×", "/" => "÷", "," => ",", "." => ",", "%" => "%",
            "r" => "¹⁄ₓ", "q" => "x²", "@" => "²√x", "=" => "=", "enter" => "=",
            "backspace" => "⌫", "delete" => "CE", "escape" => "C", "f9" => "±",
            _ => null,
        };
    }
}
