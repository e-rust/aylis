//! Exportar os dados de um dataset para um arquivo (item 23): o componente `Export` diz o que
//! exportar (o `data_source`, os campos); o provedor ligado em `provider` diz o formato
//! (`CsvProvider`, `XlsxProvider` ou outro). Sem provedor, CSV. Um formato só entra no app que
//! tem o componente do provedor dele (a planilha não pesa em quem só exporta CSV).

use aylis_core::providers::{CellValue, ColumnKind, ExportColumn, ExportProvider, ExportTable, Provided};
use aylis_core::{Component, ComponentId, EventCtx, MethodInfo, PropKind, PropValue, Ui, properties};
use aylis_data::{DataSource, FieldType, Value, view};

/// Exporta os registros de um `DataSource` (na ordem e com o filtro que a tela mostra).
pub struct Export {
    pub data_source: Option<ComponentId>,
    /// Quem grava o arquivo (um provedor de exportação); sem ele, CSV.
    pub provider: Option<ComponentId>,
    /// Os campos, separados por `;` (vazio: os visíveis, na ordem do dataset).
    pub fields: String,
    /// O nome da aba (onde o formato tem abas).
    pub sheet_name: String,
}

impl Default for Export {
    fn default() -> Self {
        Self { data_source: None, provider: None, fields: String::new(), sheet_name: "Dados".into() }
    }
}

impl Component for Export {
    fn class_name(&self) -> &'static str {
        "Export"
    }

    properties! {
        data_source: Option<ComponentId> => "Data",
        provider: Option<ComponentId> => "Data",
        fields: String => "Data",
        sheet_name: String => "Data",
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.data_source == Some(freed) {
            self.data_source = None;
        }
        if self.provider == Some(freed) {
            self.provider = None;
        }
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("save_to", &[PropKind::String], Some(PropKind::Bool)).params(&["path"]), MethodInfo::new("extension", &[], Some(PropKind::String))];
        METHODS
    }

    /// `save_to(path)`: grava o arquivo (`true`; a falha volta como erro, com o motivo, que o
    /// acesso tipado devolve no `Err`); `extension()`: a extensão do formato do provedor.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let id = ctx.id;
        match method {
            "save_to" => {
                let path = args.first().and_then(|v| v.to_str().ok()).ok_or("save_to pede o caminho do arquivo")?.to_string();
                let bytes = export_with(ctx.ui, self, id)?;
                std::fs::write(&path, bytes).map_err(|e| format!("não consegui gravar {path}: {e}"))?;
                Ok(PropValue::Bool(true))
            }
            "extension" => Ok(PropValue::String(with_provider(ctx.ui, self.provider, |p| p.extension().to_string()))),
            _ => Err(format!("Export não tem o método '{method}'")),
        }
    }
}

/// Usa o provedor ligado (ou o CSV padrão).
fn with_provider<R>(ui: &Ui, provider: Option<ComponentId>, f: impl FnOnce(&dyn ExportProvider) -> R) -> R {
    match provider.and_then(|p| ui.provided(p)) {
        Some(Provided::Export(p)) => f(p),
        _ => f(&CsvProvider::default()),
    }
}

impl Export {
    /// O arquivo exportado (no formato do provedor).
    pub fn bytes(ui: &Ui, id: ComponentId) -> Result<Vec<u8>, String> {
        let e = ui.get::<Export>(id).ok_or("não é um Export")?;
        export_with(ui, e, id)
    }

    /// A tabela que vai para o arquivo: os títulos, os tipos e os valores.
    pub fn table(ui: &Ui, id: ComponentId) -> Result<ExportTable, String> {
        let e = ui.get::<Export>(id).ok_or("não é um Export")?;
        table_of(ui, e)
    }
}

fn export_with(ui: &Ui, e: &Export, _id: ComponentId) -> Result<Vec<u8>, String> {
    let table = table_of(ui, e)?;
    with_provider(ui, e.provider, |p| p.write(&[table]))
}

