//! O banco SQLite do ERP de exemplo: cria as 504 tabelas na primeira vez, põe uns registros
//! em cada uma, lê uma tabela para um ClientDataSet e grava o delta dele.
//!
//! O aylis não sabe de banco (item 16): a tela manda as alterações (o `Delta`, na ordem de
//! gravar, com os itens junto do mestre) e aqui vira SQL. A chave que o SQLite gera para um
//! mestre novo passa sozinha aos itens dele (`Delta::accept`).

use std::cell::RefCell;
use std::path::PathBuf;

use aylis::data::json::value_to_json;
use aylis::prelude::*;
use rusqlite::Connection;
use rusqlite::types::Value as Sql;
use serde_json::{Map, Value};

use super::esquema::{TABELAS, TELAS, Tabela, Tela};

/// Registros de exemplo em cada tabela mestre (e 3 itens em cada um).
const REGISTROS: i64 = 20;

thread_local! {
    static BANCO: RefCell<Option<Connection>> = const { RefCell::new(None) };
}

/// Onde fica o banco: `AYLIS_ERP_DB`, ou a pasta temporária (o exemplo recria sozinho).
pub fn caminho() -> PathBuf {
    std::env::var_os("AYLIS_ERP_DB").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("aylis-erp").join("erp.db"))
}

/// Roda `f` com a conexão (aberta, criada e semeada na primeira vez).
fn com<R>(f: impl FnOnce(&mut Connection) -> Result<R, String>) -> Result<R, String> {
    BANCO.with(|b| {
        let mut b = b.borrow_mut();
        if b.is_none() {
            *b = Some(abrir()?);
        }
        f(b.as_mut().expect("aberto acima"))
    })
}

fn abrir() -> Result<Connection, String> {
    let caminho = caminho();
    if let Some(pasta) = caminho.parent() {
        std::fs::create_dir_all(pasta).map_err(|e| format!("{}: {e}", pasta.display()))?;
    }
    let mut db = Connection::open(&caminho).map_err(|e| format!("{}: {e}", caminho.display()))?;
    db.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = OFF;").map_err(|e| e.to_string())?;
    criar(&mut db).map_err(|e| e.to_string())?;
    Ok(db)
}

fn tipo_sql(tipo: &str) -> &'static str {
    match tipo {
        "integer" | "boolean" => "INTEGER",
        "float" => "REAL",
        _ => "TEXT",
    }
}

fn criar(db: &mut Connection) -> rusqlite::Result<()> {
    let tx = db.transaction()?;
    for t in TABELAS {
        let colunas: Vec<String> =
            t.campos.iter().filter(|(c, _)| *c != "id").map(|(c, tipo)| format!("{c} {}", tipo_sql(tipo))).collect();
        tx.execute_batch(&format!("CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY AUTOINCREMENT, {});", t.nome, colunas.join(", ")))?;
        if t.campos.iter().any(|(c, _)| *c == "mestre_id") {
            tx.execute_batch(&format!("CREATE INDEX IF NOT EXISTS {0}_mestre ON {0} (mestre_id);", t.nome))?;
        }
    }
    let vazio: i64 = tx.query_row(&format!("SELECT count(*) FROM {}", TABELAS[0].nome), [], |r| r.get(0))?;
    if vazio == 0 {
        semear(&tx)?;
    }
    tx.commit()
}

