//! Provedores (item 25): o que depende de projeto de terceiros vira um par. O componente que o
//! app usa diz **o que** faz (o `Barcode`, o `Export`, o `Report`, o `MapView`, com as
//! propriedades e os eventos de sempre); um componente provedor diz **quem** faz (com a
//! biblioteca). O componente aponta o provedor pela propriedade `provider`, como o
//! `data_source` aponta o dataset; sem provedor ligado, vale o provedor padrão do Aylis (não é
//! erro). Trocar de biblioteca é trocar o provedor; um formato novo é um provedor novo; o app
//! só leva o código dos provedores que os forms usam (o `aylis-build` registra por classe).
//!
//! Um contrato (trait) por assunto, aqui no núcleo (sem nada de sistema nem de biblioteca):
//! [`BarcodeProvider`], [`ExportProvider`], [`PdfProvider`], [`MapTileProvider`],
//! [`MediaTagsProvider`], [`HttpProvider`] e [`WebSocketProvider`] (os clientes de rede). O
//! componente provedor devolve o seu em [`Component::provided`](crate::Component::provided).

use crate::pages::{Page, PageSet};
use crate::ui::Ui;
use crate::component::ComponentId;

/// O que um componente provedor oferece (um assunto por vez).
#[derive(Clone, Copy)]
pub enum Provided<'a> {
    Barcode(&'a dyn BarcodeProvider),
    Export(&'a dyn ExportProvider),
    Pdf(&'a dyn PdfProvider),
    MapTiles(&'a dyn MapTileProvider),
    MediaTags(&'a dyn MediaTagsProvider),
    Http(&'a dyn HttpProvider),
    WebSocket(&'a dyn WebSocketProvider),
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

/// Quem transforma as páginas gravadas (um relatório) em PDF. O documento é montado página
/// por página ([`PdfProvider::begin`]): um relatório enorme não precisa de todas as páginas na
/// memória ao mesmo tempo. O padrão (sem provedor ligado no `Report`) é o de
/// [`Services::pdf`](crate::Services::pdf), que o `aylis-providers` põe (o `KrillaPdfProvider`);
/// o núcleo não escreve PDF (decidido pelo autor em 15/11).
pub trait PdfProvider {
    /// Começa um documento com páginas de `width` × `height` pontos.
    fn begin(&self, width: f32, height: f32) -> Result<Box<dyn PdfDocument>, String>;

    /// O PDF de todas as páginas de uma vez.
    fn write(&self, pages: &PageSet) -> Result<Vec<u8>, String> {
        let mut doc = self.begin(pages.width, pages.height)?;
        for page in &pages.pages {
            doc.add_page(page)?;
        }
        doc.finish()
    }
}

/// Um PDF sendo montado ([`PdfProvider::begin`]).
pub trait PdfDocument {
    fn add_page(&mut self, page: &Page) -> Result<(), String>;
    /// Os bytes do arquivo.
    fn finish(self: Box<Self>) -> Result<Vec<u8>, String>;
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

// ---------------------------------------------------------------------------
// Tags de mídia

/// O que se lê de um arquivo de áudio ou vídeo: as tags, a duração e a capa embutida (o
/// `MediaInfo`; decidido em 21/11: um componente com provedor, não propriedades do player).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaTags {
    pub title: String,
    pub artist: String,
    pub album: String,
    /// O número da faixa no álbum (0: sem).
    pub track: u32,
    /// O ano (0: sem).
    pub year: u32,
    /// Em segundos (0: não se sabe).
    pub duration: f64,
    /// A imagem embutida (a capa do álbum), como está no arquivo (PNG, JPEG).
    pub cover: Option<Vec<u8>>,
}

/// De onde ler: um caminho do disco ou os bytes (um arquivo do projeto), com a extensão.
#[derive(Clone, Debug)]
pub enum MediaInput {
    Path(String),
    Bytes(Vec<u8>, String),
}

/// Quem lê as tags: roda em outra thread (uma playlist inteira em segundo plano).
pub type TagReader = std::sync::Arc<dyn Fn(&MediaInput) -> Result<MediaTags, String> + Send + Sync>;

/// O provedor de tags de mídia: devolve o leitor (que vai para outra thread).
pub trait MediaTagsProvider {
    fn tag_reader(&self) -> TagReader;
}

// ---------------------------------------------------------------------------
// Rede: HTTP e WebSocket

/// Um pedido HTTP: o método, o endereço, os cabeçalhos e o corpo, com os limites que o
/// provedor respeita (o prazo, o tamanho da resposta e os redirecionamentos).
#[derive(Clone, Debug, PartialEq)]
pub struct HttpRequest {
    /// `GET`, `POST`, `PUT`, `PATCH`, `DELETE`... (em maiúsculas).
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    /// O prazo do pedido inteiro (conectar, mandar e ler), em milissegundos; 0: sem prazo.
    pub timeout_ms: u32,
    /// A maior resposta aceita, em bytes (acima disso, erro); 0: o padrão (16 MB).
    pub max_response_bytes: u64,
    /// Quantos redirecionamentos seguir (0: nenhum; a resposta 3xx volta como está).
    pub max_redirects: u32,
    /// Aceita `http://` (sem TLS) para fora deste computador, também num redirecionamento.
    /// Desligado, só `https://`, e `http://` só para `localhost`/`127.0.0.1`/`::1`.
    pub allow_http: bool,
}

/// O tamanho padrão da maior resposta aceita (16 MB).
pub const HTTP_MAX_RESPONSE: u64 = 16 * 1024 * 1024;

impl Default for HttpRequest {
    fn default() -> Self {
        Self { method: "GET".into(), url: String::new(), headers: Vec::new(), body: Vec::new(), timeout_ms: 30_000, max_response_bytes: HTTP_MAX_RESPONSE, max_redirects: 5, allow_http: false }
    }
}

impl HttpRequest {
    /// Um pedido com o método e o endereço.
    pub fn new(method: &str, url: impl Into<String>) -> Self {
        Self { method: method.trim().to_ascii_uppercase(), url: url.into(), ..Default::default() }
    }

    pub fn get(url: impl Into<String>) -> Self {
        Self::new("GET", url)
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self::new("POST", url)
    }

    pub fn put(url: impl Into<String>) -> Self {
        Self::new("PUT", url)
    }

    pub fn patch(url: impl Into<String>) -> Self {
        Self::new("PATCH", url)
    }

    pub fn delete(url: impl Into<String>) -> Self {
        Self::new("DELETE", url)
    }

    /// Põe um cabeçalho (troca o de mesmo nome, sem diferença de maiúsculas).
    pub fn header(mut self, name: &str, value: impl Into<String>) -> Self {
        self.set_header(name, value);
        self
    }

    /// Como [`header`](Self::header), sem consumir o pedido.
    pub fn set_header(&mut self, name: &str, value: impl Into<String>) {
        self.headers.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
        self.headers.push((name.to_string(), value.into()));
    }

    /// O valor de um cabeçalho do pedido.
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }

    /// `Authorization: Bearer <token>`.
    pub fn bearer(self, token: &str) -> Self {
        self.header("Authorization", format!("Bearer {token}"))
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// O corpo em texto JSON já montado, com `Content-Type: application/json`.
    pub fn json_text(self, json: impl Into<String>) -> Self {
        self.header("Content-Type", "application/json").body(json.into().into_bytes())
    }

    pub fn timeout_ms(mut self, ms: u32) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn max_response_bytes(mut self, bytes: u64) -> Self {
        self.max_response_bytes = bytes;
        self
    }

    pub fn max_redirects(mut self, n: u32) -> Self {
        self.max_redirects = n;
        self
    }

    pub fn allow_http(mut self, allow: bool) -> Self {
        self.allow_http = allow;
        self
    }

    /// O limite da resposta valendo (0 vira o padrão).
    pub fn response_limit(&self) -> u64 {
        if self.max_response_bytes == 0 { HTTP_MAX_RESPONSE } else { self.max_response_bytes }
    }

    /// Confere o esquema de `url` (`https`, ou `http` como diz [`allow_http`](Self::allow_http)).
    /// Todo provedor chama antes de mandar e a cada redirecionamento.
    pub fn check_url(&self, url: &str) -> Result<(), String> {
        check_scheme(url, "https", "http", self.allow_http)
    }
}

/// O esquema seguro (`https`/`wss`) sempre; o inseguro (`http`/`ws`) só para este computador
/// ou com `allow_insecure`.
pub fn check_scheme(url: &str, secure: &str, insecure: &str, allow_insecure: bool) -> Result<(), String> {
    let lower = url.trim().to_ascii_lowercase();
    let (scheme, rest) = lower.split_once("://").ok_or_else(|| format!("endereço sem esquema: '{url}'"))?;
    if scheme == secure {
        return Ok(());
    }
    if scheme != insecure {
        return Err(format!("esquema não aceito: '{scheme}' (use {secure}://)"));
    }
    if allow_insecure || is_loopback_host(url_host(rest)) {
        return Ok(());
    }
    Err(format!("{insecure}:// sem TLS só para este computador; use {secure}:// (ou ligue o allow_http/allow_insecure do componente)"))
}

/// O host de `rest` (o endereço sem o `esquema://`): sem usuário, porta e caminho.
pub fn url_host(rest: &str) -> &str {
    let rest = rest.split_once("://").map_or(rest, |(_, r)| r);
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let host = authority.rsplit('@').next().unwrap_or_default();
    if let Some(v6) = host.strip_prefix('[') {
        return v6.split(']').next().unwrap_or_default();
    }
    host.split(':').next().unwrap_or_default()
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "localhost" | "::1") || host.starts_with("127.")
}

/// A resposta: o status, os cabeçalhos (nomes em minúsculas), o corpo e o endereço final
/// (depois dos redirecionamentos).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub url: String,
}

impl HttpResponse {
    /// Status 2xx.
    pub fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// O valor de um cabeçalho da resposta.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }

    /// O corpo como texto (UTF-8; o que não for vira `�`).
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

/// Onde a resposta volta, de qualquer thread (o componente leva para a thread da tela).
pub type HttpDone = Box<dyn FnOnce(Result<HttpResponse, String>) + Send>;

/// Quem faz os pedidos HTTP (o `ureq` no desktop e no celular, o `fetch` no navegador). Erro
/// é só falha de transporte (endereço, TLS, prazo, limite); um 404 é resposta.
pub trait HttpProvider {
    /// Manda `request` sem travar a tela; `done` recebe a resposta, uma vez só.
    fn send(&self, request: HttpRequest, done: HttpDone);
}

/// Um pedido de conexão WebSocket.
#[derive(Clone, Debug, PartialEq)]
pub struct WebSocketRequest {
    pub url: String,
    /// Os subprotocolos oferecidos (`Sec-WebSocket-Protocol`).
    pub protocols: Vec<String>,
    /// Cabeçalhos do pedido de abertura (no navegador não valem: o `WebSocket` não deixa).
    pub headers: Vec<(String, String)>,
    /// O prazo para abrir, em milissegundos; 0: sem prazo.
    pub connect_timeout_ms: u32,
    /// A maior mensagem aceita, em bytes (acima disso, a conexão fecha com 1009).
    pub max_message_bytes: u64,
    /// Aceita `ws://` (sem TLS) para fora deste computador.
    pub allow_insecure: bool,
}

impl Default for WebSocketRequest {
    fn default() -> Self {
        Self { url: String::new(), protocols: Vec::new(), headers: Vec::new(), connect_timeout_ms: 10_000, max_message_bytes: HTTP_MAX_RESPONSE, allow_insecure: false }
    }
}

impl WebSocketRequest {
    pub fn check_url(&self) -> Result<(), String> {
        check_scheme(&self.url, "wss", "ws", self.allow_insecure)
    }

    /// O limite das mensagens valendo (0 vira o padrão).
    pub fn message_limit(&self) -> u64 {
        if self.max_message_bytes == 0 { HTTP_MAX_RESPONSE } else { self.max_message_bytes }
    }
}

/// O que acontece numa conexão WebSocket.
#[derive(Clone, Debug, PartialEq)]
pub enum WebSocketEvent {
    /// Abriu (com o subprotocolo que o servidor escolheu, vazio se nenhum).
    Open { protocol: String },
    Text(String),
    Binary(Vec<u8>),
    /// Uma falha (não abriu, caiu); o `Closed` vem depois.
    Error(String),
    /// Fechou (o código e o motivo; 1006 quando caiu sem o fechamento do protocolo). É sempre
    /// o último evento da conexão.
    Closed { code: u16, reason: String },
}

/// Onde os eventos chegam, de qualquer thread.
pub type WebSocketEvents = Box<dyn FnMut(WebSocketEvent) + Send>;

/// Uma conexão aberta por um [`WebSocketProvider`]: mandar e fechar, da thread da tela.
pub trait WebSocketConnection {
    fn send_text(&mut self, text: &str) -> Result<(), String>;
    fn send_binary(&mut self, data: &[u8]) -> Result<(), String>;
    /// Fecha (o `Closed` chega pelos eventos).
    fn close(&mut self, code: u16, reason: &str);
}

/// Quem abre conexões WebSocket (o `tungstenite` no desktop e no celular, o `WebSocket` do
/// navegador na web).
pub trait WebSocketProvider {
    /// Começa a abrir (sem travar): o `Open`, ou o `Error` e o `Closed`, chegam por `events`.
    fn connect(&self, request: WebSocketRequest, events: WebSocketEvents) -> Box<dyn WebSocketConnection>;
}
