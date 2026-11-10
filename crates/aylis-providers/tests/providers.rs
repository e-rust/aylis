//! Provedores (itens 23 e 25): o componente diz o que faz, o provedor ligado em `provider`
//! diz quem faz; sem provedor, o padrão do Aylis. Exportar para CSV e XLSX, um código de
//! barras por outro provedor, o código de cada registro no relatório, o PDF pelo provedor e
//! o servidor do mapa.

use std::io::Read;

use aylis_controls::Form;
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::providers::{BarcodeImage, BarcodeOptions, BarcodeProvider, ErrorCorrection, MapTileProvider, Provided};
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::{BandKind, Report, ReportBand, report};
use aylis_providers::*;

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    aylis_providers::register(&mut ui);
    let form = ui.add_root(Form::new("x", 800.0, 600.0));
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("codigo", FieldType::String).label("Código"), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome"), cds);
    ui.add(Field::new("preco", FieldType::Currency).label("Preço").format("#,##0.00"), cds);
    ui.add(Field::new("validade", FieldType::Date).label("Validade"), cds);
    let rows = vec![
        vec!["7891000100103".into(), "Café; torrado".into(), Decimal::new(1590, 2).into(), Value::Date(Date::new(2027, 1, 31).unwrap())],
        vec!["7891000053508".into(), "Açúcar \"cristal\"".into(), Decimal::new(489, 2).into(), Value::Null],
    ];
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    let ds = ui.add(DataSource::new(cds), form);
    (ui, form, ds)
}

#[test]
fn export_writes_csv_by_default_and_xlsx_with_its_provider() {
    let (mut ui, form, ds) = setup();
    let export = ui.add(Export { data_source: Some(ds), ..Default::default() }, form);
    let csv = String::from_utf8(Export::bytes(&ui, export).unwrap()).unwrap();
    assert_eq!(
        csv,
        "\u{feff}Código;Nome;Preço;Validade\r\n7891000100103;\"Café; torrado\";15,9;2027-01-31\r\n7891000053508;\"Açúcar \"\"cristal\"\"\";4,89;\r\n",
        "o padrão: CSV com ; e vírgula decimal, aspas só onde precisa"
    );
    // A planilha: outro provedor, o mesmo Export.
    let xlsx = ui.add(XlsxProvider, form);
    ui.set_prop(export, "provider", PropValue::Component(Some(xlsx))).unwrap();
    assert_eq!(ui.invoke(export, "extension", &[]).unwrap(), PropValue::String("xlsx".into()));
    let bytes = Export::bytes(&ui, export).unwrap();
    assert_eq!(&bytes[..2], b"PK", "um .xlsx é um zip");
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    let mut shared = String::new();
    zip.by_name("xl/sharedStrings.xml").unwrap().read_to_string(&mut shared).unwrap();
    assert!(shared.contains("Café; torrado") && shared.contains("Preço"), "{shared}");
    let mut book = String::new();
    zip.by_name("xl/workbook.xml").unwrap().read_to_string(&mut book).unwrap();
    assert!(book.contains("name=\"Dados\""), "a aba: {book}");
    // Gravar pelo método (o que as outras linguagens chamam).
    let path = std::env::temp_dir().join(format!("aylis_export_{}.xlsx", std::process::id()));
    assert_eq!(ui.invoke(export, "save_to", &[PropValue::String(path.display().to_string())]).unwrap(), PropValue::Bool(true));
    assert!(std::fs::metadata(&path).unwrap().len() > 1000);
    let _ = std::fs::remove_file(&path);
}

/// Um formato que o Aylis não tem, de outro provedor (o jeito de acrescentar um formato).
#[derive(Default)]
struct Morse;

impl BarcodeProvider for Morse {
    fn formats(&self) -> Vec<&'static str> {
        vec!["morse"]
    }
    fn encode(&self, _format: &str, data: &str, options: &BarcodeOptions) -> Result<BarcodeImage, String> {
        // A correção de erro que chegou do componente vai no texto (para o teste ver).
        let text = if options.error_correction == ErrorCorrection::Medium { data.into() } else { format!("{data} {:?}", options.error_correction) };
        Ok(BarcodeImage::Linear { modules: data.chars().flat_map(|c| if c == '1' { [true, true, true, false] } else { [true, false, false, false] }).collect(), text })
    }
}

