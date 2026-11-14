//! Interface de desenho (o "TCanvas"). O core só conhece este trait; a implementação
//! real (Skia na CPU ou na GPU) fica no crate de renderização.

use crate::color::Color;
use crate::geometry::{PointF, RectF, SizeF};
use crate::image::Image;
use crate::path::Path;

/// Uma parada de cor num gradiente (`offset` de 0 a 1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    pub offset: f32,
    pub color: Color,
}

impl GradientStop {
    pub const fn new(offset: f32, color: Color) -> Self {
        Self { offset, color }
    }
}

/// Como preencher uma forma.
#[derive(Clone, Debug, PartialEq)]
pub enum Brush {
    Solid(Color),
    Linear { start: PointF, end: PointF, stops: Vec<GradientStop> },
    Radial { center: PointF, radius: f32, stops: Vec<GradientStop> },
    /// Gradiente cônico (ótimo para bordas animadas girando).
    Sweep { center: PointF, start_angle: f32, stops: Vec<GradientStop> },
}

impl From<Color> for Brush {
    fn from(c: Color) -> Self {
        Brush::Solid(c)
    }
}

/// Como uma camada se junta ao que está por baixo dela (o `mix-blend-mode` do CSS e do
/// Photoshop). Só os modos que o Skia, o Direct2D e o Canvas 2D do navegador desenham igual.
/// `Difference` com branco é o negativo (a marca d'água que inverte o vídeo por baixo).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// Por cima, sem mistura.
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    /// A diferença entre as cores: o branco inverte, o preto não muda.
    Difference,
    /// Como a diferença, com menos contraste.
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    pub const ALL: [BlendMode; 16] = [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Overlay,
        BlendMode::Darken,
        BlendMode::Lighten,
        BlendMode::ColorDodge,
        BlendMode::ColorBurn,
        BlendMode::HardLight,
        BlendMode::SoftLight,
        BlendMode::Difference,
        BlendMode::Exclusion,
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity,
    ];

    /// O nome no CSS (`globalCompositeOperation` do Canvas 2D): `color-dodge`...
    pub fn css(self) -> &'static str {
        match self {
            BlendMode::Normal => "source-over",
            BlendMode::Multiply => "multiply",
            BlendMode::Screen => "screen",
            BlendMode::Overlay => "overlay",
            BlendMode::Darken => "darken",
            BlendMode::Lighten => "lighten",
            BlendMode::ColorDodge => "color-dodge",
            BlendMode::ColorBurn => "color-burn",
            BlendMode::HardLight => "hard-light",
            BlendMode::SoftLight => "soft-light",
            BlendMode::Difference => "difference",
            BlendMode::Exclusion => "exclusion",
            BlendMode::Hue => "hue",
            BlendMode::Saturation => "saturation",
            BlendMode::Color => "color",
            BlendMode::Luminosity => "luminosity",
        }
    }
}

impl crate::prop::PropEnum for BlendMode {
    const NAMES: &'static [&'static str] = &[
        "normal",
        "multiply",
        "screen",
        "overlay",
        "darken",
        "lighten",
        "color_dodge",
        "color_burn",
        "hard_light",
        "soft_light",
        "difference",
        "exclusion",
        "hue",
        "saturation",
        "color",
        "luminosity",
    ];
    fn from_index(i: usize) -> Option<Self> {
        BlendMode::ALL.get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Ponta das linhas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// Junção entre segmentos.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineJoin {
    #[default]
    Miter,
    Round,
    Bevel,
}

/// Como contornar uma forma.
#[derive(Clone, Debug, PartialEq)]
pub struct Stroke {
    pub brush: Brush,
    pub width: f32,
    /// Traço e espaço alternados (`[4.0, 2.0]`); `None` = linha contínua.
    pub dash: Option<Vec<f32>>,
    pub cap: LineCap,
    pub join: LineJoin,
}

impl Stroke {
    pub fn new(brush: impl Into<Brush>, width: f32) -> Self {
        Self { brush: brush.into(), width, dash: None, cap: LineCap::Butt, join: LineJoin::Miter }
    }

