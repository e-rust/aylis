
/// O form frmPrincipal: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrincipal {
    pub form: Form,
    pub cdsPedidos: ClientDataSet,
    pub cdsPedidosNumero: Field,
    pub cdsPedidosEmissao: Field,
    pub cdsPedidosCliente: Field,
    pub cdsPedidosCidade: Field,
    pub cdsPedidosUf: Field,
    pub cdsPedidosVendedor: Field,
    pub cdsPedidosSituacao: Field,
    pub cdsPedidosItens: Field,
    pub cdsPedidosPeso: Field,
    pub cdsPedidosTotal: Field,
    pub dsPedidos: DataSource,
    pub cdsItens: ClientDataSet,
    pub cdsItensPedido: Field,
    pub cdsItensItem: Field,
    pub cdsItensCodigo: Field,
    pub cdsItensProduto: Field,
    pub cdsItensUnidade: Field,
    pub cdsItensQuantidade: Field,
    pub cdsItensPreco: Field,
    pub cdsItensDesconto: Field,
    pub cdsItensTotal: Field,
    pub dsItens: DataSource,
    pub cdsParcelas: ClientDataSet,
    pub cdsParcelasPedido: Field,
    pub cdsParcelasParcela: Field,
    pub cdsParcelasVencimento: Field,
    pub cdsParcelasForma: Field,
    pub cdsParcelasValor: Field,
    pub dsParcelas: DataSource,
    pub tabPrincipal: TabControl,
    pub tabDados: TabItem,
    pub lblDica: Label,
    pub gbDetalhes: GroupBox,
    pub grdParcelas: DataGrid,
    pub splDetalhes: Splitter,
    pub grdItens: DataGrid,
    pub splDados: Splitter,
    pub grdPedidos: DataGrid,
    pub tabPedidos: TabItem,
    pub prvPedidos: ReportPreview,
    pub repPedidos: Report,
    pub bndPedTitulo: ReportBand,
    pub lblPedTitulo: Label,
    pub lblPedSubtitulo: Label,
    pub bndPedCabecalho: ReportBand,
    pub lblPedEmitido: Label,
    pub lblPedPagina: Label,
    pub bndPedMestre: ReportBand,
    pub lblPedNumero: Label,
    pub lblPedEmissao: Label,
    pub lblPedSituacao: Label,
    pub lblPedTotal: Label,
    pub lblPedCliente: Label,
    pub lblPedPeso: Label,
    pub bndPedItensCab: ReportBand,
    pub lblPedHItem: Label,
    pub lblPedHProduto: Label,
    pub lblPedHUn: Label,
    pub lblPedHQtd: Label,
    pub lblPedHPreco: Label,
    pub lblPedHDesc: Label,
    pub lblPedHTotal: Label,
    pub bndPedItens: ReportBand,
    pub lblPedItem: Label,
    pub lblPedProduto: Label,
    pub lblPedUn: Label,
    pub lblPedQtd: Label,
    pub lblPedPreco: Label,
    pub lblPedDesc: Label,
    pub lblPedItemTotal: Label,
    pub bndPedItensRod: ReportBand,
    pub linPedItens: Line,
    pub lblPedResumo: Label,
    pub lblPedSomaQtd: Label,
    pub lblPedSomaTotal: Label,
    pub bndPedSumario: ReportBand,
    pub linPedSumario: Line,
    pub lblPedGeral: Label,
    pub lblPedGeralTotal: Label,
    pub bndPedRodape: ReportBand,
    pub lblPedRodape: Label,
    pub tabVendedores: TabItem,
    pub prvVendedores: ReportPreview,
    pub repVendedores: Report,
    pub bndVenTitulo: ReportBand,
    pub lblVenTitulo: Label,
    pub bndVenCabecalho: ReportBand,
    pub lblVenEmitido: Label,
    pub lblVenPagina: Label,
    pub bndVenGrupo: ReportBand,
    pub lblVenVendedor: Label,
    pub bndVenPedido: ReportBand,
    pub lblVenNumero: Label,
    pub lblVenCliente: Label,
    pub lblVenSituacao: Label,
    pub lblVenTotal: Label,
    pub bndVenItens: ReportBand,
    pub lblVenProduto: Label,
    pub lblVenConta: Label,
    pub lblVenItemTotal: Label,
    pub bndVenParcelasCab: ReportBand,
    pub lblVenParcelasCab: Label,
    pub bndVenParcelas: ReportBand,
    pub lblVenParcela: Label,
    pub lblVenParcelaValor: Label,
    pub bndVenParcelasRod: ReportBand,
    pub bndVenGrupoRod: ReportBand,
    pub linVenGrupo: Line,
    pub lblVenGrupoTotal: Label,
    pub lblVenGrupoSoma: Label,
    pub bndVenSumario: ReportBand,
    pub lblVenGeral: Label,
    pub lblVenGeralSoma: Label,
    pub bndVenRodape: ReportBand,
    pub lblVenRodape: Label,
    pub data: super::principal::Data,
}

