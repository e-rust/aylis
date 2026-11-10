// As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
// modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).
//
// - `2 + 3 = = =` repete a última operação (8, 11...).
// - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
//   (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
// - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
// - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
// - Dividir por zero mostra a mensagem e trava até limpar ou digitar.

#pragma once

#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <map>
#include <optional>
#include <string>
#include <utility>
#include <vector>

namespace motor {

enum class Op { Somar, Subtrair, Multiplicar, Dividir };

inline const char* DIV_ZERO = "Não é possível dividir por zero";
inline const char* INVALIDA = "Entrada inválida";

inline std::string simbolo(Op op) {
    switch (op) {
        case Op::Somar: return "+";
        case Op::Subtrair: return "−";
        case Op::Multiplicar: return "×";
        default: return "÷";
    }
}

// O resultado de `a op b`; `erro` fica com a mensagem quando não dá.
inline double aplicar(Op op, double a, double b, std::string& erro) {
    erro.clear();
    switch (op) {
        case Op::Somar: return a + b;
        case Op::Subtrair: return a - b;
        case Op::Multiplicar: return a * b;
        default:
            if (b == 0.0) {
                erro = a == 0.0 ? "Resultado indefinido" : DIV_ZERO;
                return 0.0;
            }
            return a / b;
    }
}

// `1234567` → `1.234.567`.
inline std::string agrupar(const std::string& digitos) {
    std::string out;
    for (size_t i = 0; i < digitos.size(); i++) {
        if (i > 0 && (digitos.size() - i) % 3 == 0) out += '.';
        out += digitos[i];
    }
    return out;
}

inline std::string sem_zeros_no_fim(std::string s) {
    while (!s.empty() && s.back() == '0') s.pop_back();
    if (!s.empty() && s.back() == '.') s.pop_back();
    return s;
}

inline std::string trocar(std::string s, char de, const std::string& para) {
    std::string out;
    for (char c : s) {
        if (c == de) out += para; else out += c;
    }
    return out;
}

// O número como a calculadora mostra: vírgula decimal, ponto nos milhares, até 16 dígitos
// significativos e notação científica nos muito grandes ou pequenos.
inline std::string formatar(double v) {
    if (std::isnan(v) || std::isinf(v)) return INVALIDA;
    if (v == 0.0) return "0";
    double a = std::fabs(v);
    char buf[512];
    if (!(a >= 1e-16 && a < 1e16)) {
        std::snprintf(buf, sizeof buf, "%.15e", v);
        std::string s = buf;
        size_t e = s.find('e');
        std::string mantissa = trocar(sem_zeros_no_fim(s.substr(0, e)), '.', ",");
        int exp = std::atoi(s.c_str() + e + 1);
        return mantissa + "e" + (exp >= 0 ? "+" : "") + std::to_string(exp);
    }
    // A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens).
    int e = 0;
    double t = a;
    while (t >= 10.0) { t /= 10.0; e++; }
    while (t < 1.0) { t *= 10.0; e--; }
    int casas = std::min(std::max(15 - e, 0), 16);
    std::snprintf(buf, sizeof buf, "%.*f", casas, a);
    std::string s = buf;
    if (s.find('.') != std::string::npos) s = sem_zeros_no_fim(s);
    size_t ponto = s.find('.');
    std::string out = agrupar(ponto == std::string::npos ? s : s.substr(0, ponto));
    if (ponto != std::string::npos) out += "," + s.substr(ponto + 1);
    if (v < 0.0) out = "-" + out;
    return out;
}

inline double para_numero(const std::string& texto) {
    std::string s = trocar(texto, ',', ".");
    if (s.empty()) return 0.0;
    char* fim = nullptr;
    double v = std::strtod(s.c_str(), &fim);
    return fim == s.c_str() ? 0.0 : v;
}

struct Conta {
    std::string expressao;
    double resultado;
};

class Calculadora {
public:
    std::vector<double> memoria;
    std::vector<Conta> historico;
    std::string expressao;

    std::string visor() const {
        if (erro_) return *erro_;
        if (entrada_.empty()) return "0";
        if (novo_) return formatar(valor());
        bool negativo = entrada_[0] == '-';
        std::string resto = negativo ? entrada_.substr(1) : entrada_;
        size_t virgula = resto.find(',');
        std::string out = std::string(negativo ? "-" : "") + agrupar(virgula == std::string::npos ? resto : resto.substr(0, virgula));
        if (virgula != std::string::npos) out += "," + resto.substr(virgula + 1);
        return out;
    }

    bool tem_erro() const { return erro_.has_value(); }

    double valor() const { return para_numero(entrada_); }

