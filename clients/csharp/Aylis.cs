// Cliente do protocolo do aylis para backends em C# (.NET 6+), via aylis-host.
//
// O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por linha.
//
//     var aylis = new Aylis();
//     aylis.Send("load", new() { ["layout"] = layout });
//     var name = aylis.Call("get", new() { ["target"] = "edtName", ["prop"] = "text" });
//     aylis.On("btn_salvar_click", ev => { ... });
//     aylis.Run("frmClientes");
//
// Os handlers rodam na thread do Run/Loop. De outras threads, use Send (seguro).

using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace AylisClient;

public sealed class AylisException(string message) : Exception(message);

/// <summary>Um evento da tela.</summary>
/// <summary>Um evento da tela. <c>Info</c>: dados extras de alguns eventos (a URL de quem
/// mandou, no <c>on_message</c> do WebView); nulo nos outros.</summary>
public sealed record AylisEvent(string Event, string Handler, string Form, string SenderName, string SenderClass, JsonNode? Args, JsonNode? Info = null)
{
    /// <summary>Num evento que pergunta (<c>on_close_query</c>, <c>before_post</c>, <c>on_key_down</c>...), o
    /// número da pergunta: a tela espera a resposta, que o cliente manda quando o handler termina.</summary>
    public long? Ask { get; init; }

    /// <summary>A resposta de um evento que pergunta, gravável pelo handler: <c>ev.Reply["can_close"] = false</c>
    /// (fechar), <c>ev.Reply["key"] = ""</c> (engolir a tecla), <c>ev.Reply["key"] = "."</c> (trocar o caractere),
    /// <c>data</c>/<c>label</c> (o arrasto), <c>accept</c> (soltar aqui). Recusar com mensagem
    /// (<c>before_post</c>): <c>throw new Exception("Informe o CPF")</c>.</summary>
    public JsonObject Reply { get; } = new();
}

/// <summary>Um componente de um form, pelo id dele na tela (resolvido quando a instância do form
/// é montada). As classes de cada componente vêm do Components.cs gerado.</summary>
public class AylisComponent(Aylis aylis, long id, string name)
{
    public Aylis Aylis { get; } = aylis;
    public long Id { get; } = id;
    public string Name { get; } = name;

    protected JsonNode? GetProp(string prop) => Aylis.Call("get", new() { ["target"] = Id, ["prop"] = prop });

    protected void SetProp(string prop, JsonNode? value) => Aylis.Send("set", new() { ["target"] = Id, ["props"] = new JsonObject { [prop] = value } });

    public void SetFocus() => Aylis.Send("focus", new() { ["target"] = Id });
}

/// <summary>A base dos forms: cada form aberto na tela tem a sua
/// instância (duas janelas do mesmo form, duas instâncias), com os componentes resolvidos ao
/// abrir. A classe gerada de cada form monta os componentes em <c>Bind</c>.</summary>
public class AylisForm(Aylis aylis, long id, string name) : AylisComponent(aylis, id, name)
{
    private readonly List<AylisComponent> parts = [];

    /// <summary>Monta os componentes a partir da lista da tela (a classe gerada sobrescreve).</summary>
    public virtual void Bind(JsonArray list) { }

    private static JsonObject? Find(JsonArray list, string name) => list.OfType<JsonObject>().FirstOrDefault(c => c["name"]?.ToString() == name);

    /// <summary>O componente do form; o que falta ou mudou de classe: o form não bate com o código.</summary>
    protected T Resolve<T>(JsonArray list, string name, string cls, Func<Aylis, long, string, T> make) where T : AylisComponent
    {
        var item = Find(list, name) ?? throw new AylisException($"o form {Name} não bate com o código: falta o componente {name} ({cls})");
        if (item["class"]?.ToString() != cls) throw new AylisException($"o form {Name} não bate com o código: {name} é {item["class"]} no form e {cls} no código");
        var part = make(Aylis, item["id"]!.GetValue<long>(), name);
        parts.Add(part);
        return part;
    }

