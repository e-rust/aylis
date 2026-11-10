"""Eventos do form frmPaint: o código é seu (o registro dos handlers fica no
forms/__init__.py gerado). Dentro de um handler, o form é a janela do evento.

O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
ferramentas, as cores, o zoom, o arquivo e a barra de status."""

from .components import frmPaint  # noqa: E402

from aylis_client import AylisError

# O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%).
estado = {"arquivo": "", "modificado": False, "cor_ativa": 1, "zoom": 100}

def ferramentas():
    """Os botões de ferramenta (os do form do evento) e a ferramenta de cada um."""
    return [
        (frmPaint.btnSelecionar, "select"), (frmPaint.btnLapis, "pencil"), (frmPaint.btnPincel, "brush"), (frmPaint.btnBorracha, "eraser"),
        (frmPaint.btnBalde, "fill"), (frmPaint.btnContaGotas, "picker"), (frmPaint.btnLinha, "line"), (frmPaint.btnRetangulo, "rectangle"),
        (frmPaint.btnElipse, "ellipse"),
    ]


def nome_do_arquivo(caminho):
    """O nome do arquivo sem a pasta."""
    return caminho.replace("\\", "/").split("/")[-1]


def titulo(aylis):
    """O título como o do Paint do Windows: "*Sem título - Paint"."""
    nome = nome_do_arquivo(estado["arquivo"]) if estado["arquivo"] else "Sem título"
    frmPaint.caption = "%s%s - Paint" % ("*" if estado["modificado"] else "", nome)


def atualizar_edicao(aylis):
    """Desfazer, refazer e o tamanho da imagem acompanham o desenho."""
    desfazer = frmPaint.pcDesenho.can_undo() is True
    refazer = frmPaint.pcDesenho.can_redo() is True
    frmPaint.mnuDesfazer.enabled = desfazer
    frmPaint.btnDesfazer.enabled = desfazer
    frmPaint.mnuRefazer.enabled = refazer
    frmPaint.btnRefazer.enabled = refazer
    frmPaint.lblTamanho.text = "%spx" % frmPaint.pcDesenho.size()


def marcar_modificado(aylis):
    estado["modificado"] = True
    titulo(aylis)
    atualizar_edicao(aylis)


def aplicar_zoom(aylis, zoom):
    estado["zoom"] = zoom
    frmPaint.pcDesenho.zoom = zoom / 100
    frmPaint.lblZoom.text = "%d%%" % zoom


def salvar_em(aylis, caminho):
    try:
        frmPaint.pcDesenho.save(caminho)
    except AylisError as erro:
        aylis.message_dlg("Não foi possível salvar a imagem.\n%s" % erro, kind="error")
        return
    estado["arquivo"] = caminho
    estado["modificado"] = False
    titulo(aylis)


def novo_click(aylis, event):
    frmPaint.pcDesenho.new(640, 400)
    estado["arquivo"] = ""
    estado["modificado"] = False
    titulo(aylis)
    atualizar_edicao(aylis)


def abrir_click(aylis, event):
    frmPaint.dlgAbrir.execute()


def salvar_click(aylis, event):
    if estado["arquivo"]:
        salvar_em(aylis, estado["arquivo"])
    else:
        salvar_como_click(aylis, event)


def salvar_como_click(aylis, event):
    frmPaint.dlgSalvar.execute()


def sair_click(aylis, event):
    frmPaint.close()


def desfazer_click(aylis, event):
    frmPaint.pcDesenho.undo()
    marcar_modificado(aylis)


def refazer_click(aylis, event):
    frmPaint.pcDesenho.redo()
    marcar_modificado(aylis)


def limpar_click(aylis, event):
    frmPaint.pcDesenho.clear()
    marcar_modificado(aylis)


def ampliar_click(aylis, event):
    aplicar_zoom(aylis, min(estado["zoom"] * 2, 800))


def reduzir_click(aylis, event):
    aplicar_zoom(aylis, max(estado["zoom"] // 2, 25))


def tamanho_real_click(aylis, event):
    aplicar_zoom(aylis, 100)


def ferramenta_click(aylis, event):
    escolhida = next((f for f in ferramentas() if f[0].id == event["sender"]["id"]), None)
    if escolhida is None:
        return
    frmPaint.pcDesenho.tool = escolhida[1]
    marcar_ferramenta(aylis, escolhida[0])


def marcar_ferramenta(aylis, escolhido):
    """Só o botão da ferramenta escolhida fica marcado."""
    for botao, _ in ferramentas():
        botao.checked = botao.id == escolhido.id


def marcar_cor(aylis, cor):
    """A cor que a paleta troca fica com o anel azul, como no Paint."""
    estado["cor_ativa"] = cor
    for circulo, n in ((frmPaint.btnCor1, 1), (frmPaint.btnCor2, 2)):
        circulo.stroke = "#0067C0" if n == cor else "#9A9A9A"
        circulo.stroke_width = 3.0 if n == cor else 1.0


def espessura_change(aylis, event):
    # "5 px" → 5
    try:
        frmPaint.pcDesenho.brush_size = int(str(event.get("args") or "").split(" ")[0])
    except ValueError:
        pass


def preencher_change(aylis, event):
    if isinstance(event.get("args"), bool):
        frmPaint.pcDesenho.filled = event["args"]


def cor1_click(aylis, event):
    marcar_cor(aylis, 1)


def cor2_click(aylis, event):
    marcar_cor(aylis, 2)


def paleta_click(aylis, event):
    cor = frmPaint.sender_as_circle(event).fill
    if estado["cor_ativa"] == 1:
        frmPaint.pcDesenho.color = cor
        frmPaint.btnCor1.fill = cor
    else:
        frmPaint.pcDesenho.back_color = cor
        frmPaint.btnCor2.fill = cor


def desenho_change(aylis, event):
    marcar_modificado(aylis)


def desenho_pick(aylis, event):
    # "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito)
    texto = event.get("args") or ""
    if texto.endswith("2"):
        frmPaint.btnCor2.fill = texto[:-1]
    else:
        frmPaint.btnCor1.fill = texto


def desenho_cursor(aylis, event):
    texto = event.get("args") or ""
    frmPaint.lblPosicao.text = texto + "px" if texto else ""


def abrir_select(aylis, event):
    caminho = event.get("args") or ""
    try:
        frmPaint.pcDesenho.open(caminho)
    except AylisError as erro:
        aylis.message_dlg("Não foi possível abrir a imagem.\n%s" % erro, kind="error")
        return
    estado["arquivo"] = caminho
    estado["modificado"] = False
    titulo(aylis)
    atualizar_edicao(aylis)


def salvar_select(aylis, event):
    salvar_em(aylis, event.get("args") or "")


def recortar_click(aylis, event):
    frmPaint.pcDesenho.cut()


def copiar_click(aylis, event):
    frmPaint.pcDesenho.copy()


def colar_click(aylis, event):
    # O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção.
    if frmPaint.pcDesenho.paste() is True:
        marcar_ferramenta(aylis, frmPaint.btnSelecionar)


def excluir_click(aylis, event):
    frmPaint.pcDesenho.delete()


def selecionar_tudo_click(aylis, event):
    frmPaint.pcDesenho.select_all()
    marcar_ferramenta(aylis, frmPaint.btnSelecionar)
