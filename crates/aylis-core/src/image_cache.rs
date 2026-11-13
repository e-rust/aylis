//! O cache de imagens dos renderizadores: o que já foi feito de uma [`Image`] (decodificada,
//! enviada à placa de vídeo) para não refazer a cada pintura.
//!
//! Uma entrada sai quando o app solta a imagem (ninguém mais pede aquele id) ou, passando dos
//! [limites](ImageCacheLimits), a usada há mais tempo. Os limites valem para o processo todo
//! e vêm do `image_cache` do `.aylisproj`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

use crate::image::{Image, WeakImage};

/// Quanto os renderizadores guardam de imagens prontas que o app ainda segura. Passar do
/// limite não erra nada na tela: a imagem que sai é refeita (decodificada ou reenviada à
/// placa) quando aparecer de novo, o que custa tempo de pintura.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageCacheLimits {
    /// Megabytes de imagens em pixels (quadros de vídeo, prévias, câmera, os blocos de um
    /// desenho) guardadas na placa. Precisa caber o que fica na tela ao mesmo tempo; um
    /// desenho 4K inteiro visível ocupa ~33 MB.
    pub pixels_mb: u32,
    /// Quantas imagens de arquivo (PNG, JPEG, SVG...) ficam guardadas, decodificadas e na
    /// placa. Precisa caber as que aparecem juntas numa tela (ícones, miniaturas, uma galeria).
    pub files: u32,
    /// Megabytes das camadas guardadas ([`LayerCache`]): o desenho de um controle com
    /// `cache_layer`, ou das páginas numa transição, guardado como imagem para andar e
    /// esmaecer sem repintar os filhos. Precisa caber as que andam juntas (uma transição de
    /// aba em 4K são duas de ~33 MB). `0`: nenhuma fica guardada (tudo é repintado a cada
    /// quadro).
    pub layers_mb: u32,
}

impl ImageCacheLimits {
    pub const DEFAULT: Self = Self { pixels_mb: 96, files: 256, layers_mb: 128 };
    /// Os menores e maiores valores aceitos: abaixo, uma tela comum já refaria imagens a cada
    /// pintura; acima, o cache poderia tomar a memória da placa.
    pub const PIXELS_MB_RANGE: (u32, u32) = (8, 4096);
    pub const FILES_RANGE: (u32, u32) = (16, 65_536);
    pub const LAYERS_MB_RANGE: (u32, u32) = (0, 4096);

    /// Os limites dentro das faixas aceitas.
    pub fn clamped(self) -> Self {
        Self {
            pixels_mb: self.pixels_mb.clamp(Self::PIXELS_MB_RANGE.0, Self::PIXELS_MB_RANGE.1),
            files: self.files.clamp(Self::FILES_RANGE.0, Self::FILES_RANGE.1),
            layers_mb: self.layers_mb.clamp(Self::LAYERS_MB_RANGE.0, Self::LAYERS_MB_RANGE.1),
        }
    }

    /// Os bytes de [`layers_mb`](Self::layers_mb).
    pub fn layers_bytes(self) -> usize {
        self.layers_mb as usize * (1 << 20)
    }

    /// Os bytes de [`pixels_mb`](Self::pixels_mb).
    pub fn pixels_bytes(self) -> usize {
        self.pixels_mb as usize * (1 << 20)
    }
}

impl Default for ImageCacheLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

static PIXELS_MB: AtomicU32 = AtomicU32::new(ImageCacheLimits::DEFAULT.pixels_mb);
static FILES: AtomicU32 = AtomicU32::new(ImageCacheLimits::DEFAULT.files);
static LAYERS_MB: AtomicU32 = AtomicU32::new(ImageCacheLimits::DEFAULT.layers_mb);
static AUTO_LAYERS: AtomicBool = AtomicBool::new(true);

/// Os limites em vigor.
pub fn limits() -> ImageCacheLimits {
    ImageCacheLimits { pixels_mb: PIXELS_MB.load(Ordering::Relaxed), files: FILES.load(Ordering::Relaxed), layers_mb: LAYERS_MB.load(Ordering::Relaxed) }
}

/// Troca os limites (o projeto faz isso ao abrir); valores fora da faixa são ajustados.
pub fn set_limits(limits: ImageCacheLimits) {
    let l = limits.clamped();
    PIXELS_MB.store(l.pixels_mb, Ordering::Relaxed);
    FILES.store(l.files, Ordering::Relaxed);
    LAYERS_MB.store(l.layers_mb, Ordering::Relaxed);
}

/// As camadas automáticas: as páginas numa transição e o contêiner que recorta os filhos com
/// o lugar ou a transparência animando vão para uma camada guardada sem o app pedir (o
/// `cache_layer` pedido no controle vale sempre). É o padrão do projeto (`auto_layers` do
/// `image_cache`); cada form pode trocar pela propriedade `layer_cache` ([`LayerCaching`]).
pub fn auto_layers() -> bool {
    AUTO_LAYERS.load(Ordering::Relaxed)
}