    /// <summary>Um componente que só existe em algumas plataformas (null se não está aqui).</summary>
    protected T? ResolveOptional<T>(JsonArray list, string name, Func<Aylis, long, string, T> make) where T : AylisComponent
    {
        if (Find(list, name) is not JsonObject item) return null;
        var part = make(Aylis, item["id"]!.GetValue<long>(), name);
        parts.Add(part);
        return part;
    }

    /// <summary>O componente deste form com o id (o do evento).</summary>
    public AylisComponent? ComponentById(long id) => id == Id ? this : parts.FirstOrDefault(p => p.Id == id);

    public void Show() => Aylis.Send("show", new() { ["target"] = Id });

    public void Close() => Aylis.Send("close", new() { ["target"] = Id });
}

public class Aylis
{
    /// <summary>As classes dos forms registradas (o Components.cs gerado registra): como criar
    /// a instância e como torná-la a atual (o frmX gerado aponta para a janela do evento).</summary>
    private static readonly Dictionary<string, (Func<Aylis, long, string, AylisForm> Make, Action<AylisForm> Activate)> formClasses = new(StringComparer.OrdinalIgnoreCase);

    /// <summary>Registra a classe de um form: cada form com esse nome aberto na tela vira uma
    /// instância dela.</summary>
    public static void RegisterFormClass(string name, Func<Aylis, long, string, AylisForm> make, Action<AylisForm> activate) => formClasses[name] = (make, activate);

    /// <summary>As instâncias dos forms, pelo id do form na tela.</summary>
    private readonly Dictionary<long, AylisForm> forms = new();

    /// <summary>A instância do form da tela (criada e montada na primeira vez), já como a atual;
    /// null se a classe dele não foi registrada. Um form que não bate com o código lança
    /// AylisException.</summary>
    public AylisForm? FormInstance(string form, long id)
    {
        if (!formClasses.TryGetValue(form, out var entry)) return null;
        if (!forms.TryGetValue(id, out var instance))
        {
            instance = entry.Make(this, id, form);
            instance.Bind((JsonArray)Call("components", new() { ["target"] = id })!.DeepClone());
            forms[id] = instance;
        }
        entry.Activate(instance);
        return instance;
    }

    /// <summary>Cria o form na tela e devolve a instância dele; <c>another</c>: mais uma janela do mesmo form.</summary>
    public AylisForm? CreateForm(string form, bool another = false)
    {
        var created = Call("create_form", new() { ["name"] = form, ["new"] = another });
        return FormInstance(form, created!["id"]!.GetValue<long>());
    }

    private readonly TextReader? input;
    private readonly TextWriter? output;
    // Numa sessão (Server.Serve): as mensagens chegam por aqui e saem pelo servidor.
    private readonly System.Collections.Concurrent.BlockingCollection<JsonObject?>? inbox;
    private readonly System.Collections.Concurrent.ConcurrentQueue<Action> posted = new();
    private readonly object writeLock = new();
    private readonly Dictionary<string, Action<AylisEvent>> handlers = new();
    private readonly Queue<JsonObject> queued = new();
    private int nextId;
    // Esperando um diálogo (MessageDlg, ShowModal): os eventos rodam já (os do modal); e os
    // handlers rodando (um evento que pergunta roda mesmo durante um Call).
    private int waiting;
    private int running;
    // A thread que lê (a do Loop): um Call de outra thread leria a mesma entrada.
    private int owner;

    public Aylis()
    {
        // UTF-8 sem BOM nos dois sentidos (o console do Windows usaria a página de código).
        var utf8 = new UTF8Encoding(false);
        input = new StreamReader(Console.OpenStandardInput(), utf8);
        output = new StreamWriter(Console.OpenStandardOutput(), utf8) { AutoFlush = true, NewLine = "\n" };
    }

    /// <summary>A conexão de uma sessão (sem o stdin e o stdout, que são do servidor).</summary>
    private protected Aylis(bool session)
    {
        _ = session;
        inbox = new System.Collections.Concurrent.BlockingCollection<JsonObject?>();
    }

    /// <summary>Entrega uma mensagem à sessão (o servidor chama); null: a tela saiu.</summary>
    internal void Deliver(JsonObject? message) => inbox?.Add(message);