impl Component for Morse {
    fn class_name(&self) -> &'static str {
        "Morse"
    }
    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Barcode(self))
    }
}

fn bars(ui: &mut Ui, form: ComponentId) -> usize {
    let mut canvas = RecordingCanvas::new(None);
    ui.layout();
    ui.paint(form, &mut canvas);
    canvas.ops.iter().filter(|op| matches!(op, Op::FillRect { .. })).count()
}

#[test]
fn a_barcode_draws_with_the_builtin_provider_or_another_one() {
    let (mut ui, form, _) = setup();
    let empty = bars(&mut ui, form);
    let code = ui.add(Barcode::new("ean13", "789100010010"), form);
    ui.control_mut(code).unwrap().set_rect(10.0, 10.0, 300.0, 100.0);
    let ean = bars(&mut ui, form) - empty;
    assert!(ean > 25, "as barras do EAN-13: {ean}");
    ui.set_prop(code, "format", "pdf417").unwrap();
    assert_eq!(bars(&mut ui, form) - empty, 1, "formato que o padrão não tem: só o fundo e o aviso");
    let morse = ui.add(Morse, form);
    ui.set_prop(code, "provider", PropValue::Component(Some(morse))).unwrap();
    ui.set_prop(code, "format", "morse").unwrap();
    ui.set_prop(code, "text", "1011").unwrap();
    assert_eq!(bars(&mut ui, form) - empty, 1 + 4, "o fundo do código e as 4 barras do outro provedor");
    ui.set_prop(code, "format", "qr").unwrap();
    ui.set_prop(code, "provider", PropValue::Component(None)).unwrap();
    assert!(bars(&mut ui, form) - empty > 100, "QR Code pelo padrão");
}

#[test]
fn the_qr_error_correction_is_a_property_that_reaches_the_provider() {
    let (mut ui, form, _) = setup();
    let code = ui.add(Barcode::new("qr", "https://aylis.dev/docs/componentes/Barcode?registro=000123456789"), form);
    assert_eq!(ui.get_prop(code, "error_correction").unwrap(), PropValue::Enum("medium".into()), "o padrão é o M");
    let size = |ui: &Ui| match ui.get::<Barcode>(code).unwrap().encode(None).unwrap() {
        BarcodeImage::Matrix { width, .. } => width,
        other => panic!("{other:?}"),
    };
    let medium = size(&ui);
    ui.set_prop(code, "error_correction", PropValue::Enum("high".into())).unwrap();
    assert!(size(&ui) > medium, "H: mais módulos que o M");
    assert!(ui.set_prop(code, "error_correction", PropValue::Enum("x".into())).is_err(), "só L, M, Q e H");
    // O nível vai para o provedor ligado, não só para o do Aylis.
    let morse = ui.add(Morse, form);
    ui.set_prop(code, "provider", PropValue::Component(Some(morse))).unwrap();
    ui.set_prop(code, "format", "morse").unwrap();
    ui.set_prop(code, "text", "10").unwrap();
    ui.set_prop(code, "error_correction", PropValue::Enum("quartile".into())).unwrap();
    ui.control_mut(code).unwrap().set_rect(10.0, 10.0, 300.0, 100.0);
    let mut canvas = RecordingCanvas::new(None);
    ui.layout();
    ui.paint(form, &mut canvas);
    let texts: Vec<&str> = canvas.ops.iter().filter_map(|op| if let Op::Text { text, .. } = op { Some(text.as_str()) } else { None }).collect();
    assert!(texts.contains(&"10 Quartile"), "o provedor ligado recebeu o Q: {texts:?}");
}

