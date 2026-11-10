//! Câmera: a lista das câmeras e a abertura de uma delas como fonte de quadros ao vivo.
//!
//! | Sistema | Como |
//! |---|---|
//! | Windows | Media Foundation (o leitor de vídeo, com a câmera como fonte) |
//! | Linux | GStreamer (`v4l2src`) |
//! | Navegador | `getUserMedia` (o navegador pede a permissão) |
//! | Android | Camera2 do NDK (a permissão é pedida na primeira vez) |
//! | macOS, iOS | ainda não |

#[cfg(not(target_arch = "wasm32"))]
use crate::video::VideoDecoder;

/// Uma câmera do sistema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CameraInfo {
    /// O que passar para abrir (o `camera` do `CameraView`).
    pub id: String,
    pub name: String,
    /// Para onde a lente aponta, quando o sistema diz (celular); `None` no desktop.
    pub facing: Option<CameraFacing>,
}

/// Para onde a lente de uma câmera aponta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraFacing {
    /// A da tela (selfie).
    Front,
    /// A de trás.
    Back,
    /// Uma câmera ligada por fora (USB).
    External,
}

/// A qualidade da imagem pedida à câmera: o maior tamanho que o aparelho entrega até o limite,
/// na proporção do sensor (sem cortar nem esticar).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CameraQuality {
    /// Até 640x480: o mais leve (análise de imagem, rede fraca).
    Low,
    /// Até 1280x960: nítido na tela do celular sem pesar. O padrão.
    #[default]
    Medium,
    /// Até 1920x1440 (Full HD).
    High,
    /// O maior que a câmera dá (cada quadro vira RGBA na CPU: pesa).
    Max,
}

impl aylis_core::prop::PropEnum for CameraQuality {
    const NAMES: &'static [&'static str] = &["low", "medium", "high", "max"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Low, Self::Medium, Self::High, Self::Max].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl CameraQuality {
    /// O limite de pixels por quadro.
    pub fn max_pixels(self) -> u64 {
        match self {
            Self::Low => 640 * 480,
            Self::Medium => 1280 * 960,
            Self::High => 1920 * 1440,
            Self::Max => u64::MAX,
        }
    }
}

/// O que pedir ao abrir a câmera.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CameraRequest {
    pub quality: CameraQuality,
    /// Um tamanho exato (`resolution` do `CameraView`); se a câmera não tiver, vale a
    /// qualidade.
    pub resolution: Option<(u32, u32)>,
}

/// O que a câmera aberta sabe fazer (o aparelho diz ao abrir).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CameraCaps {
    /// Os tamanhos que ela entrega, do maior para o menor.
    pub sizes: Vec<(u32, u32)>,
    /// Tem flash (a lanterna do `torch`).
    pub has_flash: bool,
}

/// `1280x960` (também `1280×960` e `1280 x 960`).
pub fn parse_size(text: &str) -> Option<(u32, u32)> {
    let (w, h) = text.trim().split_once(['x', 'X', '×'])?;
    Some((w.trim().parse().ok()?, h.trim().parse().ok()?)).filter(|(w, h)| *w > 0 && *h > 0)
}

/// O tamanho a abrir entre `sizes`: o exato pedido, se a câmera tiver; senão o maior até o
/// limite da qualidade na proporção do sensor (a do maior tamanho), depois o maior até o
/// limite em qualquer proporção, por fim o menor. `None` sem tamanhos (o sistema escolhe).
pub fn choose_size(sizes: &[(u32, u32)], request: &CameraRequest) -> Option<(u32, u32)> {
    if let Some(exact) = request.resolution.filter(|r| sizes.contains(r)) {
        return Some(exact);
    }
    let area = |&(w, h): &(u32, u32)| w as u64 * h as u64;
    let &(sw, sh) = sizes.iter().max_by_key(|s| area(s))?;
    let limit = request.quality.max_pixels();
    let fits = |s: &&(u32, u32)| area(s) <= limit;
    let same_shape = |s: &&(u32, u32)| s.0 as u64 * sh as u64 == s.1 as u64 * sw as u64;
    sizes
        .iter()
        .filter(fits)
        .filter(same_shape)
        .max_by_key(|s| area(s))
        .or_else(|| sizes.iter().filter(fits).max_by_key(|s| area(s)))
        .or_else(|| sizes.iter().min_by_key(|s| area(s)))
        .copied()
}

