//! `aylis-tool`: o Executar e o Publicar da IDE na linha de comando.
//!
//! ```text
//! aylis-tool check [--python] [--target desktop|android|web|ios]...
//! aylis-tool run <projeto.aylisproj> [--target ...] [--language go]
//! aylis-tool publish <projeto.aylisproj> [--target ...] [--out pasta] [--debug] [--features a,b]
//! aylis-tool clean [--days 7] [--target pasta] [--project projeto.aylisproj]
//! aylis-tool installer-ide [--out pasta] [--skip-build]
//! aylis-tool translations [--apply tabela.tsv] [--templates]
//! aylis-tool new <pasta> <nome> [linguagem]
//! aylis-tool generate <projeto.aylisproj>
//! aylis-tool script <projeto.aylisproj> <roteiro.json> [--language go] [--output estado.json]
//! aylis-tool setup <linguagem>... | --all  instala o compilador, o depurador e o servidor de linguagem
//! aylis-tool ibge <pasta> [--offline]     os dados do exemplo do IBGE (examples/IBGE/assets/dados)
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use aylis_project::{Language, Project};
use aylis_tools::env::{self, Status};
use aylis_tools::process::{Event, Level};
use aylis_tools::{Control, Target, publish, run};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("check") => check(&args[1..]),
        // Os textos da IDE sem tradução; com --apply, acrescenta as da tabela.
        Some("translations") => {
            let root = aylis_tools::aylis_root();
            match value_of(&args[1..], "--apply") {
                Some(table) => std::fs::read_to_string(&table).map_err(|e| format!("{table}: {e}")).and_then(|t| aylis_tools::translations::apply(&root, &t)).map(|n| println!("{n} textos acrescentados")),
                None => {
                    let list = if args.iter().any(|a| a == "--templates") { aylis_tools::translations::missing_templates(&root) } else { aylis_tools::translations::missing(&root) };
                    for s in list {
                        println!("{s}");
                    }
                    Ok(())
                }
            }
        }
        // A IDE lite no navegador (o "Experimentar" da página do aylis).
        Some("ide-web") => {
            let out = PathBuf::from(value_of(&args[1..], "--out").unwrap_or_else(|| "dist/ide-web".into()));
            std::fs::create_dir_all(&out)
                .map_err(|e| e.to_string())
                .and_then(|_| aylis_tools::publish::ide_web(&out, &Control::default(), &mut print))
                .map(|index| println!("pronto: {}", index.display()))
        }
        // Os componentes de verdade da documentação: um wasm por família da paleta.
        Some("demos") => {
            let out = PathBuf::from(value_of(&args[1..], "--out").unwrap_or_else(|| "dist/demos".into()));
            let only: Vec<String> = value_of(&args[1..], "--family").map(|f| f.split(',').map(str::to_string).collect()).unwrap_or_default();
            std::fs::create_dir_all(&out)
                .map_err(|e| e.to_string())
                .and_then(|_| aylis_tools::publish::demos(&out, &only, &Control::default(), &mut print))
                .map(|pages| println!("pronto: {} famílias em {}", pages.len(), out.display()))
        }
        // Serve uma pasta publicada para a web (http://127.0.0.1:8080, só nesta máquina).
        Some("serve") => match args.get(1) {
            Some(dir) => aylis_tools::serve::serve(&PathBuf::from(dir), false, &Control::default(), &mut |e| {
                if let Event::Step(s) = e {
                    println!("{s}");
                }
            }),
            None => Err("uso: aylis-tool serve <pasta publicada para a web>".into()),
        },
        // Um pacote de código (tipo B): o crate assinado com a chave do autor.
        Some("pack-code") => match (args.get(1), value_of(&args[1..], "--name"), value_of(&args[1..], "--component")) {
            (Some(dir), Some(name), Some(component)) => {
                let manifest = aylis_project::packages::Manifest {
                    name: name.clone(),
                    version: value_of(&args[1..], "--version").unwrap_or_else(|| "1.0.0".into()),
                    author: value_of(&args[1..], "--author").unwrap_or_default(),
                    description: value_of(&args[1..], "--description").unwrap_or_default(),
                    category: value_of(&args[1..], "--category").unwrap_or_else(|| "Pacotes".into()),
                    component,
                    ..Default::default()
                };
                let out = PathBuf::from(value_of(&args[1..], "--out").unwrap_or_else(|| format!("{name}.aylispkg")));
                aylis_tools::packages::author_key()
                    .and_then(|key| aylis_tools::packages::pack_code(&out, &manifest, &PathBuf::from(dir), &key))
                    .map(|_| println!("ok: {}", out.display()))
            }
            _ => Err("uso: aylis-tool pack-code <pasta do crate> --name Medidor --component PressureDial [--version 1.0.0] [--author nome] [--category aba] [--out arquivo.aylispkg]".into()),
        },
        // Os documentos de teste do aylis-docview (PDF, XLSX, DOCX montados à mão).
        Some("test-docs") => {
            let dir = aylis_tools::aylis_root().join("crates").join("aylis-docview").join("tests").join("data");
            aylis_tools::testdocs::generate(&dir).map(|_| println!("ok: {}", dir.display()))
        }
        // A lista do seletor de emoji (crates/aylis-controls-base/data/emoji.tsv) a partir das listas do Unicode e do CLDR.
        Some("emoji") => match args.get(1) {
            Some(folder) => {
                let dest = aylis_tools::aylis_root().join("crates").join("aylis-controls-base").join("data").join("emoji.tsv");
                aylis_tools::emoji::generate(std::path::Path::new(folder), &dest).map(|n| println!("{n} emojis em {}", dest.display()))
            }
            None => Err("uso: aylis-tool emoji <pasta com emoji-test.txt, pt.json e pt-derived.json>".into()),
        },
        // Mede o tempo de quadro dos exemplos (compilados em release) e grava um JSON.
        Some("bench") => match args.get(1).filter(|a| !a.starts_with("--")) {
            Some(dest) => {
                let renderers: Vec<String> = value_of(&args, "--renderers").map_or_else(|| aylis_tools::bench::RENDERERS.map(String::from).to_vec(), |r| r.split(',').map(str::to_string).collect());
                let compare = value_of(&args, "--compare").map(PathBuf::from);
                aylis_tools::bench::run(&aylis_tools::aylis_root(), std::path::Path::new(dest), &renderers, compare.as_deref())
            }
            None => Err("uso: aylis-tool bench <saída.json> [--compare base.json] [--renderers d2d,gpu]".into()),
        },
        // A pasta resources/: ícones, fontes e emojis animados livres, baixados.
        Some("resources") => {
            let root = aylis_tools::aylis_root().join("resources");
            let all = !args.iter().any(|a| matches!(a.as_str(), "--icons" | "--fonts" | "--lottie"));
            let has = |flag: &str| all || args.iter().any(|a| a == flag);
            (|| -> Result<(), String> {
                if has("--icons") {
                    aylis_tools::resources::icons(&root)?;
                }
                if has("--fonts") {
                    aylis_tools::resources::fonts(&root)?;
                }
                if has("--lottie") {
                    aylis_tools::resources::lottie(&root)?;
                }
                Ok(())
            })()
        }
        // O adaptador DAP sobre o jdb (a IDE depura Java por ele): aylis-tool jdb-dap <jdb> <fontes>
        Some("jdb-dap") => match (args.get(1), args.get(2)) {
            (Some(jdb), Some(sources)) => aylis_tools::jdb_dap::serve(std::path::Path::new(jdb), std::path::Path::new(sources)),
            _ => Err("uso: aylis-tool jdb-dap <jdb> <pasta dos fontes>".into()),
        },
        // Um projeto novo, como Arquivo → Novo projeto da IDE: aylis-tool new <pasta> <nome> [linguagem]
        Some("new") => match (args.get(1), args.get(2)) {
            (Some(location), Some(name)) => {
                let language = args.get(3).map_or(Some(Language::Rust), |l| Language::from_name(l).or_else(|| Language::ALL.into_iter().find(|x| format!("{x:?}").eq_ignore_ascii_case(l))));
                match language {
                    Some(language) => aylis_project::create(std::path::Path::new(location), name, language)
                        .and_then(|p| aylis_project::codegen::generate(&p).map(|_| p))
                        .map(|p| println!("pronto: {}", p.path.display())),
                    None => Err(format!("linguagem desconhecida: {} (use {})", args[3], Language::ALL.map(|l| l.name()).join(", "))),
                }
            }
            _ => Err("uso: aylis-tool new <pasta> <nome> [Rust|Python|TypeScript|C#|Java|Go|C++|Pascal]".into()),
        },
        // A ponte do MCP para o Claude Desktop e o Claude Code: stdio ↔ o servidor da IDE, com o
        // token lido de %LOCALAPPDATA%\aylis\mcp.json (nunca na configuração deles).
        Some("mcp") => aylis_tools::mcp::bridge(),
        // O "Gerar código" da IDE: aylis-tool generate <projeto.aylisproj> [--language python]
        Some("generate") => match args.get(1) {
            Some(path) => Project::open(path)
                .and_then(|p| with_language(p, &args))
                .and_then(|p| aylis_project::codegen::generate(&p))
                .map(|files| files.iter().for_each(|f| println!("{}", f.display()))),
            None => Err("uso: aylis-tool generate <projeto.aylisproj> [--language python]".into()),
        },
        // O compilador: gera, verifica e compila (a IDE chama com --json e mostra os eventos).
        // aylis-tool build <projeto.aylisproj> [--target t] [--release] [--no-generate]
        // [--no-compile] [--language l] [--json]
        Some("build") => build(&args[1..]),
        // Toca um roteiro de teste sem janela: aylis-tool script <projeto.aylisproj> <roteiro.json> [--output estado.json]
        Some("script") => match (args.get(1), args.get(2)) {
            (Some(path), Some(script)) => std::path::absolute(path).map_err(|e| format!("{path}: {e}")).and_then(Project::open).and_then(|project| with_language(project, &args)).and_then(|project| {
                let output = value_of(&args, "--output").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join(format!("aylis_script_{}.json", std::process::id())));
                let script = std::path::absolute(script).map_err(|e| format!("{script}: {e}"))?;
                let state = run::script(&project, &script, &output, aylis_host(), &Control::new(), &mut print)?;
                println!("{state}");
                Ok(())
            }),
            _ => Err("uso: aylis-tool script <projeto.aylisproj> <roteiro.json> [--output estado.json]".into()),
        },
        // Configura linguagens sem passo manual (o assistente "Configurar linguagens" da IDE).
        Some("setup") => setup(&args[1..]),
        // Os dados do exemplo do IBGE (baixa o que falta e gera os arquivos do app).
        Some("ibge") => match args.get(1) {
            Some(dir) => aylis_tools::ibge::run(std::path::Path::new(dir), args.iter().any(|a| a == "--offline"), &mut |m| println!("{m}")),
            None => Err("uso: aylis-tool ibge <pasta dos dados> [--offline] (examples/IBGE/assets/dados)".to_string()),
        },
        // Baixa uma ferramenta fixada em resources/tools.lock.json, confere o SHA-256 e instala.
        Some("fetch") => match args.get(1) {
            Some(name) => aylis_tools::downloads::fetch(name, &mut |m| println!("{m}")).map(|_| ()),
            None => Err("uso: aylis-tool fetch <codelldb|netcoredbg|js-debug|clangd|bundletool|jdtls>".into()),
        },
        // Regrava resources/tools.lock.json com os hashes publicados (--latest: últimas versões).
        Some("update-tools") => {
            let root = PathBuf::from(aylis_project::codegen::aylis_root());
            aylis_tools::downloads::update_lock(&root, args.iter().any(|a| a == "--latest"), &mut |m| println!("{m}"))
        }
        // Refaz a documentação da página do aylis (os guias de docs/ e a referência de cada
        // componente) em examples/Landing/assets/docs.
        Some("docs") => {
            let root = aylis_project::codegen::aylis_root();
            let mut command = std::process::Command::new("cargo");
            command.args(["run", "-q", "-p", "aylis-ide", "--example", "docs"]).current_dir(&root);
            aylis_tools::process::run_tool(&mut command, &Control::new(), &mut print, aylis_tools::LineKind::Tool)
        }
        // Regrava resources/components.json depois de mudar uma classe (no repositório do aylis).
        Some("update-catalog") => {
            let root = aylis_project::codegen::aylis_root();
            let mut command = std::process::Command::new("cargo");
            command.args(["run", "-q", "-p", "aylis-remote", "--example", "update_catalog"]).current_dir(&root);
            aylis_tools::process::run_tool(&mut command, &Control::new(), &mut print, aylis_tools::LineKind::Tool)
        }
        Some("clean") => (|| -> Result<(), String> {
            let days = value_of(&args, "--days").and_then(|d| d.parse().ok()).unwrap_or(aylis_tools::cleanup::DEFAULT_DAYS);
            // --target: o target de outro projeto (o do NoOBS, que compila fora do do aylis).
            let target = value_of(&args, "--target").map(PathBuf::from).unwrap_or_else(aylis_tools::shared_target_dir);
            // --project: só o que é deste projeto (o .aylisproj ou a pasta do Cargo.toml).
            if let Some(project) = value_of(&args, "--project").map(PathBuf::from) {
                let dir = if project.extension().is_some_and(|e| e == "aylisproj") {
                    aylis_project::Project::open(&project).map_err(|e| e.to_string())?.code_dir()
                } else {
                    project
                };
                let r = aylis_tools::cleanup::clean_project(&target, &dir);
                println!("{}: {} unidades só deste projeto, {} MB", dir.display(), r.units, r.bytes / (1024 * 1024));
                return Ok(());
            }
            let r = aylis_tools::cleanup::clean(&target, days);
            println!("{}: {} unidades que nenhum projeto usa mais, substituídas ou sem uso, {} MB", target.display(), r.units, r.bytes / (1024 * 1024));
            Ok(())
        })(),
        // O instalador da IDE (dist/aylis-ide-<versão>-setup.exe). Rode pelo `cargo run`: o aylis-tool em
        // release não pode se sobrescrever enquanto está aberto.
        // A auditoria das dependências (cargo deny, regras em deny.toml).
        Some("audit") => aylis_tools::installer::audit(&PathBuf::from(aylis_project::codegen::aylis_root())),
        Some("installer-ide") => {
            let options = aylis_tools::installer::Options {
                out: value_of(&args, "--out").map(PathBuf::from),
                skip_build: args.iter().any(|a| a == "--skip-build"),
            };
            aylis_tools::installer::installer_ide(&options).map(|setup| {
                let mb = setup.metadata().map(|m| m.len() / (1024 * 1024)).unwrap_or(0);
                println!("pronto: {} ({mb} MB)", setup.display())
            })
        }
        Some("run") => with_project(&args[1..], |project, target, _| {
            let mut options = run::Options::new(target);
            options.aylis_host = aylis_host();
            let control = Control::new();
            let message = run::run(&project, &options, &control, &mut print)?;
            println!("{message}");
            Ok(())
        }),
        // A IDE empacotada para Linux ou macOS, com os tipos de arquivo do aylis:
        // aylis-tool ide-package --target linux|macos --binary aylis-ide [--out dist]
        Some("ide-package") => (|| -> Result<(), String> {
            let target = value_of(&args, "--target").ok_or("informe --target linux ou macos")?;
            let binary = PathBuf::from(value_of(&args, "--binary").ok_or("informe --binary (a IDE compilada no sistema de destino)")?);
            let out = PathBuf::from(value_of(&args, "--out").unwrap_or_else(|| "dist".into()));
            let logo = PathBuf::from(aylis_project::codegen::aylis_root()).join("crates/aylis-ide/resources/aylis-ide.ico");
            let made = aylis_tools::ide_package::package(&target, &binary, &logo, &out, env!("CARGO_PKG_VERSION"))?;
            println!("pronto: {}", made.display());
            Ok(())
        })(),
        // O agente remoto, no outro PC ou servidor: aylis-tool debug-agent [--listen 0.0.0.0:7444]
        // [--dir pasta]. Mostra o código de pareamento e atende para sempre.
        Some("debug-agent") => (|| -> Result<(), String> {
            let listen = value_of(&args, "--listen").unwrap_or_else(|| aylis_tools::agent::DEFAULT_ADDRESS.into());
            let dir = value_of(&args, "--dir").map(PathBuf::from).unwrap_or_else(|| std::env::temp_dir().join("aylis-agent"));
            let (private, public, secret) = aylis_tools::agent::agent_keys()?;
            let listener = std::net::TcpListener::bind(&listen).map_err(|e| format!("{listen}: {e}"))?;
            let port = listener.local_addr().map_err(|e| e.to_string())?.port();
            let host = std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")).unwrap_or_else(|_| "este-pc".into());
            let code = aylis_tools::agent::Pairing { address: format!("{host}:{port}"), public, secret }.code();
            println!("agente do Aylis em {listen}, trabalhando em {}", dir.display());
            println!("código de pareamento (troque o nome pelo IP se precisar):\n{code}");
            aylis_tools::agent::serve(listener, private, secret, dir, |line| println!("{line}"));
            Ok(())
        })(),
        // Compila e executa o projeto (Rust) no agente: aylis-tool agent-run app.aylisproj --agent CÓDIGO
        Some("agent-run") => with_project(&args[1..], |project, _, _| {
            let code = value_of(&args, "--agent").ok_or("informe o --agent CÓDIGO (o que o debug-agent mostrou)")?;
            let mut client = aylis_tools::agent::AgentClient::connect(&aylis_tools::agent::Pairing::parse(&code)?)?;
            let name = project.package_name();
            println!("enviando {}…", project.name);
            let remote = aylis_tools::agent::upload(&mut client, &name, &project.dir())?;
            println!("no agente: {remote}");
            let cwd = project.code_dir().strip_prefix(project.dir()).map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();
            client.call(serde_json::json!({ "cmd": "run", "project": name, "program": "cargo", "args": ["run"], "cwd": cwd }), std::time::Duration::from_secs(10))?;
            loop {
                match client.next_event()? {
                    Some(e) if e["event"] == "line" => println!("{}", e["text"].as_str().unwrap_or_default()),
                    Some(e) if e["event"] == "exit" => {
                        println!("terminou ({})", e["code"]);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }),
        // Um adaptador de depuração (DAP) que roda no agente, usado daqui como se fosse local:
        // aylis-tool agent-dap --agent CÓDIGO --project app.aylisproj -- codelldb --port...
        Some("agent-dap") => (|| -> Result<(), String> {
            let code = value_of(&args, "--agent").ok_or("informe o --agent CÓDIGO")?;
            let project = Project::open(value_of(&args, "--project").ok_or("informe o --project app.aylisproj")?)?;
            let adapter: Vec<String> = args.iter().skip_while(|a| *a != "--").skip(1).cloned().collect();
            let (program, rest) = adapter.split_first().ok_or("informe o adaptador depois de --")?;
            let mut client = aylis_tools::agent::AgentClient::connect(&aylis_tools::agent::Pairing::parse(&code)?)?;
            let name = project.package_name();
            aylis_tools::agent::upload(&mut client, &name, &project.dir())?;
            let reply = client.call(serde_json::json!({ "cmd": "dap_start", "project": name, "program": program, "args": rest }), std::time::Duration::from_secs(10))?;
            let remote = reply["root"].as_str().unwrap_or_default().to_string();
            let local = project.dir().display().to_string();
            aylis_tools::agent::relay_dap(&mut client, &local, &remote, std::io::stdin(), &mut std::io::stdout())
        })(),
        // O Android pelo Wi-Fi (o aparelho no cabo uma vez): aylis-tool android-wifi [--serial X].
        // Depois, o Executar e o debug-android usam o endereço (`--serial ip:5555`).
        Some("android-wifi") => (|| -> Result<(), String> {
            let serial = aylis_tools::debug::android_wifi(value_of(&args, "--serial").as_deref())?;
            println!("aparelho pelo Wi-Fi: {serial} (pode tirar o cabo)");
            Ok(())
        })(),
        // Depurar o app aberto no Android: aylis-tool debug-android app.aylisproj [--serial X]
        // [--lldb]. Grava a configuração do CodeLLDB; com --lldb, abre o lldb do NDK já anexado.
        Some("debug-android") => with_project(&args[1..], |project, _, _| {
            let serial = value_of(&args, "--serial");
            let debug = aylis_tools::debug::android_debug(&project, serial.as_deref())?;
            println!("lldb-server no aparelho; o app tem o pid {}", debug.pid);
            println!("no VS Code: \"aylis: anexar (Android)\" (CodeLLDB)");
            if args.iter().any(|a| a == "--lldb") {
                let lldb = debug.lldb.ok_or("o NDK não tem o lldb")?;
                let mut command = std::process::Command::new(lldb);
                for c in debug.commands.iter().chain(std::iter::once(&format!("process attach --pid {}", debug.pid))) {
                    command.arg("-o").arg(c);
                }
                // Outros comandos (para roteiros e testes): --batch "cmd1;cmd2".
                if let Some(batch) = value_of(&args, "--batch") {
                    for c in batch.split(';') {
                        command.arg("-o").arg(c.trim());
                    }
                    command.arg("--batch");
                }
                command.status().map_err(|e| e.to_string())?;
            }
            Ok(())
        }),
        // A chave de assinatura do Android: aylis-tool keystore chave.jks --alias app --name "Empresa"
        // (a senha em AYLIS_KEYSTORE_PASSWORD).
        Some("keystore") => (|| -> Result<(), String> {
            let path = args.get(1).filter(|a| !a.starts_with("--")).ok_or("informe o arquivo da chave")?;
            let alias = value_of(&args, "--alias").unwrap_or_else(|| "app".into());
            let name = value_of(&args, "--name").unwrap_or_else(|| "Desenvolvedor".into());
            let password = std::env::var("AYLIS_KEYSTORE_PASSWORD").map_err(|_| "informe a senha em AYLIS_KEYSTORE_PASSWORD")?;
            publish::create_keystore(std::path::Path::new(path), &alias, &password, &name, &Control::new(), &mut print)?;
            println!("chave criada: {path} (apelido {alias}); guarde o arquivo e a senha: a Play Store exige a mesma chave nas atualizações");
            Ok(())
        })(),
        Some("publish") => with_project(&args[1..], |project, target, out| {
            let mut options = publish::Options::new(target);
            options.out_dir = out;
            options.debug = args.iter().any(|a| a == "--debug");
            options.features = value_of(&args, "--features").map(|f| f.split(',').map(str::to_string).collect()).unwrap_or_default();
            options.aylis_host = aylis_host();
            // --aab: o pacote da Play Store; --installer (ou --msi): o instalador do Windows, do
            // tipo do projeto (publish.windows.installer: msi, nsis ou inno). A chave do
            // Android vem do projeto, com as senhas de AYLIS_KEYSTORE_PASSWORD/AYLIS_KEY_PASSWORD.
            options.android_bundle = args.iter().any(|a| a == "--aab");
            options.installer = args.iter().any(|a| a == "--installer" || a == "--msi");
            options.signing = publish::Signing::from_env(&project);
            if !project.publish.keystore.is_empty() && options.signing.is_none() {
                return Err("o projeto tem chave de assinatura: informe a senha em AYLIS_KEYSTORE_PASSWORD".into());
            }
            let control = Control::new();
            let published = publish::publish(&project, &options, &control, &mut print)?;
            println!("pronto: {}", published.main.display());
            Ok(())
        }),
        _ => Err("uso: aylis-tool check [<projeto.aylisproj> [--web]] | build <projeto.aylisproj> [--target t] [--release] [--no-generate] [--no-compile] [--json] | run <projeto.aylisproj> | publish <projeto.aylisproj> [--target desktop|android|web|ios] [--out pasta] [--aab] [--installer] | debug-android <projeto.aylisproj> [--serial s] [--lldb] | android-wifi [--serial s] | debug-agent [--listen endereço] [--dir pasta] | agent-run <projeto.aylisproj> --agent código | agent-dap --agent código --project <projeto.aylisproj> -- adaptador... | keystore <arquivo> [--alias a] [--name n] | clean [--days 7] | installer-ide [--out pasta] [--skip-build] | ide-package --target linux|macos --binary aylis-ide [--out pasta] | new <pasta> <nome> | generate <projeto.aylisproj> | script <projeto.aylisproj> <roteiro.json> | translations | resources | emoji <pasta> | bench <saída.json> | test-docs | docs".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // Vazio: o erro já foi dito (o `build --json` diz no próprio JSON).
            if !e.is_empty() {
                eprintln!("erro: {e}");
            }
            ExitCode::FAILURE
        }
    }
}

/// `aylis-tool setup go csharp` (ou `--all`): configura cada uma e diz o que ainda falta.
fn setup(args: &[String]) -> Result<(), String> {
    let languages: Vec<Language> = if args.iter().any(|a| a == "--all") {
        Language::ALL.to_vec()
    } else {
        args.iter()
            .map(|a| {
                Language::ALL
                    .into_iter()
                    .find(|l| l.folder().eq_ignore_ascii_case(a) || l.name().eq_ignore_ascii_case(a))
                    .ok_or_else(|| format!("linguagem desconhecida: {a} (use {})", Language::ALL.map(|l| l.folder()).join(", ")))
            })
            .collect::<Result<_, _>>()?
    };
    if languages.is_empty() {
        return Err("uso: aylis-tool setup <linguagem>... | --all".into());
    }
    let host = std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join(env::exe("aylis-host")))).filter(|p| p.is_file());
    let control = aylis_tools::process::Control::new();
    let mut missing = Vec::new();
    for language in languages {
        let checks = aylis_tools::setup::setup(language, host.as_deref(), &control, &mut |e| match e {
            Event::Step(s) => println!("▶ {s}"),
            Event::Line(l, _) => println!("  {l}"),
            Event::Diagnostic(d) => println!("  {}", d.message),
        });
        for c in checks.iter().filter(|c| c.status != Status::Ok && !c.name.starts_with("WiX")) {
            missing.push(format!("{}: {} — {}", language.name(), c.name, c.detail));
        }
        println!("{}: {}", language.name(), if aylis_tools::setup::language_ready(language) { "pronta" } else { "não ficou pronta" });
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("ainda falta:\n{}", missing.join("\n")))
    }
}

