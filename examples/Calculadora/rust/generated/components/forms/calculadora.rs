
/// O form frmCalculadora: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmCalculadora {
    pub form: Form,
    pub pnlLateral: Panel,
    pub tabLateral: TabControl,
    pub tabHistorico: TabItem,
    pub btnLimparHistorico: Button,
    pub lvHistorico: ListView,
    pub tabMemoria: TabItem,
    pub lvMemoria: ListView,
    pub lytCorpo: Layout,
    pub lytTopo: Layout,
    pub btnHistorico: Button,
    pub lblTitulo: Label,
    pub lblExpressao: Label,
    pub lblVisor: Label,
    pub grdMemoria: GridLayout,
    pub btnMC: Button,
    pub btnMR: Button,
    pub btnMMais: Button,
    pub btnMMenos: Button,
    pub btnMS: Button,
    pub grdTeclado: GridLayout,
    pub btnPorcento: Button,
    pub btnCE: Button,
    pub btnC: Button,
    pub btnApagar: Button,
    pub btnInverso: Button,
    pub btnQuadrado: Button,
    pub btnRaiz: Button,
    pub btnDividir: Button,
    pub btn7: Button,
    pub btn8: Button,
    pub btn9: Button,
    pub btnMultiplicar: Button,
    pub btn4: Button,
    pub btn5: Button,
    pub btn6: Button,
    pub btnSubtrair: Button,
    pub btn1: Button,
    pub btn2: Button,
    pub btn3: Button,
    pub btnSomar: Button,
    pub btnSinal: Button,
    pub btn0: Button,
    pub btnVirgula: Button,
    pub btnIgual: Button,
    pub data: super::calculadora::Data,
}

/// Os componentes do frmCalculadora sem o estado, para o código fora dos handlers
/// (`FrmCalculadora::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmCalculadoraComponents {
    pub form: Form,
    pub pnlLateral: Panel,
    pub tabLateral: TabControl,
    pub tabHistorico: TabItem,
    pub btnLimparHistorico: Button,
    pub lvHistorico: ListView,
    pub tabMemoria: TabItem,
    pub lvMemoria: ListView,
    pub lytCorpo: Layout,
    pub lytTopo: Layout,
    pub btnHistorico: Button,
    pub lblTitulo: Label,
    pub lblExpressao: Label,
    pub lblVisor: Label,
    pub grdMemoria: GridLayout,
    pub btnMC: Button,
    pub btnMR: Button,
    pub btnMMais: Button,
    pub btnMMenos: Button,
    pub btnMS: Button,
    pub grdTeclado: GridLayout,
    pub btnPorcento: Button,
    pub btnCE: Button,
    pub btnC: Button,
    pub btnApagar: Button,
    pub btnInverso: Button,
    pub btnQuadrado: Button,
    pub btnRaiz: Button,
    pub btnDividir: Button,
    pub btn7: Button,
    pub btn8: Button,
    pub btn9: Button,
    pub btnMultiplicar: Button,
    pub btn4: Button,
    pub btn5: Button,
    pub btn6: Button,
    pub btnSubtrair: Button,
    pub btn1: Button,
    pub btn2: Button,
    pub btn3: Button,
    pub btnSomar: Button,
    pub btnSinal: Button,
    pub btn0: Button,
    pub btnVirgula: Button,
    pub btnIgual: Button,
}