    /// <summary>Roda <c>action</c> na thread do Loop desta tela (de qualquer thread).</summary>
    public void Post(Action action)
    {
        posted.Enqueue(action);
        // Numa sessão, a caixa de entrada acorda o Loop; no desktop, a tela devolve o eco (wake).
        if (inbox != null) inbox.Add(new JsonObject { ["event"] = "__post" });
        else Send("wake");
    }

    /// <summary>Como o <c>Post</c>, ligado a um form: se ele foi liberado até lá (a janela fechou e
    /// saiu), <c>action</c> não roda.</summary>
    public void PostTo(AylisForm owner, Action action) => Post(() => { if (!freed.Contains(owner.Id)) action(); });

    /// <summary>Os forms liberados na tela (o aviso <c>__freed</c>).</summary>
    private readonly HashSet<long> freed = new();

    /// <summary>A versão do protocolo do aylis que este cliente fala (o hello).</summary>
    public const int Protocol = 1;
    private bool greeted;

    private void Write(JsonObject message)
    {
        // A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes).
        if (!greeted)
        {
            greeted = true;
            Write(new JsonObject { ["cmd"] = "hello", ["protocol"] = Protocol, ["client"] = "csharp", ["features"] = new JsonArray("done") });
        }
        if (this is Session session)
        {
            message["session"] = session.Number;
            session.Server.Write(message);
            return;
        }
        var text = message.ToJsonString();
        lock (writeLock) output!.WriteLine(text);
    }

    private JsonObject? Read()
    {
        if (inbox != null) return inbox.Take();
        for (string? line; (line = input!.ReadLine()) != null;)
        {
            if (line.Trim().Length == 0) continue;
            if (JsonNode.Parse(line) is JsonObject o) return o;
        }
        return null;
    }

    private static JsonObject Message(string cmd, JsonObject? fields)
    {
        var message = new JsonObject { ["cmd"] = cmd };
        if (fields != null)
            foreach (var (k, v) in fields) message[k] = v?.DeepClone();
        return message;
    }

    /// <summary>Envia um comando sem esperar resposta (erros chegam como evento "error").</summary>
    public void Send(string cmd, JsonObject? fields = null) => Write(Message(cmd, fields));

    /// <summary>Marca um trecho no profiler da IDE (a faixa "Marcas"):
    /// <c>using (aylis.Profile("consulta")) { ... }</c>.</summary>
    public IDisposable Profile(string name)
    {
        Send("profile_begin", new() { ["name"] = name });
        return new ProfileMark(this);
    }

    private sealed class ProfileMark : IDisposable
    {
        private readonly Aylis aylis;
        public ProfileMark(Aylis aylis) => this.aylis = aylis;
        public void Dispose() => aylis.Send("profile_end");
    }

    /// <summary>Envia um comando e espera o resultado (só na thread do Loop).</summary>
    public JsonNode? Call(string cmd, JsonObject? fields = null)
    {
        // A resposta chega pela leitura da thread do Loop: de outra thread, as duas leriam a mesma
        // entrada (use Post).
        var me = Environment.CurrentManagedThreadId;
        if (owner == 0) owner = me;
        else if (owner != me) throw new AylisException($"Call(\"{cmd}\") de outra thread: leia na thread do Loop (Post) ou antes de iniciar a thread");
        var id = ++nextId;
        var message = Message(cmd, fields);
        message["id"] = id;
        Write(message);
        for (;;)
        {
            var reply = Read() ?? throw new AylisException("a tela foi fechada");
            if (reply["id"] is JsonValue v && v.TryGetValue<int>(out var got) && got == id)
            {
                if (reply["error"] is JsonNode error) throw new AylisException(error.ToString());
                return reply["result"];
            }
            // Esperando um diálogo, os eventos (os do modal) rodam já; um evento que pergunta também
            // (o Post do próprio backend dispara o before_post).
            var kind = reply["event"]?.ToString();
            if (kind == "__post" && waiting > 0)
            {
                RunPosted();
                continue;
            }
            if (kind == "ping" && waiting > 0)
            {
                Send("pong"); // a sincronia dos roteiros: o que veio antes já rodou
                continue;
            }
            if (kind != null && kind != "closed" && kind != "ping" && kind != "__post" && (waiting > 0 || (reply["ask"] != null && running < 8)))
            {
                HandleEvent(reply);
                continue;
            }
            queued.Enqueue(reply);
        }
    }

