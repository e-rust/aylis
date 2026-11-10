//! Gera um MP4 de teste (H.264) com o Media Foundation: faixas coloridas andando e um
//! quadrado que troca de cor a cada segundo, para conferir decodificadores de outros
//! sistemas (Android, Linux, navegador) e medir o player em resoluções altas.
//!
//! ```text
//! cargo run -p aylis-media --example make_mp4 -- saida.mp4
//! cargo run -p aylis-media --example make_mp4 -- 4k.mp4 3840 2160 60 10
//! ```

#[cfg(windows)]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args.first().cloned().unwrap_or_else(|| "sample.mp4".into());
    let number = |i: usize, default: u32| args.get(i).and_then(|a| a.parse().ok()).unwrap_or(default);
    let (w, h, fps, seconds) = (number(1, 320), number(2, 240), number(3, 30), number(4, 4));
    let colors: [[u8; 3]; 4] = [[220, 40, 40], [40, 180, 60], [40, 90, 220], [240, 200, 40]];
    // O quadrado e as faixas crescem com o vídeo (as medidas originais são para 320x240).
    let (sx, sy) = (w as f32 / 320.0, h as f32 / 240.0);
    let (x0, x1, y0, y1) = ((110.0 * sx) as u32, (210.0 * sx) as u32, (70.0 * sy) as u32, (170.0 * sy) as u32);
    let band_w = ((40.0 * sx) as u32).max(1);
    let frames = (0..fps * seconds).map(move |i| {
        let mut px = vec![0u8; (w * h * 4) as usize];
        let square = colors[(i / fps) as usize % colors.len()];
        let shift = (i as f32 * 4.0 * sx) as u32;
        for y in 0..h {
            for x in 0..w {
                let o = ((y * w + x) * 4) as usize;
                let band = ((x + shift) / band_w) % 2 == 0;
                let [r, g, b] = if (x0..x1).contains(&x) && (y0..y1).contains(&y) { square } else if band { [30, 30, 30] } else { [200, 200, 200] };
                // BGRA
                px[o..o + 4].copy_from_slice(&[b, g, r, 255]);
            }
        }
        px
    });
    aylis_media::mf::write_mp4(std::path::Path::new(&path), w, h, fps, frames).unwrap();
    println!("{path}");
}

#[cfg(not(windows))]
fn main() {
    eprintln!("só no Windows (usa o Media Foundation)");
}
