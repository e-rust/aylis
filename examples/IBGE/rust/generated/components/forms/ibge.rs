
/// O form frmIBGE: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmIBGE {
    pub form: Form,
    pub cdsEstados: ClientDataSet,
    pub cdsEstadosId: Field,
    pub cdsEstadosSigla: Field,
    pub cdsEstadosNome: Field,
    pub cdsEstadosRegiao: Field,
    pub cdsEstadosPopulacao: Field,
    pub cdsEstadosArea: Field,
    pub cdsEstadosDensidade: Field,
    pub cdsEstadosPib: Field,
    pub cdsEstadosPibPerCapita: Field,
    pub cdsEstadosEstimativa: Field,
    pub cdsEstadosEstimativaAno: Field,
    pub cdsEstadosMunicipios: Field,
    pub cdsEstadosBandeira: Field,
    pub dsEstados: DataSource,
    pub cdsMunicipios: ClientDataSet,
    pub cdsMunicipiosId: Field,
    pub cdsMunicipiosUfId: Field,
    pub cdsMunicipiosUf: Field,
    pub cdsMunicipiosNome: Field,
    pub cdsMunicipiosMesorregiao: Field,
    pub cdsMunicipiosMicrorregiao: Field,
    pub cdsMunicipiosRegiaoIntermediaria: Field,
    pub cdsMunicipiosRegiaoImediata: Field,
    pub cdsMunicipiosPopulacao: Field,
    pub cdsMunicipiosArea: Field,
    pub cdsMunicipiosDensidade: Field,
    pub cdsMunicipiosPib: Field,
    pub cdsMunicipiosPibParticipacao: Field,
    pub cdsMunicipiosPibPerCapita: Field,
    pub cdsMunicipiosEstimativa: Field,
    pub cdsMunicipiosEstimativaAno: Field,
    pub cdsMunicipiosDistritos: Field,
    pub cdsMunicipiosBandeira: Field,
    pub dsMunicipios: DataSource,
    pub cdsDistritos: ClientDataSet,
    pub cdsDistritosId: Field,
    pub cdsDistritosMunicipioId: Field,
    pub cdsDistritosNome: Field,
    pub dsDistritos: DataSource,
    pub cdsPopulacao: ClientDataSet,
    pub cdsPopulacaoAno: Field,
    pub cdsPopulacaoHabitantes: Field,
    pub dsPopulacao: DataSource,
    pub cdsMaiores: ClientDataSet,
    pub cdsMaioresNome: Field,
    pub cdsMaioresHabitantes: Field,
    pub dsMaiores: DataSource,
    pub lblFonte: Label,
    pub lytListas: Layout,
    pub gbEstados: GroupBox,
    pub grdEstados: DataGrid,
    pub gbDistritos: GroupBox,
    pub grdDistritos: DataGrid,
    pub gbMunicipios: GroupBox,
    pub grdMunicipios: DataGrid,
    pub splListas: Splitter,
    pub lytPainel: Layout,
    pub pnlEstado: Panel,
    pub svgBrasil: Svg,
    pub lytBandeiraEstado: Layout,
    pub lblEstado: Label,
    pub lblRegiao: Label,
    pub imgBandeiraEstado: Image,
    pub lytNumerosEstado: Layout,
    pub dbtEstadoPopulacao: Label,
    pub lblEstadoEstimativa: Label,
    pub dbtEstadoEstimativa: Label,
    pub dbtEstadoArea: Label,
    pub dbtEstadoDensidade: Label,
    pub dbtEstadoPib: Label,
    pub dbtEstadoPibPerCapita: Label,
    pub dbtEstadoMunicipios: Label,
    pub pnlMunicipio: Panel,
    pub svgMunicipio: Svg,
    pub svgEstado: Svg,
    pub lytBandeiraMunicipio: Layout,
    pub lblMunicipio: Label,
    pub lblCodigo: Label,
    pub imgBandeiraMunicipio: Image,
    pub lblSemBandeira: Label,
    pub lytNumerosMunicipio: Layout,
    pub dbtMesorregiao: Label,
    pub dbtMicrorregiao: Label,
    pub dbtRegiaoImediata: Label,
    pub dbtPopulacao: Label,
    pub lblEstimativa: Label,
    pub dbtEstimativa: Label,
    pub dbtArea: Label,
    pub dbtDensidade: Label,
    pub dbtPib: Label,
    pub dbtPibPerCapita: Label,
    pub dbtPibParticipacao: Label,
    pub lytGraficos: Layout,
    pub dbcPopulacao: DBChart,
    pub dbcMaiores: DBChart,
    pub dbcEstados: DBChart,
    pub data: super::ibge::Data,
}

