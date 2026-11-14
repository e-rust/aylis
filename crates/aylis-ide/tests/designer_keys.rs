//! O designer pelo teclado (pedido do autor em 08/11): Shift+seta muda o
//! tamanho dos escolhidos, as setas os movem.

use aylis_core::PointF;
use aylis_ide::document::Document;

#[test]
fn shift_arrow_resizes_and_arrows_move_every_selected() {
    let mut doc = Document::new();
    let a = doc.add("Button", Some(PointF::new(20.0, 20.0))).unwrap();
    let b = doc.add("Button", Some(PointF::new(20.0, 120.0))).unwrap();
    doc.select(Some(a));
    doc.toggle(b);
    let size = |doc: &Document, id| doc.ui.control(id).map(|c| (c.width(), c.height())).unwrap();
    let place = |doc: &Document, id| doc.ui.control(id).map(|c| (c.left(), c.top())).unwrap();
    let (wa, ha) = size(&doc, a);
    let (wb, _) = size(&doc, b);
    doc.resize_by(a, 1.0, 0.0);
    doc.resize_by(a, 0.0, -1.0);
    assert_eq!(size(&doc, a), (wa + 1.0, ha - 1.0));
    assert_eq!(size(&doc, b).0, wb + 1.0, "vale para todos os escolhidos");
    let (xa, ya) = place(&doc, a);
    doc.nudge(a, 1.0, 0.0);
    assert_eq!(place(&doc, a), (xa + 1.0, ya));
    assert!(doc.modified, "entra no desfazer como mudança");
}

/// O painel ESTRUTURA pelas setas: ↓ desce, ← recolhe o escolhido (o filho some) e → abre de
/// novo; ← num sem filhos vai ao pai.
#[test]
fn the_structure_panel_follows_the_arrows() {
    use aylis_core::Key;
    use aylis_ide::{app, panels, state::shared};
    let mut doc = Document::new();
    let panel = doc.add("Panel", Some(PointF::new(20.0, 20.0))).unwrap();
    doc.select(Some(panel));
    let button = doc.add("Button", Some(PointF::new(10.0, 10.0))).unwrap();
    doc.ui.set_parent(button, Some(panel)).unwrap();
    let root = doc.root;
    let state = shared(doc);
    let mut ide = app::build(state.clone());
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let rows = |ide: &app::Ide| ide.ui.children(ide.ids.structure).len();
    let before = rows(&ide);
    assert!(before >= 3, "o form, o painel e o botão");
    state.borrow_mut().doc.select(Some(panel));
    app::refresh(&mut ide.ui, &state, &ide.ids);
    assert!(panels::structure_arrow(&mut ide.ui, &state, &ide.ids, Key::Left));
    assert_eq!(rows(&ide), before - 1, "recolhido: o botão some");
    panels::structure_arrow(&mut ide.ui, &state, &ide.ids, Key::Right);
    assert_eq!(rows(&ide), before, "aberto de novo");
    panels::structure_arrow(&mut ide.ui, &state, &ide.ids, Key::Down);
    assert_eq!(state.borrow().doc.selection, Some(button));
    panels::structure_arrow(&mut ide.ui, &state, &ide.ids, Key::Left);
    assert_eq!(state.borrow().doc.selection, Some(panel), "← no sem filhos vai ao pai");
    panels::structure_arrow(&mut ide.ui, &state, &ide.ids, Key::Up);
    assert_eq!(state.borrow().doc.selection, Some(root));
}

/// Um campo do Object Inspector ainda em edição (com o foco) grava antes de salvar ou executar,
/// como ao sair dele (o Ctrl+Shift+F9 com o cursor no campo executava sem a mudança).
#[test]
fn an_inspector_field_being_edited_is_committed() {
    use aylis_ide::{app, inspector, state::shared};
    let mut doc = Document::new();
    let button = doc.add("Button", Some(PointF::new(20.0, 20.0))).unwrap();
    doc.select(Some(button));
    let state = shared(doc);
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let current = state.borrow().doc.ui.get_prop(button, "text").unwrap().to_str().unwrap().to_string();
    // O campo do `text`: o Edit do inspector com o valor de agora.
    let mut stack = vec![ide.ids.inspector];
    let mut field = None;
    while let Some(c) = stack.pop() {
        if ide.ui.get::<aylis_controls::Edit>(c).is_some_and(|e| e.text == current) {
            field = Some(c);
            break;
        }
        stack.extend(ide.ui.children(c).iter().copied());
    }
    let field = field.expect("o campo do text");
    ide.ui.set_focus(field);
    let _ = ide.ui.set_prop(field, "text", "Gravar");
    inspector::commit_pending(&mut ide.ui);
    let now = state.borrow().doc.ui.get_prop(button, "text").unwrap().to_str().unwrap().to_string();
    assert_eq!(now, "Gravar");
}
