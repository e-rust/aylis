//! Vistas por faixa de largura (os breakpoints): as mudanças valem pela largura real do form
//! e voltam ao base; o JSON guarda o base e as vistas.

use aylis_core::*;

const FORM: &str = r#"{ "aylis": 1, "root": {
    "class": "Form", "name": "frmLoja", "width": 1200, "height": 700,
    "views": [
        { "name": "tablet", "max_width": 1023, "changes": { "pnlLado": { "width": 160 }, "": { "padding": 7 } } },
        { "name": "phone", "max_width": 599, "changes": { "pnlLado": { "visible": false }, "btnComprar": { "left": 8, "width": 300 } } }
    ],
    "children": [
        { "class": "Panel", "name": "pnlLado", "align": "left", "width": 260 },
        { "class": "Button", "name": "btnComprar", "left": 300, "top": 20, "width": 120 }
    ] } }"#;

fn prop(ui: &Ui, path: &str, name: &str) -> PropValue {
    ui.get_prop(ui.find_path(path).unwrap(), name).unwrap()
}

fn resize(ui: &mut Ui, form: ComponentId, width: f32) {
    ui.control_mut(form).unwrap().set_width(width);
    ui.layout();
}

#[test]
fn the_view_follows_the_width_and_the_json_keeps_the_base() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, FORM, None).unwrap();
    ui.layout();
    assert_eq!(ui.active_view(form), None, "1200: o base");
    assert_eq!(prop(&ui, "frmLoja.pnlLado", "width"), PropValue::Float(260.0));

    resize(&mut ui, form, 800.0);
    assert_eq!(ui.active_view(form), Some("tablet"));
    assert_eq!(prop(&ui, "frmLoja.pnlLado", "width"), PropValue::Float(160.0));
    assert_eq!(prop(&ui, "frmLoja", "padding"), PropValue::Bounds(Bounds::all(7.0)), "o próprio form (nome vazio)");

    resize(&mut ui, form, 400.0);
    assert_eq!(prop(&ui, "frmLoja", "padding"), PropValue::Bounds(Bounds::ZERO), "no celular, o padding do base");
    assert_eq!(ui.active_view(form), Some("phone"));
    assert_eq!(prop(&ui, "frmLoja.pnlLado", "visible"), PropValue::Bool(false));
    assert_eq!(prop(&ui, "frmLoja.pnlLado", "width"), PropValue::Float(260.0), "a largura do tablet não fica");
    assert_eq!(prop(&ui, "frmLoja.btnComprar", "width"), PropValue::Float(300.0));

    // Gravado com o celular ativo: o base no form, as vistas à parte.
    let json = aylis_json::save(&ui, form).unwrap();
    let doc: serde_json::Value = serde_json::from_str(&json).unwrap();
    let button = doc["root"]["children"].as_array().unwrap().iter().find(|c| c["name"] == "btnComprar").unwrap().clone();
    assert_eq!(button["width"], 120.0);
    assert_eq!(button["left"], 300.0);
    let side = doc["root"]["children"].as_array().unwrap().iter().find(|c| c["name"] == "pnlLado").unwrap().clone();
    assert!(side.get("visible").is_none(), "visível no base: {side}");
    let phone = doc["root"]["views"].as_array().unwrap().iter().find(|v| v["name"] == "phone").unwrap().clone();
    assert_eq!(phone["changes"]["btnComprar"]["width"], 300.0);
    assert_eq!(phone["max_width"], 599.0);

    // De volta ao largo: tudo como no base.
    resize(&mut ui, form, 1300.0);
    assert_eq!(ui.active_view(form), None);
    assert_eq!(prop(&ui, "frmLoja.pnlLado", "visible"), PropValue::Bool(true));
    assert_eq!(prop(&ui, "frmLoja.btnComprar", "left"), PropValue::Float(300.0));

    // O designer força uma vista, qualquer que seja a largura.
    ui.force_view(form, Some("phone"));
    ui.layout();
    assert_eq!(ui.active_view(form), Some("phone"));
    ui.force_view(form, None);
    ui.layout();
    assert_eq!(ui.active_view(form), None);
}

