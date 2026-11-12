
/// O form frmPaint: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPaint {
    pub form: Form,
    pub lytTopo: Layout,
    pub mnuPrincipal: MainMenu,
    pub mnuArquivo: MenuItem,
    pub mnuNovo: MenuItem,
    pub mnuAbrir: MenuItem,
    pub mnuSalvar: MenuItem,
    pub mnuSalvarComo: MenuItem,
    pub mnuSep1: MenuItem,
    pub mnuSair: MenuItem,
    pub mnuEditar: MenuItem,
    pub mnuDesfazer: MenuItem,
    pub mnuRefazer: MenuItem,
    pub mnuSep2: MenuItem,
    pub mnuRecortar: MenuItem,
    pub mnuCopiar: MenuItem,
    pub mnuColar: MenuItem,
    pub mnuExcluir: MenuItem,
    pub mnuSelecionarTudo: MenuItem,
    pub mnuSep3: MenuItem,
    pub mnuLimpar: MenuItem,
    pub mnuExibir: MenuItem,
    pub mnuAmpliar: MenuItem,
    pub mnuReduzir: MenuItem,
    pub mnuTamanhoReal: MenuItem,
    pub divTopo: Divider,
    pub btnSalvar: Button,
    pub btnDesfazer: Button,
    pub btnRefazer: Button,
    pub pnlFaixa: Panel,
    pub lytSelecao: Layout,
    pub lblSelecaoGrupo: Label,
    pub btnSelecionar: Button,
    pub divSelecao: Divider,
    pub lytFerramentas: Layout,
    pub lblFerramentasGrupo: Label,
    pub grdFerramentas: GridLayout,
    pub btnLapis: Button,
    pub btnBalde: Button,
    pub btnBorracha: Button,
    pub btnContaGotas: Button,
    pub divFerramentas: Divider,
    pub lytPinceis: Layout,
    pub lblPinceis: Label,
    pub btnPincel: Button,
    pub divPinceis: Divider,
    pub lytFormas: Layout,
    pub lblFormas: Label,
    pub chkPreencher: CheckBox,
    pub grdFormas: GridLayout,
    pub btnLinha: Button,
    pub btnRetangulo: Button,
    pub btnElipse: Button,
    pub divFormas: Divider,
    pub lytTamanho: Layout,
    pub lblTamanhoGrupo: Label,
    pub cbEspessura: ComboBox,
    pub divTamanho: Divider,
    pub lytCores: Layout,
    pub lblCores: Label,
    pub btnCor1: Circle,
    pub btnCor2: Circle,
    pub grdPaleta: GridLayout,
    pub btnPaleta01: Circle,
    pub btnPaleta02: Circle,
    pub btnPaleta03: Circle,
    pub btnPaleta04: Circle,
    pub btnPaleta05: Circle,
    pub btnPaleta06: Circle,
    pub btnPaleta07: Circle,
    pub btnPaleta08: Circle,
    pub btnPaleta09: Circle,
    pub btnPaleta10: Circle,
    pub btnPaleta11: Circle,
    pub btnPaleta12: Circle,
    pub btnPaleta13: Circle,
    pub btnPaleta14: Circle,
    pub btnPaleta15: Circle,
    pub btnPaleta16: Circle,
    pub btnPaleta17: Circle,
    pub btnPaleta18: Circle,
    pub btnPaleta19: Circle,
    pub btnPaleta20: Circle,
    pub stbStatus: StatusBar,
    pub lblPosicao: Label,
    pub lblTamanho: Label,
    pub lblZoom: Label,
    pub sbxArea: ScrollBox,
    pub pcDesenho: PaintCanvas,
    pub dlgAbrir: OpenDialog,
    pub dlgSalvar: SaveDialog,
    pub data: super::paint::Data,
}

