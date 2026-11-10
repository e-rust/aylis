//! Eventos do form frmPrincipal: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! Os dados dos relatórios: pedidos de venda (o mestre), os itens e as parcelas de cada
//! pedido (os detalhes, ligados pelo `master_source` no form). Inventados aqui, sempre os
//! mesmos, com textos e números (quantidades com casas, preços, descontos e totais em
//! centavos, para as somas do relatório baterem com as do pedido). Os relatórios estão
//! inteiros no form: as bandas, os textos com `[campo]` e os totais (`[SUM(total)]`).

use aylis::prelude::*;
use crate::generated::FrmPrincipal;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

const CLIENTES: [(&str, &str, &str); 12] = [
    ("Padaria Pão Dourado Ltda.", "Campinas", "SP"),
    ("Mercado Bom Preço", "Belo Horizonte", "MG"),
    ("Construtora Alicerce S.A.", "Curitiba", "PR"),
    ("Farmácia Vida Plena", "Recife", "PE"),
    ("Restaurante Sabor da Terra", "Goiânia", "GO"),
    ("Oficina Mecânica Torque", "Porto Alegre", "RS"),
    ("Escola Pequeno Saber", "Salvador", "BA"),
    ("Hotel Brisa do Mar", "Florianópolis", "SC"),
    ("Clínica Bem-Estar", "Fortaleza", "CE"),
    ("Livraria Página Virada", "Rio de Janeiro", "RJ"),
    ("Agropecuária Campo Verde", "Cuiabá", "MT"),
    ("Papelaria Traço Fino", "Manaus", "AM"),
];

const VENDEDORES: [&str; 4] = ["Ana Souza", "Bruno Lima", "Carla Mendes", "Diego Rocha"];

/// Código, descrição, unidade, preço em centavos e peso por unidade (kg).
const PRODUTOS: [(&str, &str, &str, i64, f64); 16] = [
    ("CF-500", "Café torrado e moído 500 g", "PC", 1_890, 0.5),
    ("AR-5K", "Arroz agulhinha tipo 1 5 kg", "PC", 2_749, 5.0),
    ("FJ-1K", "Feijão carioca 1 kg", "PC", 899, 1.0),
    ("OL-900", "Óleo de soja 900 ml", "UN", 765, 0.9),
    ("AC-1K", "Açúcar cristal 1 kg", "PC", 529, 1.0),
    ("CM-50", "Cimento CP II 50 kg", "SC", 3_890, 50.0),
    ("AR-M3", "Areia média lavada", "M3", 14_500, 1_500.0),
    ("TN-18", "Tinta acrílica fosca 18 L", "LT", 32_990, 24.0),
    ("PF-A4", "Papel sulfite A4 500 folhas", "RS", 2_990, 2.3),
    ("CN-AZ", "Caneta esferográfica azul cx. 50", "CX", 4_250, 0.6),
    ("DT-5L", "Detergente concentrado 5 L", "GL", 2_180, 5.2),
    ("LV-NT", "Luva nitrílica cx. 100", "CX", 3_675, 0.7),
    ("RC-20", "Ração bovina 20 kg", "SC", 8_960, 20.0),
    ("FT-01", "Filtro de óleo automotivo", "UN", 3_420, 0.4),
    ("LB-1L", "Óleo lubrificante 5W30 1 L", "LT", 4_790, 0.9),
    ("CB-FL", "Cabo flexível 2,5 mm (metro)", "MT", 389, 0.03),
];

const SITUACOES: [&str; 3] = ["Faturado", "Aberto", "Entregue"];
const FORMAS: [&str; 4] = ["Boleto", "Pix", "Cartão", "Transferência"];
const PEDIDOS: i64 = 36;

/// Um número de 0 a `n - 1` que parece sorteado, sempre o mesmo para `(a, b)`.
fn mix(a: i64, b: i64, n: i64) -> i64 {
    ((a.wrapping_mul(2_654_435_761) ^ b.wrapping_mul(40_503)).rem_euclid(1_000_003)) % n
}

