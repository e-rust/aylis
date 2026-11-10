//! Roda o backend Python de exemplo pelo aylis-host (sem janela) e confere o resultado.
//! É pulado se o Python não estiver instalado.

use std::process::Command;

#[test]
fn python_backend_with_sqlite_round_trip() {
    if Command::new("python").arg("--version").output().is_err() {
        eprintln!("python não encontrado: teste pulado");
        return;
    }
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let png = std::env::temp_dir().join("aylis_host_python_test.png");
    let output = Command::new(env!("CARGO_BIN_EXE_aylis-host"))
        .current_dir(root)
        .args(["--png", png.to_str().unwrap(), "--", "python", "clients/python/registration_backend.py", "--selftest"])
        .env("PYTHONIOENCODING", "utf-8")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    // o backend gravou no SQLite: limite alterado, cliente novo com id do banco, exclusão recusada
    assert!(stderr.contains("SELFTEST limite=1234.50 novo_id=31 pendentes=1"), "{stderr}");
    assert!(stderr.contains("SELFTEST python delta 2 1 1 501 1"), "{stderr}");
    assert!(stderr.contains("SELFTEST decimal=Decimal('1234.50')"), "{stderr}");
    // O before_post do backend pergunta: o data post e o append_record esperam a resposta.
    assert!(stderr.contains("SELFTEST before_post nome=Novo Cliente Python"), "{stderr}");
    assert!(std::fs::metadata(&png).unwrap().len() > 10_000);
    let _ = std::fs::remove_file(png);
}

#[test]
fn a_python_backend_sends_from_other_threads_and_posts_to_the_main_one() {
    if Command::new("python").arg("--version").output().is_err() {
        eprintln!("python não encontrado: teste pulado");
        return;
    }
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/threads_backend.py");
    let png = std::env::temp_dir().join(format!("aylis_host_threads_{}.png", std::process::id()));
    let output = Command::new(env!("CARGO_BIN_EXE_aylis-host"))
        .args(["--png", png.to_str().unwrap(), "--", "python", script])
        .env("PYTHONIOENCODING", "utf-8")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("SELFTEST call recusado: call() só na thread principal"), "{stderr}");
    // Os 200 envios chegaram inteiros e em ordem; a função rodou na thread principal.
    assert!(stderr.contains("SELFTEST post na principal=True texto=199"), "{stderr}");
    assert!(!stderr.contains("linha inválida"), "nenhuma mensagem misturada: {stderr}");
    let _ = std::fs::remove_file(png);
}