/// Os componentes do frmPrincipal sem o estado, para o código fora dos handlers
/// (`FrmPrincipal::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrincipalComponents {
    pub form: Form,
    pub cdsPedidos: ClientDataSet,
    pub cdsPedidosNumero: Field,
    pub cdsPedidosEmissao: Field,
    pub cdsPedidosCliente: Field,
    pub cdsPedidosCidade: Field,
    pub cdsPedidosUf: Field,
    pub cdsPedidosVendedor: Field,
    pub cdsPedidosSituacao: Field,
    pub cdsPedidosItens: Field,
    pub cdsPedidosPeso: Field,
    pub cdsPedidosTotal: Field,
    pub dsPedidos: DataSource,
    pub cdsItens: ClientDataSet,
    pub cdsItensPedido: Field,
    pub cdsItensItem: Field,
    pub cdsItensCodigo: Field,
    pub cdsItensProduto: Field,
    pub cdsItensUnidade: Field,
    pub cdsItensQuantidade: Field,
    pub cdsItensPreco: Field,
    pub cdsItensDesconto: Field,
    pub cdsItensTotal: Field,
    pub dsItens: DataSource,
    pub cdsParcelas: ClientDataSet,
    pub cdsParcelasPedido: Field,
    pub cdsParcelasParcela: Field,
    pub cdsParcelasVencimento: Field,
    pub cdsParcelasForma: Field,
    pub cdsParcelasValor: Field,
    pub dsParcelas: DataSource,
    pub tabPrincipal: TabControl,
    pub tabDados: TabItem,
    pub lblDica: Label,
    pub gbDetalhes: GroupBox,
    pub grdParcelas: DataGrid,
    pub splDetalhes: Splitter,
    pub grdItens: DataGrid,
    pub splDados: Splitter,
    pub grdPedidos: DataGrid,
    pub tabPedidos: TabItem,
    pub prvPedidos: ReportPreview,
    pub repPedidos: Report,
    pub bndPedTitulo: ReportBand,
    pub lblPedTitulo: Label,
    pub lblPedSubtitulo: Label,
    pub bndPedCabecalho: ReportBand,
    pub lblPedEmitido: Label,
    pub lblPedPagina: Label,
    pub bndPedMestre: ReportBand,
    pub lblPedNumero: Label,
    pub lblPedEmissao: Label,
    pub lblPedSituacao: Label,
    pub lblPedTotal: Label,
    pub lblPedCliente: Label,
    pub lblPedPeso: Label,
    pub bndPedItensCab: ReportBand,
    pub lblPedHItem: Label,
    pub lblPedHProduto: Label,
    pub lblPedHUn: Label,
    pub lblPedHQtd: Label,
    pub lblPedHPreco: Label,
    pub lblPedHDesc: Label,
    pub lblPedHTotal: Label,
    pub bndPedItens: ReportBand,
    pub lblPedItem: Label,
    pub lblPedProduto: Label,
    pub lblPedUn: Label,
    pub lblPedQtd: Label,
    pub lblPedPreco: Label,
    pub lblPedDesc: Label,
    pub lblPedItemTotal: Label,
    pub bndPedItensRod: ReportBand,
    pub linPedItens: Line,
    pub lblPedResumo: Label,
    pub lblPedSomaQtd: Label,
    pub lblPedSomaTotal: Label,
    pub bndPedSumario: ReportBand,
    pub linPedSumario: Line,
    pub lblPedGeral: Label,
    pub lblPedGeralTotal: Label,
    pub bndPedRodape: ReportBand,
    pub lblPedRodape: Label,
    pub tabVendedores: TabItem,
    pub prvVendedores: ReportPreview,
    pub repVendedores: Report,
    pub bndVenTitulo: ReportBand,
    pub lblVenTitulo: Label,
    pub bndVenCabecalho: ReportBand,
    pub lblVenEmitido: Label,
    pub lblVenPagina: Label,
    pub bndVenGrupo: ReportBand,
    pub lblVenVendedor: Label,
    pub bndVenPedido: ReportBand,
    pub lblVenNumero: Label,
    pub lblVenCliente: Label,
    pub lblVenSituacao: Label,
    pub lblVenTotal: Label,
    pub bndVenItens: ReportBand,
    pub lblVenProduto: Label,
    pub lblVenConta: Label,
    pub lblVenItemTotal: Label,
    pub bndVenParcelasCab: ReportBand,
    pub lblVenParcelasCab: Label,
    pub bndVenParcelas: ReportBand,
    pub lblVenParcela: Label,
    pub lblVenParcelaValor: Label,
    pub bndVenParcelasRod: ReportBand,
    pub bndVenGrupoRod: ReportBand,
    pub linVenGrupo: Line,
    pub lblVenGrupoTotal: Label,
    pub lblVenGrupoSoma: Label,
    pub bndVenSumario: ReportBand,
    pub lblVenGeral: Label,
    pub lblVenGeralSoma: Label,
    pub bndVenRodape: ReportBand,
    pub lblVenRodape: Label,
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
            cdsPedidos: ClientDataSet { id: handles::resolve(ui, root, "cdsPedidos", "ClientDataSet", &mut problems) },
            cdsPedidosNumero: Field { id: handles::resolve(ui, root, "cdsPedidosNumero", "Field", &mut problems) },
            cdsPedidosEmissao: Field { id: handles::resolve(ui, root, "cdsPedidosEmissao", "Field", &mut problems) },
            cdsPedidosCliente: Field { id: handles::resolve(ui, root, "cdsPedidosCliente", "Field", &mut problems) },
            cdsPedidosCidade: Field { id: handles::resolve(ui, root, "cdsPedidosCidade", "Field", &mut problems) },
            cdsPedidosUf: Field { id: handles::resolve(ui, root, "cdsPedidosUf", "Field", &mut problems) },
            cdsPedidosVendedor: Field { id: handles::resolve(ui, root, "cdsPedidosVendedor", "Field", &mut problems) },
            cdsPedidosSituacao: Field { id: handles::resolve(ui, root, "cdsPedidosSituacao", "Field", &mut problems) },
            cdsPedidosItens: Field { id: handles::resolve(ui, root, "cdsPedidosItens", "Field", &mut problems) },
            cdsPedidosPeso: Field { id: handles::resolve(ui, root, "cdsPedidosPeso", "Field", &mut problems) },
            cdsPedidosTotal: Field { id: handles::resolve(ui, root, "cdsPedidosTotal", "Field", &mut problems) },
            dsPedidos: DataSource { id: handles::resolve(ui, root, "dsPedidos", "DataSource", &mut problems) },
            cdsItens: ClientDataSet { id: handles::resolve(ui, root, "cdsItens", "ClientDataSet", &mut problems) },
            cdsItensPedido: Field { id: handles::resolve(ui, root, "cdsItensPedido", "Field", &mut problems) },
            cdsItensItem: Field { id: handles::resolve(ui, root, "cdsItensItem", "Field", &mut problems) },
            cdsItensCodigo: Field { id: handles::resolve(ui, root, "cdsItensCodigo", "Field", &mut problems) },
            cdsItensProduto: Field { id: handles::resolve(ui, root, "cdsItensProduto", "Field", &mut problems) },
            cdsItensUnidade: Field { id: handles::resolve(ui, root, "cdsItensUnidade", "Field", &mut problems) },
            cdsItensQuantidade: Field { id: handles::resolve(ui, root, "cdsItensQuantidade", "Field", &mut problems) },
            cdsItensPreco: Field { id: handles::resolve(ui, root, "cdsItensPreco", "Field", &mut problems) },
            cdsItensDesconto: Field { id: handles::resolve(ui, root, "cdsItensDesconto", "Field", &mut problems) },
            cdsItensTotal: Field { id: handles::resolve(ui, root, "cdsItensTotal", "Field", &mut problems) },
            dsItens: DataSource { id: handles::resolve(ui, root, "dsItens", "DataSource", &mut problems) },
            cdsParcelas: ClientDataSet { id: handles::resolve(ui, root, "cdsParcelas", "ClientDataSet", &mut problems) },
            cdsParcelasPedido: Field { id: handles::resolve(ui, root, "cdsParcelasPedido", "Field", &mut problems) },
            cdsParcelasParcela: Field { id: handles::resolve(ui, root, "cdsParcelasParcela", "Field", &mut problems) },
            cdsParcelasVencimento: Field { id: handles::resolve(ui, root, "cdsParcelasVencimento", "Field", &mut problems) },
            cdsParcelasForma: Field { id: handles::resolve(ui, root, "cdsParcelasForma", "Field", &mut problems) },
            cdsParcelasValor: Field { id: handles::resolve(ui, root, "cdsParcelasValor", "Field", &mut problems) },
            dsParcelas: DataSource { id: handles::resolve(ui, root, "dsParcelas", "DataSource", &mut problems) },
            tabPrincipal: TabControl { id: handles::resolve(ui, root, "tabPrincipal", "TabControl", &mut problems) },
            tabDados: TabItem { id: handles::resolve(ui, root, "tabDados", "TabItem", &mut problems) },
            lblDica: Label { id: handles::resolve(ui, root, "lblDica", "Label", &mut problems) },
            gbDetalhes: GroupBox { id: handles::resolve(ui, root, "gbDetalhes", "GroupBox", &mut problems) },
            grdParcelas: DataGrid { id: handles::resolve(ui, root, "grdParcelas", "DataGrid", &mut problems) },
            splDetalhes: Splitter { id: handles::resolve(ui, root, "splDetalhes", "Splitter", &mut problems) },
            grdItens: DataGrid { id: handles::resolve(ui, root, "grdItens", "DataGrid", &mut problems) },
            splDados: Splitter { id: handles::resolve(ui, root, "splDados", "Splitter", &mut problems) },
            grdPedidos: DataGrid { id: handles::resolve(ui, root, "grdPedidos", "DataGrid", &mut problems) },
            tabPedidos: TabItem { id: handles::resolve(ui, root, "tabPedidos", "TabItem", &mut problems) },
            prvPedidos: ReportPreview { id: handles::resolve(ui, root, "prvPedidos", "ReportPreview", &mut problems) },
            repPedidos: Report { id: handles::resolve(ui, root, "repPedidos", "Report", &mut problems) },
            bndPedTitulo: ReportBand { id: handles::resolve(ui, root, "bndPedTitulo", "ReportBand", &mut problems) },
            lblPedTitulo: Label { id: handles::resolve(ui, root, "lblPedTitulo", "Label", &mut problems) },
            lblPedSubtitulo: Label { id: handles::resolve(ui, root, "lblPedSubtitulo", "Label", &mut problems) },
            bndPedCabecalho: ReportBand { id: handles::resolve(ui, root, "bndPedCabecalho", "ReportBand", &mut problems) },
            lblPedEmitido: Label { id: handles::resolve(ui, root, "lblPedEmitido", "Label", &mut problems) },
            lblPedPagina: Label { id: handles::resolve(ui, root, "lblPedPagina", "Label", &mut problems) },
            bndPedMestre: ReportBand { id: handles::resolve(ui, root, "bndPedMestre", "ReportBand", &mut problems) },
            lblPedNumero: Label { id: handles::resolve(ui, root, "lblPedNumero", "Label", &mut problems) },
            lblPedEmissao: Label { id: handles::resolve(ui, root, "lblPedEmissao", "Label", &mut problems) },
            lblPedSituacao: Label { id: handles::resolve(ui, root, "lblPedSituacao", "Label", &mut problems) },
            lblPedTotal: Label { id: handles::resolve(ui, root, "lblPedTotal", "Label", &mut problems) },
            lblPedCliente: Label { id: handles::resolve(ui, root, "lblPedCliente", "Label", &mut problems) },
            lblPedPeso: Label { id: handles::resolve(ui, root, "lblPedPeso", "Label", &mut problems) },
            bndPedItensCab: ReportBand { id: handles::resolve(ui, root, "bndPedItensCab", "ReportBand", &mut problems) },
            lblPedHItem: Label { id: handles::resolve(ui, root, "lblPedHItem", "Label", &mut problems) },
            lblPedHProduto: Label { id: handles::resolve(ui, root, "lblPedHProduto", "Label", &mut problems) },
            lblPedHUn: Label { id: handles::resolve(ui, root, "lblPedHUn", "Label", &mut problems) },
            lblPedHQtd: Label { id: handles::resolve(ui, root, "lblPedHQtd", "Label", &mut problems) },
            lblPedHPreco: Label { id: handles::resolve(ui, root, "lblPedHPreco", "Label", &mut problems) },
            lblPedHDesc: Label { id: handles::resolve(ui, root, "lblPedHDesc", "Label", &mut problems) },
            lblPedHTotal: Label { id: handles::resolve(ui, root, "lblPedHTotal", "Label", &mut problems) },
            bndPedItens: ReportBand { id: handles::resolve(ui, root, "bndPedItens", "ReportBand", &mut problems) },
            lblPedItem: Label { id: handles::resolve(ui, root, "lblPedItem", "Label", &mut problems) },
            lblPedProduto: Label { id: handles::resolve(ui, root, "lblPedProduto", "Label", &mut problems) },
            lblPedUn: Label { id: handles::resolve(ui, root, "lblPedUn", "Label", &mut problems) },
            lblPedQtd: Label { id: handles::resolve(ui, root, "lblPedQtd", "Label", &mut problems) },
            lblPedPreco: Label { id: handles::resolve(ui, root, "lblPedPreco", "Label", &mut problems) },
            lblPedDesc: Label { id: handles::resolve(ui, root, "lblPedDesc", "Label", &mut problems) },
            lblPedItemTotal: Label { id: handles::resolve(ui, root, "lblPedItemTotal", "Label", &mut problems) },
            bndPedItensRod: ReportBand { id: handles::resolve(ui, root, "bndPedItensRod", "ReportBand", &mut problems) },
            linPedItens: Line { id: handles::resolve(ui, root, "linPedItens", "Line", &mut problems) },
            lblPedResumo: Label { id: handles::resolve(ui, root, "lblPedResumo", "Label", &mut problems) },
            lblPedSomaQtd: Label { id: handles::resolve(ui, root, "lblPedSomaQtd", "Label", &mut problems) },
            lblPedSomaTotal: Label { id: handles::resolve(ui, root, "lblPedSomaTotal", "Label", &mut problems) },
            bndPedSumario: ReportBand { id: handles::resolve(ui, root, "bndPedSumario", "ReportBand", &mut problems) },
            linPedSumario: Line { id: handles::resolve(ui, root, "linPedSumario", "Line", &mut problems) },
            lblPedGeral: Label { id: handles::resolve(ui, root, "lblPedGeral", "Label", &mut problems) },
            lblPedGeralTotal: Label { id: handles::resolve(ui, root, "lblPedGeralTotal", "Label", &mut problems) },
            bndPedRodape: ReportBand { id: handles::resolve(ui, root, "bndPedRodape", "ReportBand", &mut problems) },
            lblPedRodape: Label { id: handles::resolve(ui, root, "lblPedRodape", "Label", &mut problems) },
            tabVendedores: TabItem { id: handles::resolve(ui, root, "tabVendedores", "TabItem", &mut problems) },
            prvVendedores: ReportPreview { id: handles::resolve(ui, root, "prvVendedores", "ReportPreview", &mut problems) },
            repVendedores: Report { id: handles::resolve(ui, root, "repVendedores", "Report", &mut problems) },
            bndVenTitulo: ReportBand { id: handles::resolve(ui, root, "bndVenTitulo", "ReportBand", &mut problems) },
            lblVenTitulo: Label { id: handles::resolve(ui, root, "lblVenTitulo", "Label", &mut problems) },
            bndVenCabecalho: ReportBand { id: handles::resolve(ui, root, "bndVenCabecalho", "ReportBand", &mut problems) },
            lblVenEmitido: Label { id: handles::resolve(ui, root, "lblVenEmitido", "Label", &mut problems) },
            lblVenPagina: Label { id: handles::resolve(ui, root, "lblVenPagina", "Label", &mut problems) },
            bndVenGrupo: ReportBand { id: handles::resolve(ui, root, "bndVenGrupo", "ReportBand", &mut problems) },
            lblVenVendedor: Label { id: handles::resolve(ui, root, "lblVenVendedor", "Label", &mut problems) },
            bndVenPedido: ReportBand { id: handles::resolve(ui, root, "bndVenPedido", "ReportBand", &mut problems) },
            lblVenNumero: Label { id: handles::resolve(ui, root, "lblVenNumero", "Label", &mut problems) },
            lblVenCliente: Label { id: handles::resolve(ui, root, "lblVenCliente", "Label", &mut problems) },
            lblVenSituacao: Label { id: handles::resolve(ui, root, "lblVenSituacao", "Label", &mut problems) },
            lblVenTotal: Label { id: handles::resolve(ui, root, "lblVenTotal", "Label", &mut problems) },
            bndVenItens: ReportBand { id: handles::resolve(ui, root, "bndVenItens", "ReportBand", &mut problems) },
            lblVenProduto: Label { id: handles::resolve(ui, root, "lblVenProduto", "Label", &mut problems) },
            lblVenConta: Label { id: handles::resolve(ui, root, "lblVenConta", "Label", &mut problems) },
            lblVenItemTotal: Label { id: handles::resolve(ui, root, "lblVenItemTotal", "Label", &mut problems) },
            bndVenParcelasCab: ReportBand { id: handles::resolve(ui, root, "bndVenParcelasCab", "ReportBand", &mut problems) },
            lblVenParcelasCab: Label { id: handles::resolve(ui, root, "lblVenParcelasCab", "Label", &mut problems) },
            bndVenParcelas: ReportBand { id: handles::resolve(ui, root, "bndVenParcelas", "ReportBand", &mut problems) },
            lblVenParcela: Label { id: handles::resolve(ui, root, "lblVenParcela", "Label", &mut problems) },
            lblVenParcelaValor: Label { id: handles::resolve(ui, root, "lblVenParcelaValor", "Label", &mut problems) },
            bndVenParcelasRod: ReportBand { id: handles::resolve(ui, root, "bndVenParcelasRod", "ReportBand", &mut problems) },
            bndVenGrupoRod: ReportBand { id: handles::resolve(ui, root, "bndVenGrupoRod", "ReportBand", &mut problems) },
            linVenGrupo: Line { id: handles::resolve(ui, root, "linVenGrupo", "Line", &mut problems) },
            lblVenGrupoTotal: Label { id: handles::resolve(ui, root, "lblVenGrupoTotal", "Label", &mut problems) },
            lblVenGrupoSoma: Label { id: handles::resolve(ui, root, "lblVenGrupoSoma", "Label", &mut problems) },
            bndVenSumario: ReportBand { id: handles::resolve(ui, root, "bndVenSumario", "ReportBand", &mut problems) },
            lblVenGeral: Label { id: handles::resolve(ui, root, "lblVenGeral", "Label", &mut problems) },
            lblVenGeralSoma: Label { id: handles::resolve(ui, root, "lblVenGeralSoma", "Label", &mut problems) },
            bndVenRodape: ReportBand { id: handles::resolve(ui, root, "bndVenRodape", "ReportBand", &mut problems) },
            lblVenRodape: Label { id: handles::resolve(ui, root, "lblVenRodape", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrincipal não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrincipalComponents {
            form: instance.form,
            cdsPedidos: instance.cdsPedidos,
            cdsPedidosNumero: instance.cdsPedidosNumero,
            cdsPedidosEmissao: instance.cdsPedidosEmissao,
            cdsPedidosCliente: instance.cdsPedidosCliente,
            cdsPedidosCidade: instance.cdsPedidosCidade,
            cdsPedidosUf: instance.cdsPedidosUf,
            cdsPedidosVendedor: instance.cdsPedidosVendedor,
            cdsPedidosSituacao: instance.cdsPedidosSituacao,
            cdsPedidosItens: instance.cdsPedidosItens,
            cdsPedidosPeso: instance.cdsPedidosPeso,
            cdsPedidosTotal: instance.cdsPedidosTotal,
            dsPedidos: instance.dsPedidos,
            cdsItens: instance.cdsItens,
            cdsItensPedido: instance.cdsItensPedido,
            cdsItensItem: instance.cdsItensItem,
            cdsItensCodigo: instance.cdsItensCodigo,
            cdsItensProduto: instance.cdsItensProduto,
            cdsItensUnidade: instance.cdsItensUnidade,
            cdsItensQuantidade: instance.cdsItensQuantidade,
            cdsItensPreco: instance.cdsItensPreco,
            cdsItensDesconto: instance.cdsItensDesconto,
            cdsItensTotal: instance.cdsItensTotal,
            dsItens: instance.dsItens,
            cdsParcelas: instance.cdsParcelas,
            cdsParcelasPedido: instance.cdsParcelasPedido,
            cdsParcelasParcela: instance.cdsParcelasParcela,
            cdsParcelasVencimento: instance.cdsParcelasVencimento,
            cdsParcelasForma: instance.cdsParcelasForma,
            cdsParcelasValor: instance.cdsParcelasValor,
            dsParcelas: instance.dsParcelas,
            tabPrincipal: instance.tabPrincipal,
            tabDados: instance.tabDados,
            lblDica: instance.lblDica,
            gbDetalhes: instance.gbDetalhes,
            grdParcelas: instance.grdParcelas,
            splDetalhes: instance.splDetalhes,
            grdItens: instance.grdItens,
            splDados: instance.splDados,
            grdPedidos: instance.grdPedidos,
            tabPedidos: instance.tabPedidos,
            prvPedidos: instance.prvPedidos,
            repPedidos: instance.repPedidos,
            bndPedTitulo: instance.bndPedTitulo,
            lblPedTitulo: instance.lblPedTitulo,
            lblPedSubtitulo: instance.lblPedSubtitulo,
            bndPedCabecalho: instance.bndPedCabecalho,
            lblPedEmitido: instance.lblPedEmitido,
            lblPedPagina: instance.lblPedPagina,
            bndPedMestre: instance.bndPedMestre,
            lblPedNumero: instance.lblPedNumero,
            lblPedEmissao: instance.lblPedEmissao,
            lblPedSituacao: instance.lblPedSituacao,
            lblPedTotal: instance.lblPedTotal,
            lblPedCliente: instance.lblPedCliente,
            lblPedPeso: instance.lblPedPeso,
            bndPedItensCab: instance.bndPedItensCab,
            lblPedHItem: instance.lblPedHItem,
            lblPedHProduto: instance.lblPedHProduto,
            lblPedHUn: instance.lblPedHUn,
            lblPedHQtd: instance.lblPedHQtd,
            lblPedHPreco: instance.lblPedHPreco,
            lblPedHDesc: instance.lblPedHDesc,
            lblPedHTotal: instance.lblPedHTotal,
            bndPedItens: instance.bndPedItens,
            lblPedItem: instance.lblPedItem,
            lblPedProduto: instance.lblPedProduto,
            lblPedUn: instance.lblPedUn,
            lblPedQtd: instance.lblPedQtd,
            lblPedPreco: instance.lblPedPreco,
            lblPedDesc: instance.lblPedDesc,
            lblPedItemTotal: instance.lblPedItemTotal,
            bndPedItensRod: instance.bndPedItensRod,
            linPedItens: instance.linPedItens,
            lblPedResumo: instance.lblPedResumo,
            lblPedSomaQtd: instance.lblPedSomaQtd,
            lblPedSomaTotal: instance.lblPedSomaTotal,
            bndPedSumario: instance.bndPedSumario,
            linPedSumario: instance.linPedSumario,
            lblPedGeral: instance.lblPedGeral,
            lblPedGeralTotal: instance.lblPedGeralTotal,
            bndPedRodape: instance.bndPedRodape,
            lblPedRodape: instance.lblPedRodape,
            tabVendedores: instance.tabVendedores,
            prvVendedores: instance.prvVendedores,
            repVendedores: instance.repVendedores,
            bndVenTitulo: instance.bndVenTitulo,
            lblVenTitulo: instance.lblVenTitulo,
            bndVenCabecalho: instance.bndVenCabecalho,
            lblVenEmitido: instance.lblVenEmitido,
            lblVenPagina: instance.lblVenPagina,
            bndVenGrupo: instance.bndVenGrupo,
            lblVenVendedor: instance.lblVenVendedor,
            bndVenPedido: instance.bndVenPedido,
            lblVenNumero: instance.lblVenNumero,
            lblVenCliente: instance.lblVenCliente,
            lblVenSituacao: instance.lblVenSituacao,
            lblVenTotal: instance.lblVenTotal,
            bndVenItens: instance.bndVenItens,
            lblVenProduto: instance.lblVenProduto,
            lblVenConta: instance.lblVenConta,
            lblVenItemTotal: instance.lblVenItemTotal,
            bndVenParcelasCab: instance.bndVenParcelasCab,
            lblVenParcelasCab: instance.lblVenParcelasCab,
            bndVenParcelas: instance.bndVenParcelas,
            lblVenParcela: instance.lblVenParcela,
            lblVenParcelaValor: instance.lblVenParcelaValor,
            bndVenParcelasRod: instance.bndVenParcelasRod,
            bndVenGrupoRod: instance.bndVenGrupoRod,
            linVenGrupo: instance.linVenGrupo,
            lblVenGrupoTotal: instance.lblVenGrupoTotal,
            lblVenGrupoSoma: instance.lblVenGrupoSoma,
            bndVenSumario: instance.bndVenSumario,
            lblVenGeral: instance.lblVenGeral,
            lblVenGeralSoma: instance.lblVenGeralSoma,
            bndVenRodape: instance.bndVenRodape,
            lblVenRodape: instance.lblVenRodape,
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

    /// O ClientDataSet deste form com o nome (`None` se não há um ClientDataSet com ele).
    pub fn find_client_data_set(&self, name: &str) -> Option<ClientDataSet> {
        match name {
            "cdsPedidos" => Some(self.cdsPedidos),
            "cdsItens" => Some(self.cdsItens),
            "cdsParcelas" => Some(self.cdsParcelas),
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
            "grdParcelas" => Some(self.grdParcelas),
            "grdItens" => Some(self.grdItens),
            "grdPedidos" => Some(self.grdPedidos),
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
            "dsPedidos" => Some(self.dsPedidos),
            "dsItens" => Some(self.dsItens),
            "dsParcelas" => Some(self.dsParcelas),
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
            "cdsPedidosNumero" => Some(self.cdsPedidosNumero),
            "cdsPedidosEmissao" => Some(self.cdsPedidosEmissao),
            "cdsPedidosCliente" => Some(self.cdsPedidosCliente),
            "cdsPedidosCidade" => Some(self.cdsPedidosCidade),
            "cdsPedidosUf" => Some(self.cdsPedidosUf),
            "cdsPedidosVendedor" => Some(self.cdsPedidosVendedor),
            "cdsPedidosSituacao" => Some(self.cdsPedidosSituacao),
            "cdsPedidosItens" => Some(self.cdsPedidosItens),
            "cdsPedidosPeso" => Some(self.cdsPedidosPeso),
            "cdsPedidosTotal" => Some(self.cdsPedidosTotal),
            "cdsItensPedido" => Some(self.cdsItensPedido),
            "cdsItensItem" => Some(self.cdsItensItem),
            "cdsItensCodigo" => Some(self.cdsItensCodigo),
            "cdsItensProduto" => Some(self.cdsItensProduto),
            "cdsItensUnidade" => Some(self.cdsItensUnidade),
            "cdsItensQuantidade" => Some(self.cdsItensQuantidade),
            "cdsItensPreco" => Some(self.cdsItensPreco),
            "cdsItensDesconto" => Some(self.cdsItensDesconto),
            "cdsItensTotal" => Some(self.cdsItensTotal),
            "cdsParcelasPedido" => Some(self.cdsParcelasPedido),
            "cdsParcelasParcela" => Some(self.cdsParcelasParcela),
            "cdsParcelasVencimento" => Some(self.cdsParcelasVencimento),
            "cdsParcelasForma" => Some(self.cdsParcelasForma),
            "cdsParcelasValor" => Some(self.cdsParcelasValor),
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
            "gbDetalhes" => Some(self.gbDetalhes),
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
            "lblDica" => Some(self.lblDica),
            "lblPedTitulo" => Some(self.lblPedTitulo),
            "lblPedSubtitulo" => Some(self.lblPedSubtitulo),
            "lblPedEmitido" => Some(self.lblPedEmitido),
            "lblPedPagina" => Some(self.lblPedPagina),
            "lblPedNumero" => Some(self.lblPedNumero),
            "lblPedEmissao" => Some(self.lblPedEmissao),
            "lblPedSituacao" => Some(self.lblPedSituacao),
            "lblPedTotal" => Some(self.lblPedTotal),
            "lblPedCliente" => Some(self.lblPedCliente),
            "lblPedPeso" => Some(self.lblPedPeso),
            "lblPedHItem" => Some(self.lblPedHItem),
            "lblPedHProduto" => Some(self.lblPedHProduto),
            "lblPedHUn" => Some(self.lblPedHUn),
            "lblPedHQtd" => Some(self.lblPedHQtd),
            "lblPedHPreco" => Some(self.lblPedHPreco),
            "lblPedHDesc" => Some(self.lblPedHDesc),
            "lblPedHTotal" => Some(self.lblPedHTotal),
            "lblPedItem" => Some(self.lblPedItem),
            "lblPedProduto" => Some(self.lblPedProduto),
            "lblPedUn" => Some(self.lblPedUn),
            "lblPedQtd" => Some(self.lblPedQtd),
            "lblPedPreco" => Some(self.lblPedPreco),
            "lblPedDesc" => Some(self.lblPedDesc),
            "lblPedItemTotal" => Some(self.lblPedItemTotal),
            "lblPedResumo" => Some(self.lblPedResumo),
            "lblPedSomaQtd" => Some(self.lblPedSomaQtd),
            "lblPedSomaTotal" => Some(self.lblPedSomaTotal),
            "lblPedGeral" => Some(self.lblPedGeral),
            "lblPedGeralTotal" => Some(self.lblPedGeralTotal),
            "lblPedRodape" => Some(self.lblPedRodape),
            "lblVenTitulo" => Some(self.lblVenTitulo),
            "lblVenEmitido" => Some(self.lblVenEmitido),
            "lblVenPagina" => Some(self.lblVenPagina),
            "lblVenVendedor" => Some(self.lblVenVendedor),
            "lblVenNumero" => Some(self.lblVenNumero),
            "lblVenCliente" => Some(self.lblVenCliente),
            "lblVenSituacao" => Some(self.lblVenSituacao),
            "lblVenTotal" => Some(self.lblVenTotal),
            "lblVenProduto" => Some(self.lblVenProduto),
            "lblVenConta" => Some(self.lblVenConta),
            "lblVenItemTotal" => Some(self.lblVenItemTotal),
            "lblVenParcelasCab" => Some(self.lblVenParcelasCab),
            "lblVenParcela" => Some(self.lblVenParcela),
            "lblVenParcelaValor" => Some(self.lblVenParcelaValor),
            "lblVenGrupoTotal" => Some(self.lblVenGrupoTotal),
            "lblVenGrupoSoma" => Some(self.lblVenGrupoSoma),
            "lblVenGeral" => Some(self.lblVenGeral),
            "lblVenGeralSoma" => Some(self.lblVenGeralSoma),
            "lblVenRodape" => Some(self.lblVenRodape),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Line deste form com o nome (`None` se não há um Line com ele).
    pub fn find_line(&self, name: &str) -> Option<Line> {
        match name {
            "linPedItens" => Some(self.linPedItens),
            "linPedSumario" => Some(self.linPedSumario),
            "linVenGrupo" => Some(self.linVenGrupo),
            _ => None,
        }
    }

    /// O Line que disparou o evento (`None` se o `sender` não é um Line deste form).
    pub fn sender_as_line(&self, ui: &Ui, sender: ComponentId) -> Option<Line> {
        self.find_line(ui.name(sender)?)
    }

    /// O Report deste form com o nome (`None` se não há um Report com ele).
    pub fn find_report(&self, name: &str) -> Option<Report> {
        match name {
            "repPedidos" => Some(self.repPedidos),
            "repVendedores" => Some(self.repVendedores),
            _ => None,
        }
    }

    /// O Report que disparou o evento (`None` se o `sender` não é um Report deste form).
    pub fn sender_as_report(&self, ui: &Ui, sender: ComponentId) -> Option<Report> {
        self.find_report(ui.name(sender)?)
    }

    /// O ReportBand deste form com o nome (`None` se não há um ReportBand com ele).
    pub fn find_report_band(&self, name: &str) -> Option<ReportBand> {
        match name {
            "bndPedTitulo" => Some(self.bndPedTitulo),
            "bndPedCabecalho" => Some(self.bndPedCabecalho),
            "bndPedMestre" => Some(self.bndPedMestre),
            "bndPedItensCab" => Some(self.bndPedItensCab),
            "bndPedItens" => Some(self.bndPedItens),
            "bndPedItensRod" => Some(self.bndPedItensRod),
            "bndPedSumario" => Some(self.bndPedSumario),
            "bndPedRodape" => Some(self.bndPedRodape),
            "bndVenTitulo" => Some(self.bndVenTitulo),
            "bndVenCabecalho" => Some(self.bndVenCabecalho),
            "bndVenGrupo" => Some(self.bndVenGrupo),
            "bndVenPedido" => Some(self.bndVenPedido),
            "bndVenItens" => Some(self.bndVenItens),
            "bndVenParcelasCab" => Some(self.bndVenParcelasCab),
            "bndVenParcelas" => Some(self.bndVenParcelas),
            "bndVenParcelasRod" => Some(self.bndVenParcelasRod),
            "bndVenGrupoRod" => Some(self.bndVenGrupoRod),
            "bndVenSumario" => Some(self.bndVenSumario),
            "bndVenRodape" => Some(self.bndVenRodape),
            _ => None,
        }
    }

    /// O ReportBand que disparou o evento (`None` se o `sender` não é um ReportBand deste form).
    pub fn sender_as_report_band(&self, ui: &Ui, sender: ComponentId) -> Option<ReportBand> {
        self.find_report_band(ui.name(sender)?)
    }

    /// O ReportPreview deste form com o nome (`None` se não há um ReportPreview com ele).
    pub fn find_report_preview(&self, name: &str) -> Option<ReportPreview> {
        match name {
            "prvPedidos" => Some(self.prvPedidos),
            "prvVendedores" => Some(self.prvVendedores),
            _ => None,
        }
    }

    /// O ReportPreview que disparou o evento (`None` se o `sender` não é um ReportPreview deste form).
    pub fn sender_as_report_preview(&self, ui: &Ui, sender: ComponentId) -> Option<ReportPreview> {
        self.find_report_preview(ui.name(sender)?)
    }

    /// O Splitter deste form com o nome (`None` se não há um Splitter com ele).
    pub fn find_splitter(&self, name: &str) -> Option<Splitter> {
        match name {
            "splDetalhes" => Some(self.splDetalhes),
            "splDados" => Some(self.splDados),
            _ => None,
        }
    }

    /// O Splitter que disparou o evento (`None` se o `sender` não é um Splitter deste form).
    pub fn sender_as_splitter(&self, ui: &Ui, sender: ComponentId) -> Option<Splitter> {
        self.find_splitter(ui.name(sender)?)
    }

    /// O TabControl deste form com o nome (`None` se não há um TabControl com ele).
    pub fn find_tab_control(&self, name: &str) -> Option<TabControl> {
        match name {
            "tabPrincipal" => Some(self.tabPrincipal),
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
            "tabDados" => Some(self.tabDados),
            "tabPedidos" => Some(self.tabPedidos),
            "tabVendedores" => Some(self.tabVendedores),
            _ => None,
        }
    }

    /// O TabItem que disparou o evento (`None` se o `sender` não é um TabItem deste form).
    pub fn sender_as_tab_item(&self, ui: &Ui, sender: ComponentId) -> Option<TabItem> {
        self.find_tab_item(ui.name(sender)?)
    }
}
