use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::Ui;
use aylis_remote::*;
use serde_json::{Value, json};

fn ui() -> Ui {
    let mut ui = Ui::new();
    register_all(&mut ui);
    ui
}

fn run(ui: &mut Ui, command: Value) -> Value {
    execute(ui, &command).unwrap_or_else(|e| panic!("{command}: {e}"))
}

/// Guarda os eventos que iriam para o backend.
fn capture(ui: &mut Ui) -> Rc<RefCell<Vec<Value>>> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let e = events.clone();
    forward_events(ui, move |_, event| e.borrow_mut().push(event));
    events
}

#[test]
fn create_set_get_and_events() {
    let mut ui = ui();
    let events = capture(&mut ui);
    let form = run(&mut ui, json!({ "cmd": "create", "class": "Form", "name": "Frm", "props": { "caption": "Remoto" } }));
    assert!(form["id"].as_u64().is_some());
    run(&mut ui, json!({
        "cmd": "create", "class": "Button", "parent": "Frm", "name": "btnOk",
        "props": { "text": "OK", "align": "top", "margins": [4, 8], "on_click": "ok_click" }
    }));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "btnOk", "prop": "text" })), "OK");
    assert_eq!(
        run(&mut ui, json!({ "cmd": "get", "target": "btnOk", "props": ["align", "margins"] })),
        json!({ "align": "top", "margins": [4, 8] })
    );

    run(&mut ui, json!({ "cmd": "click", "target": "btnOk" }));
    let events = events.borrow();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["event"], "on_click");
    assert_eq!(events[0]["handler"], "ok_click");
    assert_eq!(events[0]["sender"]["name"], "btnOk");
    assert_eq!(events[0]["sender"]["class"], "Button");
}

#[test]
fn the_backend_answers_events_with_commands() {
    // O "backend" responde ao clique mudando o texto de outro componente.
    let mut ui = ui();
    forward_events(&mut ui, |ui, event| {
        if event["handler"] == "salvar" {
            execute(ui, &json!({ "cmd": "set", "target": "lblStatus", "props": { "text": "Salvo!" } })).unwrap();
        }
    });
    run(&mut ui, json!({ "cmd": "load", "layout": {
        "class": "Form", "name": "Frm", "children": [
            { "class": "Button", "name": "btnSalvar", "on_click": "salvar" },
            { "class": "Label", "name": "lblStatus" }
        ]
    }}));
    run(&mut ui, json!({ "cmd": "click", "target": "btnSalvar" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "lblStatus", "prop": "text" })), "Salvo!");
}

#[test]
fn set_takes_one_prop_or_many() {
    // `prop` e `value` (como no `get`, e como a descrição do `app_command` do MCP mostra) ou `props`.
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "Frm", "children": [{ "class": "Edit", "name": "edtNome" }] } }));
    run(&mut ui, json!({ "cmd": "set", "target": "edtNome", "prop": "text", "value": "Ana" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "edtNome", "prop": "text" })), "Ana");
    run(&mut ui, json!({ "cmd": "set", "target": "edtNome", "props": { "text": "Bia" } }));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "edtNome", "prop": "text" })), "Bia");
    let e = execute(&mut ui, &json!({ "cmd": "set", "target": "edtNome", "prop": "text" })).unwrap_err();
    assert!(e.contains("value"), "{e}");
    let e = execute(&mut ui, &json!({ "cmd": "set", "target": "edtNome" })).unwrap_err();
    assert!(e.contains("props"), "{e}");
}

#[test]
fn responses_and_errors() {
    let mut ui = ui();
    let ok = handle_message(&mut ui, &json!({ "id": 7, "cmd": "find", "name": "nada" })).unwrap();
    assert_eq!(ok, json!({ "id": 7, "result": null }));

    let err = handle_message(&mut ui, &json!({ "id": 8, "cmd": "set", "target": "nada", "props": {} })).unwrap();
    assert_eq!(err["id"], 8);
    assert!(err["error"].as_str().unwrap().contains("não encontrado"));

    // sem id: sucesso não responde; erro vira evento "error"
    assert!(handle_message(&mut ui, &json!({ "cmd": "create", "class": "Form" })).is_none());
    let event = handle_message(&mut ui, &json!({ "cmd": "voar" })).unwrap();
    assert_eq!(event["event"], "error");
    assert!(event["message"].as_str().unwrap().contains("voar"));

    // propriedade errada: nada fica criado pela metade
    let before = ui.components().count();
    let e = execute(&mut ui, &json!({ "cmd": "create", "class": "Button", "props": { "texto": "x" } })).unwrap_err();
    assert!(e.contains("texto"), "{e}");
    assert_eq!(ui.components().count(), before);
}

