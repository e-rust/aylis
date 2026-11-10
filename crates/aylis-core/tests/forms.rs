//! Vários forms: nomes por form, forms embutidos, handlers por form, Show/ShowModal/Close.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::*;

#[derive(Default)]
struct Block {
    base: ControlBase,
}

impl Component for Block {
    fn class_name(&self) -> &'static str {
        "Block"
    }
    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }
    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_click", EventArgs::None);
    }
}

/// Um form com um "btnSalvar".
fn form(ui: &mut Ui, name: &str) -> (ComponentId, ComponentId) {
    let f = ui.add_root(Block::default());
    ui.set_name(f, name).unwrap();
    ui.control_mut(f).unwrap().set_size(300.0, 200.0);
    let b = ui.add(Block::default(), f);
    ui.set_name(b, "btnSalvar").unwrap();
    (f, b)
}

#[test]
fn the_same_name_in_two_forms() {
    let mut ui = Ui::new();
    let (customers, b1) = form(&mut ui, "frmClientes");
    let (products, b2) = form(&mut ui, "frmProdutos");
    assert_eq!(ui.find_in(customers, "btnSalvar"), Some(b1));
    assert_eq!(ui.find_in(products, "btnSalvar"), Some(b2));
    assert_eq!(ui.find_path("frmProdutos.btnSalvar"), Some(b2));
    assert_eq!(ui.path_of(b2), "frmProdutos.btnSalvar");
    // dentro do mesmo form continua único
    let other = ui.add(Block::default(), customers);
    assert!(matches!(ui.set_name(other, "BTNSALVAR"), Err(Error::DuplicateName(_))));
    // entre os forms raiz o nome pode repetir: o mesmo form aberto duas vezes (cada janela
    // com a sua instância); o caminho acha o primeiro
    let third = ui.add_root(Block::default());
    ui.set_name(third, "frmClientes").unwrap();
    assert_eq!(ui.find_path("frmClientes"), Some(customers));
}

#[test]
fn an_embedded_form_keeps_its_own_names() {
    let mut ui = Ui::new();
    let (main_form, b_main) = form(&mut ui, "frmPrincipal");
    let panel = ui.add(Block::default(), main_form);
    ui.set_name(panel, "pnlConteudo").unwrap();
    let (customers, b_customers) = form(&mut ui, "frmClientes");

    // Form2.Parent := Panel1, com um btnSalvar nos dois
    ui.set_parent(customers, Some(panel)).unwrap();
    assert_eq!(ui.form_of(b_customers), customers);
    assert_eq!(ui.find_in(main_form, "btnSalvar"), Some(b_main), "não entra no form embutido");
    assert_eq!(ui.find_in(main_form, "frmClientes"), Some(customers), "mas acha o form embutido pelo nome");
    assert_eq!(ui.find_path("frmPrincipal.frmClientes.btnSalvar"), Some(b_customers));
    assert_eq!(ui.path_of(b_customers), "frmPrincipal.frmClientes.btnSalvar");

    // um componente comum que muda de form leva o nome para o escopo novo
    let (other, b_other) = form(&mut ui, "frmOutro");
    assert!(matches!(ui.set_parent(b_other, Some(main_form)), Err(Error::DuplicateName(_))));
    let _ = other;
}

#[test]
fn handlers_with_the_same_name_in_different_forms() {
    let mut ui = Ui::new();
    let (_, b1) = form(&mut ui, "frmClientes");
    let (_, b2) = form(&mut ui, "frmProdutos");
    let log = Rc::new(RefCell::new(Vec::new()));
    for (key, text) in [("frmClientes.salvar", "clientes"), ("salvar", "geral")] {
        let log = log.clone();
        ui.register_handler(key, move |_, _, _| log.borrow_mut().push(text));
    }
    ui.set_prop(b1, "on_click", PropValue::Event(Some("salvar".into()))).unwrap();
    ui.set_prop(b2, "on_click", PropValue::Event(Some("salvar".into()))).unwrap();
    ui.click(b1);
    ui.click(b2);
    assert_eq!(*log.borrow(), ["clientes", "geral"]);
}

