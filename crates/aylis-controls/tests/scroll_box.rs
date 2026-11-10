//! O `ScrollBox` diz quando o usuário rola (`on_scroll`), quais itens de uma lista estão à
//! vista (`visible_components`) e posiciona num componente para ele ficar inteiro à vista
//! (`scroll_to`), pelo nome (JSON, protocolo, outras linguagens) e em Rust.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

/// Uma caixa de 200 × 100 com uma lista (`Layout` no topo) de 10 linhas de 30: a lista
/// fica dentro da caixa, como num form de verdade (a fala de uma transcrição, um cartão).
fn setup() -> (Ui, ComponentId, ComponentId, ComponentId, Vec<ComponentId>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("teste", 400.0, 300.0));
    ui.set_name(f, "frmTeste").unwrap();
    let sb = ui.add(ScrollBox::default(), f);
    ui.set_name(sb, "scrLista").unwrap();
    ui.control_mut(sb).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    let list = ui.add(Layout::default(), sb);
    ui.control_mut(list).unwrap().set_align(Align::Top).set_height(300.0);
    let rows: Vec<_> = (0..10)
        .map(|i| {
            let r = ui.add(Label::new(format!("linha {i}")), list);
            ui.set_name(r, &format!("lbl{i}")).unwrap();
            ui.control_mut(r).unwrap().set_align(Align::Top).set_height(30.0);
            r
        })
        .collect();
    ui.layout();
    (ui, f, sb, list, rows)
}

fn wheel(dy: f32) -> WheelEvent {
    WheelEvent { position: PointF::new(50.0, 50.0), delta: PointF::new(0.0, dy), ..Default::default() }
}

fn position(ui: &Ui, sb: ComponentId) -> f64 {
    ui.get_prop(sb, "scroll_position").unwrap().to_f64().unwrap()
}

#[test]
fn on_scroll_tells_when_the_user_scrolls_and_not_when_the_code_does() {
    let (mut ui, f, sb, list, rows) = setup();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let s = seen.clone();
    ui.on(sb, "on_scroll", move |_, _, args| {
        if let EventArgs::Value(v) = args {
            s.borrow_mut().push(v.to_f64().unwrap());
        }
    });
    // A roda (o usuário): avisa, com a posição nova.
    ui.wheel(f, wheel(-40.0));
    assert_eq!(*seen.borrow(), [40.0]);
    assert_eq!(position(&ui, sb), 40.0);
    // No fim não rola mais: não avisa de novo.
    ui.wheel(f, wheel(-4000.0));
    ui.wheel(f, wheel(-40.0));
    assert_eq!(*seen.borrow(), [40.0, 200.0]);
    // O código posiciona (a propriedade, o `scroll_to`): não é o usuário, não avisa.
    ui.set_prop(sb, "scroll_position", PropValue::Float(10.0)).unwrap();
    ui.layout();
    assert_eq!(position(&ui, sb), 10.0);
    ui.invoke(sb, "scroll_to", &[PropValue::Component(Some(rows[8]))]).unwrap();
    assert_eq!(seen.borrow().len(), 2);
    let _ = list;
}

#[test]
fn scroll_to_keeps_the_component_whole_moving_the_least() {
    let (mut ui, _f, sb, _list, rows) = setup();
    // Abaixo da área: rola só até ele aparecer inteiro embaixo (240 + 30 − 100).
    ui.invoke(sb, "scroll_to", &[PropValue::Component(Some(rows[8]))]).unwrap();
    ui.layout();
    assert_eq!(position(&ui, sb), 170.0);
    // Já inteiro à vista: não mexe.
    ui.invoke(sb, "scroll_to", &[PropValue::Component(Some(rows[7]))]).unwrap();
    ui.layout();
    assert_eq!(position(&ui, sb), 170.0);
    // Cortado em cima: rola até ele encostar no alto.
    ui.set_prop(sb, "scroll_position", PropValue::Float(100.0)).unwrap();
    ui.layout();
    ui.invoke(sb, "scroll_to", &[PropValue::String("frmTeste.lbl2".into())]).unwrap();
    ui.layout();
    assert_eq!(position(&ui, sb), 60.0);
    // Pelo id numérico (o que os clientes das outras linguagens mandam): o `Ui::invoke` entrega
    // o componente.
    ui.invoke(sb, "scroll_to", &[PropValue::Int(rows[9].to_u64() as i64)]).unwrap();
    ui.layout();
    assert_eq!(position(&ui, sb), 200.0);
    // Em Rust, o mesmo (com o layout pendente: ele roda antes).
    ScrollBox::scroll_into_view(&mut ui, sb, rows[0]);
    ui.layout();
    assert_eq!(position(&ui, sb), 0.0);
    assert!(ui.invoke(sb, "scroll_to", &[]).is_err(), "sem o componente é erro");
}

#[test]
fn visible_components_lists_what_shows_whole_or_in_part() {
    let (mut ui, f, sb, list, rows) = setup();
    // Rolado 45: a linha 1 (30–60) está cortada em cima, a 4 (120–150) cortada embaixo.
    ui.wheel(f, wheel(-45.0));
    ui.layout();
    assert_eq!(ScrollBox::visible_children(&mut ui, sb, list, true), rows[2..4].to_vec());
    assert_eq!(ScrollBox::visible_children(&mut ui, sb, list, false), rows[1..5].to_vec());
    // Pelo nome: os caminhos, separados por `;`.
    let whole = ui.invoke(sb, "visible_components", &[PropValue::Component(Some(list)), PropValue::Bool(true)]).unwrap();
    assert_eq!(whole, PropValue::String("frmTeste.lbl2;frmTeste.lbl3".into()));
    // Uma linha escondida não conta.
    ui.control_mut(rows[2]).unwrap().set_visible(false);
    ui.layout();
    let shown = ScrollBox::visible_children(&mut ui, sb, list, true);
    assert!(!shown.contains(&rows[2]));
    // Sem a lista, os filhos da própria caixa (a lista inteira aparece em parte).
    let own = ui.invoke(sb, "visible_components", &[PropValue::Null, PropValue::Bool(false)]).unwrap();
    assert_eq!(own, PropValue::String(ui.path_of(list)));
}

/// `captures_wheel`: a roda em cima do controle fica com ele (o `on_wheel` dele dá o zoom de
/// uma prévia) e a caixa de fora não rola junto; sem a propriedade, ela rola.
#[test]
fn captures_wheel_keeps_the_wheel_from_the_containers_outside() {
    let (mut ui, f, sb, _list, rows) = setup();
    let wheels = Rc::new(RefCell::new(0));
    let w = wheels.clone();
    // O rótulo recebe o mouse (por padrão, não).
    ui.set_prop(rows[1], "hit_test", true).unwrap();
    ui.on(rows[1], "on_wheel", move |_, _, _| *w.borrow_mut() += 1);
    ui.wheel(f, wheel(-40.0));
    assert_eq!(position(&ui, sb), 40.0, "sem a propriedade, a caixa rola");
    assert_eq!(*wheels.borrow(), 1);
    ui.set_prop(sb, "scroll_position", PropValue::Float(0.0)).unwrap();
    ui.set_prop(rows[1], "captures_wheel", true).unwrap();
    ui.layout();
    ui.wheel(f, wheel(-40.0));
    assert_eq!(position(&ui, sb), 0.0, "a roda ficou com o controle");
    assert_eq!(*wheels.borrow(), 2, "o on_wheel dele recebeu");
}