/// Os componentes do frmIBGE sem o estado, para o código fora dos handlers
/// (`FrmIBGE::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmIBGEComponents {
    pub form: Form,
    pub cdsEstados: ClientDataSet,
    pub cdsEstadosId: Field,
    pub cdsEstadosSigla: Field,
    pub cdsEstadosNome: Field,
    pub cdsEstadosRegiao: Field,
    pub cdsEstadosPopulacao: Field,
    pub cdsEstadosArea: Field,
    pub cdsEstadosDensidade: Field,
    pub cdsEstadosPib: Field,
    pub cdsEstadosPibPerCapita: Field,
    pub cdsEstadosEstimativa: Field,
    pub cdsEstadosEstimativaAno: Field,
    pub cdsEstadosMunicipios: Field,
    pub cdsEstadosBandeira: Field,
    pub dsEstados: DataSource,
    pub cdsMunicipios: ClientDataSet,
    pub cdsMunicipiosId: Field,
    pub cdsMunicipiosUfId: Field,
    pub cdsMunicipiosUf: Field,
    pub cdsMunicipiosNome: Field,
    pub cdsMunicipiosMesorregiao: Field,
    pub cdsMunicipiosMicrorregiao: Field,
    pub cdsMunicipiosRegiaoIntermediaria: Field,
    pub cdsMunicipiosRegiaoImediata: Field,
    pub cdsMunicipiosPopulacao: Field,
    pub cdsMunicipiosArea: Field,
    pub cdsMunicipiosDensidade: Field,
    pub cdsMunicipiosPib: Field,
    pub cdsMunicipiosPibParticipacao: Field,
    pub cdsMunicipiosPibPerCapita: Field,
    pub cdsMunicipiosEstimativa: Field,
    pub cdsMunicipiosEstimativaAno: Field,
    pub cdsMunicipiosDistritos: Field,
    pub cdsMunicipiosBandeira: Field,
    pub dsMunicipios: DataSource,
    pub cdsDistritos: ClientDataSet,
    pub cdsDistritosId: Field,
    pub cdsDistritosMunicipioId: Field,
    pub cdsDistritosNome: Field,
    pub dsDistritos: DataSource,
    pub cdsPopulacao: ClientDataSet,
    pub cdsPopulacaoAno: Field,
    pub cdsPopulacaoHabitantes: Field,
    pub dsPopulacao: DataSource,
    pub cdsMaiores: ClientDataSet,
    pub cdsMaioresNome: Field,
    pub cdsMaioresHabitantes: Field,
    pub dsMaiores: DataSource,
    pub lblFonte: Label,
    pub lytListas: Layout,
    pub gbEstados: GroupBox,
    pub grdEstados: DataGrid,
    pub gbDistritos: GroupBox,
    pub grdDistritos: DataGrid,
    pub gbMunicipios: GroupBox,
    pub grdMunicipios: DataGrid,
    pub splListas: Splitter,
    pub lytPainel: Layout,
    pub pnlEstado: Panel,
    pub svgBrasil: Svg,
    pub lytBandeiraEstado: Layout,
    pub lblEstado: Label,
    pub lblRegiao: Label,
    pub imgBandeiraEstado: Image,
    pub lytNumerosEstado: Layout,
    pub dbtEstadoPopulacao: Label,
    pub lblEstadoEstimativa: Label,
    pub dbtEstadoEstimativa: Label,
    pub dbtEstadoArea: Label,
    pub dbtEstadoDensidade: Label,
    pub dbtEstadoPib: Label,
    pub dbtEstadoPibPerCapita: Label,
    pub dbtEstadoMunicipios: Label,
    pub pnlMunicipio: Panel,
    pub svgMunicipio: Svg,
    pub svgEstado: Svg,
    pub lytBandeiraMunicipio: Layout,
    pub lblMunicipio: Label,
    pub lblCodigo: Label,
    pub imgBandeiraMunicipio: Image,
    pub lblSemBandeira: Label,
    pub lytNumerosMunicipio: Layout,
    pub dbtMesorregiao: Label,
    pub dbtMicrorregiao: Label,
    pub dbtRegiaoImediata: Label,
    pub dbtPopulacao: Label,
    pub lblEstimativa: Label,
    pub dbtEstimativa: Label,
    pub dbtArea: Label,
    pub dbtDensidade: Label,
    pub dbtPib: Label,
    pub dbtPibPerCapita: Label,
    pub dbtPibParticipacao: Label,
    pub lytGraficos: Layout,
    pub dbcPopulacao: DBChart,
    pub dbcMaiores: DBChart,
    pub dbcEstados: DBChart,
}