/// Uns registros que façam sentido em cada tabela (as tabelas vêm aos pares: o mestre e os
/// itens ou o histórico dele).
fn semear(tx: &rusqlite::Transaction<'_>) -> rusqlite::Result<()> {
    for par in TABELAS.chunks(2) {
        let (mestre, itens) = (&par[0], &par[1]);
        let tela = TELAS.iter().find(|t| t.mestre == mestre.nome).expect("toda tabela mestre tem tela");
        for i in 1..=REGISTROS {
            let data = format!("2026-{:02}-{:02}", (i % 12) + 1, (i % 27) + 1);
            let unitario = 10 + i * 3;
            let valores: Vec<(&str, Sql)> = mestre
                .campos
                .iter()
                .filter(|(c, _)| *c != "id")
                .map(|(c, _)| {
                    let v = match *c {
                        "codigo" => Sql::Text(format!("{i:04}")),
                        "nome" => Sql::Text(format!("{} {i:02}", singular(tela.entidade))),
                        "ativo" => Sql::Integer((i % 5 != 0) as i64),
                        "criado_em" | "data" => Sql::Text(data.clone()),
                        "numero" => Sql::Integer(1000 + i),
                        "parceiro" => Sql::Text(format!("{} {i:02}", tela.parceiro)),
                        "valor_total" => Sql::Text(format!("{}.00", unitario * 6)),
                        "situacao" => Sql::Text(["Aberto", "Aprovado", "Concluído"][(i % 3) as usize].into()),
                        _ => Sql::Null,
                    };
                    (*c, v)
                })
                .collect();
            let id = inserir(tx, mestre, &valores)?;
            for j in 1..=3_i64 {
                let valores: Vec<(&str, Sql)> = itens
                    .campos
                    .iter()
                    .filter(|(c, _)| *c != "id")
                    .map(|(c, _)| {
                        let v = match *c {
                            "mestre_id" => Sql::Integer(id),
                            "data" => Sql::Text(data.clone()),
                            "descricao" => Sql::Text(format!("Ocorrência {j} do registro {i}")),
                            "responsavel" => Sql::Text(format!("Usuário {j}")),
                            "item" => Sql::Text(format!("Item {j}")),
                            "quantidade" => Sql::Integer(j),
                            "valor_unitario" => Sql::Text(format!("{unitario}.00")),
                            "valor_total" => Sql::Text(format!("{}.00", unitario * j)),
                            _ => Sql::Null,
                        };
                        (*c, v)
                    })
                    .collect();
                inserir(tx, itens, &valores)?;
            }
        }
    }
    Ok(())
}

/// "Funcionários" → "Funcionário" (só para os nomes de exemplo).
fn singular(plural: &str) -> String {
    let primeira = plural.split(' ').next().unwrap_or(plural);
    let s = primeira
        .strip_suffix("ões")
        .map(|r| format!("{r}ão"))
        .or_else(|| primeira.strip_suffix("is").map(|r| format!("{r}l")))
        .or_else(|| primeira.strip_suffix('s').map(str::to_string))
        .unwrap_or_else(|| primeira.to_string());
    s + &plural[primeira.len()..]
}

fn inserir(tx: &rusqlite::Transaction<'_>, tabela: &Tabela, valores: &[(&str, Sql)]) -> rusqlite::Result<i64> {
    let nomes: Vec<&str> = valores.iter().map(|(c, _)| *c).collect();
    let marcas = vec!["?"; nomes.len()].join(", ");
    tx.execute(
        &format!("INSERT INTO {} ({}) VALUES ({marcas})", tabela.nome, nomes.join(", ")),
        rusqlite::params_from_iter(valores.iter().map(|(_, v)| v)),
    )?;
    Ok(tx.last_insert_rowid())
}

fn tabela(nome: &str) -> Option<&'static Tabela> {
    TABELAS.iter().find(|t| t.nome == nome)
}

/// Os registros de uma tabela, como o `load_json` do ClientDataSet quer.
pub fn ler(nome: &str) -> Result<Value, String> {
    let t = tabela(nome).ok_or_else(|| format!("tabela {nome} não existe"))?;
    com(|db| {
        let colunas: Vec<&str> = t.campos.iter().map(|(c, _)| *c).collect();
        let mut st = db.prepare(&format!("SELECT {} FROM {} ORDER BY id", colunas.join(", "), t.nome)).map_err(|e| e.to_string())?;
        let linhas = st
            .query_map([], |r| {
                let mut obj = Map::new();
                for (i, (c, tipo)) in t.campos.iter().enumerate() {
                    let v: Sql = r.get(i)?;
                    obj.insert(c.to_string(), para_json(v, tipo));
                }
                Ok(Value::Object(obj))
            })
            .map_err(|e| e.to_string())?
            .collect::<rusqlite::Result<Vec<Value>>>()
            .map_err(|e| e.to_string())?;
        Ok(Value::Array(linhas))
    })
}

