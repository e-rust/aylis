//! O código dos projetos em TypeScript, C#, Java, Go, C++ e Free Pascal: o backend roda
//! pelo `aylis-host` (como o Python), com o cliente do protocolo de `clients/` copiado para
//! o projeto.
//!
//! Para cada form, um arquivo de eventos com o bloco de registro (refeito pela IDE a cada
//! "Gerar código") e as funções dos handlers (do usuário: a IDE só acrescenta as novas).
//! Mais o arquivo que registra todos os forms, o ponto de entrada e o arquivo de projeto da
//! linguagem (`.csproj`, `go.mod`).

use std::path::{Path, PathBuf};

use crate::Language;
use crate::codegen::{read, write};

const BEGIN: &str = "aylis-ide:register-begin";
const END: &str = "aylis-ide:register-end";
const DECL_BEGIN: &str = "aylis-ide:declarations-begin";
const DECL_END: &str = "aylis-ide:declarations-end";

/// `frmClientes` → `FrmClientes` (nome de classe/tipo).
fn class_name(form: &str) -> String {
    let mut c = form.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).filter(|c| c.is_alphanumeric() || *c == '_').collect(),
        None => "Form".into(),
    }
}

/// Troca o trecho entre os marcadores `begin`/`end` (das linhas deles inteiras).
fn replace_named(text: &str, begin: &str, end: &str, block: &str) -> String {
    match (text.find(begin), text.find(end)) {
        (Some(b), Some(e)) if b < e => {
            let start = text[..b].rfind('\n').map_or(0, |i| i + 1);
            let stop = text[e..].find('\n').map_or(text.len(), |i| e + i + 1);
            format!("{}{}{}", &text[..start], block, &text[stop..])
        }
        _ => text.to_string(),
    }
}

/// Põe `insert` antes da última linha que começa com `marker` (o fim da classe, `end.`).
fn insert_before_last(text: &str, marker: &str, insert: &str) -> String {
    match text.rfind(&format!("\n{marker}")) {
        Some(i) => format!("{}{}{}", &text[..i + 1], insert.trim_start_matches('\n'), &text[i + 1..]),
        None => format!("{text}{insert}"),
    }
}

/// Como cada linguagem escreve o arquivo de eventos de um form.
struct FormCode {
    /// O arquivo (relativo à pasta do projeto).
    file: String,
    /// O arquivo novo (com `{block}` e, onde houver, `{decl}`).
    new: String,
    /// O bloco de registro.
    block: String,
    /// As declarações (C++, Pascal), num bloco próprio.
    decl: Option<String>,
    /// A função de um handler, se ainda não existe (`(texto que acusa que existe, função)`).
    stubs: Vec<(String, String)>,
    /// Onde entram as funções novas: no fim do arquivo (vazio) ou antes da última linha
    /// que começa com isto.
    insert_before: &'static str,
}

