// O executor de testes mínimo do Aylis para C++ (o `test` da IDE e do MCP): cada teste da pasta
// `tests/` do projeto se registra com `AYLIS_TEST(nome) { ... }` e confere com `AYLIS_CHECK`,
// `AYLIS_CHECK_EQ` e `AYLIS_CHECK_NEAR`. O Aylis compila os testes com um `main` próprio
// (`AYLIS_TEST_MAIN`), que roda todos e imprime uma linha por teste:
//
//     AYLIS_TEST ok soma
//     AYLIS_TEST FAIL divisao esperado 2, veio 3 (tests/motor.cpp:12)
//
// e sai com 1 se algum falhou.

#pragma once

#include <cmath>
#include <cstdio>
#include <functional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace aylis::test {

struct Case {
    std::string name;
    std::function<void()> run;
};

inline std::vector<Case>& cases() {
    static std::vector<Case> all;
    return all;
}

struct Register {
    Register(const char* name, std::function<void()> run) { cases().push_back({name, std::move(run)}); }
};

// Uma conferência que não deu: o executor marca o teste como falho, com a mensagem.
struct Failure : std::runtime_error {
    using std::runtime_error::runtime_error;
};

inline std::string where(const char* file, int line) {
    std::ostringstream s;
    s << " (" << file << ":" << line << ")";
    return s.str();
}

template <typename A, typename B>
void equal(const A& expected, const B& actual, const char* file, int line) {
    if (!(expected == actual)) {
        std::ostringstream s;
        s << "esperado " << expected << ", veio " << actual << where(file, line);
        throw Failure(s.str());
    }
}

inline void near(double expected, double actual, double tolerance, const char* file, int line) {
    if (std::fabs(expected - actual) > tolerance) {
        std::ostringstream s;
        s << "esperado " << expected << " (±" << tolerance << "), veio " << actual << where(file, line);
        throw Failure(s.str());
    }
}

// Roda os testes registrados e devolve o código de saída (1 se algum falhou).
inline int run_all() {
    int passed = 0, failed = 0;
    for (const auto& c : cases()) {
        try {
            c.run();
            std::printf("AYLIS_TEST ok %s\n", c.name.c_str());
            passed++;
        } catch (const std::exception& e) {
            std::string text = e.what();
            for (auto& ch : text) {
                if (ch == '\n' || ch == '\r') ch = ' ';
            }
            std::printf("AYLIS_TEST FAIL %s %s\n", c.name.c_str(), text.c_str());
            failed++;
        } catch (...) {
            std::printf("AYLIS_TEST FAIL %s exceção desconhecida\n", c.name.c_str());
            failed++;
        }
    }
    std::printf("AYLIS_TEST_DONE %d ok, %d falharam\n", passed, failed);
    std::fflush(stdout);
    return failed > 0 ? 1 : 0;
}

}  // namespace aylis::test

#define AYLIS_TEST_CONCAT2(a, b) a##b
#define AYLIS_TEST_CONCAT(a, b) AYLIS_TEST_CONCAT2(a, b)
#define AYLIS_TEST(name)                                                                                  \
    static void AYLIS_TEST_CONCAT(aylis_test_, name)();                                                  \
    static ::aylis::test::Register AYLIS_TEST_CONCAT(aylis_register_, name)(#name, &AYLIS_TEST_CONCAT(aylis_test_, name)); \
    static void AYLIS_TEST_CONCAT(aylis_test_, name)()
#define AYLIS_CHECK(cond) \
    do { if (!(cond)) throw ::aylis::test::Failure(std::string("não confere: ") + #cond + ::aylis::test::where(__FILE__, __LINE__)); } while (0)
#define AYLIS_CHECK_EQ(expected, actual) ::aylis::test::equal((expected), (actual), __FILE__, __LINE__)
#define AYLIS_CHECK_NEAR(expected, actual, tolerance) ::aylis::test::near((expected), (actual), (tolerance), __FILE__, __LINE__)

#ifdef AYLIS_TEST_MAIN
int main() {
    return ::aylis::test::run_all();
}
#endif
