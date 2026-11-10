//! Os dados abertos do IBGE para o exemplo `examples/IBGE`: baixa as respostas da API
//! (servicodados.ibge.gov.br) e as bandeiras (Wikimedia Commons, achadas pelo Wikidata) uma
//! vez só, guardadas no cache (`ibge-cache`, ao lado da pasta `assets`: o que já está lá não
//! é baixado de novo), e
//! monta os arquivos que o aplicativo lê, iguais para as 8 linguagens:
//!
//! ```text
//! estados.json      id, sigla, nome, regiao, populacao, area, densidade, pib, pib_per_capita...
//! municipios.json   id, uf_id, uf, nome, mesorregiao, microrregiao, regiões, os números...
//! distritos.json    id, municipio_id, nome
//! populacao.json    { "35": [[2001, 37032403], ...], "3550308": [...] } (estimativas e Censo 2022)
//! mapas/BR.svg      os estados (id "e35")
//! mapas/35.svg      os municípios do estado (id "m3550308")
//! bandeiras/estados/SP.png, bandeiras/municipios/3550308.png
//! bandeiras/CREDITOS.md  de cada bandeira, o original no Commons, o autor e a licença
//! ```
//!
//! ```text
//! aylis-tool ibge examples/IBGE/assets/dados            baixa o que falta e gera
//! aylis-tool ibge examples/IBGE/assets/dados --offline  só com o que já está no cache
//! ```
//!
//! Os downloads são feitos pelo `curl` (vem com o Windows 10 e 11, o macOS e o Linux).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Map, Value, json};

const API: &str = "https://servicodados.ibge.gov.br/api";
const AGENT: &str = "aylis-example-ibge/1.0 (https://github.com/e-rust/aylis)";