fn form_code(lang: Language, module: &str, form: &str, handlers: &[String]) -> FormCode {
    let class = class_name(form);
    match lang {
        Language::TypeScript => FormCode {
            file: format!("src/forms/{module}.ts"),
            // O arquivo é do usuário: os handlers exportados (o registro fica no index.ts
            // gerado); dentro deles, {form} é a janela do evento.
            new: format!(
                "// Eventos do form {form}: o código é seu (o registro dos handlers fica no index.ts\n\
                 // gerado; a IDE só acrescenta os handlers novos). Dentro de um handler, {form} é a\n\
                 // janela do evento.\n\n\
                 import type {{ Aylis, AylisEvent }} from \"../aylis.ts\";\n"
            ),
            block: String::new(),
            decl: None,
            stubs: handlers.iter().map(|h| (format!("function {h}("), format!("\nexport function {h}(aylis: Aylis, ev: AylisEvent) {{\n    void aylis, ev;\n}}\n"))).collect(),
            insert_before: "",
        },
        Language::CSharp => FormCode {
            file: format!("src/Forms/{class}.cs"),
            // A unit é do usuário (o registro fica no AllForms.cs gerado); dentro de um
            // handler, {form} é a janela do evento.
            new: format!(
                "// Eventos do form {form}: o código é seu (o registro dos handlers fica no AllForms.cs\n\
                 // gerado; a IDE só acrescenta os handlers novos). Dentro de um handler, {form} é a\n\
                 // janela do evento.\n\n\
                 using AylisClient;\n\n\
                 namespace App.Forms;\n\n\
                 public static class {class}\n{{\n}}\n"
            ),
            block: String::new(),
            decl: None,
            stubs: handlers.iter().map(|h| (format!("void {h}("), format!("\n    public static void {h}(Aylis aylis, AylisEvent ev)\n    {{\n    }}\n"))).collect(),
            insert_before: "}",
        },
        Language::Java => FormCode {
            file: format!("src/forms/{class}.java"),
            // A unit é do usuário (o registro fica no Forms.java gerado); dentro de um handler,
            // {form} é a janela do evento.
            new: format!(
                "// Eventos do form {form}: o código é seu (o registro dos handlers fica no Forms.java\n\
                 // gerado; a IDE só acrescenta os handlers novos). Dentro de um handler, {form} é a\n\
                 // janela do evento.\n\n\
                 package forms;\n\n\
                 import dev.aylis.Aylis;\n\n\
                 public final class {class} {{\n}}\n"
            ),
            block: String::new(),
            decl: None,
            stubs: handlers.iter().map(|h| (format!("void {h}("), format!("\n    public static void {h}(Aylis aylis, Aylis.Event ev) {{\n    }}\n"))).collect(),
            insert_before: "}",
        },
        Language::Go => FormCode {
            file: format!("src/forms/{module}.go"),
            // A unit é do usuário (o registro fica no forms.go gerado); dentro de um handler,
            // {form} é a janela do evento.
            new: format!(
                "// Eventos do form {form}: o código é seu (o registro dos handlers fica no forms.go\n\
                 // gerado; a IDE só acrescenta os handlers novos). Dentro de um handler, {form} é a\n\
                 // janela do evento.\n\n\
                 package forms\n\n\
                 import \"aylis.dev/cliente/aylis\"\n\n\
                 var _ = aylis.Log\n\n\
                 // {class} reúne os handlers do form {form}.\n\
                 type {class} struct{{}}\n"
            ),
            block: String::new(),
            decl: None,
            stubs: handlers.iter().map(|h| (format!(") {h}("), format!("\nfunc (f {class}) {h}(app *aylis.Aylis, ev aylis.Event) {{\n}}\n"))).collect(),
            insert_before: "",
        },
        Language::Cpp => {
            let ns = format!("frm_{module}");
            FormCode {
                file: format!("src/forms/{module}.cpp"),
                new: format!(
                    "// Eventos do form {form}: o código é seu (as declarações e o registro dos handlers\n\
                     // ficam no forms.hpp gerado; a IDE só acrescenta os handlers novos). Dentro de um\n\
                     // handler, {form} é a janela do evento.\n\n\
                     #include \"../aylis.hpp\"\n\
                     #include \"forms.hpp\"\n\n\
                     namespace {ns} {{\n\n}}  // namespace {ns}\n"
                ),
                block: String::new(),
                decl: None,
                stubs: handlers
                    .iter()
                    .map(|h| (format!("void {h}(aylis::Aylis& app, const aylis::Event& ev) {{"), format!("\nvoid {h}(aylis::Aylis& app, const aylis::Event& ev) {{\n    (void)app;\n    (void)ev;\n}}\n")))
                    .collect(),
                insert_before: "}  // namespace",
            }
        }
        // A unit do form é do usuário (sem bloco marcado): a classe dele herda a base gerada
        // (os componentes), os handlers são métodos published (a tela os acha pelo nome) e o
        // RegisterForm fica na initialization. A geração só acrescenta os handlers novos
        // (`update_pascal_unit`).
        Language::Pascal => FormCode {
            file: format!("src/forms/{module}.pas"),
            new: format!(
                "{{ O form {form}: os handlers são métodos published (achados pelo nome usado no\n  \
                 layout) e o estado fica em campos private, um por janela aberta. Os componentes e a\n  \
                 classe base vêm da unit gerada (ayliscomponents); a IDE só acrescenta os handlers\n  \
                 novos. }}\n\
                 unit {module};\n\n{{$mode objfpc}}{{$H+}}\n\n\
                 interface\n\nuses\n  fpjson, aylis, ayliscomponents;\n\n\
                 type\n  T{class} = class(T{class}Base)\n  published\n  end;\n\n\
                 var\n  {form}: T{class};\n\n\
                 implementation\n\n\
                 initialization\n  RegisterForm(T{class}, '{form}', @{form});\nend.\n"
            ),
            decl: None,
            block: String::new(),
            stubs: handlers.iter().map(|h| (format!("T{class}.{h}("), format!("\nprocedure T{class}.{h}({});\nbegin\nend;\n", pascal_params(form, h)))).collect(),
            insert_before: "initialization",
        },
        Language::Rust | Language::Python => unreachable!("Rust e Python têm o próprio gerador"),
    }
}