#[test]
fn wake_echoes_the_post_event() {
    let mut ui = ui();
    // O Post de outra thread do backend: o eco acorda o laço dele (com ou sem id).
    assert_eq!(handle_message(&mut ui, &json!({ "cmd": "wake" })), Some(json!({ "event": "__post" })));
    assert_eq!(handle_message(&mut ui, &json!({ "cmd": "wake", "id": 3 })), Some(json!({ "event": "__post" })));
}

#[test]
fn batch_runs_several_commands() {
    let mut ui = ui();
    let results = run(&mut ui, json!({ "cmd": "batch", "commands": [
        { "cmd": "create", "class": "Form", "name": "F" },
        { "cmd": "create", "class": "Edit", "parent": "F", "name": "e", "props": { "text": "abc" } },
        { "cmd": "get", "target": "e", "prop": "text" },
        { "cmd": "get", "target": "x", "prop": "text" }
    ]}));
    assert_eq!(results[2], "abc");
    assert!(results[3]["error"].is_string());
}

#[test]
fn data_round_trip_with_a_remote_backend() {
    let mut ui = ui();
    let events = capture(&mut ui);
    run(&mut ui, json!({ "cmd": "locale", "locale": "pt-BR" }));
    run(&mut ui, json!({ "cmd": "load", "layout": {
        "class": "Form", "name": "Frm", "children": [
            { "class": "ClientDataSet", "name": "cds", "key_fields": "id", "active": true,
              "on_apply_updates": "gravar_clientes",
              "children": [
                { "class": "Field", "name": "id", "type": "auto_inc" },
                { "class": "Field", "name": "nome", "type": "string", "required": true },
                { "class": "Field", "name": "limite", "type": "currency", "display_format": "R$ #,##0.00" }
              ] },
            { "class": "DataSource", "name": "ds", "data_set": "cds" },
            { "class": "Edit", "name": "edtNome", "bindings": { "text": { "source": "ds", "field": "nome" } } }
        ]
    }}));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "load", "rows": [
        { "id": 1, "nome": "Ana", "limite": "100.00" },
        { "id": 2, "nome": "Bia", "limite": "250.50" }
    ]}));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "edtNome", "prop": "text" })), "Ana");

    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "next" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "text", "field": "limite" })), "R$ 250,50");
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "edit" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_text", "field": "limite", "text": "300" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "append_record" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_value", "field": "nome", "value": "Caio" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "post" }));
    let info = run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }));
    assert_eq!(info["changes"], 2);
    assert_eq!(info["state"], "browse");

    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "apply" }));
    let delta = events.borrow().last().cloned().unwrap();
    assert_eq!(delta["handler"], "gravar_clientes");
    let changes = delta["args"]["changes"].as_array().unwrap();
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0]["op"], "update");
    assert_eq!(changes[0]["new"]["limite"], "300.00");
    assert_eq!(changes[1]["op"], "insert");

    // o backend grava e devolve o id do novo
    let results = json!([
        { "ref": changes[0]["ref"] },
        { "ref": changes[1]["ref"], "values": { "id": 77 } }
    ]);
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "reconcile", "results": results }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["changes"], 0);
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "last" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "record" })), json!({ "id": 77, "nome": "Caio", "limite": null }));
}

#[test]
fn themes_and_save() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "create", "class": "Form", "name": "F", "props": { "caption": "x" } }));
    run(&mut ui, json!({ "cmd": "theme", "name": "dark" }));
    assert_eq!(ui.theme().name, "Aylis Dark");
    let saved = run(&mut ui, json!({ "cmd": "save", "target": "F" }));
    assert_eq!(saved["root"]["caption"], "x");
}

