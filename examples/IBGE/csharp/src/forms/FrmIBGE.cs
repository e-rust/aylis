// Eventos do form frmIBGE: o código é seu (o registro dos handlers fica no AllForms.cs
// gerado). Dentro de um handler, o form é a janela do evento.
//
// Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa
// e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem
// o mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou
// município escolhido: as bandeiras, os mapas e os dados dos gráficos.

using System.Text.Json.Nodes;
using AylisClient;
using static App.Forms.Components;

namespace App.Forms;

public static class FrmIBGE
{
    /// <summary>A pasta dos dados no projeto (para os componentes).</summary>
    const string Dados = "assets/dados/";

    /// <summary>A mesma pasta no disco (o backend roda na pasta da linguagem).</summary>
    static readonly string Pasta = new[] { "../assets/dados", "assets/dados" }.FirstOrDefault(Directory.Exists) ?? "../assets/dados";

    /// <summary>Os municípios (para os 10 maiores de cada estado).</summary>
    static JsonArray municipios = new();

    /// <summary>Código do estado ou do município → [[ano, habitantes], ...].</summary>
    static JsonObject populacao = new();

    /// <summary>Um arquivo JSON da pasta dos dados (null se faltar).</summary>
    static JsonNode? Ler(string arquivo)
    {
        var caminho = Path.Combine(Pasta, arquivo);
        return File.Exists(caminho) ? JsonNode.Parse(File.ReadAllText(caminho)) : null;
    }

    /// <summary>Põe os registros (uma lista de objetos) num ClientDataSet do form.</summary>
    static void Carregar(Aylis aylis, AylisComponent dataset, JsonNode? linhas) =>
        aylis.Call("data", new JsonObject { ["target"] = dataset.Id, ["op"] = "load", ["rows"] = linhas ?? new JsonArray() });

    /// <summary>O registro atual de um ClientDataSet (campo → valor).</summary>
    static JsonObject Registro(Aylis aylis, AylisComponent dataset) =>
        aylis.Call("data", new JsonObject { ["target"] = dataset.Id, ["op"] = "record" }) as JsonObject ?? new JsonObject();

    /// <summary>O valor de um campo como texto (vazio se for nulo).</summary>
    static string Texto(JsonNode? v) => v?.ToString() ?? "";

    /// <summary>O caminho de uma imagem da pasta dos dados (vazio: não tem).</summary>
    static string Imagem(string caminho) => caminho == "" ? "" : Dados + caminho;

    public static void form_show(Aylis aylis, AylisEvent ev)
    {
        var estados = Ler("estados.json");
        if (estados == null)
        {
            frmIBGE.lblFonte.Text = "Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados";
            return;
        }
        municipios = Ler("municipios.json") as JsonArray ?? new JsonArray();
        populacao = Ler("populacao.json") as JsonObject ?? new JsonObject();
        frmIBGE.svgBrasil.Source = Dados + "mapas/BR.svg";
        // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
        Carregar(aylis, frmIBGE.cdsDistritos, Ler("distritos.json"));
        Carregar(aylis, frmIBGE.cdsMunicipios, municipios);
        Carregar(aylis, frmIBGE.cdsEstados, estados);
        estado_mudou(aylis, ev);
    }

    /// <summary>Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios.</summary>
    public static void estado_mudou(Aylis aylis, AylisEvent ev)
    {
        var e = Registro(aylis, frmIBGE.cdsEstados);
        if (e["id"] == null) return;
        var uf = e["id"]!.GetValue<long>();
        frmIBGE.lblEstado.Text = Texto(e["nome"]);
        frmIBGE.lblRegiao.Text = $"{Texto(e["sigla"])} · Região {Texto(e["regiao"])}";
        frmIBGE.imgBandeiraEstado.ImageValue = Imagem(Texto(e["bandeira"]));
        frmIBGE.lblEstadoEstimativa.Text = $"Estimativa de {Texto(e["estimativa_ano"])}";
        frmIBGE.svgBrasil.Highlight = $"e{uf}";
        var mapa = $"{Dados}mapas/{uf}.svg";
        frmIBGE.svgEstado.Source = mapa;
        frmIBGE.svgMunicipio.Source = mapa;

        var maiores = municipios
            .Where(m => m!["uf_id"]!.GetValue<long>() == uf)
            .Select(m => (Nome: m!["nome"]!.GetValue<string>(), Habitantes: m["populacao"]?.GetValue<long>() ?? 0))
            .OrderByDescending(m => m.Habitantes)
            .ThenBy(m => m.Nome, StringComparer.Ordinal)
            .Take(10)
            .Select(m => (JsonNode)new JsonObject { ["nome"] = m.Nome, ["habitantes"] = m.Habitantes });
        Carregar(aylis, frmIBGE.cdsMaiores, new JsonArray(maiores.ToArray()));
        municipio_mudou(aylis, ev);
    }

    /// <summary>Outro município: a bandeira, o destaque nos mapas e a população ano a ano.</summary>
    public static void municipio_mudou(Aylis aylis, AylisEvent ev)
    {
        var m = Registro(aylis, frmIBGE.cdsMunicipios);
        if (m["id"] == null) return;
        var codigo = m["id"]!.GetValue<long>();
        frmIBGE.lblMunicipio.Text = Texto(m["nome"]);
        frmIBGE.lblCodigo.Text = $"Código IBGE {codigo}";
        var bandeira = Texto(m["bandeira"]);
        frmIBGE.imgBandeiraMunicipio.ImageValue = Imagem(bandeira);
        frmIBGE.lblSemBandeira.Visible = bandeira == "";
        frmIBGE.lblEstimativa.Text = $"Estimativa de {Texto(m["estimativa_ano"])}";
        var alvo = $"m{codigo}";
        frmIBGE.svgEstado.Highlight = alvo;
        frmIBGE.svgMunicipio.Highlight = alvo;
        frmIBGE.svgMunicipio.ZoomTo = alvo;

        var linhas = new JsonArray();
        foreach (var p in populacao[codigo.ToString()] as JsonArray ?? new JsonArray())
            linhas.Add(new JsonObject { ["ano"] = p![0]!.ToString(), ["habitantes"] = p[1]!.GetValue<long>() });
        Carregar(aylis, frmIBGE.cdsPopulacao, linhas);
    }
}