/// Ordena do maior para o menor, sem repetir.
pub(crate) fn sort_sizes(mut sizes: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    sizes.sort_by_key(|&(w, h)| std::cmp::Reverse((w as u64 * h as u64, w)));
    sizes.dedup();
    sizes
}

/// A câmera de `camera` (o `camera` do `CameraView`): o `id` de [`cameras`], `front` ou
/// `back` (a primeira com a lente para aquele lado; sem essa informação, como no desktop, a
/// primeira câmera) ou vazio (a primeira).
pub fn resolve_camera(camera: &str) -> Option<String> {
    let list = cameras();
    let facing = match camera.trim().to_ascii_lowercase().as_str() {
        "" => return list.into_iter().next().map(|c| c.id),
        "front" => CameraFacing::Front,
        "back" => CameraFacing::Back,
        _ => return Some(camera.to_string()),
    };
    let wanted = list.iter().find(|c| c.facing == Some(facing)).map(|c| c.id.clone());
    wanted.or_else(|| list.iter().find(|c| c.facing.is_none()).map(|c| c.id.clone()))
}

/// As câmeras do sistema.
pub fn cameras() -> Vec<CameraInfo> {
    #[cfg(windows)]
    {
        crate::mf::cameras().into_iter().map(|(name, id)| CameraInfo { id, name, facing: None }).collect()
    }
    #[cfg(target_os = "linux")]
    {
        let mut out: Vec<CameraInfo> = std::fs::read_dir("/dev")
            .map(|d| {
                d.flatten()
                    .map(|e| e.path())
                    .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("video")))
                    .map(|p| {
                        let dev = p.display().to_string();
                        let sys = format!("/sys/class/video4linux/{}/name", p.file_name().and_then(|n| n.to_str()).unwrap_or_default());
                        CameraInfo { name: std::fs::read_to_string(sys).map(|n| n.trim().to_string()).unwrap_or_else(|_| dev.clone()), id: dev, facing: None }
                    })
                    .collect()
            })
            .unwrap_or_default();
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }
    #[cfg(target_os = "android")]
    {
        crate::android_camera::cameras().into_iter().map(|(name, id, facing)| CameraInfo { id, name, facing }).collect()
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "android")))]
    {
        Vec::new()
    }
}

/// Abre a câmera `id` (vazio: a primeira) como fonte de quadros ao vivo (o `CameraView`
/// usa; serve também para ler quadros direto, sem tela), na qualidade padrão.
#[cfg(not(target_arch = "wasm32"))]
pub fn open_camera(id: &str) -> Result<Box<dyn VideoDecoder>, String> {
    open_camera_with(id, &CameraRequest::default())
}

/// Como [`open_camera`], com a qualidade ou o tamanho pedidos. O que a câmera sabe fazer
/// (tamanhos, flash) vem do decodificador: [`VideoDecoder::camera_caps`]. No navegador, o
/// `WebVideo::open_camera` faz o mesmo (o `CameraView` usa).
#[cfg(not(target_arch = "wasm32"))]
pub fn open_camera_with(id: &str, request: &CameraRequest) -> Result<Box<dyn VideoDecoder>, String> {
    let id = resolve_camera(id).ok_or_else(|| aylis_core::messages::t("nenhuma câmera"))?;
    #[cfg(windows)]
    {
        Ok(crate::mf::open_camera(&id, *request))
    }
    #[cfg(target_os = "linux")]
    {
        crate::gst::open_camera(&id, request)
    }
    #[cfg(target_os = "android")]
    {
        crate::android_camera::open_camera(&id, request)
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "android")))]
    {
        let _ = (id, request);
        Err(aylis_core::messages::t("nenhuma câmera"))
    }
}

/// Um plano de uma imagem YUV: os bytes, o passo da linha e o do pixel.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
#[derive(Clone, Copy)]
pub(crate) struct Plane<'a> {
    pub data: &'a [u8],
    pub row: usize,
    pub pixel: usize,
}