    /// <summary>O valor de um campo decimal (Currency, BCD) como <c>decimal</c>, exato: o protocolo manda o
    /// texto, e ele nunca passa por <c>double</c>. <c>null</c> se o campo é nulo.</summary>
    public decimal? FieldDecimal(string dataSet, string field)
    {
        var value = Call("data", new() { ["target"] = dataSet, ["op"] = "value", ["field"] = field });
        return value is null ? null : decimal.Parse(value is JsonValue v && v.TryGetValue<string>(out var s) ? s : value.ToJsonString(), System.Globalization.NumberStyles.Number, System.Globalization.CultureInfo.InvariantCulture);
    }

    /// <summary>Caixa de mensagem que espera: devolve o botão
    /// (<c>"yes"</c>, <c>"no"</c>, <c>"ok"</c>...). Enquanto espera, os eventos continuam rodando.</summary>
    public string MessageDlg(string text, string kind = "information", params string[] buttons)
    {
        var list = new JsonArray();
        foreach (var b in buttons.Length == 0 ? new[] { "ok" } : buttons) list.Add(b);
        return WaitFor("message_dlg", new() { ["text"] = text, ["kind"] = kind, ["buttons"] = list });
    }

    /// <summary>Abre o form como modal e espera ele fechar: devolve o
    /// resultado (<c>"ok"</c>, <c>"cancel"</c>...).</summary>
    public string ShowModal(AylisForm form) => WaitFor("show_modal", new() { ["target"] = form.Id });

    private string WaitFor(string cmd, JsonObject fields)
    {
        fields["wait"] = true;
        waiting++;
        // O que chegou antes (enquanto outro Call esperava) roda já.
        for (var n = queued.Count; n > 0; n--)
        {
            var m = queued.Dequeue();
            var k = m["event"]?.ToString();
            if (k == "__post") RunPosted();
            else if (k == "ping") Send("pong");
            else if (k != null && k != "closed" && k != "error") HandleEvent(m);
            else queued.Enqueue(m);
        }
        try { return Call(cmd, fields)?.ToString() ?? ""; }
        finally { waiting--; }
    }

    /// <summary>Handler pelo nome usado no layout ("frmClientes.nome": só daquele form).</summary>
    public void On(string name, Action<AylisEvent> handler) => handlers[name] = handler;

    /// <summary>Abre o form e trata os eventos até a tela fechar.</summary>
    public void Run(string? form = null)
    {
        Call("run", new JsonObject { ["form"] = form });
        // A instância do form principal antes do primeiro evento (o código pode usar frmX).
        if (form != null && Call("find", new() { ["name"] = form }) is JsonValue found && found.TryGetValue<long>(out var formId))
            FormInstance(form, formId);
        Loop();
    }

    /// <summary>Trata os eventos até a tela fechar.</summary>
    public void Loop()
    {
        for (;;)
        {
            var message = queued.Count > 0 ? queued.Dequeue() : Read();
            if (message == null) return;
            var kind = message["event"]?.ToString();
            if (kind == null) continue;
            if (kind == "closed") return;
            if (kind == "__post")
            {
                RunPosted();
                continue;
            }
            // Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes.
            if (kind == "ping")
            {
                Send("pong");
                continue;
            }
            if (kind == "error")
            {
                Log("erro na tela:", message["message"]);
                continue;
            }
            HandleEvent(message);
        }
    }

    private void RunPosted()
    {
        while (posted.TryDequeue(out var job))
        {
            try { job(); }
            catch (Exception e) { Log("erro no post", e.Message); }
        }
    }

