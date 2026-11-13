//! O cache de imagens dos renderizadores: a imagem fraca (viva enquanto o app tiver a imagem
//! ou um clone dela), a saída pela usada há mais tempo e os limites do projeto.

use aylis_core::Image;
use aylis_core::image_cache::{self, ImageCache, ImageCacheLimits};

/// Os limites são do processo: os testes que os trocam esperam a vez.
static LIMITS: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn the_weak_image_dies_with_the_last_copy() {
    let image = Image::rgba(2, 2, vec![0; 16]);
    let weak = image.downgrade();
    let copy = image.clone();
    drop(image);
    assert!(weak.is_alive(), "um clone ainda segura a imagem");
    drop(copy);
    assert!(!weak.is_alive(), "sem cópias, o cache pode soltar o que fez dela");
}

#[test]
fn the_weak_image_does_not_keep_the_pixels() {
    let image = Image::rgba(1, 1, vec![1, 2, 3, 4]);
    let weak = image.downgrade();
    let id = image.id();
    drop(image);
    assert!(!weak.is_alive());
    // Uma imagem nova nunca reaproveita o id: o que o cache guardou da antiga não volta a servir.
    assert_ne!(Image::rgba(1, 1, vec![1, 2, 3, 4]).id(), id);
}

fn pixels(n: u8) -> Image {
    Image::rgba(1, 1, vec![n; 4])
}

#[test]
fn over_the_limit_the_least_recently_used_leaves() {
    let (a, b, c) = (pixels(1), pixels(2), pixels(3));
    let mut cache = ImageCache::default();
    cache.insert(&a, "a", 0, 2, usize::MAX);
    cache.insert(&b, "b", 0, 2, usize::MAX);
    // `a` usada de novo: quem sai ao entrar a terceira é `b`, não a primeira a entrar.
    assert_eq!(cache.get(&a), Some(&"a"));
    cache.insert(&c, "c", 0, 2, usize::MAX);
    assert_eq!(cache.len(), 2);
    assert!(cache.get(&a).is_some() && cache.get(&c).is_some());
    assert!(cache.get(&b).is_none());
}

#[test]
fn the_byte_limit_keeps_what_fits_and_never_the_newest_out() {
    let images: Vec<Image> = (0..4).map(pixels).collect();
    let mut cache = ImageCache::default();
    for (i, image) in images.iter().enumerate() {
        cache.insert(image, i, 100, usize::MAX, 250);
    }
    assert_eq!(cache.len(), 2, "250 bytes cabem duas de 100");
    assert_eq!(cache.bytes(), 200);
    assert!(cache.get(&images[3]).is_some());
    // Uma sozinha maior que o teto ainda fica: é a que vai ser desenhada agora.
    let big = pixels(9);
    cache.insert(&big, 9, 1000, usize::MAX, 250);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&big), Some(&9));
}

#[test]
fn dropped_images_leave_before_any_limit() {
    let kept = pixels(1);
    let mut cache = ImageCache::default();
    cache.insert(&kept, 1, 100, 10, usize::MAX);
    {
        let dropped = pixels(2);
        cache.insert(&dropped, 2, 100, 10, usize::MAX);
    }
    cache.forget_dropped();
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.bytes(), 100);
    assert_eq!(cache.get(&kept), Some(&1));
}

#[test]
fn the_limits_stay_inside_the_accepted_range() {
    let _turn = LIMITS.lock().unwrap_or_else(|e| e.into_inner());
    let (min_mb, max_mb) = ImageCacheLimits::PIXELS_MB_RANGE;
    let (min_files, max_files) = ImageCacheLimits::FILES_RANGE;
    let (min_layers, max_layers) = ImageCacheLimits::LAYERS_MB_RANGE;
    assert_eq!(
        ImageCacheLimits { pixels_mb: 0, files: 0, layers_mb: 0 }.clamped(),
        ImageCacheLimits { pixels_mb: min_mb, files: min_files, layers_mb: min_layers }
    );
    assert_eq!(
        ImageCacheLimits { pixels_mb: u32::MAX, files: u32::MAX, layers_mb: u32::MAX }.clamped(),
        ImageCacheLimits { pixels_mb: max_mb, files: max_files, layers_mb: max_layers }
    );
    image_cache::set_limits(ImageCacheLimits { pixels_mb: 200, files: 1000, layers_mb: 64 });
    assert_eq!(image_cache::limits(), ImageCacheLimits { pixels_mb: 200, files: 1000, layers_mb: 64 });
    image_cache::set_limits(ImageCacheLimits::DEFAULT);
}