fn para_json(v: Sql, tipo: &str) -> Value {
    match (v, tipo) {
        (Sql::Null, _) => Value::Null,
        (Sql::Integer(i), "boolean") => Value::Bool(i != 0),
        (Sql::Integer(i), _) => Value::from(i),
        (Sql::Real(x), _) => Value::from(x),
        (Sql::Text(s), _) => Value::String(s),
        (Sql::Blob(_), _) => Value::Null,
    }
}

/// Um valor do delta para o SQLite (decimais e datas como texto, lógicos como 0 e 1).
fn para_sql(v: &DataValue) -> Sql {
    match value_to_json(v) {
        Value::Null => Sql::Null,
        Value::Bool(b) => Sql::Integer(b as i64),
        Value::Number(n) => n.as_i64().map(Sql::Integer).unwrap_or_else(|| Sql::Real(n.as_f64().unwrap_or_default())),
        Value::String(s) => Sql::Text(s),
        other => Sql::Text(other.to_string()),
    }
}

/// Grava o delta de uma tela numa transação: o mestre e os itens juntos. No primeiro erro,
/// desfaz tudo e só aquele registro volta recusado (os outros continuam pendentes na tela).
pub fn gravar(tela: &Tela, changes: &[Change]) -> Vec<ApplyResult> {
    let mut delta = Delta::new(changes.to_vec());
    let feito = com(|db| {
        let tx = db.transaction().map_err(|e| e.to_string())?;
        for i in 0..delta.len() {
            let c = delta.change(i).clone();
            let nome = if c.data_set == "cdsMestre" { tela.mestre } else { tela.itens };
            let id = |lista: &[(String, DataValue)]| lista.iter().find(|(n, _)| n == "id").map(|(_, v)| para_sql(v)).unwrap_or(Sql::Null);
            let resultado = match c.kind {
                ChangeKind::Insert => {
                    let valores: Vec<(&str, Sql)> = c.new.iter().filter(|(n, _)| n != "id").map(|(n, v)| (n.as_str(), para_sql(v))).collect();
                    let t = tabela(nome).ok_or("tabela desconhecida")?;
                    inserir(&tx, t, &valores).map(|novo| vec![("id".to_string(), DataValue::Int(novo))])
                }
                ChangeKind::Update => {
                    let sets: Vec<String> = c.new.iter().map(|(n, _)| format!("{n} = ?")).collect();
                    let mut params: Vec<Sql> = c.new.iter().map(|(_, v)| para_sql(v)).collect();
                    params.push(id(&c.key));
                    tx.execute(&format!("UPDATE {nome} SET {} WHERE id = ?", sets.join(", ")), rusqlite::params_from_iter(params)).map(|_| Vec::new())
                }
                ChangeKind::Delete => tx.execute(&format!("DELETE FROM {nome} WHERE id = ?"), [id(&c.key)]).map(|_| Vec::new()),
            };
            match resultado {
                Ok(valores) => delta.accept(c.reference, valores),
                Err(e) => return Err(format!("{}|{e}", c.reference)),
            }
        }
        tx.commit().map_err(|e| e.to_string())
    });
    match feito {
        Ok(()) => delta.into_results(),
        Err(erro) => match erro.split_once('|') {
            Some((r, mensagem)) => vec![ApplyResult::error(r.parse().unwrap_or_default(), format!("não gravado: {mensagem}"))],
            None => changes.iter().map(|c| ApplyResult::error(c.reference, erro.clone())).collect(),
        },
    }
}