#[test]
fn several_forms_with_the_same_names_and_show_modal() {
    let mut ui = ui();
    let events = capture(&mut ui);
    let form = |name: &str| json!({
        "class": "Form", "name": name,
        "children": [{ "class": "Button", "name": "btnSalvar", "text": "Salvar", "on_click": "salvar" }]
    });
    run(&mut ui, json!({ "cmd": "load", "layout": form("frmPrincipal") }));
    run(&mut ui, json!({ "cmd": "load", "layout": form("frmCliente") }));
    run(&mut ui, json!({ "cmd": "show", "target": "frmPrincipal" }));

    // o caminho escolhe o btnSalvar certo; o evento diz de qual form veio
    run(&mut ui, json!({ "cmd": "click", "target": "frmCliente.btnSalvar" }));
    let e = events.borrow_mut().pop().unwrap();
    assert_eq!((e["form"].as_str(), e["sender"]["path"].as_str()), (Some("frmCliente"), Some("frmCliente.btnSalvar")));

    run(&mut ui, json!({ "cmd": "show_modal", "target": "frmCliente", "handler": "cliente_fechou" }));
    assert!(execute(&mut ui, &json!({ "cmd": "modal_result", "target": "frmCliente", "result": "talvez" })).is_err());
    run(&mut ui, json!({ "cmd": "modal_result", "target": "frmCliente", "result": "ok" }));
    let e = events.borrow_mut().pop().unwrap();
    assert_eq!(e["event"], "on_modal_result");
    assert_eq!(e["handler"], "cliente_fechou");
    assert_eq!(e["args"], "ok");

    // fechar o principal encerra
    assert_eq!(run(&mut ui, json!({ "cmd": "close", "target": "frmPrincipal" })), true);
    assert!(ui.close_requested());
}

#[test]
fn a_form_loaded_inside_another_keeps_its_names() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": {
        "class": "Form", "name": "frmPrincipal",
        "children": [
            { "class": "Button", "name": "btnSalvar" },
            { "class": "Panel", "name": "pnlConteudo" },
            { "class": "DataSource", "name": "dsClientes" }
        ]
    }}));
    // um form com btnSalvar dentro do painel, ligado ao dsClientes do form de fora
    run(&mut ui, json!({ "cmd": "load", "parent": "pnlConteudo", "layout": {
        "class": "Form", "name": "frmClientes",
        "children": [
            { "class": "Button", "name": "btnSalvar" },
            { "class": "DataNavigator", "name": "edtNome", "data_source": "dsClientes" }
        ]
    }}));
    let edt = ui.find_path("frmPrincipal.frmClientes.edtNome").unwrap();
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": edt.to_u64(), "prop": "data_source" })), "frmPrincipal.dsClientes");
    assert_ne!(ui.find_path("frmPrincipal.btnSalvar"), ui.find_path("frmPrincipal.frmClientes.btnSalvar"));
}

#[test]
fn message_dlg_toast_and_dialogs_by_protocol() {
    let mut ui = ui();
    let events = capture(&mut ui);
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "frmPrincipal",
        "children": [{ "class": "OpenDialog", "name": "dlgAbrir", "on_cancel": "abrir_cancelado" }] } }));
    run(&mut ui, json!({ "cmd": "show", "target": "frmPrincipal" }));
    let dlg = run(&mut ui, json!({ "cmd": "message_dlg", "text": "Excluir?", "kind": "confirmation", "buttons": ["yes", "no"], "handler": "respondeu" }));
    run(&mut ui, json!({ "cmd": "modal_result", "target": dlg, "result": "no" }));
    let e = events.borrow_mut().pop().unwrap();
    assert_eq!((e["handler"].as_str(), e["args"].as_str()), (Some("respondeu"), Some("no")));
    run(&mut ui, json!({ "cmd": "toast", "text": "Salvo" }));
    // sem diálogos de sistema (teste), o OpenDialog cancela
    run(&mut ui, json!({ "cmd": "execute", "target": "dlgAbrir" }));
    ui.process_posted();
    let e = events.borrow_mut().pop().unwrap();
    assert_eq!(e["handler"], "abrir_cancelado");
}

