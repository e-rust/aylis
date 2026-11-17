//! O ícone nos nós do TreeView sem desalinhar o texto, e onde cada nó é pintado (pedido vindo
//! de um app em 17/11).

use aylis_controls::{Form, TreeNode, TreeView};
use aylis_core::{PropValue, Ui};

fn tree(ui: &mut Ui, items: &str) -> aylis_core::ComponentId {
    aylis_controls::register(ui);
    let f = ui.add_root(Form::new("teste", 400.0, 300.0));
    let t = ui.add(TreeView::default(), f);
    ui.control_mut(t).unwrap().set_rect(0.0, 0.0, 240.0, 200.0);
    ui.set_prop(t, "items", items).unwrap();
    ui.update_layout();
    t
}

#[test]
fn the_icon_comes_after_a_pipe_and_every_node_keeps_the_text_aligned() {
    let mut ui = Ui::new();
    let t = tree(&mut ui, "Brasil|folder;>Campinas|home;Portugal;>Lisboa;Chile|flag;>Santiago");
    let tv = ui.get::<TreeView>(t).unwrap();
    assert_eq!((tv.nodes()[0].text.as_str(), tv.nodes()[0].icon.as_str()), ("Brasil", "folder"));
    assert_eq!(tv.nodes()[1].icon, "");
    let a = tv.node_rects(&[0]).unwrap();
    let b = tv.node_rects(&[1]).unwrap();
    assert!(a.icon.is_some() && b.icon.is_some(), "a coluna do ícone vale para todos os pais");
    assert_eq!(a.text.x, b.text.x, "o texto dos pais alinhado com e sem ícone");
    assert!(a.text.x >= a.icon.unwrap().right(), "o texto depois do ícone");
    assert_eq!(b.row.y - a.row.y, 26.0, "uma linha abaixo");

    // Sem ícone em nenhum nó, nada de coluna (como antes).
    ui.set_prop(t, "items", "Brasil;>Campinas;Portugal").unwrap();
    let tv = ui.get::<TreeView>(t).unwrap();
    let c = tv.node_rects(&[0]).unwrap();
    assert!(c.icon.is_none() && c.text.x < a.text.x);

    // As folhas seguem o `leaf_layout`; Rio é folha sem ícone, irmã de São Paulo.
    ui.set_prop(t, "items", "Brasil|folder;>São Paulo|folder;>>Campinas|home;>Rio").unwrap();
    ui.get_mut::<TreeView>(t).unwrap().expand_all(true);
    let rects = |ui: &Ui, path: &[usize]| ui.get::<TreeView>(t).unwrap().node_rects(path).unwrap();
    // aligned (o padrão): todo texto do mesmo nível no mesmo lugar, o ícone na coluna dos pais.
    let (sp, rio) = (rects(&ui, &[0, 0]), rects(&ui, &[0, 1]));
    assert_eq!(rio.text.x, sp.text.x, "o texto da folha alinhado com o irmão com filhos");
    assert_eq!(rio.icon.unwrap().x, sp.icon.unwrap().x, "o ícone na coluna dos pais");
    // icon_in_arrow: o ícone no lugar da seta, o texto continua alinhado.
    ui.set_prop(t, "leaf_layout", PropValue::Enum("icon_in_arrow".into())).unwrap();
    let (sp, rio) = (rects(&ui, &[0, 0]), rects(&ui, &[0, 1]));
    assert_eq!(rio.text.x, sp.text.x);
    let icon = rio.icon.unwrap();
    assert!(icon.x >= rio.arrow.x && icon.right() <= rio.arrow.right(), "no lugar da seta: {icon:?} {:?}", rio.arrow);
    // compact: sem o espaço da seta, o texto antes do dos irmãos com filhos.
    ui.set_prop(t, "leaf_layout", PropValue::Enum("compact".into())).unwrap();
    let (sp, rio) = (rects(&ui, &[0, 0]), rects(&ui, &[0, 1]));
    assert!(rio.text.x < sp.text.x && rio.icon.unwrap().x == rio.arrow.x);
    // O recuo, o tamanho do ícone e o espaço até o texto também são do app.
    ui.set_prop(t, "leaf_layout", PropValue::Enum("aligned".into())).unwrap();
    let before = rects(&ui, &[0, 0]);
    ui.set_prop(t, "indent", 30.0).unwrap();
    ui.set_prop(t, "icon_size", 20.0).unwrap();
    ui.set_prop(t, "icon_gap", 10.0).unwrap();
    let after = rects(&ui, &[0, 0]);
    assert_eq!(after.arrow.width, 30.0);
    assert_eq!(after.icon.unwrap().width, 20.0);
    assert_eq!(after.text.x - after.icon.unwrap().right(), 10.0);
    assert!(after.text.x > before.text.x);

    // A API grava o ícone no texto de `items`.
    let tv = ui.get_mut::<TreeView>(t).unwrap();
    tv.set_nodes(vec![TreeNode::new("A").with_icon("file").with_children(vec![TreeNode::new("B")])]);
    assert_eq!(tv.items, "A|file;>B");
}

#[test]
fn item_rect_says_where_a_node_is_painted() {
    let mut ui = Ui::new();
    let t = tree(&mut ui, "Brasil;>São Paulo;Portugal");
    // São Paulo está dentro de um nó fechado: não está à vista.
    assert_eq!(ui.invoke(t, "item_rect", &[PropValue::String("Brasil/São Paulo".into())]).unwrap(), PropValue::Null);
    let PropValue::Bounds(b) = ui.invoke(t, "item_rect", &[PropValue::String("Portugal".into())]).unwrap() else { panic!() };
    assert_eq!((b.top, b.bottom - b.top), (27.0, 26.0), "a segunda linha visível: {b:?}");

    ui.get_mut::<TreeView>(t).unwrap().expand_all(true);
    let PropValue::Bounds(sp) = ui.invoke(t, "item_text_rect", &[PropValue::String("Brasil/São Paulo".into())]).unwrap() else { panic!() };
    let PropValue::Bounds(br) = ui.invoke(t, "item_text_rect", &[PropValue::String("Brasil".into())]).unwrap() else { panic!() };
    assert!(sp.left > br.left && sp.top > br.top, "o filho recuado e abaixo: {sp:?} {br:?}");
    assert!(ui.invoke(t, "item_rect", &[]).is_err());
}
