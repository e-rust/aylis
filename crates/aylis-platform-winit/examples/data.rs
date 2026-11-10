//! Dados completos: mestre-detalhe, grid com linha de filtro, ordenação e coluna fixa,
//! campos calculados e lookup (o ComboBox ligado a outro dataset) e máscara de CPF.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example data
//! ```

use aylis_controls::*;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

fn put(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) -> ComponentId {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
    id
}

fn caption(ui: &mut Ui, parent: ComponentId, text: &str, x: f32, y: f32) {
    let l = ui.add(Label::new(text), parent);
    put(ui, l, x, y, 400.0, 20.0);
    let c = ui.theme().color("text_muted");
    let _ = ui.set_prop(l, "color", c);
    let _ = ui.set_prop(l, "font_size", 12);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("aylis — dados", 1180.0, 760.0));
    ui.set_name(form, "frmDados")?;
    ui.control_mut(form).unwrap().set_padding(16.0);

    // ----- clientes (mestre) -----
    let customers = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Cód.").width(60.0), customers);
    ui.add(Field::new("nome", FieldType::String).label("Nome").width(150.0), customers);
    ui.add(Field::new("cidade", FieldType::String).label("Cidade").width(120.0), customers);
    ui.add(Field::new("cpf", FieldType::String).label("CPF").mask(mask::CPF).width(125.0), customers);
    ui.add(Field::new("limite", FieldType::Currency).label("Limite").format("#,##0.00").width(110.0), customers);
    ui.add(Field::new("categoria", FieldType::String).label("Categoria").calculated("iif(limite >= 5000, 'VIP', 'Normal')").width(90.0), customers);
    ui.add(Field::new("ativo", FieldType::Boolean).label("Ativo").width(60.0), customers);
    let ds_customers = ui.add(DataSource::new(customers), form);
    let names = ["Maria Silva", "João Pereira", "Ana Costa", "Carlos Souza", "Paula Nogueira", "Lucas Lima", "Fernanda Rocha", "Rafael Alves", "Beatriz Melo", "Gustavo Reis"];
    let cities = ["São Paulo", "Rio de Janeiro", "Recife", "Curitiba", "Salvador"];
    let rows = (0..names.len())
        .map(|i| {
            vec![
                Value::Int(i as i64 + 1),
                names[i].into(),
                cities[i % cities.len()].into(),
                Value::from(format!("{:011}", 12345678900u64 + i as u64 * 1_111_111)),
                Value::Decimal(Decimal::new(((i * 1733) % 9000 + 500) as i128 * 100, 2)),
                Value::Bool(i % 3 != 0),
            ]
        })
        .collect();
    data_set(&mut ui, customers).load_rows(rows)?;

    // ----- pedidos (detalhe) com lookup e calculado -----
    let products = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer), products);
    ui.add(Field::new("descricao", FieldType::String), products);
    let ds_products = ui.add(DataSource::new(products), form);
    data_set(&mut ui, products).load_rows(
        ["Teclado", "Mouse", "Monitor 27\"", "Cadeira", "Notebook"].iter().enumerate().map(|(i, d)| vec![Value::Int(i as i64 + 1), (*d).into()]).collect(),
    )?;
    let orders = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Pedido").width(70.0), orders);
    ui.add(Field::new("cliente_id", FieldType::Integer).visible_flag(false), orders);
    ui.add(Field::new("produto_id", FieldType::Integer).label("Prod.").width(55.0), orders);
    ui.add(Field::new("produto", FieldType::String).label("Produto").lookup(products, "produto_id", "id", "descricao").width(130.0), orders);
    ui.add(Field::new("qtd", FieldType::Integer).label("Qtd").width(50.0), orders);
    ui.add(Field::new("preco", FieldType::Currency).label("Preço").format("#,##0.00").width(90.0), orders);
    ui.add(Field::new("total", FieldType::Currency).label("Total").calculated("qtd * preco").format("#,##0.00").width(100.0), orders);
    let ds_orders = ui.add(DataSource::new(orders), form);
    ui.set_prop(orders, "master_source", ds_customers)?;
    ui.set_prop(orders, "master_fields", "cliente_id=id")?;
    let mut prows = Vec::new();
    for i in 0..40i64 {
        prows.push(vec![
            Value::Int(1000 + i),
            Value::Int(i % 10 + 1),
            Value::Int(i % 5 + 1),
            Value::Int(i % 4 + 1),
            Value::Decimal(Decimal::new(((i * 731) % 3000 + 99) as i128 * 100, 2)),
        ]);
    }
    data_set(&mut ui, orders).load_rows(prows)?;

    // ----- tela -----
    caption(&mut ui, form, "CLIENTES — título ordena; linha de baixo filtra; Ctrl/Shift+clique selecionam; digitar busca", 0.0, 0.0);
    let grid = ui.add(DataGrid::new(ds_customers), form);
    put(&mut ui, grid, 0.0, 24.0, 760.0, 330.0);
    ui.set_prop(grid, "filter_row", true)?;
    ui.set_prop(grid, "fixed_columns", 1u32)?;
    ui.set_prop(grid, "multi_select", true)?;
    ui.set_prop(grid, "incremental_search", true)?;
    ui.set_prop(grid, "footer", "id=count;limite=sum")?;
    caption(&mut ui, form, "PEDIDOS DO CLIENTE (mestre-detalhe) — Produto é lookup, Total é calculado", 0.0, 372.0);
    let grid2 = ui.add(DataGrid::new(ds_orders), form);
    put(&mut ui, grid2, 0.0, 396.0, 760.0, 320.0);
    ui.set_prop(grid2, "footer", "qtd=sum;total=sum")?;

    let x = 790.0;
    caption(&mut ui, form, "EDIÇÃO DO CLIENTE", x, 0.0);
    let labels = ["Nome", "CPF (máscara)", "Limite"];
    for (i, (label, field)) in labels.iter().zip(["nome", "cpf", "limite"]).enumerate() {
        caption(&mut ui, form, label, x, 30.0 + i as f32 * 62.0);
        let e = ui.add(Edit::new(""), form);
        put(&mut ui, e, x, 50.0 + i as f32 * 62.0, 340.0, 32.0);
        ui.set_binding(e, "text", Some(ds_customers), field);
    }
    caption(&mut ui, form, "Categoria (calculado)", x, 216.0);
    let cat = ui.add(Label::new(""), form);
    put(&mut ui, cat, x, 236.0, 340.0, 24.0);
    ui.set_binding(cat, "text", Some(ds_customers), "categoria");
    caption(&mut ui, form, "PRODUTO DO PEDIDO (lista de outro dataset)", x, 396.0);
    let lookup = ui.add(ComboBox::default(), form);
    put(&mut ui, lookup, x, 420.0, 340.0, 32.0);
    ui.set_binding(lookup, "items", Some(ds_products), "descricao");
    ui.set_binding(lookup, "values", Some(ds_products), "id");
    ui.set_binding(lookup, "value", Some(ds_orders), "produto_id");
    let nav = ui.add(DataNavigator::new(ds_orders), form);
    put(&mut ui, nav, x, 470.0, 306.0, 32.0);
    data_set(&mut ui, customers).refresh_links();
    data_set(&mut ui, orders).refresh_links();

    // Para a verificação: AYLIS_FILTER aplica um filtro na abertura.
    if let Ok(f) = std::env::var("AYLIS_FILTER") {
        data_set(&mut ui, customers).set_filter(&f)?;
    }
    if let Ok(i) = std::env::var("AYLIS_ORDER") {
        data_set(&mut ui, customers).set_index(&i)?;
    }
    aylis_platform_winit::run(ui, form)
}

/// `Field::visible(false)` sem conflitar com o nome do campo `visible`.
trait VisibleFlag {
    fn visible_flag(self, visible: bool) -> Self;
}

impl VisibleFlag for Field {
    fn visible_flag(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}
