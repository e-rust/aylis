
/// O form frmFisMensagensFiscaisCad: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmFisMensagensFiscaisCad {
    pub form: Form,
    pub cdsMestre: ClientDataSet,
    pub cdsMestreId: Field,
    pub cdsMestreCodigo: Field,
    pub cdsMestreNome: Field,
    pub cdsMestreAtivo: Field,
    pub cdsMestreCriadoEm: Field,
    pub cdsMestreObservacao: Field,
    pub dsMestre: DataSource,
    pub cdsItens: ClientDataSet,
    pub cdsItensId: Field,
    pub cdsItensMestreId: Field,
    pub cdsItensData: Field,
    pub cdsItensDescricao: Field,
    pub cdsItensResponsavel: Field,
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
    pub layCodigo: Layout,
    pub lblCodigo: Label,
    pub edtCodigo: Edit,
    pub layNome: Layout,
    pub lblNome: Label,
    pub edtNome: Edit,
    pub layAtivo: Layout,
    pub lblAtivo: Label,
    pub chkAtivo: CheckBox,
    pub layCriadoEm: Layout,
    pub lblCriadoEm: Label,
    pub edtCriadoEm: Edit,
    pub layObservacao: Layout,
    pub lblObservacao: Label,
    pub edtObservacao: Edit,
    pub gbItens: GroupBox,
    pub grdItens: DataGrid,
    pub data: super::fis_mensagens_fiscais_cad::Data,
}

/// Os componentes do frmFisMensagensFiscaisCad sem o estado, para o código fora dos handlers
/// (`FrmFisMensagensFiscaisCad::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmFisMensagensFiscaisCadComponents {
    pub form: Form,
    pub cdsMestre: ClientDataSet,
    pub cdsMestreId: Field,
    pub cdsMestreCodigo: Field,
    pub cdsMestreNome: Field,
    pub cdsMestreAtivo: Field,
    pub cdsMestreCriadoEm: Field,
    pub cdsMestreObservacao: Field,
    pub dsMestre: DataSource,
    pub cdsItens: ClientDataSet,
    pub cdsItensId: Field,
    pub cdsItensMestreId: Field,
    pub cdsItensData: Field,
    pub cdsItensDescricao: Field,
    pub cdsItensResponsavel: Field,
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
    pub layCodigo: Layout,
    pub lblCodigo: Label,
    pub edtCodigo: Edit,
    pub layNome: Layout,
    pub lblNome: Label,
    pub edtNome: Edit,
    pub layAtivo: Layout,
    pub lblAtivo: Label,
    pub chkAtivo: CheckBox,
    pub layCriadoEm: Layout,
    pub lblCriadoEm: Label,
    pub edtCriadoEm: Edit,
    pub layObservacao: Layout,
    pub lblObservacao: Label,
    pub edtObservacao: Edit,
    pub gbItens: GroupBox,
    pub grdItens: DataGrid,
}

