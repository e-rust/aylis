//! O fuso do projeto (`time_zone` no `.aylisproj`): no lugar do fuso do aparelho, todos veem a
//! hora da empresa. Com a feature `time-zones` (a tabela de fusos IANA, que a IDE liga quando
//! o projeto tem `time_zone`).

/// Fixa o fuso do app pelo nome (`"America/Sao_Paulo"`); vazio deixa o do aparelho.
pub(crate) fn install(name: &str) {
    if name.is_empty() {
        return;
    }
    #[cfg(feature = "time-zones")]
    match name.parse::<chrono_tz::Tz>() {
        Ok(tz) => {
            let _ = ZONE.set(tz);
            aylis_core::platform::set_project_offset_fn(Some(offset));
        }
        Err(_) => aylis_core::stderr_line!("aylis: fuso '{name}' desconhecido (use um nome IANA, como America/Sao_Paulo); fica o do aparelho"),
    }
    #[cfg(not(feature = "time-zones"))]
    aylis_core::stderr_line!("aylis: o projeto pede o fuso '{name}', mas o app foi compilado sem a feature time-zones; fica o do aparelho");
}

#[cfg(feature = "time-zones")]
static ZONE: std::sync::OnceLock<chrono_tz::Tz> = std::sync::OnceLock::new();

/// Segundos que o fuso do projeto está à frente do UTC no instante `utc` (com o horário de
/// verão da época).
#[cfg(feature = "time-zones")]
fn offset(utc: i64) -> i64 {
    use chrono::Offset;
    use chrono::TimeZone;
    let Some(tz) = ZONE.get() else { return 0 };
    match chrono::DateTime::from_timestamp(utc, 0) {
        Some(at) => tz.offset_from_utc_datetime(&at.naive_utc()).fix().local_minus_utc() as i64,
        None => 0,
    }
}

#[cfg(all(test, feature = "time-zones"))]
mod tests {
    #[test]
    fn the_project_time_zone_wins_over_the_device() {
        aylis_core::platform::set_local_offset_fn(|_| 0);
        super::install("America/Manaus");
        // 2026-11-02T13:00:00Z: em Manaus (UTC−4), 09:00.
        assert_eq!(aylis_core::platform::local_offset(1_793_624_400), -4 * 3600);
    }
}
