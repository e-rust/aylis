"""Eventos do form frmPlayer: o código é seu (o registro dos handlers fica no
forms/__init__.py gerado). Dentro de um handler, o form é a janela do evento.

O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
toca é o componente VideoPlayer (vpVideo), áudio e vídeo."""

from .components import frmPlayer  # noqa: E402

import math

# O que o form guarda. repetir: 0 não repete, 1 repete a fila, 2 repete a mídia.
estado = {"fila": [], "atual": -1, "tocando": False, "embaralhar": False, "repetir": 0, "mudo": False, "semente": 12345}

# As extensões de áudio (o resto é vídeo).
AUDIO = ["mp3", "wav", "m4a", "flac", "ogg", "wma", "aac"]


def nome_do_arquivo(caminho):
    """O nome do arquivo sem a pasta."""
    return caminho.replace("\\", "/").split("/")[-1]


def pasta(caminho):
    """A pasta do arquivo (vazio se não tiver)."""
    i = max(caminho.rfind("\\"), caminho.rfind("/"))
    return caminho[:i] if i >= 0 else ""


def sem_extensao(nome):
    i = nome.rfind(".")
    return nome[:i] if i > 0 else nome


def tipo(caminho):
    nome = nome_do_arquivo(caminho)
    i = nome.rfind(".")
    ext = nome[i + 1:].lower() if i >= 0 else ""
    return "Música" if ext in AUDIO else "Vídeo"