impl std::ops::Deref for FrmFisMensagensFiscaisCad {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmFisMensagensFiscaisCad {
    /// Monta a instância de um frmFisMensagensFiscaisCad recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            cdsMestre: ClientDataSet { id: handles::resolve(ui, root, "cdsMestre", "ClientDataSet", &mut problems) },
            cdsMestreId: Field { id: handles::resolve(ui, root, "cdsMestreId", "Field", &mut problems) },
            cdsMestreCodigo: Field { id: handles::resolve(ui, root, "cdsMestreCodigo", "Field", &mut problems) },
            cdsMestreNome: Field { id: handles::resolve(ui, root, "cdsMestreNome", "Field", &mut problems) },
            cdsMestreAtivo: Field { id: handles::resolve(ui, root, "cdsMestreAtivo", "Field", &mut problems) },
            cdsMestreCriadoEm: Field { id: handles::resolve(ui, root, "cdsMestreCriadoEm", "Field", &mut problems) },
            cdsMestreObservacao: Field { id: handles::resolve(ui, root, "cdsMestreObservacao", "Field", &mut problems) },
            dsMestre: DataSource { id: handles::resolve(ui, root, "dsMestre", "DataSource", &mut problems) },
            cdsItens: ClientDataSet { id: handles::resolve(ui, root, "cdsItens", "ClientDataSet", &mut problems) },
            cdsItensId: Field { id: handles::resolve(ui, root, "cdsItensId", "Field", &mut problems) },
            cdsItensMestreId: Field { id: handles::resolve(ui, root, "cdsItensMestreId", "Field", &mut problems) },
            cdsItensData: Field { id: handles::resolve(ui, root, "cdsItensData", "Field", &mut problems) },
            cdsItensDescricao: Field { id: handles::resolve(ui, root, "cdsItensDescricao", "Field", &mut problems) },
            cdsItensResponsavel: Field { id: handles::resolve(ui, root, "cdsItensResponsavel", "Field", &mut problems) },
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
            layCodigo: Layout { id: handles::resolve(ui, root, "layCodigo", "Layout", &mut problems) },
            lblCodigo: Label { id: handles::resolve(ui, root, "lblCodigo", "Label", &mut problems) },
            edtCodigo: Edit { id: handles::resolve(ui, root, "edtCodigo", "Edit", &mut problems) },
            layNome: Layout { id: handles::resolve(ui, root, "layNome", "Layout", &mut problems) },
            lblNome: Label { id: handles::resolve(ui, root, "lblNome", "Label", &mut problems) },
            edtNome: Edit { id: handles::resolve(ui, root, "edtNome", "Edit", &mut problems) },
            layAtivo: Layout { id: handles::resolve(ui, root, "layAtivo", "Layout", &mut problems) },
            lblAtivo: Label { id: handles::resolve(ui, root, "lblAtivo", "Label", &mut problems) },
            chkAtivo: CheckBox { id: handles::resolve(ui, root, "chkAtivo", "CheckBox", &mut problems) },
            layCriadoEm: Layout { id: handles::resolve(ui, root, "layCriadoEm", "Layout", &mut problems) },
            lblCriadoEm: Label { id: handles::resolve(ui, root, "lblCriadoEm", "Label", &mut problems) },
            edtCriadoEm: Edit { id: handles::resolve(ui, root, "edtCriadoEm", "Edit", &mut problems) },
            layObservacao: Layout { id: handles::resolve(ui, root, "layObservacao", "Layout", &mut problems) },
            lblObservacao: Label { id: handles::resolve(ui, root, "lblObservacao", "Label", &mut problems) },
            edtObservacao: Edit { id: handles::resolve(ui, root, "edtObservacao", "Edit", &mut problems) },
            gbItens: GroupBox { id: handles::resolve(ui, root, "gbItens", "GroupBox", &mut problems) },
            grdItens: DataGrid { id: handles::resolve(ui, root, "grdItens", "DataGrid", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmFisMensagensFiscaisCad não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmFisMensagensFiscaisCadComponents {
            form: instance.form,
            cdsMestre: instance.cdsMestre,
            cdsMestreId: instance.cdsMestreId,
            cdsMestreCodigo: instance.cdsMestreCodigo,
            cdsMestreNome: instance.cdsMestreNome,
            cdsMestreAtivo: instance.cdsMestreAtivo,
            cdsMestreCriadoEm: instance.cdsMestreCriadoEm,
            cdsMestreObservacao: instance.cdsMestreObservacao,
            dsMestre: instance.dsMestre,
            cdsItens: instance.cdsItens,
            cdsItensId: instance.cdsItensId,
            cdsItensMestreId: instance.cdsItensMestreId,
            cdsItensData: instance.cdsItensData,
            cdsItensDescricao: instance.cdsItensDescricao,
            cdsItensResponsavel: instance.cdsItensResponsavel,
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
            layCodigo: instance.layCodigo,
            lblCodigo: instance.lblCodigo,
            edtCodigo: instance.edtCodigo,
            layNome: instance.layNome,
            lblNome: instance.lblNome,
            edtNome: instance.edtNome,
            layAtivo: instance.layAtivo,
            lblAtivo: instance.lblAtivo,
            chkAtivo: instance.chkAtivo,
            layCriadoEm: instance.layCriadoEm,
            lblCriadoEm: instance.lblCriadoEm,
            edtCriadoEm: instance.edtCriadoEm,
            layObservacao: instance.layObservacao,
            lblObservacao: instance.lblObservacao,
            edtObservacao: instance.edtObservacao,
            gbItens: instance.gbItens,
            grdItens: instance.grdItens,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmFisMensagensFiscaisCad aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmFisMensagensFiscaisCadComponents> {
        ui.form_components(ui.find_path("frmFisMensagensFiscaisCad")?)
    }

    /// Cria o frmFisMensagensFiscaisCad (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmFisMensagensFiscaisCad")
    }

    /// Cria mais um frmFisMensagensFiscaisCad dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmFisMensagensFiscaisCad`, `frmFisMensagensFiscaisCad2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmFisMensagensFiscaisCadComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmFisMensagensFiscaisCad")?;
        let name = ui.free_name_in(parent, "frmFisMensagensFiscaisCad");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmFisMensagensFiscaisCad (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmFisMensagensFiscaisCadComponents> {
        ui.form_components(root)
    }

    /// O frmFisMensagensFiscaisCad aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmFisMensagensFiscaisCad> {
        ui.form_instance(ui.find_path("frmFisMensagensFiscaisCad")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmFisMensagensFiscaisCad> {
        let form = ui.find_path("frmFisMensagensFiscaisCad")?;
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

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkAtivo" => Some(self.chkAtivo),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
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
            "edtCodigo" => Some(self.edtCodigo),
            "edtNome" => Some(self.edtNome),
            "edtCriadoEm" => Some(self.edtCriadoEm),
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
            "cdsMestreCodigo" => Some(self.cdsMestreCodigo),
            "cdsMestreNome" => Some(self.cdsMestreNome),
            "cdsMestreAtivo" => Some(self.cdsMestreAtivo),
            "cdsMestreCriadoEm" => Some(self.cdsMestreCriadoEm),
            "cdsMestreObservacao" => Some(self.cdsMestreObservacao),
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
            "lblCodigo" => Some(self.lblCodigo),
            "lblNome" => Some(self.lblNome),
            "lblAtivo" => Some(self.lblAtivo),
            "lblCriadoEm" => Some(self.lblCriadoEm),
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
            "layCodigo" => Some(self.layCodigo),
            "layNome" => Some(self.layNome),
            "layAtivo" => Some(self.layAtivo),
            "layCriadoEm" => Some(self.layCriadoEm),
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
