//! Eventos do form frmIBGE: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! Os dados vêm da pasta `assets/dados` (a mesma para as 8 linguagens; `aylis-tool ibge`
//! baixa e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que
//! fazem o mestre-detalhe sozinhos (`master_source`); aqui fica o que muda a cada estado ou
//! município escolhido: as bandeiras, os mapas e os dados dos gráficos.

use std::cell::RefCell;
use std::collections::HashMap;

use aylis::prelude::*;
use serde_json::{Value, json};

use crate::generated::FrmIBGE;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// A pasta dos dados, dentro do projeto.
const DADOS: &str = "assets/dados/";

#[derive(Default)]
struct Dados {
    /// Os municípios (para os 10 maiores de cada estado).
    municipios: Vec<Value>,
    /// Código do estado ou do município → `[[ano, habitantes], ...]`.
    populacao: HashMap<String, Value>,
}

thread_local! {
    static DADOS_CARREGADOS: RefCell<Dados> = RefCell::new(Dados::default());
}

/// Um arquivo JSON da pasta dos dados (`Null` se faltar).
fn ler(ui: &Ui, arquivo: &str) -> Value {
    ui.services()
        .resources
        .as_ref()
        .and_then(|r| r.read(&format!("{DADOS}{arquivo}")))
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or(Value::Null)
}

impl FrmIBGE {
    /// Põe os registros (uma lista de objetos) num ClientDataSet do form.
    fn carregar(&mut self, ui: &mut Ui, dataset: ComponentId, linhas: &Value) {
        if let Err(e) = data_set(ui, dataset).load_json(linhas) {
            eprintln!("{}: {}", ui.name(dataset).unwrap_or_default(), e.message);
        }
    }

    /// Um campo do registro atual de um ClientDataSet do form.
    fn campo(&mut self, ui: &Ui, dataset: ComponentId, nome: &str) -> DataValue {
        view(ui, dataset).value(nome).unwrap_or(DataValue::Null)
    }

    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let estados = ler(ui, "estados.json");
        if !estados.is_array() {
            self.lblFonte.set_text(ui, "Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados");
            return;
        }
        let municipios = ler(ui, "municipios.json");
        let distritos = ler(ui, "distritos.json");
        let populacao = ler(ui, "populacao.json");
        DADOS_CARREGADOS.with(|d| {
            let mut d = d.borrow_mut();
            d.municipios = municipios.as_array().cloned().unwrap_or_default();
            d.populacao = populacao.as_object().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default();
        });
        self.svgBrasil.set_source(ui, format!("{DADOS}mapas/BR.svg"));
        // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
        self.carregar(ui, self.cdsDistritos.id, &distritos);
        self.carregar(ui, self.cdsMunicipios.id, &municipios);
        self.carregar(ui, self.cdsEstados.id, &estados);
        self.estado_mudou(ui, _sender, _args);
    }

    /// Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios.
    pub fn estado_mudou(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let Some(id) = self.campo(ui, self.cdsEstados.id, "id").as_i64() else { return };
        let sigla = self.campo(ui, self.cdsEstados.id, "sigla").to_string();
        self.lblEstado.set_text(ui, self.campo(ui, self.cdsEstados.id, "nome").to_string());
        self.lblRegiao.set_text(ui, format!("{sigla} · Região {}", self.campo(ui, self.cdsEstados.id, "regiao")));
        self.imgBandeiraEstado.set_image(ui, imagem(&self.campo(ui, self.cdsEstados.id, "bandeira").to_string()));
        self.lblEstadoEstimativa.set_text(ui, format!("Estimativa de {}", self.campo(ui, self.cdsEstados.id, "estimativa_ano")));
        self.svgBrasil.set_highlight(ui, format!("e{id}"));
        let mapa = format!("{DADOS}mapas/{id}.svg");
        self.svgEstado.set_source(ui, mapa.clone());
        self.svgMunicipio.set_source(ui, mapa);

        let mut do_estado: Vec<(String, i64)> = DADOS_CARREGADOS.with(|d| {
            d.borrow()
                .municipios
                .iter()
                .filter(|m| m["uf_id"].as_i64() == Some(id))
                .map(|m| (m["nome"].as_str().unwrap_or_default().to_string(), m["populacao"].as_i64().unwrap_or(0)))
                .collect()
        });
        do_estado.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let maiores: Vec<Value> = do_estado.iter().take(10).map(|(nome, habitantes)| json!({ "nome": nome, "habitantes": habitantes })).collect();
        self.carregar(ui, self.cdsMaiores.id, &Value::Array(maiores));
        self.municipio_mudou(ui, sender, args);
    }

    /// Outro município: a bandeira, o destaque nos mapas e a população ano a ano.
    pub fn municipio_mudou(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let Some(id) = self.campo(ui, self.cdsMunicipios.id, "id").as_i64() else { return };
        self.lblMunicipio.set_text(ui, self.campo(ui, self.cdsMunicipios.id, "nome").to_string());
        self.lblCodigo.set_text(ui, format!("Código IBGE {id}"));
        let bandeira = self.campo(ui, self.cdsMunicipios.id, "bandeira").to_string();
        self.imgBandeiraMunicipio.set_image(ui, imagem(&bandeira));
        self.lblSemBandeira.set_visible(ui, bandeira.is_empty());
        self.lblEstimativa.set_text(ui, format!("Estimativa de {}", self.campo(ui, self.cdsMunicipios.id, "estimativa_ano")));
        let alvo = format!("m{id}");
        self.svgEstado.set_highlight(ui, alvo.clone());
        self.svgMunicipio.set_highlight(ui, alvo.clone());
        self.svgMunicipio.set_zoom_to(ui, alvo);

        let serie = DADOS_CARREGADOS.with(|d| d.borrow().populacao.get(&id.to_string()).cloned()).unwrap_or(Value::Null);
        let linhas: Vec<Value> = serie
            .as_array()
            .into_iter()
            .flatten()
            .map(|p| json!({ "ano": p[0].to_string(), "habitantes": p[1] }))
            .collect();
        self.carregar(ui, self.cdsPopulacao.id, &Value::Array(linhas));
    }
}

/// O caminho de uma imagem da pasta dos dados (vazio: não tem).
fn imagem(caminho: &str) -> String {
    if caminho.is_empty() { String::new() } else { format!("{DADOS}{caminho}") }
}

