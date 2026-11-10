package dev.aylis;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;

/**
 * Cliente do protocolo do aylis para backends em Java (11+), via aylis-host, sem
 * dependências.
 *
 * <p>O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por
 * linha. Os objetos JSON são {@code Map<String, Object>}, as listas {@code List<Object>},
 * os números {@code Double} (ou {@code Long}).
 *
 * <pre>
 * Aylis aylis = new Aylis();
 * aylis.send("load", Aylis.obj("layout", layout));
 * Object name = aylis.call("get", Aylis.obj("target", "edtName", "prop", "text"));
 * aylis.on("btn_salvar_click", ev -> { ... });
 * aylis.run("frmClientes");
 * </pre>
 *
 * Os handlers rodam na thread do run/loop. De outras threads, use send (seguro).
 */
public class Aylis {
    /** Um evento da tela. {@code info}: dados extras de alguns eventos (a URL de quem mandou, no
     *  {@code on_message} do WebView); vazio nos outros. */
    /**
     * Um evento da tela. Num evento que pergunta ({@code on_close_query}, {@code before_post},
     * {@code on_key_down}...), {@code ask} é o número da pergunta, e {@code reply} é a resposta,
     * gravável pelo handler: {@code ev.reply().put("can_close", false)} (fechar), {@code "key"} vazio
     * (engolir a tecla) ou outro caractere (trocar), {@code "data"}/{@code "label"} (o arrasto),
     * {@code "accept"} (soltar aqui). Recusar com mensagem ({@code before_post}):
     * {@code throw new AylisException("Informe o CPF")}. O cliente responde quando o handler termina.
     */
    public record Event(String event, String handler, String form, Map<String, Object> sender, Object args, Map<String, Object> info, Long ask, Map<String, Object> reply) {}

    public static final class AylisException extends RuntimeException {
        public AylisException(String message) {
            super(message);
        }
    }

    /** Um componente de um form, pelo id dele na tela (resolvido quando a instância do form é
     *  montada). As classes de cada componente vêm do Components.java gerado. */
    public static class AylisComponent {
        public final Aylis aylis;
        public final long id;
        public final String name;

        public AylisComponent(Aylis aylis, long id, String name) {
            this.aylis = aylis;
            this.id = id;
            this.name = name;
        }

        protected Object getProp(String prop) {
            return aylis.call("get", obj("target", id, "prop", prop));
        }

        protected void setProp(String prop, Object value) {
            Map<String, Object> props = new LinkedHashMap<>();
            props.put(prop, value);
            aylis.send("set", obj("target", id, "props", props));
        }

        public void setFocus() {
            aylis.send("focus", obj("target", id));
        }
    }

    /** Cria um componente (o construtor da classe gerada). */
    public interface ComponentMaker<T extends AylisComponent> {
        T make(Aylis aylis, long id, String name);
    }

    /** A base dos forms (como o TForm do Delphi): cada form aberto na tela tem a sua instância
     *  (duas janelas do mesmo form, duas instâncias), com os componentes resolvidos ao abrir. A
     *  classe gerada de cada form monta os componentes em {@code bind}. */
    public static class AylisForm extends AylisComponent {
        private final List<AylisComponent> parts = new ArrayList<>();

        public AylisForm(Aylis aylis, long id, String name) {
            super(aylis, id, name);
        }

        /** Monta os componentes a partir da lista da tela (a classe gerada sobrescreve). */
        public void bind(List<Map<String, Object>> list) {}

        private static Map<String, Object> find(List<Map<String, Object>> list, String name) {
            for (Map<String, Object> c : list) if (name.equals(c.get("name"))) return c;
            return null;
        }

        /** O componente do form; o que falta ou mudou de classe: o form não bate com o código. */
        protected <T extends AylisComponent> T resolve(List<Map<String, Object>> list, String name, String cls, ComponentMaker<T> make) {
            Map<String, Object> item = find(list, name);
            if (item == null) throw new AylisException("o form " + this.name + " não bate com o código: falta o componente " + name + " (" + cls + ")");
            if (!cls.equals(item.get("class"))) throw new AylisException("o form " + this.name + " não bate com o código: " + name + " é " + item.get("class") + " no form e " + cls + " no código");
            T part = make.make(aylis, ((Number) item.get("id")).longValue(), name);
            parts.add(part);
            return part;
        }

