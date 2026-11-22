//! As colunas de ícone e de imagem do `DataGrid` (decididas em 22/11): o campo diz a posição (ou
//! o nome) do ícone num `ImageList`, ou guarda a imagem (um `blob`), e a grade desenha na
//! altura da linha (`row_height`).

use aylis_controls::{Form, ImageList};
use aylis_core::icons::Icon;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

/// Canvas que conta os ícones (os contornos preenchidos) e as imagens desenhadas.
#[derive(Default)]
struct Probe {
    paths: usize,
    images: Vec<RectF>,
    texts: Vec<String>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, _: RectF, _: &TextStyle) {
        self.texts.push(text.to_string());
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
    fn fill_path(&mut self, _: &aylis_core::Path, _: &Brush) {
        self.paths += 1;
    }
    fn draw_image(&mut self, _: &Image, _: RectF, dest: RectF, _: f32) {
        self.images.push(dest);
    }
    fn image_size(&mut self, _: &Image) -> Option<SizeF> {
        Some(SizeF::new(100.0, 100.0))
    }
}

#[test]
fn icon_and_image_columns_draw_from_the_image_list_and_the_blob() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    let form = ui.add_root(Form::new("grid", 700.0, 400.0));
    // O ícone do "tocando": um quadrado na grade de 24.
    ui.register_icon("app:tocando", Icon { size: 24.0, fill: aylis_core::Path::from_svg("M4 4H20V20H4Z").unwrap(), ..Default::default() });
    let images = ui.add(ImageList::default(), form);
    ui.set_prop(images, "items", r#"[{"name": "parada"}, {"name": "tocando", "icon": "app:tocando"}]"#).unwrap();
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("estado", FieldType::Integer), cds);
    ui.add(Field::new("capa", FieldType::Blob), cds);
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 600.0, 332.0);
    ui.set_prop(grid, "images", PropValue::Component(Some(images))).unwrap();
    ui.set_prop(grid, "row_height", 56.0).unwrap();
    ui.set_prop(grid, "columns", r#"[{"field": "estado", "kind": "icon", "width": 48}, {"field": "id"}, {"field": "capa", "kind": "image", "width": 80}]"#).unwrap();
    ui.layout();
    // A linha 1 toca (estado 1); a 2 está parada (0, um item sem ícone); só a 1 tem capa.
    let png = vec![0x89, b'P', b'N', b'G', 1, 2, 3];
    data_set(&mut ui, cds).load_rows(vec![vec![1.into(), 1.into(), Value::Blob(png.into())], vec![2.into(), 0.into(), Value::Null]]).unwrap();
    ui.layout();
    let mut probe = Probe::default();
    ui.paint(form, &mut probe);
    assert_eq!(probe.paths, 1, "um ícone: o da linha que toca");
    assert_eq!(probe.images.len(), 1, "uma capa");
    assert!(!probe.texts.iter().any(|t| t == "0"), "o item sem ícone fica vazio, sem o número: {:?}", probe.texts);
    // A capa cabe na altura da linha (56, menos a margem).
    let capa = probe.images[0];
    assert!(capa.height <= 48.0 + 0.1 && capa.height > 40.0, "{capa:?}");
    // A coluna volta ao JSON com o tipo.
    let cols = ColumnSpec::parse_list(r#"[{"field": "capa", "kind": "image"}]"#);
    assert!(ColumnSpec::list_to_json(&cols).contains("\"kind\":\"image\""));
}