#[test]
fn show_modal_blocks_the_others_and_returns_the_result() {
    let mut ui = Ui::new();
    let (main_form, b_main) = form(&mut ui, "frmPrincipal");
    let (dialog, _) = form(&mut ui, "frmDialogo");
    ui.show(main_form).unwrap();
    assert_eq!(ui.main_form(), Some(main_form));

    let result = Rc::new(RefCell::new(None));
    let r = result.clone();
    ui.show_modal(dialog, move |_, res| *r.borrow_mut() = Some(res)).unwrap();
    assert_eq!(ui.open_forms(), vec![main_form, dialog]);
    assert!(ui.input_blocked(main_form));
    assert!(!ui.input_blocked(dialog));
    assert_eq!(ui.take_activation(), Some(dialog));

    // o form de baixo não recebe clique enquanto o modal está aberto
    let clicks = Rc::new(RefCell::new(0));
    let c = clicks.clone();
    ui.on_click(b_main, move |_, _| *c.borrow_mut() += 1);
    ui.layout();
    let p = PointerEvent { position: PointF::new(5.0, 5.0), ..Default::default() };
    ui.control_mut(b_main).unwrap().set_rect(0.0, 0.0, 50.0, 50.0);
    ui.pointer_down(main_form, p);
    ui.pointer_up(main_form, p);
    assert_eq!(*clicks.borrow(), 0);

    ui.set_modal_result(dialog, ModalResult::Ok);
    assert_eq!(*result.borrow(), Some(ModalResult::Ok));
    assert_eq!(ui.open_forms(), vec![main_form]);
    assert!(!ui.input_blocked(main_form));
    assert_eq!(ui.take_activation(), Some(main_form), "volta para o dono");
    ui.pointer_down(main_form, p);
    ui.pointer_up(main_form, p);
    assert_eq!(*clicks.borrow(), 1);
}

#[test]
fn on_close_query_can_keep_the_form_open() {
    let mut ui = Ui::new();
    let (main_form, _) = form(&mut ui, "frmPrincipal");
    let (dialog, _) = form(&mut ui, "frmDialogo");
    ui.show(main_form).unwrap();
    let allow = Rc::new(RefCell::new(false));
    let a = allow.clone();
    ui.on(dialog, "on_close_query", move |ui, _, _| {
        if !*a.borrow() {
            ui.cancel_close();
        }
    });
    let closed = Rc::new(RefCell::new(Vec::new()));
    let c = closed.clone();
    ui.on(dialog, "on_close", move |_, _, args| c.borrow_mut().push(args.clone()));
    ui.show_modal(dialog, |_, _| {}).unwrap();

    ui.set_modal_result(dialog, ModalResult::Cancel);
    assert!(ui.is_open(dialog), "CanClose := False");
    assert_eq!(ui.modal_result(dialog), ModalResult::None, "o resultado volta a None");

    *allow.borrow_mut() = true;
    assert!(ui.close(dialog));
    assert!(!ui.is_open(dialog));
    assert_eq!(*closed.borrow(), [EventArgs::Value(PropValue::Enum("none".into()))]);
    assert!(!ui.close_requested());

    // fechar o principal encerra o aplicativo
    ui.close(main_form);
    assert!(ui.close_requested());
}

#[test]
fn freeing_an_open_form_removes_it() {
    let mut ui = Ui::new();
    let (main_form, _) = form(&mut ui, "frmPrincipal");
    let (other, _) = form(&mut ui, "frmOutro");
    ui.show(main_form).unwrap();
    ui.show(other).unwrap();
    ui.free(other);
    assert_eq!(ui.open_forms(), vec![main_form]);
}

fn monitor(device: &str, x: i32, width: u32, primary: bool) -> aylis_core::monitors::Monitor {
    aylis_core::monitors::Monitor {
        name: format!("Monitor {device}"),
        device: device.into(),
        primary,
        x,
        y: 0,
        width,
        height: 2160,
        work_x: x,
        work_y: 0,
        work_width: width,
        work_height: 2100,
        scale: 1.5,
        refresh_hz: if primary { 160.0 } else { 60.0 },
        modes: Vec::new(),
    }
}

#[test]
fn a_remembered_placement_comes_back_only_while_its_monitor_is_on_and_visible() {
    use aylis_core::monitors::{SavedPlacement, restore_placement};
    let monitors = [monitor(r"\\.\DISPLAY1", 0, 3840, true), monitor(r"\\.\DISPLAY2", -3840, 3840, false)];
    let at = |x: i32, y: i32| SavedPlacement { monitor: r"\\.\DISPLAY2".into(), placement: WindowPlacement { x, y, width: 1200, height: 800, maximized: false } };
    assert_eq!(restore_placement(&at(-3000, 100), &monitors).map(|p| p.x), Some(-3000));
    // O monitor desligado: abre como se não houvesse posição guardada.
    assert_eq!(restore_placement(&at(-3000, 100), &monitors[..1]), None);
    // A barra de título fora de todos os monitores.
    assert_eq!(restore_placement(&at(-3000, 5000), &monitors), None);
}

#[test]
fn the_monitor_list_tracks_changes_and_goes_out_as_json() {
    let mut ui = Ui::new();
    assert!(ui.monitors().is_empty());
    let list = vec![monitor(r"\\.\DISPLAY1", 0, 3840, true)];
    ui.set_monitors(list.clone());
    let version = ui.monitors_version();
    ui.set_monitors(list);
    assert_eq!(ui.monitors_version(), version, "a mesma lista não conta como mudança");
    let json = ui.monitors()[0].to_json(0);
    assert!(json.contains(r#""device":"\\\\.\\DISPLAY1""#) && json.contains("\"refresh_hz\":160") && json.contains("\"primary\":true"), "{json}");
}
