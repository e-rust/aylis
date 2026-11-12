//! Som por endereço (`http://`, `https://`): o arquivo é lido aos poucos, com `Range`, e
//! decodificado pelo Symphonia como um arquivo do disco (o relatório do Tabernáculo, W11).
//!
//! A busca (`seek`) vira um pedido novo a partir do byte certo; um servidor que não aceita
//! `Range` (responde 200 em vez de 206) ainda toca, mas voltar atrás baixa de novo do começo.

use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;
use std::time::Duration;

/// Avançar até tanto lendo o que já está chegando sai mais barato que um pedido novo.
const SKIP_READING: u64 = 256 * 1024;

/// Um arquivo na rede lido como um arquivo local (`Read` + `Seek`), para o Symphonia.
pub struct HttpSource {
    url: String,
    agent: ureq::Agent,
    /// O tamanho total, se o servidor disse.
    len: Option<u64>,
    /// O servidor aceita `Range`.
    ranges: bool,
    /// Onde a próxima leitura começa.
    pos: u64,
    /// A resposta aberta e o byte em que ela está (`None`: abre no próximo `read`).
    body: Mutex<Option<(ureq::BodyReader<'static>, u64)>>,
}

impl HttpSource {
    /// Abre o endereço (o primeiro pedido já diz o tamanho e se aceita `Range`).
    pub fn open(url: &str) -> Result<Self, String> {
        let agent: ureq::Agent = ureq::Agent::config_builder().timeout_connect(Some(Duration::from_secs(10))).timeout_per_read(Some(Duration::from_secs(30))).build().into();
        let mut this = Self { url: url.to_string(), agent, len: None, ranges: false, pos: 0, body: Mutex::new(None) };
        let (body, start) = this.request(0)?;
        this.body = Mutex::new(Some((body, start)));
        Ok(this)
    }

    /// Pede o arquivo a partir de `from`. Devolve a resposta e o byte em que ela começa (0
    /// se o servidor ignorou o `Range`).
    fn request(&mut self, from: u64) -> Result<(ureq::BodyReader<'static>, u64), String> {
        let response = self
            .agent
            .get(&self.url)
            .header("Range", format!("bytes={from}-"))
            // Sem compressão: os bytes têm de ser os do arquivo, para o `Range` valer.
            .header("Accept-Encoding", "identity")
            .call()
            .map_err(|e| format!("{}: {e}", self.url))?;
        let status = response.status().as_u16();
        let header = |name: &str| response.headers().get(name).and_then(|v| v.to_str().ok()).map(str::to_string);
        let start = match status {
            206 => {
                self.ranges = true;
                // `bytes 100-199/1234`
                let range = header("content-range").unwrap_or_default();
                if let Some(total) = range.rsplit('/').next().and_then(|t| t.trim().parse::<u64>().ok()) {
                    self.len = Some(total);
                }
                range.split_whitespace().nth(1).and_then(|r| r.split('-').next()).and_then(|s| s.parse::<u64>().ok()).unwrap_or(from)
            }
            200 => {
                self.ranges = false;
                if let Some(len) = header("content-length").and_then(|l| l.parse::<u64>().ok()) {
                    self.len = Some(len);
                }
                0
            }
            // O pedido passou do fim (416): nada mais a ler.
            416 => return Err("fim do arquivo".into()),
            _ => return Err(format!("{}: HTTP {status}", self.url)),
        };
        Ok((response.into_body().into_reader(), start))
    }
}

impl Read for HttpSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() || self.len.is_some_and(|l| self.pos >= l) {
            return Ok(0);
        }
        let mut current = self.body.lock().map_err(|_| std::io::Error::other("HTTP"))?.take();
        // A resposta aberta está atrás de `pos` (perto): lê e descarta até lá.
        if let Some((body, at)) = &mut current {
            if *at < self.pos && (self.pos - *at <= SKIP_READING || !self.ranges) {
                let skip = self.pos - *at;
                let skipped = std::io::copy(&mut body.by_ref().take(skip), &mut std::io::sink())?;
                *at += skipped;
            }
            if *at != self.pos {
                current = None;
            }
        }
        let (mut body, mut at) = match current {
            Some(c) => c,
            None => {
                let (mut body, start) = self.request(self.pos).map_err(std::io::Error::other)?;
                // Sem `Range`, a resposta vem do começo: descarta até `pos`.
                let skipped = std::io::copy(&mut body.by_ref().take(self.pos - start.min(self.pos)), &mut std::io::sink())?;
                (body, start + skipped)
            }
        };
        let n = match body.read(buf) {
            Ok(n) => n,
            // A conexão caiu (o servidor fecha a que ficou parada): pede de novo daqui.
            Err(_) => {
                let (fresh, start) = self.request(at).map_err(std::io::Error::other)?;
                body = fresh;
                at = start;
                if at != self.pos {
                    return Err(std::io::Error::other("o servidor não continuou de onde parou"));
                }
                body.read(buf)?
            }
        };
        at += n as u64;
        self.pos = at;
        if let Ok(mut b) = self.body.lock() {
            *b = Some((body, at));
        }
        Ok(n)
    }
}

impl Seek for HttpSource {
    fn seek(&mut self, to: SeekFrom) -> std::io::Result<u64> {
        let target = match to {
            SeekFrom::Start(p) => Some(p),
            SeekFrom::Current(d) => self.pos.checked_add_signed(d),
            SeekFrom::End(d) => self.len.and_then(|l| l.checked_add_signed(d)),
        };
        let target = target.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "posição fora do arquivo"))?;
        // A resposta aberta só serve para frente; o `read` decide se lê até lá ou pede de novo.
        if target < self.pos {
            if let Ok(mut b) = self.body.lock() {
                *b = None;
            }
        }
        self.pos = target;
        Ok(target)
    }
}

impl symphonia::core::io::MediaSource for HttpSource {
    fn is_seekable(&self) -> bool {
        // Sem `Range` também volta (baixando de novo do começo): o MP4 precisa do índice do fim.
        true
    }

    fn byte_len(&self) -> Option<u64> {
        self.len
    }
}
