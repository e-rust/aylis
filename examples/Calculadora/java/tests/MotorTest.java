// As contas da calculadora, como a do Windows (os mesmos casos do teste em Rust,
// examples/Calculadora/rust/tests/motor.rs). Rodam pelo `test` do Aylis (o TestRunner).

import dev.aylis.Check;
import dev.aylis.Test;
import forms.Calculadora;

public class MotorTest {
    private static Calculadora conta(String... teclas) {
        Calculadora c = new Calculadora();
        for (String t : teclas) {
            Check.that(c.tecla(t), "tecla " + t);
        }
        return c;
    }

    @Test
    public void contasBasicasEEncadeadas() {
        Check.equal("42", conta("1", "2", "+", "3", "0", "=").visor());
        Check.equal("12 + 30 =", conta("1", "2", "+", "3", "0", "=").expressao);
        // 2 + 3 × 4: a do Windows (padrão) faz na ordem: (2 + 3) × 4.
        Check.equal("20", conta("2", "+", "3", "×", "4", "=").visor());
        Check.equal("3,5", conta("7", "÷", "2", "=").visor());
        Check.equal("0,3", conta("0", ",", "1", "+", "0", ",", "2", "=").visor());
    }

    @Test
    public void igualRepetido() {
        Check.equal("11", conta("2", "+", "3", "=", "=", "=").visor());
    }
}