    pub fn dashed(mut self, pattern: Vec<f32>) -> Self {
        self.dash = Some(pattern);
        self
    }

    /// Pontas e junções arredondadas (o traço dos ícones).
    pub fn round(mut self) -> Self {
        self.cap = LineCap::Round;
        self.join = LineJoin::Round;
        self
    }
}

/// Sombra projetada.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shadow {
    pub color: Color,
    /// Desfoque (raio, em unidades lógicas).
    pub blur: f32,
    pub offset: PointF,
    /// Quanto a sombra cresce além da forma (negativo encolhe).
    pub spread: f32,
}

impl Shadow {
    pub fn new(color: Color, blur: f32, dx: f32, dy: f32) -> Self {
        Self { color, blur, offset: PointF::new(dx, dy), spread: 0.0 }
    }

    /// Até onde a sombra passa da forma: o desfoque (os renderizadores usam sigma = metade
    /// do raio, e o desfoque some em 3 sigmas), o espalhamento e o deslocamento.
    pub fn reach(&self) -> f32 {
        self.blur * 1.5 + self.spread.max(0.0) + self.offset.x.abs().max(self.offset.y.abs())
    }

    /// Lê `"dx dy desfoque [espalhamento] [#cor]"`, como o `box-shadow` do CSS (aceita `px`).
    /// Sem cor, fica preto a 25%.
    pub fn parse(spec: &str) -> Option<Shadow> {
        let mut numbers = Vec::new();
        let mut color = None;
        for token in spec.split(|c: char| c.is_whitespace() || c == ',').filter(|t| !t.is_empty()) {
            let number = token.strip_suffix("px").unwrap_or(token);
            if let Ok(v) = number.parse::<f32>() {
                numbers.push(v);
            } else if color.is_none() {
                color = Some(token.parse::<Color>().ok()?);
            } else {
                return None;
            }
        }
        if !(2..=4).contains(&numbers.len()) {
            return None;
        }
        Some(Shadow {
            color: color.unwrap_or(Color::BLACK.with_opacity(0.25)),
            blur: numbers.get(2).copied().unwrap_or(0.0).max(0.0),
            offset: PointF::new(numbers[0], numbers[1]),
            spread: numbers.get(3).copied().unwrap_or(0.0),
        })
    }

    /// O texto que [`Shadow::parse`] lê (o espalhamento só aparece quando não é zero).
    pub fn to_spec(&self) -> String {
        let n = |v: f32| format!("{}", (v * 100.0).round() / 100.0);
        let mut s = format!("{} {} {}", n(self.offset.x), n(self.offset.y), n(self.blur));
        if self.spread != 0.0 {
            s += &format!(" {}", n(self.spread));
        }
        s + " " + &self.color.to_hex()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    #[default]
    Normal,
    Medium,
    SemiBold,
    Bold,
    Black,
}

/// Fonte. `family` vazio = fonte padrão do sistema.
#[derive(Clone, Debug, PartialEq)]
pub struct FontSpec {
    pub family: String,
    pub size: f32,
    pub weight: FontWeight,
    pub italic: bool,
    pub underline: bool,
    /// Riscado no meio.
    pub strikeout: bool,
}

impl Default for FontSpec {
    fn default() -> Self {
        Self { family: String::new(), size: 14.0, weight: FontWeight::Normal, italic: false, underline: false, strikeout: false }
    }
}

impl FontSpec {
    pub fn new(family: impl Into<String>, size: f32) -> Self {
        Self { family: family.into(), size, ..Default::default() }
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
}

/// Alinhamento do texto dentro do retângulo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    /// Esquerda (ou topo).
    #[default]
    Leading,
    Center,
    /// Direita (ou base).
    Trailing,
    /// Justificado: as linhas quebradas ocupam a largura
    /// toda, com o espaço que sobra dividido entre as palavras; a última linha de cada
    /// parágrafo (e o texto de uma linha só) fica à esquerda. Na vertical, vale o topo.
    Justify,
}

impl crate::prop::PropEnum for TextAlign {
    const NAMES: &'static [&'static str] = &["leading", "center", "trailing", "justify"];

