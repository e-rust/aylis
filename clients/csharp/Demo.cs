// Backend de demonstração em C#: um campo, um botão e a resposta do backend.
//
//   dotnet build clients/csharp -c Release
//   aylis-host -- clients/csharp/bin/Release/net8.0/AylisDemo.exe
//   aylis-host --png tela.png -- clients/csharp/bin/Release/net8.0/AylisDemo.exe --selftest

using System.Text.Json.Nodes;
using AylisClient;

var selftest = args.Contains("--selftest");
var aylis = new Aylis();

var layoutFile = Path.Combine(AppContext.BaseDirectory, "demo_layout.json");
aylis.Send("load", new JsonObject { ["layout"] = JsonNode.Parse(File.ReadAllText(layoutFile)) });

// O on_close_query pergunta: a tela só fecha com a resposta (o ev.Reply).
aylis.On("pode_fechar", ev =>
{
    Aylis.Log("SELFTEST csharp pergunta");
    Aylis.Log("SELFTEST csharp " + Delta.Selftest());
    ev.Reply["can_close"] = true;
});

aylis.On("hello", _ =>
{
    var name = aylis.Call("get", new JsonObject { ["target"] = "edtName", ["prop"] = "text" })?.ToString();
    if (string.IsNullOrEmpty(name)) name = "mundo";
    aylis.Send("set", new JsonObject { ["target"] = "lblMsg", ["props"] = new JsonObject { ["text"] = $"Olá, {name}! (C#)" } });
    if (selftest)
    {
        Aylis.Log("SELFTEST csharp", aylis.Call("get", new JsonObject { ["target"] = "lblMsg", ["prop"] = "text" }));
        // Fechar pelo backend passa pelo on_close_query, que pergunta (e espera a resposta).
        aylis.Send("close", new JsonObject { ["target"] = "frmDemo" });
    }
});

aylis.Call("run", new JsonObject { ["form"] = "frmDemo" });
if (selftest)
{
    aylis.Send("set", new JsonObject { ["target"] = "edtName", ["props"] = new JsonObject { ["text"] = "Ana" } });
    aylis.Send("click", new JsonObject { ["target"] = "btnHello" });
}
aylis.Loop();