/// Baixa o que falta e gera os arquivos do aplicativo em `dir`. `log` recebe o andamento.
pub fn run(dir: &Path, offline: bool, log: &mut dyn FnMut(&str)) -> Result<(), String> {
    // O cache fica fora de `assets` (que vai inteiro para dentro do executável): ao lado.
    let cache = match dir.parent() {
        Some(assets) if assets.file_name().is_some_and(|n| n == "assets") => assets.parent().unwrap_or(assets).join("ibge-cache"),
        _ => dir.join("ibge"),
    };
    let get = |rel: &str, url: &str, log: &mut dyn FnMut(&str)| -> Result<Value, String> { fetch_json(&cache.join(rel), url, offline, log) };

    log("Localidades (estados, municípios e distritos)");
    let estados = get("localidades/estados.json", &format!("{API}/v1/localidades/estados?orderBy=nome"), log)?;
    let municipios = get("localidades/municipios.json", &format!("{API}/v1/localidades/municipios?orderBy=nome"), log)?;
    let distritos = get("localidades/distritos.json", &format!("{API}/v1/localidades/distritos?orderBy=nome"), log)?;

    log("Censo 2022, PIB e estimativas de população");
    let mut censo = HashMap::new();
    let mut pib = HashMap::new();
    let mut series: BTreeMap<String, BTreeMap<i64, f64>> = BTreeMap::new();
    for level in ["N3", "N6"] {
        let all = format!("localidades={level}%5Ball%5D");
        let c = get(&format!("estatisticas/censo2022_{level}.json"), &format!("{API}/v3/agregados/4714/periodos/2022/variaveis/93|6318|614?{all}"), log)?;
        read_aggregate(&c, &mut censo);
        let p = get(&format!("estatisticas/pib_{level}.json"), &format!("{API}/v3/agregados/5938/periodos/2021/variaveis/37|497?{all}"), log)?;
        read_aggregate(&p, &mut pib);
    }
    // As estimativas: os estados de uma vez; os municípios ano a ano (todos os anos juntos
    // passam do que a API responde).
    let est_n3 = get("estatisticas/estimativas_N3.json", &format!("{API}/v3/agregados/6579/periodos/all/variaveis/9324?localidades=N3%5Ball%5D"), log)?;
    let mut years: Vec<String> = Vec::new();
    for s in series_of(&est_n3) {
        for (year, v) in s.1 {
            if !years.contains(&year) {
                years.push(year.clone());
            }
            if let (Ok(y), Some(v)) = (year.parse(), number(&v)) {
                series.entry(s.0.clone()).or_default().insert(y, v);
            }
        }
    }
    years.sort();
    for year in &years {
        let e = get(&format!("estatisticas/estimativas_N6_{year}.json"), &format!("{API}/v3/agregados/6579/periodos/{year}/variaveis/9324?localidades=N6%5Ball%5D"), log)?;
        for (id, values) in series_of(&e) {
            for (y, v) in values {
                if let (Ok(y), Some(v)) = (y.parse(), number(&v)) {
                    series.entry(id.clone()).or_default().insert(y, v);
                }
            }
        }
    }
    // O Censo 2022 entra na série (as estimativas pulam 2022).
    for ((id, _), v) in censo.iter().filter(|((_, var), _)| var == "93") {
        if let Some(v) = number(v) {
            series.entry(id.clone()).or_default().insert(2022, v);
        }
    }

    log("Malhas (os mapas)");
    let mut meshes = vec![("BR".to_string(), get("malhas/BR.json", &format!("{API}/v3/malhas/paises/BR?intrarregiao=UF&qualidade=minima&formato=application/vnd.geo%2Bjson"), log)?)];
    for uf in estados.as_array().into_iter().flatten() {
        let id = uf["id"].to_string();
        let m = get(&format!("malhas/{id}.json"), &format!("{API}/v3/malhas/estados/{id}?intrarregiao=municipio&qualidade=intermediaria&formato=application/vnd.geo%2Bjson"), log)?;
        meshes.push((id, m));
    }

    log("Bandeiras (Wikidata e Wikimedia Commons)");
    let flags = flags(dir, &cache, offline, log)?;

    log("Gerando os arquivos do aplicativo");
    let value = |id: &str, var: &str, from: &HashMap<(String, String), Value>| from.get(&(id.to_string(), var.to_string())).and_then(number);
    let last_estimate = |id: &str| series.get(id).and_then(|s| s.iter().filter(|(y, _)| **y != 2022).next_back()).map(|(y, v)| (*y, *v));
    let int = |v: Option<f64>| v.map(|x| x.round() as i64);
    let per_capita = |id: &str| {
        let p = value(id, "37", &pib)?;
        let people = series.get(id)?.get(&2021)?;
        Some((p * 1000.0 / people * 100.0).round() / 100.0)
    };

    // Distritos: a lista e quantos cada município tem.
    let mut district_count: HashMap<i64, i64> = HashMap::new();
    let mut district_rows = Vec::new();
    for d in distritos.as_array().into_iter().flatten() {
        let city = d["municipio"]["id"].as_i64().unwrap_or(0);
        *district_count.entry(city).or_default() += 1;
        district_rows.push(json!({ "id": d["id"], "municipio_id": city, "nome": d["nome"] }));
    }

    let mut city_count: HashMap<i64, i64> = HashMap::new();
    let mut city_rows = Vec::new();
    for m in municipios.as_array().into_iter().flatten() {
        let id = m["id"].as_i64().unwrap_or(0);
        let key = id.to_string();
        // Município novo pode vir sem microrregião: o estado sai da região imediata.
        let uf = if m["microrregiao"].is_object() { &m["microrregiao"]["mesorregiao"]["UF"] } else { &m["regiao-imediata"]["regiao-intermediaria"]["UF"] };
        let uf_id = uf["id"].as_i64().unwrap_or(0);
        *city_count.entry(uf_id).or_default() += 1;
        let (year, estimate) = last_estimate(&key).map_or((Value::Null, Value::Null), |(y, v)| (y.into(), (v.round() as i64).into()));
        let flag = flags.get(&key).cloned().unwrap_or_default();
        city_rows.push(json!({
            "id": id,
            "uf_id": uf_id,
            "uf": uf["sigla"],
            "nome": m["nome"],
            "mesorregiao": m["microrregiao"]["mesorregiao"]["nome"],
            "microrregiao": m["microrregiao"]["nome"],
            "regiao_intermediaria": m["regiao-imediata"]["regiao-intermediaria"]["nome"],
            "regiao_imediata": m["regiao-imediata"]["nome"],
            "populacao": int(value(&key, "93", &censo)),
            "area": value(&key, "6318", &censo),
            "densidade": value(&key, "614", &censo),
            "pib": int(value(&key, "37", &pib)),
            "pib_participacao": value(&key, "497", &pib),
            "pib_per_capita": per_capita(&key),
            "estimativa": estimate,
            "estimativa_ano": year,
            "distritos": district_count.get(&id).copied().unwrap_or(0),
            "bandeira": flag,
        }));
    }

    let mut state_rows = Vec::new();
    for uf in estados.as_array().into_iter().flatten() {
        let id = uf["id"].as_i64().unwrap_or(0);
        let key = id.to_string();
        let sigla = uf["sigla"].as_str().unwrap_or_default();
        let (year, estimate) = last_estimate(&key).map_or((Value::Null, Value::Null), |(y, v)| (y.into(), (v.round() as i64).into()));
        state_rows.push(json!({
            "id": id,
            "sigla": sigla,
            "nome": uf["nome"],
            "regiao": uf["regiao"]["nome"],
            "populacao": int(value(&key, "93", &censo)),
            "area": value(&key, "6318", &censo),
            "densidade": value(&key, "614", &censo),
            "pib": int(value(&key, "37", &pib)),
            "pib_per_capita": per_capita(&key),
            "estimativa": estimate,
            "estimativa_ano": year,
            "municipios": city_count.get(&id).copied().unwrap_or(0),
            "bandeira": flags.get(sigla).cloned().unwrap_or_default(),
        }));
    }

    let population: Map<String, Value> = series.iter().map(|(id, s)| (id.clone(), Value::Array(s.iter().map(|(y, v)| json!([y, v.round() as i64])).collect()))).collect();
    write(&dir.join("estados.json"), &Value::Array(state_rows))?;
    write(&dir.join("municipios.json"), &Value::Array(city_rows))?;
    write(&dir.join("distritos.json"), &Value::Array(district_rows))?;
    write(&dir.join("populacao.json"), &Value::Object(population))?;

    std::fs::create_dir_all(dir.join("mapas")).map_err(|e| e.to_string())?;
    for (id, mesh) in &meshes {
        let prefix = if id == "BR" { "e" } else { "m" };
        let svg = mesh_to_svg(mesh, prefix);
        std::fs::write(dir.join("mapas").join(format!("{id}.svg")), svg).map_err(|e| format!("mapas/{id}.svg: {e}"))?;
    }
    log(&format!("Pronto: {} estados, {} municípios, {} distritos, {} bandeiras", estados.as_array().map_or(0, Vec::len), municipios.as_array().map_or(0, Vec::len), distritos.as_array().map_or(0, Vec::len), flags.len()));
    Ok(())
}

