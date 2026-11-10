
/// O form frmDocumentacao: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmDocumentacao {
    pub form: Form,
    pub pnlTopo: Panel,
    pub lblMarca: Label,
    pub lblDocs: Label,
    pub btnVoltar: Button,
    pub edtBusca: Edit,
    pub pnlLateral: Panel,
    pub tvIndice: TreeView,
    pub lbBusca: ListBox,
    pub layNestaPagina: Layout,
    pub lblNestaPagina: Label,
    pub lbTitulos: ListBox,
    pub layConteudo: Layout,
    pub lblTrilha: Label,
    pub pnlDemo: Panel,
    pub lblDemo: Label,
    pub wbDemo: WebView,
    pub layNavegacao: Layout,
    pub btnAnterior: Button,
    pub btnProxima: Button,
    pub mdPagina: Markdown,
    pub data: super::documentacao::Data,
}

/// Os componentes do frmDocumentacao sem o estado, para o código fora dos handlers
/// (`FrmDocumentacao::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmDocumentacaoComponents {
    pub form: Form,
    pub pnlTopo: Panel,
    pub lblMarca: Label,
    pub lblDocs: Label,
    pub btnVoltar: Button,
    pub edtBusca: Edit,
    pub pnlLateral: Panel,
    pub tvIndice: TreeView,
    pub lbBusca: ListBox,
    pub layNestaPagina: Layout,
    pub lblNestaPagina: Label,
    pub lbTitulos: ListBox,
    pub layConteudo: Layout,
    pub lblTrilha: Label,
    pub pnlDemo: Panel,
    pub lblDemo: Label,
    pub wbDemo: WebView,
    pub layNavegacao: Layout,
    pub btnAnterior: Button,
    pub btnProxima: Button,
    pub mdPagina: Markdown,
}

