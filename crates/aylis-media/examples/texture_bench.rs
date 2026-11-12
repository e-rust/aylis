//! Mede o quadro de vídeo pelos dois caminhos do Media Foundation (o relatório do
//! Tabernáculo, W15), no tamanho do vídeo:
//!
//! - **pixels:** decodifica na placa, converte para RGB32 e copia o quadro para a CPU (e o
//!   renderizador o manda de volta para a placa);
//! - **textura:** decodifica na placa e o conversor de vídeo do Direct3D 11 escreve o quadro
//!   numa textura compartilhada, que o Direct2D compõe direto.
//!
//! ```text
//! cargo run --release -p aylis-media --example make_mp4 -- 4k.mp4 3840 2160 60 6
//! cargo run --release -p aylis-media --example texture_bench -- 4k.mp4
//! ```

#[cfg(windows)]
fn main() {
    use aylis_core::GpuDevice;
    use aylis_media::{MediaSource, mf, open_video};

    let file = std::env::args().nth(1).expect("informe um vídeo (veja o make_mp4)");
    let source = MediaSource::File(file.into());
    let Some(adapter_luid) = mf::video_adapter() else {
        eprintln!("sem placa com decodificação");
        return;
    };
    for texture in [false, true, false, true] {
        let mut decoder = open_video(&source).expect("abrir o vídeo");
        if texture {
            decoder.set_gpu_device(Some(GpuDevice::Direct3D11 { adapter_luid }));
        }
        // O primeiro quadro abre o decodificador (fora da medição).
        let first = decoder.next_frame().expect("primeiro quadro");
        let size = first.size().unwrap_or_default();
        drop(first);
        let mut times = Vec::new();
        let started = std::time::Instant::now();
        let mut last = None;
        loop {
            let t = std::time::Instant::now();
            let Some(frame) = decoder.next_frame() else { break };
            times.push(t.elapsed().as_secs_f64() * 1000.0);
            assert_eq!(frame.gpu.is_some(), texture);
            last = Some(frame);
        }
        // A placa termina o que ficou na fila dela (a cópia do último quadro espera tudo):
        // o total conta o trabalho da GPU, não só o envio.
        let _ = last.and_then(|f| f.to_image());
        let total = started.elapsed().as_secs_f64();
        times.sort_by(f64::total_cmp);
        let at = |q: f64| times[((times.len() - 1) as f64 * q).round() as usize];
        let avg = times.iter().sum::<f64>() / times.len() as f64;
        println!(
            "{:8} {}x{}  quadros={}  media_ms={avg:.2}  p50_ms={:.2}  p95_ms={:.2}  quadros_por_s={:.0}",
            if texture { "textura" } else { "pixels" },
            size.width,
            size.height,
            times.len(),
            at(0.5),
            at(0.95),
            times.len() as f64 / total
        );
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("só no Windows (Media Foundation)");
}
