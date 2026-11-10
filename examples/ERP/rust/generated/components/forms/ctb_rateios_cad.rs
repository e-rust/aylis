
/// O form frmCtbRateiosCad: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmCtbRateiosCad {
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
    pub cdsItens: ClientDataSet,
    pub cdsItensId: Field,
    pub cdsItensMestreId: Field,
    pub cdsItensItem: Field,
    pub cdsItensQuantidade: Field,
    pub cdsItensValorUnitario: Field,
    pub cdsItensValorTotal: Field,
    pub dsItens: DataSource,
    pub pnlBarra: Panel,
    pub btnNovo: Button,
    pub btnSalvar: Button,
    pub btnExcluir: Button,
    pub btnCancelar: Button,
    pub btnNovoItem: Button,
    pub btnFechar: Button,
    pub edtBusca: Edit,
    pub lblStatus: Label,
    pub grdLista: DataGrid,
    pub splLista: Splitter,
    pub layDireita: Layout,
    pub gbDados: GroupBox,
    pub layNumero: Layout,
    pub lblNumero: Label,
    pub edtNumero: Edit,
    pub layData: Layout,
    pub lblData: Label,
    pub edtData: Edit,
    pub layParceiro: Layout,
    pub lblParceiro: Label,
    pub edtParceiro: Edit,
    pub layValorTotal: Layout,
    pub lblValorTotal: Label,
    pub edtValorTotal: Edit,
    pub laySituacao: Layout,
    pub lblSituacao: Label,
    pub edtSituacao: Edit,
    pub layObservacao: Layout,
    pub lblObservacao: Label,
    pub edtObservacao: Edit,
    pub gbItens: GroupBox,
    pub grdItens: DataGrid,
    pub data: super::ctb_rateios_cad::Data,
}

/// Os componentes do frmCtbRateiosCad sem o estado, para o código fora dos handlers
/// (`FrmCtbRateiosCad::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmCtbRateiosCadComponents {
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
    pub cdsItens: ClientDataSet,
    pub cdsItensId: Field,
    pub cdsItensMestreId: Field,
    pub cdsItensItem: Field,
    pub cdsItensQuantidade: Field,
    pub cdsItensValorUnitario: Field,
    pub cdsItensValorTotal: Field,
    pub dsItens: DataSource,
    pub pnlBarra: Panel,
    pub btnNovo: Button,
    pub btnSalvar: Button,
    pub btnExcluir: Button,
    pub btnCancelar: Button,
    pub btnNovoItem: Button,
    pub btnFechar: Button,
    pub edtBusca: Edit,
    pub lblStatus: Label,
    pub grdLista: DataGrid,
    pub splLista: Splitter,
    pub layDireita: Layout,
    pub gbDados: GroupBox,
    pub layNumero: Layout,
    pub lblNumero: Label,
    pub edtNumero: Edit,
    pub layData: Layout,
    pub lblData: Label,
    pub edtData: Edit,
    pub layParceiro: Layout,
    pub lblParceiro: Label,
    pub edtParceiro: Edit,
    pub layValorTotal: Layout,
    pub lblValorTotal: Label,
    pub edtValorTotal: Edit,
    pub laySituacao: Layout,
    pub lblSituacao: Label,
    pub edtSituacao: Edit,
    pub layObservacao: Layout,
    pub lblObservacao: Label,
    pub edtObservacao: Edit,
    pub gbItens: GroupBox,
    pub grdItens: DataGrid,
}