fn table_of(ui: &Ui, e: &Export) -> Result<ExportTable, String> {
    let ds = e.data_source.and_then(|s| ui.get::<DataSource>(s)).and_then(|s| s.data_set).ok_or("o Export precisa de um data_source ligado a um dataset")?;
    let v = view(ui, ds);
    let names: Vec<String> = if e.fields.trim().is_empty() {
        v.field_names().into_iter().filter(|n| v.field(n).is_ok_and(|f| f.visible)).collect()
    } else {
        e.fields.split(';').map(|f| f.trim().to_string()).filter(|f| !f.is_empty()).collect()
    };
    let mut columns = Vec::new();
    for name in &names {
        let f = v.field(name).map_err(|e| e.to_string())?;
        let kind = match f.field_type {
            FieldType::Integer | FieldType::AutoInc => ColumnKind::Integer,
            FieldType::Float | FieldType::Currency => ColumnKind::Number,
            FieldType::Boolean => ColumnKind::Bool,
            FieldType::Date => ColumnKind::Date,
            FieldType::DateTime | FieldType::Timestamp => ColumnKind::DateTime,
            _ => ColumnKind::Text,
        };
        columns.push(ExportColumn { title: f.caption().to_string(), kind, format: f.display_format.clone() });
    }
    let mut rows = Vec::with_capacity(v.visible_count());
    for i in 0..v.visible_count() {
        let mut row = Vec::with_capacity(names.len());
        for name in &names {
            row.push(cell(v.visible_value(i, name).unwrap_or_default()));
        }
        rows.push(row);
    }
    Ok(ExportTable { name: e.sheet_name.clone(), columns, rows })
}

/// O valor do dataset como célula.
fn cell(value: Value) -> CellValue {
    match value {
        Value::Null => CellValue::Empty,
        Value::Bool(b) => CellValue::Bool(b),
        Value::Int(i) => CellValue::Number(i as f64),
        Value::Float(f) => CellValue::Number(f),
        Value::Decimal(d) => CellValue::Number(d.to_f64()),
        Value::Date(d) => CellValue::Date(d.year(), u32::from(d.month()), u32::from(d.day())),
        Value::DateTime(dt) => CellValue::DateTime(dt.date.year(), u32::from(dt.date.month()), u32::from(dt.date.day()), u32::from(dt.time.hour()), u32::from(dt.time.minute()), u32::from(dt.time.second())),
        Value::Blob(_) => CellValue::Empty,
        other => CellValue::Text(other.to_string()),
    }
}

// ---------------------------------------------------------------------------
// CSV

/// CSV (o padrão do `Export`): `separator` entre as colunas (`;`, o que o Excel em português
/// abre direto), a vírgula decimal (`decimal_comma`) e o BOM do UTF-8 (para o Excel mostrar
/// os acentos).
pub struct CsvProvider {
    pub separator: String,
    pub decimal_comma: bool,
    pub bom: bool,
    /// A primeira linha com os títulos.
    pub header: bool,
}

impl Default for CsvProvider {
    fn default() -> Self {
        Self { separator: ";".into(), decimal_comma: true, bom: true, header: true }
    }
}

impl Component for CsvProvider {
    fn class_name(&self) -> &'static str {
        "CsvProvider"
    }

    properties! {
        separator: String => "Data",
        decimal_comma: bool => "Data",
        bom: bool => "Data",
        header: bool => "Data",
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Export(self))
    }
}

impl ExportProvider for CsvProvider {
    fn extension(&self) -> &'static str {
        "csv"
    }

    fn mime(&self) -> &'static str {
        "text/csv"
    }

    fn write(&self, tables: &[ExportTable]) -> Result<Vec<u8>, String> {
        let sep = if self.separator.is_empty() { ";" } else { self.separator.as_str() };
        let quote = |s: &str| {
            if s.contains(sep) || s.contains('"') || s.contains('\n') || s.contains('\r') {
                format!("\"{}\"", s.replace('"', "\"\""))
            } else {
                s.to_string()
            }
        };
        let mut out = String::new();
        if self.bom {
            out.push('\u{feff}');
        }
        for table in tables {
            if self.header {
                out.push_str(&table.columns.iter().map(|c| quote(&c.title)).collect::<Vec<_>>().join(sep));
                out.push_str("\r\n");
            }
            for row in &table.rows {
                let cells: Vec<String> = row
                    .iter()
                    .map(|c| match c {
                        CellValue::Empty => String::new(),
                        CellValue::Text(t) => quote(t),
                        CellValue::Number(n) => {
                            let s = if n.fract() == 0.0 && n.abs() < 1e15 { format!("{}", *n as i64) } else { format!("{n}") };
                            if self.decimal_comma { s.replace('.', ",") } else { s }
                        }
                        CellValue::Bool(b) => if *b { "1" } else { "0" }.into(),
                        CellValue::Date(y, m, d) => format!("{y:04}-{m:02}-{d:02}"),
                        CellValue::DateTime(y, m, d, h, mi, s) => format!("{y:04}-{m:02}-{d:02} {h:02}:{mi:02}:{s:02}"),
                    })
                    .collect();
                out.push_str(&cells.join(sep));
                out.push_str("\r\n");
            }
        }
        Ok(out.into_bytes())
    }
}

