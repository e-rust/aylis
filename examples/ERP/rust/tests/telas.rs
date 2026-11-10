//! O ERP de exemplo sem janela (item 21): o menu lista as telas, todas abrem e carregam do
//! banco, e um pedido novo com itens grava numa vez só, com os itens recebendo o id do pedido.

use std::time::Instant;

use aylis::prelude::*;

fn app() -> aylis::Application {
    let pasta = std::env::temp_dir().join(format!("aylis-erp-teste-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&pasta);
    // SAFETY: o teste é o único do arquivo e roda antes de abrir o banco (uma thread só).
    unsafe { std::env::set_var("AYLIS_ERP_DB", pasta.join("erp.db")) };
    erp::app().expect("o projeto abre")
}

fn clicar(ui: &mut Ui, form: ComponentId, botao: &str) {
    let id = ui.find_in(form, botao).unwrap_or_else(|| panic!("{botao}"));
    ui.fire(id, "on_click", &EventArgs::None);
}

fn status(ui: &Ui, form: ComponentId) -> String {
    ui.get_prop(ui.find_in(form, "lblStatus").unwrap(), "text").unwrap().to_str().unwrap().to_string()
}

#[test]
fn every_screen_opens_and_a_new_order_saves_with_its_items() {
    let mut app = app();
    let ui = &mut app.ui;
    let menu = app.main_form;
    ui.show(menu).unwrap();
    let telas = ui.find_in(menu, "cdsTelas").unwrap();
    assert_eq!(view(ui, telas).record_count(), 1008, "{}", status(ui, menu));

    // Todas as telas: criar, mostrar (carrega do banco) e liberar.
    let inicio = Instant::now();
    let mut abertas = 0;
    for tela in erp::erp::esquema::TELAS {
        let form = ui.create_form(tela.form).unwrap_or_else(|e| panic!("{}: {e}", tela.form));
        ui.show(form).unwrap();
        let texto = status(ui, form);
        assert!(texto.ends_with("registros"), "{}: {texto}", tela.form);
        ui.close(form);
        ui.free(form);
        abertas += 1;
    }
    let tempo = inicio.elapsed();
    let cada = tempo.as_secs_f64() * 1000.0 / abertas as f64;
    eprintln!("ERP: {abertas} telas abertas e fechadas em {:.2} s ({cada:.1} ms cada)", tempo.as_secs_f64());
    // O limite (medido em 04/11: 7,7 ms em debug nesta máquina), com folga para máquinas lentas.
    assert!(cada < 60.0, "abrir uma tela levou {cada:.1} ms");

    // Um pedido de venda novo com dois itens, gravado de uma vez.
    let form = ui.create_form("frmVenPedidosDeVendaCad").unwrap();
    ui.show(form).unwrap();
    let pedidos = ui.find_in(form, "cdsMestre").unwrap();
    let itens = ui.find_in(form, "cdsItens").unwrap();
    let antes = view(ui, pedidos).record_count();
    clicar(ui, form, "btnNovo");
    data_set(ui, pedidos).set_value("parceiro", "Cliente do teste").unwrap();
    for produto in ["Caneta", "Caderno"] {
        clicar(ui, form, "btnNovoItem");
        let mut ds = data_set(ui, itens);
        ds.set_value("item", produto).unwrap();
        ds.set_value("quantidade", 2).unwrap();
    }
    clicar(ui, form, "btnSalvar");
    assert_eq!(status(ui, form), "3 alterações gravadas");
    let id = view(ui, pedidos).value("id").unwrap();
    assert!(matches!(id, DataValue::Int(n) if n > 0), "o pedido ficou com o id do banco: {id:?}");
    assert_eq!(view(ui, itens).record_count(), 2, "os itens continuam no pedido novo");
    assert_eq!(view(ui, itens).value("mestre_id").unwrap(), id);

    // Reabrir do banco: o pedido e os itens estão lá.
    ui.close(form);
    ui.free(form);
    let form = ui.create_form("frmVenPedidosDeVendaCad").unwrap();
    ui.show(form).unwrap();
    let pedidos = ui.find_in(form, "cdsMestre").unwrap();
    assert_eq!(view(ui, pedidos).record_count(), antes + 1);
    assert!(data_set(ui, pedidos).locate("id", id).unwrap());
    assert_eq!(view(ui, ui.find_in(form, "cdsItens").unwrap()).record_count(), 2);

    // A busca filtra pelo cliente.
    let busca = ui.find_in(form, "edtBusca").unwrap();
    ui.fire(busca, "on_change", &EventArgs::Text("do teste".into()));
    assert_eq!(view(ui, pedidos).record_count(), 1);
}
