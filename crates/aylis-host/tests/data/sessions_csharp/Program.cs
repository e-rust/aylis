// Backend de teste das sessões em C# (aylis-host --bridge ... --sessions): um processo para
// todas as telas, um contador por tela; "boom" lança uma exceção que fica só naquela sessão,
// "lento" demora sem travar as outras e "entrar"/"comum" ligam as telas do mesmo usuário.

using System.Text.Json.Nodes;
using AylisClient;

var layout = JsonNode.Parse("""
{"class": "Form", "name": "frm", "width": 200, "height": 80, "children": [
  {"class": "Label", "name": "lbl", "text": "-"},
  {"class": "Button", "name": "btn", "text": "+", "on_click": "somar"},
  {"class": "Button", "name": "boom", "text": "!", "on_click": "boom"},
  {"class": "Button", "name": "lento", "text": "z", "on_click": "lento"},
  {"class": "Label", "name": "lblu", "text": "-"},
  {"class": "Button", "name": "entrar", "text": "e", "on_click": "entrar"},
  {"class": "Button", "name": "comum", "text": "c", "on_click": "comum"}
]}
""")!;

void Set(Session s, string target, string text) => s.Send("set", new() { ["target"] = target, ["props"] = new JsonObject { ["text"] = text } });

Server.Serve(s =>
{
    var total = 0;
    Aylis.Log($"SELFTEST sessão {s.Number} abriu (pid {Environment.ProcessId})");
    s.On("somar", _ => { total++; Set(s, "frm.lbl", $"{s.Number}:{total}"); });
    s.On("boom", _ => throw new InvalidOperationException("erro de propósito"));
    s.On("lento", _ => Thread.Sleep(3000));
    s.On("entrar", _ => { s.Login("ana"); Set(s, "frm.lblu", "ana"); });
    s.On("comum", _ =>
    {
        var user = s.User!;
        int n;
        lock (user.Lock)
        {
            n = (user.Data.TryGetValue("total", out var t) ? (int)t! : 0) + 1;
            user.Data["total"] = n;
        }
        user.Broadcast(t => Set(t, "frm.lblu", $"u:{n}"));
    });
    s.Send("load", new() { ["layout"] = layout.DeepClone() });
    s.Run("frm");
}, s => Aylis.Log($"SELFTEST sessão {s.Number} saiu"));
