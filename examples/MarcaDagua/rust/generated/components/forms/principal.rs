
/// O form frmPrincipal: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrincipal {
    pub form: Form,
    pub layBarra: Layout,
    pub btnAbrir: Button,
    pub btnTocar: Button,
    pub segGrade: SegmentedControl,
    pub fpsMedidor: FpsMeter,
    pub edtTexto: Edit,
    pub chkInverter: CheckBox,
    pub chkMover: CheckBox,
    pub chkVideo: CheckBox,
    pub grdVideos: GridLayout,
    pub layCelula: Layout,
    pub imgQuadro: Image,
    pub vpVideo: VideoPlayer,
    pub layMarca: Layout,
    pub lblMarca: Label,
    pub aniMarca: FloatAnimation,
    pub dlgAbrir: OpenDialog,
    pub data: super::principal::Data,
}

/// Os componentes do frmPrincipal sem o estado, para o código fora dos handlers
/// (`FrmPrincipal::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrincipalComponents {
    pub form: Form,
    pub layBarra: Layout,
    pub btnAbrir: Button,
    pub btnTocar: Button,
    pub segGrade: SegmentedControl,
    pub fpsMedidor: FpsMeter,
    pub edtTexto: Edit,
    pub chkInverter: CheckBox,
    pub chkMover: CheckBox,
    pub chkVideo: CheckBox,
    pub grdVideos: GridLayout,
    pub layCelula: Layout,
    pub imgQuadro: Image,
    pub vpVideo: VideoPlayer,
    pub layMarca: Layout,
    pub lblMarca: Label,
    pub aniMarca: FloatAnimation,
    pub dlgAbrir: OpenDialog,
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
        #[allow(unused_mut)] // um form sem componentes não acrescenta nada
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            layBarra: Layout { id: handles::resolve(ui, root, "layBarra", "Layout", &mut problems) },
            btnAbrir: Button { id: handles::resolve(ui, root, "btnAbrir", "Button", &mut problems) },
            btnTocar: Button { id: handles::resolve(ui, root, "btnTocar", "Button", &mut problems) },
            segGrade: SegmentedControl { id: handles::resolve(ui, root, "segGrade", "SegmentedControl", &mut problems) },
            fpsMedidor: FpsMeter { id: handles::resolve(ui, root, "fpsMedidor", "FpsMeter", &mut problems) },
            edtTexto: Edit { id: handles::resolve(ui, root, "edtTexto", "Edit", &mut problems) },
            chkInverter: CheckBox { id: handles::resolve(ui, root, "chkInverter", "CheckBox", &mut problems) },
            chkMover: CheckBox { id: handles::resolve(ui, root, "chkMover", "CheckBox", &mut problems) },
            chkVideo: CheckBox { id: handles::resolve(ui, root, "chkVideo", "CheckBox", &mut problems) },
            grdVideos: GridLayout { id: handles::resolve(ui, root, "grdVideos", "GridLayout", &mut problems) },
            layCelula: Layout { id: handles::resolve(ui, root, "layCelula", "Layout", &mut problems) },
            imgQuadro: Image { id: handles::resolve(ui, root, "imgQuadro", "Image", &mut problems) },
            vpVideo: VideoPlayer { id: handles::resolve(ui, root, "vpVideo", "VideoPlayer", &mut problems) },
            layMarca: Layout { id: handles::resolve(ui, root, "layMarca", "Layout", &mut problems) },
            lblMarca: Label { id: handles::resolve(ui, root, "lblMarca", "Label", &mut problems) },
            aniMarca: FloatAnimation { id: handles::resolve(ui, root, "aniMarca", "FloatAnimation", &mut problems) },
            dlgAbrir: OpenDialog { id: handles::resolve(ui, root, "dlgAbrir", "OpenDialog", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrincipal não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrincipalComponents {
            form: instance.form,
            layBarra: instance.layBarra,
            btnAbrir: instance.btnAbrir,
            btnTocar: instance.btnTocar,
            segGrade: instance.segGrade,
            fpsMedidor: instance.fpsMedidor,
            edtTexto: instance.edtTexto,
            chkInverter: instance.chkInverter,
            chkMover: instance.chkMover,
            chkVideo: instance.chkVideo,
            grdVideos: instance.grdVideos,
            layCelula: instance.layCelula,
            imgQuadro: instance.imgQuadro,
            vpVideo: instance.vpVideo,
            layMarca: instance.layMarca,
            lblMarca: instance.lblMarca,
            aniMarca: instance.aniMarca,
            dlgAbrir: instance.dlgAbrir,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPrincipal aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`. É o caminho no
    /// código que um handler do próprio frmPrincipal chama (no handler mesmo, o `self`).
    pub fn components(ui: &Ui) -> Option<FrmPrincipalComponents> {
        ui.form_components(ui.find_path("frmPrincipal")?)
    }

    /// Cria o frmPrincipal (ou devolve o que já está criado); mostrar é com
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
    ///
    /// Dentro de um handler do próprio frmPrincipal, dá `None`: a instância está emprestada ao
    /// handler que roda. Ali, use o `self` do handler ou, no código chamado por ele,
    /// [`components`](Self::components) (em debug, o `None` por empréstimo avisa no stderr).
    pub fn get(ui: &Ui) -> Option<&FrmPrincipal> {
        ui.form_instance(ui.find_path("frmPrincipal")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPrincipal> {
        let form = ui.find_path("frmPrincipal")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnAbrir" => Some(self.btnAbrir),
            "btnTocar" => Some(self.btnTocar),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkInverter" => Some(self.chkInverter),
            "chkMover" => Some(self.chkMover),
            "chkVideo" => Some(self.chkVideo),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edtTexto" => Some(self.edtTexto),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O FloatAnimation deste form com o nome (`None` se não há um FloatAnimation com ele).
    pub fn find_float_animation(&self, name: &str) -> Option<FloatAnimation> {
        match name {
            "aniMarca" => Some(self.aniMarca),
            _ => None,
        }
    }

    /// O FloatAnimation que disparou o evento (`None` se o `sender` não é um FloatAnimation deste form).
    pub fn sender_as_float_animation(&self, ui: &Ui, sender: ComponentId) -> Option<FloatAnimation> {
        self.find_float_animation(ui.name(sender)?)
    }

    /// O FpsMeter deste form com o nome (`None` se não há um FpsMeter com ele).
    pub fn find_fps_meter(&self, name: &str) -> Option<FpsMeter> {
        match name {
            "fpsMedidor" => Some(self.fpsMedidor),
            _ => None,
        }
    }

    /// O FpsMeter que disparou o evento (`None` se o `sender` não é um FpsMeter deste form).
    pub fn sender_as_fps_meter(&self, ui: &Ui, sender: ComponentId) -> Option<FpsMeter> {
        self.find_fps_meter(ui.name(sender)?)
    }

    /// O GridLayout deste form com o nome (`None` se não há um GridLayout com ele).
    pub fn find_grid_layout(&self, name: &str) -> Option<GridLayout> {
        match name {
            "grdVideos" => Some(self.grdVideos),
            _ => None,
        }
    }

    /// O GridLayout que disparou o evento (`None` se o `sender` não é um GridLayout deste form).
    pub fn sender_as_grid_layout(&self, ui: &Ui, sender: ComponentId) -> Option<GridLayout> {
        self.find_grid_layout(ui.name(sender)?)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgQuadro" => Some(self.imgQuadro),
            _ => None,
        }
    }

    /// O Image que disparou o evento (`None` se o `sender` não é um Image deste form).
    pub fn sender_as_image(&self, ui: &Ui, sender: ComponentId) -> Option<Image> {
        self.find_image(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblMarca" => Some(self.lblMarca),
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
            "layBarra" => Some(self.layBarra),
            "layCelula" => Some(self.layCelula),
            "layMarca" => Some(self.layMarca),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
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

    /// O SegmentedControl deste form com o nome (`None` se não há um SegmentedControl com ele).
    pub fn find_segmented_control(&self, name: &str) -> Option<SegmentedControl> {
        match name {
            "segGrade" => Some(self.segGrade),
            _ => None,
        }
    }

    /// O SegmentedControl que disparou o evento (`None` se o `sender` não é um SegmentedControl deste form).
    pub fn sender_as_segmented_control(&self, ui: &Ui, sender: ComponentId) -> Option<SegmentedControl> {
        self.find_segmented_control(ui.name(sender)?)
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