/// Veja [`auto_layers`]; o projeto faz isso ao abrir.
pub fn set_auto_layers(on: bool) {
    AUTO_LAYERS.store(on, Ordering::Relaxed);
}

/// A propriedade `layer_cache` do form: se as camadas automáticas valem nele.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayerCaching {
    /// O padrão do projeto ([`auto_layers`]).
    #[default]
    Project,
    On,
    Off,
}

impl LayerCaching {
    pub const ALL: [LayerCaching; 3] = [LayerCaching::Project, LayerCaching::On, LayerCaching::Off];

    /// Ligadas ou não, com o padrão do projeto no lugar de `Project`.
    pub fn enabled(self) -> bool {
        match self {
            LayerCaching::Project => auto_layers(),
            LayerCaching::On => true,
            LayerCaching::Off => false,
        }
    }
}

impl crate::prop::PropEnum for LayerCaching {
    const NAMES: &'static [&'static str] = &["project", "on", "off"];

    fn from_index(index: usize) -> Option<Self> {
        LayerCaching::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

struct Layer<T> {
    owner: u64,
    value: T,
    bytes: usize,
    used: u64,
}

/// As camadas guardadas de um renderizador ([`Canvas::begin_cached`](crate::Canvas::begin_cached)),
/// pela chave completa (a do núcleo com o tamanho e a escala em pixels). Cada uma lembra a
/// chave que o núcleo passou (`owner`), para o núcleo soltar a de um controle quando ela deixa
/// de valer ([`forget`](Self::forget)). Passando de [`ImageCacheLimits::layers_mb`], sai a
/// usada há mais tempo; uma camada maior que o limite inteiro nem é montada
/// ([`accepts`](Self::accepts)).
pub struct LayerCache<T> {
    entries: HashMap<u64, Layer<T>>,
    clock: u64,
    bytes: usize,
    counters: Option<Arc<Counters>>,
}

impl<T> Default for LayerCache<T> {
    fn default() -> Self {
        Self { entries: HashMap::new(), clock: 0, bytes: 0, counters: None }
    }
}

impl<T> LayerCache<T> {
    /// Um cache que aparece no painel de memória ([`stats`]) com este nome.
    pub fn named(name: &'static str) -> Self {
        let counters = Arc::new(Counters { name, images: AtomicUsize::new(0), bytes: AtomicUsize::new(0) });
        REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).push(Arc::downgrade(&counters));
        Self { counters: Some(counters), ..Self::default() }
    }

    fn publish(&self) {
        if let Some(c) = &self.counters {
            c.images.store(self.entries.len(), Ordering::Relaxed);
            c.bytes.store(self.bytes, Ordering::Relaxed);
        }
    }

    /// Se vale montar uma camada de `bytes`: com o limite em zero, ou menor que ela, não (quem
    /// chama desenha direto, sem a imagem fora da tela).
    pub fn accepts(bytes: usize) -> bool {
        bytes > 0 && bytes <= limits().layers_bytes()
    }

    /// A camada pela chave completa, marcada como usada agora.
    pub fn get(&mut self, key: u64) -> Option<&T> {
        self.clock += 1;
        let clock = self.clock;
        self.entries.get_mut(&key).map(|e| {
            e.used = clock;
            &e.value
        })
    }

    /// Guarda a camada; passando do limite, tira as usadas há mais tempo (nunca a que entrou).
    pub fn insert(&mut self, owner: u64, key: u64, value: T, bytes: usize) {
        let max = limits().layers_bytes();
        if bytes > max {
            return;
        }
        self.clock += 1;
        if let Some(old) = self.entries.insert(key, Layer { owner, value, bytes, used: self.clock }) {
            self.bytes -= old.bytes;
        }
        self.bytes += bytes;
        while self.bytes > max {
            let Some(oldest) = self.entries.iter().filter(|(k, _)| **k != key).min_by_key(|(_, e)| e.used).map(|(k, _)| *k) else { break };
            if let Some(e) = self.entries.remove(&oldest) {
                self.bytes -= e.bytes;
            }
        }
        self.publish();
    }

    /// Solta as camadas montadas com a chave `owner` do núcleo (a camada deixou de valer: a
    /// transição acabou, o desenho de dentro mudou).
    pub fn forget(&mut self, owner: u64) {
        let mut freed = 0;
        self.entries.retain(|_, e| {
            let keep = e.owner != owner;
            if !keep {
                freed += e.bytes;
            }
            keep
        });
        if freed > 0 {
            self.bytes -= freed;
            self.publish();
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
        self.publish();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// A soma dos bytes guardados.
    pub fn bytes(&self) -> usize {
        self.bytes
    }
}

struct Entry<T> {
    image: WeakImage,
    value: T,
    bytes: usize,
    used: u64,
}

/// Quanto um cache com nome guarda agora, para o painel de memória ([`stats`]).
struct Counters {
    name: &'static str,
    images: AtomicUsize,
    bytes: AtomicUsize,
}

/// Os contadores dos caches com nome que existem (os que sumiram ficam mortos e saem).
static REGISTRY: Mutex<Vec<Weak<Counters>>> = Mutex::new(Vec::new());

/// Um cache no painel de memória: o nome, quantas imagens e quantos bytes guarda.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CacheStat {
    pub name: &'static str,
    pub images: usize,
    pub bytes: usize,
}

/// O que cada cache com nome guarda agora (os de mesmo nome somados, como os de duas
/// janelas), na ordem em que apareceram. Para o painel de memória do app em depuração e o
/// Profiler da IDE.
pub fn stats() -> Vec<CacheStat> {
    let mut registry = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    registry.retain(|w| w.strong_count() > 0);
    let mut out: Vec<CacheStat> = Vec::new();
    for c in registry.iter().filter_map(Weak::upgrade) {
        let (images, bytes) = (c.images.load(Ordering::Relaxed), c.bytes.load(Ordering::Relaxed));
        match out.iter_mut().find(|s| s.name == c.name) {
            Some(s) => {
                s.images += images;
                s.bytes += bytes;
            }
            None => out.push(CacheStat { name: c.name, images, bytes }),
        }
    }
    out
}

/// O que um renderizador fez de cada imagem, pelo [`Image::id`], com a imagem fraca ao lado.
pub struct ImageCache<T> {
    entries: HashMap<u64, Entry<T>>,
    clock: u64,
    bytes: usize,
    counters: Option<Arc<Counters>>,
}

impl<T> Default for ImageCache<T> {
    fn default() -> Self {
        Self { entries: HashMap::new(), clock: 0, bytes: 0, counters: None }
    }
}

impl<T> ImageCache<T> {
    /// Um cache que aparece no painel de memória ([`stats`]) com este nome ("Direct2D ·
    /// pixels na placa").
    pub fn named(name: &'static str) -> Self {
        let counters = Arc::new(Counters { name, images: AtomicUsize::new(0), bytes: AtomicUsize::new(0) });
        REGISTRY.lock().unwrap_or_else(|e| e.into_inner()).push(Arc::downgrade(&counters));
        Self { counters: Some(counters), ..Self::default() }
    }

    /// Atualiza os contadores do painel depois de uma mudança.
    fn publish(&self) {
        if let Some(c) = &self.counters {
            c.images.store(self.entries.len(), Ordering::Relaxed);
            c.bytes.store(self.bytes, Ordering::Relaxed);
        }
    }

    /// O valor guardado da imagem, marcado como usado agora.
    pub fn get(&mut self, image: &Image) -> Option<&T> {
        self.clock += 1;
        let clock = self.clock;
        self.entries.get_mut(&image.id()).map(|e| {
            e.used = clock;
            &e.value
        })
    }

    /// O valor pelo id, sem marcar uso (quem termina de carregar depois, como o navegador).
    pub fn get_by_id_mut(&mut self, id: u64) -> Option<&mut T> {
        self.entries.get_mut(&id).map(|e| &mut e.value)
    }

    /// Guarda o valor da imagem (`bytes`: quanto ocupa, para o limite em bytes). Antes, solta
    /// o das imagens mortas; depois, enquanto passar de `max_count` entradas ou `max_bytes`,
    /// tira a usada há mais tempo (nunca a que acabou de entrar).
    pub fn insert(&mut self, image: &Image, value: T, bytes: usize, max_count: usize, max_bytes: usize) {
        self.forget_dropped();
        self.clock += 1;
        if let Some(old) = self.entries.insert(image.id(), Entry { image: image.downgrade(), value, bytes, used: self.clock }) {
            self.bytes -= old.bytes;
        }
        self.bytes += bytes;
        while self.entries.len() > 1 && (self.entries.len() > max_count || self.bytes > max_bytes) {
            let Some(oldest) = self.entries.iter().filter(|(id, _)| **id != image.id()).min_by_key(|(_, e)| e.used).map(|(id, _)| *id) else { break };
            if let Some(e) = self.entries.remove(&oldest) {
                self.bytes -= e.bytes;
            }
        }
        self.publish();
    }

    /// Solta o que é de imagens que o app já soltou.
    pub fn forget_dropped(&mut self) {
        let mut freed = 0;
        self.entries.retain(|_, e| {
            let alive = e.image.is_alive();
            if !alive {
                freed += e.bytes;
            }
            alive
        });
        self.bytes -= freed;
        self.publish();
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
        self.publish();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// A soma dos `bytes` guardados.
    pub fn bytes(&self) -> usize {
        self.bytes
    }
}