const CONDITIONS: &str = r#"{ "aylis": 1, "root": {
    "class": "Form", "name": "frmPainel", "width": 1200, "height": 700, "padding": [10, 10, 10, 10],
    "views": [
        { "name": "phone", "max_width": 599, "font_scale": 0.5, "spacing_scale": 2,
          "changes": { "pnlLado": { "parent": "pnlRodape", "index": 0, "align": "client" } } },
        { "name": "retrato", "max_width": 0, "orientation": "portrait", "changes": { "lblTitulo": { "visible": false } } },
        { "name": "baixa", "max_width": 0, "max_height": 300, "changes": { "lblTitulo": { "width": 50 } } },
        { "name": "outra", "max_width": 0, "platforms": ["plataforma-que-nao-existe"], "changes": { "lblTitulo": { "left": 99 } } }
    ],
    "children": [
        { "class": "Panel", "name": "pnlLado", "align": "left", "width": 200, "margins": [4, 4, 4, 4] },
        { "class": "Label", "name": "lblTitulo", "left": 300, "top": 10, "width": 200, "text": "Painel", "font_size": 20 },
        { "class": "Panel", "name": "pnlRodape", "align": "bottom", "height": 80, "children": [
            { "class": "Button", "name": "btnOk", "align": "right", "width": 90 }
        ] }
    ] } }"#;

fn resize_to(ui: &mut Ui, form: ComponentId, width: f32, height: f32) {
    ui.control_mut(form).unwrap().set_width(width).set_height(height);
    ui.layout();
}

/// A vista escolhida pela altura, pela orientação e pela plataforma, além da largura; a mais
/// estreita que vale ganha.
#[test]
fn the_view_follows_height_orientation_and_platform() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, CONDITIONS, None).unwrap();
    ui.layout();
    assert_eq!(ui.active_view(form), None, "1200 × 700: o base (a plataforma da vista outra não é esta)");
    resize_to(&mut ui, form, 700.0, 900.0);
    assert_eq!(ui.active_view(form), Some("retrato"));
    assert_eq!(prop(&ui, "frmPainel.lblTitulo", "visible"), PropValue::Bool(false));
    resize_to(&mut ui, form, 900.0, 250.0);
    assert_eq!(ui.active_view(form), Some("baixa"));
    assert_eq!(prop(&ui, "frmPainel.lblTitulo", "visible"), PropValue::Bool(true), "a do retrato voltou");
    assert_eq!(prop(&ui, "frmPainel.lblTitulo", "width"), PropValue::Float(50.0));
    resize_to(&mut ui, form, 400.0, 900.0);
    assert_eq!(ui.active_view(form), Some("phone"), "a de largura ganha da de orientação");
    let json = aylis_json::save(&ui, form).unwrap();
    let doc: serde_json::Value = serde_json::from_str(&json).unwrap();
    let views = doc["root"]["views"].as_array().unwrap();
    let retrato = views.iter().find(|v| v["name"] == "retrato").unwrap();
    assert_eq!(retrato["orientation"], "portrait");
    assert!(retrato.get("font_scale").is_none(), "o padrão não é gravado");
    let outra = views.iter().find(|v| v["name"] == "outra").unwrap();
    assert_eq!(outra["platforms"][0], "plataforma-que-nao-existe");
    let baixa = views.iter().find(|v| v["name"] == "baixa").unwrap();
    assert_eq!(baixa["max_height"], 300.0);
}