    fn from_index(index: usize) -> Option<Self> {
        [TextAlign::Leading, TextAlign::Center, TextAlign::Trailing, TextAlign::Justify].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font: FontSpec,
    pub color: Color,
    pub h_align: TextAlign,
    pub v_align: TextAlign,
    /// Quebra automática de linha (nas palavras; palavra maior que a largura, no caractere).
    pub wrap: bool,
    /// Reticências quando não couber (por caractere, por palavra ou nenhuma).
    pub trimming: crate::text::Trimming,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font: FontSpec::default(),
            color: Color::BLACK,
            h_align: TextAlign::Leading,
            v_align: TextAlign::Center,
            wrap: false,
            trimming: crate::text::Trimming::Character,
        }
    }
}

/// Superfície de desenho. Coordenadas em unidades lógicas, relativas à transformação
/// atual (o `Ui` já posiciona o canvas na origem de cada controle antes de pintá-lo).
pub trait Canvas {
    /// Guarda transformação, clip e camada atuais.
    fn save(&mut self);
    /// Volta ao último `save` (ou `save_layer`).
    fn restore(&mut self);
    fn translate(&mut self, dx: f32, dy: f32);
    fn scale(&mut self, sx: f32, sy: f32);
    fn rotate(&mut self, degrees: f32);
    fn clip_rect(&mut self, rect: RectF);
    /// Abre uma camada com opacidade; tudo até o `restore` correspondente é composto
    /// junto com essa opacidade.
    fn save_layer(&mut self, opacity: f32);

    /// Como [`save_layer`](Self::save_layer), e no `restore` a camada se junta ao que está
    /// por baixo pelo modo `mode`. `bounds`: até onde a camada desenha (nas coordenadas de
    /// agora), para o renderizador misturar só ali. O padrão (um canvas sem mistura) é a
    /// camada normal.
    fn save_layer_blend(&mut self, opacity: f32, mode: BlendMode, bounds: RectF) {
        let _ = (mode, bounds);
        self.save_layer(opacity);
    }