impl std::ops::Deref for FrmIBGE {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmIBGE {
    /// Monta a instância de um frmIBGE recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            cdsEstados: ClientDataSet { id: handles::resolve(ui, root, "cdsEstados", "ClientDataSet", &mut problems) },
            cdsEstadosId: Field { id: handles::resolve(ui, root, "cdsEstadosId", "Field", &mut problems) },
            cdsEstadosSigla: Field { id: handles::resolve(ui, root, "cdsEstadosSigla", "Field", &mut problems) },
            cdsEstadosNome: Field { id: handles::resolve(ui, root, "cdsEstadosNome", "Field", &mut problems) },
            cdsEstadosRegiao: Field { id: handles::resolve(ui, root, "cdsEstadosRegiao", "Field", &mut problems) },
            cdsEstadosPopulacao: Field { id: handles::resolve(ui, root, "cdsEstadosPopulacao", "Field", &mut problems) },
            cdsEstadosArea: Field { id: handles::resolve(ui, root, "cdsEstadosArea", "Field", &mut problems) },
            cdsEstadosDensidade: Field { id: handles::resolve(ui, root, "cdsEstadosDensidade", "Field", &mut problems) },
            cdsEstadosPib: Field { id: handles::resolve(ui, root, "cdsEstadosPib", "Field", &mut problems) },
            cdsEstadosPibPerCapita: Field { id: handles::resolve(ui, root, "cdsEstadosPibPerCapita", "Field", &mut problems) },
            cdsEstadosEstimativa: Field { id: handles::resolve(ui, root, "cdsEstadosEstimativa", "Field", &mut problems) },
            cdsEstadosEstimativaAno: Field { id: handles::resolve(ui, root, "cdsEstadosEstimativaAno", "Field", &mut problems) },
            cdsEstadosMunicipios: Field { id: handles::resolve(ui, root, "cdsEstadosMunicipios", "Field", &mut problems) },
            cdsEstadosBandeira: Field { id: handles::resolve(ui, root, "cdsEstadosBandeira", "Field", &mut problems) },
            dsEstados: DataSource { id: handles::resolve(ui, root, "dsEstados", "DataSource", &mut problems) },
            cdsMunicipios: ClientDataSet { id: handles::resolve(ui, root, "cdsMunicipios", "ClientDataSet", &mut problems) },
            cdsMunicipiosId: Field { id: handles::resolve(ui, root, "cdsMunicipiosId", "Field", &mut problems) },
            cdsMunicipiosUfId: Field { id: handles::resolve(ui, root, "cdsMunicipiosUfId", "Field", &mut problems) },
            cdsMunicipiosUf: Field { id: handles::resolve(ui, root, "cdsMunicipiosUf", "Field", &mut problems) },
            cdsMunicipiosNome: Field { id: handles::resolve(ui, root, "cdsMunicipiosNome", "Field", &mut problems) },
            cdsMunicipiosMesorregiao: Field { id: handles::resolve(ui, root, "cdsMunicipiosMesorregiao", "Field", &mut problems) },
            cdsMunicipiosMicrorregiao: Field { id: handles::resolve(ui, root, "cdsMunicipiosMicrorregiao", "Field", &mut problems) },
            cdsMunicipiosRegiaoIntermediaria: Field { id: handles::resolve(ui, root, "cdsMunicipiosRegiaoIntermediaria", "Field", &mut problems) },
            cdsMunicipiosRegiaoImediata: Field { id: handles::resolve(ui, root, "cdsMunicipiosRegiaoImediata", "Field", &mut problems) },
            cdsMunicipiosPopulacao: Field { id: handles::resolve(ui, root, "cdsMunicipiosPopulacao", "Field", &mut problems) },
            cdsMunicipiosArea: Field { id: handles::resolve(ui, root, "cdsMunicipiosArea", "Field", &mut problems) },
            cdsMunicipiosDensidade: Field { id: handles::resolve(ui, root, "cdsMunicipiosDensidade", "Field", &mut problems) },
            cdsMunicipiosPib: Field { id: handles::resolve(ui, root, "cdsMunicipiosPib", "Field", &mut problems) },
            cdsMunicipiosPibParticipacao: Field { id: handles::resolve(ui, root, "cdsMunicipiosPibParticipacao", "Field", &mut problems) },
            cdsMunicipiosPibPerCapita: Field { id: handles::resolve(ui, root, "cdsMunicipiosPibPerCapita", "Field", &mut problems) },
            cdsMunicipiosEstimativa: Field { id: handles::resolve(ui, root, "cdsMunicipiosEstimativa", "Field", &mut problems) },
            cdsMunicipiosEstimativaAno: Field { id: handles::resolve(ui, root, "cdsMunicipiosEstimativaAno", "Field", &mut problems) },
            cdsMunicipiosDistritos: Field { id: handles::resolve(ui, root, "cdsMunicipiosDistritos", "Field", &mut problems) },
            cdsMunicipiosBandeira: Field { id: handles::resolve(ui, root, "cdsMunicipiosBandeira", "Field", &mut problems) },
            dsMunicipios: DataSource { id: handles::resolve(ui, root, "dsMunicipios", "DataSource", &mut problems) },
            cdsDistritos: ClientDataSet { id: handles::resolve(ui, root, "cdsDistritos", "ClientDataSet", &mut problems) },
            cdsDistritosId: Field { id: handles::resolve(ui, root, "cdsDistritosId", "Field", &mut problems) },
            cdsDistritosMunicipioId: Field { id: handles::resolve(ui, root, "cdsDistritosMunicipioId", "Field", &mut problems) },
            cdsDistritosNome: Field { id: handles::resolve(ui, root, "cdsDistritosNome", "Field", &mut problems) },
            dsDistritos: DataSource { id: handles::resolve(ui, root, "dsDistritos", "DataSource", &mut problems) },
            cdsPopulacao: ClientDataSet { id: handles::resolve(ui, root, "cdsPopulacao", "ClientDataSet", &mut problems) },
            cdsPopulacaoAno: Field { id: handles::resolve(ui, root, "cdsPopulacaoAno", "Field", &mut problems) },
            cdsPopulacaoHabitantes: Field { id: handles::resolve(ui, root, "cdsPopulacaoHabitantes", "Field", &mut problems) },
            dsPopulacao: DataSource { id: handles::resolve(ui, root, "dsPopulacao", "DataSource", &mut problems) },
            cdsMaiores: ClientDataSet { id: handles::resolve(ui, root, "cdsMaiores", "ClientDataSet", &mut problems) },
            cdsMaioresNome: Field { id: handles::resolve(ui, root, "cdsMaioresNome", "Field", &mut problems) },
            cdsMaioresHabitantes: Field { id: handles::resolve(ui, root, "cdsMaioresHabitantes", "Field", &mut problems) },
            dsMaiores: DataSource { id: handles::resolve(ui, root, "dsMaiores", "DataSource", &mut problems) },
            lblFonte: Label { id: handles::resolve(ui, root, "lblFonte", "Label", &mut problems) },
            lytListas: Layout { id: handles::resolve(ui, root, "lytListas", "Layout", &mut problems) },
            gbEstados: GroupBox { id: handles::resolve(ui, root, "gbEstados", "GroupBox", &mut problems) },
            grdEstados: DataGrid { id: handles::resolve(ui, root, "grdEstados", "DataGrid", &mut problems) },
            gbDistritos: GroupBox { id: handles::resolve(ui, root, "gbDistritos", "GroupBox", &mut problems) },
            grdDistritos: DataGrid { id: handles::resolve(ui, root, "grdDistritos", "DataGrid", &mut problems) },
            gbMunicipios: GroupBox { id: handles::resolve(ui, root, "gbMunicipios", "GroupBox", &mut problems) },
            grdMunicipios: DataGrid { id: handles::resolve(ui, root, "grdMunicipios", "DataGrid", &mut problems) },
            splListas: Splitter { id: handles::resolve(ui, root, "splListas", "Splitter", &mut problems) },
            lytPainel: Layout { id: handles::resolve(ui, root, "lytPainel", "Layout", &mut problems) },
            pnlEstado: Panel { id: handles::resolve(ui, root, "pnlEstado", "Panel", &mut problems) },
            svgBrasil: Svg { id: handles::resolve(ui, root, "svgBrasil", "Svg", &mut problems) },
            lytBandeiraEstado: Layout { id: handles::resolve(ui, root, "lytBandeiraEstado", "Layout", &mut problems) },
            lblEstado: Label { id: handles::resolve(ui, root, "lblEstado", "Label", &mut problems) },
            lblRegiao: Label { id: handles::resolve(ui, root, "lblRegiao", "Label", &mut problems) },
            imgBandeiraEstado: Image { id: handles::resolve(ui, root, "imgBandeiraEstado", "Image", &mut problems) },
            lytNumerosEstado: Layout { id: handles::resolve(ui, root, "lytNumerosEstado", "Layout", &mut problems) },
            dbtEstadoPopulacao: Label { id: handles::resolve(ui, root, "dbtEstadoPopulacao", "Label", &mut problems) },
            lblEstadoEstimativa: Label { id: handles::resolve(ui, root, "lblEstadoEstimativa", "Label", &mut problems) },
            dbtEstadoEstimativa: Label { id: handles::resolve(ui, root, "dbtEstadoEstimativa", "Label", &mut problems) },
            dbtEstadoArea: Label { id: handles::resolve(ui, root, "dbtEstadoArea", "Label", &mut problems) },
            dbtEstadoDensidade: Label { id: handles::resolve(ui, root, "dbtEstadoDensidade", "Label", &mut problems) },
            dbtEstadoPib: Label { id: handles::resolve(ui, root, "dbtEstadoPib", "Label", &mut problems) },
            dbtEstadoPibPerCapita: Label { id: handles::resolve(ui, root, "dbtEstadoPibPerCapita", "Label", &mut problems) },
            dbtEstadoMunicipios: Label { id: handles::resolve(ui, root, "dbtEstadoMunicipios", "Label", &mut problems) },
            pnlMunicipio: Panel { id: handles::resolve(ui, root, "pnlMunicipio", "Panel", &mut problems) },
            svgMunicipio: Svg { id: handles::resolve(ui, root, "svgMunicipio", "Svg", &mut problems) },
            svgEstado: Svg { id: handles::resolve(ui, root, "svgEstado", "Svg", &mut problems) },
            lytBandeiraMunicipio: Layout { id: handles::resolve(ui, root, "lytBandeiraMunicipio", "Layout", &mut problems) },
            lblMunicipio: Label { id: handles::resolve(ui, root, "lblMunicipio", "Label", &mut problems) },
            lblCodigo: Label { id: handles::resolve(ui, root, "lblCodigo", "Label", &mut problems) },
            imgBandeiraMunicipio: Image { id: handles::resolve(ui, root, "imgBandeiraMunicipio", "Image", &mut problems) },
            lblSemBandeira: Label { id: handles::resolve(ui, root, "lblSemBandeira", "Label", &mut problems) },
            lytNumerosMunicipio: Layout { id: handles::resolve(ui, root, "lytNumerosMunicipio", "Layout", &mut problems) },
            dbtMesorregiao: Label { id: handles::resolve(ui, root, "dbtMesorregiao", "Label", &mut problems) },
            dbtMicrorregiao: Label { id: handles::resolve(ui, root, "dbtMicrorregiao", "Label", &mut problems) },
            dbtRegiaoImediata: Label { id: handles::resolve(ui, root, "dbtRegiaoImediata", "Label", &mut problems) },
            dbtPopulacao: Label { id: handles::resolve(ui, root, "dbtPopulacao", "Label", &mut problems) },
            lblEstimativa: Label { id: handles::resolve(ui, root, "lblEstimativa", "Label", &mut problems) },
            dbtEstimativa: Label { id: handles::resolve(ui, root, "dbtEstimativa", "Label", &mut problems) },
            dbtArea: Label { id: handles::resolve(ui, root, "dbtArea", "Label", &mut problems) },
            dbtDensidade: Label { id: handles::resolve(ui, root, "dbtDensidade", "Label", &mut problems) },
            dbtPib: Label { id: handles::resolve(ui, root, "dbtPib", "Label", &mut problems) },
            dbtPibPerCapita: Label { id: handles::resolve(ui, root, "dbtPibPerCapita", "Label", &mut problems) },
            dbtPibParticipacao: Label { id: handles::resolve(ui, root, "dbtPibParticipacao", "Label", &mut problems) },
            lytGraficos: Layout { id: handles::resolve(ui, root, "lytGraficos", "Layout", &mut problems) },
            dbcPopulacao: DBChart { id: handles::resolve(ui, root, "dbcPopulacao", "DBChart", &mut problems) },
            dbcMaiores: DBChart { id: handles::resolve(ui, root, "dbcMaiores", "DBChart", &mut problems) },
            dbcEstados: DBChart { id: handles::resolve(ui, root, "dbcEstados", "DBChart", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmIBGE não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmIBGEComponents {
            form: instance.form,
            cdsEstados: instance.cdsEstados,
            cdsEstadosId: instance.cdsEstadosId,
            cdsEstadosSigla: instance.cdsEstadosSigla,
            cdsEstadosNome: instance.cdsEstadosNome,
            cdsEstadosRegiao: instance.cdsEstadosRegiao,
            cdsEstadosPopulacao: instance.cdsEstadosPopulacao,
            cdsEstadosArea: instance.cdsEstadosArea,
            cdsEstadosDensidade: instance.cdsEstadosDensidade,
            cdsEstadosPib: instance.cdsEstadosPib,
            cdsEstadosPibPerCapita: instance.cdsEstadosPibPerCapita,
            cdsEstadosEstimativa: instance.cdsEstadosEstimativa,
            cdsEstadosEstimativaAno: instance.cdsEstadosEstimativaAno,
            cdsEstadosMunicipios: instance.cdsEstadosMunicipios,
            cdsEstadosBandeira: instance.cdsEstadosBandeira,
            dsEstados: instance.dsEstados,
            cdsMunicipios: instance.cdsMunicipios,
            cdsMunicipiosId: instance.cdsMunicipiosId,
            cdsMunicipiosUfId: instance.cdsMunicipiosUfId,
            cdsMunicipiosUf: instance.cdsMunicipiosUf,
            cdsMunicipiosNome: instance.cdsMunicipiosNome,
            cdsMunicipiosMesorregiao: instance.cdsMunicipiosMesorregiao,
            cdsMunicipiosMicrorregiao: instance.cdsMunicipiosMicrorregiao,
            cdsMunicipiosRegiaoIntermediaria: instance.cdsMunicipiosRegiaoIntermediaria,
            cdsMunicipiosRegiaoImediata: instance.cdsMunicipiosRegiaoImediata,
            cdsMunicipiosPopulacao: instance.cdsMunicipiosPopulacao,
            cdsMunicipiosArea: instance.cdsMunicipiosArea,
            cdsMunicipiosDensidade: instance.cdsMunicipiosDensidade,
            cdsMunicipiosPib: instance.cdsMunicipiosPib,
            cdsMunicipiosPibParticipacao: instance.cdsMunicipiosPibParticipacao,
            cdsMunicipiosPibPerCapita: instance.cdsMunicipiosPibPerCapita,
            cdsMunicipiosEstimativa: instance.cdsMunicipiosEstimativa,
            cdsMunicipiosEstimativaAno: instance.cdsMunicipiosEstimativaAno,
            cdsMunicipiosDistritos: instance.cdsMunicipiosDistritos,
            cdsMunicipiosBandeira: instance.cdsMunicipiosBandeira,
            dsMunicipios: instance.dsMunicipios,
            cdsDistritos: instance.cdsDistritos,
            cdsDistritosId: instance.cdsDistritosId,
            cdsDistritosMunicipioId: instance.cdsDistritosMunicipioId,
            cdsDistritosNome: instance.cdsDistritosNome,
            dsDistritos: instance.dsDistritos,
            cdsPopulacao: instance.cdsPopulacao,
            cdsPopulacaoAno: instance.cdsPopulacaoAno,
            cdsPopulacaoHabitantes: instance.cdsPopulacaoHabitantes,
            dsPopulacao: instance.dsPopulacao,
            cdsMaiores: instance.cdsMaiores,
            cdsMaioresNome: instance.cdsMaioresNome,
            cdsMaioresHabitantes: instance.cdsMaioresHabitantes,
            dsMaiores: instance.dsMaiores,
            lblFonte: instance.lblFonte,
            lytListas: instance.lytListas,
            gbEstados: instance.gbEstados,
            grdEstados: instance.grdEstados,
            gbDistritos: instance.gbDistritos,
            grdDistritos: instance.grdDistritos,
            gbMunicipios: instance.gbMunicipios,
            grdMunicipios: instance.grdMunicipios,
            splListas: instance.splListas,
            lytPainel: instance.lytPainel,
            pnlEstado: instance.pnlEstado,
            svgBrasil: instance.svgBrasil,
            lytBandeiraEstado: instance.lytBandeiraEstado,
            lblEstado: instance.lblEstado,
            lblRegiao: instance.lblRegiao,
            imgBandeiraEstado: instance.imgBandeiraEstado,
            lytNumerosEstado: instance.lytNumerosEstado,
            dbtEstadoPopulacao: instance.dbtEstadoPopulacao,
            lblEstadoEstimativa: instance.lblEstadoEstimativa,
            dbtEstadoEstimativa: instance.dbtEstadoEstimativa,
            dbtEstadoArea: instance.dbtEstadoArea,
            dbtEstadoDensidade: instance.dbtEstadoDensidade,
            dbtEstadoPib: instance.dbtEstadoPib,
            dbtEstadoPibPerCapita: instance.dbtEstadoPibPerCapita,
            dbtEstadoMunicipios: instance.dbtEstadoMunicipios,
            pnlMunicipio: instance.pnlMunicipio,
            svgMunicipio: instance.svgMunicipio,
            svgEstado: instance.svgEstado,
            lytBandeiraMunicipio: instance.lytBandeiraMunicipio,
            lblMunicipio: instance.lblMunicipio,
            lblCodigo: instance.lblCodigo,
            imgBandeiraMunicipio: instance.imgBandeiraMunicipio,
            lblSemBandeira: instance.lblSemBandeira,
            lytNumerosMunicipio: instance.lytNumerosMunicipio,
            dbtMesorregiao: instance.dbtMesorregiao,
            dbtMicrorregiao: instance.dbtMicrorregiao,
            dbtRegiaoImediata: instance.dbtRegiaoImediata,
            dbtPopulacao: instance.dbtPopulacao,
            lblEstimativa: instance.lblEstimativa,
            dbtEstimativa: instance.dbtEstimativa,
            dbtArea: instance.dbtArea,
            dbtDensidade: instance.dbtDensidade,
            dbtPib: instance.dbtPib,
            dbtPibPerCapita: instance.dbtPibPerCapita,
            dbtPibParticipacao: instance.dbtPibParticipacao,
            lytGraficos: instance.lytGraficos,
            dbcPopulacao: instance.dbcPopulacao,
            dbcMaiores: instance.dbcMaiores,
            dbcEstados: instance.dbcEstados,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmIBGE aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmIBGEComponents> {
        ui.form_components(ui.find_path("frmIBGE")?)
    }

    /// Cria o frmIBGE (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmIBGE")
    }

    /// Cria mais um frmIBGE dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmIBGE`, `frmIBGE2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmIBGEComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmIBGE")?;
        let name = ui.free_name_in(parent, "frmIBGE");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmIBGE (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmIBGEComponents> {
        ui.form_components(root)
    }

    /// O frmIBGE aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmIBGE> {
        ui.form_instance(ui.find_path("frmIBGE")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmIBGE> {
        let form = ui.find_path("frmIBGE")?;
        ui.form_instance_mut(form)
    }

    /// O ClientDataSet deste form com o nome (`None` se não há um ClientDataSet com ele).
    pub fn find_client_data_set(&self, name: &str) -> Option<ClientDataSet> {
        match name {
            "cdsEstados" => Some(self.cdsEstados),
            "cdsMunicipios" => Some(self.cdsMunicipios),
            "cdsDistritos" => Some(self.cdsDistritos),
            "cdsPopulacao" => Some(self.cdsPopulacao),
            "cdsMaiores" => Some(self.cdsMaiores),
            _ => None,
        }
    }

    /// O ClientDataSet que disparou o evento (`None` se o `sender` não é um ClientDataSet deste form).
    pub fn sender_as_client_data_set(&self, ui: &Ui, sender: ComponentId) -> Option<ClientDataSet> {
        self.find_client_data_set(ui.name(sender)?)
    }

    /// O DBChart deste form com o nome (`None` se não há um DBChart com ele).
    pub fn find_d_b_chart(&self, name: &str) -> Option<DBChart> {
        match name {
            "dbcPopulacao" => Some(self.dbcPopulacao),
            "dbcMaiores" => Some(self.dbcMaiores),
            "dbcEstados" => Some(self.dbcEstados),
            _ => None,
        }
    }

    /// O DBChart que disparou o evento (`None` se o `sender` não é um DBChart deste form).
    pub fn sender_as_d_b_chart(&self, ui: &Ui, sender: ComponentId) -> Option<DBChart> {
        self.find_d_b_chart(ui.name(sender)?)
    }

    /// O DataGrid deste form com o nome (`None` se não há um DataGrid com ele).
    pub fn find_data_grid(&self, name: &str) -> Option<DataGrid> {
        match name {
            "grdEstados" => Some(self.grdEstados),
            "grdDistritos" => Some(self.grdDistritos),
            "grdMunicipios" => Some(self.grdMunicipios),
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
            "dsEstados" => Some(self.dsEstados),
            "dsMunicipios" => Some(self.dsMunicipios),
            "dsDistritos" => Some(self.dsDistritos),
            "dsPopulacao" => Some(self.dsPopulacao),
            "dsMaiores" => Some(self.dsMaiores),
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
            "cdsEstadosId" => Some(self.cdsEstadosId),
            "cdsEstadosSigla" => Some(self.cdsEstadosSigla),
            "cdsEstadosNome" => Some(self.cdsEstadosNome),
            "cdsEstadosRegiao" => Some(self.cdsEstadosRegiao),
            "cdsEstadosPopulacao" => Some(self.cdsEstadosPopulacao),
            "cdsEstadosArea" => Some(self.cdsEstadosArea),
            "cdsEstadosDensidade" => Some(self.cdsEstadosDensidade),
            "cdsEstadosPib" => Some(self.cdsEstadosPib),
            "cdsEstadosPibPerCapita" => Some(self.cdsEstadosPibPerCapita),
            "cdsEstadosEstimativa" => Some(self.cdsEstadosEstimativa),
            "cdsEstadosEstimativaAno" => Some(self.cdsEstadosEstimativaAno),
            "cdsEstadosMunicipios" => Some(self.cdsEstadosMunicipios),
            "cdsEstadosBandeira" => Some(self.cdsEstadosBandeira),
            "cdsMunicipiosId" => Some(self.cdsMunicipiosId),
            "cdsMunicipiosUfId" => Some(self.cdsMunicipiosUfId),
            "cdsMunicipiosUf" => Some(self.cdsMunicipiosUf),
            "cdsMunicipiosNome" => Some(self.cdsMunicipiosNome),
            "cdsMunicipiosMesorregiao" => Some(self.cdsMunicipiosMesorregiao),
            "cdsMunicipiosMicrorregiao" => Some(self.cdsMunicipiosMicrorregiao),
            "cdsMunicipiosRegiaoIntermediaria" => Some(self.cdsMunicipiosRegiaoIntermediaria),
            "cdsMunicipiosRegiaoImediata" => Some(self.cdsMunicipiosRegiaoImediata),
            "cdsMunicipiosPopulacao" => Some(self.cdsMunicipiosPopulacao),
            "cdsMunicipiosArea" => Some(self.cdsMunicipiosArea),
            "cdsMunicipiosDensidade" => Some(self.cdsMunicipiosDensidade),
            "cdsMunicipiosPib" => Some(self.cdsMunicipiosPib),
            "cdsMunicipiosPibParticipacao" => Some(self.cdsMunicipiosPibParticipacao),
            "cdsMunicipiosPibPerCapita" => Some(self.cdsMunicipiosPibPerCapita),
            "cdsMunicipiosEstimativa" => Some(self.cdsMunicipiosEstimativa),
            "cdsMunicipiosEstimativaAno" => Some(self.cdsMunicipiosEstimativaAno),
            "cdsMunicipiosDistritos" => Some(self.cdsMunicipiosDistritos),
            "cdsMunicipiosBandeira" => Some(self.cdsMunicipiosBandeira),
            "cdsDistritosId" => Some(self.cdsDistritosId),
            "cdsDistritosMunicipioId" => Some(self.cdsDistritosMunicipioId),
            "cdsDistritosNome" => Some(self.cdsDistritosNome),
            "cdsPopulacaoAno" => Some(self.cdsPopulacaoAno),
            "cdsPopulacaoHabitantes" => Some(self.cdsPopulacaoHabitantes),
            "cdsMaioresNome" => Some(self.cdsMaioresNome),
            "cdsMaioresHabitantes" => Some(self.cdsMaioresHabitantes),
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
            "gbEstados" => Some(self.gbEstados),
            "gbDistritos" => Some(self.gbDistritos),
            "gbMunicipios" => Some(self.gbMunicipios),
            _ => None,
        }
    }

    /// O GroupBox que disparou o evento (`None` se o `sender` não é um GroupBox deste form).
    pub fn sender_as_group_box(&self, ui: &Ui, sender: ComponentId) -> Option<GroupBox> {
        self.find_group_box(ui.name(sender)?)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgBandeiraEstado" => Some(self.imgBandeiraEstado),
            "imgBandeiraMunicipio" => Some(self.imgBandeiraMunicipio),
            _ => None,
        }
    }

    /// O Image que disparou o evento (`None` se o `sender` não é um Image deste form).
    pub fn sender_as_image(&self, ui: &Ui, sender: ComponentId) -> Option<Image> {
        self.find_image(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblFonte" => Some(self.lblFonte),
            "lblEstado" => Some(self.lblEstado),
            "lblRegiao" => Some(self.lblRegiao),
            "dbtEstadoPopulacao" => Some(self.dbtEstadoPopulacao),
            "lblEstadoEstimativa" => Some(self.lblEstadoEstimativa),
            "dbtEstadoEstimativa" => Some(self.dbtEstadoEstimativa),
            "dbtEstadoArea" => Some(self.dbtEstadoArea),
            "dbtEstadoDensidade" => Some(self.dbtEstadoDensidade),
            "dbtEstadoPib" => Some(self.dbtEstadoPib),
            "dbtEstadoPibPerCapita" => Some(self.dbtEstadoPibPerCapita),
            "dbtEstadoMunicipios" => Some(self.dbtEstadoMunicipios),
            "lblMunicipio" => Some(self.lblMunicipio),
            "lblCodigo" => Some(self.lblCodigo),
            "lblSemBandeira" => Some(self.lblSemBandeira),
            "dbtMesorregiao" => Some(self.dbtMesorregiao),
            "dbtMicrorregiao" => Some(self.dbtMicrorregiao),
            "dbtRegiaoImediata" => Some(self.dbtRegiaoImediata),
            "dbtPopulacao" => Some(self.dbtPopulacao),
            "lblEstimativa" => Some(self.lblEstimativa),
            "dbtEstimativa" => Some(self.dbtEstimativa),
            "dbtArea" => Some(self.dbtArea),
            "dbtDensidade" => Some(self.dbtDensidade),
            "dbtPib" => Some(self.dbtPib),
            "dbtPibPerCapita" => Some(self.dbtPibPerCapita),
            "dbtPibParticipacao" => Some(self.dbtPibParticipacao),
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
            "lytListas" => Some(self.lytListas),
            "lytPainel" => Some(self.lytPainel),
            "lytBandeiraEstado" => Some(self.lytBandeiraEstado),
            "lytNumerosEstado" => Some(self.lytNumerosEstado),
            "lytBandeiraMunicipio" => Some(self.lytBandeiraMunicipio),
            "lytNumerosMunicipio" => Some(self.lytNumerosMunicipio),
            "lytGraficos" => Some(self.lytGraficos),
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
            "pnlEstado" => Some(self.pnlEstado),
            "pnlMunicipio" => Some(self.pnlMunicipio),
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
            "splListas" => Some(self.splListas),
            _ => None,
        }
    }

    /// O Splitter que disparou o evento (`None` se o `sender` não é um Splitter deste form).
    pub fn sender_as_splitter(&self, ui: &Ui, sender: ComponentId) -> Option<Splitter> {
        self.find_splitter(ui.name(sender)?)
    }

    /// O Svg deste form com o nome (`None` se não há um Svg com ele).
    pub fn find_svg(&self, name: &str) -> Option<Svg> {
        match name {
            "svgBrasil" => Some(self.svgBrasil),
            "svgMunicipio" => Some(self.svgMunicipio),
            "svgEstado" => Some(self.svgEstado),
            _ => None,
        }
    }

    /// O Svg que disparou o evento (`None` se o `sender` não é um Svg deste form).
    pub fn sender_as_svg(&self, ui: &Ui, sender: ComponentId) -> Option<Svg> {
        self.find_svg(ui.name(sender)?)
    }
}
