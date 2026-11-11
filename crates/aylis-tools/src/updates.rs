//! Há versão nova do Aylis no GitHub Releases? (como o NoOBS faz)
//!
//! `GET https://api.github.com/repos/e-rust/aylis/releases?per_page=30`, sem autenticação
//! (60 pedidos por hora por IP: folgado para um por dia). A IDE consulta uma vez por dia, depois
//! das 6 h locais ([`due`]), e só marca o dia como consultado quando o GitHub respondeu: sem
//! rede, tenta de novo mais tarde. Falha é silenciosa por desenho: pior que não avisar de uma
//! versão nova é avisar errado.

/// O repositório das releases (dono/nome).
pub const REPOSITORY: &str = "e-rust/aylis";

/// A hora local a partir da qual a consulta do dia acontece (release raramente sai no meio do
/// dia; com uma janela de 24 h deslizante, o horário escorregava para quando a IDE abriu).
pub const CHECK_HOUR: i64 = 6;

/// A página das releases (o "Abrir página" quando a resposta não traz a da versão).
pub fn releases_url() -> String {
    format!("https://github.com/{REPOSITORY}/releases")
}

/// Uma release publicada.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub tag: String,
    pub prerelease: bool,
    pub url: String,
}

/// A resposta do GitHub.
#[derive(Clone, Debug, Default)]
pub struct Fetched {
    /// O código HTTP; 0 = não houve resposta (rede, DNS, prazo). Com qualquer resposta, até
    /// 403 ou 500, a consulta do dia "aconteceu"; sem resposta, tenta de novo.
    pub status: u16,
    pub releases: Vec<Release>,
    /// O motivo da falha (só para o log).
    pub error: String,
}

/// A versão de uma tag (`v0.1.0`, `0.2.0-beta.1`): os três números e a parte de pré-release.
pub fn parse_version(tag: &str) -> Option<((u64, u64, u64), String)> {
    let tag = tag.trim().trim_start_matches(['v', 'V']);
    let (numbers, pre) = tag.split_once('-').map_or((tag, ""), |(n, p)| (n, p));
    let numbers = numbers.split('+').next()?;
    let mut parts = numbers.split('.').map(|p| p.parse::<u64>().ok());
    let v = (parts.next()??, parts.next().flatten().unwrap_or(0), parts.next().flatten().unwrap_or(0));
    Some((v, pre.to_string()))
}

/// `a` é mais nova que `b`? Mesmos números: a final é mais nova que a pré-release.
pub fn is_newer(a: &str, b: &str) -> bool {
    let (Some((va, pa)), Some((vb, pb))) = (parse_version(a), parse_version(b)) else { return false };
    match va.cmp(&vb) {
        std::cmp::Ordering::Equal => match (pa.is_empty(), pb.is_empty()) {
            (true, false) => true,
            (false, false) => compare_pre(&pa, &pb) == std::cmp::Ordering::Greater,
            _ => false,
        },
        order => order == std::cmp::Ordering::Greater,
    }
}

/// Compara as partes de pré-release como o semver: por ponto, números como números
/// (`beta.10` depois de `beta.9`) e texto como texto.
fn compare_pre(a: &str, b: &str) -> std::cmp::Ordering {
    let (mut a, mut b) = (a.split('.'), b.split('.'));
    loop {
        let order = match (a.next(), b.next()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(x), Some(y)) => match (x.parse::<u64>(), y.parse::<u64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                _ => x.cmp(y),
            },
        };
        if order != std::cmp::Ordering::Equal {
            return order;
        }
    }
}

/// A release a oferecer para quem está em `current`: a mais nova que ela. Quem está numa
/// versão final só vê as finais; quem está numa pré-release vê as duas.
pub fn pick_update<'a>(current: &str, releases: &'a [Release]) -> Option<&'a Release> {
    let on_prerelease = parse_version(current).is_some_and(|(_, pre)| !pre.is_empty());
    releases
        .iter()
        .filter(|r| on_prerelease || !r.prerelease)
        .filter(|r| is_newer(&r.tag, current))
        .fold(None, |best: Option<&Release>, r| match best {
            Some(b) if !is_newer(&r.tag, &b.tag) => Some(b),
            _ => Some(r),
        })
}

/// Está na hora da consulta do dia? `last` e `now` em segundos UTC; `offset`, os segundos que a
/// hora local está à frente do UTC. O marco é o último [`CHECK_HOUR`] local que já passou (hoje,
/// ou ontem antes dele); é hora se a última consulta foi antes do marco.
pub fn due(last: i64, now: i64, offset: i64) -> bool {
    last < mark(now, offset)
}

/// Segundos até a próxima consulta (0 se já é hora).
pub fn seconds_until_due(last: i64, now: i64, offset: i64) -> i64 {
    if due(last, now, offset) { 0 } else { (mark(now, offset) + 86_400 - now).max(1) }
}

/// O último marco ([`CHECK_HOUR`] local) que já passou, em segundos UTC.
fn mark(now: i64, offset: i64) -> i64 {
    let local = now + offset;
    let today = local - local.rem_euclid(86_400) + CHECK_HOUR * 3600;
    let mark_local = if local < today { today - 86_400 } else { today };
    mark_local - offset
}

