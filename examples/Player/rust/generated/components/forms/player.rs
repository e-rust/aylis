
/// O form frmPlayer: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPlayer {
    pub form: Form,
    pub mnuPrincipal: MainMenu,
    pub mnuArquivo: MenuItem,
    pub mnuAbrir: MenuItem,
    pub mnuLimparFila: MenuItem,
    pub mnuSep1: MenuItem,
    pub mnuSair: MenuItem,
    pub mnuReproduzir: MenuItem,
    pub mnuReproduzirPausar: MenuItem,
    pub mnuParar: MenuItem,
    pub mnuAnterior: MenuItem,
    pub mnuProxima: MenuItem,
    pub mnuSep2: MenuItem,
    pub mnuEmbaralhar: MenuItem,
    pub mnuRepetir: MenuItem,
    pub mnuExibir: MenuItem,
    pub mnuAgora: MenuItem,
    pub mnuFila: MenuItem,
    pub pnlControles: Panel,
    pub lytBarra: Layout,
    pub lblTempo: Label,
    pub lblDuracao: Label,
    pub sbPosicao: SeekBar,
    pub lytInfo: Layout,
    pub icoCapa: Icon,
    pub lytTextos: Layout,
    pub lblTitulo: Label,
    pub lblDetalhe: Label,
    pub lytVolume: Layout,
    pub btnMudo: Button,
    pub tbVolume: TrackBar,
    pub lytBotoes: Layout,
    pub lytBotoesCentro: Layout,
    pub btnEmbaralhar: Button,
    pub btnAnterior: Button,
    pub btnReproduzir: Button,
    pub btnProxima: Button,
    pub btnRepetir: Button,
    pub lytNavegacao: Layout,
    pub btnAgora: Button,
    pub btnFila: Button,
    pub divNavegacao: Divider,
    pub btnAbrirArquivos: Button,
    pub pnlAgora: Panel,
    pub vpVideo: VideoPlayer,
    pub pnlFila: Panel,
    pub lblFila: Label,
    pub lytFilaBotoes: Layout,
    pub btnAdicionar: Button,
    pub btnLimparFila: Button,
    pub lvFila: ListView,
    pub dlgAbrir: OpenDialog,
    pub tmrPosicao: Timer,
    pub data: super::player::Data,
}

/// Os componentes do frmPlayer sem o estado, para o código fora dos handlers
/// (`FrmPlayer::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPlayerComponents {
    pub form: Form,
    pub mnuPrincipal: MainMenu,
    pub mnuArquivo: MenuItem,
    pub mnuAbrir: MenuItem,
    pub mnuLimparFila: MenuItem,
    pub mnuSep1: MenuItem,
    pub mnuSair: MenuItem,
    pub mnuReproduzir: MenuItem,
    pub mnuReproduzirPausar: MenuItem,
    pub mnuParar: MenuItem,
    pub mnuAnterior: MenuItem,
    pub mnuProxima: MenuItem,
    pub mnuSep2: MenuItem,
    pub mnuEmbaralhar: MenuItem,
    pub mnuRepetir: MenuItem,
    pub mnuExibir: MenuItem,
    pub mnuAgora: MenuItem,
    pub mnuFila: MenuItem,
    pub pnlControles: Panel,
    pub lytBarra: Layout,
    pub lblTempo: Label,
    pub lblDuracao: Label,
    pub sbPosicao: SeekBar,
    pub lytInfo: Layout,
    pub icoCapa: Icon,
    pub lytTextos: Layout,
    pub lblTitulo: Label,
    pub lblDetalhe: Label,
    pub lytVolume: Layout,
    pub btnMudo: Button,
    pub tbVolume: TrackBar,
    pub lytBotoes: Layout,
    pub lytBotoesCentro: Layout,
    pub btnEmbaralhar: Button,
    pub btnAnterior: Button,
    pub btnReproduzir: Button,
    pub btnProxima: Button,
    pub btnRepetir: Button,
    pub lytNavegacao: Layout,
    pub btnAgora: Button,
    pub btnFila: Button,
    pub divNavegacao: Divider,
    pub btnAbrirArquivos: Button,
    pub pnlAgora: Panel,
    pub vpVideo: VideoPlayer,
    pub pnlFila: Panel,
    pub lblFila: Label,
    pub lytFilaBotoes: Layout,
    pub btnAdicionar: Button,
    pub btnLimparFila: Button,
    pub lvFila: ListView,
    pub dlgAbrir: OpenDialog,
    pub tmrPosicao: Timer,
}

