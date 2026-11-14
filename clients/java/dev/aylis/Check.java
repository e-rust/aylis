// As conferências dos testes do projeto: uma que não confere lança um `AssertionError` com a
// mensagem, e o `TestRunner` marca o teste como falho.

package dev.aylis;

import java.util.Objects;

public final class Check {
    private Check() {}

    /** Confere que `actual` é igual a `expected`. */
    public static void equal(Object expected, Object actual) {
        if (!Objects.equals(expected, actual)) {
            throw new AssertionError("esperado " + expected + ", veio " + actual);
        }
    }

    /** Confere que dois números são iguais até `tolerance`. */
    public static void near(double expected, double actual, double tolerance) {
        if (Math.abs(expected - actual) > tolerance) {
            throw new AssertionError("esperado " + expected + " (±" + tolerance + "), veio " + actual);
        }
    }

    /** Confere que `condition` é verdadeira. */
    public static void that(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }
}
