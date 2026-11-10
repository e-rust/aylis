
/// O form frmFisBeneficiosFiscaisHist: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmFisBeneficiosFiscaisHist {
    pub form: Form,
    pub cdsItens: ClientDataSet,
    pub cdsItensId: Field,
    pub cdsItensMestreId: Field,
    pub cdsItensData: Field,
    pub cdsItensDescricao: Field,
    pub cdsItensResponsavel: Field,
    pub dsItens: DataSource,
    pub pnlBarra: Panel,
    pub edtBusca: Edit,
    pub lblStatus: Label,
    pub grdItens: DataGrid,
    pub data: super::fis_beneficios_fiscais_hist::Data,
}

/// Os componentes do frmFisBeneficiosFiscaisHist sem o estado, para o código fora dos handlers
/// (`FrmFisBeneficiosFiscaisHist::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmFisBeneficiosFiscaisHistComponents {
    pub form: Form,
    pub cdsItens: ClientDataSet,
    pub cdsItensId: Field,
    pub cdsItensMestreId: Field,
    pub cdsItensData: Field,
    pub cdsItensDescricao: Field,
    pub cdsItensResponsavel: Field,
    pub dsItens: DataSource,
    pub pnlBarra: Panel,
    pub edtBusca: Edit,
    pub lblStatus: Label,
    pub grdItens: DataGrid,
}

impl std::ops::Deref for FrmFisBeneficiosFiscaisHist {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmFisBeneficiosFiscaisHist {
    /// Monta a instância de um frmFisBeneficiosFiscaisHist recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            cdsItens: ClientDataSet { id: handles::resolve(ui, root, "cdsItens", "ClientDataSet", &mut problems) },
            cdsItensId: Field { id: handles::resolve(ui, root, "cdsItensId", "Field", &mut problems) },
            cdsItensMestreId: Field { id: handles::resolve(ui, root, "cdsItensMestreId", "Field", &mut problems) },
            cdsItensData: Field { id: handles::resolve(ui, root, "cdsItensData", "Field", &mut problems) },
            cdsItensDescricao: Field { id: handles::resolve(ui, root, "cdsItensDescricao", "Field", &mut problems) },
            cdsItensResponsavel: Field { id: handles::resolve(ui, root, "cdsItensResponsavel", "Field", &mut problems) },
            dsItens: DataSource { id: handles::resolve(ui, root, "dsItens", "DataSource", &mut problems) },
            pnlBarra: Panel { id: handles::resolve(ui, root, "pnlBarra", "Panel", &mut problems) },
            edtBusca: Edit { id: handles::resolve(ui, root, "edtBusca", "Edit", &mut problems) },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            grdItens: DataGrid { id: handles::resolve(ui, root, "grdItens", "DataGrid", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmFisBeneficiosFiscaisHist não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmFisBeneficiosFiscaisHistComponents {
            form: instance.form,
            cdsItens: instance.cdsItens,
            cdsItensId: instance.cdsItensId,
            cdsItensMestreId: instance.cdsItensMestreId,
            cdsItensData: instance.cdsItensData,
            cdsItensDescricao: instance.cdsItensDescricao,
            cdsItensResponsavel: instance.cdsItensResponsavel,
            dsItens: instance.dsItens,
            pnlBarra: instance.pnlBarra,
            edtBusca: instance.edtBusca,
            lblStatus: instance.lblStatus,
            grdItens: instance.grdItens,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmFisBeneficiosFiscaisHist aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmFisBeneficiosFiscaisHistComponents> {
        ui.form_components(ui.find_path("frmFisBeneficiosFiscaisHist")?)
    }

    /// Cria o frmFisBeneficiosFiscaisHist (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmFisBeneficiosFiscaisHist")
    }

    /// Cria mais um frmFisBeneficiosFiscaisHist dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmFisBeneficiosFiscaisHist`, `frmFisBeneficiosFiscaisHist2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmFisBeneficiosFiscaisHistComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmFisBeneficiosFiscaisHist")?;
        let name = ui.free_name_in(parent, "frmFisBeneficiosFiscaisHist");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmFisBeneficiosFiscaisHist (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmFisBeneficiosFiscaisHistComponents> {
        ui.form_components(root)
    }

    /// O frmFisBeneficiosFiscaisHist aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmFisBeneficiosFiscaisHist> {
        ui.form_instance(ui.find_path("frmFisBeneficiosFiscaisHist")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmFisBeneficiosFiscaisHist> {
        let form = ui.find_path("frmFisBeneficiosFiscaisHist")?;
        ui.form_instance_mut(form)
    }

    /// O ClientDataSet deste form com o nome (`None` se não há um ClientDataSet com ele).
    pub fn find_client_data_set(&self, name: &str) -> Option<ClientDataSet> {
        match name {
            "cdsItens" => Some(self.cdsItens),
            _ => None,
        }
    }

    /// O ClientDataSet que disparou o evento (`None` se o `sender` não é um ClientDataSet deste form).
    pub fn sender_as_client_data_set(&self, ui: &Ui, sender: ComponentId) -> Option<ClientDataSet> {
        self.find_client_data_set(ui.name(sender)?)
    }

    /// O DataGrid deste form com o nome (`None` se não há um DataGrid com ele).
    pub fn find_data_grid(&self, name: &str) -> Option<DataGrid> {
        match name {
            "grdItens" => Some(self.grdItens),
            _ => None,
        }
    }

    /// O DataGrid que disparou o evento (`None` se o `sender` não é um DataGrid deste form).
    pub fn sender_as_data_grid(&self, ui: &Ui, sender: ComponentId) -> Option<DataGrid> {
        self.find_data_grid(ui.name(sender)?)
    }

    /// O DataSource deste form com o nome (`None` se não há um DataSource com ele).
    pub fn find_data_source(&self, name: &str) -> Option<DataSource> {
        match name {
            "dsItens" => Some(self.dsItens),
            _ => None,
        }
    }

    /// O DataSource que disparou o evento (`None` se o `sender` não é um DataSource deste form).
    pub fn sender_as_data_source(&self, ui: &Ui, sender: ComponentId) -> Option<DataSource> {
        self.find_data_source(ui.name(sender)?)
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

    /// O Field deste form com o nome (`None` se não há um Field com ele).
    pub fn find_field(&self, name: &str) -> Option<Field> {
        match name {
            "cdsItensId" => Some(self.cdsItensId),
            "cdsItensMestreId" => Some(self.cdsItensMestreId),
            "cdsItensData" => Some(self.cdsItensData),
            "cdsItensDescricao" => Some(self.cdsItensDescricao),
            "cdsItensResponsavel" => Some(self.cdsItensResponsavel),
            _ => None,
        }
    }

    /// O Field que disparou o evento (`None` se o `sender` não é um Field deste form).
    pub fn sender_as_field(&self, ui: &Ui, sender: ComponentId) -> Option<Field> {
        self.find_field(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblStatus" => Some(self.lblStatus),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlBarra" => Some(self.pnlBarra),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
