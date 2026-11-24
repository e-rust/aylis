//! O HTTP do Aylis fora do navegador: o `ureq` (rustls, as raízes do Mozilla embutidas) em
//! quatro threads de trabalho, criadas no primeiro pedido e paradas no `recv` depois (parado
//! não gasta). Os redirecionamentos são seguidos aqui, um a um, para conferir cada endereço:
//! sem descer de `https` para `http`, e sem levar `Authorization`/`Cookie` para outro host.

use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use aylis_core::providers::{HttpDone, HttpRequest, HttpResponse};

const WORKERS: usize = 4;

/// O `User-Agent` padrão.
const USER_AGENT: &str = concat!("aylis/", env!("CARGO_PKG_VERSION"));

type Job = (HttpRequest, HttpDone);

fn queue() -> &'static Mutex<Sender<Job>> {
    static QUEUE: OnceLock<Mutex<Sender<Job>>> = OnceLock::new();
    QUEUE.get_or_init(|| {
        let (tx, rx) = channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..WORKERS {
            let rx = rx.clone();
            let _ = std::thread::Builder::new().name(format!("aylis-http-client-{i}")).spawn(move || worker(&rx));
        }
        Mutex::new(tx)
    })
}

fn worker(rx: &Mutex<Receiver<Job>>) {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .user_agent(USER_AGENT)
        // O status volta como resposta (um 404 não é falha de transporte).
        .http_status_as_error(false)
        // Os redirecionamentos são seguidos em `run`, conferidos um a um.
        .max_redirects(0)
        .build()
        .into();
    loop {
        let job = match rx.lock() {
            Ok(r) => r.recv(),
            Err(_) => return,
        };
        let Ok((request, done)) = job else { return };
        done(run(&agent, request));
    }
}

/// Põe o pedido na fila das threads de trabalho.
pub(super) fn send(request: HttpRequest, done: HttpDone) {
    let job = (request, done);
    let (_, done) = match queue().lock() {
        Ok(q) => match q.send(job) {
            Ok(()) => return,
            Err(e) => e.0,
        },
        Err(_) => job,
    };
    done(Err("HTTP: sem threads de trabalho".into()));
}

/// O pedido, com os redirecionamentos e os limites.
pub(crate) fn run(agent: &ureq::Agent, mut request: HttpRequest) -> Result<HttpResponse, String> {
    let deadline = (request.timeout_ms > 0).then(|| Instant::now() + Duration::from_millis(u64::from(request.timeout_ms)));
    let limit = request.response_limit();
    let mut url = request.url.trim().to_string();
    let mut redirects = 0;
    loop {
        request.check_url(&url)?;
        let remaining = match deadline {
            Some(d) => {
                let left = d.saturating_duration_since(Instant::now());
                if left.is_zero() {
                    return Err(format!("HTTP: prazo esgotado ({} ms)", request.timeout_ms));
                }
                Some(left)
            }
            None => None,
        };
        let mut builder = ureq::http::Request::builder().method(request.method.as_str()).uri(url.as_str());
        for (k, v) in &request.headers {
            builder = builder.header(k.as_str(), v.as_str());
        }
        let response = if request.body.is_empty() {
            let r = builder.body(()).map_err(|e| format!("HTTP: pedido inválido: {e}"))?;
            agent.run(agent.configure_request(r).timeout_global(remaining).build())
        } else {
            let r = builder.body(request.body.clone()).map_err(|e| format!("HTTP: pedido inválido: {e}"))?;
            agent.run(agent.configure_request(r).timeout_global(remaining).build())
        }
        .map_err(|e| describe(&e, &request))?;
        let status = response.status().as_u16();
        let location = response.headers().get("location").and_then(|v| v.to_str().ok()).map(str::to_string);
        if (300..400).contains(&status) && status != 304 && redirects < request.max_redirects {
            if let Some(location) = location {
                redirects += 1;
                let next = join(&url, &location);
                if url.to_ascii_lowercase().starts_with("https:") && next.to_ascii_lowercase().starts_with("http:") && !request.allow_http {
                    return Err(format!("HTTP: redirecionamento de https para http recusado ({next})"));
                }
                if aylis_core::providers::url_host(&next) != aylis_core::providers::url_host(&url) {
                    // Outro host não recebe as credenciais deste.
                    request.headers.retain(|(k, _)| !matches!(k.to_ascii_lowercase().as_str(), "authorization" | "cookie" | "proxy-authorization"));
                }
                if status == 303 || (matches!(status, 301 | 302) && request.method == "POST") {
                    // Como os navegadores: o 303 (e o 301/302 de um POST) segue com GET, sem corpo.
                    request.method = "GET".into();
                    request.body.clear();
                    request.headers.retain(|(k, _)| !matches!(k.to_ascii_lowercase().as_str(), "content-type" | "content-length"));
                }
                url = next;
                continue;
            }
        }
        let headers = response.headers().iter().map(|(k, v)| (k.as_str().to_ascii_lowercase(), String::from_utf8_lossy(v.as_bytes()).into_owned())).collect();
        let body = response.into_body().into_with_config().limit(limit).read_to_vec().map_err(|e| match e {
            ureq::Error::BodyExceedsLimit(_) => format!("HTTP: resposta maior que o limite ({limit} bytes)"),
            other => describe(&other, &request),
        })?;
        return Ok(HttpResponse { status, headers, body, url });
    }
}

fn describe(e: &ureq::Error, request: &HttpRequest) -> String {
    match e {
        ureq::Error::Timeout(_) => format!("HTTP: prazo esgotado ({} ms)", request.timeout_ms),
        other => format!("HTTP: {other}"),
    }
}

/// O endereço de um `Location` (absoluto, do mesmo host ou relativo ao caminho atual).
pub(crate) fn join(base: &str, location: &str) -> String {
    let location = location.trim();
    if location.contains("://") {
        return location.to_string();
    }
    let (scheme, rest) = base.split_once("://").unwrap_or(("https", base));
    if let Some(net) = location.strip_prefix("//") {
        return format!("{scheme}://{net}");
    }
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let (authority, path) = rest.split_at(authority_end);
    if location.starts_with('/') {
        return format!("{scheme}://{authority}{location}");
    }
    let path = path.split(['?', '#']).next().unwrap_or_default();
    let dir = path.rfind('/').map_or("/", |i| &path[..=i]);
    format!("{scheme}://{authority}{dir}{location}")
}

#[cfg(test)]
mod tests {
    use super::join;

    #[test]
    fn location_joins_like_a_browser() {
        assert_eq!(join("https://a.com/x/y?q=1", "https://b.com/z"), "https://b.com/z");
        assert_eq!(join("https://a.com/x/y?q=1", "/z"), "https://a.com/z");
        assert_eq!(join("https://a.com/x/y?q=1", "z"), "https://a.com/x/z");
        assert_eq!(join("https://a.com", "z"), "https://a.com/z");
        assert_eq!(join("http://a.com:8080/x/", "//c.com/w"), "http://c.com/w");
    }
}