#[test]
fn data_batch_values_filter_index_and_lookup() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "frm", "children": [
        { "class": "ClientDataSet", "name": "cds", "key_fields": "id", "children": [
            { "class": "Field", "name": "id", "type": "integer" },
            { "class": "Field", "name": "nome", "type": "string" },
            { "class": "Field", "name": "uf", "type": "string" },
            { "class": "Field", "name": "rotulo", "type": "string", "kind": "calculated", "expression": "nome + '/' + uf" } ] } ] } }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "load", "rows": [
        { "id": 1, "nome": "Ana", "uf": "SP" }, { "id": 2, "nome": "Bia", "uf": "RJ" }, { "id": 3, "nome": "Caio", "uf": "SP" } ] }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_filter", "filter": "uf = 'SP'" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_index", "index": "nome desc" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["count"], 2);
    // trocar o índice mantém o registro atual (Ana); o primeiro agora é o Caio
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "text", "field": "rotulo" })), "Ana/SP");
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "first" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "text", "field": "rotulo" })), "Caio/SP");
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "locate", "field": "id", "value": 1 })), true);
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_values", "values": { "nome": "Ana Paula", "uf": "MG" } }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "post" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["count"], 1, "MG saiu do filtro");
    assert!(execute(&mut ui, &json!({ "cmd": "data", "target": "cds", "op": "set_values", "values": { "id": "x" } })).is_err());
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "cancel" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "lookup", "key_fields": "id", "keys": 2, "result_field": "nome" })), "Bia");
    // Situação do registro e desfazer só ele.
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_filter", "filter": "" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "locate", "field": "id", "value": 1 }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["status"], "modified");
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "revert_record" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["status"], "unmodified");
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "text", "field": "nome" })), "Ana");
    // Filtro com parâmetro.
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_param", "name": "uf", "value": "RJ" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_filter", "filter": "uf = :uf" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["count"], 1);
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "text", "field": "nome" })), "Bia");
}

#[test]
fn inspection_commands() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "frmA", "children": [
        { "class": "Panel", "name": "pnl", "children": [ { "class": "Button", "name": "btnOk", "text": "OK" } ] } ] } }));
    let tree = run(&mut ui, json!({ "cmd": "tree" }));
    assert_eq!(tree[0]["name"], "frmA");
    assert_eq!(tree[0]["children"][0]["children"][0]["path"], "frmA.btnOk");
    let props = run(&mut ui, json!({ "cmd": "props", "target": "frmA.btnOk" }));
    let text = props.as_array().unwrap().iter().find(|p| p["name"] == "text").unwrap();
    assert_eq!((text["kind"].as_str(), text["value"].as_str()), (Some("string"), Some("OK")));
    assert!(props.as_array().unwrap().iter().any(|p| p["name"] == "align" && p["options"].is_array()));
    run(&mut ui, json!({ "cmd": "highlight", "target": "frmA.btnOk" }));
}