/// Os componentes do frmPaint sem o estado, para o código fora dos handlers
/// (`FrmPaint::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPaintComponents {
    pub form: Form,
    pub lytTopo: Layout,
    pub mnuPrincipal: MainMenu,
    pub mnuArquivo: MenuItem,
    pub mnuNovo: MenuItem,
    pub mnuAbrir: MenuItem,
    pub mnuSalvar: MenuItem,
    pub mnuSalvarComo: MenuItem,
    pub mnuSep1: MenuItem,
    pub mnuSair: MenuItem,
    pub mnuEditar: MenuItem,
    pub mnuDesfazer: MenuItem,
    pub mnuRefazer: MenuItem,
    pub mnuSep2: MenuItem,
    pub mnuRecortar: MenuItem,
    pub mnuCopiar: MenuItem,
    pub mnuColar: MenuItem,
    pub mnuExcluir: MenuItem,
    pub mnuSelecionarTudo: MenuItem,
    pub mnuSep3: MenuItem,
    pub mnuLimpar: MenuItem,
    pub mnuExibir: MenuItem,
    pub mnuAmpliar: MenuItem,
    pub mnuReduzir: MenuItem,
    pub mnuTamanhoReal: MenuItem,
    pub divTopo: Divider,
    pub btnSalvar: Button,
    pub btnDesfazer: Button,
    pub btnRefazer: Button,
    pub pnlFaixa: Panel,
    pub lytSelecao: Layout,
    pub lblSelecaoGrupo: Label,
    pub btnSelecionar: Button,
    pub divSelecao: Divider,
    pub lytFerramentas: Layout,
    pub lblFerramentasGrupo: Label,
    pub grdFerramentas: GridLayout,
    pub btnLapis: Button,
    pub btnBalde: Button,
    pub btnBorracha: Button,
    pub btnContaGotas: Button,
    pub divFerramentas: Divider,
    pub lytPinceis: Layout,
    pub lblPinceis: Label,
    pub btnPincel: Button,
    pub divPinceis: Divider,
    pub lytFormas: Layout,
    pub lblFormas: Label,
    pub chkPreencher: CheckBox,
    pub grdFormas: GridLayout,
    pub btnLinha: Button,
    pub btnRetangulo: Button,
    pub btnElipse: Button,
    pub divFormas: Divider,
    pub lytTamanho: Layout,
    pub lblTamanhoGrupo: Label,
    pub cbEspessura: ComboBox,
    pub divTamanho: Divider,
    pub lytCores: Layout,
    pub lblCores: Label,
    pub btnCor1: Circle,
    pub btnCor2: Circle,
    pub grdPaleta: GridLayout,
    pub btnPaleta01: Circle,
    pub btnPaleta02: Circle,
    pub btnPaleta03: Circle,
    pub btnPaleta04: Circle,
    pub btnPaleta05: Circle,
    pub btnPaleta06: Circle,
    pub btnPaleta07: Circle,
    pub btnPaleta08: Circle,
    pub btnPaleta09: Circle,
    pub btnPaleta10: Circle,
    pub btnPaleta11: Circle,
    pub btnPaleta12: Circle,
    pub btnPaleta13: Circle,
    pub btnPaleta14: Circle,
    pub btnPaleta15: Circle,
    pub btnPaleta16: Circle,
    pub btnPaleta17: Circle,
    pub btnPaleta18: Circle,
    pub btnPaleta19: Circle,
    pub btnPaleta20: Circle,
    pub stbStatus: StatusBar,
    pub lblPosicao: Label,
    pub lblTamanho: Label,
    pub lblZoom: Label,
    pub sbxArea: ScrollBox,
    pub pcDesenho: PaintCanvas,
    pub dlgAbrir: OpenDialog,
    pub dlgSalvar: SaveDialog,
}

