"""Eventos do form frmIBGE: o código é seu (o registro dos handlers fica no
forms/__init__.py gerado). Dentro de um handler, o form é a janela do evento.

Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa e
gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem o
mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou município
escolhido: as bandeiras, os mapas e os dados dos gráficos."""

import json
import os

from .components import frmIBGE  # noqa: E402

# A pasta dos dados no projeto (para os componentes) e no disco (para o backend, que roda
# na pasta da linguagem).
DADOS = "assets/dados/"
PASTA = next((p for p in ("../assets/dados", "assets/dados") if os.path.isdir(p)), "../assets/dados")

# Os municípios (para os 10 maiores de cada estado) e as séries de população por código.
_municipios = []
_populacao = {}


def ler(arquivo):
    """Um arquivo JSON da pasta dos dados (None se faltar)."""
    try:
        with open(os.path.join(PASTA, arquivo), encoding="utf-8") as f:
            return json.load(f)
    except OSError:
        return None


def carregar(aylis, dataset, linhas):
    """Põe os registros (uma lista de objetos) num ClientDataSet do form."""
    aylis.call("data", target=dataset.id, op="load", rows=linhas)


def registro(aylis, dataset):
    """O registro atual de um ClientDataSet (campo → valor)."""
    return aylis.call("data", target=dataset.id, op="record") or {}


def imagem(caminho):
    """O caminho de uma imagem da pasta dos dados (vazio: não tem)."""
    return DADOS + caminho if caminho else ""


def form_show(aylis, event):
    global _municipios, _populacao
    estados = ler("estados.json")
    if estados is None:
        frmIBGE.lblFonte.text = "Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados"
        return
    _municipios = ler("municipios.json") or []
    _populacao = ler("populacao.json") or {}
    frmIBGE.svgBrasil.source = DADOS + "mapas/BR.svg"
    # Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
    carregar(aylis, frmIBGE.cdsDistritos, ler("distritos.json") or [])
    carregar(aylis, frmIBGE.cdsMunicipios, _municipios)
    carregar(aylis, frmIBGE.cdsEstados, estados)
    estado_mudou(aylis, event)


def estado_mudou(aylis, event):
    """Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios."""
    e = registro(aylis, frmIBGE.cdsEstados)
    if e.get("id") is None:
        return
    uf = e["id"]
    frmIBGE.lblEstado.text = e["nome"]
    frmIBGE.lblRegiao.text = f"{e['sigla']} · Região {e['regiao']}"
    frmIBGE.imgBandeiraEstado.image = imagem(e.get("bandeira") or "")
    frmIBGE.lblEstadoEstimativa.text = f"Estimativa de {e['estimativa_ano']}"
    frmIBGE.svgBrasil.highlight = f"e{uf}"
    mapa = f"{DADOS}mapas/{uf}.svg"
    frmIBGE.svgEstado.source = mapa
    frmIBGE.svgMunicipio.source = mapa

    do_estado = [m for m in _municipios if m["uf_id"] == uf]
    do_estado.sort(key=lambda m: (-(m["populacao"] or 0), m["nome"]))
    carregar(aylis, frmIBGE.cdsMaiores, [{"nome": m["nome"], "habitantes": m["populacao"]} for m in do_estado[:10]])
    municipio_mudou(aylis, event)


def municipio_mudou(aylis, event):
    """Outro município: a bandeira, o destaque nos mapas e a população ano a ano."""
    m = registro(aylis, frmIBGE.cdsMunicipios)
    if m.get("id") is None:
        return
    codigo = m["id"]
    frmIBGE.lblMunicipio.text = m["nome"]
    frmIBGE.lblCodigo.text = f"Código IBGE {codigo}"
    bandeira = m.get("bandeira") or ""
    frmIBGE.imgBandeiraMunicipio.image = imagem(bandeira)
    frmIBGE.lblSemBandeira.visible = not bandeira
    frmIBGE.lblEstimativa.text = f"Estimativa de {m['estimativa_ano']}"
    alvo = f"m{codigo}"
    frmIBGE.svgEstado.highlight = alvo
    frmIBGE.svgMunicipio.highlight = alvo
    frmIBGE.svgMunicipio.zoom_to = alvo

    serie = _populacao.get(str(codigo), [])
    carregar(aylis, frmIBGE.cdsPopulacao, [{"ano": str(ano), "habitantes": habitantes} for ano, habitantes in serie])
