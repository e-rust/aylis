//! Provedores (item 25): o que depende de projeto de terceiros vira um par. O componente que o
//! app usa diz **o que** faz (o `Barcode`, o `Export`, o `Report`, o `MapView`, com as
//! propriedades e os eventos de sempre); um componente provedor diz **quem** faz (com a
//! biblioteca). O componente aponta o provedor pela propriedade `provider`, como o
//! `data_source` aponta o dataset; sem provedor ligado, vale o provedor padrão do Aylis (não é
//! erro). Trocar de biblioteca é trocar o provedor; um formato novo é um provedor novo; o app
//! só leva o código dos provedores que os forms usam (o `aylis-build` registra por classe).
//!
//! Um contrato (trait) por assunto, aqui no núcleo (sem nada de sistema nem de biblioteca):
//! [`BarcodeProvider`], [`ExportProvider`], [`PdfProvider`] e [`MapTileProvider`]. O
//! componente provedor devolve o seu em [`Component::provided`](crate::Component::provided).

use crate::pages::PageSet;
use crate::ui::Ui;
use crate::component::ComponentId;

/// O que um componente provedor oferece (um assunto por vez).
#[derive(Clone, Copy)]
pub enum Provided<'a> {
    Barcode(&'a dyn BarcodeProvider),
    Export(&'a dyn ExportProvider),
    Pdf(&'a dyn PdfProvider),
    MapTiles(&'a dyn MapTileProvider),
}

// ---------------------------------------------------------------------------
// Códigos de barras

/// Um código pronto para desenhar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BarcodeImage {
    /// De barras (1D): os módulos da esquerda para a direita (`true` = barra escura), e o
    /// texto legível que vai embaixo.
    Linear { modules: Vec<bool>, text: String },
    /// Em matriz (2D, como o QR Code): `width` módulos por linha, linha a linha.
    Matrix { width: usize, modules: Vec<bool> },
}

/// O nível de correção de erro de um código em matriz (o QR Code): quanto do código pode
/// estar estragado (sujo, rasgado, com um logo por cima) e ainda ser lido. Mais correção é
/// um código maior para o mesmo dado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// L: cerca de 7%.
    Low,
    /// M: cerca de 15% (o padrão).
    #[default]
    Medium,
    /// Q: cerca de 25%.
    Quartile,
    /// H: cerca de 30%.
    High,
}

impl crate::prop::PropEnum for ErrorCorrection {
    const NAMES: &'static [&'static str] = &["low", "medium", "quartile", "high"];

    fn from_index(index: usize) -> Option<Self> {
        [ErrorCorrection::Low, ErrorCorrection::Medium, ErrorCorrection::Quartile, ErrorCorrection::High].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// O que o componente pede além do formato e do dado (o provedor usa o que o formato tem).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BarcodeOptions {
    /// A correção de erro dos códigos em matriz (o QR Code); os de barras não têm.
    pub error_correction: ErrorCorrection,
}

/// Quem gera os códigos de barras.
pub trait BarcodeProvider {
    /// Os formatos que sabe gerar (`code128`, `ean13`, `qr`...), em minúsculas.
    fn formats(&self) -> Vec<&'static str>;
    /// O código de `data` no formato `format`, com as opções do componente. Dado que não cabe
    /// no formato: `Err` com o motivo (o controle mostra o motivo no lugar do código).
    fn encode(&self, format: &str, data: &str, options: &BarcodeOptions) -> Result<BarcodeImage, String>;
}

// ---------------------------------------------------------------------------
// Exportação de tabelas (planilha, CSV...)

/// O tipo de uma coluna exportada.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnKind {
    #[default]
    Text,
    Integer,
    /// Número com casas (dinheiro, quantidade).
    Number,
    Date,
    DateTime,
    Bool,
}

/// Um valor de uma célula exportada.
#[derive(Clone, Debug, PartialEq)]
pub enum CellValue {
    Empty,
    Text(String),
    Number(f64),
    Bool(bool),
    /// Data (ano, mês, dia).
    Date(i32, u32, u32),
    /// Data e hora (ano, mês, dia, hora, minuto, segundo).
    DateTime(i32, u32, u32, u32, u32, u32),
}

/// Uma coluna: o título, o tipo e o formato de exibição (`#,##0.00`; vazio: o padrão).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExportColumn {
    pub title: String,
    pub kind: ColumnKind,
    pub format: String,
}

/// Uma tabela para exportar (uma aba da planilha).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExportTable {
    pub name: String,
    pub columns: Vec<ExportColumn>,
    pub rows: Vec<Vec<CellValue>>,
}

/// Quem grava tabelas num formato de arquivo.
pub trait ExportProvider {
    /// A extensão do arquivo, sem o ponto (`xlsx`, `csv`).
    fn extension(&self) -> &'static str;
    /// O tipo do conteúdo (`text/csv`...).
    fn mime(&self) -> &'static str;
    /// O arquivo com as tabelas (uma aba por tabela, onde o formato tem abas).
    fn write(&self, tables: &[ExportTable]) -> Result<Vec<u8>, String>;
}

// ---------------------------------------------------------------------------
// PDF

/// Quem transforma as páginas gravadas (um relatório) em PDF.
pub trait PdfProvider {
    fn write(&self, pages: &PageSet) -> Result<Vec<u8>, String>;
}

/// O PDF do próprio Aylis (as fontes padrão do PDF, sem biblioteca de fora): o padrão quando
/// nenhum provedor está ligado.
pub struct BuiltinPdf;

impl PdfProvider for BuiltinPdf {
    fn write(&self, pages: &PageSet) -> Result<Vec<u8>, String> {
        Ok(pages.to_pdf())
    }
}

// ---------------------------------------------------------------------------
// Mapas

/// Quem fornece as imagens (tiles) do mapa.
pub trait MapTileProvider {
    /// O endereço da imagem do quadro `x`, `y` no nível de zoom `z`.
    fn tile_url(&self, z: u32, x: u32, y: u32) -> String;
    /// O crédito que o mapa mostra no canto (exigido pela maioria dos servidores).
    fn attribution(&self) -> String;
}

/// Um servidor de blocos pelo endereço: `url` com `{z}`, `{x}` e `{y}` (e `{s}`, trocado por
/// `a`, `b` ou `c` para dividir os pedidos), e o crédito que a licença dele pede. O padrão é
/// o OpenStreetMap, o que o `MapView` usa sem provedor ligado; o `TileServerProvider` é este
/// como componente (outro servidor é só outro endereço nele).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileServer {
    pub url: String,
    pub attribution: String,
}

impl Default for TileServer {
    fn default() -> Self {
        Self { url: "https://tile.openstreetmap.org/{z}/{x}/{y}.png".into(), attribution: "© OpenStreetMap".into() }
    }
}

impl TileServer {
    /// O endereço do bloco a partir do modelo (`{z}`, `{x}`, `{y}` e `{s}`).
    pub fn expand(template: &str, z: u32, x: u32, y: u32) -> String {
        let shard = ["a", "b", "c"][((x + y) % 3) as usize];
        template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string()).replace("{s}", shard)
    }
}

impl MapTileProvider for TileServer {
    fn tile_url(&self, z: u32, x: u32, y: u32) -> String {
        Self::expand(&self.url, z, x, y)
    }

    fn attribution(&self) -> String {
        self.attribution.clone()
    }
}

// ---------------------------------------------------------------------------

impl Ui {
    /// O que o componente `id` provê (um componente provedor), se for um.
    pub fn provided(&self, id: ComponentId) -> Option<Provided<'_>> {
        self.component(id)?.provided()
    }
}
