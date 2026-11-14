// O executor de testes mínimo do Aylis (o `test` da IDE e do MCP, sem JUnit): acha as classes
// compiladas na pasta passada, roda cada método marcado com `@Test` (sem parâmetros; num
// objeto novo quando não é estático) e imprime uma linha por teste, que o Aylis lê:
//
//     AYLIS_TEST ok forms.MotorTest.soma
//     AYLIS_TEST FAIL forms.MotorTest.divisao esperado 2, veio 3
//
// Sai com 1 se algum falhou.

package dev.aylis;

import java.io.File;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public final class TestRunner {
    private TestRunner() {}

    public static void main(String[] args) throws Exception {
        File root = new File(args.length > 0 ? args[0] : ".");
        List<String> classes = new ArrayList<>();
        collect(root, root, classes);
        classes.sort(Comparator.naturalOrder());
        int failed = 0;
        int passed = 0;
        for (String name : classes) {
            Class<?> type;
            try {
                // Sem inicializar: o `Main` e os forms só rodam código quando um teste os usa.
                type = Class.forName(name, false, TestRunner.class.getClassLoader());
            } catch (Throwable e) {
                continue;
            }
            List<Method> tests = new ArrayList<>();
            for (Method m : type.getDeclaredMethods()) {
                if (m.isAnnotationPresent(Test.class) && m.getParameterCount() == 0) {
                    tests.add(m);
                }
            }
            tests.sort(Comparator.comparing(Method::getName));
            for (Method m : tests) {
                String id = name + "." + m.getName();
                try {
                    m.setAccessible(true);
                    Object target = Modifier.isStatic(m.getModifiers()) ? null : construct(type);
                    m.invoke(target);
                    System.out.println("AYLIS_TEST ok " + id);
                    passed++;
                } catch (InvocationTargetException e) {
                    System.out.println("AYLIS_TEST FAIL " + id + " " + message(e.getCause()));
                    failed++;
                } catch (Throwable e) {
                    System.out.println("AYLIS_TEST FAIL " + id + " " + message(e));
                    failed++;
                }
            }
        }
        System.out.println("AYLIS_TEST_DONE " + passed + " ok, " + failed + " falharam");
        System.exit(failed > 0 ? 1 : 0);
    }

    private static Object construct(Class<?> type) throws Exception {
        var constructor = type.getDeclaredConstructor();
        constructor.setAccessible(true);
        return constructor.newInstance();
    }

    /** A mensagem numa linha só (a linha do teste tem de ficar inteira). */
    private static String message(Throwable e) {
        String text = e.getMessage() == null ? e.getClass().getSimpleName() : e.getMessage();
        return text.replace('\r', ' ').replace('\n', ' ');
    }

    /** Os nomes das classes compiladas em `dir` (sem as do próprio cliente nem as internas). */
    private static void collect(File root, File dir, List<String> out) {
        File[] entries = dir.listFiles();
        if (entries == null) {
            return;
        }
        for (File f : entries) {
            if (f.isDirectory()) {
                collect(root, f, out);
            } else if (f.getName().endsWith(".class") && !f.getName().contains("$")) {
                String rel = root.toPath().relativize(f.toPath()).toString();
                String name = rel.substring(0, rel.length() - ".class".length()).replace(File.separatorChar, '.');
                if (!name.startsWith("dev.aylis.")) {
                    out.add(name);
                }
            }
        }
    }
}