    void digito(int d) {
        sair_do_erro();
        if (novo_) {
            entrada_.clear();
            novo_ = false;
            funcao_texto_.reset();
            if (!pendente_) expressao.clear();
        }
        if (std::count_if(entrada_.begin(), entrada_.end(), [](char c) { return c >= '0' && c <= '9'; }) >= 16) return;
        if (entrada_ == "0" || entrada_ == "-0") entrada_.pop_back();
        entrada_ += static_cast<char>('0' + d);
    }

    void virgula() {
        sair_do_erro();
        if (novo_) {
            entrada_.clear();
            novo_ = false;
            funcao_texto_.reset();
        }
        if (entrada_.find(',') == std::string::npos) {
            if (entrada_.empty() || entrada_ == "-") entrada_ += "0";
            entrada_ += ",";
        }
    }

    void apagar() {
        if (erro_) { sair_do_erro(); return; }
        if (novo_) {
            if (!pendente_) expressao.clear();
            return;
        }
        if (!entrada_.empty()) entrada_.pop_back();
        if (entrada_ == "-" || entrada_ == "-0") entrada_.clear();
    }

    void inverter_sinal() {
        if (erro_ || entrada_.empty()) return;
        entrada_ = entrada_[0] == '-' ? entrada_.substr(1) : "-" + entrada_;
        if (novo_) {
            double v = valor();
            std::string dentro = funcao_texto_ ? *funcao_texto_ : formatar(-v);
            funcao_texto_ = "negate(" + dentro + ")";
            expressao_funcao();
        }
    }

    void limpar_entrada() {
        if (erro_) { sair_do_erro(); return; }
        entrada_.clear();
        funcao_texto_.reset();
        if (!pendente_) expressao.clear();
        else if (acumulado_) expressao = formatar(*acumulado_) + " " + simbolo(*pendente_);
        novo_ = false;
    }

    void limpar() { zerar(); }

    void operacao(Op op) {
        if (erro_) return;
        double v = valor();
        if (acumulado_ && pendente_) {
            if (novo_ && !funcao_texto_) {
                // Trocou a operação antes de digitar o segundo número.
                pendente_ = op;
                expressao = formatar(*acumulado_) + " " + simbolo(op);
                return;
            }
            // Encadeado: 2 + 3 + → mostra 5.
            std::string e;
            double r = aplicar(*pendente_, *acumulado_, v, e);
            if (!e.empty()) {
                expressao = formatar(*acumulado_) + " " + simbolo(*pendente_) + " " + texto_do_numero(v) + " " + simbolo(op);
                erro_ = e;
                return;
            }
            acumulado_ = r;
            mostrar(r);
        } else {
            acumulado_ = v;
        }
        pendente_ = op;
        repetir_.reset();
        novo_ = true;
        funcao_texto_.reset();
        expressao = formatar(*acumulado_) + " " + simbolo(op);
    }

    void igual() {
        if (erro_) { sair_do_erro(); return; }
        double v = valor();
        double a, b;
        Op op;
        if (acumulado_ && pendente_) {
            a = *acumulado_; op = *pendente_; b = v;
        } else if (repetir_) {
            a = v; op = repetir_->first; b = repetir_->second;
        } else {
            // Só um número: `5 =`.
            expressao = texto_do_numero(v) + " =";
            historico.push_back({expressao, v});
            novo_ = true;
            funcao_texto_.reset();
            return;
        }
        std::string segundo = pendente_ ? texto_do_numero(b) : formatar(b);
        expressao = formatar(a) + " " + simbolo(op) + " " + segundo + " =";
        std::string e;
        double r = aplicar(op, a, b, e);
        if (e.empty()) {
            historico.push_back({expressao, r});
            mostrar(r);
            repetir_ = std::make_pair(op, b);
        } else {
            erro_ = e;
        }
        acumulado_.reset();
        pendente_.reset();
        novo_ = true;
        funcao_texto_.reset();
    }

    void porcentagem() {
        if (erro_) return;
        double v = valor();
        double r;
        if (acumulado_ && pendente_ && (*pendente_ == Op::Somar || *pendente_ == Op::Subtrair)) r = *acumulado_ * v / 100.0;
        else if (acumulado_ && pendente_) r = v / 100.0;
        else r = 0.0;  // sem conta, o Windows zera
        mostrar(r);
        funcao_texto_ = formatar(r);
        novo_ = true;
        expressao_funcao();
    }

    void funcao(const std::string& f) {
        if (erro_) return;
        double v = valor();
        std::string dentro = texto_do_numero(v);
        double r = 0.0;
        std::string e;
        std::string texto;
        if (f == "inverso") {
            texto = "1/(" + dentro + ")";
            if (v == 0.0) e = DIV_ZERO; else r = 1.0 / v;
        } else if (f == "quadrado") {
            texto = "sqr(" + dentro + ")";
            r = v * v;
        } else {
            texto = "√(" + dentro + ")";
            if (v < 0.0) e = INVALIDA; else r = std::sqrt(v);
        }
        funcao_texto_ = texto;
        expressao_funcao();
        if (e.empty()) mostrar(r); else erro_ = e;
        novo_ = true;
    }

