
/// O form frmPrincipal: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrincipal {
    pub form: Form,
    pub lblStatus: Label,
    pub tabTestes: TabControl,
    pub tabTeclado: TabItem,
    pub lblTec1: Label,
    pub edtNormal: Edit,
    pub lblTec2: Label,
    pub edtSemSugestao: Edit,
    pub lblTec3: Label,
    pub edtSenha: Edit,
    pub tab3D: TabItem,
    pub m3dCena: Model3D,
    pub tabCamera: TabItem,
    pub pnlCamBotoes: Panel,
    pub btnCamLigar: Button,
    pub btnCamTrocar: Button,
    pub btnCamFoto: Button,
    pub btnCamCiclo: Button,
    pub pnlCamQualidade: Panel,
    pub segCamQualidade: SegmentedControl,
    pub btnCamFlash: Button,
    pub lblCamInfo: Label,
    pub camView: CameraView,
    pub tabMicrofone: TabItem,
    pub btnMicLigar: Button,
    pub btnMicGravar: Button,
    pub lvlMic: LevelMeter,
    pub lblMic: Label,
    pub micEntrada: Microphone,
    pub btnMicApps: Button,
    pub micApps: Microphone,
    pub tabVideo: TabItem,
    pub pnlVidBotoes: Panel,
    pub btnVidTocar: Button,
    pub btnVidParar: Button,
    pub btnVidCiclo: Button,
    pub vpVideo: VideoPlayer,
    pub tabNotificacao: TabItem,
    pub btnNotSimples: Button,
    pub btnNotBotoes: Button,
    pub btnNotProgresso: Button,
    pub btnNotForm: Button,
    pub ntfSimples: Notification,
    pub ntfBotoes: Notification,
    pub ntfProgresso: Notification,
    pub ntfForm: Notification,
    pub tabSistema: TabItem,
    pub btnTema: Button,
    pub btnFps: Button,
    pub btnAbrirSite: Button,
    pub btnCompartilhar: Button,
    pub btnVibrar: Button,
    pub btnServico: Button,
    pub svcFundo: BackgroundService,
    pub btnDiscar: Button,
    pub fonTelefone: Phone,
    pub batBateria: Battery,
    pub netRede: Network,
    pub locLocal: Location,
    pub motMovimento: MotionSensor,
    pub cmpBussola: Compass,
    pub proxPerto: ProximitySensor,
    pub luzAmbiente: LightSensor,
    pub vibVibra: Vibration,
    pub lnkApp: AppLink,
    pub lblSistema: Label,
    pub tmrTick: Timer,
    pub data: super::principal::Data,
}

