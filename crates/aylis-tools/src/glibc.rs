//! A glibc mínima dos executáveis do Linux (decidido com o autor em 22/11: a 2.35, a do
//! Ubuntu 22.04).
//!
//! Um executável do Linux só roda numa glibc igual ou mais nova que a de onde foi ligado: os
//! pacotes da 0.9.1, compilados no Ubuntu 26.04 do WSL (glibc 2.43), nem abriam no Mint 22
//! (2.39). Por isso o Aylis compila para o Linux num Ubuntu 22.04 (no Windows, a distribuição
//! `Ubuntu-22.04` do WSL; [`crate::wsl::distro`]) e confere o resultado aqui: as versões que o
//! executável pede ficam na seção `.gnu.version_r` do ELF (o que o `objdump -T` mostra).
//!
//! O `cargo zigbuild` (ligar para a glibc escolhida em qualquer máquina) foi tentado e não
//! serve: o Skia pré-compilado do `skia-safe` é feito contra a libstdc++ do GCC, e o zig liga
//! com a libc++ dele; a libstdc++ estática de um GCC novo pede a glibc nova.
//!
//! A 2.35 cobre o Ubuntu 22.04, o Mint 21, o Debian 12, o Fedora 36, o RHEL 10 e os mais novos.

use std::path::Path;

/// A glibc mínima (maior, menor).
pub const MINIMUM: (u32, u32) = (2, 35);

/// A libstdc++ que acompanha a glibc mínima (a do GCC 12, no Ubuntu 22.04): o que pedir mais
/// que ela também não abre lá.
pub const MINIMUM_GLIBCXX: (u32, u32, u32) = (3, 4, 30);

/// A distribuição do WSL que compila para o Linux: a glibc dela é a mínima.
pub const WSL_DISTRO: &str = "Ubuntu-22.04";

/// O texto da glibc mínima ("2.35").
pub fn minimum_text() -> String {
    format!("{}.{}", MINIMUM.0, MINIMUM.1)
}

/// A versão de uma glibc pela saída do `ldd --version` ("ldd (Ubuntu GLIBC 2.35-0ubuntu3) 2.35").
pub fn parse_ldd(text: &str) -> Option<(u32, u32)> {
    let first = text.lines().next()?;
    let last = first.split_whitespace().last()?;
    let mut parts = last.split('.');
    Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
}

/// A glibc de quem compila passa da mínima.
pub fn too_new(version: (u32, u32)) -> bool {
    version > MINIMUM
}

/// O que um executável pede às bibliotecas do sistema.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Requirements {
    /// A maior `GLIBC_x.y[.z]`.
    pub glibc: Option<Vec<u32>>,
    /// A maior `GLIBCXX_x.y.z` (a libstdc++ ligada dinamicamente).
    pub glibcxx: Option<Vec<u32>>,
    /// Pede a `GLIBC_PRIVATE` (só a mesma glibc de quem ligou tem).
    pub private: bool,
}

impl Requirements {
    /// Por que não roda na glibc mínima; `None` quando roda.
    pub fn problem(&self) -> Option<String> {
        let min = [MINIMUM.0, MINIMUM.1];
        let cxx = [MINIMUM_GLIBCXX.0, MINIMUM_GLIBCXX.1, MINIMUM_GLIBCXX.2];
        let mut problems = Vec::new();
        if let Some(v) = self.glibc.as_ref().filter(|v| v.as_slice() > min.as_slice()) {
            problems.push(format!("pede a glibc {} (o mínimo do Aylis é a {})", dotted(v), minimum_text()));
        }
        if let Some(v) = self.glibcxx.as_ref().filter(|v| v.as_slice() > cxx.as_slice()) {
            problems.push(format!("pede a libstdc++ {} (a do GCC 12 é a {})", dotted(v), dotted(&cxx)));
        }
        if self.private {
            problems.push("pede a GLIBC_PRIVATE (só roda na mesma glibc de quem compilou)".into());
        }
        (!problems.is_empty()).then(|| problems.join("; "))
    }
}

fn dotted(v: &[u32]) -> String {
    v.iter().map(u32::to_string).collect::<Vec<_>>().join(".")
}

