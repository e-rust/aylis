//! Monta um form típico de cadastro, todo criado por nome de classe e propriedades
//! (como faria o carregador do arquivo de layout), e mostra as posições calculadas
//! antes e depois de redimensionar o form.
//!
//! ```text
//! cargo run -p aylis-core --example align_demo
//! ```

use aylis_core::*;

/// Um controle qualquer, só para o demo (ainda não há controles de verdade no core).
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
}

fn block(ui: &mut Ui, parent: Option<ComponentId>, props: &[(&str, PropValue)]) -> ComponentId {
    let id = ui.create("Block", parent).expect("classe registrada");
    for (name, value) in props {
        ui.set_prop(id, name, value.clone()).unwrap_or_else(|e| panic!("{e}"));
    }
    id
}

fn main() {
    let mut ui = Ui::new();
    ui.register_class::<Block>("Demo");

    let form = block(&mut ui, None, &[
        ("name", "FrmClientes".into()),
        ("width", 640.into()),
        ("height", 480.into()),
        ("padding", 8.into()),
    ]);

    let toolbar = block(&mut ui, Some(form), &[
        ("name", "pnlToolbar".into()),
        ("align", "top".into()),
        ("height", 40.into()),
        ("margins.bottom", 8.into()),
        ("padding", 4.into()),
    ]);
    for name in ["btnNovo", "btnSalvar", "btnExcluir"] {
        block(&mut ui, Some(toolbar), &[
            ("name", name.into()),
            ("align", "left".into()),
            ("width", 90.into()),
            ("margins.right", 4.into()),
        ]);
    }

    block(&mut ui, Some(form), &[
        ("name", "pnlStatus".into()),
        ("align", "bottom".into()),
        ("height", 24.into()),
    ]);
    block(&mut ui, Some(form), &[
        ("name", "pnlMenu".into()),
        ("align", "left".into()),
        ("width", 160.into()),
        ("margins.right", 8.into()),
    ]);
    let content = block(&mut ui, Some(form), &[("name", "pnlConteudo".into()), ("align", "client".into())]);
    block(&mut ui, Some(content), &[
        ("name", "btnOk".into()),
        ("left", 340.into()),
        ("top", 330.into()),
        ("width", 100.into()),
        ("height", 28.into()),
        ("anchors", "right,bottom".into()),
    ]);

    ui.on(content, "on_resize", |ui, sender, _| {
        let b = ui.bounds(sender).unwrap();
        println!("  on_resize: pnlConteudo agora tem {}x{}", b.width, b.height);
    });

    println!("== 640 x 480");
    ui.layout();
    print!("{}", ui.dump(form));

    println!("\n== 1024 x 768 (btnOk acompanha o canto inferior direito)");
    ui.set_prop(form, "width", 1024).unwrap();
    ui.set_prop(form, "height", 768).unwrap();
    ui.layout();
    print!("{}", ui.dump(form));
}