/// O JSON guardado em `file`, ou baixado de `url` (e guardado) se ainda não está lá.
fn fetch_json(file: &Path, url: &str, offline: bool, log: &mut dyn FnMut(&str)) -> Result<Value, String> {
    if let Some(v) = std::fs::read(file).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok()).filter(valid) {
        return Ok(v);
    }
    if offline {
        return Err(format!("{} não está no cache (rode sem --offline)", file.display()));
    }
    log(&format!("  baixando {}", file.file_name().unwrap_or_default().to_string_lossy()));
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let tmp = file.with_extension("tmp");
    let status = Command::new("curl")
        .args(["-sSL", "--retry", "4", "--compressed", "-A", AGENT, "-o"])
        .arg(&tmp)
        .arg(url)
        .status()
        .map_err(|e| format!("curl: {e}"))?;
    let v = std::fs::read(&tmp).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok()).filter(valid);
    let Some(v) = v.filter(|_| status.success()) else {
        let _ = std::fs::remove_file(&tmp);
        return Err(format!("não consegui baixar {url}"));
    };
    std::fs::rename(&tmp, file).map_err(|e| e.to_string())?;
    Ok(v)
}

/// Resposta de erro da API (`{"statusCode": 500, ...}`) não vale.
fn valid(v: &Value) -> bool {
    v.get("statusCode").is_none()
}