impl std::ops::Deref for FrmCtbRateiosCad {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmCtbRateiosCad {
    /// Monta a instância de um frmCtbRateiosCad recém-criado (o `register` liga). Um componente que
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
            cdsItens: ClientDataSet { id: handles::resolve(ui, root, "cdsItens", "ClientDataSet", &mut problems) },
            cdsItensId: Field { id: handles::resolve(ui, root, "cdsItensId", "Field", &mut problems) },
            cdsItensMestreId: Field { id: handles::resolve(ui, root, "cdsItensMestreId", "Field", &mut problems) },
            cdsItensItem: Field { id: handles::resolve(ui, root, "cdsItensItem", "Field", &mut problems) },
            cdsItensQuantidade: Field { id: handles::resolve(ui, root, "cdsItensQuantidade", "Field", &mut problems) },
            cdsItensValorUnitario: Field { id: handles::resolve(ui, root, "cdsItensValorUnitario", "Field", &mut problems) },
            cdsItensValorTotal: Field { id: handles::resolve(ui, root, "cdsItensValorTotal", "Field", &mut problems) },
            dsItens: DataSource { id: handles::resolve(ui, root, "dsItens", "DataSource", &mut problems) },
            pnlBarra: Panel { id: handles::resolve(ui, root, "pnlBarra", "Panel", &mut problems) },
            btnNovo: Button { id: handles::resolve(ui, root, "btnNovo", "Button", &mut problems) },
            btnSalvar: Button { id: handles::resolve(ui, root, "btnSalvar", "Button", &mut problems) },
            btnExcluir: Button { id: handles::resolve(ui, root, "btnExcluir", "Button", &mut problems) },
            btnCancelar: Button { id: handles::resolve(ui, root, "btnCancelar", "Button", &mut problems) },
            btnNovoItem: Button { id: handles::resolve(ui, root, "btnNovoItem", "Button", &mut problems) },
            btnFechar: Button { id: handles::resolve(ui, root, "btnFechar", "Button", &mut problems) },
            edtBusca: Edit { id: handles::resolve(ui, root, "edtBusca", "Edit", &mut problems) },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            grdLista: DataGrid { id: handles::resolve(ui, root, "grdLista", "DataGrid", &mut problems) },
            splLista: Splitter { id: handles::resolve(ui, root, "splLista", "Splitter", &mut problems) },
            layDireita: Layout { id: handles::resolve(ui, root, "layDireita", "Layout", &mut problems) },
            gbDados: GroupBox { id: handles::resolve(ui, root, "gbDados", "GroupBox", &mut problems) },
            layNumero: Layout { id: handles::resolve(ui, root, "layNumero", "Layout", &mut problems) },
            lblNumero: Label { id: handles::resolve(ui, root, "lblNumero", "Label", &mut problems) },
            edtNumero: Edit { id: handles::resolve(ui, root, "edtNumero", "Edit", &mut problems) },
            layData: Layout { id: handles::resolve(ui, root, "layData", "Layout", &mut problems) },
            lblData: Label { id: handles::resolve(ui, root, "lblData", "Label", &mut problems) },
            edtData: Edit { id: handles::resolve(ui, root, "edtData", "Edit", &mut problems) },
            layParceiro: Layout { id: handles::resolve(ui, root, "layParceiro", "Layout", &mut problems) },
            lblParceiro: Label { id: handles::resolve(ui, root, "lblParceiro", "Label", &mut problems) },
            edtParceiro: Edit { id: handles::resolve(ui, root, "edtParceiro", "Edit", &mut problems) },
            layValorTotal: Layout { id: handles::resolve(ui, root, "layValorTotal", "Layout", &mut problems) },
            lblValorTotal: Label { id: handles::resolve(ui, root, "lblValorTotal", "Label", &mut problems) },
            edtValorTotal: Edit { id: handles::resolve(ui, root, "edtValorTotal", "Edit", &mut problems) },
            laySituacao: Layout { id: handles::resolve(ui, root, "laySituacao", "Layout", &mut problems) },
            lblSituacao: Label { id: handles::resolve(ui, root, "lblSituacao", "Label", &mut problems) },
            edtSituacao: Edit { id: handles::resolve(ui, root, "edtSituacao", "Edit", &mut problems) },
            layObservacao: Layout { id: handles::resolve(ui, root, "layObservacao", "Layout", &mut problems) },
            lblObservacao: Label { id: handles::resolve(ui, root, "lblObservacao", "Label", &mut problems) },
            edtObservacao: Edit { id: handles::resolve(ui, root, "edtObservacao", "Edit", &mut problems) },
            gbItens: GroupBox { id: handles::resolve(ui, root, "gbItens", "GroupBox", &mut problems) },
            grdItens: DataGrid { id: handles::resolve(ui, root, "grdItens", "DataGrid", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmCtbRateiosCad não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmCtbRateiosCadComponents {
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
            cdsItens: instance.cdsItens,
            cdsItensId: instance.cdsItensId,
            cdsItensMestreId: instance.cdsItensMestreId,
            cdsItensItem: instance.cdsItensItem,
            cdsItensQuantidade: instance.cdsItensQuantidade,
            cdsItensValorUnitario: instance.cdsItensValorUnitario,
            cdsItensValorTotal: instance.cdsItensValorTotal,
            dsItens: instance.dsItens,
            pnlBarra: instance.pnlBarra,
            btnNovo: instance.btnNovo,
            btnSalvar: instance.btnSalvar,
            btnExcluir: instance.btnExcluir,
            btnCancelar: instance.btnCancelar,
            btnNovoItem: instance.btnNovoItem,
            btnFechar: instance.btnFechar,
            edtBusca: instance.edtBusca,
            lblStatus: instance.lblStatus,
            grdLista: instance.grdLista,
            splLista: instance.splLista,
            layDireita: instance.layDireita,
            gbDados: instance.gbDados,
            layNumero: instance.layNumero,
            lblNumero: instance.lblNumero,
            edtNumero: instance.edtNumero,
            layData: instance.layData,
            lblData: instance.lblData,
            edtData: instance.edtData,
            layParceiro: instance.layParceiro,
            lblParceiro: instance.lblParceiro,
            edtParceiro: instance.edtParceiro,
            layValorTotal: instance.layValorTotal,
            lblValorTotal: instance.lblValorTotal,
            edtValorTotal: instance.edtValorTotal,
            laySituacao: instance.laySituacao,
            lblSituacao: instance.lblSituacao,
            edtSituacao: instance.edtSituacao,
            layObservacao: instance.layObservacao,
            lblObservacao: instance.lblObservacao,
            edtObservacao: instance.edtObservacao,
            gbItens: instance.gbItens,
            grdItens: instance.grdItens,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmCtbRateiosCad aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmCtbRateiosCadComponents> {
        ui.form_components(ui.find_path("frmCtbRateiosCad")?)
    }

    /// Cria o frmCtbRateiosCad (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmCtbRateiosCad")
    }

    /// Cria mais um frmCtbRateiosCad dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmCtbRateiosCad`, `frmCtbRateiosCad2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmCtbRateiosCadComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmCtbRateiosCad")?;
        let name = ui.free_name_in(parent, "frmCtbRateiosCad");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmCtbRateiosCad (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmCtbRateiosCadComponents> {
        ui.form_components(root)
    }

    /// O frmCtbRateiosCad aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmCtbRateiosCad> {
        ui.form_instance(ui.find_path("frmCtbRateiosCad")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmCtbRateiosCad> {
        let form = ui.find_path("frmCtbRateiosCad")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnNovo" => Some(self.btnNovo),
            "btnSalvar" => Some(self.btnSalvar),
            "btnExcluir" => Some(self.btnExcluir),
            "btnCancelar" => Some(self.btnCancelar),
            "btnNovoItem" => Some(self.btnNovoItem),
            "btnFechar" => Some(self.btnFechar),
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
            "grdLista" => Some(self.grdLista),
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
            "dsMestre" => Some(self.dsMestre),
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
            "edtNumero" => Some(self.edtNumero),
            "edtData" => Some(self.edtData),
            "edtParceiro" => Some(self.edtParceiro),
            "edtValorTotal" => Some(self.edtValorTotal),
            "edtSituacao" => Some(self.edtSituacao),
            "edtObservacao" => Some(self.edtObservacao),
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
            "cdsItensId" => Some(self.cdsItensId),
            "cdsItensMestreId" => Some(self.cdsItensMestreId),
            "cdsItensItem" => Some(self.cdsItensItem),
            "cdsItensQuantidade" => Some(self.cdsItensQuantidade),
            "cdsItensValorUnitario" => Some(self.cdsItensValorUnitario),
            "cdsItensValorTotal" => Some(self.cdsItensValorTotal),
            _ => None,
        }
    }

    /// O Field que disparou o evento (`None` se o `sender` não é um Field deste form).
    pub fn sender_as_field(&self, ui: &Ui, sender: ComponentId) -> Option<Field> {
        self.find_field(ui.name(sender)?)
    }

    /// O GroupBox deste form com o nome (`None` se não há um GroupBox com ele).
    pub fn find_group_box(&self, name: &str) -> Option<GroupBox> {
        match name {
            "gbDados" => Some(self.gbDados),
            "gbItens" => Some(self.gbItens),
            _ => None,
        }
    }

    /// O GroupBox que disparou o evento (`None` se o `sender` não é um GroupBox deste form).
    pub fn sender_as_group_box(&self, ui: &Ui, sender: ComponentId) -> Option<GroupBox> {
        self.find_group_box(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblStatus" => Some(self.lblStatus),
            "lblNumero" => Some(self.lblNumero),
            "lblData" => Some(self.lblData),
            "lblParceiro" => Some(self.lblParceiro),
            "lblValorTotal" => Some(self.lblValorTotal),
            "lblSituacao" => Some(self.lblSituacao),
            "lblObservacao" => Some(self.lblObservacao),
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
            "layDireita" => Some(self.layDireita),
            "layNumero" => Some(self.layNumero),
            "layData" => Some(self.layData),
            "layParceiro" => Some(self.layParceiro),
            "layValorTotal" => Some(self.layValorTotal),
            "laySituacao" => Some(self.laySituacao),
            "layObservacao" => Some(self.layObservacao),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
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

    /// O Splitter deste form com o nome (`None` se não há um Splitter com ele).
    pub fn find_splitter(&self, name: &str) -> Option<Splitter> {
        match name {
            "splLista" => Some(self.splLista),
            _ => None,
        }
    }

    /// O Splitter que disparou o evento (`None` se o `sender` não é um Splitter deste form).
    pub fn sender_as_splitter(&self, ui: &Ui, sender: ComponentId) -> Option<Splitter> {
        self.find_splitter(ui.name(sender)?)
    }
}
