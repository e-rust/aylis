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
    std::vector<aylis_components::ListViewItem> itens;
    for (auto it = calc.historico.rbegin(); it != calc.historico.rend(); ++it) itens.push_back({motor::formatar(it->resultado), it->expressao});
    if (itens.empty()) itens.push_back({"Ainda não há histórico", ""});
    frmCalculadora.lvHistorico.set_items(itens);
    itens.clear();
    for (auto it = calc.memoria.rbegin(); it != calc.memoria.rend(); ++it) itens.push_back({motor::formatar(*it), ""});
    if (itens.empty()) itens.push_back({"Não há nada salvo na memória", ""});
    frmCalculadora.lvMemoria.set_items(itens);
}

// Aperta um botão pelo texto dele.
static void apertar(aylis::Aylis& app, const std::string& texto) {
    if (calc.tecla(texto)) atualizar(app);
}

void form_key_down(aylis::Aylis& app, const aylis::Event& ev) {
    std::string botao = motor::botao_do_teclado(ev.args["key"].str(), ev.args["ctrl"].boolean(), ev.args["shift"].boolean());
    if (botao == "historico") historico_click(app, frmCalculadora.btnHistorico);
    else if (!botao.empty()) apertar(app, botao);
}

void limpar_historico_click(aylis::Aylis& app, aylis_components::Button sender) {
    (void)sender;
    calc.historico.clear();
    atualizar(app);
}

void historico_item_click(aylis::Aylis& app, aylis_components::ListView sender, long long value) {
    (void)sender;
    // A lista mostra a conta mais nova em cima.
    if (value >= 0 && value < static_cast<long long>(calc.historico.size())) {
        size_t i = static_cast<size_t>(value);
        calc.usar(calc.historico[calc.historico.size() - 1 - i].resultado);
        atualizar(app);
    }
}

void memoria_item_click(aylis::Aylis& app, aylis_components::ListView sender, long long value) {
    (void)sender;
    if (value >= 0 && value < static_cast<long long>(calc.memoria.size())) {
        size_t i = static_cast<size_t>(value);
        calc.usar(calc.memoria[calc.memoria.size() - 1 - i]);
        atualizar(app);
    }
}

void historico_click(aylis::Aylis& app, aylis_components::Button sender) {
    (void)app;
    (void)sender;
    frmCalculadora.pnlLateral.set_visible(!frmCalculadora.pnlLateral.visible());
}

void tecla_click(aylis::Aylis& app, aylis_components::Button sender) {
    std::string texto = sender.text();
    apertar(app, texto);
}

}  // namespace frm_calculadora
