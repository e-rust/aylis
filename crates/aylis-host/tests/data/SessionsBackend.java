// Backend de teste das sessões em Java (aylis-host --bridge ... --sessions): um processo para
// todas as telas, um contador por tela; "boom" lança uma exceção que fica só naquela sessão,
// "lento" demora sem travar as outras e "entrar"/"comum" ligam as telas do mesmo usuário.

import dev.aylis.Aylis;
import dev.aylis.Server;
import dev.aylis.Session;
import dev.aylis.User;
import java.util.List;
import java.util.Map;

public class SessionsBackend {
    static Map<String, Object> button(String name, String handler) {
        return Aylis.obj("class", "Button", "name", name, "text", name, "on_click", handler);
    }

    static void set(Session s, String target, String text) {
        s.send("set", Aylis.obj("target", target, "props", Aylis.obj("text", text)));
    }

    public static void main(String[] args) {
        Map<String, Object> layout = Aylis.obj("class", "Form", "name", "frm", "width", 200, "height", 80, "children", List.of(
                Aylis.obj("class", "Label", "name", "lbl", "text", "-"), button("btn", "somar"), button("boom", "boom"), button("lento", "lento"),
                Aylis.obj("class", "Label", "name", "lblu", "text", "-"), button("entrar", "entrar"), button("comum", "comum")));
        Server.serve(s -> {
            int[] total = {0};
            Aylis.log("SELFTEST sessão " + s.number() + " abriu (pid " + ProcessHandle.current().pid() + ")");
            s.on("somar", ev -> set(s, "frm.lbl", s.number() + ":" + (++total[0])));
            s.on("boom", ev -> {
                throw new IllegalStateException("erro de propósito");
            });
            s.on("lento", ev -> {
                try {
                    Thread.sleep(3000);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
            });
            s.on("entrar", ev -> {
                s.login("ana");
                set(s, "frm.lblu", "ana");
            });
            s.on("comum", ev -> {
                User u = s.user();
                int n;
                synchronized (u.lock()) {
                    n = (int) u.data.getOrDefault("total", 0) + 1;
                    u.data.put("total", n);
                }
                u.broadcast(t -> set(t, "frm.lblu", "u:" + n));
            });
            s.send("load", Aylis.obj("layout", layout));
            s.run("frm");
        }, s -> Aylis.log("SELFTEST sessão " + s.number() + " saiu"));
    }
}
