//! O que as 1009 telas do ERP de exemplo têm em comum: abrir (carregar do banco), incluir,
//! gravar, excluir, cancelar, buscar e abrir outra tela. Cada form chama daqui com as
//! [`Partes`] dele, montadas do acesso tipado (`self.cdsMestre.id`...): nada é achado pelo
//! nome. Qual tabela e qual campo de busca vêm de [`esquema::TELAS`], pelo nome do form.
//!
//! As partes de cada tela: `cdsMestre` (a entidade) e `cdsItens` (os itens ou o histórico,
//! ligados por `mestre_id`), quando ela tem, e o `lblStatus`; o menu tem o `cdsTelas`.

pub mod banco;
pub mod esquema;

use std::cell::RefCell;
use std::collections::HashMap;

use aylis::prelude::*;

use esquema::{TELAS, Tela};

/// Os componentes de uma tela que o código comum usa, tirados do acesso tipado do form.
#[derive(Clone, Copy, Debug)]
pub struct Partes {
    pub form: ComponentId,
    pub mestre: Option<ComponentId>,
    pub itens: Option<ComponentId>,
    pub status: Option<ComponentId>,
    /// A lista das telas (só no menu).
    pub telas: Option<ComponentId>,
}

thread_local! {
    /// As partes de cada tela aberta (a lista abre o cadastro e posiciona o `cdsMestre` dele;
    /// o cadastro é de outro tipo, então a lista não tem o acesso tipado dele).
    static ABERTAS: RefCell<HashMap<ComponentId, Partes>> = RefCell::new(HashMap::new());
}

fn tela(ui: &Ui, form: ComponentId) -> Option<&'static Tela> {
    let nome = ui.name(form)?;
    TELAS.iter().find(|t| t.form.eq_ignore_ascii_case(nome))
}

fn status(ui: &mut Ui, p: Partes, texto: impl Into<String>) {
    if let Some(l) = p.status {
        let _ = ui.set_prop(l, "text", texto.into());
    }
}

fn erro(ui: &mut Ui, p: Partes, resultado: Result<(), String>) {
    if let Err(e) = resultado {
        status(ui, p, e);
    }
}

fn carregar(ui: &mut Ui, cds: Option<ComponentId>, tabela: &str) -> Result<(), String> {
    let Some(cds) = cds else { return Ok(()) };
    let linhas = banco::ler(tabela)?;
    data_set(ui, cds).load_json(&linhas).map_err(|e| e.message)
}

/// O `on_show` de cada tela (e do menu).
pub fn abrir(ui: &mut Ui, p: Partes) {
    ABERTAS.with(|a| a.borrow_mut().insert(p.form, p));
    if p.telas.is_some() {
        return menu(ui, p);
    }
    let Some(t) = tela(ui, p.form) else { return };
    // Os itens antes: quando o mestre abre, eles já filtram pelo registro atual.
    let mut resultado = if t.itens.is_empty() { Ok(()) } else { carregar(ui, p.itens, t.itens) };
    if resultado.is_ok() && !t.mestre.is_empty() {
        resultado = carregar(ui, p.mestre, t.mestre);
        if let Some(cds) = p.mestre {
            // A gravação: o delta do mestre (com os itens) vira SQL numa transação.
            data_set(ui, cds).on_apply(move |changes| banco::gravar(t, changes));
        }
    }
    let total = (if t.mestre.is_empty() { p.itens } else { p.mestre }).map_or(0, |c| view(ui, c).record_count());
    match resultado {
        Ok(()) => status(ui, p, format!("{total} registros")),
        Err(e) => status(ui, p, e),
    }
}

fn editando(ui: &mut Ui, cds: Option<ComponentId>) -> Result<(), String> {
    let Some(cds) = cds else { return Ok(()) };
    if view(ui, cds).state().is_editing() { data_set(ui, cds).post().map_err(|e| e.message) } else { Ok(()) }
}

pub fn novo(ui: &mut Ui, p: Partes) {
    let (Some(t), Some(cds)) = (tela(ui, p.form), p.mestre) else { return };
    let resultado = editando(ui, p.itens).and_then(|_| {
        let numero = view(ui, cds).record_count() as i64 + 1001;
        let mut ds = data_set(ui, cds);
        ds.append().map_err(|e| e.message)?;
        let hoje = Date::today();
        let iniciais: Vec<(&str, DataValue)> = if t.documento {
            vec![("numero", numero.into()), ("data", DataValue::Date(hoje)), ("situacao", "Aberto".into())]
        } else {
            vec![("ativo", true.into()), ("criado_em", DataValue::Date(hoje))]
        };
        for (campo, valor) in iniciais {
            ds.set_value(campo, valor).map_err(|e| e.message)?;
        }
        Ok(())
    });
    erro(ui, p, resultado);
}

pub fn novo_item(ui: &mut Ui, p: Partes) {
    let Some(itens) = p.itens else { return };
    let resultado = editando(ui, p.mestre).and_then(|_| data_set(ui, itens).append().map_err(|e| e.message));
    erro(ui, p, resultado);
}