/// O arquivo de eventos do form: novo, ou o de antes com os blocos refeitos e as funções
/// novas acrescentadas.
/// A unit de um form em Pascal: nova, a do modelo; existente, só ganha os handlers que o
/// form cita e ela não tem (a declaração na seção published da classe e o corpo antes da
/// initialization).
fn update_pascal_unit(existing: Option<String>, code: &FormCode, form: &str, handlers: &[String]) -> String {
    let class = class_name(form);
    let mut text = existing.unwrap_or_else(|| code.new.clone());
    for h in handlers {
        // Um evento que pergunta ligado a um handler com a assinatura comum (o form mudou, ou
        // um projeto de antes): a assinatura passa a ser a do evento, nos dois lugares.
        let params = pascal_params(form, h);
        let plain = "Sender: TAylisComponent; Event: TJSONObject";
        if params != plain {
            text = text.replace(&format!("procedure {h}({plain});"), &format!("procedure {h}({params});"));
            text = text.replace(&format!("procedure T{class}.{h}({plain});"), &format!("procedure T{class}.{h}({params});"));
        }
        if text.contains(&format!("T{class}.{h}(")) {
            continue;
        }
        // A declaração: no fim da primeira seção published da classe.
        let head = format!("T{class} = class(");
        if let Some(start) = text.find(&head) {
            let published = text[start..].find("\n  published\n").map(|i| start + i + "\n  published\n".len());
            let end = text[start..].find("\n  end;").map(|i| start + i + 1);
            let at = match (published, end) {
                // Depois dos métodos já declarados nela (até a próxima seção ou o fim).
                (Some(p), Some(e)) => {
                    let next = ["\n  private", "\n  protected", "\n  public", "\n  strict"].iter().filter_map(|s| text[p - 1..e].find(s).map(|i| p - 1 + i + 1)).min();
                    next.unwrap_or(e)
                }
                (None, Some(e)) => e,
                _ => text.len(),
            };
            text.insert_str(at, &format!("    procedure {h}({params});\n"));
        }
        let body = format!("\nprocedure T{class}.{h}({params});\nbegin\nend;\n");
        text = insert_before_last(&text, "initialization", &body);
    }
    text
}

fn update_form(existing: Option<String>, code: &FormCode) -> String {
    let mut text = match existing {
        Some(text) => {
            let mut t = replace_named(&text, BEGIN, END, &code.block);
            if let Some(decl) = &code.decl {
                t = replace_named(&t, DECL_BEGIN, DECL_END, decl);
            }
            t
        }
        None => code.new.replace("{block}", &code.block).replace("{decl}", code.decl.as_deref().unwrap_or("")),
    };
    for (exists, stub) in &code.stubs {
        if !text.contains(exists.as_str()) {
            text = if code.insert_before.is_empty() { format!("{text}{stub}") } else { insert_before_last(&text, code.insert_before, stub) };
        }
    }
    text
}