impl std::ops::Deref for FrmPlayer {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmPlayer {
    /// Monta a instância de um frmPlayer recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            mnuPrincipal: MainMenu { id: handles::resolve(ui, root, "mnuPrincipal", "MainMenu", &mut problems) },
            mnuArquivo: MenuItem { id: handles::resolve(ui, root, "mnuArquivo", "MenuItem", &mut problems) },
            mnuAbrir: MenuItem { id: handles::resolve(ui, root, "mnuAbrir", "MenuItem", &mut problems) },
            mnuLimparFila: MenuItem { id: handles::resolve(ui, root, "mnuLimparFila", "MenuItem", &mut problems) },
            mnuSep1: MenuItem { id: handles::resolve(ui, root, "mnuSep1", "MenuItem", &mut problems) },
            mnuSair: MenuItem { id: handles::resolve(ui, root, "mnuSair", "MenuItem", &mut problems) },
            mnuReproduzir: MenuItem { id: handles::resolve(ui, root, "mnuReproduzir", "MenuItem", &mut problems) },
            mnuReproduzirPausar: MenuItem { id: handles::resolve(ui, root, "mnuReproduzirPausar", "MenuItem", &mut problems) },
            mnuParar: MenuItem { id: handles::resolve(ui, root, "mnuParar", "MenuItem", &mut problems) },
            mnuAnterior: MenuItem { id: handles::resolve(ui, root, "mnuAnterior", "MenuItem", &mut problems) },
            mnuProxima: MenuItem { id: handles::resolve(ui, root, "mnuProxima", "MenuItem", &mut problems) },
            mnuSep2: MenuItem { id: handles::resolve(ui, root, "mnuSep2", "MenuItem", &mut problems) },
            mnuEmbaralhar: MenuItem { id: handles::resolve(ui, root, "mnuEmbaralhar", "MenuItem", &mut problems) },
            mnuRepetir: MenuItem { id: handles::resolve(ui, root, "mnuRepetir", "MenuItem", &mut problems) },
            mnuExibir: MenuItem { id: handles::resolve(ui, root, "mnuExibir", "MenuItem", &mut problems) },
            mnuAgora: MenuItem { id: handles::resolve(ui, root, "mnuAgora", "MenuItem", &mut problems) },
            mnuFila: MenuItem { id: handles::resolve(ui, root, "mnuFila", "MenuItem", &mut problems) },
            pnlControles: Panel { id: handles::resolve(ui, root, "pnlControles", "Panel", &mut problems) },
            lytBarra: Layout { id: handles::resolve(ui, root, "lytBarra", "Layout", &mut problems) },
            lblTempo: Label { id: handles::resolve(ui, root, "lblTempo", "Label", &mut problems) },
            lblDuracao: Label { id: handles::resolve(ui, root, "lblDuracao", "Label", &mut problems) },
            sbPosicao: SeekBar { id: handles::resolve(ui, root, "sbPosicao", "SeekBar", &mut problems) },
            lytInfo: Layout { id: handles::resolve(ui, root, "lytInfo", "Layout", &mut problems) },
            icoCapa: Icon { id: handles::resolve(ui, root, "icoCapa", "Icon", &mut problems) },
            lytTextos: Layout { id: handles::resolve(ui, root, "lytTextos", "Layout", &mut problems) },
            lblTitulo: Label { id: handles::resolve(ui, root, "lblTitulo", "Label", &mut problems) },
            lblDetalhe: Label { id: handles::resolve(ui, root, "lblDetalhe", "Label", &mut problems) },
            lytVolume: Layout { id: handles::resolve(ui, root, "lytVolume", "Layout", &mut problems) },
            btnMudo: Button { id: handles::resolve(ui, root, "btnMudo", "Button", &mut problems) },
            tbVolume: TrackBar { id: handles::resolve(ui, root, "tbVolume", "TrackBar", &mut problems) },
            lytBotoes: Layout { id: handles::resolve(ui, root, "lytBotoes", "Layout", &mut problems) },
            lytBotoesCentro: Layout { id: handles::resolve(ui, root, "lytBotoesCentro", "Layout", &mut problems) },
            btnEmbaralhar: Button { id: handles::resolve(ui, root, "btnEmbaralhar", "Button", &mut problems) },
            btnAnterior: Button { id: handles::resolve(ui, root, "btnAnterior", "Button", &mut problems) },
            btnReproduzir: Button { id: handles::resolve(ui, root, "btnReproduzir", "Button", &mut problems) },
            btnProxima: Button { id: handles::resolve(ui, root, "btnProxima", "Button", &mut problems) },
            btnRepetir: Button { id: handles::resolve(ui, root, "btnRepetir", "Button", &mut problems) },
            lytNavegacao: Layout { id: handles::resolve(ui, root, "lytNavegacao", "Layout", &mut problems) },
            btnAgora: Button { id: handles::resolve(ui, root, "btnAgora", "Button", &mut problems) },
            btnFila: Button { id: handles::resolve(ui, root, "btnFila", "Button", &mut problems) },
            divNavegacao: Divider { id: handles::resolve(ui, root, "divNavegacao", "Divider", &mut problems) },
            btnAbrirArquivos: Button { id: handles::resolve(ui, root, "btnAbrirArquivos", "Button", &mut problems) },
            pnlAgora: Panel { id: handles::resolve(ui, root, "pnlAgora", "Panel", &mut problems) },
            vpVideo: VideoPlayer { id: handles::resolve(ui, root, "vpVideo", "VideoPlayer", &mut problems) },
            pnlFila: Panel { id: handles::resolve(ui, root, "pnlFila", "Panel", &mut problems) },
            lblFila: Label { id: handles::resolve(ui, root, "lblFila", "Label", &mut problems) },
            lytFilaBotoes: Layout { id: handles::resolve(ui, root, "lytFilaBotoes", "Layout", &mut problems) },
            btnAdicionar: Button { id: handles::resolve(ui, root, "btnAdicionar", "Button", &mut problems) },
            btnLimparFila: Button { id: handles::resolve(ui, root, "btnLimparFila", "Button", &mut problems) },
            lvFila: ListView { id: handles::resolve(ui, root, "lvFila", "ListView", &mut problems) },
            dlgAbrir: OpenDialog { id: handles::resolve(ui, root, "dlgAbrir", "OpenDialog", &mut problems) },
            tmrPosicao: Timer { id: handles::resolve(ui, root, "tmrPosicao", "Timer", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPlayer não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPlayerComponents {
            form: instance.form,
            mnuPrincipal: instance.mnuPrincipal,
            mnuArquivo: instance.mnuArquivo,
            mnuAbrir: instance.mnuAbrir,
            mnuLimparFila: instance.mnuLimparFila,
            mnuSep1: instance.mnuSep1,
            mnuSair: instance.mnuSair,
            mnuReproduzir: instance.mnuReproduzir,
            mnuReproduzirPausar: instance.mnuReproduzirPausar,
            mnuParar: instance.mnuParar,
            mnuAnterior: instance.mnuAnterior,
            mnuProxima: instance.mnuProxima,
            mnuSep2: instance.mnuSep2,
            mnuEmbaralhar: instance.mnuEmbaralhar,
            mnuRepetir: instance.mnuRepetir,
            mnuExibir: instance.mnuExibir,
            mnuAgora: instance.mnuAgora,
            mnuFila: instance.mnuFila,
            pnlControles: instance.pnlControles,
            lytBarra: instance.lytBarra,
            lblTempo: instance.lblTempo,
            lblDuracao: instance.lblDuracao,
            sbPosicao: instance.sbPosicao,
            lytInfo: instance.lytInfo,
            icoCapa: instance.icoCapa,
            lytTextos: instance.lytTextos,
            lblTitulo: instance.lblTitulo,
            lblDetalhe: instance.lblDetalhe,
            lytVolume: instance.lytVolume,
            btnMudo: instance.btnMudo,
            tbVolume: instance.tbVolume,
            lytBotoes: instance.lytBotoes,
            lytBotoesCentro: instance.lytBotoesCentro,
            btnEmbaralhar: instance.btnEmbaralhar,
            btnAnterior: instance.btnAnterior,
            btnReproduzir: instance.btnReproduzir,
            btnProxima: instance.btnProxima,
            btnRepetir: instance.btnRepetir,
            lytNavegacao: instance.lytNavegacao,
            btnAgora: instance.btnAgora,
            btnFila: instance.btnFila,
            divNavegacao: instance.divNavegacao,
            btnAbrirArquivos: instance.btnAbrirArquivos,
            pnlAgora: instance.pnlAgora,
            vpVideo: instance.vpVideo,
            pnlFila: instance.pnlFila,
            lblFila: instance.lblFila,
            lytFilaBotoes: instance.lytFilaBotoes,
            btnAdicionar: instance.btnAdicionar,
            btnLimparFila: instance.btnLimparFila,
            lvFila: instance.lvFila,
            dlgAbrir: instance.dlgAbrir,
            tmrPosicao: instance.tmrPosicao,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPlayer aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmPlayerComponents> {
        ui.form_components(ui.find_path("frmPlayer")?)
    }

    /// Cria o frmPlayer (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmPlayer")
    }

    /// Cria mais um frmPlayer dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmPlayer`, `frmPlayer2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmPlayerComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmPlayer")?;
        let name = ui.free_name_in(parent, "frmPlayer");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmPlayer (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmPlayerComponents> {
        ui.form_components(root)
    }

    /// O frmPlayer aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmPlayer> {
        ui.form_instance(ui.find_path("frmPlayer")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPlayer> {
        let form = ui.find_path("frmPlayer")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnMudo" => Some(self.btnMudo),
            "btnEmbaralhar" => Some(self.btnEmbaralhar),
            "btnAnterior" => Some(self.btnAnterior),
            "btnReproduzir" => Some(self.btnReproduzir),
            "btnProxima" => Some(self.btnProxima),
            "btnRepetir" => Some(self.btnRepetir),
            "btnAgora" => Some(self.btnAgora),
            "btnFila" => Some(self.btnFila),
            "btnAbrirArquivos" => Some(self.btnAbrirArquivos),
            "btnAdicionar" => Some(self.btnAdicionar),
            "btnLimparFila" => Some(self.btnLimparFila),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Divider deste form com o nome (`None` se não há um Divider com ele).
    pub fn find_divider(&self, name: &str) -> Option<Divider> {
        match name {
            "divNavegacao" => Some(self.divNavegacao),
            _ => None,
        }
    }

    /// O Divider que disparou o evento (`None` se o `sender` não é um Divider deste form).
    pub fn sender_as_divider(&self, ui: &Ui, sender: ComponentId) -> Option<Divider> {
        self.find_divider(ui.name(sender)?)
    }

    /// O Icon deste form com o nome (`None` se não há um Icon com ele).
    pub fn find_icon(&self, name: &str) -> Option<Icon> {
        match name {
            "icoCapa" => Some(self.icoCapa),
            _ => None,
        }
    }

    /// O Icon que disparou o evento (`None` se o `sender` não é um Icon deste form).
    pub fn sender_as_icon(&self, ui: &Ui, sender: ComponentId) -> Option<Icon> {
        self.find_icon(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTempo" => Some(self.lblTempo),
            "lblDuracao" => Some(self.lblDuracao),
            "lblTitulo" => Some(self.lblTitulo),
            "lblDetalhe" => Some(self.lblDetalhe),
            "lblFila" => Some(self.lblFila),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Layout deste form com o nome (`None` se não há um Layout com ele).
    pub fn find_layout(&self, name: &str) -> Option<Layout> {
        match name {
            "lytBarra" => Some(self.lytBarra),
            "lytInfo" => Some(self.lytInfo),
            "lytTextos" => Some(self.lytTextos),
            "lytVolume" => Some(self.lytVolume),
            "lytBotoes" => Some(self.lytBotoes),
            "lytBotoesCentro" => Some(self.lytBotoesCentro),
            "lytNavegacao" => Some(self.lytNavegacao),
            "lytFilaBotoes" => Some(self.lytFilaBotoes),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O ListView deste form com o nome (`None` se não há um ListView com ele).
    pub fn find_list_view(&self, name: &str) -> Option<ListView> {
        match name {
            "lvFila" => Some(self.lvFila),
            _ => None,
        }
    }

    /// O ListView que disparou o evento (`None` se o `sender` não é um ListView deste form).
    pub fn sender_as_list_view(&self, ui: &Ui, sender: ComponentId) -> Option<ListView> {
        self.find_list_view(ui.name(sender)?)
    }

    /// O MainMenu deste form com o nome (`None` se não há um MainMenu com ele).
    pub fn find_main_menu(&self, name: &str) -> Option<MainMenu> {
        match name {
            "mnuPrincipal" => Some(self.mnuPrincipal),
            _ => None,
        }
    }

    /// O MainMenu que disparou o evento (`None` se o `sender` não é um MainMenu deste form).
    pub fn sender_as_main_menu(&self, ui: &Ui, sender: ComponentId) -> Option<MainMenu> {
        self.find_main_menu(ui.name(sender)?)
    }

    /// O MenuItem deste form com o nome (`None` se não há um MenuItem com ele).
    pub fn find_menu_item(&self, name: &str) -> Option<MenuItem> {
        match name {
            "mnuArquivo" => Some(self.mnuArquivo),
            "mnuAbrir" => Some(self.mnuAbrir),
            "mnuLimparFila" => Some(self.mnuLimparFila),
            "mnuSep1" => Some(self.mnuSep1),
            "mnuSair" => Some(self.mnuSair),
            "mnuReproduzir" => Some(self.mnuReproduzir),
            "mnuReproduzirPausar" => Some(self.mnuReproduzirPausar),
            "mnuParar" => Some(self.mnuParar),
            "mnuAnterior" => Some(self.mnuAnterior),
            "mnuProxima" => Some(self.mnuProxima),
            "mnuSep2" => Some(self.mnuSep2),
            "mnuEmbaralhar" => Some(self.mnuEmbaralhar),
            "mnuRepetir" => Some(self.mnuRepetir),
            "mnuExibir" => Some(self.mnuExibir),
            "mnuAgora" => Some(self.mnuAgora),
            "mnuFila" => Some(self.mnuFila),
            _ => None,
        }
    }

    /// O MenuItem que disparou o evento (`None` se o `sender` não é um MenuItem deste form).
    pub fn sender_as_menu_item(&self, ui: &Ui, sender: ComponentId) -> Option<MenuItem> {
        self.find_menu_item(ui.name(sender)?)
    }

    /// O OpenDialog deste form com o nome (`None` se não há um OpenDialog com ele).
    pub fn find_open_dialog(&self, name: &str) -> Option<OpenDialog> {
        match name {
            "dlgAbrir" => Some(self.dlgAbrir),
            _ => None,
        }
    }

    /// O OpenDialog que disparou o evento (`None` se o `sender` não é um OpenDialog deste form).
    pub fn sender_as_open_dialog(&self, ui: &Ui, sender: ComponentId) -> Option<OpenDialog> {
        self.find_open_dialog(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlControles" => Some(self.pnlControles),
            "pnlAgora" => Some(self.pnlAgora),
            "pnlFila" => Some(self.pnlFila),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O SeekBar deste form com o nome (`None` se não há um SeekBar com ele).
    pub fn find_seek_bar(&self, name: &str) -> Option<SeekBar> {
        match name {
            "sbPosicao" => Some(self.sbPosicao),
            _ => None,
        }
    }

    /// O SeekBar que disparou o evento (`None` se o `sender` não é um SeekBar deste form).
    pub fn sender_as_seek_bar(&self, ui: &Ui, sender: ComponentId) -> Option<SeekBar> {
        self.find_seek_bar(ui.name(sender)?)
    }

    /// O Timer deste form com o nome (`None` se não há um Timer com ele).
    pub fn find_timer(&self, name: &str) -> Option<Timer> {
        match name {
            "tmrPosicao" => Some(self.tmrPosicao),
            _ => None,
        }
    }

    /// O Timer que disparou o evento (`None` se o `sender` não é um Timer deste form).
    pub fn sender_as_timer(&self, ui: &Ui, sender: ComponentId) -> Option<Timer> {
        self.find_timer(ui.name(sender)?)
    }

    /// O TrackBar deste form com o nome (`None` se não há um TrackBar com ele).
    pub fn find_track_bar(&self, name: &str) -> Option<TrackBar> {
        match name {
            "tbVolume" => Some(self.tbVolume),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
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
