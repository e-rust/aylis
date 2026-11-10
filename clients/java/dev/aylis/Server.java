package dev.aylis;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;
import java.util.function.Consumer;

/** O processo das sessões: lê o stdin e entrega cada mensagem à tela dela. */
public final class Server {
    private final PrintStream out = new PrintStream(System.out, true, StandardCharsets.UTF_8);
    private final Map<Integer, Session> sessions = new HashMap<>();
    private final Map<String, User> users = new HashMap<>();

    synchronized void write(Map<String, Object> message) {
        out.print(Aylis.Json.write(message));
        out.print('\n');
        out.flush();
    }

    User userOf(String id) {
        synchronized (users) {
            return users.computeIfAbsent(id, User::new);
        }
    }

    void leave(User user, Session session) {
        boolean empty;
        synchronized (user.lock()) {
            user.sessions.remove(session);
            empty = user.sessions.isEmpty();
        }
        if (empty) {
            synchronized (users) {
                users.remove(user.id());
            }
        }
    }

    /**
     * Atende as telas da ponte com {@code --sessions}: {@code started} roda numa thread para
     * cada tela que conecta (monte o estado e chame {@code run}); {@code ended} (pode ser null)
     * quando ela sai.
     */
    public static void serve(Consumer<Session> started, Consumer<Session> ended) {
        new Server().run(started, ended);
    }

    @SuppressWarnings("unchecked")
    private void run(Consumer<Session> started, Consumer<Session> ended) {
        BufferedReader in = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
        try {
            for (String line; (line = in.readLine()) != null; ) {
                if (line.isBlank() || !(Aylis.Json.parse(line) instanceof Map<?, ?> parsed)) continue;
                Map<String, Object> message = (Map<String, Object>) parsed;
                if (!(message.remove("session") instanceof Number n)) continue;
                int number = n.intValue();
                Object kind = message.get("event");
                if ("session_open".equals(kind)) {
                    Session session = new Session(this, number);
                    synchronized (sessions) {
                        sessions.put(number, session);
                    }
                    Thread t = new Thread(() -> {
                        try {
                            started.accept(session);
                        } catch (RuntimeException e) {
                            Aylis.log("erro na sessão", number, e.getMessage());
                        } finally {
                            try {
                                if (ended != null) ended.accept(session);
                            } catch (RuntimeException e) {
                                Aylis.log("erro em ended da sessão", number, e.getMessage());
                            }
                            session.logout();
                            synchronized (sessions) {
                                sessions.remove(number);
                            }
                        }
                    }, "sessao-" + number);
                    t.setDaemon(true);
                    t.start();
                    continue;
                }
                Session target;
                synchronized (sessions) {
                    target = sessions.get(number);
                }
                if (target != null) target.deliver("session_close".equals(kind) ? Aylis.obj("event", "closed") : message);
            }
        } catch (IOException e) {
            Aylis.log("erro lendo as telas:", e.getMessage());
        }
    }
}