/// O handler vazio que a geração acrescenta (`stub`) e o texto que marca a definição dele
/// no arquivo (`marker`), como `function salvar_click(`.
pub(crate) fn stub(lang: Language, module: &str, form: &str, handler: &str) -> (String, String) {
    let code = form_code(lang, module, form, &[handler.to_string()]);
    let (marker, stub) = code.stubs.into_iter().next().unwrap_or_default();
    (marker, stub)
}

/// O arquivo de eventos de um form (para a IDE abrir).
pub fn form_file(lang: Language, module: &str, form: &str) -> String {
    form_code(lang, module, form, &[]).file
}

/// Gera o código de um projeto: os arquivos dos forms (`forms`: módulo, form, handlers),
/// o registro de todos, o ponto de entrada, o cliente e o arquivo de projeto. A unit de um form
/// só é aberta se ele está em `changed` (`None`: todos) ou se ainda não existe: os handlers só
/// mudam quando o form muda.
pub fn generate(dir: &Path, project_name: &str, lang: Language, forms: &[(String, String, Vec<String>)], changed: Option<&[String]>) -> Result<Vec<PathBuf>, String> {
    let mut written = Vec::new();
    let mut put = |rel: &str, text: &str, always: bool| -> Result<(), String> {
        let path = dir.join(rel);
        if always || !path.exists() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            write(&path, text)?;
            written.push(path);
        }
        Ok(())
    };
    // Se a unit existe: pela listagem da pasta (uma por pasta, não uma consulta por form).
    let mut files = crate::FormFiles::default();
    for (module, form, handlers) in forms {
        let code = form_code(lang, module, form, handlers);
        let unit = dir.join(&code.file);
        if changed.is_some_and(|c| !c.iter().any(|n| n.eq_ignore_ascii_case(form))) && files.exists(&unit) {
            continue;
        }
        let existing = read(&unit);
        let text = if lang == Language::Pascal { update_pascal_unit(existing.clone(), &code, form, handlers) } else { update_form(existing.clone(), &code) };
        if existing.as_deref() != Some(text.as_str()) {
            put(&code.file, &text, true)?;
        }
    }
    let classes: Vec<(String, String)> = forms.iter().map(|(m, f, _)| (m.clone(), class_name(f))).collect();
    let header = "Forms do projeto. Gerado pela IDE do Aylis a cada \"Gerar código\": não edite.";
    match lang {
        Language::TypeScript => {
            // O cliente vem do aylis (o `src/aylis.ts` só reexporta o da instalação).
            let from = import_specifier(&client_path(&dir.join("src"), "typescript/aylis.ts"));
            put("src/aylis.ts", &format!("// {BRIDGE_MARK} (a instalação, ou o repositório): este arquivo só aponta para ele.\n// {GENERATED}\n\nexport * from \"{from}\";\n"), true)?;
            // As classes dos forms se registram ao carregar o components.ts; aqui, os handlers.
            let mut t = format!("// {header}\n\nimport type {{ Aylis }} from \"../aylis.ts\";\nimport \"./components.ts\";\n");
            for (m, _) in &classes {
                t.push_str(&format!("import * as {m} from \"./{m}.ts\";\n"));
            }
            t.push_str("\n/** Registra os handlers de todos os forms. */\nexport function registerForms(aylis: Aylis) {\n");
            for (m, form, handlers) in forms {
                for h in handlers {
                    t.push_str(&format!("    aylis.on(\"{form}.{h}\", (ev) => {m}.{h}(aylis, ev));\n"));
                }
            }
            if forms.iter().all(|(_, _, h)| h.is_empty()) {
                t.push_str("    void aylis;\n");
            }
            t.push_str("}\n");
            put("src/forms/index.ts", &t, true)?;
            put(
                "src/main.ts",
                &format!(
                    "// {project_name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\n// aylis-host --project ../{project_name}.aylisproj -- bun src/main.ts\n\n\
                     import {{ Aylis }} from \"./aylis.ts\";\nimport {{ registerForms }} from \"./forms/index.ts\";\n\n\
                     const aylis = new Aylis();\nregisterForms(aylis);\nawait aylis.run(); // abre o form principal do projeto\n"
                ),
                false,
            )?;
        }
        Language::CSharp => {
            // O cliente vem do aylis: o `App.csproj` compila o da instalação (a cópia antiga sai).
            remove_client_copy(dir, "src/Aylis.cs", "protocolo do aylis");
            // As classes dos forms e os handlers (as units do usuário só têm os métodos).
            let mut t = format!("// {header}\n\nusing AylisClient;\n\nnamespace App.Forms;\n\npublic static class AllForms\n{{\n    /// <summary>Registra as classes dos forms e os handlers de todos.</summary>\n    public static void Register(Aylis aylis)\n    {{\n        Components.RegisterClasses();\n");
            for (_, form, handlers) in forms {
                let c = class_name(form);
                for h in handlers {
                    t.push_str(&format!("        aylis.On(\"{form}.{h}\", ev => {c}.{h}(aylis, ev));\n"));
                }
            }
            t.push_str("    }\n}\n");
            put("src/Forms/AllForms.cs", &t, true)?;
            put(
                "src/Program.cs",
                &format!("// {project_name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\n\nusing App.Forms;\nusing AylisClient;\n\nvar aylis = new Aylis();\nAllForms.Register(aylis);\naylis.Run(); // abre o form principal do projeto\n"),
                false,
            )?;
            put(
                "App.csproj",
                "<Project Sdk=\"Microsoft.NET.Sdk\">\n\n  <!-- O backend do projeto (C#), executado pelo aylis-host. Gerado pela IDE do Aylis. -->\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net8.0</TargetFramework>\n    <ImplicitUsings>enable</ImplicitUsings>\n    <Nullable>enable</Nullable>\n    <AssemblyName>App</AssemblyName>\n    <InvariantGlobalization>true</InvariantGlobalization>\n  </PropertyGroup>\n\n  <ItemGroup>\n    <Compile Remove=\".aylis/**\" />\n  </ItemGroup>\n\n</Project>\n",
                false,
            )?;
            // O cliente da instalação no `App.csproj` (o de antes ganha a linha; o caminho é refeito).
            if let Ok(text) = std::fs::read_to_string(dir.join("App.csproj")) {
                let updated = csproj_with_client(&text, &client_path(dir, "csharp/Aylis.cs"));
                if updated != text {
                    put("App.csproj", &updated, true)?;
                }
            }
        }
        Language::Java => {
            // O cliente vem do aylis: a compilação junta o `dev.aylis` da instalação (as cópias
            // antigas saem).
            for rel in ["src/dev/aylis/Aylis.java", "src/dev/aylis/Session.java", "src/dev/aylis/User.java", "src/dev/aylis/Server.java"] {
                remove_client_copy(dir, rel, "package dev.aylis;");
            }
            let _ = std::fs::remove_dir(dir.join("src/dev/aylis"));
            let _ = std::fs::remove_dir(dir.join("src/dev"));
            // As classes dos forms e os handlers (as units do usuário só têm os métodos).
            let mut t = format!("// {header}\n\npackage forms;\n\nimport dev.aylis.Aylis;\n\npublic final class Forms {{\n    /** Registra as classes dos forms e os handlers de todos. */\n    public static void register(Aylis aylis) {{\n        Components.registerClasses();\n");
            for (_, form, handlers) in forms {
                let c = class_name(form);
                for h in handlers {
                    t.push_str(&format!("        aylis.on(\"{form}.{h}\", ev -> {c}.{h}(aylis, ev));\n"));
                }
            }
            t.push_str("    }\n}\n");
            put("src/forms/Forms.java", &t, true)?;
            put(
                "src/Main.java",
                &format!("// {project_name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\n\nimport dev.aylis.Aylis;\n\npublic class Main {{\n    public static void main(String[] args) {{\n        Aylis aylis = new Aylis();\n        forms.Forms.register(aylis);\n        aylis.run(null); // abre o form principal do projeto\n    }}\n}}\n"),
                false,
            )?;
        }
        Language::Go => {
            // O cliente vem do aylis: o `go.mod` aponta o módulo dele para a instalação (as cópias
            // antigas saem, e o código que importava a cópia passa a importar o módulo).
            remove_client_copy(dir, "src/aylis/aylis.go", "protocolo do aylis");
            remove_client_copy(dir, "src/aylis/sessions.go", "package aylis");
            let _ = std::fs::remove_dir(dir.join("src/aylis"));
            retarget_go_imports(dir);
            // Os handlers de todos os forms (os tipos dos forms se registram no init do
            // components.go).
            let mut t = format!("// {header}\n\npackage forms\n\nimport \"aylis.dev/cliente/aylis\"\n\n// Register registra os handlers de todos os forms.\nfunc Register(app *aylis.Aylis) {{\n");
            for (_, form, handlers) in forms {
                let c = class_name(form);
                for h in handlers {
                    t.push_str(&format!("\tapp.On(\"{form}.{h}\", func(ev aylis.Event) {{ {c}{{}}.{h}(app, ev) }})\n"));
                }
            }
            if forms.iter().all(|(_, _, h)| h.is_empty()) {
                t.push_str("\t_ = app\n");
            }
            t.push_str("}\n");
            put("src/forms/forms.go", &t, true)?;
            put(
                "main.go",
                &format!("// {project_name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\npackage main\n\nimport (\n\t\"app/src/forms\"\n\t\"aylis.dev/cliente/aylis\"\n)\n\nfunc main() {{\n\tapp := aylis.New()\n\tforms.Register(app)\n\tapp.Run(\"\") // abre o form principal do projeto\n}}\n"),
                false,
            )?;
            // O módulo do cliente aponta para o do aylis (gerado a cada vez: o caminho é desta
            // máquina, ou relativo dentro do aylis).
            let module = client_path(dir, "go");
            put("go.mod", &format!("module app\n\ngo 1.21\n\n// {BRIDGE_MARK}: {GENERATED}\nrequire aylis.dev/cliente v0.0.0\n\nreplace aylis.dev/cliente => {module}\n"), true)?;
        }
        Language::Cpp => {
            // O cliente vem do aylis (o `src/aylis.hpp` só inclui o da instalação).
            let from = client_path(&dir.join("src"), "cpp/aylis.hpp");
            put("src/aylis.hpp", &format!("// {BRIDGE_MARK} (a instalação, ou o repositório): este arquivo só aponta para ele.\n// {GENERATED}\n\n#pragma once\n\n#include \"{from}\"\n"), true)?;
            // As declarações dos handlers (as units do usuário só têm as funções), as classes dos
            // forms e o registro de todos.
            let mut t = format!("// {header}\n\n#pragma once\n\n#include \"../aylis.hpp\"\n#include \"components.hpp\"\n\n");
            for (m, _, handlers) in forms {
                t.push_str(&format!("namespace frm_{m} {{\n"));
                for h in handlers {
                    t.push_str(&format!("void {h}(aylis::Aylis& app, const aylis::Event& ev);\n"));
                }
                t.push_str(&format!("}}  // namespace frm_{m}\n\n"));
            }
            t.push_str("// Registra as classes dos forms e os handlers de todos.\ninline void register_forms(aylis::Aylis& app) {\n    aylis_components::register_classes();\n");
            for (m, form, handlers) in forms {
                for h in handlers {
                    t.push_str(&format!("    app.on(\"{form}.{h}\", [&app](const aylis::Event& ev) {{ frm_{m}::{h}(app, ev); }});\n"));
                }
            }
            if forms.iter().all(|(_, _, h)| h.is_empty()) {
                t.push_str("    (void)app;\n");
            }
            t.push_str("}\n");
            put("src/forms/forms.hpp", &t, true)?;
            put(
                "src/main.cpp",
                &format!("// {project_name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar).\n\n#include \"aylis.hpp\"\n#include \"forms/forms.hpp\"\n\nint main() {{\n    aylis::Aylis app;\n    register_forms(app);\n    app.run();  // abre o form principal do projeto\n    return 0;\n}}\n"),
                false,
            )?;
        }
        Language::Pascal => {
            // O cliente vem do aylis (o `src/aylis.pas` só inclui o da instalação).
            let from = client_path(&dir.join("src"), "pascal/aylis.pas");
            put("src/aylis.pas", &format!("{{ {BRIDGE_MARK} (a instalação, ou o repositório): este arquivo só aponta para ele.\n  {GENERATED} }}\n{{$I {from}}}\n"), true)?;
            let units: Vec<&str> = classes.iter().map(|(m, _)| m.as_str()).collect();
            // As units dos forms registram as classes deles na initialization (RegisterForm):
            // aqui elas só entram no programa.
            let mut t = format!("{{ {header} }}\nunit aylisforms;\n\n{{$mode objfpc}}{{$H+}}\n\ninterface\n\nuses\n  aylis;\n\n{{ Os forms do projeto (cada unit registra a classe do form dela ao carregar). }}\nprocedure RegisterForms(Aylis: TAylis);\n\nimplementation\n\n");
            let mut used = vec!["ayliscomponents"];
            used.extend(units.iter().copied());
            t.push_str(&format!("uses\n  {};\n\n", used.join(", ")));
            t.push_str("procedure RegisterForms(Aylis: TAylis);\nbegin\nend;\n\nend.\n");
            put("src/forms/aylisforms.pas", &t, true)?;
            put(
                "src/app.lpr",
                &format!("{{ {project_name}: o ponto de entrada. Gerado pela IDE do Aylis (pode editar). }}\nprogram app;\n\n{{$mode objfpc}}{{$H+}}\n\nuses\n  aylis, aylisforms;\n\nvar\n  Tela: TAylis;\nbegin\n  Tela := TAylis.Create;\n  RegisterForms(Tela);\n  Tela.Run('');  {{ abre o form principal do projeto }}\n  Tela.Free;\nend.\n"),
                false,
            )?;
        }
        Language::Rust | Language::Python => unreachable!(),
    }
    Ok(written)
}