fn value_of(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned()
}

fn targets(args: &[String]) -> Result<Vec<Target>, String> {
    let mut out = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--target" {
            let t = it.next().ok_or("--target sem valor")?;
            out.push(Target::from_id(t).ok_or_else(|| format!("plataforma desconhecida: {t}"))?);
        }
    }
    Ok(out)
}

fn check(args: &[String]) -> Result<(), String> {
    // Com um projeto: a verificação dele (a mesma que a IDE faz antes de Executar e Publicar).
    if let Some(file) = args.iter().find(|a| a.ends_with(".aylisproj")) {
        return check_project(std::path::Path::new(file), args.iter().any(|a| a == "--web"));
    }
    let language = if args.iter().any(|a| a == "--python") { Language::Python } else { Language::Rust };
    let mut list = targets(args)?;
    if list.is_empty() {
        list = Target::ALL.to_vec();
    }
    let checks = env::check(language, &list, aylis_host().as_deref());
    print_checks(&checks);
    Ok(())
}

/// `aylis-tool check <projeto.aylisproj> [--web]`: os problemas do projeto; erro sai com
/// código 1 (a CI e um script de build param ali).
fn check_project(file: &std::path::Path, web: bool) -> Result<(), String> {
    let project = aylis_project::Project::open(file)?;
    let mut ui = check_ui();
    let issues = aylis_project::check::project(&project, Some(&mut ui), web);
    for issue in &issues {
        let place = match (issue.form.as_str(), issue.component.as_str()) {
            ("", _) => String::new(),
            (form, "") => format!("{form}: "),
            (form, component) => format!("{form}.{component}: "),
        };
        println!("{} {place}{}", if issue.error { "✗" } else { "!" }, issue.message);
    }
    let errors = issues.iter().filter(|i| i.error).count();
    println!("{}: {errors} erro(s), {} aviso(s)", project.name, issues.len() - errors);
    if errors > 0 { Err(format!("{errors} erro(s) no projeto")) } else { Ok(()) }
}