pub fn salvar(ui: &mut Ui, p: Partes) {
    let Some(cds) = p.mestre else { return };
    let resultado = editando(ui, p.mestre).and_then(|_| editando(ui, p.itens));
    if let Err(e) = resultado {
        return status(ui, p, e);
    }
    let pendentes = view(ui, cds).change_count() + p.itens.map_or(0, |i| view(ui, i).change_count());
    match data_set(ui, cds).apply_updates() {
        Ok(()) => status(ui, p, format!("{pendentes} alterações gravadas")),
        Err(e) => status(ui, p, e.message),
    }
}

pub fn excluir(ui: &mut Ui, p: Partes) {
    let Some(cds) = p.mestre else { return };
    let resultado = data_set(ui, cds).delete().map_err(|e| e.message);
    match resultado {
        Ok(()) => status(ui, p, "Excluído: clique em Salvar para gravar"),
        Err(e) => status(ui, p, e),
    }
}

pub fn cancelar(ui: &mut Ui, p: Partes) {
    for cds in [p.itens, p.mestre].into_iter().flatten() {
        let mut ds = data_set(ui, cds);
        let _ = ds.cancel();
        let _ = ds.cancel_updates();
    }
    status(ui, p, "Alterações desfeitas");
}

pub fn fechar(ui: &mut Ui, p: Partes) {
    ABERTAS.with(|a| a.borrow_mut().remove(&p.form));
    ui.close(p.form);
}

/// O texto entre aspas de um filtro (`'` vira `''`).
fn literal(texto: &str) -> String {
    format!("'%{}%'", texto.replace('\'', "''"))
}

pub fn buscar(ui: &mut Ui, p: Partes, texto: &str) {
    let (cds, filtro) = if p.telas.is_some() {
        let l = literal(texto);
        (p.telas, format!("entidade like {l} or modulo like {l} or tela like {l}"))
    } else {
        let Some(t) = tela(ui, p.form) else { return };
        if t.mestre.is_empty() {
            (p.itens, format!("{} like {}", t.busca_itens, literal(texto)))
        } else {
            (p.mestre, format!("{} like {}", t.busca, literal(texto)))
        }
    };
    let Some(cds) = cds else { return };
    let filtro = if texto.trim().is_empty() { String::new() } else { filtro };
    let resultado = data_set(ui, cds).set_filter(&filtro).map_err(|e| e.message);
    let total = view(ui, cds).record_count();
    match resultado {
        Ok(()) => status(ui, p, format!("{total} registros")),
        Err(e) => status(ui, p, e),
    }
}

/// Abre a tela `nome` (escolhida numa linha de dados: o menu tem as 1009, então o form vem
/// pelo nome que está na linha).
fn mostrar(ui: &mut Ui, nome: &str) -> Result<ComponentId, String> {
    let form = ui.create_form(nome).map_err(|e| e.to_string())?;
    ui.show(form).map_err(|e| e.to_string())?;
    Ok(form)
}

/// O duplo clique (ou o botão) do menu e da lista: abre a tela escolhida; da lista, o
/// cadastro já no registro.
pub fn abrir_tela(ui: &mut Ui, p: Partes) {
    if let Some(cds) = p.telas {
        let Ok(DataValue::String(nome)) = view(ui, cds).value("form") else { return };
        let resultado = mostrar(ui, &nome).map(|_| ());
        return erro(ui, p, resultado);
    }
    let (Some(t), Some(cds)) = (tela(ui, p.form), p.mestre) else { return };
    let id = view(ui, cds).value("id").unwrap_or(DataValue::Null);
    let resultado = mostrar(ui, t.cadastro).and_then(|cadastro| {
        // As partes do cadastro, que ele deu ao abrir (o `on_show` roda no `show`).
        let Some(mestre) = ABERTAS.with(|a| a.borrow().get(&cadastro).and_then(|c| c.mestre)) else { return Ok(()) };
        data_set(ui, mestre).locate("id", id).map(|_| ()).map_err(|e| e.message)
    });
    erro(ui, p, resultado);
}

/// O menu: todas as telas numa grade, com busca.
fn menu(ui: &mut Ui, p: Partes) {
    let Some(cds) = p.telas else { return };
    let tipo = |t: &Tela| match t.tipo {
        "Cad" => "Cadastro",
        "Lista" => "Lista",
        "Consulta" => "Consulta",
        _ if t.documento => "Itens de todos",
        _ => "Histórico de todos",
    };
    let linhas: Vec<serde_json::Value> = TELAS
        .iter()
        .map(|t| serde_json::json!({ "modulo": t.modulo, "entidade": t.entidade, "tela": tipo(t), "form": t.form }))
        .collect();
    let resultado = data_set(ui, cds).load_json(&serde_json::Value::Array(linhas)).map_err(|e| e.message);
    match resultado {
        Ok(()) => {
            let texto = format!("{} telas, {} tabelas; banco em {}", TELAS.len() + 1, esquema::TABELAS.len(), banco::caminho().display());
            status(ui, p, texto)
        }
        Err(e) => status(ui, p, e),
    }
}
