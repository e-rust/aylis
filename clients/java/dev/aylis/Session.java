package dev.aylis;

/**
 * Uma tela da ponte com {@code --sessions} ({@code aylis-host --bridge ... --sessions}): um
 * processo atende todas as telas e cada uma roda numa thread própria (um handler demorado não
 * trava as outras; uma exceção fica nela). Guarde o estado da tela em variáveis do
 * {@code started} passado ao {@link Server#serve} (nunca em campos estáticos, que são de
 * todas). A API é a do {@link Aylis}.
 */
public final class Session extends Aylis {
    private final Server server;
    private final int number;
    private User user;

    Session(Server server, int number) {
        super(true);
        this.server = server;
        this.number = number;
    }

    /** O número da tela na ponte. */
    public int number() {
        return number;
    }

    public Server server() {
        return server;
    }

    /** O usuário logado (o mesmo nas telas dele); null antes do {@link #login}. */
    public User user() {
        return user;
    }

    /** Liga esta tela ao usuário {@code id}: as telas do mesmo usuário dividem o mesmo {@link User}. */
    public void login(String id) {
        logout();
        user = server.userOf(id);
        synchronized (user.lock()) {
            user.sessions.add(this);
        }
    }

    /** Solta o usuário desta tela (sozinho, ao fechar). */
    public void logout() {
        if (user != null) server.leave(user, this);
        user = null;
    }
}
