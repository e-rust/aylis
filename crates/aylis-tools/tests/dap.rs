//! Depuração de verdade pelo protocolo DAP: um programa Rust com o CodeLLDB e um script
//! Python com o debugpy. Pula se o adaptador não estiver instalado.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use aylis_tools::dap::{Breakpoint, Event, Session, codelldb};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_dap_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Espera um evento que satisfaça `f` (até 30 s).
fn wait(s: &mut Session, f: impl Fn(&Event) -> bool) -> Event {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(e) = s.events().into_iter().find(|e| f(e)) {
            return e;
        }
        assert!(Instant::now() < deadline, "evento não chegou");
        std::thread::sleep(Duration::from_millis(20));
    }
}

const RUST: &str = "fn soma(a: i32, b: i32) -> i32 {
    let total = a + b;
    let dobro = total * 2;
    dobro / 2
}

fn main() {
    let nome = String::from(\"aylis\");
    let x = soma(2, 3);
    println!(\"{nome} {x}\");
}
";

#[test]
fn rust_stops_at_a_breakpoint_and_shows_the_variables() {
    let Some(adapter) = codelldb() else {
        eprintln!("CodeLLDB não encontrado: teste pulado");
        return;
    };
    let dir = temp("rust");
    let src = dir.join("main.rs");
    std::fs::write(&src, RUST).unwrap();
    let exe = dir.join(if cfg!(windows) { "prog.exe" } else { "prog" });
    let built = Command::new("rustc").args(["-g", "-C", "opt-level=0", "-o"]).arg(&exe).arg(&src).status().unwrap();
    assert!(built.success());

    let mut bps = BTreeMap::new();
    bps.insert(src.clone(), vec![Breakpoint::at(2)]); // let total = a + b;
    let mut s = Session::launch_rust(&adapter, &exe, &dir, &[], &bps).unwrap();
    let stopped = wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    assert!(matches!(&stopped, Event::Stopped { reason, .. } if reason == "breakpoint"), "{stopped:?}");
    let stack = s.stack().unwrap();
    assert!(stack[0].name.contains("soma"), "{stack:?}");
    assert_eq!(stack[0].line, 2);
    assert!(stack[0].file.as_ref().is_some_and(|f| f.ends_with("main.rs")));
    let locals = s.locals(stack[0].id).unwrap();
    let value = |name: &str| locals.iter().find(|v| v.name == name).map(|v| v.value.clone());
    assert_eq!(value("a").as_deref(), Some("2"), "{locals:?}");
    assert_eq!(value("b").as_deref(), Some("3"));
    // Um passo: a linha seguinte, com o total calculado.
    s.step_over().unwrap();
    wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    let stack = s.stack().unwrap();
    assert_eq!(stack[0].line, 3, "{stack:?}");
    let locals = s.locals(stack[0].id).unwrap();
    assert!(locals.iter().any(|v| v.name == "total" && v.value == "5"), "{locals:?}");
    // Sai da função: volta ao main, onde está o nome.
    s.step_out().unwrap();
    wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    let stack = s.stack().unwrap();
    assert!(stack[0].name.contains("main"), "{stack:?}");
    // Continua até o fim.
    s.resume().unwrap();
    wait(&mut s, |e| matches!(e, Event::Terminated | Event::Exited(_)));
    let _ = std::fs::remove_dir_all(dir);
}

const PYTHON: &str = "def soma(a, b):
    total = a + b
    return total

nome = 'aylis'
x = soma(2, 3)
print(nome, x)
";

#[test]
fn python_stops_at_a_breakpoint_and_shows_the_variables() {
    let has = Command::new("python").args(["-c", "import debugpy"]).status().is_ok_and(|s| s.success());
    if !has {
        eprintln!("debugpy não encontrado: teste pulado");
        return;
    }
    let dir = temp("python");
    let script = dir.join("main.py");
    std::fs::write(&script, PYTHON).unwrap();
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let mut child = Command::new("python")
        .args(["-Xfrozen_modules=off", "-m", "debugpy", "--listen", &port.to_string(), "--wait-for-client"])
        .arg(&script)
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut bps = BTreeMap::new();
    bps.insert(script.clone(), vec![Breakpoint::at(2)]);
    let mut s = Session::attach_python(port, &bps).unwrap();
    let stopped = wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    assert!(matches!(&stopped, Event::Stopped { reason, .. } if reason == "breakpoint"), "{stopped:?}");
    let stack = s.stack().unwrap();
    assert_eq!((stack[0].name.as_str(), stack[0].line), ("soma", 2));
    let locals = s.locals(stack[0].id).unwrap();
    assert!(locals.iter().any(|v| v.name == "a" && v.value == "2"), "{locals:?}");
    assert_eq!(s.evaluate(stack[0].id, "a * 10").unwrap(), "20");
    s.step_over().unwrap();
    wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    let stack = s.stack().unwrap();
    let locals = s.locals(stack[0].id).unwrap();
    assert!(locals.iter().any(|v| v.name == "total" && v.value == "5"), "{locals:?}");
    s.resume().unwrap();
    wait(&mut s, |e| matches!(e, Event::Terminated | Event::Exited(_)));
    let _ = child.wait();
    let _ = std::fs::remove_dir_all(dir);
}

const LOOP: &str = "total = 0
for i in range(10):
    total += i
print(total)
";

#[test]
fn python_conditional_and_hit_count_breakpoints() {
    let has = Command::new("python").args(["-c", "import debugpy"]).status().is_ok_and(|s| s.success());
    if !has {
        eprintln!("debugpy não encontrado: teste pulado");
        return;
    }
    let dir = temp("python_cond");
    let script = dir.join("laco.py");
    std::fs::write(&script, LOOP).unwrap();
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let mut child = Command::new("python")
        .args(["-Xfrozen_modules=off", "-m", "debugpy", "--listen", &port.to_string(), "--wait-for-client"])
        .arg(&script)
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut bps = BTreeMap::new();
    // Só para quando i == 6.
    bps.insert(script.clone(), vec![Breakpoint { line: 3, condition: "i == 6".into(), ..Default::default() }]);
    let mut s = Session::attach_python(port, &bps).unwrap();
    wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    let stack = s.stack().unwrap();
    assert_eq!(s.evaluate(stack[0].id, "i").unwrap(), "6");
    // Troca por "na terceira passagem daqui em diante" (i = 9 seria a 3a depois do 7).
    s.set_breakpoints(&script, &[Breakpoint { line: 3, hit_condition: "3".into(), ..Default::default() }]).unwrap();
    s.resume().unwrap();
    wait(&mut s, |e| matches!(e, Event::Stopped { .. }));
    let stack = s.stack().unwrap();
    let i: i64 = s.evaluate(stack[0].id, "i").unwrap().parse().unwrap();
    assert!(i > 6, "a contagem pulou passagens: i = {i}");
    s.set_breakpoints(&script, &[]).unwrap();
    s.resume().unwrap();
    wait(&mut s, |e| matches!(e, Event::Terminated | Event::Exited(_)));
    let _ = child.wait();
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn rust_conditional_breakpoint_skips_when_false() {
    let Some(adapter) = codelldb() else {
        eprintln!("CodeLLDB não encontrado: teste pulado");
        return;
    };
    let dir = temp("rust_cond");
    let src = dir.join("main.rs");
    std::fs::write(&src, RUST).unwrap();
    let exe = dir.join(if cfg!(windows) { "prog.exe" } else { "prog" });
    assert!(Command::new("rustc").args(["-g", "-C", "opt-level=0", "-o"]).arg(&exe).arg(&src).status().unwrap().success());
    let mut bps = BTreeMap::new();
    // soma(2, 3): a nunca é 7, então não para; a segunda condição vale.
    bps.insert(src.clone(), vec![Breakpoint { line: 2, condition: "a == 7".into(), ..Default::default() }, Breakpoint { line: 3, condition: "b == 3".into(), ..Default::default() }]);
    let mut s = Session::launch_rust(&adapter, &exe, &dir, &[], &bps).unwrap();
    let stopped = wait(&mut s, |e| matches!(e, Event::Stopped { .. } | Event::Terminated | Event::Exited(_)));
    assert!(matches!(stopped, Event::Stopped { .. }), "{stopped:?}");
    let stack = s.stack().unwrap();
    assert_eq!(stack[0].line, 3, "parou na linha da condição verdadeira");
    s.resume().unwrap();
    wait(&mut s, |e| matches!(e, Event::Terminated | Event::Exited(_)));
    let _ = std::fs::remove_dir_all(dir);
}

const HEAVY: &str = "use std::collections::HashMap;

struct Cliente {
    nome: String,
    idade: u32,
    notas: Vec<f64>,
}

fn main() {
    let numeros: Vec<u64> = (0..100_000).collect();
    let mut mapa = HashMap::new();
    for i in 0..5_000 {
        mapa.insert(format!(\"chave{i}\"), i);
    }
    let cliente = Cliente { nome: \"Ana\".into(), idade: 30, notas: vec![7.5, 8.0, 9.25] };
    let mut soma = 0u64;
    for n in numeros.iter().take(5) {
        soma += n;
    }
    println!(\"{} {} {} {}\", soma, mapa.len(), cliente.nome, cliente.idade + cliente.notas.len() as u32);
}
";

/// A sessão numa thread: passos com variáveis grandes chegam rápido, e os campos de uma
/// variável vêm só quando pedidos.
#[test]
fn rust_steps_through_the_worker_with_heavy_locals() {
    use aylis_tools::dap::{Go, Job, Reply};
    let Some(adapter) = codelldb() else {
        eprintln!("CodeLLDB não encontrado: teste pulado");
        return;
    };
    let dir = temp("rust_worker");
    let src = dir.join("main.rs");
    std::fs::write(&src, HEAVY).unwrap();
    let exe = dir.join(if cfg!(windows) { "prog.exe" } else { "prog" });
    assert!(Command::new("rustc").args(["-g", "-C", "opt-level=0", "-o"]).arg(&exe).arg(&src).status().unwrap().success());
    let mut bps = BTreeMap::new();
    bps.insert(src.clone(), vec![Breakpoint::at(16)]); // let mut soma = 0u64;
    let w = Session::launch_rust(&adapter, &exe, &dir, &[], &bps).unwrap().into_worker(None);
    let next = |f: &dyn Fn(&Reply) -> bool| -> Reply {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            for r in w.replies() {
                if f(&r) {
                    return r;
                }
            }
            assert!(Instant::now() < deadline, "resposta não chegou");
            std::thread::sleep(Duration::from_millis(5));
        }
    };
    let stopped = |r: &Reply| matches!(r, Reply::Events(e) if e.iter().any(|e| matches!(e, Event::Stopped { .. })));
    next(&stopped);
    let fetch = |w: &aylis_tools::dap::Worker| {
        let t = Instant::now();
        w.send(Job::Stopped);
        let Reply::Stopped { frames, frame, locals, .. } = next(&|r| matches!(r, Reply::Stopped { .. })) else { unreachable!() };
        (frames[frame].line, locals, t.elapsed())
    };
    let (line, locals, first) = fetch(&w);
    let mut times = vec![first];
    assert_eq!(line, 16);
    for name in ["numeros", "mapa", "cliente"] {
        assert!(locals.iter().any(|v| v.name == name), "{name}: {locals:?}");
    }
    // Os campos do cliente, sob demanda.
    let cliente = locals.iter().find(|v| v.name == "cliente").unwrap();
    assert!(cliente.children > 0);
    w.send(Job::Children { reference: cliente.children });
    let Reply::Children { vars, .. } = next(&|r| matches!(r, Reply::Children { .. })) else { unreachable!() };
    assert!(vars.iter().any(|v| v.name == "idade" && v.value == "30"), "{vars:?}");
    // O mapa: o tamanho sem varrer a tabela, e os itens ao abrir.
    let mapa = locals.iter().find(|v| v.name == "mapa").unwrap();
    assert!(mapa.value.contains("5000"), "{mapa:?}");
    w.send(Job::Children { reference: mapa.children });
    let Reply::Children { vars, .. } = next(&|r| matches!(r, Reply::Children { .. })) else { unreachable!() };
    assert!(vars.len() >= 100, "itens do mapa: {}", vars.len());
    // Passos (F8): cada um até a pilha e as variáveis chegarem.
    for _ in 0..6 {
        let t = Instant::now();
        w.send(Job::Go(Go::StepOver));
        next(&stopped);
        let step = t.elapsed();
        let (_, _, took) = fetch(&w);
        times.push(took);
        eprintln!("passo: {step:?} (parar) + {took:?} (pilha e variáveis)");
    }
    let slowest = times.iter().max().unwrap();
    assert!(*slowest < Duration::from_millis(500), "pilha e variáveis demoraram {slowest:?}");
    w.send(Job::Stop);
    let _ = std::fs::remove_dir_all(dir);
}

/// Gasta ~150 ms de processador e aloca 1 MB entre a linha 13 e a 16; o contador imita o do
/// `aylis` com `dev`.
const BUSY: &str = "use std::sync::atomic::{AtomicI64, Ordering};
#[unsafe(no_mangle)]
pub static AYLIS_DEV_LIVE_BYTES: AtomicI64 = AtomicI64::new(0);

fn gasta() -> u64 {
    let t = std::time::Instant::now();
    let mut x = 0u64;
    while t.elapsed() < std::time::Duration::from_millis(150) { x = std::hint::black_box(x.wrapping_add(1)); }
    x
}

fn main() {
    let antes = 1;
    let x = gasta();
    let v = vec![0u8; 1 << 20]; AYLIS_DEV_LIVE_BYTES.fetch_add(v.len() as i64, Ordering::Relaxed);
    println!(\"{antes} {x} {}\", v.len());
}
";

#[test]
fn rust_step_reports_cpu_time_and_memory_at_the_end_of_the_line() {
    use aylis_tools::dap::{Go, Job, Reply};
    let Some(adapter) = codelldb() else {
        eprintln!("CodeLLDB não encontrado: teste pulado");
        return;
    };
    let dir = temp("rust_timing");
    let src = dir.join("main.rs");
    std::fs::write(&src, BUSY).unwrap();
    let exe = dir.join(if cfg!(windows) { "prog.exe" } else { "prog" });
    assert!(Command::new("rustc").args(["-g", "-C", "opt-level=0", "-o"]).arg(&exe).arg(&src).status().unwrap().success());
    let mut bps = BTreeMap::new();
    bps.insert(src.clone(), vec![Breakpoint::at(13), Breakpoint::at(16)]);
    let mut s = Session::launch_rust(&adapter, &exe, &dir, &[], &bps).unwrap();
    // Compilado sozinho, o crate se chama `main` (no app, `aylis::dev_memory::...`).
    s.memory_expression = Some("main::AYLIS_DEV_LIVE_BYTES".into());
    let w = s.into_worker(None);
    let next = |f: &dyn Fn(&Reply) -> bool| -> Reply {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            for r in w.replies() {
                if f(&r) {
                    return r;
                }
            }
            assert!(Instant::now() < deadline, "resposta não chegou");
            std::thread::sleep(Duration::from_millis(5));
        }
    };
    let stopped = |r: &Reply| matches!(r, Reply::Events(e) if e.iter().any(|e| matches!(e, Event::Stopped { .. })));
    next(&stopped);
    w.send(Job::Stopped);
    let Reply::Stopped { frames, frame, .. } = next(&|r| matches!(r, Reply::Stopped { .. })) else { unreachable!() };
    assert_eq!(frames[frame].line, 13);
    // Espera parado: o tempo no depurador não conta.
    std::thread::sleep(Duration::from_millis(400));
    w.send(Job::Go(Go::Resume));
    next(&stopped);
    w.send(Job::Stopped);
    let Reply::Stopped { frames, frame, timing, .. } = next(&|r| matches!(r, Reply::Stopped { .. })) else { unreachable!() };
    assert_eq!(frames[frame].line, 16);
    let timing = timing.expect("medição");
    eprintln!("passo: {timing:?} → {}", timing.note());
    let cpu = timing.cpu.expect("tempo de processador");
    assert!((Duration::from_millis(120)..Duration::from_millis(400)).contains(&cpu), "processador {cpu:?}");
    assert!(timing.wall >= cpu && timing.wall < Duration::from_millis(400) + Duration::from_secs(2), "relógio {:?}", timing.wall);
    assert_eq!(timing.memory, Some(1 << 20), "memória");
    assert!(!timing.whole_process, "a thread do sistema");
    w.send(Job::Stop);
    let _ = std::fs::remove_dir_all(dir);
}