/// Os parâmetros do método de um handler em Pascal: os eventos que perguntam com `var`, como
/// no Delphi (o cliente lê o que o handler mudou e responde à tela); os outros, o comum.
fn pascal_params(form: &str, handler: &str) -> &'static str {
    match crate::codegen::question_event(form, handler).as_deref() {
        Some("on_close_query") => "Sender: TAylisComponent; var CanClose: Boolean",
        Some("on_key_down") => "Sender: TAylisComponent; var Key: string; Event: TJSONObject",
        Some("on_key_press") => "Sender: TAylisComponent; var Key: string",
        Some("on_drag_start") => "Sender: TAylisComponent; var Data, Caption: string",
        Some("on_drag_over") => "Sender: TAylisComponent; const Data: string; var Accept: Boolean",
        Some("on_intercept") => "Sender: TAylisComponent; Info: TJSONObject; var Cancel: Boolean",
        _ => "Sender: TAylisComponent; Event: TJSONObject",
    }
}

/// O caminho de `from` (uma pasta do projeto) até `clients/<sub>` do aylis: relativo quando o
/// projeto está dentro do aylis (os exemplos funcionam em qualquer clone), absoluto fora (a
/// instalação). O cliente do protocolo não é mais copiado para o projeto (item 12): o projeto
/// aponta para o da instalação, e um aylis novo vale para todos os projetos.
fn client_path(from: &Path, sub: &str) -> String {
    format!("{}/clients/{sub}", crate::codegen::aylis_path_for(from))
}