        /** Um componente que só existe em algumas plataformas (null se não está aqui). */
        protected <T extends AylisComponent> T resolveOptional(List<Map<String, Object>> list, String name, ComponentMaker<T> make) {
            Map<String, Object> item = find(list, name);
            if (item == null) return null;
            T part = make.make(aylis, ((Number) item.get("id")).longValue(), name);
            parts.add(part);
            return part;
        }

        /** O componente deste form com o id (o do evento). */
        public AylisComponent componentById(long id) {
            if (id == this.id) return this;
            for (AylisComponent p : parts) if (p.id == id) return p;
            return null;
        }

        public void show() {
            aylis.send("show", obj("target", id));
        }

        public void close() {
            aylis.send("close", obj("target", id));
        }
    }

    private record FormClass(ComponentMaker<? extends AylisForm> make, Consumer<AylisForm> activate) {}

    private static final Map<String, FormClass> FORM_CLASSES = new HashMap<>();

    /** Registra a classe de um form (o Components.java gerado faz): cada form com esse nome
     *  aberto na tela vira uma instância dela; {@code activate} a torna a atual (o frmX gerado
     *  aponta para a janela cujo evento está rodando). */
    public static void registerFormClass(String name, ComponentMaker<? extends AylisForm> make, Consumer<AylisForm> activate) {
        FORM_CLASSES.put(name.toLowerCase(), new FormClass(make, activate));
    }

    /** As instâncias dos forms, pelo id do form na tela. */
    private final Map<Long, AylisForm> forms = new HashMap<>();

    /** A instância do form da tela (criada e montada na primeira vez), já como a atual; null se
     *  a classe dele não foi registrada. Um form que não bate com o código lança AylisException. */
    @SuppressWarnings("unchecked")
    public AylisForm formInstance(String form, long id) {
        FormClass entry = FORM_CLASSES.get(form.toLowerCase());
        if (entry == null) return null;
        AylisForm instance = forms.get(id);
        if (instance == null) {
            instance = entry.make().make(this, id, form);
            instance.bind((List<Map<String, Object>>) call("components", obj("target", id)));
            forms.put(id, instance);
        }
        entry.activate().accept(instance);
        return instance;
    }

    /** Cria o form na tela e devolve a instância dele (como o Application.CreateForm do Delphi);
     *  {@code another}: mais uma janela do mesmo form. */
    @SuppressWarnings("unchecked")
    public AylisForm createForm(String form, boolean another) {
        Map<String, Object> created = (Map<String, Object>) call("create_form", obj("name", form, "new", another));
        return formInstance(form, ((Number) created.get("id")).longValue());
    }

    private final BufferedReader in = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
    private final PrintStream out = new PrintStream(System.out, true, StandardCharsets.UTF_8);
    private final Map<String, Consumer<Event>> handlers = new HashMap<>();
    private final ArrayDeque<Map<String, Object>> queued = new ArrayDeque<>();
    private long nextId;
    // Esperando um diálogo (messageDlg, showModal): os eventos rodam já (os do modal); e os
    // handlers rodando (um evento que pergunta roda mesmo durante um call).
    private int waiting;
    private int running;
    // A thread que lê (a do loop): um call de outra thread leria a mesma entrada.
    private Thread owner;
    // Numa sessão (Server.serve): as mensagens chegam por aqui e saem pelo servidor.
    private final java.util.concurrent.LinkedBlockingQueue<Map<String, Object>> inbox;
    private final java.util.concurrent.ConcurrentLinkedQueue<Runnable> posted = new java.util.concurrent.ConcurrentLinkedQueue<>();

    public Aylis() {
        inbox = null;
    }

    /** A conexão de uma sessão (o stdin e o stdout são do servidor). */
    Aylis(boolean session) {
        inbox = new java.util.concurrent.LinkedBlockingQueue<>();
    }

    /** Entrega uma mensagem à sessão (o servidor chama). */
    void deliver(Map<String, Object> message) {
        inbox.add(message);
    }

    /** Como o {@code post}, ligado a um form: se ele foi liberado até lá (a janela fechou e saiu),
     *  {@code job} não roda. */
    public void postTo(AylisForm owner, Runnable job) {
        post(() -> {
            if (!freed.contains(owner.id)) job.run();
        });
    }

