
/// O form frmPrincipal: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrincipal {
    pub form: Form,
    pub pnlTopo: Panel,
    pub segNivel: SegmentedControl,
    pub fpsMedidor: FpsMeter,
    pub lblInfo: Label,
    pub grdLingotes: GridLayout,
    pub m3dLingote: Model3D,
    pub tmrAjuste: Timer,
    pub data: super::principal::Data,
}

/// Os componentes do frmPrincipal sem o estado, para o código fora dos handlers
/// (`FrmPrincipal::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrincipalComponents {
    pub form: Form,
    pub pnlTopo: Panel,
    pub segNivel: SegmentedControl,
    pub fpsMedidor: FpsMeter,
    pub lblInfo: Label,
    pub grdLingotes: GridLayout,
    pub m3dLingote: Model3D,
    pub tmrAjuste: Timer,
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
            pnlTopo: Panel { id: handles::resolve(ui, root, "pnlTopo", "Panel", &mut problems) },
            segNivel: SegmentedControl { id: handles::resolve(ui, root, "segNivel", "SegmentedControl", &mut problems) },
            fpsMedidor: FpsMeter { id: handles::resolve(ui, root, "fpsMedidor", "FpsMeter", &mut problems) },
            lblInfo: Label { id: handles::resolve(ui, root, "lblInfo", "Label", &mut problems) },
            grdLingotes: GridLayout { id: handles::resolve(ui, root, "grdLingotes", "GridLayout", &mut problems) },
            m3dLingote: Model3D { id: handles::resolve(ui, root, "m3dLingote", "Model3D", &mut problems) },
            tmrAjuste: Timer { id: handles::resolve(ui, root, "tmrAjuste", "Timer", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrincipal não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrincipalComponents {
            form: instance.form,
            pnlTopo: instance.pnlTopo,
            segNivel: instance.segNivel,
            fpsMedidor: instance.fpsMedidor,
            lblInfo: instance.lblInfo,
            grdLingotes: instance.grdLingotes,
            m3dLingote: instance.m3dLingote,
            tmrAjuste: instance.tmrAjuste,
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
            "grdLingotes" => Some(self.grdLingotes),
            _ => None,
        }
    }

    /// O GridLayout que disparou o evento (`None` se o `sender` não é um GridLayout deste form).
    pub fn sender_as_grid_layout(&self, ui: &Ui, sender: ComponentId) -> Option<GridLayout> {
        self.find_grid_layout(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblInfo" => Some(self.lblInfo),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Model3D deste form com o nome (`None` se não há um Model3D com ele).
    pub fn find_model3_d(&self, name: &str) -> Option<Model3D> {
        match name {
            "m3dLingote" => Some(self.m3dLingote),
            _ => None,
        }
    }

    /// O Model3D que disparou o evento (`None` se o `sender` não é um Model3D deste form).
    pub fn sender_as_model3_d(&self, ui: &Ui, sender: ComponentId) -> Option<Model3D> {
        self.find_model3_d(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlTopo" => Some(self.pnlTopo),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O SegmentedControl deste form com o nome (`None` se não há um SegmentedControl com ele).
    pub fn find_segmented_control(&self, name: &str) -> Option<SegmentedControl> {
        match name {
            "segNivel" => Some(self.segNivel),
            _ => None,
        }
    }

    /// O SegmentedControl que disparou o evento (`None` se o `sender` não é um SegmentedControl deste form).
    pub fn sender_as_segmented_control(&self, ui: &Ui, sender: ComponentId) -> Option<SegmentedControl> {
        self.find_segmented_control(ui.name(sender)?)
    }

    /// O Timer deste form com o nome (`None` se não há um Timer com ele).
    pub fn find_timer(&self, name: &str) -> Option<Timer> {
        match name {
            "tmrAjuste" => Some(self.tmrAjuste),
            _ => None,
        }
    }

    /// O Timer que disparou o evento (`None` se o `sender` não é um Timer deste form).
    pub fn sender_as_timer(&self, ui: &Ui, sender: ComponentId) -> Option<Timer> {
        self.find_timer(ui.name(sender)?)
    }
}