    /// Preenche um retângulo (`radius` > 0 arredonda os cantos).
    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush);
    /// Traça a borda de um retângulo **por dentro**: o traço inteiro cabe em `rect` (o
    /// `box-sizing: border-box` do CSS), nos três renderizadores. Uma moldura de 1 px fica no
    /// primeiro pixel de dentro, nítida; para traçar pelo meio da linha, aumente o retângulo
    /// em meia largura do traço (o relatório do Tabernáculo, T09).
    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &Stroke);
    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush);
    /// Traça a elipse por dentro de `rect`, como o [`stroke_rect`](Self::stroke_rect).
    fn stroke_ellipse(&mut self, rect: RectF, stroke: &Stroke);
    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke);
    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle);
    /// Tamanho do texto numa linha só (sem quebra), com a mesma fonte usada no desenho.
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF;

    /// Desenha a parte `src` da imagem (em pixels da imagem) no retângulo `dest`. O padrão
    /// não desenha nada (canvases de teste).
    fn draw_image(&mut self, image: &Image, src: RectF, dest: RectF, opacity: f32) {
        let _ = (image, src, dest, opacity);
    }

    /// Tamanho da imagem em pixels (decodifica se preciso). `None` se não der para ler.
    fn image_size(&mut self, image: &Image) -> Option<SizeF> {
        image.known_size()
    }

    /// Recorta para um retângulo de cantos arredondados (o padrão recorta sem arredondar).
    fn clip_round_rect(&mut self, rect: RectF, radius: f32) {
        let _ = radius;
        self.clip_rect(rect);
    }

    /// Preenche um caminho (regra "não zero"). O padrão não preenche (canvases de teste).
    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        let _ = (path, brush);
    }

    /// Recorta para o interior do caminho (`inverse`: para o exterior). O padrão recorta
    /// para o retângulo do caminho (e não recorta com `inverse`).
    fn clip_path(&mut self, path: &Path, inverse: bool) {
        if !inverse {
            self.clip_rect(path.bounds());
        }
    }

    /// Desfoca o que já foi desenhado embaixo de `rect` (cantos `radius`), com o desvio
    /// `blur` (o "material" do vidro: Liquid Glass, Mica, Acrylic). Devolve `false` se o
    /// canvas não sabe desfocar (quem chama usa um fundo mais opaco no lugar).
    fn backdrop_blur(&mut self, rect: RectF, radius: f32, blur: f32) -> bool {
        let _ = (rect, radius, blur);
        false
    }

    /// Máscara de alfa (o "track matte" do After Effects). `begin_masked` abre uma camada
    /// para o conteúdo; depois de desenhá-lo, `begin_mask` passa a desenhar a máscara (o
    /// alfa dela recorta o conteúdo; com `inverse`, o contrário) e `end_mask` compõe tudo.
    /// Devolve `false` se o canvas não sabe fazer (aí quem chama recorta com
    /// [`clip_path`](Canvas::clip_path)).
    fn begin_masked(&mut self, inverse: bool) -> bool {
        let _ = inverse;
        false
    }

    /// Veja [`begin_masked`](Canvas::begin_masked).
    fn begin_mask(&mut self) {}

    /// Veja [`begin_masked`](Canvas::begin_masked).
    fn end_mask(&mut self) {}

    /// Desenho caro e parado (um mapa com centenas de caminhos, um gráfico vetorial)
    /// guardado como imagem pela `key`: `true` quando a imagem guardada já foi desenhada em
    /// `rect` (quem chama não desenha nada). Com `false`, quem chama desenha em `rect` e
    /// chama [`end_cached`](Canvas::end_cached), que guarda o resultado e o compõe. A
    /// chave deve mudar com tudo o que muda o desenho (cores, destaque, área); o tamanho em
    /// pixels já entra nela. O padrão não guarda nada (desenha sempre).
    fn begin_cached(&mut self, key: u64, rect: RectF) -> bool {
        let _ = (key, rect);
        false
    }

    /// Veja [`begin_cached`](Canvas::begin_cached).
    fn end_cached(&mut self) {}

    /// Solta as imagens guardadas com a `key` de [`begin_cached`](Canvas::begin_cached): o
    /// núcleo chama quando a camada de um controle deixa de valer (a transição acabou, o
    /// desenho de dentro mudou), para ela não ocupar memória até sair pelo limite.
    fn forget_cached(&mut self, key: u64) {
        let _ = key;
    }

    /// Contorna um caminho. O padrão desenha as curvas como retas.
    fn stroke_path(&mut self, path: &Path, stroke: &Stroke) {
        for (points, closed) in path.flatten(8) {
            for w in points.windows(2) {
                self.draw_line(w[0], w[1], stroke);
            }
            if closed && points.len() > 2 {
                self.draw_line(points[points.len() - 1], points[0], stroke);
            }
        }
    }

    /// Sombra de um retângulo (de cantos arredondados). O padrão não desenha.
    fn draw_shadow(&mut self, rect: RectF, radius: f32, shadow: &Shadow) {
        let _ = (rect, radius, shadow);
    }

    /// Quantos pixels da tela há em uma unidade lógica, com a transformação atual (para
    /// quem desenha por fora, como o 3D, criar a imagem já no tamanho certo).
    fn device_scale(&self) -> f32 {
        1.0
    }

    /// A GPU em que este canvas desenha, para quem desenha por fora (o 3D) usar a mesma e
    /// entregar o resultado sem passar pela CPU. `None`: canvas na CPU.
    fn gpu_device(&self) -> Option<GpuDevice> {
        None
    }

    /// Compõe no retângulo `dest` uma imagem que já está na GPU (veja [`GpuSurface`]), sem
    /// copiar para a CPU. Devolve `false` se o canvas não sabe (quem chama copia os pixels
    /// e usa [`draw_image`](Canvas::draw_image)).
    fn draw_gpu_surface(&mut self, dest: RectF, surface: &GpuSurface) -> bool {
        let _ = (dest, surface);
        false
    }

    /// Quem desenha por fora (o 3D) mexeu no estado da GPU deste canvas (no OpenGL, o mesmo
    /// contexto): o canvas não pode confiar no que guardou. Vale também quando o resultado
    /// volta pela CPU, com [`draw_image`](Canvas::draw_image).
    fn gpu_state_changed(&mut self) {}
}

