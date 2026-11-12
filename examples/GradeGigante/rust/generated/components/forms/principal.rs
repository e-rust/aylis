
/// O form frmPrincipal: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrincipal {
    pub form: Form,
    pub cdsDados: ClientDataSet,
    pub cdsDadosId: Field,
    pub cdsDadosT0: Field,
    pub cdsDadosT1: Field,
    pub cdsDadosT2: Field,
    pub cdsDadosT3: Field,
    pub cdsDadosT4: Field,
    pub cdsDadosT5: Field,
    pub cdsDadosT6: Field,
    pub cdsDadosT7: Field,
    pub cdsDadosT8: Field,
    pub cdsDadosT9: Field,
    pub cdsDadosT10: Field,
    pub cdsDadosT11: Field,
    pub cdsDadosT12: Field,
    pub cdsDadosT13: Field,
    pub cdsDadosT14: Field,
    pub cdsDadosT15: Field,
    pub cdsDadosT16: Field,
    pub cdsDadosT17: Field,
    pub cdsDadosT18: Field,
    pub cdsDadosT19: Field,
    pub cdsDadosT20: Field,
    pub cdsDadosT21: Field,
    pub cdsDadosT22: Field,
    pub cdsDadosT23: Field,
    pub cdsDadosT24: Field,
    pub cdsDadosT25: Field,
    pub cdsDadosT26: Field,
    pub cdsDadosT27: Field,
    pub cdsDadosT28: Field,
    pub cdsDadosT29: Field,
    pub cdsDadosT30: Field,
    pub cdsDadosT31: Field,
    pub cdsDadosT32: Field,
    pub cdsDadosT33: Field,
    pub cdsDadosT34: Field,
    pub cdsDadosT35: Field,
    pub cdsDadosT36: Field,
    pub cdsDadosT37: Field,
    pub cdsDadosT38: Field,
    pub cdsDadosT39: Field,
    pub cdsDadosT40: Field,
    pub cdsDadosT41: Field,
    pub cdsDadosT42: Field,
    pub cdsDadosT43: Field,
    pub cdsDadosT44: Field,
    pub cdsDadosT45: Field,
    pub cdsDadosT46: Field,
    pub cdsDadosT47: Field,
    pub cdsDadosT48: Field,
    pub cdsDadosT49: Field,
    pub cdsDadosN0: Field,
    pub cdsDadosN1: Field,
    pub cdsDadosN2: Field,
    pub cdsDadosN3: Field,
    pub cdsDadosN4: Field,
    pub cdsDadosN5: Field,
    pub cdsDadosN6: Field,
    pub cdsDadosN7: Field,
    pub cdsDadosN8: Field,
    pub cdsDadosN9: Field,
    pub cdsDadosN10: Field,
    pub cdsDadosN11: Field,
    pub cdsDadosN12: Field,
    pub cdsDadosN13: Field,
    pub cdsDadosN14: Field,
    pub cdsDadosN15: Field,
    pub cdsDadosN16: Field,
    pub cdsDadosN17: Field,
    pub cdsDadosN18: Field,
    pub cdsDadosN19: Field,
    pub cdsDadosN20: Field,
    pub cdsDadosN21: Field,
    pub cdsDadosN22: Field,
    pub cdsDadosN23: Field,
    pub cdsDadosN24: Field,
    pub cdsDadosN25: Field,
    pub cdsDadosN26: Field,
    pub cdsDadosN27: Field,
    pub cdsDadosN28: Field,
    pub cdsDadosN29: Field,
    pub cdsDadosN30: Field,
    pub cdsDadosN31: Field,
    pub cdsDadosN32: Field,
    pub cdsDadosN33: Field,
    pub cdsDadosN34: Field,
    pub cdsDadosN35: Field,
    pub cdsDadosN36: Field,
    pub cdsDadosN37: Field,
    pub cdsDadosN38: Field,
    pub cdsDadosN39: Field,
    pub cdsDadosN40: Field,
    pub cdsDadosN41: Field,
    pub cdsDadosN42: Field,
    pub cdsDadosN43: Field,
    pub cdsDadosN44: Field,
    pub cdsDadosN45: Field,
    pub cdsDadosN46: Field,
    pub cdsDadosN47: Field,
    pub cdsDadosN48: Field,
    pub dsDados: DataSource,
    pub lytTopo: Layout,
    pub lblRegistros: Label,
    pub nbRegistros: NumberBox,
    pub btnCarregar: Button,
    pub btnParar: Button,
    pub pbCarga: ProgressBar,
    pub lblStatus: Label,
    pub lblDica: Label,
    pub grdDados: DataGrid,
    pub data: super::principal::Data,
}

