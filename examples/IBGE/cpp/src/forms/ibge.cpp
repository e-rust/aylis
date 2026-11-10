// Eventos do form frmIBGE: o código é seu (as declarações e o registro dos handlers
// ficam no forms.hpp gerado). Dentro de um handler, o form é a janela do evento.
//
// Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa
// e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem
// o mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou
// município escolhido: as bandeiras, os mapas e os dados dos gráficos.

#include <algorithm>
#include <filesystem>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#include "../aylis.hpp"
#include "forms.hpp"

namespace frm_ibge {

namespace {

// A pasta dos dados no projeto (para os componentes).
const std::string dados = "assets/dados/";

// A mesma pasta no disco (o backend roda na pasta da linguagem).
std::filesystem::path pasta() {
    for (const char* p : {"../assets/dados", "assets/dados"}) {
        if (std::filesystem::is_directory(p)) return p;
    }
    return "../assets/dados";
}

// Os municípios (para os 10 maiores de cada estado) e as séries de população por código.
aylis::Json municipios = aylis::Json::array({});
aylis::Json populacao = aylis::Json::object();

// Um arquivo JSON da pasta dos dados (null se faltar).
aylis::Json ler(const std::string& arquivo) {
    std::ifstream in(pasta() / arquivo, std::ios::binary);
    if (!in) return nullptr;
    std::stringstream texto;
    texto << in.rdbuf();
    return aylis::Json::parse(texto.str());
}

// Põe os registros (uma lista de objetos) num ClientDataSet do form.
void carregar(aylis::Aylis& app, const aylis::Component& dataset, aylis::Json linhas) {
    app.call("data", aylis::Json{{"target", dataset.id()}, {"op", "load"}, {"rows", std::move(linhas)}});
}

// O registro atual de um ClientDataSet (campo → valor).
aylis::Json registro(aylis::Aylis& app, const aylis::Component& dataset) {
    return app.call("data", aylis::Json{{"target", dataset.id()}, {"op", "record"}});
}

// Um número de um campo (0 se for nulo).
long long numero(const aylis::Json& v) {
    return v.kind() == aylis::Json::Kind::Number ? static_cast<long long>(v.number()) : 0;
}

// O caminho de uma imagem da pasta dos dados (vazio: não tem).
std::string imagem(const std::string& caminho) {
    return caminho.empty() ? std::string() : dados + caminho;
}

}  // namespace

void form_show(aylis::Aylis& app, const aylis::Event& ev) {
    aylis::Json estados = ler("estados.json");
    if (estados.is_null()) {
        frmIBGE.lblFonte.set_text("Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados");
        return;
    }
    municipios = ler("municipios.json");
    populacao = ler("populacao.json");
    frmIBGE.svgBrasil.set_source(dados + "mapas/BR.svg");
    // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
    carregar(app, frmIBGE.cdsDistritos, ler("distritos.json"));
    carregar(app, frmIBGE.cdsMunicipios, municipios);
    carregar(app, frmIBGE.cdsEstados, estados);
    estado_mudou(app, ev);
}

// Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios.
void estado_mudou(aylis::Aylis& app, const aylis::Event& ev) {
    aylis::Json e = registro(app, frmIBGE.cdsEstados);
    if (e["id"].is_null()) return;
    long long uf = numero(e["id"]);
    frmIBGE.lblEstado.set_text(e["nome"].str());
    frmIBGE.lblRegiao.set_text(e["sigla"].str() + " · Região " + e["regiao"].str());
    frmIBGE.imgBandeiraEstado.set_image(imagem(e["bandeira"].str()));
    frmIBGE.lblEstadoEstimativa.set_text("Estimativa de " + e["estimativa_ano"].str());
    frmIBGE.svgBrasil.set_highlight("e" + std::to_string(uf));
    std::string mapa = dados + "mapas/" + std::to_string(uf) + ".svg";
    frmIBGE.svgEstado.set_source(mapa);
    frmIBGE.svgMunicipio.set_source(mapa);

    std::vector<const aylis::Json*> do_estado;
    for (const aylis::Json& m : municipios.items()) {
        if (numero(m["uf_id"]) == uf) do_estado.push_back(&m);
    }
    std::stable_sort(do_estado.begin(), do_estado.end(), [](const aylis::Json* a, const aylis::Json* b) {
        long long pa = numero((*a)["populacao"]), pb = numero((*b)["populacao"]);
        return pa != pb ? pa > pb : (*a)["nome"].str() < (*b)["nome"].str();
    });
    std::vector<aylis::Json> maiores;
    for (size_t i = 0; i < do_estado.size() && i < 10; ++i) {
        maiores.push_back(aylis::Json{{"nome", (*do_estado[i])["nome"]}, {"habitantes", (*do_estado[i])["populacao"]}});
    }
    carregar(app, frmIBGE.cdsMaiores, aylis::Json::array(std::move(maiores)));
    municipio_mudou(app, ev);
}

// Outro município: a bandeira, o destaque nos mapas e a população ano a ano.
void municipio_mudou(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    aylis::Json m = registro(app, frmIBGE.cdsMunicipios);
    if (m["id"].is_null()) return;
    long long codigo = numero(m["id"]);
    frmIBGE.lblMunicipio.set_text(m["nome"].str());
    frmIBGE.lblCodigo.set_text("Código IBGE " + std::to_string(codigo));
    std::string bandeira = m["bandeira"].str();
    frmIBGE.imgBandeiraMunicipio.set_image(imagem(bandeira));
    frmIBGE.lblSemBandeira.set_visible(bandeira.empty());
    frmIBGE.lblEstimativa.set_text("Estimativa de " + m["estimativa_ano"].str());
    std::string alvo = "m" + std::to_string(codigo);
    frmIBGE.svgEstado.set_highlight(alvo);
    frmIBGE.svgMunicipio.set_highlight(alvo);
    frmIBGE.svgMunicipio.set_zoom_to(alvo);

    std::vector<aylis::Json> linhas;
    for (const aylis::Json& p : populacao[std::to_string(codigo)].items()) {
        linhas.push_back(aylis::Json{{"ano", std::to_string(numero(p.items()[0]))}, {"habitantes", p.items()[1]}});
    }
    carregar(app, frmIBGE.cdsPopulacao, aylis::Json::array(std::move(linhas)));
}

}  // namespace frm_ibge