    /// <summary>Roda o handler do evento. Um evento que pergunta (com <c>ask</c>) recebe a resposta
    /// quando o handler termina: o <c>ev.Reply</c> ou, com exceção, a recusa com a mensagem (a tela
    /// mostra).</summary>
    private void HandleEvent(JsonObject message)
    {
        // Forms liberados na tela: as instâncias saem e os PostTo deles não rodam.
        if (message["event"]?.ToString() == "__freed")
        {
            if (message["forms"] is JsonArray ids)
                foreach (var id in ids)
                    if (id is JsonValue v && v.TryGetValue<long>(out var freedId)) { freed.Add(freedId); forms.Remove(freedId); }
            return;
        }
        var kind = message["event"]?.ToString() ?? "";
        long? ask = message["ask"] is JsonValue a && a.TryGetValue<long>(out var n) ? n : null;
        var ev = new AylisEvent(kind, message["handler"]?.ToString() ?? "", message["form"]?.ToString() ?? "",
            message["sender"]?["name"]?.ToString() ?? "", message["sender"]?["class"]?.ToString() ?? "", message["args"], message["info"]) { Ask = ask };
        running++;
        try
        {
            if (!handlers.TryGetValue($"{ev.Form}.{ev.Handler}", out var handler) && !handlers.TryGetValue(ev.Handler, out handler))
                Log("evento sem handler:", ev.Handler);
            else
            {
                // A janela do evento vira a atual (frmX aponta para ela durante o handler).
                if (message["form_id"] is JsonValue fid && fid.TryGetValue<long>(out var formId)) FormInstance(ev.Form, formId);
                handler(ev);
            }
            if (ask != null) Send("answer", new() { ["ask"] = ask, ["values"] = ev.Reply.DeepClone() });
        }
        catch (Exception e)
        {
            if (ask != null)
                Send("answer", new() { ["ask"] = ask, ["error"] = e.Message });
            else
            {
                Log("erro no handler", ev.Handler, e.Message);
                // A tela mostra o erro ao usuário.
                Send("exception", new() { ["message"] = e.Message, ["path"] = ev.Form, ["handler"] = ev.Handler });
            }
        }
        finally
        {
            running--;
            // O handler terminou: a tela deixa de mostrar o backend ocupado.
            if (message["seq"] is JsonValue s && s.TryGetValue<long>(out var seq)) Send("done", new() { ["seq"] = seq });
        }
    }

    /// <summary>Escreve no console do aylis-host (o stdout é do protocolo).</summary>
    public static void Log(params object?[] args)
    {
        var err = new StreamWriter(Console.OpenStandardError(), new UTF8Encoding(false)) { AutoFlush = true };
        err.WriteLine("[backend] " + string.Join(" ", args.Select(a => a?.ToString())));
    }
}

/// <summary>Uma tela da ponte com <c>--sessions</c> (<c>aylis-host --bridge ... --sessions</c>):
/// um processo atende todas as telas e cada uma roda numa thread própria (um handler demorado
/// não trava as outras; uma exceção fica nela). Guarde o estado da tela em variáveis locais do
/// <c>started</c> passado ao <see cref="Server.Serve"/> (nunca em campos estáticos, que são de
/// todas). A API é a do <see cref="Aylis"/>.</summary>
public sealed class Session : Aylis
{
    /// <summary>O número da tela na ponte.</summary>
    public int Number { get; }
    /// <summary>O usuário logado (o mesmo nas telas dele); nulo antes do <see cref="Login"/>.</summary>
    public User? User { get; private set; }
    public Server Server { get; }

    internal Session(Server server, int number) : base(true)
    {
        Server = server;
        Number = number;
    }

    /// <summary>Liga esta tela ao usuário <c>id</c>: as telas do mesmo usuário dividem o mesmo
    /// <see cref="User"/>.</summary>
    public void Login(string id)
    {
        Logout();
        User = Server.UserOf(id);
        lock (User.Lock) User.Sessions.Add(this);
    }

    /// <summary>Solta o usuário desta tela (sozinho, ao fechar).</summary>
    public void Logout()
    {
        if (User != null) Server.Leave(User, this);
        User = null;
    }
}

/// <summary>O usuário logado, dividido pelas telas dele. <c>Data</c> guarda o que vale para
/// todas (o carrinho, as preferências); use <c>lock (user.Lock)</c> quando duas telas podem
/// mexer ao mesmo tempo.</summary>
public sealed class User(string id)
{
    public string Id { get; } = id;
    public Dictionary<string, object?> Data { get; } = new();
    public object Lock { get; } = new();
    internal List<Session> Sessions { get; } = new();