impl std::ops::Deref for FrmCalculadora {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmCalculadora {
    /// Monta a instância de um frmCalculadora recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlLateral: Panel { id: handles::resolve(ui, root, "pnlLateral", "Panel", &mut problems) },
            tabLateral: TabControl { id: handles::resolve(ui, root, "tabLateral", "TabControl", &mut problems) },
            tabHistorico: TabItem { id: handles::resolve(ui, root, "tabHistorico", "TabItem", &mut problems) },
            btnLimparHistorico: Button { id: handles::resolve(ui, root, "btnLimparHistorico", "Button", &mut problems) },
            lvHistorico: ListView { id: handles::resolve(ui, root, "lvHistorico", "ListView", &mut problems) },
            tabMemoria: TabItem { id: handles::resolve(ui, root, "tabMemoria", "TabItem", &mut problems) },
            lvMemoria: ListView { id: handles::resolve(ui, root, "lvMemoria", "ListView", &mut problems) },
            lytCorpo: Layout { id: handles::resolve(ui, root, "lytCorpo", "Layout", &mut problems) },
            lytTopo: Layout { id: handles::resolve(ui, root, "lytTopo", "Layout", &mut problems) },
            btnHistorico: Button { id: handles::resolve(ui, root, "btnHistorico", "Button", &mut problems) },
            lblTitulo: Label { id: handles::resolve(ui, root, "lblTitulo", "Label", &mut problems) },
            lblExpressao: Label { id: handles::resolve(ui, root, "lblExpressao", "Label", &mut problems) },
            lblVisor: Label { id: handles::resolve(ui, root, "lblVisor", "Label", &mut problems) },
            grdMemoria: GridLayout { id: handles::resolve(ui, root, "grdMemoria", "GridLayout", &mut problems) },
            btnMC: Button { id: handles::resolve(ui, root, "btnMC", "Button", &mut problems) },
            btnMR: Button { id: handles::resolve(ui, root, "btnMR", "Button", &mut problems) },
            btnMMais: Button { id: handles::resolve(ui, root, "btnMMais", "Button", &mut problems) },
            btnMMenos: Button { id: handles::resolve(ui, root, "btnMMenos", "Button", &mut problems) },
            btnMS: Button { id: handles::resolve(ui, root, "btnMS", "Button", &mut problems) },
            grdTeclado: GridLayout { id: handles::resolve(ui, root, "grdTeclado", "GridLayout", &mut problems) },
            btnPorcento: Button { id: handles::resolve(ui, root, "btnPorcento", "Button", &mut problems) },
            btnCE: Button { id: handles::resolve(ui, root, "btnCE", "Button", &mut problems) },
            btnC: Button { id: handles::resolve(ui, root, "btnC", "Button", &mut problems) },
            btnApagar: Button { id: handles::resolve(ui, root, "btnApagar", "Button", &mut problems) },
            btnInverso: Button { id: handles::resolve(ui, root, "btnInverso", "Button", &mut problems) },
            btnQuadrado: Button { id: handles::resolve(ui, root, "btnQuadrado", "Button", &mut problems) },
            btnRaiz: Button { id: handles::resolve(ui, root, "btnRaiz", "Button", &mut problems) },
            btnDividir: Button { id: handles::resolve(ui, root, "btnDividir", "Button", &mut problems) },
            btn7: Button { id: handles::resolve(ui, root, "btn7", "Button", &mut problems) },
            btn8: Button { id: handles::resolve(ui, root, "btn8", "Button", &mut problems) },
            btn9: Button { id: handles::resolve(ui, root, "btn9", "Button", &mut problems) },
            btnMultiplicar: Button { id: handles::resolve(ui, root, "btnMultiplicar", "Button", &mut problems) },
            btn4: Button { id: handles::resolve(ui, root, "btn4", "Button", &mut problems) },
            btn5: Button { id: handles::resolve(ui, root, "btn5", "Button", &mut problems) },
            btn6: Button { id: handles::resolve(ui, root, "btn6", "Button", &mut problems) },
            btnSubtrair: Button { id: handles::resolve(ui, root, "btnSubtrair", "Button", &mut problems) },
            btn1: Button { id: handles::resolve(ui, root, "btn1", "Button", &mut problems) },
            btn2: Button { id: handles::resolve(ui, root, "btn2", "Button", &mut problems) },
            btn3: Button { id: handles::resolve(ui, root, "btn3", "Button", &mut problems) },
            btnSomar: Button { id: handles::resolve(ui, root, "btnSomar", "Button", &mut problems) },
            btnSinal: Button { id: handles::resolve(ui, root, "btnSinal", "Button", &mut problems) },
            btn0: Button { id: handles::resolve(ui, root, "btn0", "Button", &mut problems) },
            btnVirgula: Button { id: handles::resolve(ui, root, "btnVirgula", "Button", &mut problems) },
            btnIgual: Button { id: handles::resolve(ui, root, "btnIgual", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmCalculadora não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmCalculadoraComponents {
            form: instance.form,
            pnlLateral: instance.pnlLateral,
            tabLateral: instance.tabLateral,
            tabHistorico: instance.tabHistorico,
            btnLimparHistorico: instance.btnLimparHistorico,
            lvHistorico: instance.lvHistorico,
            tabMemoria: instance.tabMemoria,
            lvMemoria: instance.lvMemoria,
            lytCorpo: instance.lytCorpo,
            lytTopo: instance.lytTopo,
            btnHistorico: instance.btnHistorico,
            lblTitulo: instance.lblTitulo,
            lblExpressao: instance.lblExpressao,
            lblVisor: instance.lblVisor,
            grdMemoria: instance.grdMemoria,
            btnMC: instance.btnMC,
            btnMR: instance.btnMR,
            btnMMais: instance.btnMMais,
            btnMMenos: instance.btnMMenos,
            btnMS: instance.btnMS,
            grdTeclado: instance.grdTeclado,
            btnPorcento: instance.btnPorcento,
            btnCE: instance.btnCE,
            btnC: instance.btnC,
            btnApagar: instance.btnApagar,
            btnInverso: instance.btnInverso,
            btnQuadrado: instance.btnQuadrado,
            btnRaiz: instance.btnRaiz,
            btnDividir: instance.btnDividir,
            btn7: instance.btn7,
            btn8: instance.btn8,
            btn9: instance.btn9,
            btnMultiplicar: instance.btnMultiplicar,
            btn4: instance.btn4,
            btn5: instance.btn5,
            btn6: instance.btn6,
            btnSubtrair: instance.btnSubtrair,
            btn1: instance.btn1,
            btn2: instance.btn2,
            btn3: instance.btn3,
            btnSomar: instance.btnSomar,
            btnSinal: instance.btnSinal,
            btn0: instance.btn0,
            btnVirgula: instance.btnVirgula,
            btnIgual: instance.btnIgual,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmCalculadora aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmCalculadoraComponents> {
        ui.form_components(ui.find_path("frmCalculadora")?)
    }

    /// Cria o frmCalculadora (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmCalculadora")
    }

    /// Cria mais um frmCalculadora dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmCalculadora`, `frmCalculadora2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmCalculadoraComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmCalculadora")?;
        let name = ui.free_name_in(parent, "frmCalculadora");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmCalculadora (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmCalculadoraComponents> {
        ui.form_components(root)
    }

    /// O frmCalculadora aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmCalculadora> {
        ui.form_instance(ui.find_path("frmCalculadora")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmCalculadora> {
        let form = ui.find_path("frmCalculadora")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnLimparHistorico" => Some(self.btnLimparHistorico),
            "btnHistorico" => Some(self.btnHistorico),
            "btnMC" => Some(self.btnMC),
            "btnMR" => Some(self.btnMR),
            "btnMMais" => Some(self.btnMMais),
            "btnMMenos" => Some(self.btnMMenos),
            "btnMS" => Some(self.btnMS),
            "btnPorcento" => Some(self.btnPorcento),
            "btnCE" => Some(self.btnCE),
            "btnC" => Some(self.btnC),
            "btnApagar" => Some(self.btnApagar),
            "btnInverso" => Some(self.btnInverso),
            "btnQuadrado" => Some(self.btnQuadrado),
            "btnRaiz" => Some(self.btnRaiz),
            "btnDividir" => Some(self.btnDividir),
            "btn7" => Some(self.btn7),
            "btn8" => Some(self.btn8),
            "btn9" => Some(self.btn9),
            "btnMultiplicar" => Some(self.btnMultiplicar),
            "btn4" => Some(self.btn4),
            "btn5" => Some(self.btn5),
            "btn6" => Some(self.btn6),
            "btnSubtrair" => Some(self.btnSubtrair),
            "btn1" => Some(self.btn1),
            "btn2" => Some(self.btn2),
            "btn3" => Some(self.btn3),
            "btnSomar" => Some(self.btnSomar),
            "btnSinal" => Some(self.btnSinal),
            "btn0" => Some(self.btn0),
            "btnVirgula" => Some(self.btnVirgula),
            "btnIgual" => Some(self.btnIgual),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O GridLayout deste form com o nome (`None` se não há um GridLayout com ele).
    pub fn find_grid_layout(&self, name: &str) -> Option<GridLayout> {
        match name {
            "grdMemoria" => Some(self.grdMemoria),
            "grdTeclado" => Some(self.grdTeclado),
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
            "lblTitulo" => Some(self.lblTitulo),
            "lblExpressao" => Some(self.lblExpressao),
            "lblVisor" => Some(self.lblVisor),
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
            "lytCorpo" => Some(self.lytCorpo),
            "lytTopo" => Some(self.lytTopo),
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
            "lvHistorico" => Some(self.lvHistorico),
            "lvMemoria" => Some(self.lvMemoria),
            _ => None,
        }
    }

    /// O ListView que disparou o evento (`None` se o `sender` não é um ListView deste form).
    pub fn sender_as_list_view(&self, ui: &Ui, sender: ComponentId) -> Option<ListView> {
        self.find_list_view(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlLateral" => Some(self.pnlLateral),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O TabControl deste form com o nome (`None` se não há um TabControl com ele).
    pub fn find_tab_control(&self, name: &str) -> Option<TabControl> {
        match name {
            "tabLateral" => Some(self.tabLateral),
            _ => None,
        }
    }

    /// O TabControl que disparou o evento (`None` se o `sender` não é um TabControl deste form).
    pub fn sender_as_tab_control(&self, ui: &Ui, sender: ComponentId) -> Option<TabControl> {
        self.find_tab_control(ui.name(sender)?)
    }

    /// O TabItem deste form com o nome (`None` se não há um TabItem com ele).
    pub fn find_tab_item(&self, name: &str) -> Option<TabItem> {
        match name {
            "tabHistorico" => Some(self.tabHistorico),
            "tabMemoria" => Some(self.tabMemoria),
            _ => None,
        }
    }

    /// O TabItem que disparou o evento (`None` se o `sender` não é um TabItem deste form).
    pub fn sender_as_tab_item(&self, ui: &Ui, sender: ComponentId) -> Option<TabItem> {
        self.find_tab_item(ui.name(sender)?)
    }
}