/// Lê as versões pedidas de um ELF de 64 bits (little-endian). Um executável sem
/// `.gnu.version_r` (estático) não pede nada.
pub fn requirements(elf: &[u8]) -> Result<Requirements, String> {
    if elf.len() < 64 || &elf[..4] != b"\x7fELF" {
        return Err("não é um executável do Linux (ELF)".into());
    }
    if elf[4] != 2 || elf[5] != 1 {
        return Err("só o ELF de 64 bits little-endian é conferido".into());
    }
    let u16_at = |o: usize| elf.get(o..o + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let u32_at = |o: usize| elf.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
    let u64_at = |o: usize| elf.get(o..o + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap()));
    let bad = || "ELF cortado ou inválido".to_string();
    let shoff = u64_at(0x28).ok_or_else(bad)? as usize;
    let shentsize = u16_at(0x3A).ok_or_else(bad)? as usize;
    let shnum = u16_at(0x3C).ok_or_else(bad)? as usize;
    let mut out = Requirements::default();
    const SHT_GNU_VERNEED: u32 = 0x6fff_fffe;
    for i in 0..shnum {
        let sh = shoff + i * shentsize;
        if u32_at(sh + 4).ok_or_else(bad)? != SHT_GNU_VERNEED {
            continue;
        }
        let offset = u64_at(sh + 0x18).ok_or_else(bad)? as usize;
        let link = u32_at(sh + 0x28).ok_or_else(bad)? as usize;
        let count = u32_at(sh + 0x2C).ok_or_else(bad)? as usize;
        // A tabela de nomes (`.dynstr`) é a seção ligada.
        let strings = shoff + link * shentsize;
        let str_offset = u64_at(strings + 0x18).ok_or_else(bad)? as usize;
        let name_at = |o: usize| -> Option<&str> {
            let start = str_offset + o;
            let end = start + elf.get(start..)?.iter().position(|&b| b == 0)?;
            std::str::from_utf8(&elf[start..end]).ok()
        };
        let mut need = offset;
        for _ in 0..count {
            let aux_count = u16_at(need + 2).ok_or_else(bad)? as usize;
            let mut aux = need + u32_at(need + 8).ok_or_else(bad)? as usize;
            for _ in 0..aux_count {
                let name = name_at(u32_at(aux + 8).ok_or_else(bad)? as usize).ok_or_else(bad)?;
                out.note(name);
                let next = u32_at(aux + 12).ok_or_else(bad)? as usize;
                if next == 0 {
                    break;
                }
                aux += next;
            }
            let next = u32_at(need + 12).ok_or_else(bad)? as usize;
            if next == 0 {
                break;
            }
            need += next;
        }
    }
    Ok(out)
}

impl Requirements {
    fn note(&mut self, name: &str) {
        let parse = |rest: &str| rest.split('.').map(str::parse::<u32>).collect::<Result<Vec<_>, _>>().ok();
        if name == "GLIBC_PRIVATE" {
            self.private = true;
        } else if let Some(v) = name.strip_prefix("GLIBCXX_").and_then(parse) {
            if self.glibcxx.as_ref().is_none_or(|old| v > *old) {
                self.glibcxx = Some(v);
            }
        } else if let Some(v) = name.strip_prefix("GLIBC_").and_then(parse)
            && self.glibc.as_ref().is_none_or(|old| v > *old)
        {
            self.glibc = Some(v);
        }
    }
}

/// Confere um executável: o erro diz o que ele pede a mais que a glibc mínima.
pub fn check_file(path: &Path) -> Result<Requirements, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    check_bytes(&name, &bytes).map_err(|e| if bytes.starts_with(b"\x7fELF") { e } else { format!("{}: {e}", path.display()) })
}

/// Confere o conteúdo de um executável chamado `name`.
pub fn check_bytes(name: &str, bytes: &[u8]) -> Result<Requirements, String> {
    let req = requirements(bytes)?;
    match req.problem() {
        Some(p) => Err(format!(
            "{name} {p}: não abre no Ubuntu 22.04, no Mint 21 nem no Debian 12 (compile num Ubuntu 22.04: no Windows, o publicar do Aylis usa a distribuição {WSL_DISTRO} do WSL)"
        )),
        None => Ok(req),
    }
}

/// Como [`check_bytes`], mas o que não é um ELF inteiro passa (os testes empacotam bytes de
/// mentira, como os 4 bytes mágicos sozinhos).
pub fn check_if_elf(name: &str, bytes: &[u8]) -> Result<(), String> {
    if bytes.len() >= 64 && bytes.starts_with(b"\x7fELF") { check_bytes(name, bytes).map(|_| ()) } else { Ok(()) }
}