/// Os componentes do frmPrincipal sem o estado, para o código fora dos handlers
/// (`FrmPrincipal::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrincipalComponents {
    pub form: Form,
    pub lblStatus: Label,
    pub tabTestes: TabControl,
    pub tabTeclado: TabItem,
    pub lblTec1: Label,
    pub edtNormal: Edit,
    pub lblTec2: Label,
    pub edtSemSugestao: Edit,
    pub lblTec3: Label,
    pub edtSenha: Edit,
    pub tab3D: TabItem,
    pub m3dCena: Model3D,
    pub tabCamera: TabItem,
    pub pnlCamBotoes: Panel,
    pub btnCamLigar: Button,
    pub btnCamTrocar: Button,
    pub btnCamFoto: Button,
    pub btnCamCiclo: Button,
    pub pnlCamQualidade: Panel,
    pub segCamQualidade: SegmentedControl,
    pub btnCamFlash: Button,
    pub lblCamInfo: Label,
    pub camView: CameraView,
    pub tabMicrofone: TabItem,
    pub btnMicLigar: Button,
    pub btnMicGravar: Button,
    pub lvlMic: LevelMeter,
    pub lblMic: Label,
    pub micEntrada: Microphone,
    pub btnMicApps: Button,
    pub micApps: Microphone,
    pub tabVideo: TabItem,
    pub pnlVidBotoes: Panel,
    pub btnVidTocar: Button,
    pub btnVidParar: Button,
    pub btnVidCiclo: Button,
    pub vpVideo: VideoPlayer,
    pub tabNotificacao: TabItem,
    pub btnNotSimples: Button,
    pub btnNotBotoes: Button,
    pub btnNotProgresso: Button,
    pub btnNotForm: Button,
    pub ntfSimples: Notification,
    pub ntfBotoes: Notification,
    pub ntfProgresso: Notification,
    pub ntfForm: Notification,
    pub tabSistema: TabItem,
    pub btnTema: Button,
    pub btnFps: Button,
    pub btnAbrirSite: Button,
    pub btnCompartilhar: Button,
    pub btnVibrar: Button,
    pub btnServico: Button,
    pub svcFundo: BackgroundService,
    pub btnDiscar: Button,
    pub fonTelefone: Phone,
    pub batBateria: Battery,
    pub netRede: Network,
    pub locLocal: Location,
    pub motMovimento: MotionSensor,
    pub cmpBussola: Compass,
    pub proxPerto: ProximitySensor,
    pub luzAmbiente: LightSensor,
    pub vibVibra: Vibration,
    pub lnkApp: AppLink,
    pub lblSistema: Label,
    pub tmrTick: Timer,
}