#[test]
fn field_events_by_name_go_to_the_backend_in_batches_and_answer_later() {
    let mut ui = ui();
    let events = capture(&mut ui);
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "frm", "children": [
        { "class": "ClientDataSet", "name": "cds", "key_fields": "id", "children": [
            { "class": "Field", "name": "id", "type": "integer" },
            { "class": "Field", "name": "uf", "type": "string", "on_get_text": "uf_nome" },
            { "class": "Field", "name": "cpf", "type": "string", "on_validate": "cpf_valido" } ] } ] } }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "load", "rows": [
        { "id": 1, "uf": "SP", "cpf": "1" }, { "id": 2, "uf": "RJ", "cpf": "2" }, { "id": 3, "uf": "SP", "cpf": "3" } ] }));

    // A tela pede o texto de cada linha; enquanto não chega, mostra o valor.
    let texts = |ui: &Ui| -> Vec<String> {
        let ds = ui.find_path("frm.cds").unwrap();
        let v = aylis_data::view(ui, ds);
        (0..v.visible_count()).map(|i| v.visible_text(i, "uf").unwrap()).collect()
    };
    assert_eq!(texts(&ui), ["SP", "RJ", "SP"]);
    assert!(events.borrow().is_empty(), "nada sai durante a pintura");
    ui.process_posted();
    // Um evento só, com os valores sem repetição.
    let sent: Vec<Value> = events.borrow().iter().filter(|e| e["event"] == "on_get_text").cloned().collect();
    assert_eq!(sent.len(), 1, "{sent:?}");
    assert_eq!(sent[0]["handler"], "uf_nome");
    assert_eq!(sent[0]["args"], json!({ "field": "uf", "values": ["SP", "RJ"] }));
    // O backend responde quando puder; os textos ficam guardados por valor.
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "field_texts", "field": "uf", "texts": [["SP", "São Paulo"], ["RJ", "Rio de Janeiro"]] }));
    assert_eq!(texts(&ui), ["São Paulo", "Rio de Janeiro", "São Paulo"]);
    ui.process_posted();
    assert_eq!(events.borrow().iter().filter(|e| e["event"] == "on_get_text").count(), 1, "com cache, não pede de novo");

    // Validação: o valor entra na hora, o backend responde depois.
    events.borrow_mut().clear();
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "edit" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_value", "field": "cpf", "value": "123" }));
    let validate = events.borrow().iter().find(|e| e["event"] == "on_validate").cloned().expect("on_validate");
    assert_eq!(validate["handler"], "cpf_valido");
    assert_eq!(validate["args"]["value"], "123");
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "field_error", "field": "cpf", "message": "CPF inválido" }));
    let err = execute(&mut ui, &json!({ "cmd": "data", "target": "cds", "op": "post" })).unwrap_err();
    assert!(err.contains("CPF inválido"), "{err}");
    // Outro valor: o erro anterior sai; com a resposta boa, grava.
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "set_value", "field": "cpf", "value": "12345678909" }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "field_error", "field": "cpf", "message": null }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "post" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "text", "field": "cpf" })), "12345678909");
}

#[test]
fn a_backend_exception_is_shown_like_an_app_error() {
    let mut ui = ui();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    ui.on_exception(move |_, e| {
        log.borrow_mut().push(e.clone());
        true
    });
    run(&mut ui, json!({ "cmd": "exception", "message": "Access violation", "path": "frmPedido", "handler": "btn_salvar_click" }));
    let seen = seen.borrow();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].message, "Access violation");
    assert_eq!(seen[0].to_string(), "Access violation (frmPedido.btn_salvar_click)");
}

