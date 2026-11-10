//! Eventos do form frmPrincipal: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! O teste do celular de verdade (docs/testes-em-dispositivos.md): uma aba por item que o
//! emulador não cobre. Teclado sem sugestões, 3D, câmera traseira e frontal (com a troca e
//! um ciclo de 20 aberturas), microfone (nível e gravação de 5 s), vídeo (com um ciclo de 10
//! aberturas), notificações (simples, com botões, com progresso, com a imagem de um form) e
//! sistema (tema claro/escuro, limite de 30 quadros/s, área segura). Rodar com
//! `aylis-tool run examples/TesteAparelho/TesteAparelho.aylisproj --target android`.

use aylis::prelude::*;
use crate::generated::{FrmPrincipal, Notification};

use std::cell::Cell;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

thread_local! {
    /// O 3D já recebeu o lingote.
    static CENA: Cell<bool> = const { Cell::new(false) };
    /// Ciclos de abrir e fechar que faltam (câmera, vídeo).
    static CICLO_CAM: Cell<u32> = const { Cell::new(0) };
    static CICLO_VID: Cell<u32> = const { Cell::new(0) };
    /// Fim da gravação do microfone (em segundos do `ui.time()`; 0: não grava).
    static GRAVA_ATE: Cell<f64> = const { Cell::new(0.0) };
    static PROGRESSO: Cell<f64> = const { Cell::new(-1.0) };
    static FPS30: Cell<bool> = const { Cell::new(false) };
}

impl FrmPrincipal {
    fn status(&mut self, ui: &mut Ui, texto: impl Into<String>) {
        self.lblStatus.set_text(ui, texto);
    }

