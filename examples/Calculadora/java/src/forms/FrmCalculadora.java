// Eventos do form frmCalculadora: o código é seu (o registro dos handlers fica no Forms.java
// gerado). Dentro de um handler, o form é a janela do evento.

package forms;

import dev.aylis.Aylis;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import static forms.Components.frmCalculadora;

public final class FrmCalculadora {
    static final Calculadora calc = new Calculadora();

    /** Os botões que só funcionam sem erro no visor (os da janela do evento). */
    static Components.Button[] semErro() {
        return new Components.Button[] {
            frmCalculadora.btnMMais, frmCalculadora.btnMMenos, frmCalculadora.btnMS, frmCalculadora.btnPorcento,
            frmCalculadora.btnInverso, frmCalculadora.btnQuadrado, frmCalculadora.btnRaiz, frmCalculadora.btnDividir,
            frmCalculadora.btnMultiplicar, frmCalculadora.btnSubtrair, frmCalculadora.btnSomar, frmCalculadora.btnSinal,
            frmCalculadora.btnVirgula,
        };
    }

    /** Põe o estado da calculadora na tela. */
    static void atualizar(Aylis aylis) {
        String visor = calc.visor();
        // O número encolhe para caber, como no Windows.
        int n = visor.codePointCount(0, visor.length());
        int tamanho = calc.temErro() ? 26 : n <= 11 ? 46 : Math.max(46 * 11 / n, 22);
        frmCalculadora.lblVisor.setText(visor);
        frmCalculadora.lblVisor.setFontSize(tamanho);
        frmCalculadora.lblExpressao.setText(calc.expressao);
        boolean erro = calc.temErro();
        for (Components.Button botao : semErro()) botao.setEnabled(!erro);
        for (Components.Button botao : new Components.Button[] {frmCalculadora.btnMC, frmCalculadora.btnMR}) botao.setEnabled(!erro && !calc.memoria.isEmpty());
        List<String> itens = new ArrayList<>();
        for (int i = calc.historico.size() - 1; i >= 0; i--) {
            Calculadora.Conta c = calc.historico.get(i);
            itens.add(Calculadora.formatar(c.resultado()) + "|" + c.expressao());
        }
        frmCalculadora.lvHistorico.setItems(itens.isEmpty() ? "Ainda não há histórico|" : String.join(";", itens));
        itens.clear();
        for (int i = calc.memoria.size() - 1; i >= 0; i--) itens.add(Calculadora.formatar(calc.memoria.get(i)) + "|");
        frmCalculadora.lvMemoria.setItems(itens.isEmpty() ? "Não há nada salvo na memória|" : String.join(";", itens));
    }

    /** Aperta um botão pelo texto dele. */
    static void apertar(Aylis aylis, String texto) {
        if (calc.tecla(texto)) atualizar(aylis);
    }

    static int indice(Aylis.Event ev) {
        return ev.args() instanceof Number n ? n.intValue() : -1;
    }

    public static void form_key_down(Aylis aylis, Aylis.Event ev) {
        if (!(ev.args() instanceof Map<?, ?> args)) return;
        String tecla = String.valueOf(args.get("key"));
        boolean ctrl = Boolean.TRUE.equals(args.get("ctrl"));
        boolean shift = Boolean.TRUE.equals(args.get("shift"));
        String botao = Calculadora.botaoDoTeclado(tecla, ctrl, shift);
        if ("historico".equals(botao)) historico_click(aylis, ev);
        else if (botao != null) apertar(aylis, botao);
    }

    public static void limpar_historico_click(Aylis aylis, Aylis.Event ev) {
        calc.historico.clear();
        atualizar(aylis);
    }

    public static void historico_item_click(Aylis aylis, Aylis.Event ev) {
        // A lista mostra a conta mais nova em cima.
        int i = indice(ev);
        if (i >= 0 && i < calc.historico.size()) {
            calc.usar(calc.historico.get(calc.historico.size() - 1 - i).resultado());
            atualizar(aylis);
        }
    }

    public static void memoria_item_click(Aylis aylis, Aylis.Event ev) {
        int i = indice(ev);
        if (i >= 0 && i < calc.memoria.size()) {
            calc.usar(calc.memoria.get(calc.memoria.size() - 1 - i));
            atualizar(aylis);
        }
    }

    public static void historico_click(Aylis aylis, Aylis.Event ev) {
        frmCalculadora.pnlLateral.setVisible(!frmCalculadora.pnlLateral.isVisible());
    }

    public static void tecla_click(Aylis aylis, Aylis.Event ev) {
        apertar(aylis, frmCalculadora.senderAsButton(ev).getText());
    }
}