#[test]
fn the_bridge_compresses_for_a_screen_that_announces_deflate() {
    let mut ui = ui();
    // A tela anuncia na resposta do hello.
    let reply = handle_message(&mut ui, &json!({ "id": 1, "cmd": "hello", "protocol": aylis_remote::PROTOCOL_VERSION })).unwrap();
    assert!(aylis_remote::announces_deflate(&reply.to_string()), "{reply}");
    assert!(!aylis_remote::announces_deflate(r#"{"event":"on_click","handler":"capabilities"}"#));
    // Uma carga de dados, com os nomes de campo repetidos, fica bem menor.
    let rows: Vec<Value> = (0..500).map(|i| json!({ "codigo": i, "nome": format!("Cliente {i}"), "cidade": "São Paulo" })).collect();
    let text = json!({ "cmd": "data", "op": "load", "rows": rows }).to_string();
    let packed = aylis_remote::compress(&text);
    assert!(packed.len() * 5 < text.len(), "{} → {}", text.len(), packed.len());
    assert_eq!(aylis_remote::decompress(&packed).as_deref(), Some(text.as_str()));
    assert_eq!(aylis_remote::decompress(b"nada disso"), None);
}

#[test]
fn a_different_protocol_version_is_refused_with_what_to_update() {
    let mut ui = ui();
    let newer = handle_message(&mut ui, &json!({ "id": 1, "cmd": "hello", "protocol": aylis_remote::PROTOCOL_VERSION + 1, "update_url": "https://loja/app" })).unwrap();
    assert!(newer["error"].as_str().unwrap().contains("atualize o aplicativo"), "{newer}");
    let older = handle_message(&mut ui, &json!({ "id": 2, "cmd": "hello", "protocol": 0 })).unwrap();
    assert!(older["error"].as_str().unwrap().contains("atualize o backend"), "{older}");
    let mut handshake = aylis_remote::Handshake::default();
    assert!(handshake.check(&json!({ "cmd": "load" })).is_some(), "antes do hello, recusado");
    assert!(handshake.check(&json!({ "cmd": "hello", "protocol": 1 })).is_none());
    assert!(handshake.check(&json!({ "cmd": "load" })).is_none());
}

/// Um evento que pergunta vai com `ask` e espera o `answer`; cada evento leva `seq` (com o
/// cliente que anunciou `done`); o form liberado é avisado (`__freed`).
#[test]
fn questions_seq_done_and_freed_forms() {
    let mut ui = ui();
    let events = capture(&mut ui);
    run(&mut ui, json!({ "cmd": "hello", "protocol": PROTOCOL_VERSION, "features": ["done"] }));
    let form = run(&mut ui, json!({ "cmd": "create", "class": "Form", "name": "Frm", "props": { "on_close_query": "pode_fechar" } }));
    let id = form["id"].as_u64().unwrap();
    ui.show(aylis_core::ComponentId::from_u64(id)).unwrap();
    assert_eq!(run(&mut ui, json!({ "cmd": "close", "target": "Frm" })), json!(false), "espera a resposta");
    let (ask, seq) = {
        let e = events.borrow();
        let last = e.last().unwrap();
        assert_eq!(last["event"], "on_close_query");
        (last["ask"].as_u64().unwrap(), last["seq"].as_u64().unwrap())
    };
    assert!(ui.backend_busy());
    run(&mut ui, json!({ "cmd": "answer", "ask": ask, "values": { "can_close": true } }));
    run(&mut ui, json!({ "cmd": "done", "seq": seq }));
    assert!(!ui.backend_busy());
    assert!(!ui.is_open(aylis_core::ComponentId::from_u64(id)), "fechou com a resposta");
    run(&mut ui, json!({ "cmd": "free", "target": "Frm" }));
    let e = events.borrow();
    assert_eq!(e.last().unwrap()["event"], "__freed");
    assert_eq!(e.last().unwrap()["forms"], json!([id]));
}

/// A carga colunar (item 13): os mesmos registros que o `load`, sem repetir os nomes.
#[test]
fn load_columns_fills_the_dataset() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "root": { "class": "DataModule", "name": "dm", "children": [
        { "class": "ClientDataSet", "name": "cds", "children": [
            { "class": "Field", "name": "id", "type": "integer" },
            { "class": "Field", "name": "nome", "type": "string" },
            { "class": "Field", "name": "uf", "type": "string" }
        ] }
    ] } } }));
    run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "load_columns", "fields": ["nome", "id"], "columns": [["Ana", "Bia"], [1, 2]] }));
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["count"], 2);
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "record" })), json!({ "id": 1, "nome": "Ana", "uf": null }));
    assert!(execute(&mut ui, &json!({ "cmd": "data", "target": "cds", "op": "load_columns", "fields": ["id"], "columns": [[1], [2]] })).is_err(), "colunas a mais");
}

