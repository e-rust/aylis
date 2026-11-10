//! A lógica sem tela do app: roda no serviço em segundo plano (o componente
//! BackgroundService), numa thread que continua com a tela fechada. Gerado pela IDE do aylis
//! (o corpo é seu).
//!
//! O teste: conta de 2 em 2 segundos e manda o número à tela; com a tela fechada, os números
//! esperam e chegam todos quando ela volta.

use std::time::Duration;

/// Conta enquanto o serviço está ligado; responde o que a tela mandar.
pub fn run(service: aylis::background::Service) {
    let mut count = 0u64;
    while !service.stopped() {
        if let Some(message) = service.recv(Duration::from_secs(2)) {
            service.send(format!("recebi: {message}"));
        }
        count += 1;
        service.send(format!("conta {count}"));
    }
}
