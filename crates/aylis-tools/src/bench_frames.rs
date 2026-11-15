//! `aylis-tool bench-frames`: mede, na placa desta máquina, quanto custa mostrar quadros de
//! câmera e de vídeo no Direct2D com e sem o reaproveitamento do bitmap do quadro anterior, em
//! vários tamanhos, e sugere o `frame_reuse_kb` do `image_cache` do projeto.
//!
//! ```text
//! aylis-tool bench-frames                       # 1, 4, 9, 16 e 36 câmeras numa tela 1920x1080
//! aylis-tool bench-frames --cameras 1,16 --frames 300 --json quadros.json
//! ```
//!
//! Roda o exemplo `camera_frames` do `aylis-d2d` (em release), que troca o limite em execução
//! como o app faria; cada caso, [`ROUNDS`] vezes alternando com e sem, e vale a mediana. Feche o que pesa na máquina (compilações) antes: a medida é de tempo.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::json;

use crate::process::{Control, LineKind, Sink, run_tool};

pub const CAMERAS: [u32; 5] = [1, 4, 9, 16, 36];
pub const ROUNDS: usize = 3;
/// O limite que força sempre reaproveitar (o máximo aceito).
const ALWAYS: u32 = aylis_core::image_cache::FRAME_REUSE_KB_RANGE.1;

/// Um caso medido: a tela dividida entre `cameras`, cada quadro de `width × height`.
#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    pub cameras: u32,
    pub width: u32,
    pub height: u32,
    /// Ms por quadro criando um bitmap a cada quadro.
    pub create_ms: f64,
    /// Ms por quadro reaproveitando.
    pub reuse_ms: f64,
}

impl Case {
    /// Os KB de um quadro.
    pub fn frame_kb(&self) -> u32 {
        ((self.width as u64 * self.height as u64 * 4).div_ceil(1024)) as u32
    }
}

/// O tamanho de cada câmera numa tela `width × height` dividida em grade (como o exemplo faz).
pub fn tile(width: u32, height: u32, cameras: u32) -> (u32, u32) {
    let cols = (cameras.max(1) as f32).sqrt().ceil() as u32;
    let rows = cameras.max(1).div_ceil(cols);
    (width / cols, height / rows)
}

/// O `frame_reuse_kb` sugerido: o maior quadro em que reaproveitar foi mais rápido, se todos os
/// menores também foram; `0` se nem o menor ganhou.
pub fn suggest(cases: &[Case]) -> u32 {
    let mut sorted: Vec<&Case> = cases.iter().collect();
    sorted.sort_by_key(|c| c.frame_kb());
    let mut kb = 0;
    for c in sorted {
        if c.reuse_ms < c.create_ms {
            kb = c.frame_kb();
        } else {
            break;
        }
    }
    kb
}

/// O tempo da linha `AYLIS_FRAMES ms=...` do exemplo.
fn parse_ms(output: &str) -> Option<f64> {
    let line = output.lines().find(|l| l.starts_with("AYLIS_FRAMES"))?;
    line.split_whitespace().find_map(|w| w.strip_prefix("ms=")).and_then(|v| v.parse().ok())
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}

/// Compila o exemplo (release) e devolve o executável.
pub fn build(root: &Path, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let mut c = Command::new("cargo");
    c.current_dir(root).args(["build", "--release", "-p", "aylis-d2d", "--example", "camera_frames"]);
    run_tool(&mut c, control, sink, LineKind::Tool)?;
    let exe = crate::cargo_target_dir(root).join("release").join("examples").join(crate::env::exe("camera_frames"));
    if exe.is_file() { Ok(exe) } else { Err(format!("{}: não foi gerado", exe.display())) }
}

/// Uma rodada: ms por quadro com o limite `kb`.
fn round(exe: &Path, frames: u32, width: u32, height: u32, cameras: u32, kb: u32) -> Result<f64, String> {
    let out = Command::new(exe)
        .args([frames.to_string(), width.to_string(), height.to_string(), cameras.to_string(), kb.to_string()])
        .output()
        .map_err(|e| format!("{}: {e}", exe.display()))?;
    parse_ms(&String::from_utf8_lossy(&out.stdout)).ok_or_else(|| format!("{}: sem a medida ({})", exe.display(), String::from_utf8_lossy(&out.stderr).trim()))
}

/// Mede cada caso alternando com e sem o reaproveitamento.
pub fn measure(exe: &Path, frames: u32, width: u32, height: u32, cameras: &[u32], progress: &mut dyn FnMut(&Case)) -> Result<Vec<Case>, String> {
    let mut cases = Vec::new();
    for &n in cameras {
        let (mut create, mut reuse) = (Vec::new(), Vec::new());
        for _ in 0..ROUNDS {
            create.push(round(exe, frames, width, height, n, 0)?);
            reuse.push(round(exe, frames, width, height, n, ALWAYS)?);
        }
        let (w, h) = tile(width, height, n);
        let case = Case { cameras: n, width: w, height: h, create_ms: median(create), reuse_ms: median(reuse) };
        progress(&case);
        cases.push(case);
    }
    Ok(cases)
}

/// Os casos e a sugestão em JSON (como `medicoes/d2d_frames.json`).
pub fn to_json(cases: &[Case], frames: u32) -> serde_json::Value {
    json!({
        "frames": frames,
        "rounds": ROUNDS,
        "cases": cases.iter().map(|c| json!({
            "cameras": c.cameras, "size": format!("{}x{}", c.width, c.height), "frame_kb": c.frame_kb(),
            "create_ms": (c.create_ms * 1000.0).round() / 1000.0, "reuse_ms": (c.reuse_ms * 1000.0).round() / 1000.0,
        })).collect::<Vec<_>>(),
        "suggested_frame_reuse_kb": suggest(cases),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case(cameras: u32, create_ms: f64, reuse_ms: f64) -> Case {
        let (width, height) = tile(1920, 1080, cameras);
        Case { cameras, width, height, create_ms, reuse_ms }
    }

    #[test]
    fn the_suggestion_is_the_largest_frame_where_reusing_won() {
        // Como em 14/11: ganha até 640x360 (900 KB), perde de 960x540 para cima.
        let cases = [case(1, 0.92, 1.45), case(4, 1.50, 1.70), case(9, 1.81, 1.73), case(16, 2.46, 1.62), case(36, 7.26, 1.69)];
        assert_eq!(suggest(&cases), 900);
        // Nem o menor ganhou: desliga.
        assert_eq!(suggest(&[case(36, 1.0, 2.0), case(16, 1.0, 2.0)]), 0);
        // Um tamanho do meio perdeu: para nele, mesmo que um maior ganhe (ruído).
        assert_eq!(suggest(&[case(36, 2.0, 1.0), case(16, 1.0, 2.0), case(1, 2.0, 1.0)]), 225);
    }

    #[test]
    fn the_tiles_and_the_output_line() {
        assert_eq!(tile(1920, 1080, 1), (1920, 1080));
        assert_eq!(tile(1920, 1080, 16), (480, 270));
        assert_eq!(tile(1920, 1080, 5), (640, 540));
        assert_eq!(parse_ms("600 quadros...\nAYLIS_FRAMES ms=1.234 bitmaps=72\n"), Some(1.234));
        assert_eq!(parse_ms("nada"), None);
    }
}