/// Um nome solto que está em mais de um form (frames com o mesmo `lblTotal`) é recusado com os
/// caminhos, em vez de mexer no primeiro (relatório do InsightDB, 11/11).
#[test]
fn an_ambiguous_name_lists_the_paths() {
    let mut ui = ui();
    for form in ["fraVendas", "fraEstoque"] {
        let root = ui.add_root(aylis_controls::Form::new(form, 200.0, 100.0));
        ui.set_name(root, form).unwrap();
        let label = ui.add(aylis_controls::Label::new("0"), root);
        ui.set_name(label, "lblTotal").unwrap();
    }
    let error = execute(&mut ui, &json!({ "cmd": "get", "target": "lblTotal", "prop": "text" })).unwrap_err();
    assert!(error.contains("fraVendas.lblTotal") && error.contains("fraEstoque.lblTotal"), "{error}");
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "fraEstoque.lblTotal", "prop": "text" })), json!("0"));
}
/// O clique com o mouse de verdade e as teclas seguradas: Ctrl+clique e Shift+clique na
/// lista de seleção múltipla, como a mão faria (o `click` simples continua o do componente).
#[test]
fn click_with_modifiers_uses_the_real_mouse() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "Frm", "width": 400, "height": 300, "children": [
        { "class": "ListBox", "name": "lstNomes", "items": "Ana;Bia;Caio;Duda;Eva", "multi_select": true, "item_height": 28, "align": "client" }
    ] } }));
    let row = |i: u32| json!(14 + 28 * i);
    run(&mut ui, json!({ "cmd": "click", "target": "lstNomes", "x": 40, "y": row(0) }));
    run(&mut ui, json!({ "cmd": "click", "target": "lstNomes", "x": 40, "y": row(2), "modifiers": "Ctrl" }));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "lstNomes", "prop": "selected" })), "0;2");
    run(&mut ui, json!({ "cmd": "click", "target": "lstNomes", "x": 40, "y": row(4), "modifiers": ["shift"] }));
    let selected = run(&mut ui, json!({ "cmd": "get", "target": "lstNomes", "prop": "selected" }));
    assert!(selected.as_str().unwrap().ends_with("2;3;4"), "Shift estende do último até o clicado: {selected}");
    let e = execute(&mut ui, &json!({ "cmd": "click", "target": "lstNomes", "modifiers": "Ctrl+Hyper" })).unwrap_err();
    assert!(e.contains("hyper"), "{e}");
    let e = execute(&mut ui, &json!({ "cmd": "click", "target": "lstNomes", "button": "lateral" })).unwrap_err();
    assert!(e.contains("lateral"), "{e}");
}

/// `drag` arrasta de um componente a outro: o arrastar e soltar do Aylis recebe o que o
/// origem declarou, e as teclas seguradas vão junto.
#[test]
fn drag_moves_from_one_component_to_another() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "Frm", "width": 400, "height": 300, "children": [
        { "class": "Panel", "name": "pnlOrigem", "align": "left", "width": 120, "draggable": true, "drag_data": "cliente:7" },
        { "class": "Panel", "name": "pnlDestino", "align": "right", "width": 120, "accepts_drops": true }
    ] } }));
    let dropped = Rc::new(RefCell::new(Vec::new()));
    let d = dropped.clone();
    let destino = ui.find("pnlDestino").unwrap();
    ui.on(destino, "on_drag_drop", move |_, _, args| d.borrow_mut().push(format!("{args:?}")));
    run(&mut ui, json!({ "cmd": "drag", "from": "pnlOrigem", "to": "pnlDestino", "modifiers": "Ctrl" }));
    assert_eq!(dropped.borrow().len(), 1, "soltou no destino");
    assert!(dropped.borrow()[0].contains("cliente:7"), "{:?}", dropped.borrow());
    // Soltar no próprio origem não chega ao destino.
    run(&mut ui, json!({ "cmd": "drag", "from": "pnlOrigem", "to": "pnlOrigem", "to_x": 100, "steps": 3 }));
    assert_eq!(dropped.borrow().len(), 1);
    let e = execute(&mut ui, &json!({ "cmd": "drag", "from": "pnlOrigem" })).unwrap_err();
    assert!(e.contains("to"), "{e}");
}

/// A tecla com `modifiers` soma às da própria tecla: Ctrl+A seleciona tudo na lista.
#[test]
fn key_takes_modifiers_too() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "Frm", "width": 400, "height": 300, "children": [
        { "class": "ListBox", "name": "lstNomes", "items": "Ana;Bia;Caio", "multi_select": true, "align": "client" }
    ] } }));
    run(&mut ui, json!({ "cmd": "focus", "target": "lstNomes" }));
    run(&mut ui, json!({ "cmd": "key", "target": "Frm", "key": "A", "modifiers": ["ctrl"] }));
    assert_eq!(run(&mut ui, json!({ "cmd": "get", "target": "lstNomes", "prop": "selected" })), "0;1;2");
}