impl std::ops::Deref for FrmPrincipal {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmPrincipal {
    /// Monta a instância de um frmPrincipal recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            tabTestes: TabControl { id: handles::resolve(ui, root, "tabTestes", "TabControl", &mut problems) },
            tabTeclado: TabItem { id: handles::resolve(ui, root, "tabTeclado", "TabItem", &mut problems) },
            lblTec1: Label { id: handles::resolve(ui, root, "lblTec1", "Label", &mut problems) },
            edtNormal: Edit { id: handles::resolve(ui, root, "edtNormal", "Edit", &mut problems) },
            lblTec2: Label { id: handles::resolve(ui, root, "lblTec2", "Label", &mut problems) },
            edtSemSugestao: Edit { id: handles::resolve(ui, root, "edtSemSugestao", "Edit", &mut problems) },
            lblTec3: Label { id: handles::resolve(ui, root, "lblTec3", "Label", &mut problems) },
            edtSenha: Edit { id: handles::resolve(ui, root, "edtSenha", "Edit", &mut problems) },
            tab3D: TabItem { id: handles::resolve(ui, root, "tab3D", "TabItem", &mut problems) },
            m3dCena: Model3D { id: handles::resolve(ui, root, "m3dCena", "Model3D", &mut problems) },
            tabCamera: TabItem { id: handles::resolve(ui, root, "tabCamera", "TabItem", &mut problems) },
            pnlCamBotoes: Panel { id: handles::resolve(ui, root, "pnlCamBotoes", "Panel", &mut problems) },
            btnCamLigar: Button { id: handles::resolve(ui, root, "btnCamLigar", "Button", &mut problems) },
            btnCamTrocar: Button { id: handles::resolve(ui, root, "btnCamTrocar", "Button", &mut problems) },
            btnCamFoto: Button { id: handles::resolve(ui, root, "btnCamFoto", "Button", &mut problems) },
            btnCamCiclo: Button { id: handles::resolve(ui, root, "btnCamCiclo", "Button", &mut problems) },
            pnlCamQualidade: Panel { id: handles::resolve(ui, root, "pnlCamQualidade", "Panel", &mut problems) },
            segCamQualidade: SegmentedControl { id: handles::resolve(ui, root, "segCamQualidade", "SegmentedControl", &mut problems) },
            btnCamFlash: Button { id: handles::resolve(ui, root, "btnCamFlash", "Button", &mut problems) },
            lblCamInfo: Label { id: handles::resolve(ui, root, "lblCamInfo", "Label", &mut problems) },
            camView: CameraView { id: handles::resolve(ui, root, "camView", "CameraView", &mut problems) },
            tabMicrofone: TabItem { id: handles::resolve(ui, root, "tabMicrofone", "TabItem", &mut problems) },
            btnMicLigar: Button { id: handles::resolve(ui, root, "btnMicLigar", "Button", &mut problems) },
            btnMicGravar: Button { id: handles::resolve(ui, root, "btnMicGravar", "Button", &mut problems) },
            lvlMic: LevelMeter { id: handles::resolve(ui, root, "lvlMic", "LevelMeter", &mut problems) },
            lblMic: Label { id: handles::resolve(ui, root, "lblMic", "Label", &mut problems) },
            micEntrada: Microphone { id: handles::resolve(ui, root, "micEntrada", "Microphone", &mut problems) },
            btnMicApps: Button { id: handles::resolve(ui, root, "btnMicApps", "Button", &mut problems) },
            micApps: Microphone { id: handles::resolve(ui, root, "micApps", "Microphone", &mut problems) },
            tabVideo: TabItem { id: handles::resolve(ui, root, "tabVideo", "TabItem", &mut problems) },
            pnlVidBotoes: Panel { id: handles::resolve(ui, root, "pnlVidBotoes", "Panel", &mut problems) },
            btnVidTocar: Button { id: handles::resolve(ui, root, "btnVidTocar", "Button", &mut problems) },
            btnVidParar: Button { id: handles::resolve(ui, root, "btnVidParar", "Button", &mut problems) },
            btnVidCiclo: Button { id: handles::resolve(ui, root, "btnVidCiclo", "Button", &mut problems) },
            vpVideo: VideoPlayer { id: handles::resolve(ui, root, "vpVideo", "VideoPlayer", &mut problems) },
            tabNotificacao: TabItem { id: handles::resolve(ui, root, "tabNotificacao", "TabItem", &mut problems) },
            btnNotSimples: Button { id: handles::resolve(ui, root, "btnNotSimples", "Button", &mut problems) },
            btnNotBotoes: Button { id: handles::resolve(ui, root, "btnNotBotoes", "Button", &mut problems) },
            btnNotProgresso: Button { id: handles::resolve(ui, root, "btnNotProgresso", "Button", &mut problems) },
            btnNotForm: Button { id: handles::resolve(ui, root, "btnNotForm", "Button", &mut problems) },
            ntfSimples: Notification { id: handles::resolve(ui, root, "ntfSimples", "Notification", &mut problems) },
            ntfBotoes: Notification { id: handles::resolve(ui, root, "ntfBotoes", "Notification", &mut problems) },
            ntfProgresso: Notification { id: handles::resolve(ui, root, "ntfProgresso", "Notification", &mut problems) },
            ntfForm: Notification { id: handles::resolve(ui, root, "ntfForm", "Notification", &mut problems) },
            tabSistema: TabItem { id: handles::resolve(ui, root, "tabSistema", "TabItem", &mut problems) },
            btnTema: Button { id: handles::resolve(ui, root, "btnTema", "Button", &mut problems) },
            btnFps: Button { id: handles::resolve(ui, root, "btnFps", "Button", &mut problems) },
            btnAbrirSite: Button { id: handles::resolve(ui, root, "btnAbrirSite", "Button", &mut problems) },
            btnCompartilhar: Button { id: handles::resolve(ui, root, "btnCompartilhar", "Button", &mut problems) },
            btnVibrar: Button { id: handles::resolve(ui, root, "btnVibrar", "Button", &mut problems) },
            btnServico: Button { id: handles::resolve(ui, root, "btnServico", "Button", &mut problems) },
            svcFundo: BackgroundService { id: handles::resolve(ui, root, "svcFundo", "BackgroundService", &mut problems) },
            btnDiscar: Button { id: handles::resolve(ui, root, "btnDiscar", "Button", &mut problems) },
            fonTelefone: Phone { id: handles::resolve(ui, root, "fonTelefone", "Phone", &mut problems) },
            batBateria: Battery { id: handles::resolve(ui, root, "batBateria", "Battery", &mut problems) },
            netRede: Network { id: handles::resolve(ui, root, "netRede", "Network", &mut problems) },
            locLocal: Location { id: handles::resolve(ui, root, "locLocal", "Location", &mut problems) },
            motMovimento: MotionSensor { id: handles::resolve(ui, root, "motMovimento", "MotionSensor", &mut problems) },
            cmpBussola: Compass { id: handles::resolve(ui, root, "cmpBussola", "Compass", &mut problems) },
            proxPerto: ProximitySensor { id: handles::resolve(ui, root, "proxPerto", "ProximitySensor", &mut problems) },
            luzAmbiente: LightSensor { id: handles::resolve(ui, root, "luzAmbiente", "LightSensor", &mut problems) },
            vibVibra: Vibration { id: handles::resolve(ui, root, "vibVibra", "Vibration", &mut problems) },
            lnkApp: AppLink { id: handles::resolve(ui, root, "lnkApp", "AppLink", &mut problems) },
            lblSistema: Label { id: handles::resolve(ui, root, "lblSistema", "Label", &mut problems) },
            tmrTick: Timer { id: handles::resolve(ui, root, "tmrTick", "Timer", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrincipal não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrincipalComponents {
            form: instance.form,
            lblStatus: instance.lblStatus,
            tabTestes: instance.tabTestes,
            tabTeclado: instance.tabTeclado,
            lblTec1: instance.lblTec1,
            edtNormal: instance.edtNormal,
            lblTec2: instance.lblTec2,
            edtSemSugestao: instance.edtSemSugestao,
            lblTec3: instance.lblTec3,
            edtSenha: instance.edtSenha,
            tab3D: instance.tab3D,
            m3dCena: instance.m3dCena,
            tabCamera: instance.tabCamera,
            pnlCamBotoes: instance.pnlCamBotoes,
            btnCamLigar: instance.btnCamLigar,
            btnCamTrocar: instance.btnCamTrocar,
            btnCamFoto: instance.btnCamFoto,
            btnCamCiclo: instance.btnCamCiclo,
            pnlCamQualidade: instance.pnlCamQualidade,
            segCamQualidade: instance.segCamQualidade,
            btnCamFlash: instance.btnCamFlash,
            lblCamInfo: instance.lblCamInfo,
            camView: instance.camView,
            tabMicrofone: instance.tabMicrofone,
            btnMicLigar: instance.btnMicLigar,
            btnMicGravar: instance.btnMicGravar,
            lvlMic: instance.lvlMic,
            lblMic: instance.lblMic,
            micEntrada: instance.micEntrada,
            btnMicApps: instance.btnMicApps,
            micApps: instance.micApps,
            tabVideo: instance.tabVideo,
            pnlVidBotoes: instance.pnlVidBotoes,
            btnVidTocar: instance.btnVidTocar,
            btnVidParar: instance.btnVidParar,
            btnVidCiclo: instance.btnVidCiclo,
            vpVideo: instance.vpVideo,
            tabNotificacao: instance.tabNotificacao,
            btnNotSimples: instance.btnNotSimples,
            btnNotBotoes: instance.btnNotBotoes,
            btnNotProgresso: instance.btnNotProgresso,
            btnNotForm: instance.btnNotForm,
            ntfSimples: instance.ntfSimples,
            ntfBotoes: instance.ntfBotoes,
            ntfProgresso: instance.ntfProgresso,
            ntfForm: instance.ntfForm,
            tabSistema: instance.tabSistema,
            btnTema: instance.btnTema,
            btnFps: instance.btnFps,
            btnAbrirSite: instance.btnAbrirSite,
            btnCompartilhar: instance.btnCompartilhar,
            btnVibrar: instance.btnVibrar,
            btnServico: instance.btnServico,
            svcFundo: instance.svcFundo,
            btnDiscar: instance.btnDiscar,
            fonTelefone: instance.fonTelefone,
            batBateria: instance.batBateria,
            netRede: instance.netRede,
            locLocal: instance.locLocal,
            motMovimento: instance.motMovimento,
            cmpBussola: instance.cmpBussola,
            proxPerto: instance.proxPerto,
            luzAmbiente: instance.luzAmbiente,
            vibVibra: instance.vibVibra,
            lnkApp: instance.lnkApp,
            lblSistema: instance.lblSistema,
            tmrTick: instance.tmrTick,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPrincipal aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmPrincipalComponents> {
        ui.form_components(ui.find_path("frmPrincipal")?)
    }

    /// Cria o frmPrincipal (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmPrincipal")
    }

    /// Cria mais um frmPrincipal dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmPrincipal`, `frmPrincipal2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmPrincipalComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmPrincipal")?;
        let name = ui.free_name_in(parent, "frmPrincipal");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        // Como o componente Frame: sem `fill` próprio, transparente (G05).
        if matches!(ui.get_prop(root, "fill"), Ok(aylis::prelude::PropValue::Null)) {
            let _ = ui.set_prop(root, "fill", aylis::prelude::Color::TRANSPARENT);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmPrincipal (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmPrincipalComponents> {
        ui.form_components(root)
    }

    /// O frmPrincipal aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmPrincipal> {
        ui.form_instance(ui.find_path("frmPrincipal")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPrincipal> {
        let form = ui.find_path("frmPrincipal")?;
        ui.form_instance_mut(form)
    }

    /// O AppLink deste form com o nome (`None` se não há um AppLink com ele).
    pub fn find_app_link(&self, name: &str) -> Option<AppLink> {
        match name {
            "lnkApp" => Some(self.lnkApp),
            _ => None,
        }
    }

    /// O AppLink que disparou o evento (`None` se o `sender` não é um AppLink deste form).
    pub fn sender_as_app_link(&self, ui: &Ui, sender: ComponentId) -> Option<AppLink> {
        self.find_app_link(ui.name(sender)?)
    }

    /// O BackgroundService deste form com o nome (`None` se não há um BackgroundService com ele).
    pub fn find_background_service(&self, name: &str) -> Option<BackgroundService> {
        match name {
            "svcFundo" => Some(self.svcFundo),
            _ => None,
        }
    }

    /// O BackgroundService que disparou o evento (`None` se o `sender` não é um BackgroundService deste form).
    pub fn sender_as_background_service(&self, ui: &Ui, sender: ComponentId) -> Option<BackgroundService> {
        self.find_background_service(ui.name(sender)?)
    }

    /// O Battery deste form com o nome (`None` se não há um Battery com ele).
    pub fn find_battery(&self, name: &str) -> Option<Battery> {
        match name {
            "batBateria" => Some(self.batBateria),
            _ => None,
        }
    }

    /// O Battery que disparou o evento (`None` se o `sender` não é um Battery deste form).
    pub fn sender_as_battery(&self, ui: &Ui, sender: ComponentId) -> Option<Battery> {
        self.find_battery(ui.name(sender)?)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnCamLigar" => Some(self.btnCamLigar),
            "btnCamTrocar" => Some(self.btnCamTrocar),
            "btnCamFoto" => Some(self.btnCamFoto),
            "btnCamCiclo" => Some(self.btnCamCiclo),
            "btnCamFlash" => Some(self.btnCamFlash),
            "btnMicLigar" => Some(self.btnMicLigar),
            "btnMicGravar" => Some(self.btnMicGravar),
            "btnMicApps" => Some(self.btnMicApps),
            "btnVidTocar" => Some(self.btnVidTocar),
            "btnVidParar" => Some(self.btnVidParar),
            "btnVidCiclo" => Some(self.btnVidCiclo),
            "btnNotSimples" => Some(self.btnNotSimples),
            "btnNotBotoes" => Some(self.btnNotBotoes),
            "btnNotProgresso" => Some(self.btnNotProgresso),
            "btnNotForm" => Some(self.btnNotForm),
            "btnTema" => Some(self.btnTema),
            "btnFps" => Some(self.btnFps),
            "btnAbrirSite" => Some(self.btnAbrirSite),
            "btnCompartilhar" => Some(self.btnCompartilhar),
            "btnVibrar" => Some(self.btnVibrar),
            "btnServico" => Some(self.btnServico),
            "btnDiscar" => Some(self.btnDiscar),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O CameraView deste form com o nome (`None` se não há um CameraView com ele).
    pub fn find_camera_view(&self, name: &str) -> Option<CameraView> {
        match name {
            "camView" => Some(self.camView),
            _ => None,
        }
    }

    /// O CameraView que disparou o evento (`None` se o `sender` não é um CameraView deste form).
    pub fn sender_as_camera_view(&self, ui: &Ui, sender: ComponentId) -> Option<CameraView> {
        self.find_camera_view(ui.name(sender)?)
    }

    /// O Compass deste form com o nome (`None` se não há um Compass com ele).
    pub fn find_compass(&self, name: &str) -> Option<Compass> {
        match name {
            "cmpBussola" => Some(self.cmpBussola),
            _ => None,
        }
    }

    /// O Compass que disparou o evento (`None` se o `sender` não é um Compass deste form).
    pub fn sender_as_compass(&self, ui: &Ui, sender: ComponentId) -> Option<Compass> {
        self.find_compass(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edtNormal" => Some(self.edtNormal),
            "edtSemSugestao" => Some(self.edtSemSugestao),
            "edtSenha" => Some(self.edtSenha),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblStatus" => Some(self.lblStatus),
            "lblTec1" => Some(self.lblTec1),
            "lblTec2" => Some(self.lblTec2),
            "lblTec3" => Some(self.lblTec3),
            "lblCamInfo" => Some(self.lblCamInfo),
            "lblMic" => Some(self.lblMic),
            "lblSistema" => Some(self.lblSistema),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O LevelMeter deste form com o nome (`None` se não há um LevelMeter com ele).
    pub fn find_level_meter(&self, name: &str) -> Option<LevelMeter> {
        match name {
            "lvlMic" => Some(self.lvlMic),
            _ => None,
        }
    }

    /// O LevelMeter que disparou o evento (`None` se o `sender` não é um LevelMeter deste form).
    pub fn sender_as_level_meter(&self, ui: &Ui, sender: ComponentId) -> Option<LevelMeter> {
        self.find_level_meter(ui.name(sender)?)
    }

    /// O LightSensor deste form com o nome (`None` se não há um LightSensor com ele).
    pub fn find_light_sensor(&self, name: &str) -> Option<LightSensor> {
        match name {
            "luzAmbiente" => Some(self.luzAmbiente),
            _ => None,
        }
    }

    /// O LightSensor que disparou o evento (`None` se o `sender` não é um LightSensor deste form).
    pub fn sender_as_light_sensor(&self, ui: &Ui, sender: ComponentId) -> Option<LightSensor> {
        self.find_light_sensor(ui.name(sender)?)
    }

    /// O Location deste form com o nome (`None` se não há um Location com ele).
    pub fn find_location(&self, name: &str) -> Option<Location> {
        match name {
            "locLocal" => Some(self.locLocal),
            _ => None,
        }
    }

    /// O Location que disparou o evento (`None` se o `sender` não é um Location deste form).
    pub fn sender_as_location(&self, ui: &Ui, sender: ComponentId) -> Option<Location> {
        self.find_location(ui.name(sender)?)
    }

    /// O Microphone deste form com o nome (`None` se não há um Microphone com ele).
    pub fn find_microphone(&self, name: &str) -> Option<Microphone> {
        match name {
            "micEntrada" => Some(self.micEntrada),
            "micApps" => Some(self.micApps),
            _ => None,
        }
    }

    /// O Microphone que disparou o evento (`None` se o `sender` não é um Microphone deste form).
    pub fn sender_as_microphone(&self, ui: &Ui, sender: ComponentId) -> Option<Microphone> {
        self.find_microphone(ui.name(sender)?)
    }

    /// O Model3D deste form com o nome (`None` se não há um Model3D com ele).
    pub fn find_model3_d(&self, name: &str) -> Option<Model3D> {
        match name {
            "m3dCena" => Some(self.m3dCena),
            _ => None,
        }
    }

    /// O Model3D que disparou o evento (`None` se o `sender` não é um Model3D deste form).
    pub fn sender_as_model3_d(&self, ui: &Ui, sender: ComponentId) -> Option<Model3D> {
        self.find_model3_d(ui.name(sender)?)
    }

    /// O MotionSensor deste form com o nome (`None` se não há um MotionSensor com ele).
    pub fn find_motion_sensor(&self, name: &str) -> Option<MotionSensor> {
        match name {
            "motMovimento" => Some(self.motMovimento),
            _ => None,
        }
    }

    /// O MotionSensor que disparou o evento (`None` se o `sender` não é um MotionSensor deste form).
    pub fn sender_as_motion_sensor(&self, ui: &Ui, sender: ComponentId) -> Option<MotionSensor> {
        self.find_motion_sensor(ui.name(sender)?)
    }

    /// O Network deste form com o nome (`None` se não há um Network com ele).
    pub fn find_network(&self, name: &str) -> Option<Network> {
        match name {
            "netRede" => Some(self.netRede),
            _ => None,
        }
    }

    /// O Network que disparou o evento (`None` se o `sender` não é um Network deste form).
    pub fn sender_as_network(&self, ui: &Ui, sender: ComponentId) -> Option<Network> {
        self.find_network(ui.name(sender)?)
    }

    /// O Notification deste form com o nome (`None` se não há um Notification com ele).
    pub fn find_notification(&self, name: &str) -> Option<Notification> {
        match name {
            "ntfSimples" => Some(self.ntfSimples),
            "ntfBotoes" => Some(self.ntfBotoes),
            "ntfProgresso" => Some(self.ntfProgresso),
            "ntfForm" => Some(self.ntfForm),
            _ => None,
        }
    }

    /// O Notification que disparou o evento (`None` se o `sender` não é um Notification deste form).
    pub fn sender_as_notification(&self, ui: &Ui, sender: ComponentId) -> Option<Notification> {
        self.find_notification(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlCamBotoes" => Some(self.pnlCamBotoes),
            "pnlCamQualidade" => Some(self.pnlCamQualidade),
            "pnlVidBotoes" => Some(self.pnlVidBotoes),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O Phone deste form com o nome (`None` se não há um Phone com ele).
    pub fn find_phone(&self, name: &str) -> Option<Phone> {
        match name {
            "fonTelefone" => Some(self.fonTelefone),
            _ => None,
        }
    }

    /// O Phone que disparou o evento (`None` se o `sender` não é um Phone deste form).
    pub fn sender_as_phone(&self, ui: &Ui, sender: ComponentId) -> Option<Phone> {
        self.find_phone(ui.name(sender)?)
    }

    /// O ProximitySensor deste form com o nome (`None` se não há um ProximitySensor com ele).
    pub fn find_proximity_sensor(&self, name: &str) -> Option<ProximitySensor> {
        match name {
            "proxPerto" => Some(self.proxPerto),
            _ => None,
        }
    }

    /// O ProximitySensor que disparou o evento (`None` se o `sender` não é um ProximitySensor deste form).
    pub fn sender_as_proximity_sensor(&self, ui: &Ui, sender: ComponentId) -> Option<ProximitySensor> {
        self.find_proximity_sensor(ui.name(sender)?)
    }

    /// O SegmentedControl deste form com o nome (`None` se não há um SegmentedControl com ele).
    pub fn find_segmented_control(&self, name: &str) -> Option<SegmentedControl> {
        match name {
            "segCamQualidade" => Some(self.segCamQualidade),
            _ => None,
        }
    }

    /// O SegmentedControl que disparou o evento (`None` se o `sender` não é um SegmentedControl deste form).
    pub fn sender_as_segmented_control(&self, ui: &Ui, sender: ComponentId) -> Option<SegmentedControl> {
        self.find_segmented_control(ui.name(sender)?)
    }

    /// O TabControl deste form com o nome (`None` se não há um TabControl com ele).
    pub fn find_tab_control(&self, name: &str) -> Option<TabControl> {
        match name {
            "tabTestes" => Some(self.tabTestes),
            _ => None,
        }
    }

    /// O TabControl que disparou o evento (`None` se o `sender` não é um TabControl deste form).
    pub fn sender_as_tab_control(&self, ui: &Ui, sender: ComponentId) -> Option<TabControl> {
        self.find_tab_control(ui.name(sender)?)
    }

    /// O TabItem deste form com o nome (`None` se não há um TabItem com ele).
    pub fn find_tab_item(&self, name: &str) -> Option<TabItem> {
        match name {
            "tabTeclado" => Some(self.tabTeclado),
            "tab3D" => Some(self.tab3D),
            "tabCamera" => Some(self.tabCamera),
            "tabMicrofone" => Some(self.tabMicrofone),
            "tabVideo" => Some(self.tabVideo),
            "tabNotificacao" => Some(self.tabNotificacao),
            "tabSistema" => Some(self.tabSistema),
            _ => None,
        }
    }

    /// O TabItem que disparou o evento (`None` se o `sender` não é um TabItem deste form).
    pub fn sender_as_tab_item(&self, ui: &Ui, sender: ComponentId) -> Option<TabItem> {
        self.find_tab_item(ui.name(sender)?)
    }

    /// O Timer deste form com o nome (`None` se não há um Timer com ele).
    pub fn find_timer(&self, name: &str) -> Option<Timer> {
        match name {
            "tmrTick" => Some(self.tmrTick),
            _ => None,
        }
    }

    /// O Timer que disparou o evento (`None` se o `sender` não é um Timer deste form).
    pub fn sender_as_timer(&self, ui: &Ui, sender: ComponentId) -> Option<Timer> {
        self.find_timer(ui.name(sender)?)
    }

    /// O Vibration deste form com o nome (`None` se não há um Vibration com ele).
    pub fn find_vibration(&self, name: &str) -> Option<Vibration> {
        match name {
            "vibVibra" => Some(self.vibVibra),
            _ => None,
        }
    }

    /// O Vibration que disparou o evento (`None` se o `sender` não é um Vibration deste form).
    pub fn sender_as_vibration(&self, ui: &Ui, sender: ComponentId) -> Option<Vibration> {
        self.find_vibration(ui.name(sender)?)
    }

    /// O VideoPlayer deste form com o nome (`None` se não há um VideoPlayer com ele).
    pub fn find_video_player(&self, name: &str) -> Option<VideoPlayer> {
        match name {
            "vpVideo" => Some(self.vpVideo),
            _ => None,
        }
    }

    /// O VideoPlayer que disparou o evento (`None` se o `sender` não é um VideoPlayer deste form).
    pub fn sender_as_video_player(&self, ui: &Ui, sender: ComponentId) -> Option<VideoPlayer> {
        self.find_video_player(ui.name(sender)?)
    }
}
