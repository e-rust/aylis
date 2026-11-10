//! Os índices do `Ui`: quem aponta para quem (as propriedades do tipo componente) e os nomes
//! por form. Têm de dar sempre o mesmo que a busca completa, depois de qualquer mudança.

mod common;

use common::{Link, boxy};
use aylis_core::*;

/// Sem `notification`: só o `Ui` pode limpar a referência dele.
#[derive(Default)]
struct BareLink {
    target: Option<ComponentId>,
}

impl Component for BareLink {
    fn class_name(&self) -> &'static str {
        "BareLink"
    }
    fn props(&self) -> &'static [PropInfo] {
        const PROPS: &[PropInfo] = &[PropInfo::new("target", PropKind::Component, "Data")];
        PROPS
    }
    fn get_prop(&self, name: &str) -> Option<PropValue> {
        (name == "target").then(|| self.target.into())
    }
    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        match name {
            "target" => self.target = value.to_component()?,
            _ => return Err(Error::UnknownProperty(name.into())),
        }
        Ok(())
    }
}

#[test]
fn referrers_follow_every_change() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    let a = ui.add(boxy(10.0, 10.0), form);
    let b = ui.add(boxy(10.0, 10.0), form);
    let link = ui.add(Link::default(), form);
    assert!(ui.referrers(a).is_empty());

    ui.set_prop(link, "target", PropValue::Component(Some(a))).unwrap();
    assert_eq!(ui.referrers(a), vec![(link, "target")]);
    ui.set_prop(link, "target", PropValue::Component(Some(b))).unwrap();
    assert!(ui.referrers(a).is_empty());
    assert_eq!(ui.referrers(b), vec![(link, "target")]);
    // Mudado direto no componente (get_mut): o índice cai também.
    ui.get_mut::<Link>(link).unwrap().target = Some(a);
    assert_eq!(ui.referrers(a), vec![(link, "target")]);
    let other = ui.add(Link { target: Some(a) }, form);
    assert_eq!(ui.referrers(a).len(), 2);
    ui.free(other);
    assert_eq!(ui.referrers(a), vec![(link, "target")]);
}

#[test]
fn freeing_a_target_clears_the_references_to_it() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    let target = ui.add(boxy(10.0, 10.0), form);
    let bare = ui.add(BareLink::default(), form);
    ui.set_prop(bare, "target", PropValue::Component(Some(target))).unwrap();

    ui.free(target);

    // O componente não tem `notification`, e mesmo assim ficou sem a referência morta.
    assert_eq!(ui.get_prop(bare, "target"), Ok(PropValue::Component(None)));
}

#[test]
fn names_follow_every_change() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    ui.set_name(form, "frmPedido").unwrap();
    let panel = ui.add(boxy(100.0, 100.0), form);
    let button = ui.add(boxy(10.0, 10.0), panel);
    ui.set_name(button, "btnSalvar").unwrap();
    assert_eq!(ui.find_path("frmPedido.btnSalvar"), Some(button));
    assert_eq!(ui.find("BTNSALVAR"), Some(button), "sem diferenciar maiúsculas");

    ui.set_name(button, "btnGravar").unwrap();
    assert_eq!(ui.find_path("frmPedido.btnSalvar"), None);
    assert_eq!(ui.find_in(form, "btnGravar"), Some(button));

    // Um form embutido tem os próprios nomes: o de fora não enxerga dentro dele.
    let inner = ui.add_root(boxy(100.0, 100.0));
    ui.set_name(inner, "frmItens").unwrap();
    let grid = ui.add(boxy(10.0, 10.0), inner);
    ui.set_name(grid, "grdItens").unwrap();
    ui.set_parent(inner, Some(panel)).unwrap();
    assert_eq!(ui.find_in(form, "grdItens"), None);
    assert_eq!(ui.find_in(form, "frmItens"), Some(inner));
    assert_eq!(ui.find_path("frmPedido.frmItens.grdItens"), Some(grid));

    ui.free(button);
    assert_eq!(ui.find("btnGravar"), None);
}

/// Liberar várias de uma vez (esvaziar uma lista): o índice de referências perde só o que
/// saiu (sem remontar) e continua igual à busca completa; o de nomes segue certo.
#[test]
fn freeing_many_at_once_keeps_the_indexes_right() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    let list = ui.add(boxy(200.0, 200.0), form);
    let rows: Vec<ComponentId> = (0..50).map(|_| ui.add(boxy(10.0, 10.0), list)).collect();
    let kept = ui.add(boxy(10.0, 10.0), form);
    ui.set_name(kept, "kept").unwrap();
    let named_row = rows[3];
    ui.set_name(named_row, "row3").unwrap();
    // Uma linha aponta para outra que fica; outra aponta para uma que sai.
    let to_kept = ui.add(BareLink::default(), rows[0]);
    ui.set_prop(to_kept, "target", PropValue::Component(Some(kept))).unwrap();
    let outside = ui.add(BareLink::default(), form);
    ui.set_prop(outside, "target", PropValue::Component(Some(rows[1]))).unwrap();
    assert_eq!(ui.referrers(kept), vec![(to_kept, "target")]);

    assert_eq!(ui.free_many(&rows), 50);
    assert!(ui.children(list).is_empty());
    assert!(rows.iter().all(|&r| !ui.contains(r)));
    assert!(ui.referrers(kept).is_empty(), "quem apontava saiu junto");
    assert_eq!(ui.get_prop(outside, "target"), Ok(PropValue::Component(None)), "a referência morta foi limpa");
    assert_eq!(ui.find("kept"), Some(kept));
    assert_eq!(ui.find("row3"), None);
    // Repetir não libera nada.
    assert_eq!(ui.free_many(&rows), 0);
}