/// A vista muda um componente de contêiner e volta; o JSON gravado com ela ativa guarda a
/// árvore do base, e a escala de fonte e de espaços vale para o form inteiro.
#[test]
fn the_view_moves_a_component_and_scales_fonts_and_spacing() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, CONDITIONS, None).unwrap();
    ui.layout();
    let side = ui.find_path("frmPainel.pnlLado").unwrap();
    let footer = ui.find_path("frmPainel.pnlRodape").unwrap();
    let label = ui.find_path("frmPainel.lblTitulo").unwrap();
    assert_eq!(ui.parent(side), Some(form));
    assert_eq!(ui.font_of(label).size, 20.0);
    let side_base = ui.absolute_rect(side).unwrap();
    assert_eq!(side_base.x, 14.0, "padding 10 + margem 4");

    resize_to(&mut ui, form, 400.0, 600.0);
    assert_eq!(ui.active_view(form), Some("phone"));
    assert_eq!(ui.parent(side), Some(footer), "no celular, no rodapé");
    assert_eq!(ui.children(footer)[0], side, "na posição pedida");
    assert_eq!(ui.font_of(label).size, 10.0, "a fonte na metade");
    assert_eq!(prop(&ui, "frmPainel.lblTitulo", "font_size"), PropValue::Float(20.0), "a propriedade fica a do desenho");
    assert_eq!(prop(&ui, "frmPainel", "padding"), PropValue::Bounds(Bounds::all(10.0)), "o padding também");
    let footer_rect = ui.absolute_rect(footer).unwrap();
    assert_eq!(footer_rect.x, 20.0, "os espaços em dobro: padding 20");

    // Gravado no celular: a árvore e as propriedades do base.
    let json = aylis_json::save(&ui, form).unwrap();
    let doc: serde_json::Value = serde_json::from_str(&json).unwrap();
    let names: Vec<&str> = doc["root"]["children"].as_array().unwrap().iter().map(|c| c["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["pnlLado", "lblTitulo", "pnlRodape"]);
    let rodape = doc["root"]["children"].as_array().unwrap().iter().find(|c| c["name"] == "pnlRodape").unwrap().clone();
    assert_eq!(rodape["children"].as_array().unwrap().len(), 1, "só o botão: {rodape}");
    let phone = doc["root"]["views"].as_array().unwrap().iter().find(|v| v["name"] == "phone").unwrap().clone();
    assert_eq!(phone["changes"]["pnlLado"]["parent"], "pnlRodape");
    assert_eq!(phone["changes"]["pnlLado"]["index"], 0);
    assert_eq!(phone["font_scale"], 0.5);
    assert_eq!(phone["spacing_scale"], 2.0);

    // O JSON gravado carrega igual.
    let mut other = Ui::new();
    aylis_controls::register(&mut other);
    let again = aylis_json::load(&mut other, &json, None).unwrap();
    other.layout();
    let moved = other.find_path("frmPainel.pnlLado").unwrap();
    assert_eq!(other.parent(moved), other.find_path("frmPainel.pnlRodape"), "gravado com 400 de largura: o celular");
    resize_to(&mut other, again, 1200.0, 700.0);
    assert_eq!(other.parent(moved), Some(again));

    // De volta ao largo: no lugar, na ordem e no tamanho de antes.
    resize_to(&mut ui, form, 1200.0, 700.0);
    assert_eq!(ui.active_view(form), None);
    assert_eq!(ui.children(form)[0], side);
    assert_eq!(ui.font_of(label).size, 20.0);
    assert_eq!(ui.absolute_rect(side).unwrap(), side_base);
}

/// O `column_span` no `GridLayout`: ocupa várias colunas, pula para a linha seguinte quando
/// não cabe e, numa grade mais estreita, fica na linha inteira.
#[test]
fn the_grid_child_spans_columns() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(
        &mut ui,
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmGrade", "width": 430, "height": 400, "children": [
        { "class": "GridLayout", "name": "grd", "align": "client", "columns": 4, "gap": 10, "children": [
            { "class": "Panel", "name": "a", "height": 40 },
            { "class": "Panel", "name": "b", "height": 40, "column_span": 2 },
            { "class": "Panel", "name": "c", "height": 40, "column_span": 2 },
            { "class": "Panel", "name": "d", "height": 40, "column_span": 9 }
        ] } ] } }"#,
        None,
    )
    .unwrap();
    ui.layout();
    let rect = |name: &str| ui.absolute_rect(ui.find_path(&format!("frmGrade.grd.{name}")).unwrap()).unwrap();
    // 4 colunas de 100 com 10 entre elas.
    assert_eq!((rect("a").x, rect("a").width), (0.0, 100.0));
    assert_eq!((rect("b").x, rect("b").width, rect("b").y), (110.0, 210.0, 0.0));
    assert_eq!((rect("c").x, rect("c").width, rect("c").y), (0.0, 210.0, 50.0), "não cabe na primeira linha");
    assert_eq!((rect("d").x, rect("d").width, rect("d").y), (0.0, 430.0, 100.0), "mais que a grade: a linha toda");
    assert_eq!(prop(&ui, "frmGrade.grd.b", "column_span"), PropValue::Int(2));
    let json = aylis_json::save(&ui, form).unwrap();
    assert!(json.contains("\"column_span\": 2") || json.contains("\"column_span\":2"), "{json}");
}

