//! # aylis-media
//!
//! Áudio e vídeo para o aylis.
//!
//! | Componente | Equivalente no Delphi |
//! |---|---|
//! | [`MediaPlayer`] (não visual) | `TMediaPlayer` (áudio) |
//! | [`VideoPlayer`] | `TMediaPlayer` + `TMediaPlayerControl` |
//! | [`Sound`] (não visual) | efeitos curtos, tocando por cima de si mesmos |
//! | [`AudioOutput`] (não visual) | a lista de saídas de áudio (veja [`output`]) |
//! | [`AudioMixer`] (não visual) | mistura microfones, loopback e players |
//!
//! **Áudio:** decodificado com o `symphonia` (MP3, AAC/M4A, WAV, FLAC, OGG Vorbis) e tocado
//! com o `cpal`, igual em todos os sistemas. Fontes próprias implementam [`AudioSource`].
//! Um endereço (`https://...`) toca lido aos poucos, com `Range` ([`open_audio_location`];
//! feature `http`, a padrão; no Windows, sem ela, pelo Media Foundation).
//!
//! **Vídeo, em três camadas** (escolha onde plugar):
//!
//! 1. [`FrameSink`]: entregue quadros prontos de qualquer thread (câmera, stream, o seu
//!    codec, o FFmpeg chamado por você) — o `VideoPlayer` desenha o último.
//! 2. [`VideoDecoder`]: um decodificador; o player cuida da thread, do relógio, de pausar,
//!    buscar e repetir, e toca a trilha de áudio dele.
//! 3. [`register_decoder`]: qual decodificador abre cada arquivo. Os seus primeiro, depois
//!    o do sistema:
//!
//! | Sistema | Decodificador | Observação |
//! |---|---|---|
//! | Windows | Media Foundation | MP4 H.264 + AAC garantido; HEVC, VP9, WMV conforme o sistema; decodifica e converte as cores na placa de vídeo (sem ela, no processador) |
//! | Android | MediaCodec (NDK) | hardware do aparelho; YUV 4:2:0 vira RGBA |
//! | Linux | GStreamer | carregado na hora: sem ele instalado, o vídeo avisa e o app abre normal |
//! | macOS, iOS | AVFoundation | `AVAssetReader` em BGRA |
//! | Navegador | `<video>` | o navegador toca imagem e som; o aylis desenha cada quadro |
//!
//! Fora do Windows e do navegador, o áudio do vídeo é decodificado pelo symphonia.
//!
//! O `VideoPlayer` desenha cada quadro ele mesmo (cantos arredondados, controles por cima,
//! animação), sem janela nativa "por cima de tudo".
//!
//! **Desempenho.** O decodificador entrega os quadros no tamanho em que aparecem
//! ([`VideoDecoder::set_output_size`]): um 4K num player de 1.000 pontos não é copiado
//! inteiro da placa a cada quadro. Atrasado, ele decodifica sem converter os quadros que não
//! vão aparecer ([`VideoDecoder::skip_until`]) e, muito atrasado, salta de quadro-chave em
//! quadro-chave ([`VideoDecoder::seek_keyframe`]). `rate` muda a velocidade (o som fica mudo
//! fora de 1) e [`VideoPlayer::playback_quality`] conta os quadros mostrados e os pulados.
//! Medido no Windows (Media Foundation, Radeon RX 9070 XT, player de 1.000 x 600 pontos), um
//! 4K de 60 quadros por segundo: 60 mostrados em 1x, 120 em 2x, 240 em 4x; em 8x, cerca de
//! 400 decodificados por segundo; em 16x e 32x, de 13 a 17 quadros-chave por segundo (antes
//! eram 17 quadros por segundo em 1x).
//!
//! **O quadro na placa** (Windows, Direct2D; o relatório do Tabernáculo, W15): quando o
//! canvas compõe texturas da mesma placa ([`Canvas::gpu_device`](aylis_core::Canvas::gpu_device)),
//! o Media Foundation entrega o quadro decodificado e o conversor de vídeo do Direct3D 11 o
//! escreve numa textura compartilhada ([`Frame::gpu`]), que o Direct2D desenha sem passar pela
//! CPU. O Skia e o navegador recebem os pixels (a mesma imagem, copiada uma vez). Medido com
//! um 4K de 60 quadros por segundo no tamanho dele: o decodificador gasta 1,6 ms por quadro
//! (eram 13 a 15 ms descendo o quadro), a pintura no p95 0,6 ms (eram 8,6 a 9 ms subindo o
//! quadro) e o processo 13 a 18% de um núcleo (eram 79 a 83%).

#[cfg(target_os = "android")]
mod android_camera;
mod audio;
pub mod camera;
mod camera_view;
pub mod capture;
mod microphone;
pub mod output;
mod sound;
#[cfg(target_vendor = "apple")]
pub mod avf;
mod components;
#[cfg(all(feature = "http", not(target_arch = "wasm32")))]
mod http;
#[cfg(target_os = "linux")]
pub mod gst;
#[cfg(target_os = "android")]
pub mod mediacodec;
#[cfg(windows)]
pub mod mf;
mod video;
#[cfg(target_arch = "wasm32")]
mod web;

pub use audio::{AudioOpener, AudioPlayer, AudioSource, SymphoniaSource, open_audio_location};
pub use components::{MediaPlayer, PlaybackQuality, VideoPlayer};
pub use camera::{CameraFacing, CameraInfo, cameras, resolve_camera};
pub use camera_view::CameraView;
pub use microphone::Microphone;
pub use output::{OutputDevice, OutputSpec, Tap, format_outputs, output_devices, parse_outputs};
pub use sound::{AudioMixer, AudioOutput, MixSource, Sound, parse_sources};
pub use video::{Frame, FrameSink, GpuFrame, GpuFrameSource, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo, open_video, register_decoder};

use aylis_core::Ui;

/// Registra os componentes de mídia (categoria "Media" da paleta).
pub fn register(ui: &mut Ui) {
    ui.register_class::<MediaPlayer>("Media");
    ui.register_class::<VideoPlayer>("Media");
    ui.register_class::<Microphone>("Media");
    ui.register_class::<AudioOutput>("Media");
    ui.register_class::<Sound>("Media");
    ui.register_class::<AudioMixer>("Media");
    ui.register_class::<CameraView>("Media");
}

/// Começa a tocar os players com `autoplay` (o `aylis::Application` chama ao abrir). Os
/// erros vão para o `on_error` de cada um.
pub fn autoplay(ui: &mut Ui) {
    if ui.design_mode() {
        return;
    }
    let ids: Vec<_> = ui.components().collect();
    for id in ids {
        if ui.get::<MediaPlayer>(id).is_some_and(|m| m.autoplay) {
            let _ = MediaPlayer::play(ui, id);
        } else if ui.get::<VideoPlayer>(id).is_some_and(|v| v.autoplay) {
            let _ = VideoPlayer::play(ui, id);
        }
    }
}