#[test]
fn each_record_gets_its_barcode_in_the_report_and_the_pdf_comes_from_the_provider() {
    let (mut ui, form, ds) = setup();
    let rep = ui.add(Report::new(ds), form);
    let band = ui.add(ReportBand::new(BandKind::Detail, 60.0), rep);
    let code = ui.add(Barcode::new("ean13", "[codigo]"), band);
    ui.control_mut(code).unwrap().set_rect(0.0, 0.0, 200.0, 56.0);
    ui.layout();
    let pages = report::prepare(&mut ui, rep);
    let texts = pages.pages[0].texts();
    assert!(texts.contains(&"7891000100103") && texts.contains(&"7891000053508"), "cada registro com o seu código: {texts:?}");
    assert_eq!(ui.get_prop(code, "text").unwrap(), PropValue::String("[codigo]".into()), "o modelo volta depois");
    let builtin = report::to_pdf(&mut ui, rep);
    let pdf = ui.add(AylisPdfProvider, form);
    ui.set_prop(rep, "provider", PropValue::Component(Some(pdf))).unwrap();
    assert_eq!(report::to_pdf(&mut ui, rep), builtin, "o provedor do Aylis dá o mesmo PDF");
}

#[test]
fn the_tile_server_provider_gives_the_url_and_the_credit() {
    let tiles = TileServerProvider { url: "https://{s}.tiles.exemplo.com/{z}/{x}/{y}.png?k=1".into(), attribution: "© Exemplo".into() };
    assert_eq!(tiles.tile_url(3, 1, 1), "https://c.tiles.exemplo.com/3/1/1.png?k=1");
    assert_eq!(tiles.attribution(), "© Exemplo");
    assert_eq!(TileServerProvider::default().tile_url(1, 0, 0), "https://tile.openstreetmap.org/1/0/0.png");
}

/// Guarda os endereços pedidos (sem rede).
struct Requests(std::sync::Arc<std::sync::Mutex<Vec<String>>>);

impl aylis_core::platform::HttpService for Requests {
    fn get(&mut self, url: &str, _done: Box<dyn FnOnce(Result<Vec<u8>, String>) + Send>) {
        self.0.lock().unwrap().push(url.to_string());
    }
}

/// Pinta o mapa e devolve os endereços dos blocos pedidos e o texto do crédito.
fn map_requests(ui: &mut Ui, form: ComponentId, log: &std::sync::Arc<std::sync::Mutex<Vec<String>>>) -> (Vec<String>, Vec<String>) {
    log.lock().unwrap().clear();
    let mut canvas = RecordingCanvas::new(None);
    ui.layout();
    ui.paint(form, &mut canvas);
    ui.process_posted();
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(form, &mut canvas);
    let texts = canvas.ops.iter().filter_map(|op| if let Op::Text { text, .. } = op { Some(text.clone()) } else { None }).collect();
    (log.lock().unwrap().clone(), texts)
}

#[test]
fn the_map_server_comes_only_from_the_provider() {
    let (mut ui, form, _) = setup();
    let log = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    ui.services_mut().http = Some(Box::new(Requests(log.clone())));
    let map = ui.add(aylis_controls::MapView::default(), form);
    ui.control_mut(map).unwrap().set_rect(0.0, 0.0, 400.0, 300.0);
    // Um caminho só: o MapView não tem mais endereço nem crédito próprios.
    assert!(ui.get_prop(map, "tile_url").is_err() && ui.get_prop(map, "attribution").is_err());
    let (urls, texts) = map_requests(&mut ui, form, &log);
    assert!(!urls.is_empty() && urls.iter().all(|u| u.starts_with("https://tile.openstreetmap.org/")), "sem provedor, o OpenStreetMap: {urls:?}");
    assert!(texts.iter().any(|t| t == "© OpenStreetMap"), "{texts:?}");
    let tiles = ui.add(TileServerProvider { url: "https://{s}.tiles.exemplo.com/{z}/{x}/{y}.png".into(), attribution: "© Exemplo".into() }, form);
    ui.set_prop(map, "provider", PropValue::Component(Some(tiles))).unwrap();
    let (urls, texts) = map_requests(&mut ui, form, &log);
    assert!(!urls.is_empty() && urls.iter().all(|u| u.contains(".tiles.exemplo.com/")), "com o provedor, o servidor dele: {urls:?}");
    assert!(texts.iter().any(|t| t == "© Exemplo") && !texts.iter().any(|t| t == "© OpenStreetMap"), "{texts:?}");
}