/// Várias mudanças de lugar no mesmo contêiner (saindo, entrando, só trocando a ordem): a
/// árvore gravada e a volta ao base são as do desenho, ida e volta mais de uma vez.
#[test]
fn several_moves_in_one_container_come_back_in_order() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(
        &mut ui,
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "frm", "width": 1000, "height": 600,
        "views": [ { "name": "phone", "max_width": 599, "changes": {
            "a": { "parent": "q" },
            "c": { "parent": "q", "index": 0 },
            "e": { "index": 0 },
            "d": { "parent": "" },
            "x": { "parent": "p", "index": 1 }
        } } ],
        "children": [
            { "class": "Panel", "name": "p", "children": [
                { "class": "Panel", "name": "a" }, { "class": "Panel", "name": "b" }, { "class": "Panel", "name": "c" },
                { "class": "Panel", "name": "d" }, { "class": "Panel", "name": "e" }
            ] },
            { "class": "Panel", "name": "q", "children": [ { "class": "Panel", "name": "x" }, { "class": "Panel", "name": "y" } ] }
        ] } }"#,
        None,
    )
    .unwrap();
    ui.layout();
    let id = |name: &str| ui.find_in(form, name).unwrap();
    let names = |ui: &Ui, list: &[ComponentId]| list.iter().map(|&c| ui.name(c).unwrap().to_string()).collect::<Vec<_>>();
    let (p, q) = (id("p"), id("q"));
    let base_p = names(&ui, ui.children(p));
    let base_q = names(&ui, ui.children(q));
    let base_form = names(&ui, ui.children(form));
    assert_eq!(base_p, ["a", "b", "c", "d", "e"]);

    for round in 0..2 {
        resize(&mut ui, form, 400.0);
        assert_eq!(ui.active_view(form), Some("phone"));
        assert_eq!(names(&ui, ui.children(p)), ["e", "x", "b"], "rodada {round}");
        assert_eq!(names(&ui, ui.children(q)), ["c", "y", "a"], "rodada {round}");
        assert_eq!(names(&ui, ui.children(form)), ["p", "q", "d"], "rodada {round}");
        // O base visto de fora, com a vista ativa (o que o JSON grava).
        assert_eq!(names(&ui, &ui.base_children(p)), base_p, "rodada {round}");
        assert_eq!(names(&ui, &ui.base_children(q)), base_q, "rodada {round}");
        assert_eq!(names(&ui, &ui.base_children(form)), base_form, "rodada {round}");
        let doc: serde_json::Value = serde_json::from_str(&aylis_json::save(&ui, form).unwrap()).unwrap();
        let saved_p: Vec<&str> = doc["root"]["children"][0]["children"].as_array().unwrap().iter().map(|c| c["name"].as_str().unwrap()).collect();
        assert_eq!(saved_p, base_p, "rodada {round}");

        resize(&mut ui, form, 1000.0);
        assert_eq!(ui.active_view(form), None);
        assert_eq!(names(&ui, ui.children(p)), base_p, "rodada {round}");
        assert_eq!(names(&ui, ui.children(q)), base_q, "rodada {round}");
        assert_eq!(names(&ui, ui.children(form)), base_form, "rodada {round}");
    }
}
