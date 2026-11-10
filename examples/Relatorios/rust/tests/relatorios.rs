//! Os dois relatórios do exemplo, montados sem janela: cada pedido sai com os itens dele, a
//! soma dos itens bate com o total do pedido, as parcelas aparecem no relatório por vendedor
//! e o PDF é gerado. `AYLIS_REPORT_PDF=pasta` grava os PDFs para olhar.

use aylis::core::{EventArgs, Ui};
use aylis::core::pages::PageSet;
use aylis::data::view;
use aylis::dbcontrols::report;

fn aberto() -> aylis::Application {
    let mut app = relatorios::app().expect("o app abre");
    let form = app.ui.find_path("frmPrincipal").unwrap();
    app.ui.fire(form, "on_show", &EventArgs::None);
    app.ui.layout();
    app
}

fn textos(pages: &PageSet) -> Vec<String> {
    pages.pages.iter().flat_map(|p| p.texts()).map(str::to_string).collect()
}

fn salvar(ui: &mut Ui, nome: &str) {
    if let Ok(pasta) = std::env::var("AYLIS_REPORT_PDF") {
        let rep = ui.find_path(&format!("frmPrincipal.{nome}")).unwrap();
        std::fs::write(std::path::Path::new(&pasta).join(format!("{nome}.pdf")), report::to_pdf(ui, rep)).unwrap();
    }
}

#[test]
fn pedidos_com_itens_somam_o_total_de_cada_pedido() {
    let mut app = aberto();
    let ui = &mut app.ui;
    let pedidos = ui.find_path("frmPrincipal.cdsPedidos").unwrap();
    let itens = ui.find_path("frmPrincipal.cdsItens").unwrap();
    assert_eq!(view(ui, pedidos).record_count(), 36);
    // O detalhe filtra pelo pedido atual (o primeiro), não traz todos.
    let do_primeiro = view(ui, itens).record_count();
    assert!((2..=7).contains(&do_primeiro), "{do_primeiro} itens");
    let atual = view(ui, pedidos).rec_no();

    let rep = ui.find_path("frmPrincipal.repPedidos").unwrap();
    let pages = report::prepare(ui, rep);
    assert!(pages.pages.len() >= 2, "{} páginas", pages.pages.len());
    let t = textos(&pages);
    // Cada pedido: o título dele, o total no cabeçalho e o mesmo total na soma dos itens.
    let v = view(ui, pedidos);
    for row in 0..v.record_count() {
        let numero = v.text_at(row, "numero").unwrap();
        let total = v.text_at(row, "total").unwrap();
        let itens = v.text_at(row, "itens").unwrap();
        let at = t.iter().position(|s| *s == format!("Pedido nº {numero}")).unwrap_or_else(|| panic!("pedido {numero}"));
        let resumo = format!("{itens} itens do pedido {numero}");
        let fim = t[at..].iter().position(|s| s.starts_with(&resumo)).unwrap_or_else(|| panic!("{resumo}")) + at;
        // A soma dos itens (o rodapé do detalhe) vem logo depois do resumo.
        assert_eq!(t[fim + 2], total, "pedido {numero}");
        assert!(t[at..fim].contains(&total), "o total do pedido {numero} no cabeçalho");
    }
    assert!(t.iter().any(|s| s.starts_with("R$ ")), "valores em reais");
    assert!(!t.iter().any(|s| s.contains('[')), "sobrou um colchete: {:?}", t.iter().find(|s| s.contains('[')));
    // O cursor do mestre volta para onde estava.
    assert_eq!(view(ui, pedidos).rec_no(), atual);
    salvar(ui, "repPedidos");
}

#[test]
fn vendas_por_vendedor_com_itens_e_parcelas() {
    let mut app = aberto();
    let ui = &mut app.ui;
    let rep = ui.find_path("frmPrincipal.repVendedores").unwrap();
    let pages = report::prepare(ui, rep);
    let t = textos(&pages);
    for vendedor in ["Ana Souza", "Bruno Lima", "Carla Mendes", "Diego Rocha"] {
        assert_eq!(t.iter().filter(|s| **s == format!("Vendedor: {vendedor}")).count(), 1, "{vendedor}");
        assert!(t.iter().any(|s| s.starts_with(&format!("Total de {vendedor}: "))), "{vendedor}");
    }
    assert!(t.iter().any(|s| s.starts_with("Pagamento em ")), "as parcelas");
    assert!(t.iter().any(|s| s.contains("ª · vence em ")), "as parcelas");
    assert!(!t.iter().any(|s| s.contains('[')), "sobrou um colchete: {:?}", t.iter().find(|s| s.contains('[')));
    assert!(report::to_pdf(ui, rep).starts_with(b"%PDF"));
    salvar(ui, "repVendedores");
}
