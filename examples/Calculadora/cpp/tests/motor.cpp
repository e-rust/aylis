// As contas da calculadora, como a do Windows (os mesmos casos do teste em Rust,
// examples/Calculadora/rust/tests/motor.rs). Rodam pelo `test` do Aylis (aylis_test.hpp).

#include <initializer_list>
#include <string>

#include "aylis_test.hpp"
#include "forms/motor.hpp"

static motor::Calculadora conta(std::initializer_list<const char*> teclas) {
    motor::Calculadora c;
    for (const char* t : teclas) {
        AYLIS_CHECK(c.tecla(t));
    }
    return c;
}

AYLIS_TEST(contas_basicas_e_encadeadas) {
    AYLIS_CHECK_EQ(std::string("42"), conta({"1", "2", "+", "3", "0", "="}).visor());
    AYLIS_CHECK_EQ(std::string("12 + 30 ="), conta({"1", "2", "+", "3", "0", "="}).expressao);
    // 2 + 3 × 4: a do Windows (padrão) faz na ordem: (2 + 3) × 4.
    AYLIS_CHECK_EQ(std::string("20"), conta({"2", "+", "3", "×", "4", "="}).visor());
    AYLIS_CHECK_EQ(std::string("3,5"), conta({"7", "÷", "2", "="}).visor());
    AYLIS_CHECK_EQ(std::string("0,3"), conta({"0", ",", "1", "+", "0", ",", "2", "="}).visor());
}

AYLIS_TEST(igual_repetido) {
    AYLIS_CHECK_EQ(std::string("11"), conta({"2", "+", "3", "=", "=", "="}).visor());
}