    /// <summary>Roda <c>action(tela)</c> na thread de cada tela do usuário (atualizar todas).</summary>
    public void Broadcast(Action<Session> action)
    {
        Session[] sessions;
        lock (Lock) sessions = Sessions.ToArray();
        foreach (var s in sessions) s.Post(() => action(s));
    }
}

/// <summary>O processo das sessões: lê o stdin e entrega cada mensagem à tela dela.</summary>
public sealed class Server
{
    private readonly TextWriter output = new StreamWriter(Console.OpenStandardOutput(), new UTF8Encoding(false)) { AutoFlush = true, NewLine = "\n" };
    private readonly Dictionary<int, Session> sessions = new();
    private readonly Dictionary<string, User> users = new();

    internal void Write(JsonObject message)
    {
        var text = message.ToJsonString();
        lock (output) output.WriteLine(text);
    }

    internal User UserOf(string id)
    {
        lock (users)
        {
            if (!users.TryGetValue(id, out var user)) users[id] = user = new User(id);
            return user;
        }
    }

    internal void Leave(User user, Session session)
    {
        bool empty;
        lock (user.Lock)
        {
            user.Sessions.Remove(session);
            empty = user.Sessions.Count == 0;
        }
        if (empty) lock (users) users.Remove(user.Id);
    }

    /// <summary>Atende as telas da ponte com <c>--sessions</c>: <c>started</c> roda numa thread
    /// para cada tela que conecta (monte o estado e chame <c>Run</c>); <c>ended</c> quando ela sai.</summary>
    public static void Serve(Action<Session> started, Action<Session>? ended = null) => new Server().Run(started, ended);

    private void Run(Action<Session> started, Action<Session>? ended)
    {
        var input = new StreamReader(Console.OpenStandardInput(), new UTF8Encoding(false));
        for (string? line; (line = input.ReadLine()) != null;)
        {
            if (line.Trim().Length == 0 || JsonNode.Parse(line) is not JsonObject message) continue;
            if (message["session"] is not JsonValue n || !n.TryGetValue<int>(out var number)) continue;
            message.Remove("session");
            var kind = message["event"]?.ToString();
            if (kind == "session_open")
            {
                var session = new Session(this, number);
                lock (sessions) sessions[number] = session;
                new Thread(() =>
                {
                    try { started(session); }
                    catch (Exception e) { Aylis.Log("erro na sessão", number, e.Message); }
                    finally
                    {
                        try { ended?.Invoke(session); }
                        catch (Exception e) { Aylis.Log("erro em ended da sessão", number, e.Message); }
                        session.Logout();
                        lock (sessions) sessions.Remove(number);
                    }
                }) { IsBackground = true, Name = $"sessao-{number}" }.Start();
                continue;
            }
            Session? target;
            lock (sessions) sessions.TryGetValue(number, out target);
            target?.Deliver(kind == "session_close" ? new JsonObject { ["event"] = "closed" } : message);
        }
    }
}

/// <summary>
/// As alterações de um <c>apply_updates</c>, para o backend gravar do jeito dele, na ordem de
/// gravar (o mestre incluído antes dos itens; as exclusões do mestre por último). Cada alteração
/// é um <c>JsonObject</c>: <c>ref</c>, <c>op</c> ("insert", "update", "delete"), <c>data_set</c>,
/// <c>key</c>, <c>old</c>, <c>new</c> e, no item incluído sob um mestre também incluído,
/// <c>master</c>. Ao aceitar o mestre com a chave gerada, os itens dele já recebem a chave antes
/// de serem gravados:
/// <code>
/// var delta = new Delta(ev.Args);
/// foreach (var c in delta.Changes)
///     if ((string?)c["op"] == "insert" &amp;&amp; (string?)c["data_set"] == "cdsPedidos")
///         delta.Accept(c, new JsonObject { ["id"] = InserirPedido(c) });
/// ev.Reply["results"] = delta.ResultsJson();
/// </code>
/// </summary>
public sealed class Delta
{
    public string DataSet { get; } = "";
    public List<JsonObject> Changes { get; } = new();
    public List<JsonObject> Results { get; } = new();

