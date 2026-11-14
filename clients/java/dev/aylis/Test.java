// Marca um método de teste do projeto (pasta `tests/`), que o `test` do Aylis roda pelo
// `TestRunner` (sem JUnit; com o JUnit no projeto, o Aylis usa ele).

package dev.aylis;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
public @interface Test {
}