    // Usa um resultado do histórico ou da memória (clicar na lista).
    void usar(double valor) {
        sair_do_erro();
        mostrar(valor);
        novo_ = true;
        funcao_texto_.reset();
    }

    void memoria_guardar() {
        if (!erro_) {
            memoria.push_back(valor());
            novo_ = true;
        }
    }

    void memoria_ler() {
        if (!memoria.empty()) usar(memoria.back());
    }

    void memoria_somar(double sinal) {
        if (erro_) return;
        double v = valor() * sinal;
        if (!memoria.empty()) memoria.back() += v; else memoria.push_back(v);
        novo_ = true;
    }

    // Aperta um botão pelo texto dele (o mesmo da tela e do teclado).
    bool tecla(const std::string& t) {
        if (t.size() == 1 && t[0] >= '0' && t[0] <= '9') digito(t[0] - '0');
        else if (t == "," || t == ".") virgula();
        else if (t == "+") operacao(Op::Somar);
        else if (t == "−" || t == "-") operacao(Op::Subtrair);
        else if (t == "×" || t == "*") operacao(Op::Multiplicar);
        else if (t == "÷" || t == "/") operacao(Op::Dividir);
        else if (t == "=") igual();
        else if (t == "%") porcentagem();
        else if (t == "⌫") apagar();
        else if (t == "CE") limpar_entrada();
        else if (t == "C") limpar();
        else if (t == "±") inverter_sinal();
        else if (t == "¹⁄ₓ") funcao("inverso");
        else if (t == "x²") funcao("quadrado");
        else if (t == "²√x") funcao("raiz");
        else if (t == "MC") memoria.clear();
        else if (t == "MR") memoria_ler();
        else if (t == "M+") memoria_somar(1.0);
        else if (t == "M-") memoria_somar(-1.0);
        else if (t == "MS") memoria_guardar();
        else return false;
        return true;
    }

private:
    std::string entrada_;
    std::optional<double> acumulado_;
    std::optional<Op> pendente_;
    std::optional<std::pair<Op, double>> repetir_;
    bool novo_ = false;
    std::optional<std::string> funcao_texto_;
    std::optional<std::string> erro_;

    void zerar() {
        entrada_.clear();
        acumulado_.reset();
        pendente_.reset();
        repetir_.reset();
        novo_ = false;
        funcao_texto_.reset();
        expressao.clear();
        erro_.reset();
    }

    void mostrar(double v) {
        entrada_ = v == 0.0 ? std::string() : trocar(formatar(v), '.', "");
        if (std::fabs(v) >= 1e16 || (v != 0.0 && std::fabs(v) < 1e-16)) {
            // Notação científica: guarda o valor exato (17 dígitos voltam ao mesmo número).
            char buf[64];
            std::snprintf(buf, sizeof buf, "%.16e", v);
            entrada_ = trocar(buf, '.', ",");
        }
    }

    void sair_do_erro() {
        if (erro_) zerar();
    }

    std::string texto_do_numero(double v) const { return funcao_texto_ ? *funcao_texto_ : formatar(v); }

    void expressao_funcao() {
        std::string texto = funcao_texto_ ? *funcao_texto_ : "";
        expressao = acumulado_ && pendente_ ? formatar(*acumulado_) + " " + simbolo(*pendente_) + " " + texto : texto;
    }
};

// O botão de uma tecla, como os atalhos da calculadora do Windows; "historico" mostra ou
// esconde o histórico. Vazio: a tecla não faz nada.
inline std::string botao_do_teclado(const std::string& tecla, bool ctrl, bool shift) {
    if (ctrl) {
        static const std::map<std::string, std::string> m = {{"m", "MS"}, {"r", "MR"}, {"p", "M+"}, {"q", "M-"}, {"l", "MC"}, {"h", "historico"}};
        auto it = m.find(tecla);
        return it == m.end() ? "" : it->second;
    }
    if (shift) {
        static const std::map<std::string, std::string> m = {{"=", "+"}, {"8", "×"}, {"5", "%"}, {"2", "²√x"}};
        auto it = m.find(tecla);
        if (it != m.end()) return it->second;
    }
    if (tecla.size() == 1 && tecla[0] >= '0' && tecla[0] <= '9') return tecla;
    static const std::map<std::string, std::string> m = {
        {"+", "+"}, {"-", "−"}, {"*", "×"}, {"/", "÷"}, {",", ","}, {".", ","}, {"%", "%"}, {"r", "¹⁄ₓ"}, {"q", "x²"},
        {"@", "²√x"}, {"=", "="}, {"enter", "="}, {"backspace", "⌫"}, {"delete", "CE"}, {"escape", "C"}, {"f9", "±"}};
    auto it = m.find(tecla);
    return it == m.end() ? "" : it->second;
}

}  // namespace motor
