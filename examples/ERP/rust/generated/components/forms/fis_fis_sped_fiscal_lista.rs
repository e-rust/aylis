
/// O form frmFisSpedFiscalLista: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmFisSpedFiscalLista {
    pub form: Form,
    pub cdsMestre: ClientDataSet,
    pub cdsMestreId: Field,
    pub cdsMestreNumero: Field,
    pub cdsMestreData: Field,
    pub cdsMestreParceiro: Field,
    pub cdsMestreValorTotal: Field,
    pub cdsMestreSituacao: Field,
    pub cdsMestreObservacao: Field,
    pub dsMestre: DataSource,
    pub pnlBarra: Panel,
    pub btnAbrir: Button,
    pub edtBusca: Edit,
    pub lblStatus: Label,
    pub grdLista: DataGrid,
    pub data: super::fis_fis_sped_fiscal_lista::Data,
}

/// Os componentes do frmFisSpedFiscalLista sem o estado, para o código fora dos handlers
/// (`FrmFisSpedFiscalLista::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmFisSpedFiscalListaComponents {
    pub form: Form,
    pub cdsMestre: ClientDataSet,
    pub cdsMestreId: Field,
    pub cdsMestreNumero: Field,
    pub cdsMestreData: Field,
    pub cdsMestreParceiro: Field,
    pub cdsMestreValorTotal: Field,
    pub cdsMestreSituacao: Field,
    pub cdsMestreObservacao: Field,
    pub dsMestre: DataSource,
    pub pnlBarra: Panel,
    pub btnAbrir: Button,
    pub edtBusca: Edit,
    pub lblStatus: Label,
    pub grdLista: DataGrid,
}

impl std::ops::Deref for FrmFisSpedFiscalLista {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmFisSpedFiscalLista {
    /// Monta a instância de um frmFisSpedFiscalLista recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            cdsMestre: ClientDataSet { id: handles::resolve(ui, root, "cdsMestre", "ClientDataSet", &mut problems) },
            cdsMestreId: Field { id: handles::resolve(ui, root, "cdsMestreId", "Field", &mut problems) },
            cdsMestreNumero: Field { id: handles::resolve(ui, root, "cdsMestreNumero", "Field", &mut problems) },
            cdsMestreData: Field { id: handles::resolve(ui, root, "cdsMestreData", "Field", &mut problems) },
            cdsMestreParceiro: Field { id: handles::resolve(ui, root, "cdsMestreParceiro", "Field", &mut problems) },
            cdsMestreValorTotal: Field { id: handles::resolve(ui, root, "cdsMestreValorTotal", "Field", &mut problems) },
            cdsMestreSituacao: Field { id: handles::resolve(ui, root, "cdsMestreSituacao", "Field", &mut problems) },
            cdsMestreObservacao: Field { id: handles::resolve(ui, root, "cdsMestreObservacao", "Field", &mut problems) },
            dsMestre: DataSource { id: handles::resolve(ui, root, "dsMestre", "DataSource", &mut problems) },
            pnlBarra: Panel { id: handles::resolve(ui, root, "pnlBarra", "Panel", &mut problems) },
            btnAbrir: Button { id: handles::resolve(ui, root, "btnAbrir", "Button", &mut problems) },
            edtBusca: Edit { id: handles::resolve(ui, root, "edtBusca", "Edit", &mut problems) },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            grdLista: DataGrid { id: handles::resolve(ui, root, "grdLista", "DataGrid", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmFisSpedFiscalLista não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmFisSpedFiscalListaComponents {
            form: instance.form,
            cdsMestre: instance.cdsMestre,
            cdsMestreId: instance.cdsMestreId,
            cdsMestreNumero: instance.cdsMestreNumero,
            cdsMestreData: instance.cdsMestreData,
            cdsMestreParceiro: instance.cdsMestreParceiro,
            cdsMestreValorTotal: instance.cdsMestreValorTotal,
            cdsMestreSituacao: instance.cdsMestreSituacao,
            cdsMestreObservacao: instance.cdsMestreObservacao,
            dsMestre: instance.dsMestre,
            pnlBarra: instance.pnlBarra,
            btnAbrir: instance.btnAbrir,
            edtBusca: instance.edtBusca,
            lblStatus: instance.lblStatus,
            grdLista: instance.grdLista,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmFisSpedFiscalLista aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmFisSpedFiscalListaComponents> {
        ui.form_components(ui.find_path("frmFisSpedFiscalLista")?)
    }

    /// Cria o frmFisSpedFiscalLista (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmFisSpedFiscalLista")
    }

    /// Cria mais um frmFisSpedFiscalLista dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmFisSpedFiscalLista`, `frmFisSpedFiscalLista2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmFisSpedFiscalListaComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmFisSpedFiscalLista")?;
        let name = ui.free_name_in(parent, "frmFisSpedFiscalLista");
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

    /// Os componentes de uma instância do frmFisSpedFiscalLista (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmFisSpedFiscalListaComponents> {
        ui.form_components(root)
    }

    /// O frmFisSpedFiscalLista aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmFisSpedFiscalLista> {
        ui.form_instance(ui.find_path("frmFisSpedFiscalLista")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmFisSpedFiscalLista> {
        let form = ui.find_path("frmFisSpedFiscalLista")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnAbrir" => Some(self.btnAbrir),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O ClientDataSet deste form com o nome (`None` se não há um ClientDataSet com ele).
    pub fn find_client_data_set(&self, name: &str) -> Option<ClientDataSet> {
        match name {
            "cdsMestre" => Some(self.cdsMestre),
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
            "grdLista" => Some(self.grdLista),
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
            "dsMestre" => Some(self.dsMestre),
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
            "cdsMestreId" => Some(self.cdsMestreId),
            "cdsMestreNumero" => Some(self.cdsMestreNumero),
            "cdsMestreData" => Some(self.cdsMestreData),
            "cdsMestreParceiro" => Some(self.cdsMestreParceiro),
            "cdsMestreValorTotal" => Some(self.cdsMestreValorTotal),
            "cdsMestreSituacao" => Some(self.cdsMestreSituacao),
            "cdsMestreObservacao" => Some(self.cdsMestreObservacao),
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
