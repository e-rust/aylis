//! Forms com instância (item 2): cada form aberto tem a sua, os handlers falam com a do form
//! de quem disparou, e um evento do mesmo form dentro de um handler dele espera ele terminar.

mod common;

use common::boxy;
use aylis_core::instances::form_handler;
use aylis_core::*;

/// A "struct gerada" de um form de teste: o botão resolvido e o estado.
struct FrmPedido {
    btn: ComponentId,
    clicks: u32,
    log: Vec<String>,
}

impl FrmPedido {
    fn bind(ui: &mut Ui, root: ComponentId) -> std::result::Result<Box<dyn std::any::Any>, String> {
        let btn = ui.find_in(root, "btnSalvar").ok_or("frmPedido: falta o componente btnSalvar (Button)")?;
        Ok(Box::new(FrmPedido { btn, clicks: 0, log: Vec::new() }))
    }

    fn salvar_click(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        self.clicks += 1;
        self.log.push("salvar começa".into());
        // Um evento do mesmo form disparado aqui dentro: espera este terminar.
        ui.fire(sender, "on_dbl_click", &EventArgs::None);
        self.log.push("salvar termina".into());
    }

    fn duplo(&mut self, _ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.log.push("duplo".into());
    }
}

fn open(ui: &mut Ui) -> ComponentId {
    let form = ui.add_root(boxy(300.0, 200.0));
    ui.set_name(form, "frmPedido").unwrap();
    let btn = ui.add(boxy(80.0, 30.0), form);
    ui.set_name(btn, "btnSalvar").unwrap();
    ui.set_prop(btn, "on_click", "salvar_click").unwrap();
    ui.set_prop(btn, "on_dbl_click", "duplo").unwrap();
    ui.bind_form(form).unwrap();
    form
}

fn setup() -> Ui {
    let mut ui = Ui::new();
    ui.register_form_class("frmPedido", FrmPedido::bind);
    ui.register_handler("frmPedido.salvar_click", form_handler(FrmPedido::salvar_click));
    ui.register_handler("frmPedido.duplo", form_handler(FrmPedido::duplo));
    ui
}

#[test]
fn each_open_form_has_its_own_instance() {
    let mut ui = setup();
    let a = open(&mut ui);
    let b = open(&mut ui);
    let btn_a = ui.form_instance::<FrmPedido>(a).unwrap().btn;
    let btn_b = ui.form_instance::<FrmPedido>(b).unwrap().btn;
    assert_ne!(btn_a, btn_b, "cada instância com os componentes do seu form");
    ui.fire(btn_b, "on_click", &EventArgs::None);
    ui.fire(btn_b, "on_click", &EventArgs::None);
    ui.fire(btn_a, "on_click", &EventArgs::None);
    assert_eq!(ui.form_instance::<FrmPedido>(a).unwrap().clicks, 1);
    assert_eq!(ui.form_instance::<FrmPedido>(b).unwrap().clicks, 2);
}

#[test]
fn an_event_of_the_same_form_inside_a_handler_waits_for_it() {
    let mut ui = setup();
    let form = open(&mut ui);
    let btn = ui.form_instance::<FrmPedido>(form).unwrap().btn;
    ui.fire(btn, "on_click", &EventArgs::None);
    let log = &ui.form_instance::<FrmPedido>(form).unwrap().log;
    assert_eq!(log, &["salvar começa", "salvar termina", "duplo"]);
}

#[test]
fn a_form_that_does_not_match_the_code_is_refused() {
    let mut ui = setup();
    let form = ui.add_root(boxy(300.0, 200.0));
    ui.set_name(form, "frmPedido").unwrap();
    let error = ui.bind_form(form).unwrap_err();
    assert!(error.contains("btnSalvar"), "{error}");
    assert!(ui.form_instance::<FrmPedido>(form).is_none());
}

#[test]
fn freeing_the_form_drops_the_instance() {
    let mut ui = setup();
    let form = open(&mut ui);
    ui.free(form);
    assert!(ui.form_instance::<FrmPedido>(form).is_none());
    // Outro form no mesmo lugar (o slot reaproveitado) não herda a instância.
    let other = ui.add_root(boxy(10.0, 10.0));
    assert!(ui.form_instance::<FrmPedido>(other).is_none());
}

/// A cópia dos componentes (o `FrmX::components(ui)` gerado): vale também enquanto a
/// instância está emprestada a um handler do form, quando o `form_instance` dá `None`.
#[derive(Clone, Copy)]
struct FrmPedidoComponents {
    btn: ComponentId,
}