fn write(file: &Path, v: &Value) -> Result<(), String> {
    std::fs::write(file, serde_json::to_string(v).unwrap_or_default()).map_err(|e| format!("{}: {e}", file.display()))
}

/// Os valores de uma consulta de agregados: (localidade, variável) → valor (do único período).
fn read_aggregate(v: &Value, out: &mut HashMap<(String, String), Value>) {
    for var in v.as_array().into_iter().flatten() {
        let id = var["id"].as_str().unwrap_or_default().to_string();
        for r in var["resultados"].as_array().into_iter().flatten() {
            for s in r["series"].as_array().into_iter().flatten() {
                let place = s["localidade"]["id"].as_str().unwrap_or_default().to_string();
                if let Some(value) = s["serie"].as_object().and_then(|m| m.values().next()) {
                    out.insert((place, id.clone()), value.clone());
                }
            }
        }
    }
}

/// As séries de uma consulta de agregados: localidade → [(período, valor)].
fn series_of(v: &Value) -> Vec<(String, Vec<(String, Value)>)> {
    let mut out = Vec::new();
    for var in v.as_array().into_iter().flatten() {
        for r in var["resultados"].as_array().into_iter().flatten() {
            for s in r["series"].as_array().into_iter().flatten() {
                let place = s["localidade"]["id"].as_str().unwrap_or_default().to_string();
                let values = s["serie"].as_object().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default();
                out.push((place, values));
            }
        }
    }
    out
}