/// As camadas guardadas: o limite é em bytes (sai a usada há mais tempo, nunca a que entrou),
/// uma maior que o limite inteiro não entra, e o núcleo solta as de uma chave sua.
#[test]
fn the_layer_cache_keeps_the_byte_limit_and_forgets_by_owner() {
    let _turn = LIMITS.lock().unwrap_or_else(|e| e.into_inner());
    use aylis_core::LayerCache;
    let mb = 1 << 20;
    image_cache::set_limits(ImageCacheLimits { layers_mb: 10, ..ImageCacheLimits::DEFAULT });
    let mut cache: LayerCache<&str> = LayerCache::default();
    assert!(LayerCache::<&str>::accepts(10 * mb) && !LayerCache::<&str>::accepts(11 * mb));
    cache.insert(1, 100, "página A", 4 * mb);
    cache.insert(2, 200, "página B", 4 * mb);
    assert_eq!(cache.get(100), Some(&"página A"));
    // A B é a usada há mais tempo: sai para a C caber.
    cache.insert(3, 300, "página C", 4 * mb);
    assert_eq!(cache.get(200), None);
    assert_eq!((cache.len(), cache.bytes()), (2, 8 * mb));
    cache.insert(4, 400, "grande demais", 11 * mb);
    assert_eq!(cache.get(400), None);
    // Duas imagens do mesmo dono (duas escalas): as duas saem juntas.
    cache.insert(1, 101, "página A em 2x", 1 * mb);
    cache.forget(1);
    assert_eq!((cache.get(100).is_none(), cache.get(101).is_none()), (true, true));
    assert_eq!((cache.len(), cache.bytes()), (1, 4 * mb));
    // Com o limite em zero, nada é montado.
    image_cache::set_limits(ImageCacheLimits { layers_mb: 0, ..ImageCacheLimits::DEFAULT });
    assert!(!LayerCache::<&str>::accepts(1));
    image_cache::set_limits(ImageCacheLimits::DEFAULT);
}

#[test]
fn named_caches_show_up_in_the_memory_panel() {
    let (a, b) = (pixels(1), pixels(2));
    let mut first = ImageCache::named("teste · pixels");
    let mut second = ImageCache::named("teste · pixels");
    first.insert(&a, (), 400, usize::MAX, usize::MAX);
    second.insert(&b, (), 100, usize::MAX, usize::MAX);
    let stat = |name: &str| image_cache::stats().into_iter().find(|s| s.name == name);
    // Dois caches de mesmo nome (duas janelas) somam.
    assert_eq!(stat("teste · pixels").map(|s| (s.images, s.bytes)), Some((2, 500)));
    drop(a);
    first.forget_dropped();
    assert_eq!(stat("teste · pixels").map(|s| (s.images, s.bytes)), Some((1, 100)));
    let frames = aylis_core::memory_panel::Frames { fps: 59.6, max_fps: 30.0 };
    let lines = aylis_core::memory_panel::lines(Some(frames), Some(aylis_core::memory_panel::ProcessMemory { private: 52 << 20, working_set: 130 << 20 }));
    assert_eq!(lines[0], "Quadros: 60 por segundo (limite 30)");
    assert!(lines[1].contains("52,0 MB comprometida") && lines[1].contains("130,0 MB working set"), "{lines:?}");
    assert!(lines.iter().any(|l| l.contains("teste · pixels: 1")), "{lines:?}");
    drop((first, second));
    assert_eq!(stat("teste · pixels"), None, "o cache que sumiu sai do painel");
}