/// YUV 4:2:0 (o que as câmeras entregam) em RGBA, já girado `degrees` (0, 90, 180 ou 270)
/// numa passada só. Contas inteiras (BT.601 de faixa completa em ponto fixo) e as linhas de
/// saída repartidas entre os núcleos: um quadro de 12 MP sai em dezenas de milissegundos, não
/// em centenas. Com `step` > 1, pega um pixel a cada `step` em cada direção: o quadro já no
/// tamanho em que aparece (a foto converte o quadro cheio, com `step` 1).
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
pub(crate) fn yuv420_to_rgba(y: Plane<'_>, u: Plane<'_>, v: Plane<'_>, w: usize, h: usize, degrees: i32, step: usize) -> (usize, usize, Vec<u8>) {
    let degrees = degrees.rem_euclid(360);
    let step = step.max(1);
    let (fw, fh) = if degrees == 90 || degrees == 270 { (h, w) } else { (w, h) };
    let (ow, oh) = (fw.div_ceil(step), fh.div_ceil(step));
    let mut out = vec![0u8; ow * oh * 4];
    if ow == 0 || oh == 0 {
        return (ow, oh, out);
    }
    // De onde vem o pixel (ox, oy) da saída: o (ox, oy) do quadro cheio girado é o
    // `(ox * step, oy * step)`.
    let source = move |ox: usize, oy: usize| {
        let (ox, oy) = ((ox * step).min(fw - 1), (oy * step).min(fh - 1));
        match degrees {
            90 => (oy, h - 1 - ox),
            180 => (w - 1 - ox, h - 1 - oy),
            270 => (w - 1 - oy, ox),
            _ => (ox, oy),
        }
    };
    // Em blocos de colunas: girado, uma linha da saída é uma coluna da origem, e percorrer a
    // linha inteira erraria o cache a cada pixel (800 ms num quadro de 12 MP no ZenFone). Num
    // bloco estreito, as linhas da origem lidas continuam no cache de uma linha da saída para
    // a seguinte.
    const BLOCK: usize = 32;
    let convert = move |rows: &mut [u8], first: usize| {
        let count = rows.len() / (ow * 4);
        for start in (0..ow).step_by(BLOCK) {
            let end = (start + BLOCK).min(ow);
            for i in 0..count {
                let oy = first + i;
                let line = &mut rows[i * ow * 4..(i + 1) * ow * 4];
                for ox in start..end {
                    let px = &mut line[ox * 4..ox * 4 + 4];
                    let (sx, sy) = source(ox, oy);
                    let luma = i32::from(*y.data.get(sy * y.row + sx).unwrap_or(&0));
                    let cu = i32::from(*u.data.get((sy / 2) * u.row + (sx / 2) * u.pixel).unwrap_or(&128)) - 128;
                    let cv = i32::from(*v.data.get((sy / 2) * v.row + (sx / 2) * v.pixel).unwrap_or(&128)) - 128;
                    // 1,402 V; 0,344 U + 0,714 V; 1,772 U (em 1/256).
                    px[0] = (luma + ((359 * cv) >> 8)).clamp(0, 255) as u8;
                    px[1] = (luma - ((88 * cu + 183 * cv) >> 8)).clamp(0, 255) as u8;
                    px[2] = (luma + ((454 * cu) >> 8)).clamp(0, 255) as u8;
                    px[3] = 255;
                }
            }
        }
    };
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get()).min(8);
    // Quadro pequeno: a thread custa mais que a conta.
    if threads <= 1 || ow * oh < 200_000 {
        convert(&mut out, 0);
        return (ow, oh, out);
    }
    let rows_each = oh.div_ceil(threads);
    std::thread::scope(|scope| {
        for (n, chunk) in out.chunks_mut(rows_each * ow * 4).enumerate() {
            scope.spawn(move || convert(chunk, n * rows_each));
        }
    });
    (ow, oh, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reduzido (`step` 3): cada pixel é o do quadro cheio a cada 3, em todos os giros.
    #[test]
    fn a_reduced_frame_samples_the_full_one() {
        let (w, h) = (100usize, 70usize);
        let ys: Vec<u8> = (0..w * h).map(|i| (i % 251) as u8).collect();
        let cs: Vec<u8> = vec![128; w * h];
        let (y, c) = (Plane { data: &ys, row: w, pixel: 1 }, Plane { data: &cs, row: w, pixel: 2 });
        for degrees in [0, 90, 180, 270] {
            let (fw, fh, full) = yuv420_to_rgba(y, c, c, w, h, degrees, 1);
            let (ow, oh, small) = yuv420_to_rgba(y, c, c, w, h, degrees, 3);
            assert_eq!((ow, oh), (fw.div_ceil(3), fh.div_ceil(3)));
            for (ox, oy) in [(0, 0), (ow - 1, oh - 1), (5, 7), (ow / 2, oh / 2)] {
                let (sx, sy) = ((ox * 3).min(fw - 1), (oy * 3).min(fh - 1));
                assert_eq!(small[(oy * ow + ox) * 4], full[(sy * fw + sx) * 4], "{degrees}° ({ox},{oy})");
            }
        }
    }

    /// A conversão em ponto fixo, paralela e já girada, dá o mesmo que a de `f32` seguida do
    /// giro (a de antes), com no máximo 2 de diferença por canal.
    #[test]
    fn yuv_conversion_matches_the_float_one_in_every_rotation() {
        let (w, h) = (640usize, 480usize);
        let ys: Vec<u8> = (0..w * h).map(|i| (i * 7 % 256) as u8).collect();
        let us: Vec<u8> = (0..(w / 2) * (h / 2) * 2).map(|i| (i * 13 % 256) as u8).collect();
        let vs: Vec<u8> = (0..(w / 2) * (h / 2) * 2).map(|i| (i * 29 % 256) as u8).collect();
        // Como o Android costuma entregar: U e V intercalados, passo 2.
        let (y, u, v) = (Plane { data: &ys, row: w, pixel: 1 }, Plane { data: &us, row: w, pixel: 2 }, Plane { data: &vs, row: w, pixel: 2 });
        let float = |sx: usize, sy: usize| {
            let l = ys[sy * w + sx] as f32;
            let cu = us[(sy / 2) * w + (sx / 2) * 2] as f32 - 128.0;
            let cv = vs[(sy / 2) * w + (sx / 2) * 2] as f32 - 128.0;
            [(l + 1.402 * cv).clamp(0.0, 255.0) as i32, (l - 0.344 * cu - 0.714 * cv).clamp(0.0, 255.0) as i32, (l + 1.772 * cu).clamp(0.0, 255.0) as i32]
        };
        for degrees in [0, 90, 180, 270] {
            let (ow, oh, out) = yuv420_to_rgba(y, u, v, w, h, degrees, 1);
            assert_eq!((ow, oh), if degrees % 180 == 0 { (w, h) } else { (h, w) });
            for (ox, oy) in [(0, 0), (ow - 1, 0), (0, oh - 1), (ow - 1, oh - 1), (123, 77), (ow / 2, oh / 3)] {
                // O pixel de origem pelo giro da função antiga (`rotate`).
                let (sx, sy) = match degrees {
                    90 => (oy, h - 1 - ox),
                    180 => (w - 1 - ox, h - 1 - oy),
                    270 => (w - 1 - oy, ox),
                    _ => (ox, oy),
                };
                let o = (oy * ow + ox) * 4;
                let want = float(sx, sy);
                for c in 0..3 {
                    assert!((i32::from(out[o + c]) - want[c]).abs() <= 2, "{degrees}° ({ox},{oy}) canal {c}: {} x {}", out[o + c], want[c]);
                }
                assert_eq!(out[o + 3], 255);
            }
        }
    }

    const ZENFONE_FRONT: [(u32, u32); 8] = [(3264, 2448), (3264, 1836), (1920, 1080), (1280, 960), (1280, 720), (960, 960), (640, 480), (176, 144)];

    #[test]
    fn the_quality_picks_the_largest_size_in_the_sensor_shape() {
        let pick = |quality, resolution| choose_size(&ZENFONE_FRONT, &CameraRequest { quality, resolution });
        assert_eq!(pick(CameraQuality::Low, None), Some((640, 480)));
        assert_eq!(pick(CameraQuality::Medium, None), Some((1280, 960)), "não o 960x960, que tem mais pixels que 1280x720 mas é quadrado");
        assert_eq!(pick(CameraQuality::High, None), Some((1280, 960)), "o 1920x1080 é 16:9; o sensor é 4:3");
        assert_eq!(pick(CameraQuality::Max, None), Some((3264, 2448)));
        assert_eq!(pick(CameraQuality::Low, Some((1920, 1080))), Some((1920, 1080)), "o exato, se a câmera tem");
        assert_eq!(pick(CameraQuality::Low, Some((1000, 1000))), Some((640, 480)), "não tem: vale a qualidade");
        assert_eq!(choose_size(&[(3264, 2448)], &CameraRequest::default()), Some((3264, 2448)), "nada cabe: o menor");
        assert_eq!(choose_size(&[], &CameraRequest::default()), None);
        assert_eq!(parse_size("1920 x 1080"), Some((1920, 1080)));
        assert_eq!(parse_size("1280×960"), Some((1280, 960)));
        assert_eq!(parse_size("auto"), None);
        assert_eq!(sort_sizes(vec![(640, 480), (1280, 960), (640, 480)]), [(1280, 960), (640, 480)]);
    }
}