/// O `Ui` em que a verificação carrega os forms: todas as classes, as mesmas do designer da
/// IDE (o 3D fica fora do registro remoto, que também roda no navegador).
fn check_ui() -> aylis_core::Ui {
    let mut ui = aylis_core::Ui::new();
    aylis_remote::register_all(&mut ui);
    aylis_3d::register(&mut ui);
    ui.set_design_mode(false);
    ui
}

/// `aylis-tool build <projeto.aylisproj>`: gera, verifica e compila, parando no primeiro erro
/// (sai com código 1). Com `--json`, uma linha de JSON por evento (`aylis_tools::build`), o
/// que a IDE lê.
fn build(args: &[String]) -> Result<(), String> {
    use aylis_tools::build::{self, BuildEvent};
    let json = args.iter().any(|a| a == "--json");
    let flag = |f: &str| args.iter().any(|a| a == f);
    let emit = |value: serde_json::Value| {
        use std::io::Write;
        let mut out = std::io::stdout().lock();
        let _ = writeln!(out, "{value}");
        let _ = out.flush();
    };
    if json {
        emit(build::start_json());
    }
    // Interrompido pela IDE, o cargo que este iniciou não fica compilando sozinho.
    aylis_tools::process::children_die_with_me();
    let result = (|| -> Result<String, String> {
        let path = args.first().filter(|a| !a.starts_with("--")).ok_or("informe o .aylisproj")?;
        let project = with_language(Project::open(std::path::absolute(path).map_err(|e| format!("{path}: {e}"))?)?, args)?;
        let target = targets(args)?.first().copied().unwrap_or(Target::Desktop);
        let mut options = build::Options::new(target);
        options.release = flag("--release");
        options.generate = !flag("--no-generate");
        options.compile = !flag("--no-compile");
        let mut ui = check_ui();
        let message = build::build(&project, &options, Some(&mut ui), &Control::new(), &mut |e| {
            if json {
                emit(build::to_json(&e));
                return;
            }
            match e {
                BuildEvent::Event(e) => print(e),
                BuildEvent::Generated => {}
                BuildEvent::Issues(issues) => {
                    for issue in &issues {
                        let place = match (issue.form.as_str(), issue.component.as_str()) {
                            ("", _) => String::new(),
                            (form, "") => format!("{form}: "),
                            (form, component) => format!("{form}.{component}: "),
                        };
                        println!("{} {place}{}", if issue.error { "✗" } else { "!" }, issue.message);
                    }
                }
                BuildEvent::Built(_) => {}
            }
        });
        message
    })();
    if json {
        emit(build::end_json(&result));
        // O fim já foi dito no JSON; o código de saída ainda diz se deu certo.
        return result.map(|_| ()).map_err(|_| String::new());
    }
    result.map(|message| println!("{message}"))
}

