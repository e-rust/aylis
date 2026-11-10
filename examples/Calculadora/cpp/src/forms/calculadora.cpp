// Eventos do form frmCalculadora: o código é seu (as declarações e o registro dos handlers
// ficam no forms.hpp gerado). Dentro de um handler, o form é a janela do evento.

#include "../aylis.hpp"
#include "motor.hpp"
#include "forms.hpp"

namespace frm_calculadora {

static motor::Calculadora calc;

// Os botões que só funcionam sem erro no visor.
static const aylis_components::Button* SEM_ERRO[] = {
    &frmCalculadora.btnMMais, &frmCalculadora.btnMMenos, &frmCalculadora.btnMS, &frmCalculadora.btnPorcento,
    &frmCalculadora.btnInverso, &frmCalculadora.btnQuadrado, &frmCalculadora.btnRaiz, &frmCalculadora.btnDividir,
    &frmCalculadora.btnMultiplicar, &frmCalculadora.btnSubtrair, &frmCalculadora.btnSomar, &frmCalculadora.btnSinal,
    &frmCalculadora.btnVirgula,
};

// Quantos caracteres (não bytes) o texto UTF-8 tem.
static int caracteres(const std::string& s) {
    int n = 0;
    for (unsigned char c : s) {
        if ((c & 0xC0) != 0x80) n++;
    }
    return n;
}

static std::string juntar(const std::vector<std::string>& itens) {
    std::string out;
    for (size_t i = 0; i < itens.size(); i++) out += (i ? ";" : "") + itens[i];
    return out;
}

// Põe o estado da calculadora na tela.
static void atualizar(aylis::Aylis& app) {
    std::string visor = calc.visor();
    // O número encolhe para caber, como no Windows.
    int n = caracteres(visor);
    int tamanho = calc.tem_erro() ? 26 : n <= 11 ? 46 : std::max(46 * 11 / n, 22);
    frmCalculadora.lblVisor.set_text(visor);
    frmCalculadora.lblVisor.set_font_size(tamanho);
    frmCalculadora.lblExpressao.set_text(calc.expressao);
    bool erro = calc.tem_erro();
    for (const auto* botao : SEM_ERRO) botao->set_enabled(!erro);
    for (const auto* botao : {&frmCalculadora.btnMC, &frmCalculadora.btnMR}) botao->set_enabled(!erro && !calc.memoria.empty());
    std::vector<std::string> itens;
    for (auto it = calc.historico.rbegin(); it != calc.historico.rend(); ++it) itens.push_back(motor::formatar(it->resultado) + "|" + it->expressao);
    frmCalculadora.lvHistorico.set_items(itens.empty() ? std::string("Ainda não há histórico|") : juntar(itens));
    itens.clear();
    for (auto it = calc.memoria.rbegin(); it != calc.memoria.rend(); ++it) itens.push_back(motor::formatar(*it) + "|");
    frmCalculadora.lvMemoria.set_items(itens.empty() ? std::string("Não há nada salvo na memória|") : juntar(itens));
}

// Aperta um botão pelo texto dele.
static void apertar(aylis::Aylis& app, const std::string& texto) {
    if (calc.tecla(texto)) atualizar(app);
}

static int indice(const aylis::Event& ev) {
    return ev.args.is_null() ? -1 : static_cast<int>(ev.args.number());
}

void form_key_down(aylis::Aylis& app, const aylis::Event& ev) {
    std::string botao = motor::botao_do_teclado(ev.args["key"].str(), ev.args["ctrl"].boolean(), ev.args["shift"].boolean());
    if (botao == "historico") historico_click(app, ev);
    else if (!botao.empty()) apertar(app, botao);
}

void limpar_historico_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    calc.historico.clear();
    atualizar(app);
}

void historico_item_click(aylis::Aylis& app, const aylis::Event& ev) {
    // A lista mostra a conta mais nova em cima.
    int i = indice(ev);
    if (i >= 0 && i < static_cast<int>(calc.historico.size())) {
        calc.usar(calc.historico[calc.historico.size() - 1 - i].resultado);
        atualizar(app);
    }
}

void memoria_item_click(aylis::Aylis& app, const aylis::Event& ev) {
    int i = indice(ev);
    if (i >= 0 && i < static_cast<int>(calc.memoria.size())) {
        calc.usar(calc.memoria[calc.memoria.size() - 1 - i]);
        atualizar(app);
    }
}

void historico_click(aylis::Aylis& app, const aylis::Event& ev) {
    (void)ev;
    frmCalculadora.pnlLateral.set_visible(!frmCalculadora.pnlLateral.visible());
}

void tecla_click(aylis::Aylis& app, const aylis::Event& ev) {
    std::string texto = frmCalculadora.sender_as_button(ev).text();
    apertar(app, texto);
}

}  // namespace frm_calculadora