/// Número de um valor da API (vem como texto; "-", "..." e "X" são "sem dado").
fn number(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Bandeiras

/// Baixa as bandeiras que faltam. Devolve a sigla do estado ou o código do município →
/// caminho da imagem relativo à pasta dos dados (`bandeiras/estados/SP.png`; algumas são
/// `.jpg`, como o original no Commons).
///
/// O Wikimedia bloqueia quem pede rápido demais: o endereço das miniaturas vem da API do
/// Commons (50 por consulta, uma consulta a cada 2 s) e as imagens, uma por segundo. O que
/// já foi achado fica no cache (`wikidata/miniaturas.json`).
fn flags(dir: &Path, cache: &Path, offline: bool, log: &mut dyn FnMut(&str)) -> Result<HashMap<String, String>, String> {
    let sparql = |query: &str| format!("https://query.wikidata.org/sparql?format=json&query={}", encode(query));
    let states = fetch_json(
        &cache.join("wikidata/estados.json"),
        &sparql("SELECT ?iso ?flag WHERE { ?item wdt:P300 ?iso . FILTER(STRSTARTS(?iso, \"BR-\")) ?item wdt:P41 ?flag . }"),
        offline,
        log,
    )?;
    let cities = fetch_json(&cache.join("wikidata/municipios.json"), &sparql("SELECT ?code ?flag WHERE { ?item wdt:P1585 ?code . ?item wdt:P41 ?flag . }"), offline, log)?;
    // (chave, título do arquivo no Commons, pasta, largura)
    let mut wanted: Vec<(String, String, &str, u32)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut add = |key: String, flag: &str, folder: &'static str, width: u32| {
        let name = decode(flag.rsplit("Special:FilePath/").next().unwrap_or_default());
        if key.is_empty() || name.is_empty() || !seen.insert(key.clone()) {
            return;
        }
        wanted.push((key, format!("File:{name}"), folder, width));
    };
    for b in bindings(&states) {
        let iso = b["iso"]["value"].as_str().unwrap_or_default();
        add(iso.trim_start_matches("BR-").to_string(), b["flag"]["value"].as_str().unwrap_or_default(), "estados", 250);
    }
    for b in bindings(&cities) {
        let code = b["code"]["value"].as_str().unwrap_or_default();
        if code.len() == 7 && code.chars().all(|c| c.is_ascii_digit()) {
            add(code.to_string(), b["flag"]["value"].as_str().unwrap_or_default(), "municipios", 120);
        }
    }
    let saved = |key: &str, folder: &str| -> Option<PathBuf> {
        ["png", "jpg"].iter().map(|ext| dir.join("bandeiras").join(folder).join(format!("{key}.{ext}"))).find(|f| f.exists())
    };
    let missing: Vec<&(String, String, &str, u32)> = wanted.iter().filter(|(k, _, folder, _)| saved(k, folder).is_none()).collect();
    if !missing.is_empty() && !offline {
        log(&format!("  {} bandeiras para baixar (devagar: o Wikimedia limita os pedidos)", missing.len()));
        // 1. Os endereços das miniaturas (título → URL), guardados a cada consulta.
        let thumbs_file = cache.join("wikidata/miniaturas.json");
        let mut thumbs: Map<String, Value> = std::fs::read(&thumbs_file).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        for width in [250, 120] {
            let titles: Vec<&str> = missing.iter().filter(|(_, t, _, w)| *w == width && !thumbs.contains_key(&format!("{width}|{t}"))).map(|(_, t, _, _)| t.as_str()).collect();
            for chunk in titles.chunks(50) {
                let url = format!(
                    "https://commons.wikimedia.org/w/api.php?action=query&format=json&formatversion=2&prop=imageinfo&iiprop=url&iiurlwidth={width}&titles={}",
                    encode(&chunk.join("|"))
                );
                let out = Command::new("curl").args(["-sS", "--retry", "3", "--retry-delay", "30", "-A", AGENT, &url]).output().map_err(|e| format!("curl: {e}"))?;
                let Ok(reply) = serde_json::from_slice::<Value>(&out.stdout) else {
                    log("  o Commons recusou (muitos pedidos?): tente de novo mais tarde");
                    break;
                };
                // O título volta normalizado (`_` vira espaço): os dois jeitos apontam para a URL.
                let normalized: HashMap<&str, &str> =
                    reply["query"]["normalized"].as_array().into_iter().flatten().filter_map(|n| Some((n["to"].as_str()?, n["from"].as_str()?))).collect();
                for page in reply["query"]["pages"].as_array().into_iter().flatten() {
                    let (Some(title), Some(thumb)) = (page["title"].as_str(), page["imageinfo"][0]["thumburl"].as_str()) else { continue };
                    thumbs.insert(format!("{width}|{title}"), Value::String(thumb.to_string()));
                    if let Some(from) = normalized.get(title) {
                        thumbs.insert(format!("{width}|{from}"), Value::String(thumb.to_string()));
                    }
                }
                write(&thumbs_file, &Value::Object(thumbs.clone()))?;
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
        // 2. As imagens: um curl, uma por segundo; a extensão sai do conteúdo.
        let mut config = String::new();
        let mut pending = Vec::new();
        for (key, title, folder, width) in &missing {
            let Some(url) = thumbs.get(&format!("{width}|{title}")).and_then(Value::as_str) else { continue };
            let folder_dir = dir.join("bandeiras").join(folder);
            std::fs::create_dir_all(&folder_dir).map_err(|e| e.to_string())?;
            let tmp = folder_dir.join(format!("{key}.tmp"));
            config.push_str(&format!("url = \"{url}\"\noutput = \"{}\"\n", tmp.display().to_string().replace('\\', "/")));
            pending.push(tmp);
        }
        if !pending.is_empty() {
            log(&format!("  baixando {} imagens (uma por segundo)", pending.len()));
            let list = cache.join("bandeiras.curl");
            std::fs::write(&list, config).map_err(|e| e.to_string())?;
            let _ = Command::new("curl").args(["-sSL", "--retry", "3", "--retry-delay", "30", "--fail", "--rate", "1/s", "-A", AGENT, "-K"]).arg(&list).status();
            let _ = std::fs::remove_file(&list);
            for tmp in pending {
                let Ok(bytes) = std::fs::read(&tmp) else { continue };
                let ext = if bytes.starts_with(b"\x89PNG") {
                    "png"
                } else if bytes.starts_with(&[0xFF, 0xD8]) {
                    "jpg"
                } else {
                    let _ = std::fs::remove_file(&tmp);
                    continue;
                };
                let _ = std::fs::rename(&tmp, tmp.with_extension(ext));
            }
        }
    }
    credits(dir, cache, &wanted, &saved, offline, log)?;
    Ok(wanted
        .iter()
        .filter_map(|(key, _, folder, _)| {
            let f = saved(key, folder)?;
            Some((key.clone(), f.strip_prefix(dir).unwrap_or(&f).to_string_lossy().replace('\\', "/")))
        })
        .collect())
}

/// Grava `bandeiras/CREDITOS.md`: de cada imagem, o arquivo de origem no Commons, o autor e a
/// licença. Algumas bandeiras são CC BY-SA, que pede o crédito junto da imagem redistribuída.
/// Autor e licença vêm da API do Commons (50 por consulta, uma a cada 2 s), guardados no cache
/// (`wikidata/licencas.json`).
fn credits(
    dir: &Path,
    cache: &Path,
    wanted: &[(String, String, &str, u32)],
    saved: &dyn Fn(&str, &str) -> Option<PathBuf>,
    offline: bool,
    log: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let present: Vec<&(String, String, &str, u32)> = wanted.iter().filter(|(k, _, folder, _)| saved(k, folder).is_some()).collect();
    let file = cache.join("wikidata/licencas.json");
    let mut known: Map<String, Value> = std::fs::read(&file).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    let mut titles: Vec<&str> = present.iter().map(|(_, t, _, _)| t.as_str()).filter(|t| !known.contains_key(*t)).collect();
    titles.sort_unstable();
    titles.dedup();
    if !titles.is_empty() && !offline {
        log(&format!("  autor e licença de {} bandeiras", titles.len()));
        for chunk in titles.chunks(50) {
            let url = format!(
                "https://commons.wikimedia.org/w/api.php?action=query&format=json&formatversion=2&prop=imageinfo&iiprop=extmetadata&iiextmetadatafilter=Artist|LicenseShortName&titles={}",
                encode(&chunk.join("|"))
            );
            let out = Command::new("curl").args(["-sS", "--retry", "3", "--retry-delay", "30", "-A", AGENT, &url]).output().map_err(|e| format!("curl: {e}"))?;
            let Ok(reply) = serde_json::from_slice::<Value>(&out.stdout) else {
                log("  o Commons recusou (muitos pedidos?): tente de novo mais tarde");
                break;
            };
            let normalized: HashMap<&str, &str> =
                reply["query"]["normalized"].as_array().into_iter().flatten().filter_map(|n| Some((n["to"].as_str()?, n["from"].as_str()?))).collect();
            for page in reply["query"]["pages"].as_array().into_iter().flatten() {
                let Some(title) = page["title"].as_str() else { continue };
                let meta = &page["imageinfo"][0]["extmetadata"];
                let info = json!({
                    "artist": strip_tags(meta["Artist"]["value"].as_str().unwrap_or_default()),
                    "license": meta["LicenseShortName"]["value"].as_str().unwrap_or_default(),
                });
                known.insert(title.to_string(), info.clone());
                if let Some(from) = normalized.get(title) {
                    known.insert(from.to_string(), info);
                }
            }
            write(&file, &Value::Object(known.clone()))?;
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
    let mut text = String::from(
        "# Créditos das bandeiras\n\n\
         As bandeiras dos estados e dos municípios vêm do [Wikimedia Commons](https://commons.wikimedia.org),\n\
         achadas pelo Wikidata. Cada uma tem o autor e a licença do arquivo original, na página dele\n\
         no Commons (o link abaixo). As imagens daqui são miniaturas, sem outra alteração.\n\n\
         | Imagem | Original no Commons | Autor | Licença |\n|---|---|---|---|\n",
    );
    let cell = |s: &str| s.replace('|', "\\|").replace('\n', " ");
    for (key, title, folder, _) in present {
        let image = saved(key, folder).and_then(|f| f.strip_prefix(dir.join("bandeiras")).ok().map(|p| p.to_string_lossy().replace('\\', "/"))).unwrap_or_default();
        let name = title.trim_start_matches("File:");
        let link = format!("https://commons.wikimedia.org/wiki/File:{}", encode(&name.replace(' ', "_")).replace("%2F", "/"));
        let info = known.get(title.as_str());
        let field = |k: &str| info.and_then(|i| i[k].as_str()).filter(|s| !s.is_empty()).map(cell).unwrap_or_else(|| "veja o original".to_string());
        text.push_str(&format!("| {image} | [{}]({link}) | {} | {} |\n", cell(name), field("artist"), field("license")));
    }
    std::fs::write(dir.join("bandeiras/CREDITOS.md"), text).map_err(|e| e.to_string())
}

/// Tira as marcas de HTML (o autor no Commons costuma vir como link) e junta os espaços.
fn strip_tags(html: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for c in html.chars() {
        match c {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => out.push(c),
            _ => {}
        }
    }
    out.replace("&amp;", "&").replace("&quot;", "\"").replace("&#039;", "'").split_whitespace().collect::<Vec<_>>().join(" ")
}

/// O contrário de [`encode`]: `%C3%A9` → `é`.
fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn bindings(v: &Value) -> Vec<&Value> {
    v["results"]["bindings"].as_array().map(|a| a.iter().collect()).unwrap_or_default()
}

/// Codifica para a URL (tudo que não é letra, número ou `-_.~`).
fn encode(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Mapas

/// O GeoJSON da malha vira SVG: um `<path>` por área, com `id` = `prefix` + código. Projeção
/// simples (longitude encolhida pelo cosseno da latitude do meio), 1000 de largura, pontos
/// repetidos (depois de arredondar) fora. As cores seguem o tema (`currentColor`).
pub fn mesh_to_svg(mesh: &Value, prefix: &str) -> String {
    let features: Vec<&Value> = mesh["features"].as_array().map(|a| a.iter().collect()).unwrap_or_default();
    let rings = |f: &Value| -> Vec<Vec<(f64, f64)>> {
        let g = &f["geometry"];
        let polys: Vec<&Value> = match g["type"].as_str() {
            Some("Polygon") => vec![&g["coordinates"]],
            Some("MultiPolygon") => g["coordinates"].as_array().map(|a| a.iter().collect()).unwrap_or_default(),
            _ => Vec::new(),
        };
        polys
            .iter()
            .flat_map(|p| p.as_array().into_iter().flatten())
            .map(|ring| ring.as_array().into_iter().flatten().filter_map(|pt| Some((pt[0].as_f64()?, pt[1].as_f64()?))).collect())
            .collect()
    };
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for f in &features {
        for r in rings(f) {
            for (x, y) in r {
                (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
            }
        }
    }
    if x0 > x1 {
        return "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1 1\"/>".into();
    }
    let k_lon = ((y0 + y1) / 2.0).to_radians().cos();
    let scale = 1000.0 / ((x1 - x0) * k_lon).max(1e-9);
    let height = ((y1 - y0) * scale).ceil();
    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1000 {height}\">\n<g fill=\"currentColor\" fill-opacity=\"0.10\" stroke=\"currentColor\" stroke-opacity=\"0.55\" stroke-width=\"1\" stroke-linejoin=\"round\" vector-effect=\"non-scaling-stroke\">\n"
    );
    for f in &features {
        let code = f["properties"]["codarea"].as_str().unwrap_or_default();
        let mut d = String::new();
        for r in rings(f) {
            let mut last = (i64::MIN, i64::MIN);
            let mut n = 0;
            for (x, y) in r {
                // Décimos de unidade (o mapa tem 1000 de largura).
                let p = (((x - x0) * k_lon * scale * 10.0).round() as i64, ((y1 - y) * scale * 10.0).round() as i64);
                if p == last {
                    continue;
                }
                d.push_str(if n == 0 { "M" } else { "L" });
                d.push_str(&format!("{} {}", p.0 as f64 / 10.0, p.1 as f64 / 10.0));
                last = p;
                n += 1;
            }
            if n > 0 {
                d.push('Z');
            }
        }
        out.push_str(&format!("<path id=\"{prefix}{code}\" d=\"{d}\"/>\n"));
    }
    out.push_str("</g>\n</svg>\n");
    out
}