fn print_checks(checks: &[aylis_tools::env::Check]) {
    let mut last = None;
    for c in checks {
        if last != Some(c.target) {
            println!("\n{}", c.target.name());
            last = Some(c.target);
        }
        let mark = match c.status {
            Status::Ok => "✓",
            Status::Warning => "!",
            Status::Missing => "✗",
        };
        println!("  {mark} {:<34} {}", c.name, c.detail);
        if c.status != Status::Ok {
            if let Some(fix) = &c.fix {
                println!("      corrigir: {}", fix.command_line());
            }
            if let Some(help) = &c.help {
                println!("      {help}");
            }
        }
    }
}

/// `--language go` (a pasta ou o nome): roda o código daquela linguagem, sem mudar a ativa
/// no `.aylisproj`.
fn with_language(project: Project, args: &[String]) -> Result<Project, String> {
    match value_of(args, "--language") {
        Some(l) => Language::ALL
            .into_iter()
            .find(|x| x.folder().eq_ignore_ascii_case(&l) || x.name().eq_ignore_ascii_case(&l))
            .map(|x| project.with_language(x))
            .ok_or_else(|| format!("linguagem desconhecida: {l} (use {})", Language::ALL.map(|x| x.folder()).join(", "))),
        None => Ok(project),
    }
}