/// As releases a partir do JSON da API (rascunhos fora).
pub fn parse_releases(json: &serde_json::Value) -> Vec<Release> {
    json.as_array()
        .map(|list| {
            list.iter()
                .filter(|r| !r["draft"].as_bool().unwrap_or(false))
                .filter_map(|r| {
                    let tag = r["tag_name"].as_str()?.trim().to_string();
                    (!tag.is_empty()).then(|| Release { tag, prerelease: r["prerelease"].as_bool().unwrap_or(false), url: r["html_url"].as_str().unwrap_or("").to_string() })
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Consulta o GitHub (bloqueia: chame numa thread).
#[cfg(not(target_arch = "wasm32"))]
pub fn fetch() -> Fetched {
    use std::time::Duration;
    let url = format!("https://api.github.com/repos/{REPOSITORY}/releases?per_page=30");
    let agent: ureq::Agent = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(15))).http_status_as_error(false).build().into();
    // A API recusa (403) pedido sem User-Agent.
    let response = agent.get(&url).header("User-Agent", "aylis-ide-update-check").header("Accept", "application/vnd.github+json").call();
    let mut response = match response {
        Ok(r) => r,
        Err(e) => return Fetched { error: e.to_string(), ..Default::default() },
    };
    let status = response.status().as_u16();
    if status != 200 {
        // 404: ainda sem release; 403: o limite de pedidos.
        return Fetched { status, error: format!("HTTP {status}"), ..Default::default() };
    }
    let text = response.body_mut().read_to_string().map_err(|e| e.to_string());
    match text.and_then(|t| serde_json::from_str::<serde_json::Value>(&t).map_err(|e| e.to_string())) {
        Ok(json) => Fetched { status, releases: parse_releases(&json), error: String::new() },
        Err(e) => Fetched { status, error: format!("resposta inválida: {e}"), ..Default::default() },
    }
}

#[cfg(target_arch = "wasm32")]
pub fn fetch() -> Fetched {
    Fetched { error: "o navegador não consulta as releases".into(), ..Default::default() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(tag: &str, prerelease: bool) -> Release {
        Release { tag: tag.into(), prerelease, url: format!("https://x/{tag}") }
    }

    #[test]
    fn the_newest_release_is_offered_only_when_newer() {
        let list = [release("v0.3.0-beta.1", true), release("v0.2.0", false), release("v0.1.0", false)];
        assert_eq!(pick_update("0.1.0", &list).map(|r| r.tag.as_str()), Some("v0.2.0"), "a final ignora a pré-release");
        assert_eq!(pick_update("0.2.0", &list), None, "já está na mais nova");
        assert_eq!(pick_update("0.2.1-beta.1", &list), Some(&list[0]), "a pré-release vê as pré-releases");
        assert!(is_newer("v0.10.0", "0.9.9"), "compara os números, não o texto");
        assert!(is_newer("1.0.0", "1.0.0-rc.1"));
        assert!(!is_newer("1.0.0-rc.1", "1.0.0"));
        assert!(is_newer("0.2.0-beta.10", "0.2.0-beta.9"), "beta.10 depois de beta.9");
        assert!(is_newer("0.2.0-rc.1", "0.2.0-beta.3"));
    }

    #[test]
    fn the_check_happens_once_a_day_after_six() {
        let offset = -3 * 3600; // Brasília
        let day = 1_794_268_800; // 2026-11-10 00:00 UTC
        let local = |h: i64| day + h * 3600 - offset; // h horas locais do dia 10
        // A última foi às 7 h de ontem: às 5 h de hoje ainda não, às 6 h sim.
        let yesterday_7 = local(7) - 86_400;
        assert!(!due(yesterday_7, local(5), offset));
        assert!(due(yesterday_7, local(6), offset));
        // Feita às 6 h 30, a próxima é amanhã às 6 h.
        assert!(!due(local(6) + 1800, local(23), offset));
        assert_eq!(seconds_until_due(local(6) + 1800, local(23), offset), 7 * 3600);
        // Nunca consultou: já é hora.
        assert!(due(0, local(1), offset));
    }

    /// A consulta de verdade (precisa de rede): `cargo test -p aylis-tools --lib -- --ignored
    /// the_releases_come_from_github`.
    #[test]
    #[ignore]
    fn the_releases_come_from_github() {
        let fetched = fetch();
        assert_eq!(fetched.status, 200, "{}", fetched.error);
        assert!(fetched.releases.iter().any(|r| r.tag == "v0.1.0"), "{:?}", fetched.releases);
    }

    #[test]
    fn drafts_and_empty_tags_are_left_out() {
        let json = serde_json::json!([
            {"tag_name": "v0.2.0", "draft": true, "prerelease": false, "html_url": "a"},
            {"tag_name": "", "draft": false},
            {"tag_name": "v0.1.0", "draft": false, "prerelease": false, "html_url": "https://github.com/e-rust/aylis/releases/tag/v0.1.0"}
        ]);
        assert_eq!(parse_releases(&json), vec![Release { tag: "v0.1.0".into(), prerelease: false, url: "https://github.com/e-rust/aylis/releases/tag/v0.1.0".into() }]);
    }
}
