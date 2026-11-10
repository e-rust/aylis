//! O cache de imagens dos renderizadores: o que já foi feito de uma [`Image`] (decodificada,
//! enviada à placa de vídeo) para não refazer a cada pintura.
//!
//! Uma entrada sai quando o app solta a imagem (ninguém mais pede aquele id) ou, passando dos
//! [limites](ImageCacheLimits), a usada há mais tempo. Os limites valem para o processo todo
//! e vêm do `image_cache` do `.aylisproj`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
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
}

impl ImageCacheLimits {
    pub const DEFAULT: Self = Self { pixels_mb: 96, files: 256 };
    /// Os menores e maiores valores aceitos: abaixo, uma tela comum já refaria imagens a cada
    /// pintura; acima, o cache poderia tomar a memória da placa.
    pub const PIXELS_MB_RANGE: (u32, u32) = (8, 4096);
    pub const FILES_RANGE: (u32, u32) = (16, 65_536);

    /// Os limites dentro das faixas aceitas.
    pub fn clamped(self) -> Self {
        Self {
            pixels_mb: self.pixels_mb.clamp(Self::PIXELS_MB_RANGE.0, Self::PIXELS_MB_RANGE.1),
            files: self.files.clamp(Self::FILES_RANGE.0, Self::FILES_RANGE.1),
        }
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

/// Os limites em vigor.
pub fn limits() -> ImageCacheLimits {
    ImageCacheLimits { pixels_mb: PIXELS_MB.load(Ordering::Relaxed), files: FILES.load(Ordering::Relaxed) }
}

/// Troca os limites (o projeto faz isso ao abrir); valores fora da faixa são ajustados.
pub fn set_limits(limits: ImageCacheLimits) {
    let l = limits.clamped();
    PIXELS_MB.store(l.pixels_mb, Ordering::Relaxed);
    FILES.store(l.files, Ordering::Relaxed);
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
