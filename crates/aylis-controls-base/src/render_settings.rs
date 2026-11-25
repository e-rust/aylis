//! As configurações de pintura do app inteiro, para ler e trocar em execução (decidido em
//! 14/11: o projeto configura o app, a propriedade do form configura o form, e em execução
//! as configurações podem ser trocadas).

use aylis_core::gpu::BlendCompositor;
use aylis_core::image_cache::{self, FRAME_REUSE_KB_RANGE};
use aylis_core::{Component, EventCtx, ImageCacheLimits, properties};

/// Os valores que o [`RenderSettings`] mostra e troca.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Values {
    limits: ImageCacheLimits,
    auto_layers: bool,
    frame_reuse_kb: u32,
    blend_compositor: BlendCompositor,
}

impl Values {
    /// Os que valem agora no processo.
    fn current() -> Self {
        Self {
            limits: image_cache::limits(),
            auto_layers: image_cache::auto_layers(),
            frame_reuse_kb: image_cache::frame_reuse_kb(),
            blend_compositor: aylis_core::gpu::blend_compositor(),
        }
    }

    /// Passa a valer no app inteiro (os limites, ajustados à faixa).
    fn apply(self) {
        image_cache::set_limits(self.limits);
        image_cache::set_auto_layers(self.auto_layers);
        image_cache::set_frame_reuse_kb(self.frame_reuse_kb);
        aylis_core::gpu::set_blend_compositor(self.blend_compositor);
    }
}

/// Um número de propriedade dentro de `range` (fora, ajustado).
fn clamp(value: i64, (min, max): (u32, u32)) -> u32 {
    value.clamp(min as i64, max as i64) as u32
}

/// Lê e troca, em execução, as configurações de pintura que valem no app inteiro: as mesmas
/// do `image_cache` do `.aylisproj` (Opções do projeto → Memória), que o projeto aplica ao
/// abrir. São três níveis: o projeto configura o app (uma vez, no `.aylisproj`); a
/// propriedade do form configura aquele form (`layer_cache`, `frame_reuse_kb`); este
/// componente troca o valor do app enquanto ele roda (uma tela de "desempenho" do usuário,
/// um modo econômico). Variável de ambiente só no desenvolvimento.
///
/// - `pixels_mb`, `files`, `layers_mb`: os limites do cache de imagens
///   ([`ImageCacheLimits`]); baixar solta o excedente no próximo uso (a próxima imagem
///   guardada; a camada, na próxima pintura);
/// - `auto_layers`: as camadas automáticas nos forms com `layer_cache = project`; trocar
///   repinta;
/// - `frame_reuse_kb`: até quantos KB um quadro de vídeo reaproveita o bitmap do anterior
///   (Direct2D), nos forms sem `frame_reuse_kb` próprio; vale no quadro seguinte;
/// - `blend_compositor`: quem junta as camadas com `blend_mode` ao fundo no Direct2D
///   (`direct2d` ou `direct3d`), nos forms sem `blend_compositor` próprio; trocar repinta.
///
/// Valores fora da faixa são ajustados a ela, como o projeto faz ao abrir
/// ([`ImageCacheLimits::clamped`], [`image_cache::set_frame_reuse_kb`]); ler de volta diz o
/// que ficou.
///
/// As propriedades são computadas sobre os valores do processo e **não são gravadas no
/// form**: um form que gravasse os valores passaria por cima do projeto a cada vez que
/// abrisse, e a configuração de uma vez é a do projeto. No designer, trocar não mexe na IDE:
/// o valor fica só no componente, e o que aparece é o padrão (a IDE não aplica o
/// `image_cache` do projeto aberto a si mesma).
#[derive(Default)]
pub struct RenderSettings {
    /// Um número livre para o código (não muda nada).
    pub tag: i64,
    /// O que foi trocado no designer (lá não se mexe nos valores do processo, que são da IDE).
    design: Option<Values>,
    /// A troca de agora, até o `property_changed` decidir se aplica (fora do designer) ou
    /// guarda (no designer).
    staged: Option<Values>,
}

impl RenderSettings {
    fn values(&self) -> Values {
        self.staged.or(self.design).unwrap_or_else(Values::current)
    }

    fn change(&mut self, f: impl FnOnce(&mut Values)) {
        let mut v = self.values();
        f(&mut v);
        self.staged = Some(v);
    }

    /// Megabytes de imagens em pixels guardadas na placa ([`ImageCacheLimits::pixels_mb`]).
    fn pixels_mb(&self) -> i64 {
        self.values().limits.pixels_mb as i64
    }

    fn set_pixels_mb(&mut self, mb: i64) {
        self.change(|v| v.limits.pixels_mb = clamp(mb, ImageCacheLimits::PIXELS_MB_RANGE));
    }

    /// Quantas imagens de arquivo ficam guardadas ([`ImageCacheLimits::files`]).
    fn files(&self) -> i64 {
        self.values().limits.files as i64
    }

    fn set_files(&mut self, files: i64) {
        self.change(|v| v.limits.files = clamp(files, ImageCacheLimits::FILES_RANGE));
    }

    /// Megabytes das camadas guardadas ([`ImageCacheLimits::layers_mb`]; `0`: nenhuma).
    fn layers_mb(&self) -> i64 {
        self.values().limits.layers_mb as i64
    }

    fn set_layers_mb(&mut self, mb: i64) {
        self.change(|v| v.limits.layers_mb = clamp(mb, ImageCacheLimits::LAYERS_MB_RANGE));
    }

    /// As camadas automáticas nos forms com `layer_cache = project`.
    fn auto_layers(&self) -> bool {
        self.values().auto_layers
    }

    fn set_auto_layers(&mut self, on: bool) {
        self.change(|v| v.auto_layers = on);
    }

    /// Até quantos KB um quadro de vídeo reaproveita o bitmap do anterior (`0`: nunca).
    fn frame_reuse_kb(&self) -> i64 {
        self.values().frame_reuse_kb as i64
    }

    fn set_frame_reuse_kb(&mut self, kb: i64) {
        self.change(|v| v.frame_reuse_kb = clamp(kb, FRAME_REUSE_KB_RANGE));
    }

    /// Quem junta as camadas com `blend_mode` ao fundo no Direct2D, nos forms sem
    /// `blend_compositor` próprio.
    fn blend_compositor(&self) -> BlendCompositor {
        self.values().blend_compositor
    }

    fn set_blend_compositor(&mut self, compositor: BlendCompositor) {
        self.change(|v| v.blend_compositor = compositor);
    }
}

impl Component for RenderSettings {
    fn class_name(&self) -> &'static str {
        "RenderSettings"
    }

    properties! {
        tag: i64 => "Misc";
        computed: pixels_mb: i64 => pixels_mb / set_pixels_mb,
            files: i64 => files / set_files,
            layers_mb: i64 => layers_mb / set_layers_mb,
            auto_layers: bool => auto_layers / set_auto_layers,
            frame_reuse_kb: i64 => frame_reuse_kb / set_frame_reuse_kb,
            blend_compositor: BlendCompositor => blend_compositor / set_blend_compositor
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, _name: &str) {
        let Some(v) = self.staged.take() else { return };
        if ctx.ui.design_mode() {
            self.design = Some(v);
            return;
        }
        let before = Values::current();
        v.apply();
        // As camadas automáticas mudam a pintura de todos os forms: repinta tudo. Os limites
        // valem no próximo uso, e o reaproveitamento dos quadros, no próximo quadro de vídeo.
        if before.auto_layers != v.auto_layers || before.limits != v.limits || before.blend_compositor != v.blend_compositor {
            ctx.ui.request_repaint();
        }
    }
}