/// A GPU de um canvas (veja [`Canvas::gpu_device`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuDevice {
    /// Direct3D 11 (o Direct2D), no adaptador com este LUID.
    Direct3D11 { adapter_luid: i64 },
    /// OpenGL (o Skia na GPU). `loader` é o endereço de uma
    /// `unsafe extern "C" fn(*const c_char) -> *const c_void` que acha as funções do OpenGL
    /// pelo nome; `context` identifica o contexto (quem desenha por fora cria os recursos
    /// nele, e por isso precisa de um por contexto).
    OpenGl { loader: usize, context: usize },
    /// Navegador (Canvas 2D): quem desenha por fora usa um `<canvas>` próprio (WebGPU ou
    /// WebGL), que o canvas copia na GPU com `drawImage`.
    Web,
}

/// Imagem na GPU desenhada por fora do aylis (o 3D), para o renderizador compor com os
/// controles sem passar pela CPU (veja [`Canvas::draw_gpu_surface`]).
///
/// Os números são handles do sistema; o núcleo não os interpreta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSurface {
    /// Textura BGRA (alfa pré-multiplicado) compartilhada do Direct3D 12, com duas cercas
    /// compartilhadas: o renderizador espera `ready` chegar a `ready_value` antes de ler,
    /// e marca `release` com `release_value` quando o quadro que a leu terminar (aí quem
    /// desenha pode reescrever a textura).
    ///
    /// Vale também para uma textura do Direct3D 11 com handle NT e cercas `ID3D11Fence`
    /// compartilhadas (os quadros de vídeo do `aylis-media`): o renderizador abre as duas do
    /// mesmo jeito.
    D3D12 {
        /// Identifica a textura (o handle pode ser reaproveitado pelo sistema depois de fechado).
        id: u64,
        texture: isize,
        width: u32,
        height: u32,
        ready: isize,
        ready_value: u64,
        release: isize,
        release_value: u64,
    },
    /// Textura do OpenGL (RGBA, alfa pré-multiplicado), no mesmo contexto do canvas: os
    /// comandos já estão em ordem, sem cercas.
    GlTexture { id: u64, texture: u32, width: u32, height: u32 },
    /// Um `<canvas>` da página com `id="aylis-3d-<element>"`, já desenhado neste quadro.
    WebCanvas { element: u32, width: u32, height: u32 },
}

/// Medição de texto, usada fora da pintura (layout, auto-size).
pub trait TextService {
    /// Tamanho do texto; com `max_width`, considera a quebra de linha.
    fn measure(&self, text: &str, style: &TextStyle, max_width: Option<f32>) -> SizeF;
}

/// Sublinhado e tachado de uma linha de texto já desenhada (para os renderizadores que não
/// fazem sozinhos): `x` e `width` são os da linha, `top` e `line_height` a faixa dela.
pub fn decorate_line(canvas: &mut dyn Canvas, font: &FontSpec, color: Color, x: f32, width: f32, top: f32, line_height: f32) {
    if !(font.underline || font.strikeout) || width <= 0.0 {
        return;
    }
    let stroke = Stroke::new(color, (font.size / 14.0).max(1.0));
    let middle = top + line_height / 2.0;
    if font.underline {
        let y = middle + font.size * 0.45;
        canvas.draw_line(PointF::new(x, y), PointF::new(x + width, y), &stroke);
    }
    if font.strikeout {
        let y = middle + font.size * 0.05;
        canvas.draw_line(PointF::new(x, y), PointF::new(x + width, y), &stroke);
    }
}
