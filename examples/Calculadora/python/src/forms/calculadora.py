"""Eventos do form frmCalculadora: o código é seu (o registro dos handlers fica no
forms/__init__.py gerado). Dentro de um handler, o form é a janela do evento."""

from .components import Button, Form, ListView, ListViewItem, frmCalculadora  # noqa: E402

from motor import Calculadora, botao_do_teclado, formatar

calc = Calculadora()


def sem_erro():
    """Os botões que só funcionam sem erro no visor (os da janela do evento)."""
    return [
        frmCalculadora.btnMMais, frmCalculadora.btnMMenos, frmCalculadora.btnMS, frmCalculadora.btnPorcento,
        frmCalculadora.btnInverso, frmCalculadora.btnQuadrado, frmCalculadora.btnRaiz, frmCalculadora.btnDividir,
        frmCalculadora.btnMultiplicar, frmCalculadora.btnSubtrair, frmCalculadora.btnSomar, frmCalculadora.btnSinal,
        frmCalculadora.btnVirgula,
    ]


def atualizar(aylis):
    """Põe o estado da calculadora na tela."""
    visor = calc.visor()
    # O número encolhe para caber, como no Windows.
    n = len(visor)
    if calc.tem_erro():
        tamanho = 26
    elif n <= 11:
        tamanho = 46
    else:
        tamanho = max(46 * 11 // n, 22)
    frmCalculadora.lblVisor.text = visor
    frmCalculadora.lblVisor.font_size = tamanho
    frmCalculadora.lblExpressao.text = calc.expressao
    erro = calc.tem_erro()
    for botao in sem_erro():
        botao.enabled = not erro
    for botao in (frmCalculadora.btnMC, frmCalculadora.btnMR):
        botao.enabled = not erro and len(calc.memoria) > 0
    if calc.historico:
        historico = [ListViewItem(formatar(r), e) for e, r in reversed(calc.historico)]
    else:
        historico = [ListViewItem("Ainda não há histórico")]
    frmCalculadora.lvHistorico.items = historico
    if calc.memoria:
        memoria = [ListViewItem(formatar(m)) for m in reversed(calc.memoria)]
    else:
        memoria = [ListViewItem("Não há nada salvo na memória")]
    frmCalculadora.lvMemoria.items = memoria


def apertar(aylis, texto):
    """Aperta um botão pelo texto dele."""
    if calc.tecla(texto):
        atualizar(aylis)


def form_key_down(aylis, sender: "Form", value: "KeyEvent"):
    # aylis.reply("key", "") engoliria a tecla.
    botao = botao_do_teclado(value.key, value.ctrl, value.shift)
    if botao == "historico":
        historico_click(aylis, frmCalculadora.btnHistorico)
    elif botao is not None:
        apertar(aylis, botao)


def limpar_historico_click(aylis, sender: "Button"):
    calc.historico = []
    atualizar(aylis)


def historico_item_click(aylis, sender: "ListView", value: "int"):
    # A lista mostra a conta mais nova em cima.
    i = value
    if 0 <= i < len(calc.historico):
        calc.usar(calc.historico[len(calc.historico) - 1 - i][1])
        atualizar(aylis)


def memoria_item_click(aylis, sender: "ListView", value: "int"):
    i = value
    if 0 <= i < len(calc.memoria):
        calc.usar(calc.memoria[len(calc.memoria) - 1 - i])
        atualizar(aylis)


def historico_click(aylis, sender: "Button"):
    frmCalculadora.pnlLateral.visible = not frmCalculadora.pnlLateral.visible


def tecla_click(aylis, sender: "Button"):
    apertar(aylis, sender.text)