def tempo(segundos):
    """3:07"""
    s = int(math.floor(max(segundos, 0.0)))
    return "%d:%02d" % (s // 60, s % 60)


def atualizar_fila(aylis):
    """A fila na lista, com a mídia atual selecionada."""
    itens = ";".join("%s|%s" % (nome_do_arquivo(c), pasta(c)) for c in estado["fila"])
    frmPlayer.lvFila.items = itens
    frmPlayer.lvFila.item_index = estado["atual"]


def mostrar_atual(aylis):
    """O título, o detalhe e o título da janela."""
    if estado["atual"] < 0:
        frmPlayer.lblTitulo.text = "Nenhuma mídia"
        frmPlayer.lblDetalhe.text = ""
        frmPlayer.caption = "Reprodutor Multimídia"
        return
    caminho = estado["fila"][estado["atual"]]
    titulo = sem_extensao(nome_do_arquivo(caminho))
    frmPlayer.lblTitulo.text = titulo
    frmPlayer.lblDetalhe.text = "%s · %d de %d" % (tipo(caminho), estado["atual"] + 1, len(estado["fila"]))
    frmPlayer.caption = "%s - Reprodutor Multimídia" % titulo


def botao_reproduzir(aylis):
    """O botão do meio mostra o que ele vai fazer: tocar ou pausar."""
    tocando = estado["tocando"]
    frmPlayer.btnReproduzir.icon = "pause" if tocando else "play"
    frmPlayer.btnReproduzir.hint = "Pausar (Ctrl+P)" if tocando else "Reproduzir (Ctrl+P)"


def tocar(aylis, i):
    """Toca a mídia i da fila. Tocar é a última coisa: um erro chega no video_erro."""
    estado["atual"] = i
    estado["tocando"] = True
    frmPlayer.vpVideo.file = estado["fila"][i]
    mostrar_atual(aylis)
    atualizar_fila(aylis)
    botao_reproduzir(aylis)
    frmPlayer.sbPosicao.position = 0.0
    frmPlayer.vpVideo.play()


def proximo_indice():
    """A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se repetir a
    fila. -1: acabou."""
    n = len(estado["fila"])
    if n == 0:
        return -1
    if estado["embaralhar"]:
        if n == 1:
            return 0
        while True:
            estado["semente"] = (estado["semente"] * 1103515245 + 12345) % 2147483648
            i = estado["semente"] % n
            if i != estado["atual"]:
                return i
    if estado["atual"] + 1 < n:
        return estado["atual"] + 1
    if estado["repetir"] == 1:
        return 0
    return -1


def abrir_click(aylis, event):
    frmPlayer.dlgAbrir.execute()


def abrir_select(aylis, event):
    # Os arquivos escolhidos vêm separados por ";".
    novos = [s for s in str(event.get("args") or "").split(";") if s]
    if not novos:
        return
    primeiro = len(estado["fila"])
    estado["fila"].extend(novos)
    atualizar_fila(aylis)
    if estado["atual"] < 0:
        tocar(aylis, primeiro)
    else:
        mostrar_atual(aylis)


def limpar_fila_click(aylis, event):
    frmPlayer.vpVideo.stop()
    estado["fila"] = []
    estado["atual"] = -1
    estado["tocando"] = False
    frmPlayer.vpVideo.file = ""
    atualizar_fila(aylis)
    mostrar_atual(aylis)
    botao_reproduzir(aylis)


def sair_click(aylis, event):
    frmPlayer.close()


def reproduzir_click(aylis, event):
    if estado["atual"] < 0:
        if estado["fila"]:
            tocar(aylis, 0)
        else:
            abrir_click(aylis, event)
    elif estado["tocando"]:
        frmPlayer.vpVideo.pause()
        estado["tocando"] = False
        botao_reproduzir(aylis)
    else:
        estado["tocando"] = True
        botao_reproduzir(aylis)
        frmPlayer.vpVideo.play()


def parar_click(aylis, event):
    if estado["atual"] < 0:
        return
    frmPlayer.vpVideo.stop()
    estado["tocando"] = False
    botao_reproduzir(aylis)
    frmPlayer.sbPosicao.position = 0.0


def anterior_click(aylis, event):
    atual = estado["atual"]
    if atual > 0:
        tocar(aylis, atual - 1)
    elif atual == 0 and estado["repetir"] == 1:
        tocar(aylis, len(estado["fila"]) - 1)
    elif atual == 0:
        frmPlayer.vpVideo.seek(0.0)


def proxima_click(aylis, event):
    i = proximo_indice()
    if i >= 0:
        tocar(aylis, i)


def embaralhar_click(aylis, event):
    estado["embaralhar"] = not estado["embaralhar"]
    ligado = estado["embaralhar"]
    frmPlayer.btnEmbaralhar.checked = ligado
    frmPlayer.btnEmbaralhar.hint = "Embaralhar: ativado" if ligado else "Embaralhar: desativado"
    frmPlayer.mnuEmbaralhar.checked = ligado


def repetir_click(aylis, event):
    estado["repetir"] = (estado["repetir"] + 1) % 3
    modo = estado["repetir"]
    frmPlayer.btnRepetir.checked = modo > 0
    frmPlayer.btnRepetir.icon = "repeat_one" if modo == 2 else "repeat"
    frmPlayer.btnRepetir.hint = ["Repetir: desativado", "Repetir: todos", "Repetir: um"][modo]
    frmPlayer.mnuRepetir.checked = modo > 0


def pagina(aylis, agora):
    """Mostra a página "Reproduzindo agora" ou a "Fila de reprodução"."""
    frmPlayer.pnlAgora.visible = agora
    frmPlayer.pnlFila.visible = not agora
    frmPlayer.btnAgora.checked = agora
    frmPlayer.btnFila.checked = not agora


def agora_click(aylis, event):
    pagina(aylis, True)


def fila_click(aylis, event):
    pagina(aylis, False)


def posicao_seek(aylis, event):
    if isinstance(event.get("args"), (int, float)):
        frmPlayer.vpVideo.seek(float(event["args"]))


def mudo_click(aylis, event):
    estado["mudo"] = not estado["mudo"]
    mudo = estado["mudo"]
    frmPlayer.vpVideo.muted = mudo
    frmPlayer.btnMudo.icon = "mute" if mudo else "volume"
    frmPlayer.btnMudo.hint = "Com som" if mudo else "Sem som"


def volume_change(aylis, event):
    if isinstance(event.get("args"), (int, float)):
        frmPlayer.vpVideo.volume = float(event["args"]) / 100.0


def video_fim(aylis, event):
    if estado["repetir"] == 2 and estado["atual"] >= 0:
        tocar(aylis, estado["atual"])
        return
    i = proximo_indice()
    if i >= 0:
        tocar(aylis, i)
    else:
        estado["tocando"] = False
        botao_reproduzir(aylis)


def video_erro(aylis, event):
    estado["tocando"] = False
    botao_reproduzir(aylis)
    frmPlayer.lblDetalhe.text = "Não foi possível reproduzir este arquivo"


def fila_item_click(aylis, event):
    i = event.get("args")
    if isinstance(i, int) and 0 <= i < len(estado["fila"]):
        tocar(aylis, i)


def posicao_timer(aylis, event):
    if estado["atual"] < 0:
        return
    posicao = frmPlayer.vpVideo.position() or 0.0
    duracao = frmPlayer.vpVideo.duration() or 0.0
    frmPlayer.sbPosicao.duration = duracao
    frmPlayer.sbPosicao.position = posicao
    frmPlayer.lblTempo.text = tempo(posicao)
    frmPlayer.lblDuracao.text = tempo(duracao)
