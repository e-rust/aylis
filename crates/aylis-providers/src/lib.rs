//! # aylis-providers
//!
//! Os provedores (item 25): o que depende de projeto de terceiros vira um par, o componente
//! que o app usa e o provedor que faz (veja `aylis_core::providers`). Aqui ficam:
//!
//! - [`Barcode`]: código de barras e QR Code, com o provedor padrão [`AylisBarcodeProvider`]
//!   (Code 128, EAN-13, EAN-8, UPC-A, Code 39, ITF e QR Code);
//! - [`Export`]: os registros de um dataset num arquivo, pelo [`CsvProvider`] (o padrão) ou
//!   pelo [`XlsxProvider`] (planilha do Excel);
//! - [`AylisPdfProvider`]: o PDF do próprio Aylis para o `Report` (o padrão);
//! - [`TileServerProvider`]: o servidor dos blocos do `MapView` (o endereço e o crédito);
//! - [`OleObject`]: a automação OLE de um programa instalado (Excel, Word...), no Windows;
//! - [`WmiQuery`]: consultas e eventos do WMI (hardware, discos, processos), no Windows.
//!
//! Os provedores ficam na aba "Providers" da paleta.

mod barcode;
pub mod barcodes;
mod export;
pub mod ole;
pub mod wmi;

pub use barcode::{Barcode, AylisBarcodeProvider};
pub use barcodes::BuiltinBarcodes;
pub use export::{CsvProvider, Export, XlsxProvider};
pub use ole::OleObject;
pub use wmi::WmiQuery;

use aylis_core::providers::{BuiltinPdf, MapTileProvider, Provided, TileServer};
use aylis_core::{Component, Ui, properties};

/// O PDF do próprio Aylis como componente (para ligar explicitamente ao `Report`; sem
/// provedor, ele já usa este).
#[derive(Default)]
pub struct AylisPdfProvider;

impl Component for AylisPdfProvider {
    fn class_name(&self) -> &'static str {
        "AylisPdfProvider"
    }


    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Pdf(&BuiltinPdf))
    }
}

/// Um servidor de blocos de mapa (o único caminho para o `MapView` trocar de servidor): `url`
/// com `{z}`, `{x}` e `{y}` (e `{s}`, trocado por `a`, `b` ou `c` para dividir os pedidos), e
/// o crédito que a licença dele pede. O padrão é o OpenStreetMap (o mesmo do `MapView` sem
/// provedor); um servidor pago ou próprio é só outro endereço.
pub struct TileServerProvider {
    /// O endereço dos blocos, com `{z}`, `{x}` e `{y}` (e `{s}`, trocado por `a`, `b` ou `c`).
    pub url: String,
    /// O crédito que a licença do servidor pede, mostrado no canto do mapa.
    pub attribution: String,
}

impl Default for TileServerProvider {
    fn default() -> Self {
        let TileServer { url, attribution } = TileServer::default();
        Self { url, attribution }
    }
}

impl Component for TileServerProvider {
    fn class_name(&self) -> &'static str {
        "TileServerProvider"
    }

    properties! {
        url: String => "Data",
        attribution: String => "Data",
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::MapTiles(self))
    }
}

impl MapTileProvider for TileServerProvider {
    fn tile_url(&self, z: u32, x: u32, y: u32) -> String {
        TileServer::expand(&self.url, z, x, y)
    }

    fn attribution(&self) -> String {
        self.attribution.clone()
    }
}

/// Registra as classes deste crate.
pub fn register(ui: &mut Ui) {
    ui.register_class::<Barcode>("Reports");
    ui.register_class::<Export>("Data");
    ui.register_class::<OleObject>("System");
    ui.register_class::<WmiQuery>("System");
    ui.register_class::<AylisBarcodeProvider>("Providers");
    ui.register_class::<CsvProvider>("Providers");
    ui.register_class::<XlsxProvider>("Providers");
    ui.register_class::<AylisPdfProvider>("Providers");
    ui.register_class::<TileServerProvider>("Providers");
}
