
/// O form frmPrincipal: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrincipal {
    pub form: Form,
    pub pnlTopo: Layout,
    pub lblTitulo: Label,
    pub lblConta: Label,
    pub edtBusca: Edit,
    pub scrCartoes: ScrollBox,
    pub fluCartoes: FlowLayout,
    pub data: super::principal::Data,
}

/// Os componentes do frmPrincipal sem o estado, para o código fora dos handlers
/// (`FrmPrincipal::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrincipalComponents {
    pub form: Form,
    pub pnlTopo: Layout,
    pub lblTitulo: Label,
    pub lblConta: Label,
    pub edtBusca: Edit,
    pub scrCartoes: ScrollBox,
    pub fluCartoes: FlowLayout,
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
            pnlTopo: Layout { id: handles::resolve(ui, root, "pnlTopo", "Layout", &mut problems) },
            lblTitulo: Label { id: handles::resolve(ui, root, "lblTitulo", "Label", &mut problems) },
            lblConta: Label { id: handles::resolve(ui, root, "lblConta", "Label", &mut problems) },
            edtBusca: Edit { id: handles::resolve(ui, root, "edtBusca", "Edit", &mut problems) },
            scrCartoes: ScrollBox { id: handles::resolve(ui, root, "scrCartoes", "ScrollBox", &mut problems) },
            fluCartoes: FlowLayout { id: handles::resolve(ui, root, "fluCartoes", "FlowLayout", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrincipal não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrincipalComponents {
            form: instance.form,
            pnlTopo: instance.pnlTopo,
            lblTitulo: instance.lblTitulo,
            lblConta: instance.lblConta,
            edtBusca: instance.edtBusca,
            scrCartoes: instance.scrCartoes,
            fluCartoes: instance.fluCartoes,
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

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edtBusca" => Some(self.edtBusca),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O FlowLayout deste form com o nome (`None` se não há um FlowLayout com ele).
    pub fn find_flow_layout(&self, name: &str) -> Option<FlowLayout> {
        match name {
            "fluCartoes" => Some(self.fluCartoes),
            _ => None,
        }
    }

    /// O FlowLayout que disparou o evento (`None` se o `sender` não é um FlowLayout deste form).
    pub fn sender_as_flow_layout(&self, ui: &Ui, sender: ComponentId) -> Option<FlowLayout> {
        self.find_flow_layout(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTitulo" => Some(self.lblTitulo),
            "lblConta" => Some(self.lblConta),
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
            "pnlTopo" => Some(self.pnlTopo),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O ScrollBox deste form com o nome (`None` se não há um ScrollBox com ele).
    pub fn find_scroll_box(&self, name: &str) -> Option<ScrollBox> {
        match name {
            "scrCartoes" => Some(self.scrCartoes),
            _ => None,
        }
    }

    /// O ScrollBox que disparou o evento (`None` se o `sender` não é um ScrollBox deste form).
    pub fn sender_as_scroll_box(&self, ui: &Ui, sender: ComponentId) -> Option<ScrollBox> {
        self.find_scroll_box(ui.name(sender)?)
    }
}