    // Os forms liberados na tela (o aviso __freed).
    private final java.util.Set<Long> freed = java.util.concurrent.ConcurrentHashMap.newKeySet();

    /** Roda {@code job} na thread do loop desta tela (de qualquer thread). */
    public void post(Runnable job) {
        posted.add(job);
        // Numa sessão, a caixa de entrada acorda o loop; no desktop, a tela devolve o eco (wake).
        if (inbox != null) inbox.add(obj("event", "__post"));
        else send("wake");
    }

    /** Monta um objeto JSON: {@code obj("target", "edtName", "prop", "text")}. */
    public static Map<String, Object> obj(Object... pairs) {
        Map<String, Object> m = new LinkedHashMap<>();
        for (int i = 0; i + 1 < pairs.length; i += 2) {
            m.put((String) pairs[i], pairs[i + 1]);
        }
        return m;
    }

    /** A versão do protocolo do aylis que este cliente fala (o hello). */
    public static final int PROTOCOL = 1;
    private boolean greeted;

    private synchronized void write(Map<String, Object> message) {
        // A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes).
        if (!greeted) {
            greeted = true;
            write(obj("cmd", "hello", "protocol", PROTOCOL, "client", "java", "features", java.util.List.of("done")));
        }
        if (this instanceof Session s) {
            message.put("session", s.number());
            s.server().write(message);
            return;
        }
        out.print(Json.write(message));
        out.print('\n');
        out.flush();
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> read() {
        if (inbox != null) {
            try {
                return inbox.take();
            } catch (InterruptedException e) {
                return null;
            }
        }
        try {
            for (String line; (line = in.readLine()) != null; ) {
                if (line.isBlank()) continue;
                Object v = Json.parse(line);
                if (v instanceof Map) return (Map<String, Object>) v;
            }
        } catch (IOException e) {
            log("erro lendo a tela:", e.getMessage());
        }
        return null;
    }

    private static Map<String, Object> message(String cmd, Map<String, Object> fields) {
        Map<String, Object> m = new LinkedHashMap<>();
        m.put("cmd", cmd);
        if (fields != null) m.putAll(fields);
        return m;
    }

    /** Envia um comando sem esperar resposta (erros chegam como evento "error"). */
    public void send(String cmd, Map<String, Object> fields) {
        write(message(cmd, fields));
    }

    public void send(String cmd) {
        send(cmd, null);
    }

    /** Um trecho marcado no profiler: termina no {@code close}. */
    public interface Mark extends AutoCloseable {
        @Override
        void close();
    }

    /** Marca um trecho no profiler da IDE (a faixa "Marcas"):
     * {@code try (var p = aylis.profile("consulta")) { ... }}. */
    public Mark profile(String name) {
        send("profile_begin", obj("name", name));
        return () -> send("profile_end");
    }

    /** Envia um comando e espera o resultado (só na thread do loop). */
    public Object call(String cmd, Map<String, Object> fields) {
        // A resposta chega pela leitura da thread do loop: de outra thread, as duas leriam a mesma
        // entrada (use post).
        if (owner == null) owner = Thread.currentThread();
        else if (owner != Thread.currentThread()) throw new AylisException("call(\"" + cmd + "\") de outra thread: leia na thread do loop (post) ou antes de iniciar a thread");
        long id = ++nextId;
        Map<String, Object> m = message(cmd, fields);
        m.put("id", id);
        write(m);
        for (;;) {
            Map<String, Object> reply = read();
            if (reply == null) throw new AylisException("a tela foi fechada");
            Object got = reply.get("id");
            if (got instanceof Number n && n.longValue() == id) {
                if (reply.containsKey("error")) throw new AylisException(String.valueOf(reply.get("error")));
                return reply.get("result");
            }
            // Esperando um diálogo, os eventos (os do modal) rodam já; um evento que pergunta também
            // (o post do próprio backend dispara o before_post).
            Object kind = reply.get("event");
            if ("__post".equals(kind) && waiting > 0) {
                runPosted();
                continue;
            }
            if ("ping".equals(kind) && waiting > 0) {
                send("pong"); // a sincronia dos roteiros: o que veio antes já rodou
                continue;
            }
            if (kind != null && !"closed".equals(kind) && !"ping".equals(kind) && !"__post".equals(kind) && (waiting > 0 || (reply.containsKey("ask") && running < 8))) {
                handleEvent(reply);
                continue;
            }
            queued.add(reply);
        }
    }

    /** O valor de um campo decimal (Currency, BCD) como {@code BigDecimal}, exato: o protocolo manda
     *  o texto, e ele nunca passa por {@code double}. {@code null} se o campo é nulo. */
    public java.math.BigDecimal fieldDecimal(String dataSet, String field) {
        Object value = call("data", obj("target", dataSet, "op", "value", "field", field));
        return value == null ? null : new java.math.BigDecimal(value.toString());
    }

    /** Caixa de mensagem que espera, como o {@code MessageDlg} do Delphi: devolve o botão ({@code "yes"},
     *  {@code "no"}, {@code "ok"}...). Enquanto espera, os eventos continuam rodando. */
    public String messageDlg(String text, String kind, String... buttons) {
        return waitFor("message_dlg", obj("text", text, "kind", kind, "buttons", java.util.List.of(buttons.length == 0 ? new String[] {"ok"} : buttons)));
    }

    /** Abre o form como modal e espera ele fechar, como o {@code ShowModal} do Delphi: devolve o resultado. */
    public String showModal(AylisForm form) {
        return waitFor("show_modal", obj("target", form.id));
    }

    private String waitFor(String cmd, Map<String, Object> fields) {
        fields.put("wait", true);
        waiting++;
        // Como o laço do ShowModal do Delphi: o que chegou antes (enquanto outro call esperava) roda já.
        for (int n = queued.size(); n > 0; n--) {
            Map<String, Object> m = queued.poll();
            Object k = m.get("event");
            if ("__post".equals(k)) runPosted();
            else if ("ping".equals(k)) send("pong");
            else if (k != null && !"closed".equals(k) && !"error".equals(k)) handleEvent(m);
            else queued.add(m);
        }
        try {
            return String.valueOf(call(cmd, fields));
        } finally {
            waiting--;
        }
    }

    /** Handler pelo nome usado no layout ("frmClientes.nome": só daquele form). */
    public void on(String name, Consumer<Event> handler) {
        handlers.put(name, handler);
    }

    /** Abre o form e trata os eventos até a tela fechar. */
    public void run(String form) {
        call("run", obj("form", form));
        // A instância do form principal antes do primeiro evento (o código pode usar frmX).
        if (form != null && call("find", obj("name", form)) instanceof Number found) formInstance(form, found.longValue());
        loop();
    }

    /** Trata os eventos até a tela fechar. */
    @SuppressWarnings("unchecked")
    public void loop() {
        for (;;) {
            Map<String, Object> m = queued.isEmpty() ? read() : queued.poll();
            if (m == null) return;
            Object kind = m.get("event");
            if (kind == null) continue;
            if ("closed".equals(kind)) return;
            if ("__post".equals(kind)) {
                runPosted();
                continue;
            }
            // Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes.
            if ("ping".equals(kind)) {
                send("pong");
                continue;
            }
            if ("error".equals(kind)) {
                log("erro na tela:", m.get("message"));
                continue;
            }
            handleEvent(m);
        }
    }

    private void runPosted() {
        for (Runnable job; (job = posted.poll()) != null; ) {
            try {
                job.run();
            } catch (RuntimeException e) {
                log("erro no post", e.getMessage());
            }
        }
    }

    /** Roda o handler do evento. Um evento que pergunta (com {@code ask}) recebe a resposta quando
     *  o handler termina: o {@code ev.reply()} ou, com exceção, a recusa com a mensagem (a tela
     *  mostra), como no BeforePost do Delphi. */
    @SuppressWarnings("unchecked")
    private void handleEvent(Map<String, Object> m) {
        // Forms liberados na tela: as instâncias saem e os postTo deles não rodam.
        if ("__freed".equals(m.get("event"))) {
            if (m.get("forms") instanceof java.util.List<?> ids) {
                for (Object id : ids) {
                    if (id instanceof Number n) {
                        freed.add(n.longValue());
                        forms.remove(n.longValue());
                    }
                }
            }
            return;
        }
        Long ask = m.get("ask") instanceof Number n ? n.longValue() : null;
        Event ev = new Event(String.valueOf(m.get("event")), String.valueOf(m.get("handler")), String.valueOf(m.get("form")),
                m.get("sender") instanceof Map ? (Map<String, Object>) m.get("sender") : Map.of(), m.get("args"),
                m.get("info") instanceof Map ? (Map<String, Object>) m.get("info") : Map.of(), ask, new LinkedHashMap<>());
        Consumer<Event> h = handlers.getOrDefault(ev.form() + "." + ev.handler(), handlers.get(ev.handler()));
        running++;
        try {
            if (h == null) {
                log("evento sem handler:", ev.handler());
            } else {
                // A janela do evento vira a atual (frmX aponta para ela durante o handler).
                if (m.get("form_id") instanceof Number formId) formInstance(ev.form(), formId.longValue());
                h.accept(ev);
            }
            if (ask != null) send("answer", obj("ask", ask, "values", ev.reply()));
        } catch (RuntimeException e) {
            if (ask != null) {
                send("answer", obj("ask", ask, "error", String.valueOf(e.getMessage())));
            } else {
                log("erro no handler", ev.handler(), e.getMessage());
                // A tela mostra o erro ao usuário (como o Application.HandleException do Delphi).
                send("exception", obj("message", String.valueOf(e.getMessage()), "path", ev.form(), "handler", ev.handler()));
            }
        } finally {
            running--;
            // O handler terminou: a tela deixa de mostrar o backend ocupado.
            if (m.get("seq") instanceof Number seq) send("done", obj("seq", seq.longValue()));
        }
    }

    /** Escreve no console do aylis-host (o stdout é do protocolo). */
    public static void log(Object... args) {
        StringBuilder sb = new StringBuilder("[backend]");
        for (Object a : args) sb.append(' ').append(a);
        PrintStream err = new PrintStream(System.err, true, StandardCharsets.UTF_8);
        err.println(sb);
    }

    /**
     * As alterações de um {@code apply_updates}, para o backend gravar do jeito dele, na ordem
     * de gravar (o mestre incluído antes dos itens; as exclusões do mestre por último). Cada
     * alteração é um mapa: {@code ref}, {@code op} ("insert", "update", "delete"),
     * {@code data_set}, {@code key}, {@code old}, {@code new} e, no item incluído sob um mestre
     * também incluído, {@code master}. Ao aceitar o mestre com a chave gerada, os itens dele já
     * recebem a chave antes de serem gravados:
     *
     * <pre>{@code
     * Aylis.Delta delta = new Aylis.Delta(ev.args());
     * for (Map<String, Object> c : delta.changes()) {
     *     if ("insert".equals(c.get("op")) && "cdsPedidos".equals(c.get("data_set")))
     *         delta.accept(c, Aylis.obj("id", inserirPedido(c)));
     * }
     * ev.reply().put("results", delta.results());
     * }</pre>
     */
    public static final class Delta {
        private final String dataSet;
        private final List<Map<String, Object>> changes = new ArrayList<>();
        private final List<Object> results = new ArrayList<>();

        @SuppressWarnings("unchecked")
        public Delta(Object source) {
            Object value = source instanceof String text ? Json.parse(text) : source;
            if (value instanceof String text) value = Json.parse(text);
            List<Object> list;
            if (value instanceof Map<?, ?> m) {
                dataSet = m.get("data_set") == null ? "" : m.get("data_set").toString();
                list = m.get("changes") instanceof List<?> l ? (List<Object>) l : List.of();
            } else {
                dataSet = "";
                list = value instanceof List<?> l ? (List<Object>) l : List.of();
            }
            for (Object o : list) {
                Map<String, Object> c = new LinkedHashMap<>((Map<String, Object>) o);
                c.put("key", new LinkedHashMap<>(c.get("key") instanceof Map<?, ?> k ? (Map<String, Object>) k : Map.of()));
                c.put("new", new LinkedHashMap<>(c.get("new") instanceof Map<?, ?> n ? (Map<String, Object>) n : Map.of()));
                changes.add(c);
            }
        }

        public String dataSet() { return dataSet; }
        public List<Map<String, Object>> changes() { return changes; }
        public List<Object> results() { return results; }

        private List<Map<String, Object>> of(String op, String dataSet) {
            List<Map<String, Object>> out = new ArrayList<>();
            for (Map<String, Object> c : changes)
                if (op.equals(c.get("op")) && (dataSet == null || dataSet.equals(c.get("data_set")))) out.add(c);
            return out;
        }
        public List<Map<String, Object>> inserted() { return of("insert", null); }
        public List<Map<String, Object>> inserted(String dataSet) { return of("insert", dataSet); }
        public List<Map<String, Object>> updated() { return of("update", null); }
        public List<Map<String, Object>> updated(String dataSet) { return of("update", dataSet); }
        public List<Map<String, Object>> deleted() { return of("delete", null); }
        public List<Map<String, Object>> deleted(String dataSet) { return of("delete", dataSet); }
        public List<Map<String, Object>> ofDataSet(String dataSet) {
            List<Map<String, Object>> out = new ArrayList<>();
            for (Map<String, Object> c : changes) if (dataSet.equals(c.get("data_set"))) out.add(c);
            return out;
        }

        private static long ref(Map<String, Object> change) { return ((Number) change.get("ref")).longValue(); }

        /** Gravou; {@code values} são os que o banco definiu (a chave gerada). */
        @SuppressWarnings("unchecked")
        public void accept(Map<String, Object> change, Map<String, Object> values) {
            long ref = ref(change);
            for (Map<String, Object> other : changes) {
                if (!(other.get("master") instanceof Map<?, ?> link) || !(link.get("ref") instanceof Number r) || r.longValue() != ref) continue;
                if (!(link.get("fields") instanceof Map<?, ?> fields)) continue;
                for (Map.Entry<?, ?> f : fields.entrySet()) {
                    String detail = f.getKey().toString(), master = String.valueOf(f.getValue());
                    if (!values.containsKey(master)) continue;
                    for (String part : new String[] {"new", "key"}) {
                        Map<String, Object> map = (Map<String, Object>) other.get(part);
                        if (map.containsKey(detail)) map.put(detail, values.get(master));
                    }
                }
            }
            results.add(values.isEmpty() ? obj("ref", ref) : obj("ref", ref, "values", values));
        }

        public void accept(Map<String, Object> change) { accept(change, Map.of()); }

        /** Não gravou: a mensagem vai para a tela e o registro continua pendente. */
        public void reject(Map<String, Object> change, String message) {
            results.add(obj("ref", ref(change), "error", message));
        }
    }

    /** O delta de exemplo dos selftests dos clientes (o mesmo em todas as linguagens). */
    public static final String SAMPLE_DELTA =
        "{\"data_set\":\"cdsPedidos\",\"changes\":["
        + "{\"ref\":1,\"op\":\"insert\",\"data_set\":\"cdsPedidos\",\"key\":{\"id\":-1},\"new\":{\"id\":-1,\"cliente\":\"Joao\"}},"
        + "{\"ref\":2,\"op\":\"insert\",\"data_set\":\"cdsItens\",\"key\":{\"id\":-2},\"new\":{\"id\":-2,\"pedido_id\":-1},"
        + "\"master\":{\"data_set\":\"cdsPedidos\",\"ref\":1,\"fields\":{\"pedido_id\":\"id\"}}},"
        + "{\"ref\":3,\"op\":\"update\",\"data_set\":\"cdsItens\",\"key\":{\"id\":10},\"old\":{\"produto\":\"A\"},\"new\":{\"produto\":\"B\"}},"
        + "{\"ref\":4,\"op\":\"delete\",\"data_set\":\"cdsPedidos\",\"key\":{\"id\":7},\"old\":{\"id\":7}}]}";

    /** O texto que todos os clientes escrevem ao conferir o {@link Delta} com o {@link #SAMPLE_DELTA}. */
    public static String deltaSelftest() {
        Delta delta = new Delta(SAMPLE_DELTA);
        delta.accept(delta.inserted("cdsPedidos").get(0), obj("id", 501));
        Object item = delta.inserted("cdsItens").get(0).get("new");
        Object pedido = ((Map<?, ?>) item).get("pedido_id");
        String shown = pedido instanceof Number n && n.doubleValue() == Math.rint(n.doubleValue()) ? Long.toString(n.longValue()) : String.valueOf(pedido);
        return "delta " + delta.inserted().size() + " " + delta.updated().size() + " " + delta.deleted().size() + " " + shown + " " + delta.results().size();
    }

    /** JSON mínimo: o bastante para o protocolo. */
    public static final class Json {
        private final String s;
        private int i;

        private Json(String s) {
            this.s = s;
        }

        public static Object parse(String text) {
            Json p = new Json(text);
            p.space();
            return p.value();
        }

        public static String write(Object v) {
            StringBuilder sb = new StringBuilder();
            write(sb, v);
            return sb.toString();
        }

        @SuppressWarnings("unchecked")
        private static void write(StringBuilder sb, Object v) {
            if (v == null) {
                sb.append("null");
            } else if (v instanceof String str) {
                sb.append('"');
                for (char c : str.toCharArray()) {
                    switch (c) {
                        case '"' -> sb.append("\\\"");
                        case '\\' -> sb.append("\\\\");
                        case '\n' -> sb.append("\\n");
                        case '\r' -> sb.append("\\r");
                        case '\t' -> sb.append("\\t");
                        default -> {
                            if (c < 0x20) sb.append(String.format("\\u%04x", (int) c));
                            else sb.append(c);
                        }
                    }
                }
                sb.append('"');
            } else if (v instanceof Map<?, ?> m) {
                sb.append('{');
                boolean first = true;
                for (Map.Entry<?, ?> e : m.entrySet()) {
                    if (!first) sb.append(',');
                    first = false;
                    write(sb, String.valueOf(e.getKey()));
                    sb.append(':');
                    write(sb, e.getValue());
                }
                sb.append('}');
            } else if (v instanceof List<?> l) {
                sb.append('[');
                for (int k = 0; k < l.size(); k++) {
                    if (k > 0) sb.append(',');
                    write(sb, l.get(k));
                }
                sb.append(']');
            } else if (v instanceof Double d && d == Math.rint(d) && !Double.isInfinite(d)) {
                sb.append(d.longValue());
            } else {
                sb.append(v);
            }
        }

        private void space() {
            while (i < s.length() && Character.isWhitespace(s.charAt(i))) i++;
        }

        private Object value() {
            char c = s.charAt(i);
            if (c == '{') {
                i++;
                Map<String, Object> m = new LinkedHashMap<>();
                space();
                if (s.charAt(i) == '}') {
                    i++;
                    return m;
                }
                for (;;) {
                    space();
                    String key = string();
                    space();
                    i++; // ':'
                    space();
                    m.put(key, value());
                    space();
                    if (s.charAt(i++) == '}') return m;
                }
            }
            if (c == '[') {
                i++;
                List<Object> l = new ArrayList<>();
                space();
                if (s.charAt(i) == ']') {
                    i++;
                    return l;
                }
                for (;;) {
                    space();
                    l.add(value());
                    space();
                    if (s.charAt(i++) == ']') return l;
                }
            }
            if (c == '"') return string();
            if (s.startsWith("true", i)) {
                i += 4;
                return Boolean.TRUE;
            }
            if (s.startsWith("false", i)) {
                i += 5;
                return Boolean.FALSE;
            }
            if (s.startsWith("null", i)) {
                i += 4;
                return null;
            }
            int start = i;
            while (i < s.length() && "+-0123456789.eE".indexOf(s.charAt(i)) >= 0) i++;
            String n = s.substring(start, i);
            if (n.contains(".") || n.contains("e") || n.contains("E")) return Double.parseDouble(n);
            return Long.parseLong(n);
        }

        private String string() {
            i++; // '"'
            StringBuilder sb = new StringBuilder();
            for (;;) {
                char c = s.charAt(i++);
                if (c == '"') return sb.toString();
                if (c != '\\') {
                    sb.append(c);
                    continue;
                }
                char e = s.charAt(i++);
                switch (e) {
                    case 'n' -> sb.append('\n');
                    case 'r' -> sb.append('\r');
                    case 't' -> sb.append('\t');
                    case 'b' -> sb.append('\b');
                    case 'f' -> sb.append('\f');
                    case 'u' -> {
                        sb.append((char) Integer.parseInt(s.substring(i, i + 4), 16));
                        i += 4;
                    }
                    default -> sb.append(e);
                }
            }
        }
    }
}