    pub fn cam_ligar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let ligada = self.camView.active(ui);
        self.camView.set_active(ui, !ligada);
        self.status(ui, if ligada { "Câmera desligada" } else { "Câmera ligada" });
    }

    pub fn cam_trocar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let nova = if self.camView.camera(ui) == "front" { "back" } else { "front" };
        self.camView.set_camera(ui, nova);
        self.camView.set_mirror(ui, nova == "front");
        self.status(ui, format!("Câmera: {nova}"));
    }

    /// A qualidade da imagem: low (até 640x480), medium (até 1280x960), high (até 1920x1440) ou
    /// max (o maior que a câmera dá). Com a câmera ligada, ela reabre na nova.
    pub fn cam_qualidade_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let nomes = ["low", "medium", "high", "max"];
        let i = self.segCamQualidade.item_index(ui).clamp(0, 3) as usize;
        let _ = ui.set_prop(self.camView.id, "quality", nomes[i]);
        self.status(ui, format!("Qualidade: {}", nomes[i]));
    }

    /// Acende ou apaga o flash como lanterna (só se a câmera tiver).
    pub fn cam_flash_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let cam = self.camView.id;
        if !matches!(ui.get_prop(cam, "has_flash"), Ok(PropValue::Bool(true))) {
            self.status(ui, "Esta câmera não tem flash (ou ainda não abriu)");
            return;
        }
        let aceso = matches!(ui.get_prop(cam, "torch"), Ok(PropValue::Bool(true)));
        let _ = ui.set_prop(cam, "torch", !aceso);
        self.status(ui, if aceso { "Flash apagado" } else { "Flash aceso" });
    }

    /// A câmera abriu: os limites do aparelho (os tamanhos que ela dá e se tem flash).
    pub fn cam_pronta(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let cam = self.camView.id;
        let texto = |ui: &Ui, p: &str| match ui.get_prop(cam, p) {
            Ok(PropValue::String(s)) => s,
            _ => String::new(),
        };
        let tamanhos = texto(ui, "sizes");
        let maior = tamanhos.split(';').next().unwrap_or("?").to_string();
        let flash = matches!(ui.get_prop(cam, "has_flash"), Ok(PropValue::Bool(true)));
        self.status(ui, format!("Câmera: {} tamanhos (o maior, {maior}); flash: {}", tamanhos.split(';').filter(|s| !s.is_empty()).count(), if flash { "sim" } else { "não" }));
    }

    pub fn cam_ciclo_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        CICLO_CAM.with(|c| c.set(40));
        self.status(ui, "Câmera: abrindo e fechando 20 vezes…");
    }

    pub fn mic_ligar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let ligado = self.micEntrada.active(ui);
        self.micEntrada.set_active(ui, !ligado);
        self.status(ui, if ligado { "Microfone desligado" } else { "Microfone ligado" });
    }

    pub fn mic_gravar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let mic = self.micEntrada.id;
        match aylis::media::Microphone::record(ui, mic) {
            Ok(()) => {
                GRAVA_ATE.with(|g| g.set(ui.time() + 5.0));
                self.status(ui, "Gravando 5 s…");
            }
            Err(e) => self.status(ui, format!("Gravar: {e}")),
        }
    }

    pub fn vid_tocar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.vpVideo.play(ui);
        self.status(ui, "Vídeo tocando");
    }

    pub fn vid_parar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.vpVideo.stop(ui);
        self.status(ui, "Vídeo parado");
    }

    pub fn vid_ciclo_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        CICLO_VID.with(|c| c.set(20));
        self.status(ui, "Vídeo: abrindo e fechando 10 vezes…");
    }

    fn notificar(&mut self, ui: &mut Ui, notificacao: Notification) {
        let nome = ui.name(notificacao.id).unwrap_or_default().to_string();
        match aylis::controls::Notification::show(ui, notificacao.id) {
            Ok(()) => self.status(ui, format!("{nome}: mostrada")),
            Err(e) => self.status(ui, format!("{nome}: {e}")),
        }
    }

    pub fn not_simples_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.notificar(ui, self.ntfSimples);
    }

    pub fn not_botoes_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.notificar(ui, self.ntfBotoes);
    }

    pub fn not_progresso_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        PROGRESSO.with(|p| p.set(0.0));
        self.ntfProgresso.set_progress(ui, 0.0);
        self.notificar(ui, self.ntfProgresso);
    }

    pub fn not_form_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.notificar(ui, self.ntfForm);
    }

    pub fn not_acao(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        self.status(ui, format!("Botão da notificação: {args:?}"));
    }

    pub fn tema_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let (familia, escuro) = (ui.theme().family.clone(), ui.theme().dark);
        if let Some(tema) = aylis::core::themes::of_family(&familia, !escuro) {
            ui.set_theme(tema.clone());
            self.status(ui, format!("Tema: {}", tema.name));
        }
    }

    pub fn fps_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let limitar = !FPS30.with(|f| f.get());
        FPS30.with(|f| f.set(limitar));
        ui.set_max_fps(if limitar { 30.0 } else { 0.0 });
        self.status(ui, if limitar { "Limitado a 30 quadros/s" } else { "Sem limite de quadros" });
    }

    pub fn tick_timer(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        // O lingote da abertura da IDE no 3D, na primeira volta (o mesmo modelo, câmera, giro e
        // brilho: `aylis::scene3d::ingot`).
        if !CENA.with(|c| c.replace(true))
            && let Some(view) = ui.get_mut::<aylis::scene3d::Model3D>(self.m3dCena.id)
        {
            aylis::scene3d::ingot::apply(view);
            let cena = self.m3dCena.id;
            // Aqui é o teste dos gestos: a câmera de órbita no lugar da do modelo e o lingote
            // parado (um dedo gira, pinça aproxima, dois dedos deslocam, toque duplo centraliza).
            let _ = ui.set_prop(cena, "camera", "");
            let _ = ui.set_prop(cena, "auto_rotate", 0.0);
            let _ = ui.set_prop(cena, "orbit", true);
        }
        // O tamanho dos quadros da câmera agora (muda com a qualidade).
        let tamanho = match ui.get_prop(self.camView.id, "frame_size") {
            Ok(PropValue::String(s)) => s,
            _ => String::new(),
        };
        if self.lblCamInfo.text(ui) != tamanho {
            self.lblCamInfo.set_text(ui, tamanho);
        }
        // O nível do microfone.
        let mic = self.micEntrada.id;
        let nivel = aylis::media::Microphone::level(ui, mic);
        self.lvlMic.set_value(ui, nivel as f64);
        let apps = aylis::media::Microphone::level(ui, self.micApps.id);
        self.lblMic.set_text(ui, format!("Nível: {:.2}  apps: {:.2}", nivel, apps));
        let ate = GRAVA_ATE.with(|g| g.get());
        if ate > 0.0 && ui.time() >= ate {
            GRAVA_ATE.with(|g| g.set(0.0));
            let arquivo = std::env::temp_dir().join("teste-microfone.wav");
            match aylis::media::Microphone::stop_recording(ui, mic, &arquivo) {
                Ok(s) => self.status(ui, format!("Gravado {s:.1} s em {}", arquivo.display())),
                Err(e) => self.status(ui, format!("Gravação: {e}")),
            }
        }
        // Abrir e fechar a câmera e o vídeo (a revisão de 31/10).
        let cam = CICLO_CAM.with(|c| c.get());
        if cam > 0 {
            CICLO_CAM.with(|c| c.set(cam - 1));
            let ligada = self.camView.active(ui);
            self.camView.set_active(ui, !ligada);
            if cam == 1 {
                self.status(ui, "Câmera: ciclo terminado");
            }
        }
        let vid = CICLO_VID.with(|c| c.get());
        if vid > 0 {
            CICLO_VID.with(|c| c.set(vid - 1));
            if vid % 2 == 0 {
                self.vpVideo.set_file(ui, "");
            } else {
                self.vpVideo.set_file(ui, "assets/video/sample.mp4");
                self.vpVideo.play(ui);
            }
            if vid == 1 {
                self.status(ui, "Vídeo: ciclo terminado");
            }
        }
        // A notificação de progresso.
        let p = PROGRESSO.with(|p| p.get());
        if p >= 0.0 {
            let novo = (p + 0.05).min(1.0);
            PROGRESSO.with(|x| x.set(if novo >= 1.0 { -1.0 } else { novo }));
            self.ntfProgresso.set_progress(ui, novo);
            let _ = aylis::controls::Notification::show(ui, self.ntfProgresso.id);
        }
        // A área segura e o tamanho da tela.
        let a = ui.safe_area(self.form.id);
        let tema = ui.theme().name.clone();
        let texto = format!("Área segura: topo {:.0}, baixo {:.0}, esq. {:.0}, dir. {:.0}\nTema: {tema}\n{}", a.top, a.bottom, a.left, a.right, self.sensores(ui));
        self.lblSistema.set_text(ui, texto);
    }

    /// Tira a foto em PNG, mostra o tamanho e o tempo e apaga o arquivo: confere que a foto sai no
    /// tamanho cheio da câmera mesmo com a tela mostrando os quadros reduzidos.
    pub fn cam_foto_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let cam = self.camView.id;
        let arquivo = std::env::temp_dir().join("teste-foto.png");
        let inicio = std::time::Instant::now();
        let texto = match aylis::media::CameraView::photo(ui, cam, &arquivo) {
            Ok(()) => {
                // O tamanho pelo cabeçalho do PNG (largura e altura nos bytes 16 a 23).
                let tamanho = std::fs::read(&arquivo)
                    .ok()
                    .filter(|b| b.len() > 24)
                    .map(|b| format!("{}x{}", u32::from_be_bytes([b[16], b[17], b[18], b[19]]), u32::from_be_bytes([b[20], b[21], b[22], b[23]])))
                    .unwrap_or_default();
                let _ = std::fs::remove_file(&arquivo);
                format!("Foto {tamanho} em {} ms", inicio.elapsed().as_millis())
            }
            Err(e) => format!("Foto: {e}"),
        };
        self.status(ui, texto);
    }

    /// O AppLink abre o endereço no navegador do aparelho.
    pub fn abrir_site_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let texto = match aylis::controls::AppLink::open_url(ui, "https://example.com") {
            Ok(()) => "AppLink: site aberto".to_string(),
            Err(e) => format!("AppLink: {e}"),
        };
        self.lblStatus.set_text(ui, texto);
    }

    /// A folha de compartilhar do sistema, com um texto.
    pub fn compartilhar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if let Err(e) = aylis::controls::AppLink::share(ui, "Teste do aylis", &[]) {
            self.lblStatus.set_text(ui, format!("AppLink: {e}"));
        }
    }

    /// O link que abriu o app (ou chegou com ele aberto).
    pub fn link_recebido(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        self.lblStatus.set_text(ui, format!("Link: {}", args.text().unwrap_or_default()));
    }

    /// O que outro app compartilhou com este.
    pub fn compartilhado_recebido(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        self.lblStatus.set_text(ui, format!("Recebido: {}", args.text().unwrap_or_default()));
    }

    pub fn link_erro(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        self.lblStatus.set_text(ui, format!("AppLink: {}", args.text().unwrap_or_default()));
    }

    /// O componente Vibration: 300 ms.
    pub fn vibrar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let ok = aylis::controls::Vibration::vibrate(ui, 300);
        self.lblStatus.set_text(ui, if ok { "Vibrou" } else { "Este aparelho não vibra" });
    }

    /// Os sensores da aba Sistema, numa linha cada.
    fn sensores(&mut self, ui: &Ui) -> String {
        let (bat, rede, local, mov) = (self.batBateria, self.netRede, self.locLocal, self.motMovimento);
        format!(
            "Bateria: {:.2}% carregando {} (tem {})
    Rede: {} {}
    Local: {:.2}, {:.2} ±{:.2} m (tem {})
    Movimento: {:.2} {:.2} {:.2}
    Bússola: {:.2}°  Perto: {}  Luz: {:.2} lux",
            bat.level(ui), bat.charging(ui), bat.available(ui),
            rede.connected(ui), rede.kind(ui),
            local.latitude(ui), local.longitude(ui), local.accuracy(ui), local.available(ui),
            mov.x(ui), mov.y(ui), mov.z(ui),
            self.cmpBussola.heading(ui), self.proxPerto.near(ui), self.luzAmbiente.lux(ui),
        )
    }

    /// Liga ou desliga o serviço em segundo plano.
    pub fn servico_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let ligado = self.svcFundo.active(ui);
        self.svcFundo.set_active(ui, !ligado);
        self.lblStatus.set_text(ui, if ligado { "Serviço desligado" } else { "Serviço ligado" });
    }

    /// O que o serviço mandou (as mensagens de quando a tela estava fechada chegam juntas).
    pub fn servico_mensagem(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        self.lblStatus.set_text(ui, format!("Serviço: {}", args.text().unwrap_or_default()));
    }

    /// O Phone abre o discador com um número.
    pub fn discar_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let resultado = ui.invoke(self.fonTelefone.id, "dial", &[PropValue::from("5551234")]);
        if let Err(e) = resultado {
            self.lblStatus.set_text(ui, format!("Phone: {e}"));
        }
    }

    /// O estado da chamada (e o número, quando o sistema informa).
    pub fn chamada_estado(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let numero = self.fonTelefone.number(ui);
        self.lblStatus.set_text(ui, format!("Chamada: {} {numero}", args.text().unwrap_or_default()));
    }

    /// O som que os apps tocam (o loopback do Android: o sistema pede a autorização).
    pub fn mic_apps_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let ligado = self.micApps.active(ui);
        self.micApps.set_active(ui, !ligado);
        self.status(ui, if ligado { "Som dos apps desligado" } else { "Som dos apps ligado" });
    }
}