impl std::ops::Deref for FrmDocumentacao {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmDocumentacao {
    /// Monta a instância de um frmDocumentacao recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlTopo: Panel { id: handles::resolve(ui, root, "pnlTopo", "Panel", &mut problems) },
            lblMarca: Label { id: handles::resolve(ui, root, "lblMarca", "Label", &mut problems) },
            lblDocs: Label { id: handles::resolve(ui, root, "lblDocs", "Label", &mut problems) },
            btnVoltar: Button { id: handles::resolve(ui, root, "btnVoltar", "Button", &mut problems) },
            edtBusca: Edit { id: handles::resolve(ui, root, "edtBusca", "Edit", &mut problems) },
            pnlLateral: Panel { id: handles::resolve(ui, root, "pnlLateral", "Panel", &mut problems) },
            tvIndice: TreeView { id: handles::resolve(ui, root, "tvIndice", "TreeView", &mut problems) },
            lbBusca: ListBox { id: handles::resolve(ui, root, "lbBusca", "ListBox", &mut problems) },
            layNestaPagina: Layout { id: handles::resolve(ui, root, "layNestaPagina", "Layout", &mut problems) },
            lblNestaPagina: Label { id: handles::resolve(ui, root, "lblNestaPagina", "Label", &mut problems) },
            lbTitulos: ListBox { id: handles::resolve(ui, root, "lbTitulos", "ListBox", &mut problems) },
            layConteudo: Layout { id: handles::resolve(ui, root, "layConteudo", "Layout", &mut problems) },
            lblTrilha: Label { id: handles::resolve(ui, root, "lblTrilha", "Label", &mut problems) },
            pnlDemo: Panel { id: handles::resolve(ui, root, "pnlDemo", "Panel", &mut problems) },
            lblDemo: Label { id: handles::resolve(ui, root, "lblDemo", "Label", &mut problems) },
            wbDemo: WebView { id: handles::resolve(ui, root, "wbDemo", "WebView", &mut problems) },
            layNavegacao: Layout { id: handles::resolve(ui, root, "layNavegacao", "Layout", &mut problems) },
            btnAnterior: Button { id: handles::resolve(ui, root, "btnAnterior", "Button", &mut problems) },
            btnProxima: Button { id: handles::resolve(ui, root, "btnProxima", "Button", &mut problems) },
            mdPagina: Markdown { id: handles::resolve(ui, root, "mdPagina", "Markdown", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmDocumentacao não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmDocumentacaoComponents {
            form: instance.form,
            pnlTopo: instance.pnlTopo,
            lblMarca: instance.lblMarca,
            lblDocs: instance.lblDocs,
            btnVoltar: instance.btnVoltar,
            edtBusca: instance.edtBusca,
            pnlLateral: instance.pnlLateral,
            tvIndice: instance.tvIndice,
            lbBusca: instance.lbBusca,
            layNestaPagina: instance.layNestaPagina,
            lblNestaPagina: instance.lblNestaPagina,
            lbTitulos: instance.lbTitulos,
            layConteudo: instance.layConteudo,
            lblTrilha: instance.lblTrilha,
            pnlDemo: instance.pnlDemo,
            lblDemo: instance.lblDemo,
            wbDemo: instance.wbDemo,
            layNavegacao: instance.layNavegacao,
            btnAnterior: instance.btnAnterior,
            btnProxima: instance.btnProxima,
            mdPagina: instance.mdPagina,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmDocumentacao aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmDocumentacaoComponents> {
        ui.form_components(ui.find_path("frmDocumentacao")?)
    }

    /// Cria o frmDocumentacao (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmDocumentacao")
    }

    /// Cria mais um frmDocumentacao dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmDocumentacao`, `frmDocumentacao2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmDocumentacaoComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmDocumentacao")?;
        let name = ui.free_name_in(parent, "frmDocumentacao");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmDocumentacao (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmDocumentacaoComponents> {
        ui.form_components(root)
    }

    /// O frmDocumentacao aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmDocumentacao> {
        ui.form_instance(ui.find_path("frmDocumentacao")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmDocumentacao> {
        let form = ui.find_path("frmDocumentacao")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnVoltar" => Some(self.btnVoltar),
            "btnAnterior" => Some(self.btnAnterior),
            "btnProxima" => Some(self.btnProxima),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
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

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblMarca" => Some(self.lblMarca),
            "lblDocs" => Some(self.lblDocs),
            "lblNestaPagina" => Some(self.lblNestaPagina),
            "lblTrilha" => Some(self.lblTrilha),
            "lblDemo" => Some(self.lblDemo),
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
            "layNestaPagina" => Some(self.layNestaPagina),
            "layConteudo" => Some(self.layConteudo),
            "layNavegacao" => Some(self.layNavegacao),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O ListBox deste form com o nome (`None` se não há um ListBox com ele).
    pub fn find_list_box(&self, name: &str) -> Option<ListBox> {
        match name {
            "lbBusca" => Some(self.lbBusca),
            "lbTitulos" => Some(self.lbTitulos),
            _ => None,
        }
    }

    /// O ListBox que disparou o evento (`None` se o `sender` não é um ListBox deste form).
    pub fn sender_as_list_box(&self, ui: &Ui, sender: ComponentId) -> Option<ListBox> {
        self.find_list_box(ui.name(sender)?)
    }

    /// O Markdown deste form com o nome (`None` se não há um Markdown com ele).
    pub fn find_markdown(&self, name: &str) -> Option<Markdown> {
        match name {
            "mdPagina" => Some(self.mdPagina),
            _ => None,
        }
    }

    /// O Markdown que disparou o evento (`None` se o `sender` não é um Markdown deste form).
    pub fn sender_as_markdown(&self, ui: &Ui, sender: ComponentId) -> Option<Markdown> {
        self.find_markdown(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlTopo" => Some(self.pnlTopo),
            "pnlLateral" => Some(self.pnlLateral),
            "pnlDemo" => Some(self.pnlDemo),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O TreeView deste form com o nome (`None` se não há um TreeView com ele).
    pub fn find_tree_view(&self, name: &str) -> Option<TreeView> {
        match name {
            "tvIndice" => Some(self.tvIndice),
            _ => None,
        }
    }

    /// O TreeView que disparou o evento (`None` se o `sender` não é um TreeView deste form).
    pub fn sender_as_tree_view(&self, ui: &Ui, sender: ComponentId) -> Option<TreeView> {
        self.find_tree_view(ui.name(sender)?)
    }

    /// O WebView deste form com o nome (`None` se não há um WebView com ele).
    pub fn find_web_view(&self, name: &str) -> Option<WebView> {
        match name {
            "wbDemo" => Some(self.wbDemo),
            _ => None,
        }
    }

    /// O WebView que disparou o evento (`None` se o `sender` não é um WebView deste form).
    pub fn sender_as_web_view(&self, ui: &Ui, sender: ComponentId) -> Option<WebView> {
        self.find_web_view(ui.name(sender)?)
    }
}