// ---------------------------------------------------------------------------
// XLSX

/// Planilha do Excel (`.xlsx`), pela biblioteca `rust_xlsxwriter` (trocar de biblioteca é
/// escrever outro provedor): os títulos em negrito e congelados, números, datas e os formatos
/// dos campos (`#,##0.00`), e a largura das colunas pelo conteúdo.
#[derive(Default)]
pub struct XlsxProvider;

impl Component for XlsxProvider {
    fn class_name(&self) -> &'static str {
        "XlsxProvider"
    }


    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Export(self))
    }
}

impl ExportProvider for XlsxProvider {
    fn extension(&self) -> &'static str {
        "xlsx"
    }

    fn mime(&self) -> &'static str {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    }

    fn write(&self, tables: &[ExportTable]) -> Result<Vec<u8>, String> {
        use rust_xlsxwriter::{ExcelDateTime, Format, Workbook};
        let err = |e: rust_xlsxwriter::XlsxError| e.to_string();
        let mut book = Workbook::new();
        let bold = Format::new().set_bold();
        let date = Format::new().set_num_format("dd/mm/yyyy");
        let date_time = Format::new().set_num_format("dd/mm/yyyy hh:mm:ss");
        for table in tables {
            let sheet = book.add_worksheet();
            if !table.name.is_empty() {
                sheet.set_name(&table.name).map_err(err)?;
            }
            for (c, col) in table.columns.iter().enumerate() {
                sheet.write_string_with_format(0, c as u16, &col.title, &bold).map_err(err)?;
            }
            sheet.set_freeze_panes(1, 0).map_err(err)?;
            let number_formats: Vec<Option<Format>> = table
                .columns
                .iter()
                .map(|col| (col.kind == ColumnKind::Number || col.kind == ColumnKind::Integer).then(|| col.format.trim()).filter(|f| !f.is_empty()).map(|f| Format::new().set_num_format(f)))
                .collect();
            for (r, row) in table.rows.iter().enumerate() {
                let r = r as u32 + 1;
                for (c, value) in row.iter().enumerate() {
                    let c16 = c as u16;
                    match value {
                        CellValue::Empty => {}
                        CellValue::Text(t) => {
                            sheet.write_string(r, c16, t).map_err(err)?;
                        }
                        CellValue::Number(n) => match number_formats.get(c).and_then(Option::as_ref) {
                            Some(f) => {
                                sheet.write_number_with_format(r, c16, *n, f).map_err(err)?;
                            }
                            None => {
                                sheet.write_number(r, c16, *n).map_err(err)?;
                            }
                        },
                        CellValue::Bool(b) => {
                            sheet.write_boolean(r, c16, *b).map_err(err)?;
                        }
                        CellValue::Date(y, m, d) => {
                            let dt = ExcelDateTime::from_ymd(*y as u16, *m as u8, *d as u8).map_err(err)?;
                            sheet.write_datetime_with_format(r, c16, &dt, &date).map_err(err)?;
                        }
                        CellValue::DateTime(y, m, d, h, mi, s) => {
                            let dt = ExcelDateTime::from_ymd(*y as u16, *m as u8, *d as u8).map_err(err)?.and_hms(*h as u16, *mi as u8, *s as f64).map_err(err)?;
                            sheet.write_datetime_with_format(r, c16, &dt, &date_time).map_err(err)?;
                        }
                    }
                }
            }
            sheet.autofit();
        }
        book.save_to_buffer().map_err(err)
    }
}
