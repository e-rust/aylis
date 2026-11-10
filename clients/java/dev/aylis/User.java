package dev.aylis;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;

/**
 * O usuário logado, dividido pelas telas dele ({@link Session#login}). {@link #data} guarda o
 * que vale para todas (o carrinho, as preferências); use {@code synchronized (user.lock())}
 * quando duas telas podem mexer ao mesmo tempo.
 */
public final class User {
    private final String id;
    private final Object lock = new Object();
    final List<Session> sessions = new ArrayList<>();
    public final Map<String, Object> data = new HashMap<>();

    User(String id) {
        this.id = id;
    }

    public String id() {
        return id;
    }

    public Object lock() {
        return lock;
    }

    /** Roda {@code job.accept(tela)} na thread de cada tela do usuário (atualizar todas). */
    public void broadcast(Consumer<Session> job) {
        List<Session> list;
        synchronized (lock) {
            list = new ArrayList<>(sessions);
        }
        for (Session s : list) s.post(() -> job.accept(s));
    }
}
