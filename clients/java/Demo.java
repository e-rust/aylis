// Backend de demonstração em Java: um campo, um botão e a resposta do backend.
//
//   javac -d clients/java/out clients/java/dev/aylis/Aylis.java clients/java/Demo.java
//   aylis-host -- java -cp clients/java/out Demo
//   aylis-host --png tela.png -- java -cp clients/java/out Demo --selftest

import dev.aylis.Aylis;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

public class Demo {
    public static void main(String[] args) throws Exception {
        boolean selftest = Arrays.asList(args).contains("--selftest");
        Aylis aylis = new Aylis();

        Path layout = Path.of(System.getProperty("aylis.layout", "clients/demo_layout.json"));
        aylis.send("load", Aylis.obj("layout", Aylis.Json.parse(Files.readString(layout, StandardCharsets.UTF_8))));

        aylis.on("hello", ev -> {
            Object name = aylis.call("get", Aylis.obj("target", "edtName", "prop", "text"));
            String who = name == null || name.toString().isEmpty() ? "mundo" : name.toString();
            aylis.send("set", Aylis.obj("target", "lblMsg", "props", Aylis.obj("text", "Olá, " + who + "! (Java)")));
            if (selftest) {
                Aylis.log("SELFTEST java", aylis.call("get", Aylis.obj("target", "lblMsg", "prop", "text")));
                // Fechar pelo backend passa pelo on_close_query, que pergunta (e espera a resposta).
                aylis.send("close", Aylis.obj("target", "frmDemo"));
            }
        });

        // O on_close_query pergunta: a tela só fecha com a resposta (o ev.reply()).
        aylis.on("pode_fechar", ev -> {
            Aylis.log("SELFTEST java pergunta");
            Aylis.log("SELFTEST java " + Aylis.deltaSelftest());
            ev.reply().put("can_close", true);
        });

        aylis.call("run", Aylis.obj("form", "frmDemo"));
        if (selftest) {
            aylis.send("set", Aylis.obj("target", "edtName", "props", Aylis.obj("text", "Ana")));
            aylis.send("click", Aylis.obj("target", "btnHello"));
        }
        aylis.loop();
    }
}