fn with_project(args: &[String], f: impl FnOnce(Project, Target, Option<PathBuf>) -> Result<(), String>) -> Result<(), String> {
    let path = args.first().filter(|a| !a.starts_with("--")).ok_or("informe o .aylisproj")?;
    // Caminho absoluto: o aylis-host roda na pasta do projeto.
    let project = with_language(Project::open(std::path::absolute(path).map_err(|e| format!("{path}: {e}"))?)?, args)?;
    let target = targets(args)?.first().copied().unwrap_or(Target::Desktop);
    let out = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1)).map(PathBuf::from);
    f(project, target, out)
}

fn print(event: Event) {
    match event {
        Event::Step(s) => println!("▶ {s}"),
        Event::Line(l, _) => println!("{l}"),
        Event::Diagnostic(d) => {
            let level = match d.level {
                Level::Error => "erro",
                Level::Warning => "aviso",
                Level::Note => "nota",
            };
            match &d.location {
                Some(l) => println!("{level}: {}:{}:{}: {}", l.file.display(), l.line, l.column, d.message),
                None => println!("{level}: {}", d.message),
            }
        }
    }
}

/// O `aylis-host` ao lado deste executável.
fn aylis_host() -> Option<PathBuf> {
    let dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    Some(dir.join(env::exe("aylis-host"))).filter(|p| p.is_file())
}