    public Delta(JsonNode? source)
    {
        if (source is JsonValue v && v.TryGetValue(out string? text)) source = JsonNode.Parse(text);
        JsonArray? list = source as JsonArray;
        if (source is JsonObject o)
        {
            DataSet = (string?)o["data_set"] ?? "";
            list = o["changes"] as JsonArray;
        }
        foreach (var item in list ?? new JsonArray())
        {
            if (item?.DeepClone() is not JsonObject c) continue;
            c["key"] ??= new JsonObject();
            c["new"] ??= new JsonObject();
            Changes.Add(c);
        }
    }

    public Delta(string json) : this(JsonNode.Parse(json)) { }

    private List<JsonObject> Of(string op, string? dataSet) =>
        Changes.Where(c => (string?)c["op"] == op && (dataSet == null || (string?)c["data_set"] == dataSet)).ToList();

    public List<JsonObject> Inserted(string? dataSet = null) => Of("insert", dataSet);
    public List<JsonObject> Updated(string? dataSet = null) => Of("update", dataSet);
    public List<JsonObject> Deleted(string? dataSet = null) => Of("delete", dataSet);
    public List<JsonObject> OfDataSet(string dataSet) => Changes.Where(c => (string?)c["data_set"] == dataSet).ToList();

    /// <summary>Gravou; <paramref name="values"/> são os que o banco definiu (a chave gerada).</summary>
    public void Accept(JsonObject change, JsonObject? values = null)
    {
        var reference = (long)change["ref"]!;
        values ??= new JsonObject();
        foreach (var other in Changes)
        {
            if (other["master"] is not JsonObject link || (long?)link["ref"] != reference || link["fields"] is not JsonObject fields) continue;
            foreach (var (detail, master) in fields)
            {
                var name = (string?)master;
                if (name == null || !values.ContainsKey(name)) continue;
                foreach (var part in new[] { "new", "key" })
                    if (other[part] is JsonObject map && map.ContainsKey(detail)) map[detail] = values[name]?.DeepClone();
            }
        }
        Results.Add(values.Count == 0 ? new JsonObject { ["ref"] = reference } : new JsonObject { ["ref"] = reference, ["values"] = values.DeepClone() });
    }

    /// <summary>Não gravou: a mensagem vai para a tela e o registro continua pendente.</summary>
    public void Reject(JsonObject change, string message) =>
        Results.Add(new JsonObject { ["ref"] = (long)change["ref"]!, ["error"] = message });

    /// <summary>Os resultados prontos para o <c>ev.Reply["results"]</c>.</summary>
    public JsonArray ResultsJson() => new(Results.Select(r => (JsonNode)r.DeepClone()).ToArray());

    /// <summary>O delta de exemplo dos selftests dos clientes (o mesmo em todas as linguagens).</summary>
    public const string Sample =
        "{\"data_set\":\"cdsPedidos\",\"changes\":["
        + "{\"ref\":1,\"op\":\"insert\",\"data_set\":\"cdsPedidos\",\"key\":{\"id\":-1},\"new\":{\"id\":-1,\"cliente\":\"Joao\"}},"
        + "{\"ref\":2,\"op\":\"insert\",\"data_set\":\"cdsItens\",\"key\":{\"id\":-2},\"new\":{\"id\":-2,\"pedido_id\":-1},"
        + "\"master\":{\"data_set\":\"cdsPedidos\",\"ref\":1,\"fields\":{\"pedido_id\":\"id\"}}},"
        + "{\"ref\":3,\"op\":\"update\",\"data_set\":\"cdsItens\",\"key\":{\"id\":10},\"old\":{\"produto\":\"A\"},\"new\":{\"produto\":\"B\"}},"
        + "{\"ref\":4,\"op\":\"delete\",\"data_set\":\"cdsPedidos\",\"key\":{\"id\":7},\"old\":{\"id\":7}}]}";

    /// <summary>O texto que todos os clientes escrevem ao conferir o Delta com o <see cref="Sample"/>.</summary>
    public static string Selftest()
    {
        var delta = new Delta(Sample);
        delta.Accept(delta.Inserted("cdsPedidos")[0], new JsonObject { ["id"] = 501 });
        var item = delta.Inserted("cdsItens")[0]["new"]!["pedido_id"];
        return $"delta {delta.Inserted().Count} {delta.Updated().Count} {delta.Deleted().Count} {item} {delta.Results.Count}";
    }
}