/// Os componentes do frmPrincipal sem o estado, para o código fora dos handlers
/// (`FrmPrincipal::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrincipalComponents {
    pub form: Form,
    pub cdsDados: ClientDataSet,
    pub cdsDadosId: Field,
    pub cdsDadosT0: Field,
    pub cdsDadosT1: Field,
    pub cdsDadosT2: Field,
    pub cdsDadosT3: Field,
    pub cdsDadosT4: Field,
    pub cdsDadosT5: Field,
    pub cdsDadosT6: Field,
    pub cdsDadosT7: Field,
    pub cdsDadosT8: Field,
    pub cdsDadosT9: Field,
    pub cdsDadosT10: Field,
    pub cdsDadosT11: Field,
    pub cdsDadosT12: Field,
    pub cdsDadosT13: Field,
    pub cdsDadosT14: Field,
    pub cdsDadosT15: Field,
    pub cdsDadosT16: Field,
    pub cdsDadosT17: Field,
    pub cdsDadosT18: Field,
    pub cdsDadosT19: Field,
    pub cdsDadosT20: Field,
    pub cdsDadosT21: Field,
    pub cdsDadosT22: Field,
    pub cdsDadosT23: Field,
    pub cdsDadosT24: Field,
    pub cdsDadosT25: Field,
    pub cdsDadosT26: Field,
    pub cdsDadosT27: Field,
    pub cdsDadosT28: Field,
    pub cdsDadosT29: Field,
    pub cdsDadosT30: Field,
    pub cdsDadosT31: Field,
    pub cdsDadosT32: Field,
    pub cdsDadosT33: Field,
    pub cdsDadosT34: Field,
    pub cdsDadosT35: Field,
    pub cdsDadosT36: Field,
    pub cdsDadosT37: Field,
    pub cdsDadosT38: Field,
    pub cdsDadosT39: Field,
    pub cdsDadosT40: Field,
    pub cdsDadosT41: Field,
    pub cdsDadosT42: Field,
    pub cdsDadosT43: Field,
    pub cdsDadosT44: Field,
    pub cdsDadosT45: Field,
    pub cdsDadosT46: Field,
    pub cdsDadosT47: Field,
    pub cdsDadosT48: Field,
    pub cdsDadosT49: Field,
    pub cdsDadosN0: Field,
    pub cdsDadosN1: Field,
    pub cdsDadosN2: Field,
    pub cdsDadosN3: Field,
    pub cdsDadosN4: Field,
    pub cdsDadosN5: Field,
    pub cdsDadosN6: Field,
    pub cdsDadosN7: Field,
    pub cdsDadosN8: Field,
    pub cdsDadosN9: Field,
    pub cdsDadosN10: Field,
    pub cdsDadosN11: Field,
    pub cdsDadosN12: Field,
    pub cdsDadosN13: Field,
    pub cdsDadosN14: Field,
    pub cdsDadosN15: Field,
    pub cdsDadosN16: Field,
    pub cdsDadosN17: Field,
    pub cdsDadosN18: Field,
    pub cdsDadosN19: Field,
    pub cdsDadosN20: Field,
    pub cdsDadosN21: Field,
    pub cdsDadosN22: Field,
    pub cdsDadosN23: Field,
    pub cdsDadosN24: Field,
    pub cdsDadosN25: Field,
    pub cdsDadosN26: Field,
    pub cdsDadosN27: Field,
    pub cdsDadosN28: Field,
    pub cdsDadosN29: Field,
    pub cdsDadosN30: Field,
    pub cdsDadosN31: Field,
    pub cdsDadosN32: Field,
    pub cdsDadosN33: Field,
    pub cdsDadosN34: Field,
    pub cdsDadosN35: Field,
    pub cdsDadosN36: Field,
    pub cdsDadosN37: Field,
    pub cdsDadosN38: Field,
    pub cdsDadosN39: Field,
    pub cdsDadosN40: Field,
    pub cdsDadosN41: Field,
    pub cdsDadosN42: Field,
    pub cdsDadosN43: Field,
    pub cdsDadosN44: Field,
    pub cdsDadosN45: Field,
    pub cdsDadosN46: Field,
    pub cdsDadosN47: Field,
    pub cdsDadosN48: Field,
    pub dsDados: DataSource,
    pub lytTopo: Layout,
    pub lblRegistros: Label,
    pub nbRegistros: NumberBox,
    pub btnCarregar: Button,
    pub btnParar: Button,
    pub pbCarga: ProgressBar,
    pub lblStatus: Label,
    pub lblDica: Label,
    pub grdDados: DataGrid,
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
            cdsDados: ClientDataSet { id: handles::resolve(ui, root, "cdsDados", "ClientDataSet", &mut problems) },
            cdsDadosId: Field { id: handles::resolve(ui, root, "cdsDadosId", "Field", &mut problems) },
            cdsDadosT0: Field { id: handles::resolve(ui, root, "cdsDadosT0", "Field", &mut problems) },
            cdsDadosT1: Field { id: handles::resolve(ui, root, "cdsDadosT1", "Field", &mut problems) },
            cdsDadosT2: Field { id: handles::resolve(ui, root, "cdsDadosT2", "Field", &mut problems) },
            cdsDadosT3: Field { id: handles::resolve(ui, root, "cdsDadosT3", "Field", &mut problems) },
            cdsDadosT4: Field { id: handles::resolve(ui, root, "cdsDadosT4", "Field", &mut problems) },
            cdsDadosT5: Field { id: handles::resolve(ui, root, "cdsDadosT5", "Field", &mut problems) },
            cdsDadosT6: Field { id: handles::resolve(ui, root, "cdsDadosT6", "Field", &mut problems) },
            cdsDadosT7: Field { id: handles::resolve(ui, root, "cdsDadosT7", "Field", &mut problems) },
            cdsDadosT8: Field { id: handles::resolve(ui, root, "cdsDadosT8", "Field", &mut problems) },
            cdsDadosT9: Field { id: handles::resolve(ui, root, "cdsDadosT9", "Field", &mut problems) },
            cdsDadosT10: Field { id: handles::resolve(ui, root, "cdsDadosT10", "Field", &mut problems) },
            cdsDadosT11: Field { id: handles::resolve(ui, root, "cdsDadosT11", "Field", &mut problems) },
            cdsDadosT12: Field { id: handles::resolve(ui, root, "cdsDadosT12", "Field", &mut problems) },
            cdsDadosT13: Field { id: handles::resolve(ui, root, "cdsDadosT13", "Field", &mut problems) },
            cdsDadosT14: Field { id: handles::resolve(ui, root, "cdsDadosT14", "Field", &mut problems) },
            cdsDadosT15: Field { id: handles::resolve(ui, root, "cdsDadosT15", "Field", &mut problems) },
            cdsDadosT16: Field { id: handles::resolve(ui, root, "cdsDadosT16", "Field", &mut problems) },
            cdsDadosT17: Field { id: handles::resolve(ui, root, "cdsDadosT17", "Field", &mut problems) },
            cdsDadosT18: Field { id: handles::resolve(ui, root, "cdsDadosT18", "Field", &mut problems) },
            cdsDadosT19: Field { id: handles::resolve(ui, root, "cdsDadosT19", "Field", &mut problems) },
            cdsDadosT20: Field { id: handles::resolve(ui, root, "cdsDadosT20", "Field", &mut problems) },
            cdsDadosT21: Field { id: handles::resolve(ui, root, "cdsDadosT21", "Field", &mut problems) },
            cdsDadosT22: Field { id: handles::resolve(ui, root, "cdsDadosT22", "Field", &mut problems) },
            cdsDadosT23: Field { id: handles::resolve(ui, root, "cdsDadosT23", "Field", &mut problems) },
            cdsDadosT24: Field { id: handles::resolve(ui, root, "cdsDadosT24", "Field", &mut problems) },
            cdsDadosT25: Field { id: handles::resolve(ui, root, "cdsDadosT25", "Field", &mut problems) },
            cdsDadosT26: Field { id: handles::resolve(ui, root, "cdsDadosT26", "Field", &mut problems) },
            cdsDadosT27: Field { id: handles::resolve(ui, root, "cdsDadosT27", "Field", &mut problems) },
            cdsDadosT28: Field { id: handles::resolve(ui, root, "cdsDadosT28", "Field", &mut problems) },
            cdsDadosT29: Field { id: handles::resolve(ui, root, "cdsDadosT29", "Field", &mut problems) },
            cdsDadosT30: Field { id: handles::resolve(ui, root, "cdsDadosT30", "Field", &mut problems) },
            cdsDadosT31: Field { id: handles::resolve(ui, root, "cdsDadosT31", "Field", &mut problems) },
            cdsDadosT32: Field { id: handles::resolve(ui, root, "cdsDadosT32", "Field", &mut problems) },
            cdsDadosT33: Field { id: handles::resolve(ui, root, "cdsDadosT33", "Field", &mut problems) },
            cdsDadosT34: Field { id: handles::resolve(ui, root, "cdsDadosT34", "Field", &mut problems) },
            cdsDadosT35: Field { id: handles::resolve(ui, root, "cdsDadosT35", "Field", &mut problems) },
            cdsDadosT36: Field { id: handles::resolve(ui, root, "cdsDadosT36", "Field", &mut problems) },
            cdsDadosT37: Field { id: handles::resolve(ui, root, "cdsDadosT37", "Field", &mut problems) },
            cdsDadosT38: Field { id: handles::resolve(ui, root, "cdsDadosT38", "Field", &mut problems) },
            cdsDadosT39: Field { id: handles::resolve(ui, root, "cdsDadosT39", "Field", &mut problems) },
            cdsDadosT40: Field { id: handles::resolve(ui, root, "cdsDadosT40", "Field", &mut problems) },
            cdsDadosT41: Field { id: handles::resolve(ui, root, "cdsDadosT41", "Field", &mut problems) },
            cdsDadosT42: Field { id: handles::resolve(ui, root, "cdsDadosT42", "Field", &mut problems) },
            cdsDadosT43: Field { id: handles::resolve(ui, root, "cdsDadosT43", "Field", &mut problems) },
            cdsDadosT44: Field { id: handles::resolve(ui, root, "cdsDadosT44", "Field", &mut problems) },
            cdsDadosT45: Field { id: handles::resolve(ui, root, "cdsDadosT45", "Field", &mut problems) },
            cdsDadosT46: Field { id: handles::resolve(ui, root, "cdsDadosT46", "Field", &mut problems) },
            cdsDadosT47: Field { id: handles::resolve(ui, root, "cdsDadosT47", "Field", &mut problems) },
            cdsDadosT48: Field { id: handles::resolve(ui, root, "cdsDadosT48", "Field", &mut problems) },
            cdsDadosT49: Field { id: handles::resolve(ui, root, "cdsDadosT49", "Field", &mut problems) },
            cdsDadosN0: Field { id: handles::resolve(ui, root, "cdsDadosN0", "Field", &mut problems) },
            cdsDadosN1: Field { id: handles::resolve(ui, root, "cdsDadosN1", "Field", &mut problems) },
            cdsDadosN2: Field { id: handles::resolve(ui, root, "cdsDadosN2", "Field", &mut problems) },
            cdsDadosN3: Field { id: handles::resolve(ui, root, "cdsDadosN3", "Field", &mut problems) },
            cdsDadosN4: Field { id: handles::resolve(ui, root, "cdsDadosN4", "Field", &mut problems) },
            cdsDadosN5: Field { id: handles::resolve(ui, root, "cdsDadosN5", "Field", &mut problems) },
            cdsDadosN6: Field { id: handles::resolve(ui, root, "cdsDadosN6", "Field", &mut problems) },
            cdsDadosN7: Field { id: handles::resolve(ui, root, "cdsDadosN7", "Field", &mut problems) },
            cdsDadosN8: Field { id: handles::resolve(ui, root, "cdsDadosN8", "Field", &mut problems) },
            cdsDadosN9: Field { id: handles::resolve(ui, root, "cdsDadosN9", "Field", &mut problems) },
            cdsDadosN10: Field { id: handles::resolve(ui, root, "cdsDadosN10", "Field", &mut problems) },
            cdsDadosN11: Field { id: handles::resolve(ui, root, "cdsDadosN11", "Field", &mut problems) },
            cdsDadosN12: Field { id: handles::resolve(ui, root, "cdsDadosN12", "Field", &mut problems) },
            cdsDadosN13: Field { id: handles::resolve(ui, root, "cdsDadosN13", "Field", &mut problems) },
            cdsDadosN14: Field { id: handles::resolve(ui, root, "cdsDadosN14", "Field", &mut problems) },
            cdsDadosN15: Field { id: handles::resolve(ui, root, "cdsDadosN15", "Field", &mut problems) },
            cdsDadosN16: Field { id: handles::resolve(ui, root, "cdsDadosN16", "Field", &mut problems) },
            cdsDadosN17: Field { id: handles::resolve(ui, root, "cdsDadosN17", "Field", &mut problems) },
            cdsDadosN18: Field { id: handles::resolve(ui, root, "cdsDadosN18", "Field", &mut problems) },
            cdsDadosN19: Field { id: handles::resolve(ui, root, "cdsDadosN19", "Field", &mut problems) },
            cdsDadosN20: Field { id: handles::resolve(ui, root, "cdsDadosN20", "Field", &mut problems) },
            cdsDadosN21: Field { id: handles::resolve(ui, root, "cdsDadosN21", "Field", &mut problems) },
            cdsDadosN22: Field { id: handles::resolve(ui, root, "cdsDadosN22", "Field", &mut problems) },
            cdsDadosN23: Field { id: handles::resolve(ui, root, "cdsDadosN23", "Field", &mut problems) },
            cdsDadosN24: Field { id: handles::resolve(ui, root, "cdsDadosN24", "Field", &mut problems) },
            cdsDadosN25: Field { id: handles::resolve(ui, root, "cdsDadosN25", "Field", &mut problems) },
            cdsDadosN26: Field { id: handles::resolve(ui, root, "cdsDadosN26", "Field", &mut problems) },
            cdsDadosN27: Field { id: handles::resolve(ui, root, "cdsDadosN27", "Field", &mut problems) },
            cdsDadosN28: Field { id: handles::resolve(ui, root, "cdsDadosN28", "Field", &mut problems) },
            cdsDadosN29: Field { id: handles::resolve(ui, root, "cdsDadosN29", "Field", &mut problems) },
            cdsDadosN30: Field { id: handles::resolve(ui, root, "cdsDadosN30", "Field", &mut problems) },
            cdsDadosN31: Field { id: handles::resolve(ui, root, "cdsDadosN31", "Field", &mut problems) },
            cdsDadosN32: Field { id: handles::resolve(ui, root, "cdsDadosN32", "Field", &mut problems) },
            cdsDadosN33: Field { id: handles::resolve(ui, root, "cdsDadosN33", "Field", &mut problems) },
            cdsDadosN34: Field { id: handles::resolve(ui, root, "cdsDadosN34", "Field", &mut problems) },
            cdsDadosN35: Field { id: handles::resolve(ui, root, "cdsDadosN35", "Field", &mut problems) },
            cdsDadosN36: Field { id: handles::resolve(ui, root, "cdsDadosN36", "Field", &mut problems) },
            cdsDadosN37: Field { id: handles::resolve(ui, root, "cdsDadosN37", "Field", &mut problems) },
            cdsDadosN38: Field { id: handles::resolve(ui, root, "cdsDadosN38", "Field", &mut problems) },
            cdsDadosN39: Field { id: handles::resolve(ui, root, "cdsDadosN39", "Field", &mut problems) },
            cdsDadosN40: Field { id: handles::resolve(ui, root, "cdsDadosN40", "Field", &mut problems) },
            cdsDadosN41: Field { id: handles::resolve(ui, root, "cdsDadosN41", "Field", &mut problems) },
            cdsDadosN42: Field { id: handles::resolve(ui, root, "cdsDadosN42", "Field", &mut problems) },
            cdsDadosN43: Field { id: handles::resolve(ui, root, "cdsDadosN43", "Field", &mut problems) },
            cdsDadosN44: Field { id: handles::resolve(ui, root, "cdsDadosN44", "Field", &mut problems) },
            cdsDadosN45: Field { id: handles::resolve(ui, root, "cdsDadosN45", "Field", &mut problems) },
            cdsDadosN46: Field { id: handles::resolve(ui, root, "cdsDadosN46", "Field", &mut problems) },
            cdsDadosN47: Field { id: handles::resolve(ui, root, "cdsDadosN47", "Field", &mut problems) },
            cdsDadosN48: Field { id: handles::resolve(ui, root, "cdsDadosN48", "Field", &mut problems) },
            dsDados: DataSource { id: handles::resolve(ui, root, "dsDados", "DataSource", &mut problems) },
            lytTopo: Layout { id: handles::resolve(ui, root, "lytTopo", "Layout", &mut problems) },
            lblRegistros: Label { id: handles::resolve(ui, root, "lblRegistros", "Label", &mut problems) },
            nbRegistros: NumberBox { id: handles::resolve(ui, root, "nbRegistros", "NumberBox", &mut problems) },
            btnCarregar: Button { id: handles::resolve(ui, root, "btnCarregar", "Button", &mut problems) },
            btnParar: Button { id: handles::resolve(ui, root, "btnParar", "Button", &mut problems) },
            pbCarga: ProgressBar { id: handles::resolve(ui, root, "pbCarga", "ProgressBar", &mut problems) },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            lblDica: Label { id: handles::resolve(ui, root, "lblDica", "Label", &mut problems) },
            grdDados: DataGrid { id: handles::resolve(ui, root, "grdDados", "DataGrid", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrincipal não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrincipalComponents {
            form: instance.form,
            cdsDados: instance.cdsDados,
            cdsDadosId: instance.cdsDadosId,
            cdsDadosT0: instance.cdsDadosT0,
            cdsDadosT1: instance.cdsDadosT1,
            cdsDadosT2: instance.cdsDadosT2,
            cdsDadosT3: instance.cdsDadosT3,
            cdsDadosT4: instance.cdsDadosT4,
            cdsDadosT5: instance.cdsDadosT5,
            cdsDadosT6: instance.cdsDadosT6,
            cdsDadosT7: instance.cdsDadosT7,
            cdsDadosT8: instance.cdsDadosT8,
            cdsDadosT9: instance.cdsDadosT9,
            cdsDadosT10: instance.cdsDadosT10,
            cdsDadosT11: instance.cdsDadosT11,
            cdsDadosT12: instance.cdsDadosT12,
            cdsDadosT13: instance.cdsDadosT13,
            cdsDadosT14: instance.cdsDadosT14,
            cdsDadosT15: instance.cdsDadosT15,
            cdsDadosT16: instance.cdsDadosT16,
            cdsDadosT17: instance.cdsDadosT17,
            cdsDadosT18: instance.cdsDadosT18,
            cdsDadosT19: instance.cdsDadosT19,
            cdsDadosT20: instance.cdsDadosT20,
            cdsDadosT21: instance.cdsDadosT21,
            cdsDadosT22: instance.cdsDadosT22,
            cdsDadosT23: instance.cdsDadosT23,
            cdsDadosT24: instance.cdsDadosT24,
            cdsDadosT25: instance.cdsDadosT25,
            cdsDadosT26: instance.cdsDadosT26,
            cdsDadosT27: instance.cdsDadosT27,
            cdsDadosT28: instance.cdsDadosT28,
            cdsDadosT29: instance.cdsDadosT29,
            cdsDadosT30: instance.cdsDadosT30,
            cdsDadosT31: instance.cdsDadosT31,
            cdsDadosT32: instance.cdsDadosT32,
            cdsDadosT33: instance.cdsDadosT33,
            cdsDadosT34: instance.cdsDadosT34,
            cdsDadosT35: instance.cdsDadosT35,
            cdsDadosT36: instance.cdsDadosT36,
            cdsDadosT37: instance.cdsDadosT37,
            cdsDadosT38: instance.cdsDadosT38,
            cdsDadosT39: instance.cdsDadosT39,
            cdsDadosT40: instance.cdsDadosT40,
            cdsDadosT41: instance.cdsDadosT41,
            cdsDadosT42: instance.cdsDadosT42,
            cdsDadosT43: instance.cdsDadosT43,
            cdsDadosT44: instance.cdsDadosT44,
            cdsDadosT45: instance.cdsDadosT45,
            cdsDadosT46: instance.cdsDadosT46,
            cdsDadosT47: instance.cdsDadosT47,
            cdsDadosT48: instance.cdsDadosT48,
            cdsDadosT49: instance.cdsDadosT49,
            cdsDadosN0: instance.cdsDadosN0,
            cdsDadosN1: instance.cdsDadosN1,
            cdsDadosN2: instance.cdsDadosN2,
            cdsDadosN3: instance.cdsDadosN3,
            cdsDadosN4: instance.cdsDadosN4,
            cdsDadosN5: instance.cdsDadosN5,
            cdsDadosN6: instance.cdsDadosN6,
            cdsDadosN7: instance.cdsDadosN7,
            cdsDadosN8: instance.cdsDadosN8,
            cdsDadosN9: instance.cdsDadosN9,
            cdsDadosN10: instance.cdsDadosN10,
            cdsDadosN11: instance.cdsDadosN11,
            cdsDadosN12: instance.cdsDadosN12,
            cdsDadosN13: instance.cdsDadosN13,
            cdsDadosN14: instance.cdsDadosN14,
            cdsDadosN15: instance.cdsDadosN15,
            cdsDadosN16: instance.cdsDadosN16,
            cdsDadosN17: instance.cdsDadosN17,
            cdsDadosN18: instance.cdsDadosN18,
            cdsDadosN19: instance.cdsDadosN19,
            cdsDadosN20: instance.cdsDadosN20,
            cdsDadosN21: instance.cdsDadosN21,
            cdsDadosN22: instance.cdsDadosN22,
            cdsDadosN23: instance.cdsDadosN23,
            cdsDadosN24: instance.cdsDadosN24,
            cdsDadosN25: instance.cdsDadosN25,
            cdsDadosN26: instance.cdsDadosN26,
            cdsDadosN27: instance.cdsDadosN27,
            cdsDadosN28: instance.cdsDadosN28,
            cdsDadosN29: instance.cdsDadosN29,
            cdsDadosN30: instance.cdsDadosN30,
            cdsDadosN31: instance.cdsDadosN31,
            cdsDadosN32: instance.cdsDadosN32,
            cdsDadosN33: instance.cdsDadosN33,
            cdsDadosN34: instance.cdsDadosN34,
            cdsDadosN35: instance.cdsDadosN35,
            cdsDadosN36: instance.cdsDadosN36,
            cdsDadosN37: instance.cdsDadosN37,
            cdsDadosN38: instance.cdsDadosN38,
            cdsDadosN39: instance.cdsDadosN39,
            cdsDadosN40: instance.cdsDadosN40,
            cdsDadosN41: instance.cdsDadosN41,
            cdsDadosN42: instance.cdsDadosN42,
            cdsDadosN43: instance.cdsDadosN43,
            cdsDadosN44: instance.cdsDadosN44,
            cdsDadosN45: instance.cdsDadosN45,
            cdsDadosN46: instance.cdsDadosN46,
            cdsDadosN47: instance.cdsDadosN47,
            cdsDadosN48: instance.cdsDadosN48,
            dsDados: instance.dsDados,
            lytTopo: instance.lytTopo,
            lblRegistros: instance.lblRegistros,
            nbRegistros: instance.nbRegistros,
            btnCarregar: instance.btnCarregar,
            btnParar: instance.btnParar,
            pbCarga: instance.pbCarga,
            lblStatus: instance.lblStatus,
            lblDica: instance.lblDica,
            grdDados: instance.grdDados,
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
            "btnCarregar" => Some(self.btnCarregar),
            "btnParar" => Some(self.btnParar),
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
            "cdsDados" => Some(self.cdsDados),
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
            "grdDados" => Some(self.grdDados),
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
            "dsDados" => Some(self.dsDados),
            _ => None,
        }
    }

    /// O DataSource que disparou o evento (`None` se o `sender` não é um DataSource deste form).
    pub fn sender_as_data_source(&self, ui: &Ui, sender: ComponentId) -> Option<DataSource> {
        self.find_data_source(ui.name(sender)?)
    }

    /// O Field deste form com o nome (`None` se não há um Field com ele).
    pub fn find_field(&self, name: &str) -> Option<Field> {
        match name {
            "cdsDadosId" => Some(self.cdsDadosId),
            "cdsDadosT0" => Some(self.cdsDadosT0),
            "cdsDadosT1" => Some(self.cdsDadosT1),
            "cdsDadosT2" => Some(self.cdsDadosT2),
            "cdsDadosT3" => Some(self.cdsDadosT3),
            "cdsDadosT4" => Some(self.cdsDadosT4),
            "cdsDadosT5" => Some(self.cdsDadosT5),
            "cdsDadosT6" => Some(self.cdsDadosT6),
            "cdsDadosT7" => Some(self.cdsDadosT7),
            "cdsDadosT8" => Some(self.cdsDadosT8),
            "cdsDadosT9" => Some(self.cdsDadosT9),
            "cdsDadosT10" => Some(self.cdsDadosT10),
            "cdsDadosT11" => Some(self.cdsDadosT11),
            "cdsDadosT12" => Some(self.cdsDadosT12),
            "cdsDadosT13" => Some(self.cdsDadosT13),
            "cdsDadosT14" => Some(self.cdsDadosT14),
            "cdsDadosT15" => Some(self.cdsDadosT15),
            "cdsDadosT16" => Some(self.cdsDadosT16),
            "cdsDadosT17" => Some(self.cdsDadosT17),
            "cdsDadosT18" => Some(self.cdsDadosT18),
            "cdsDadosT19" => Some(self.cdsDadosT19),
            "cdsDadosT20" => Some(self.cdsDadosT20),
            "cdsDadosT21" => Some(self.cdsDadosT21),
            "cdsDadosT22" => Some(self.cdsDadosT22),
            "cdsDadosT23" => Some(self.cdsDadosT23),
            "cdsDadosT24" => Some(self.cdsDadosT24),
            "cdsDadosT25" => Some(self.cdsDadosT25),
            "cdsDadosT26" => Some(self.cdsDadosT26),
            "cdsDadosT27" => Some(self.cdsDadosT27),
            "cdsDadosT28" => Some(self.cdsDadosT28),
            "cdsDadosT29" => Some(self.cdsDadosT29),
            "cdsDadosT30" => Some(self.cdsDadosT30),
            "cdsDadosT31" => Some(self.cdsDadosT31),
            "cdsDadosT32" => Some(self.cdsDadosT32),
            "cdsDadosT33" => Some(self.cdsDadosT33),
            "cdsDadosT34" => Some(self.cdsDadosT34),
            "cdsDadosT35" => Some(self.cdsDadosT35),
            "cdsDadosT36" => Some(self.cdsDadosT36),
            "cdsDadosT37" => Some(self.cdsDadosT37),
            "cdsDadosT38" => Some(self.cdsDadosT38),
            "cdsDadosT39" => Some(self.cdsDadosT39),
            "cdsDadosT40" => Some(self.cdsDadosT40),
            "cdsDadosT41" => Some(self.cdsDadosT41),
            "cdsDadosT42" => Some(self.cdsDadosT42),
            "cdsDadosT43" => Some(self.cdsDadosT43),
            "cdsDadosT44" => Some(self.cdsDadosT44),
            "cdsDadosT45" => Some(self.cdsDadosT45),
            "cdsDadosT46" => Some(self.cdsDadosT46),
            "cdsDadosT47" => Some(self.cdsDadosT47),
            "cdsDadosT48" => Some(self.cdsDadosT48),
            "cdsDadosT49" => Some(self.cdsDadosT49),
            "cdsDadosN0" => Some(self.cdsDadosN0),
            "cdsDadosN1" => Some(self.cdsDadosN1),
            "cdsDadosN2" => Some(self.cdsDadosN2),
            "cdsDadosN3" => Some(self.cdsDadosN3),
            "cdsDadosN4" => Some(self.cdsDadosN4),
            "cdsDadosN5" => Some(self.cdsDadosN5),
            "cdsDadosN6" => Some(self.cdsDadosN6),
            "cdsDadosN7" => Some(self.cdsDadosN7),
            "cdsDadosN8" => Some(self.cdsDadosN8),
            "cdsDadosN9" => Some(self.cdsDadosN9),
            "cdsDadosN10" => Some(self.cdsDadosN10),
            "cdsDadosN11" => Some(self.cdsDadosN11),
            "cdsDadosN12" => Some(self.cdsDadosN12),
            "cdsDadosN13" => Some(self.cdsDadosN13),
            "cdsDadosN14" => Some(self.cdsDadosN14),
            "cdsDadosN15" => Some(self.cdsDadosN15),
            "cdsDadosN16" => Some(self.cdsDadosN16),
            "cdsDadosN17" => Some(self.cdsDadosN17),
            "cdsDadosN18" => Some(self.cdsDadosN18),
            "cdsDadosN19" => Some(self.cdsDadosN19),
            "cdsDadosN20" => Some(self.cdsDadosN20),
            "cdsDadosN21" => Some(self.cdsDadosN21),
            "cdsDadosN22" => Some(self.cdsDadosN22),
            "cdsDadosN23" => Some(self.cdsDadosN23),
            "cdsDadosN24" => Some(self.cdsDadosN24),
            "cdsDadosN25" => Some(self.cdsDadosN25),
            "cdsDadosN26" => Some(self.cdsDadosN26),
            "cdsDadosN27" => Some(self.cdsDadosN27),
            "cdsDadosN28" => Some(self.cdsDadosN28),
            "cdsDadosN29" => Some(self.cdsDadosN29),
            "cdsDadosN30" => Some(self.cdsDadosN30),
            "cdsDadosN31" => Some(self.cdsDadosN31),
            "cdsDadosN32" => Some(self.cdsDadosN32),
            "cdsDadosN33" => Some(self.cdsDadosN33),
            "cdsDadosN34" => Some(self.cdsDadosN34),
            "cdsDadosN35" => Some(self.cdsDadosN35),
            "cdsDadosN36" => Some(self.cdsDadosN36),
            "cdsDadosN37" => Some(self.cdsDadosN37),
            "cdsDadosN38" => Some(self.cdsDadosN38),
            "cdsDadosN39" => Some(self.cdsDadosN39),
            "cdsDadosN40" => Some(self.cdsDadosN40),
            "cdsDadosN41" => Some(self.cdsDadosN41),
            "cdsDadosN42" => Some(self.cdsDadosN42),
            "cdsDadosN43" => Some(self.cdsDadosN43),
            "cdsDadosN44" => Some(self.cdsDadosN44),
            "cdsDadosN45" => Some(self.cdsDadosN45),
            "cdsDadosN46" => Some(self.cdsDadosN46),
            "cdsDadosN47" => Some(self.cdsDadosN47),
            "cdsDadosN48" => Some(self.cdsDadosN48),
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
            "lblRegistros" => Some(self.lblRegistros),
            "lblStatus" => Some(self.lblStatus),
            "lblDica" => Some(self.lblDica),
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
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O NumberBox deste form com o nome (`None` se não há um NumberBox com ele).
    pub fn find_number_box(&self, name: &str) -> Option<NumberBox> {
        match name {
            "nbRegistros" => Some(self.nbRegistros),
            _ => None,
        }
    }

    /// O NumberBox que disparou o evento (`None` se o `sender` não é um NumberBox deste form).
    pub fn sender_as_number_box(&self, ui: &Ui, sender: ComponentId) -> Option<NumberBox> {
        self.find_number_box(ui.name(sender)?)
    }

    /// O ProgressBar deste form com o nome (`None` se não há um ProgressBar com ele).
    pub fn find_progress_bar(&self, name: &str) -> Option<ProgressBar> {
        match name {
            "pbCarga" => Some(self.pbCarga),
            _ => None,
        }
    }

    /// O ProgressBar que disparou o evento (`None` se o `sender` não é um ProgressBar deste form).
    pub fn sender_as_progress_bar(&self, ui: &Ui, sender: ComponentId) -> Option<ProgressBar> {
        self.find_progress_bar(ui.name(sender)?)
    }
}