/// O que conferir de uma pasta: os ELF de primeiro nível e os de `bin`/`backend` (o executável
/// da tela, o backend compilado). Arquivo que não é ELF fica de fora.
pub fn check_dir(dir: &Path) -> Result<Vec<(String, Requirements)>, String> {
    let mut found = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let mut head = [0u8; 4];
            let is_elf = std::fs::File::open(&path).and_then(|mut f| std::io::Read::read_exact(&mut f, &mut head)).is_ok() && &head == b"\x7fELF";
            if is_elf {
                let req = check_file(&path)?;
                found.push((path.strip_prefix(dir).unwrap_or(&path).display().to_string(), req));
            }
        }
    }
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Um ELF mínimo com uma `.dynstr` e uma `.gnu.version_r` pedindo `names` à `libc.so.6`.
    fn elf_with(names: &[&str]) -> Vec<u8> {
        let mut strtab = vec![0u8];
        let file_name = strtab.len() as u32;
        strtab.extend_from_slice(b"libc.so.6\0");
        let offsets: Vec<u32> = names
            .iter()
            .map(|n| {
                let o = strtab.len() as u32;
                strtab.extend_from_slice(n.as_bytes());
                strtab.push(0);
                o
            })
            .collect();
        let mut verneed = Vec::new();
        verneed.extend_from_slice(&1u16.to_le_bytes()); // vn_version
        verneed.extend_from_slice(&(names.len() as u16).to_le_bytes());
        verneed.extend_from_slice(&file_name.to_le_bytes());
        verneed.extend_from_slice(&16u32.to_le_bytes()); // vn_aux
        verneed.extend_from_slice(&0u32.to_le_bytes()); // vn_next
        for (i, o) in offsets.iter().enumerate() {
            verneed.extend_from_slice(&0u32.to_le_bytes()); // hash
            verneed.extend_from_slice(&0u16.to_le_bytes());
            verneed.extend_from_slice(&0u16.to_le_bytes());
            verneed.extend_from_slice(&o.to_le_bytes());
            let next: u32 = if i + 1 < offsets.len() { 16 } else { 0 };
            verneed.extend_from_slice(&next.to_le_bytes());
        }
        let str_at = 64usize;
        let need_at = str_at + strtab.len();
        let sh_at = need_at + verneed.len();
        let mut elf = vec![0u8; 64];
        elf[..4].copy_from_slice(b"\x7fELF");
        elf[4] = 2;
        elf[5] = 1;
        elf[0x28..0x30].copy_from_slice(&(sh_at as u64).to_le_bytes());
        elf[0x3A..0x3C].copy_from_slice(&64u16.to_le_bytes());
        elf[0x3C..0x3E].copy_from_slice(&3u16.to_le_bytes());
        elf.extend_from_slice(&strtab);
        elf.extend_from_slice(&verneed);
        let section = |kind: u32, offset: usize, size: usize, link: u32, info: u32| {
            let mut s = vec![0u8; 64];
            s[4..8].copy_from_slice(&kind.to_le_bytes());
            s[0x18..0x20].copy_from_slice(&(offset as u64).to_le_bytes());
            s[0x20..0x28].copy_from_slice(&(size as u64).to_le_bytes());
            s[0x28..0x2C].copy_from_slice(&link.to_le_bytes());
            s[0x2C..0x30].copy_from_slice(&info.to_le_bytes());
            s
        };
        elf.extend(section(0, 0, 0, 0, 0));
        elf.extend(section(3, str_at, strtab.len(), 0, 0)); // SHT_STRTAB
        elf.extend(section(0x6fff_fffe, need_at, verneed.len(), 1, 1));
        elf
    }

    #[test]
    fn the_highest_glibc_asked_is_found() {
        let req = requirements(&elf_with(&["GLIBC_2.2.5", "GLIBC_2.17", "GLIBC_2.35", "GLIBC_2.3.4", "GLIBCXX_3.4.30"])).unwrap();
        assert_eq!(req.glibc, Some(vec![2, 35]));
        assert_eq!(req.problem(), None);
    }

    #[test]
    fn a_newer_glibc_or_libstdcxx_is_refused() {
        // O caso do Mint 22 (22/11): compilado no Ubuntu 26.04.
        let req = requirements(&elf_with(&["GLIBC_2.2.5", "GLIBC_2.43", "GLIBCXX_3.4.33"])).unwrap();
        let problem = req.problem().unwrap();
        assert!(problem.contains("glibc 2.43") && problem.contains("libstdc++ 3.4.33"), "{problem}");
        let private = requirements(&elf_with(&["GLIBC_PRIVATE"])).unwrap();
        assert!(private.problem().is_some());
    }

    #[test]
    fn what_is_not_elf_is_an_error_and_a_static_elf_asks_nothing() {
        assert!(requirements(b"MZ\x90\0 not an elf at all, just some bytes to pass sixty-four.....").is_err());
        let req = requirements(&elf_with(&[])).unwrap();
        assert_eq!(req, Requirements::default());
    }

    #[test]
    fn the_glibc_of_ldd_is_read_and_compared_with_the_minimum() {
        assert_eq!(parse_ldd("ldd (Ubuntu GLIBC 2.35-0ubuntu3.8) 2.35\nCopyright"), Some((2, 35)));
        assert_eq!(parse_ldd("ldd (GNU libc) 2.43"), Some((2, 43)));
        assert!(!too_new((2, 35)) && !too_new((2, 31)) && too_new((2, 39)));
    }
}