impl std::ops::Deref for FrmPaint {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmPaint {
    /// Monta a instância de um frmPaint recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lytTopo: Layout { id: handles::resolve(ui, root, "lytTopo", "Layout", &mut problems) },
            mnuPrincipal: MainMenu { id: handles::resolve(ui, root, "mnuPrincipal", "MainMenu", &mut problems) },
            mnuArquivo: MenuItem { id: handles::resolve(ui, root, "mnuArquivo", "MenuItem", &mut problems) },
            mnuNovo: MenuItem { id: handles::resolve(ui, root, "mnuNovo", "MenuItem", &mut problems) },
            mnuAbrir: MenuItem { id: handles::resolve(ui, root, "mnuAbrir", "MenuItem", &mut problems) },
            mnuSalvar: MenuItem { id: handles::resolve(ui, root, "mnuSalvar", "MenuItem", &mut problems) },
            mnuSalvarComo: MenuItem { id: handles::resolve(ui, root, "mnuSalvarComo", "MenuItem", &mut problems) },
            mnuSep1: MenuItem { id: handles::resolve(ui, root, "mnuSep1", "MenuItem", &mut problems) },
            mnuSair: MenuItem { id: handles::resolve(ui, root, "mnuSair", "MenuItem", &mut problems) },
            mnuEditar: MenuItem { id: handles::resolve(ui, root, "mnuEditar", "MenuItem", &mut problems) },
            mnuDesfazer: MenuItem { id: handles::resolve(ui, root, "mnuDesfazer", "MenuItem", &mut problems) },
            mnuRefazer: MenuItem { id: handles::resolve(ui, root, "mnuRefazer", "MenuItem", &mut problems) },
            mnuSep2: MenuItem { id: handles::resolve(ui, root, "mnuSep2", "MenuItem", &mut problems) },
            mnuRecortar: MenuItem { id: handles::resolve(ui, root, "mnuRecortar", "MenuItem", &mut problems) },
            mnuCopiar: MenuItem { id: handles::resolve(ui, root, "mnuCopiar", "MenuItem", &mut problems) },
            mnuColar: MenuItem { id: handles::resolve(ui, root, "mnuColar", "MenuItem", &mut problems) },
            mnuExcluir: MenuItem { id: handles::resolve(ui, root, "mnuExcluir", "MenuItem", &mut problems) },
            mnuSelecionarTudo: MenuItem { id: handles::resolve(ui, root, "mnuSelecionarTudo", "MenuItem", &mut problems) },
            mnuSep3: MenuItem { id: handles::resolve(ui, root, "mnuSep3", "MenuItem", &mut problems) },
            mnuLimpar: MenuItem { id: handles::resolve(ui, root, "mnuLimpar", "MenuItem", &mut problems) },
            mnuExibir: MenuItem { id: handles::resolve(ui, root, "mnuExibir", "MenuItem", &mut problems) },
            mnuAmpliar: MenuItem { id: handles::resolve(ui, root, "mnuAmpliar", "MenuItem", &mut problems) },
            mnuReduzir: MenuItem { id: handles::resolve(ui, root, "mnuReduzir", "MenuItem", &mut problems) },
            mnuTamanhoReal: MenuItem { id: handles::resolve(ui, root, "mnuTamanhoReal", "MenuItem", &mut problems) },
            divTopo: Divider { id: handles::resolve(ui, root, "divTopo", "Divider", &mut problems) },
            btnSalvar: Button { id: handles::resolve(ui, root, "btnSalvar", "Button", &mut problems) },
            btnDesfazer: Button { id: handles::resolve(ui, root, "btnDesfazer", "Button", &mut problems) },
            btnRefazer: Button { id: handles::resolve(ui, root, "btnRefazer", "Button", &mut problems) },
            pnlFaixa: Panel { id: handles::resolve(ui, root, "pnlFaixa", "Panel", &mut problems) },
            lytSelecao: Layout { id: handles::resolve(ui, root, "lytSelecao", "Layout", &mut problems) },
            lblSelecaoGrupo: Label { id: handles::resolve(ui, root, "lblSelecaoGrupo", "Label", &mut problems) },
            btnSelecionar: Button { id: handles::resolve(ui, root, "btnSelecionar", "Button", &mut problems) },
            divSelecao: Divider { id: handles::resolve(ui, root, "divSelecao", "Divider", &mut problems) },
            lytFerramentas: Layout { id: handles::resolve(ui, root, "lytFerramentas", "Layout", &mut problems) },
            lblFerramentasGrupo: Label { id: handles::resolve(ui, root, "lblFerramentasGrupo", "Label", &mut problems) },
            grdFerramentas: GridLayout { id: handles::resolve(ui, root, "grdFerramentas", "GridLayout", &mut problems) },
            btnLapis: Button { id: handles::resolve(ui, root, "btnLapis", "Button", &mut problems) },
            btnBalde: Button { id: handles::resolve(ui, root, "btnBalde", "Button", &mut problems) },
            btnBorracha: Button { id: handles::resolve(ui, root, "btnBorracha", "Button", &mut problems) },
            btnContaGotas: Button { id: handles::resolve(ui, root, "btnContaGotas", "Button", &mut problems) },
            divFerramentas: Divider { id: handles::resolve(ui, root, "divFerramentas", "Divider", &mut problems) },
            lytPinceis: Layout { id: handles::resolve(ui, root, "lytPinceis", "Layout", &mut problems) },
            lblPinceis: Label { id: handles::resolve(ui, root, "lblPinceis", "Label", &mut problems) },
            btnPincel: Button { id: handles::resolve(ui, root, "btnPincel", "Button", &mut problems) },
            divPinceis: Divider { id: handles::resolve(ui, root, "divPinceis", "Divider", &mut problems) },
            lytFormas: Layout { id: handles::resolve(ui, root, "lytFormas", "Layout", &mut problems) },
            lblFormas: Label { id: handles::resolve(ui, root, "lblFormas", "Label", &mut problems) },
            chkPreencher: CheckBox { id: handles::resolve(ui, root, "chkPreencher", "CheckBox", &mut problems) },
            grdFormas: GridLayout { id: handles::resolve(ui, root, "grdFormas", "GridLayout", &mut problems) },
            btnLinha: Button { id: handles::resolve(ui, root, "btnLinha", "Button", &mut problems) },
            btnRetangulo: Button { id: handles::resolve(ui, root, "btnRetangulo", "Button", &mut problems) },
            btnElipse: Button { id: handles::resolve(ui, root, "btnElipse", "Button", &mut problems) },
            divFormas: Divider { id: handles::resolve(ui, root, "divFormas", "Divider", &mut problems) },
            lytTamanho: Layout { id: handles::resolve(ui, root, "lytTamanho", "Layout", &mut problems) },
            lblTamanhoGrupo: Label { id: handles::resolve(ui, root, "lblTamanhoGrupo", "Label", &mut problems) },
            cbEspessura: ComboBox { id: handles::resolve(ui, root, "cbEspessura", "ComboBox", &mut problems) },
            divTamanho: Divider { id: handles::resolve(ui, root, "divTamanho", "Divider", &mut problems) },
            lytCores: Layout { id: handles::resolve(ui, root, "lytCores", "Layout", &mut problems) },
            lblCores: Label { id: handles::resolve(ui, root, "lblCores", "Label", &mut problems) },
            btnCor1: Circle { id: handles::resolve(ui, root, "btnCor1", "Circle", &mut problems) },
            btnCor2: Circle { id: handles::resolve(ui, root, "btnCor2", "Circle", &mut problems) },
            grdPaleta: GridLayout { id: handles::resolve(ui, root, "grdPaleta", "GridLayout", &mut problems) },
            btnPaleta01: Circle { id: handles::resolve(ui, root, "btnPaleta01", "Circle", &mut problems) },
            btnPaleta02: Circle { id: handles::resolve(ui, root, "btnPaleta02", "Circle", &mut problems) },
            btnPaleta03: Circle { id: handles::resolve(ui, root, "btnPaleta03", "Circle", &mut problems) },
            btnPaleta04: Circle { id: handles::resolve(ui, root, "btnPaleta04", "Circle", &mut problems) },
            btnPaleta05: Circle { id: handles::resolve(ui, root, "btnPaleta05", "Circle", &mut problems) },
            btnPaleta06: Circle { id: handles::resolve(ui, root, "btnPaleta06", "Circle", &mut problems) },
            btnPaleta07: Circle { id: handles::resolve(ui, root, "btnPaleta07", "Circle", &mut problems) },
            btnPaleta08: Circle { id: handles::resolve(ui, root, "btnPaleta08", "Circle", &mut problems) },
            btnPaleta09: Circle { id: handles::resolve(ui, root, "btnPaleta09", "Circle", &mut problems) },
            btnPaleta10: Circle { id: handles::resolve(ui, root, "btnPaleta10", "Circle", &mut problems) },
            btnPaleta11: Circle { id: handles::resolve(ui, root, "btnPaleta11", "Circle", &mut problems) },
            btnPaleta12: Circle { id: handles::resolve(ui, root, "btnPaleta12", "Circle", &mut problems) },
            btnPaleta13: Circle { id: handles::resolve(ui, root, "btnPaleta13", "Circle", &mut problems) },
            btnPaleta14: Circle { id: handles::resolve(ui, root, "btnPaleta14", "Circle", &mut problems) },
            btnPaleta15: Circle { id: handles::resolve(ui, root, "btnPaleta15", "Circle", &mut problems) },
            btnPaleta16: Circle { id: handles::resolve(ui, root, "btnPaleta16", "Circle", &mut problems) },
            btnPaleta17: Circle { id: handles::resolve(ui, root, "btnPaleta17", "Circle", &mut problems) },
            btnPaleta18: Circle { id: handles::resolve(ui, root, "btnPaleta18", "Circle", &mut problems) },
            btnPaleta19: Circle { id: handles::resolve(ui, root, "btnPaleta19", "Circle", &mut problems) },
            btnPaleta20: Circle { id: handles::resolve(ui, root, "btnPaleta20", "Circle", &mut problems) },
            stbStatus: StatusBar { id: handles::resolve(ui, root, "stbStatus", "StatusBar", &mut problems) },
            lblPosicao: Label { id: handles::resolve(ui, root, "lblPosicao", "Label", &mut problems) },
            lblTamanho: Label { id: handles::resolve(ui, root, "lblTamanho", "Label", &mut problems) },
            lblZoom: Label { id: handles::resolve(ui, root, "lblZoom", "Label", &mut problems) },
            sbxArea: ScrollBox { id: handles::resolve(ui, root, "sbxArea", "ScrollBox", &mut problems) },
            pcDesenho: PaintCanvas { id: handles::resolve(ui, root, "pcDesenho", "PaintCanvas", &mut problems) },
            dlgAbrir: OpenDialog { id: handles::resolve(ui, root, "dlgAbrir", "OpenDialog", &mut problems) },
            dlgSalvar: SaveDialog { id: handles::resolve(ui, root, "dlgSalvar", "SaveDialog", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPaint não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPaintComponents {
            form: instance.form,
            lytTopo: instance.lytTopo,
            mnuPrincipal: instance.mnuPrincipal,
            mnuArquivo: instance.mnuArquivo,
            mnuNovo: instance.mnuNovo,
            mnuAbrir: instance.mnuAbrir,
            mnuSalvar: instance.mnuSalvar,
            mnuSalvarComo: instance.mnuSalvarComo,
            mnuSep1: instance.mnuSep1,
            mnuSair: instance.mnuSair,
            mnuEditar: instance.mnuEditar,
            mnuDesfazer: instance.mnuDesfazer,
            mnuRefazer: instance.mnuRefazer,
            mnuSep2: instance.mnuSep2,
            mnuRecortar: instance.mnuRecortar,
            mnuCopiar: instance.mnuCopiar,
            mnuColar: instance.mnuColar,
            mnuExcluir: instance.mnuExcluir,
            mnuSelecionarTudo: instance.mnuSelecionarTudo,
            mnuSep3: instance.mnuSep3,
            mnuLimpar: instance.mnuLimpar,
            mnuExibir: instance.mnuExibir,
            mnuAmpliar: instance.mnuAmpliar,
            mnuReduzir: instance.mnuReduzir,
            mnuTamanhoReal: instance.mnuTamanhoReal,
            divTopo: instance.divTopo,
            btnSalvar: instance.btnSalvar,
            btnDesfazer: instance.btnDesfazer,
            btnRefazer: instance.btnRefazer,
            pnlFaixa: instance.pnlFaixa,
            lytSelecao: instance.lytSelecao,
            lblSelecaoGrupo: instance.lblSelecaoGrupo,
            btnSelecionar: instance.btnSelecionar,
            divSelecao: instance.divSelecao,
            lytFerramentas: instance.lytFerramentas,
            lblFerramentasGrupo: instance.lblFerramentasGrupo,
            grdFerramentas: instance.grdFerramentas,
            btnLapis: instance.btnLapis,
            btnBalde: instance.btnBalde,
            btnBorracha: instance.btnBorracha,
            btnContaGotas: instance.btnContaGotas,
            divFerramentas: instance.divFerramentas,
            lytPinceis: instance.lytPinceis,
            lblPinceis: instance.lblPinceis,
            btnPincel: instance.btnPincel,
            divPinceis: instance.divPinceis,
            lytFormas: instance.lytFormas,
            lblFormas: instance.lblFormas,
            chkPreencher: instance.chkPreencher,
            grdFormas: instance.grdFormas,
            btnLinha: instance.btnLinha,
            btnRetangulo: instance.btnRetangulo,
            btnElipse: instance.btnElipse,
            divFormas: instance.divFormas,
            lytTamanho: instance.lytTamanho,
            lblTamanhoGrupo: instance.lblTamanhoGrupo,
            cbEspessura: instance.cbEspessura,
            divTamanho: instance.divTamanho,
            lytCores: instance.lytCores,
            lblCores: instance.lblCores,
            btnCor1: instance.btnCor1,
            btnCor2: instance.btnCor2,
            grdPaleta: instance.grdPaleta,
            btnPaleta01: instance.btnPaleta01,
            btnPaleta02: instance.btnPaleta02,
            btnPaleta03: instance.btnPaleta03,
            btnPaleta04: instance.btnPaleta04,
            btnPaleta05: instance.btnPaleta05,
            btnPaleta06: instance.btnPaleta06,
            btnPaleta07: instance.btnPaleta07,
            btnPaleta08: instance.btnPaleta08,
            btnPaleta09: instance.btnPaleta09,
            btnPaleta10: instance.btnPaleta10,
            btnPaleta11: instance.btnPaleta11,
            btnPaleta12: instance.btnPaleta12,
            btnPaleta13: instance.btnPaleta13,
            btnPaleta14: instance.btnPaleta14,
            btnPaleta15: instance.btnPaleta15,
            btnPaleta16: instance.btnPaleta16,
            btnPaleta17: instance.btnPaleta17,
            btnPaleta18: instance.btnPaleta18,
            btnPaleta19: instance.btnPaleta19,
            btnPaleta20: instance.btnPaleta20,
            stbStatus: instance.stbStatus,
            lblPosicao: instance.lblPosicao,
            lblTamanho: instance.lblTamanho,
            lblZoom: instance.lblZoom,
            sbxArea: instance.sbxArea,
            pcDesenho: instance.pcDesenho,
            dlgAbrir: instance.dlgAbrir,
            dlgSalvar: instance.dlgSalvar,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPaint aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmPaintComponents> {
        ui.form_components(ui.find_path("frmPaint")?)
    }

    /// Cria o frmPaint (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmPaint")
    }

    /// Cria mais um frmPaint dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmPaint`, `frmPaint2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmPaintComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmPaint")?;
        let name = ui.free_name_in(parent, "frmPaint");
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

    /// Os componentes de uma instância do frmPaint (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmPaintComponents> {
        ui.form_components(root)
    }

    /// O frmPaint aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmPaint> {
        ui.form_instance(ui.find_path("frmPaint")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPaint> {
        let form = ui.find_path("frmPaint")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnSalvar" => Some(self.btnSalvar),
            "btnDesfazer" => Some(self.btnDesfazer),
            "btnRefazer" => Some(self.btnRefazer),
            "btnSelecionar" => Some(self.btnSelecionar),
            "btnLapis" => Some(self.btnLapis),
            "btnBalde" => Some(self.btnBalde),
            "btnBorracha" => Some(self.btnBorracha),
            "btnContaGotas" => Some(self.btnContaGotas),
            "btnPincel" => Some(self.btnPincel),
            "btnLinha" => Some(self.btnLinha),
            "btnRetangulo" => Some(self.btnRetangulo),
            "btnElipse" => Some(self.btnElipse),
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
            "chkPreencher" => Some(self.chkPreencher),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O Circle deste form com o nome (`None` se não há um Circle com ele).
    pub fn find_circle(&self, name: &str) -> Option<Circle> {
        match name {
            "btnCor1" => Some(self.btnCor1),
            "btnCor2" => Some(self.btnCor2),
            "btnPaleta01" => Some(self.btnPaleta01),
            "btnPaleta02" => Some(self.btnPaleta02),
            "btnPaleta03" => Some(self.btnPaleta03),
            "btnPaleta04" => Some(self.btnPaleta04),
            "btnPaleta05" => Some(self.btnPaleta05),
            "btnPaleta06" => Some(self.btnPaleta06),
            "btnPaleta07" => Some(self.btnPaleta07),
            "btnPaleta08" => Some(self.btnPaleta08),
            "btnPaleta09" => Some(self.btnPaleta09),
            "btnPaleta10" => Some(self.btnPaleta10),
            "btnPaleta11" => Some(self.btnPaleta11),
            "btnPaleta12" => Some(self.btnPaleta12),
            "btnPaleta13" => Some(self.btnPaleta13),
            "btnPaleta14" => Some(self.btnPaleta14),
            "btnPaleta15" => Some(self.btnPaleta15),
            "btnPaleta16" => Some(self.btnPaleta16),
            "btnPaleta17" => Some(self.btnPaleta17),
            "btnPaleta18" => Some(self.btnPaleta18),
            "btnPaleta19" => Some(self.btnPaleta19),
            "btnPaleta20" => Some(self.btnPaleta20),
            _ => None,
        }
    }

    /// O Circle que disparou o evento (`None` se o `sender` não é um Circle deste form).
    pub fn sender_as_circle(&self, ui: &Ui, sender: ComponentId) -> Option<Circle> {
        self.find_circle(ui.name(sender)?)
    }

    /// O ComboBox deste form com o nome (`None` se não há um ComboBox com ele).
    pub fn find_combo_box(&self, name: &str) -> Option<ComboBox> {
        match name {
            "cbEspessura" => Some(self.cbEspessura),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O Divider deste form com o nome (`None` se não há um Divider com ele).
    pub fn find_divider(&self, name: &str) -> Option<Divider> {
        match name {
            "divTopo" => Some(self.divTopo),
            "divSelecao" => Some(self.divSelecao),
            "divFerramentas" => Some(self.divFerramentas),
            "divPinceis" => Some(self.divPinceis),
            "divFormas" => Some(self.divFormas),
            "divTamanho" => Some(self.divTamanho),
            _ => None,
        }
    }

    /// O Divider que disparou o evento (`None` se o `sender` não é um Divider deste form).
    pub fn sender_as_divider(&self, ui: &Ui, sender: ComponentId) -> Option<Divider> {
        self.find_divider(ui.name(sender)?)
    }

    /// O GridLayout deste form com o nome (`None` se não há um GridLayout com ele).
    pub fn find_grid_layout(&self, name: &str) -> Option<GridLayout> {
        match name {
            "grdFerramentas" => Some(self.grdFerramentas),
            "grdFormas" => Some(self.grdFormas),
            "grdPaleta" => Some(self.grdPaleta),
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
            "lblSelecaoGrupo" => Some(self.lblSelecaoGrupo),
            "lblFerramentasGrupo" => Some(self.lblFerramentasGrupo),
            "lblPinceis" => Some(self.lblPinceis),
            "lblFormas" => Some(self.lblFormas),
            "lblTamanhoGrupo" => Some(self.lblTamanhoGrupo),
            "lblCores" => Some(self.lblCores),
            "lblPosicao" => Some(self.lblPosicao),
            "lblTamanho" => Some(self.lblTamanho),
            "lblZoom" => Some(self.lblZoom),
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
            "lytTopo" => Some(self.lytTopo),
            "lytSelecao" => Some(self.lytSelecao),
            "lytFerramentas" => Some(self.lytFerramentas),
            "lytPinceis" => Some(self.lytPinceis),
            "lytFormas" => Some(self.lytFormas),
            "lytTamanho" => Some(self.lytTamanho),
            "lytCores" => Some(self.lytCores),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
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
            "mnuNovo" => Some(self.mnuNovo),
            "mnuAbrir" => Some(self.mnuAbrir),
            "mnuSalvar" => Some(self.mnuSalvar),
            "mnuSalvarComo" => Some(self.mnuSalvarComo),
            "mnuSep1" => Some(self.mnuSep1),
            "mnuSair" => Some(self.mnuSair),
            "mnuEditar" => Some(self.mnuEditar),
            "mnuDesfazer" => Some(self.mnuDesfazer),
            "mnuRefazer" => Some(self.mnuRefazer),
            "mnuSep2" => Some(self.mnuSep2),
            "mnuRecortar" => Some(self.mnuRecortar),
            "mnuCopiar" => Some(self.mnuCopiar),
            "mnuColar" => Some(self.mnuColar),
            "mnuExcluir" => Some(self.mnuExcluir),
            "mnuSelecionarTudo" => Some(self.mnuSelecionarTudo),
            "mnuSep3" => Some(self.mnuSep3),
            "mnuLimpar" => Some(self.mnuLimpar),
            "mnuExibir" => Some(self.mnuExibir),
            "mnuAmpliar" => Some(self.mnuAmpliar),
            "mnuReduzir" => Some(self.mnuReduzir),
            "mnuTamanhoReal" => Some(self.mnuTamanhoReal),
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

    /// O PaintCanvas deste form com o nome (`None` se não há um PaintCanvas com ele).
    pub fn find_paint_canvas(&self, name: &str) -> Option<PaintCanvas> {
        match name {
            "pcDesenho" => Some(self.pcDesenho),
            _ => None,
        }
    }

    /// O PaintCanvas que disparou o evento (`None` se o `sender` não é um PaintCanvas deste form).
    pub fn sender_as_paint_canvas(&self, ui: &Ui, sender: ComponentId) -> Option<PaintCanvas> {
        self.find_paint_canvas(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlFaixa" => Some(self.pnlFaixa),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O SaveDialog deste form com o nome (`None` se não há um SaveDialog com ele).
    pub fn find_save_dialog(&self, name: &str) -> Option<SaveDialog> {
        match name {
            "dlgSalvar" => Some(self.dlgSalvar),
            _ => None,
        }
    }

    /// O SaveDialog que disparou o evento (`None` se o `sender` não é um SaveDialog deste form).
    pub fn sender_as_save_dialog(&self, ui: &Ui, sender: ComponentId) -> Option<SaveDialog> {
        self.find_save_dialog(ui.name(sender)?)
    }

    /// O ScrollBox deste form com o nome (`None` se não há um ScrollBox com ele).
    pub fn find_scroll_box(&self, name: &str) -> Option<ScrollBox> {
        match name {
            "sbxArea" => Some(self.sbxArea),
            _ => None,
        }
    }

    /// O ScrollBox que disparou o evento (`None` se o `sender` não é um ScrollBox deste form).
    pub fn sender_as_scroll_box(&self, ui: &Ui, sender: ComponentId) -> Option<ScrollBox> {
        self.find_scroll_box(ui.name(sender)?)
    }

    /// O StatusBar deste form com o nome (`None` se não há um StatusBar com ele).
    pub fn find_status_bar(&self, name: &str) -> Option<StatusBar> {
        match name {
            "stbStatus" => Some(self.stbStatus),
            _ => None,
        }
    }

    /// O StatusBar que disparou o evento (`None` se o `sender` não é um StatusBar deste form).
    pub fn sender_as_status_bar(&self, ui: &Ui, sender: ComponentId) -> Option<StatusBar> {
        self.find_status_bar(ui.name(sender)?)
    }
}