#[test]
fn the_components_copy_is_available_while_a_handler_of_the_form_runs() {
    let mut ui = Ui::new();
    ui.register_form_class("frmPedido", |ui: &mut Ui, root: ComponentId| {
        let instance = FrmPedido::bind(ui, root)?;
        let btn = instance.downcast_ref::<FrmPedido>().map(|f| f.btn).ok_or("tipo")?;
        ui.set_form_components(root, Box::new(FrmPedidoComponents { btn }));
        Ok(instance)
    });
    thread_local!(static SEEN: std::cell::Cell<(bool, bool)> = const { std::cell::Cell::new((false, false)) });
    fn probe(_: &mut FrmPedido, ui: &mut Ui, sender: ComponentId, _: &EventArgs) {
        let form = ui.form_of(sender);
        let copy = ui.form_components::<FrmPedidoComponents>(form).map(|c| c.btn);
        SEEN.set((ui.form_instance::<FrmPedido>(form).is_none(), copy == Some(sender)));
    }
    ui.register_handler("frmPedido.salvar_click", form_handler(probe));
    let form = open(&mut ui);
    let btn = ui.form_components::<FrmPedidoComponents>(form).unwrap().btn;
    ui.fire(btn, "on_click", &EventArgs::None);
    assert_eq!(SEEN.get(), (true, true), "a instância está emprestada, mas a cópia dos componentes vale");
    ui.free(form);
    assert!(ui.form_components::<FrmPedidoComponents>(form).is_none(), "o form liberado esquece a cópia");
}

/// Um pânico num handler mostra a caixa de erro e devolve a instância ao form: os eventos
/// seguintes do mesmo form rodam (na conferência de 08/11, o form ficou "ocupado" e todo
/// evento dele ficou adiado para sempre).
#[test]
fn a_panic_in_a_handler_gives_the_instance_back_to_the_form() {
    let mut ui = setup();
    fn quebra(this: &mut FrmPedido, _: &mut Ui, _: ComponentId, _: &EventArgs) {
        this.clicks += 1;
        panic!("erro de propósito");
    }
    ui.register_handler("frmPedido.quebra", form_handler(quebra));
    let form = open(&mut ui);
    let btn = ui.form_instance::<FrmPedido>(form).unwrap().btn;
    ui.set_prop(btn, "on_click", "quebra").unwrap();
    ui.fire(btn, "on_click", &EventArgs::None);
    assert!(ui.form_instance::<FrmPedido>(form).is_some(), "a instância voltou ao form");
    ui.set_prop(btn, "on_click", "salvar_click").unwrap();
    ui.fire(btn, "on_click", &EventArgs::None);
    let instance = ui.form_instance::<FrmPedido>(form).unwrap();
    assert_eq!(instance.clicks, 2, "o clique depois do pânico rodou");
    assert_eq!(instance.log, ["salvar começa", "salvar termina", "duplo"]);
}

/// O frame repetido (o `FraX::create_in` gerado): cada instância ganha um nome livre no form
/// do pai (dois irmãos não podem ter o mesmo) e fica com os seus componentes e a sua instância.
#[test]
fn repeated_frames_get_free_names_and_their_own_instances() {
    let mut ui = setup();
    let list = ui.add_root(boxy(400.0, 400.0));
    ui.set_name(list, "frmLista").unwrap();
    assert_eq!(ui.free_name_in(list, "frmPedido"), "frmPedido");
    let mut rows = Vec::new();
    for _ in 0..3 {
        let row = open(&mut ui);
        let name = ui.free_name_in(list, "frmPedido");
        ui.set_name(row, &name).unwrap();
        ui.set_parent(row, Some(list)).unwrap();
        rows.push(row);
    }
    let names: Vec<_> = rows.iter().map(|&r| ui.name(r).unwrap().to_string()).collect();
    assert_eq!(names, ["frmPedido", "frmPedido2", "frmPedido3"]);
    let btn = ui.form_instance::<FrmPedido>(rows[1]).unwrap().btn;
    ui.fire(btn, "on_click", &EventArgs::None);
    assert_eq!(ui.form_instance::<FrmPedido>(rows[1]).unwrap().clicks, 1);
    assert_eq!(ui.form_instance::<FrmPedido>(rows[0]).unwrap().clicks, 0);
}