/// Centavos em `Decimal` (o campo moeda).
fn reais(centavos: i64) -> DataValue {
    Decimal::parse(&format!("{}.{:02}", centavos / 100, centavos % 100)).map(DataValue::from).unwrap_or_default()
}

/// Os três conjuntos de registros: pedidos, itens e parcelas.
fn inventar() -> (Vec<Vec<DataValue>>, Vec<Vec<DataValue>>, Vec<Vec<DataValue>>) {
    let (mut pedidos, mut itens, mut parcelas) = (Vec::new(), Vec::new(), Vec::new());
    for p in 0..PEDIDOS {
        let numero = 1001 + p;
        let mes = 1 + (p * 10 / PEDIDOS) as u8;
        let dia = 1 + mix(p, 1, 28) as u8;
        let (cliente, cidade, uf) = CLIENTES[mix(p, 2, CLIENTES.len() as i64) as usize];
        let vendedor = VENDEDORES[mix(p, 3, VENDEDORES.len() as i64) as usize];
        let quantos = 2 + mix(p, 4, 6);
        let (mut total, mut peso) = (0_i64, 0.0_f64);
        for i in 0..quantos {
            let (codigo, produto, unidade, preco, kg) = PRODUTOS[mix(p * 31 + i, 5, PRODUTOS.len() as i64) as usize];
            // Quantidade com até três casas quando a unidade é medida (metro, m³).
            let quantidade = match unidade {
                "MT" => (10 + mix(p, i, 190)) as f64 + mix(i, p, 4) as f64 * 0.25,
                "M3" => 1.0 + mix(p, i, 8) as f64 * 0.5,
                _ => (1 + mix(p, i + 7, 24)) as f64,
            };
            let desconto = [0.0, 0.0, 2.5, 5.0, 7.5, 10.0][mix(p, i + 11, 6) as usize];
            let bruto = (preco as f64 * quantidade).round();
            let liquido = (bruto * (1.0 - desconto / 100.0)).round() as i64;
            total += liquido;
            peso += kg * quantidade;
            itens.push(vec![
                numero.into(),
                (i + 1).into(),
                codigo.into(),
                produto.into(),
                unidade.into(),
                quantidade.into(),
                reais(preco),
                desconto.into(),
                reais(liquido),
            ]);
        }
        let emissao = Date::new(2026, mes, dia).expect("data válida");
        pedidos.push(vec![
            numero.into(),
            emissao.into(),
            cliente.into(),
            cidade.into(),
            uf.into(),
            vendedor.into(),
            SITUACOES[mix(p, 6, 3) as usize].into(),
            quantos.into(),
            ((peso * 1000.0).round() / 1000.0).into(),
            reais(total),
        ]);
        // As parcelas dividem o total; a última leva o resto dos centavos.
        let n = 1 + mix(p, 7, 4);
        let forma = FORMAS[mix(p, 8, FORMAS.len() as i64) as usize];
        for k in 0..n {
            let valor = if k == n - 1 { total - total / n * (n - 1) } else { total / n };
            let m = mes as i64 + k + 1;
            let vencimento = Date::new(2026 + ((m - 1) / 12) as i32, ((m - 1) % 12 + 1) as u8, dia.min(28)).expect("data válida");
            parcelas.push(vec![numero.into(), (k + 1).into(), vencimento.into(), forma.into(), reais(valor)]);
        }
    }
    (pedidos, itens, parcelas)
}

impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let (pedidos, itens, parcelas) = inventar();
        // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo pedido.
        for (id, linhas) in [(self.cdsItens.id, itens), (self.cdsParcelas.id, parcelas), (self.cdsPedidos.id, pedidos)] {
            if let Err(e) = data_set(ui, id).load_rows(linhas) {
                eprintln!("{}: {}", ui.name(id).unwrap_or_default(), e.message);
            }
        }
    }
}