/// O mesmo caminho como especificador de `import` do JavaScript (relativo começa com `./` ou
/// `../`; absoluto vira `file:///`).
fn import_specifier(path: &str) -> String {
    if path.starts_with("..") || path.starts_with("./") {
        path.to_string()
    } else if path.starts_with('/') {
        format!("file://{path}")
    } else {
        format!("file:///{path}")
    }
}

/// Apaga a cópia antiga do cliente que a geração punha no projeto (o arquivo é do aylis se tem
/// a marca dele). Devolve se apagou.
fn remove_client_copy(dir: &Path, rel: &str, mark: &str) -> bool {
    let path = dir.join(rel);
    let ours = std::fs::read_to_string(&path).is_ok_and(|t| t.contains(mark));
    ours && std::fs::remove_file(&path).is_ok()
}

/// O texto que marca os arquivos-ponte (gerados, apontando para o cliente da instalação).
const BRIDGE_MARK: &str = "O cliente do protocolo vem do aylis";

/// A frase dos arquivos que a geração refaz.
const GENERATED: &str = "Gerado pela IDE do Aylis a cada \"Gerar código\": não edite.";

/// O código em Go que importava a cópia antiga do cliente (`app/src/aylis`) passa a importar o
/// módulo do aylis (`aylis.dev/cliente/aylis`).
fn retarget_go_imports(dir: &Path) {
    let mut files = Vec::new();
    let mut stack = vec![dir.join("src")];
    stack.push(dir.to_path_buf());
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() && d != dir {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "go") {
                files.push(p);
            }
        }
    }
    for path in files {
        if let Ok(text) = std::fs::read_to_string(&path) {
            if text.contains("\"app/src/aylis\"") {
                let _ = write(&path, &text.replace("\"app/src/aylis\"", "\"aylis.dev/cliente/aylis\""));
            }
        }
    }
}

/// O `App.csproj` compilando o cliente do aylis (`clients/csharp/Aylis.cs`, da instalação ou do
/// repositório): troca o caminho de uma linha que já existe, ou acrescenta a linha.
fn csproj_with_client(text: &str, client: &str) -> String {
    let line = format!("    <Compile Include=\"{client}\" Link=\"Aylis.cs\" />");
    let mut out: Vec<String> = Vec::new();
    let mut found = false;
    for l in text.lines() {
        if l.contains("clients/csharp/Aylis.cs") {
            out.push(line.clone());
            found = true;
        } else {
            out.push(l.to_string());
        }
    }
    if !found {
        let block = format!("  <!-- {BRIDGE_MARK}: {GENERATED} -->\n  <ItemGroup>\n{line}\n  </ItemGroup>\n");
        if let Some(at) = out.iter().rposition(|l| l.trim() == "</Project>") {
            out.insert(at, block);
        } else {
            out.push(block);
        }
    }
    let mut s = out.join("\n");
    if text.ends_with('\n') {
        s.push('\n');
    }
    s
}

/// Os arquivos que a geração refaz em cada linguagem (o registro dos forms; o acesso tipado é
/// o [`crate::bindings::file`]), relativos à pasta da linguagem: a IDE os mostra à parte, com
/// cadeado, e abre somente leitura.
pub fn generated_files(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::Rust => &[],
        Language::Python => &["src/forms/__init__.py"],
        Language::TypeScript => &["src/forms/index.ts"],
        Language::CSharp => &["src/Forms/AllForms.cs"],
        Language::Java => &["src/forms/Forms.java"],
        Language::Go => &["src/forms/forms.go", "go.mod"],
        Language::Cpp => &["src/forms/forms.hpp"],
        Language::Pascal => &["src/forms/aylisforms.pas"],
    }
}

/// O programa (o ponto de entrada, criado uma vez e do usuário), relativo à pasta da linguagem.
pub fn program_files(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::Rust => &["src/main.rs", "src/lib.rs", "build.rs"],
        Language::Python => &["src/main.py"],
        Language::TypeScript => &["src/main.ts"],
        Language::CSharp => &["src/Program.cs"],
        Language::Java => &["src/Main.java"],
        Language::Go => &["main.go"],
        Language::Cpp => &["src/main.cpp"],
        Language::Pascal => &["src/app.lpr"],
    }
}
