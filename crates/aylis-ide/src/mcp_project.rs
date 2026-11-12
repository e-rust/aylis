//! As ferramentas do MCP para o que o usuário faz pelos menus do projeto: criar um frame, o
//! tema, as traduções, o ícone do app, a verificação, os testes e a foto de um form (o
//! relatório do Tabernáculo, M18 e M19). Cada uma passa pelas mesmas funções da IDE (o "Novo
//! form", a Aparência, a aba Traduções, a aba Ícone do app, a verificação do Executar), para
//! sair igual pelos dois caminhos.
use std::sync::{Arc, Mutex};

use aylis_core::Ui;
use serde_json::{Value, json};

use crate::app::{Ids, refresh};
use crate::document::Document;
use crate::state::Shared;

/// As seções de um tema que o `set_theme` muda (as do arquivo de tema, `aylis_json::theme`).
const THEME_SECTIONS: [&str; 8] = ["colors", "fonts", "radius", "spacing", "durations", "shadows", "styles", "components"];

fn arg<'a>(args: &'a Value, key: &str) -> &'a str {
    args.get(key).and_then(Value::as_str).unwrap_or_default()
}

fn project_of(state: &Shared) -> Result<aylis_project::Project, String> {
    state.borrow().project.clone().ok_or_else(|| "abra um projeto antes (open_project)".to_string())
}

/// Uma ferramenta deste módulo, na thread da interface; `None`: não é daqui.
pub fn run(ui: &mut Ui, state: &Shared, ids: &Ids, name: &str, args: &Value) -> Option<Result<Value, String>> {
    Some(match name {
        "new_frame" => new_frame(state, args),
        "get_theme" => get_theme(state, args),
        "set_theme" => set_theme(state, args),
        "translations" => translations(state, args),
        "set_translations" => set_translations(ui, state, ids, args),
        "set_app_icon" => set_app_icon(state, args),
        "check" => check(state, args),
        "test" => prepare_tests(ui, state),
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// Frame

/// `new_frame`: o form do frame (como o "Novo form", na pasta pedida), com o fundo transparente
/// (o `Frame` e o `create_in` mostram o fundo de quem o põe, G05) e, com `host`, um `Frame`
/// mostrando ele naquele form. Antes eram três passos à mão (`new_form`, o `fill` e o `Frame`).
fn new_frame(state: &Shared, args: &Value) -> Result<Value, String> {
    let name = arg(args, "name");
    let host = arg(args, "host");
    let project = project_of(state)?;
    if !host.is_empty() && project.form(host).is_none() {
        return Err(format!("o projeto não tem o form '{host}' (o `host`, onde o Frame vai)"));
    }
    let file = crate::actions::create_form(state, name, false, arg(args, "folder"))?;
    let size: Vec<(&str, Value)> = ["width", "height"].into_iter().filter_map(|k| args.get(k).filter(|v| v.is_number()).map(|v| (k, v.clone()))).collect();
    {
        let mut st = state.borrow_mut();
        let root = st.doc.root;
        st.doc.begin_change();
        st.doc.set_prop(root, "fill", &json!("transparent"))?;
        for (key, value) in &size {
            st.doc.set_prop(root, key, value)?;
        }
    }
    crate::actions::save(state, false);
    if host.is_empty() {
        return Ok(json!({ "form": name, "file": file, "note": "o form do frame está aberto no designer: ponha os componentes nele. Para mostrar num form, um Frame com `form` igual a este nome (ou `host` no new_frame)." }));
    }
    crate::actions::open_project_form(state, host);
    let mut st = state.borrow_mut();
    if !st.doc.form_name().eq_ignore_ascii_case(host) {
        return Err(format!("não consegui abrir o form {host}: {}", st.status));
    }
    let parent = arg(args, "parent");
    let parent_id = if parent.is_empty() { None } else { Some(st.doc.ui.find_in(st.doc.root, parent).ok_or_else(|| format!("o form {host} não tem '{parent}'"))?) };
    if parent_id.is_some_and(|p| st.doc.ui.class_name(p) == Some("Frame")) {
        return Err(format!("{parent} é um Frame: escolha o contêiner onde o Frame novo vai"));
    }
    if let Some(p) = parent_id {
        st.doc.select(Some(p));
    }
    let at = parent_id.and_then(|p| st.doc.rect_of(p)).map(|r| aylis_core::PointF::new(r.x + 8.0, r.y + 8.0));
    st.doc.begin_change();
    let id = st.doc.add_in("Frame", at, parent_id)?;
    let wanted = arg(args, "frame_name");
    if !wanted.is_empty() {
        st.doc.rename_component(id, wanted)?;
    }
    st.doc.set_prop(id, "form", &json!(name))?;
    for (key, value) in &size {
        st.doc.set_prop(id, key, value)?;
    }
    let align = arg(args, "align");
    if !align.is_empty() {
        st.doc.set_prop(id, "align", &json!(align))?;
    }
    st.doc.select(Some(id));
    let frame = st.doc.ui.name(id).unwrap_or_default().to_string();
    Ok(json!({ "form": name, "file": file, "host": host, "frame": frame, "note": format!("o form do frame foi salvo; o {host} está aberto com o Frame {frame} (salve com save)") }))
}

// ---------------------------------------------------------------------------
// Tema

/// O nome do tema da biblioteca que o `theme` do projeto escolhe ("" e "light" são o Aylis Light).
fn builtin_theme_name(theme: &str) -> String {
    match theme {
        "" | "light" => aylis_core::Theme::light().name,
        "dark" => aylis_core::Theme::dark().name,
        name => aylis_core::themes::builtin(name).map_or_else(|| name.to_string(), |t| t.name.clone()),
    }
}

/// `get_theme`: o tema do projeto como fica (o da biblioteca e o que o arquivo do projeto muda).
/// Sem `section`, tudo menos os estilos por extenso (só os nomes deles, que são muitos).
fn get_theme(state: &Shared, args: &Value) -> Result<Value, String> {
    let project = project_of(state)?;
    let theme = aylis_project::resolve_theme(&project, &aylis_project::Files::Dir(project.dir()))?;
    let mut value = aylis_json::theme_to_value(&theme);
    let file = project.theme.ends_with(".json").then(|| project.theme.clone());
    let own: Option<Value> = file.as_ref().and_then(|f| std::fs::read_to_string(project.file(f)).ok()).and_then(|t| serde_json::from_str(t.trim_start_matches('\u{feff}')).ok());
    let (section, key) = (arg(args, "section"), arg(args, "key"));
    let pick = |v: &Value| -> Option<Value> {
        let part = v.get(section)?;
        if key.is_empty() { Some(part.clone()) } else { part.get(key).cloned() }
    };
    let (body, own) = if section.is_empty() {
        if let Some(styles) = value.get_mut("styles") {
            let names: Vec<String> = styles.as_object().map(|m| m.keys().cloned().collect()).unwrap_or_default();
            *styles = json!(names);
        }
        (value, own)
    } else {
        if !THEME_SECTIONS.contains(&section) {
            return Err(format!("seção desconhecida: {section} (use {})", THEME_SECTIONS.join(", ")));
        }
        let body = pick(&value).ok_or_else(|| format!("o tema não tem {section}{}", if key.is_empty() { String::new() } else { format!(".{key}") }))?;
        (body, own.as_ref().and_then(pick))
    };
    Ok(json!({
        "theme": if project.theme.is_empty() { builtin_theme_name("") } else { project.theme.clone() },
        "file": file,
        "value": body,
        "own": own,
        "project_colors": project.colors,
        "note": "`value` é o tema como fica; `own`, o que o arquivo do projeto muda. set_theme muda um token ou um estilo (o tema da biblioteca vira a base de um arquivo em themes/)."
    }))
}

/// Um nome de arquivo a partir do nome do projeto (letras, números, `_` e `-`).
fn file_stem(name: &str) -> String {
    let stem: String = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c.to_ascii_lowercase() } else { '_' }).collect();
    if stem.trim_matches('_').is_empty() { "tema".into() } else { stem }
}

/// `set_theme`: muda um token (`colors.primary`, `radius.md`...) ou um estilo
/// (`styles.Button.danger`) no arquivo de tema do projeto. Com um tema da biblioteca, o arquivo
/// nasce em `themes/` partindo dele (`"base"`), e o projeto passa a usá-lo. `value` nulo tira a
/// chave (volta a valer a da base). Confere o tema inteiro antes de gravar, e os forms abertos
/// passam a mostrar o tema novo (como a Aparência).
fn set_theme(state: &Shared, args: &Value) -> Result<Value, String> {
    let mut project = project_of(state)?;
    let (section, key) = (arg(args, "section"), arg(args, "key"));
    if !THEME_SECTIONS.contains(&section) {
        return Err(format!("`section` é uma de: {}", THEME_SECTIONS.join(", ")));
    }
    if key.trim().is_empty() {
        return Err("falta `key` (o token, como primary, ou a classe do estilo, como Button.danger)".into());
    }
    let value = args.get("value").cloned().unwrap_or(Value::Null);
    let (file, mut doc, created) = if project.theme.ends_with(".json") {
        let text = std::fs::read_to_string(project.file(&project.theme)).map_err(|e| format!("{}: {e}", project.theme))?;
        let doc: Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", project.theme))?;
        (project.theme.clone(), doc, false)
    } else {
        let stem = file_stem(&project.name);
        let file = std::iter::once(format!("themes/{stem}.json")).chain((2..).map(|n| format!("themes/{stem}{n}.json"))).find(|f| !project.file(f).exists()).unwrap_or_default();
        (file, json!({ "name": project.name, "base": builtin_theme_name(&project.theme) }), true)
    };
    let root = doc.as_object_mut().ok_or("o arquivo de tema não é um objeto JSON")?;
    let part = root.entry(section).or_insert_with(|| json!({}));
    let part = part.as_object_mut().ok_or_else(|| format!("`{section}` do arquivo de tema não é um objeto"))?;
    if value.is_null() {
        part.remove(key);
    } else {
        part.insert(key.to_string(), value);
    }
    let text = serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())? + "\n";
    aylis_json::load_theme(&text).map_err(|e| format!("o tema não carrega com essa mudança (nada foi gravado): {e}"))?;
    let path = project.file(&file);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("não consegui criar {}: {e}", dir.display()))?;
    }
    std::fs::write(&path, text).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
    if project.theme != file {
        project.theme = file.clone();
        project.save()?;
    }
    let theme = aylis_project::resolve_theme(&project, &aylis_project::Files::Dir(project.dir()))?;
    let mut st = state.borrow_mut();
    crate::actions::show_project_theme(&mut st, &project, theme);
    st.project = Some(project);
    st.status = format!("Tema: {}.{} alterado ({})", section, key, file);
    Ok(json!({ "file": file, "created": created, "changed": format!("{section}.{key}") }))
}

// ---------------------------------------------------------------------------
// Traduções

/// O código de um idioma (`en-US`, `es`, `pt_BR`): vira o nome do arquivo em `locales/`.
fn language_arg(args: &Value) -> Result<String, String> {
    let language = arg(args, "language").trim();
    if language.is_empty() || language.len() > 20 || !language.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err("`language` é o código do idioma (en-US, es, fr...)".into());
    }
    Ok(language.to_string())
}

/// `translations`: sem `language`, os idiomas do projeto com quanto falta em cada; com ele, os
/// textos (o original e a tradução; `missing_only`: só os sem tradução), como a aba Traduções.
/// Salva antes os forms alterados (os textos saem dos arquivos), como a aba.
fn translations(state: &Shared, args: &Value) -> Result<Value, String> {
    crate::actions::save_modified_forms(state);
    let project = project_of(state)?;
    if arg(args, "language").is_empty() {
        let languages: Vec<Value> = aylis_project::i18n::languages(&project)
            .into_iter()
            .map(|l| match aylis_project::i18n::merge_locale(&project, &l) {
                Ok(m) => json!({ "language": l, "texts": m.map.len(), "missing": m.missing }),
                Err(e) => json!({ "language": l, "error": e }),
            })
            .collect();
        let app = if project.locale.is_empty() { "pt-BR" } else { project.locale.as_str() };
        return Ok(json!({ "app_language": app, "languages": languages, "note": "o texto escrito no designer (e o do tr(\"...\") no código) é a chave; set_translations grava as traduções de um idioma (um idioma novo nasce assim)" }));
    }
    let language = language_arg(args)?;
    let merged = aylis_project::i18n::merge_locale(&project, &language)?;
    let missing_only = args.get("missing_only").and_then(Value::as_bool).unwrap_or(false);
    let texts: serde_json::Map<String, Value> = merged.map.iter().filter(|(_, v)| !missing_only || v.is_empty()).map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect();
    Ok(json!({
        "language": language,
        "file": aylis_project::i18n::locale_file(&language),
        "exists": merged.path.exists(),
        "missing": merged.missing,
        "texts": texts,
    }))
}

/// `set_translations`: grava `texts` (`{"original": "tradução"}`) no idioma, com os textos novos
/// do projeto juntados, como o OK da aba Traduções. Tradução vazia: sem tradução (mostra o
/// original).
fn set_translations(ui: &mut Ui, state: &Shared, ids: &Ids, args: &Value) -> Result<Value, String> {
    let language = language_arg(args)?;
    let texts = args.get("texts").and_then(Value::as_object).ok_or("falta `texts`: { \"texto original\": \"tradução\", ... }")?;
    crate::actions::save_modified_forms(state);
    let project = project_of(state)?;
    let mut merged = aylis_project::i18n::merge_locale(&project, &language)?;
    let mut unknown = Vec::new();
    for (source, translated) in texts {
        let translated = translated.as_str().ok_or_else(|| format!("a tradução de '{source}' tem de ser texto"))?;
        if !merged.map.contains_key(source) {
            unknown.push(source.clone());
        }
        merged.map.insert(source.clone(), translated.to_string());
    }
    let path = aylis_project::i18n::save_locale(&project, &language, &merged.map)?;
    let missing = merged.map.values().filter(|v| v.is_empty()).count();
    state.borrow_mut().status = format!("Traduções gravadas: {} (troque o idioma em Projeto → Opções)", path.display());
    refresh(ui, state, ids);
    let mut out = json!({ "file": aylis_project::i18n::locale_file(&language), "set": texts.len(), "missing": missing });
    if !unknown.is_empty() {
        out["not_in_project"] = json!(unknown);
        out["note"] = json!("esses textos não aparecem nos forms nem num tr(\"...\") do código (gravados assim mesmo): confira se o original está igual");
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Ícone do app

/// `set_app_icon`: a imagem do ícone (como a aba Ícone do app): relativa ao projeto, ou um
/// caminho de fora, que é copiado para `assets/images`.
fn set_app_icon(state: &Shared, args: &Value) -> Result<Value, String> {
    let mut project = project_of(state)?;
    let image = arg(args, "image").trim().replace('\\', "/");
    if image.is_empty() {
        return Err("falta `image` (o PNG de 256 px ou mais, ou um SVG)".into());
    }
    let given = std::path::PathBuf::from(&image);
    let file = if given.is_absolute() {
        given
    } else {
        if given.components().any(|c| !matches!(c, std::path::Component::Normal(_) | std::path::Component::CurDir)) {
            return Err("use um caminho dentro do projeto (sem ..) ou um caminho completo".into());
        }
        project.file(&image)
    };
    let warnings = crate::iconview::set_app_icon(&mut project, &file, arg(args, "background"))?;
    let icon = project.publish.icon.clone();
    let mut st = state.borrow_mut();
    st.status = format!("Ícone do app: {} (gerado para cada plataforma ao publicar)", icon);
    st.project = Some(project);
    Ok(json!({ "icon": icon, "warnings": warnings, "note": "o .ico do Windows já foi gerado; as outras plataformas saem ao publicar" }))
}

// ---------------------------------------------------------------------------
// Verificação

/// `check`: a verificação do Executar e do Publicar (`aylis_project::check`, o `aylis-tool
/// check`): handlers, referências, assets, fontes, campos e o que muda de uma plataforma para
/// outra. Salva antes os forms alterados (ela lê os arquivos).
fn check(state: &Shared, args: &Value) -> Result<Value, String> {
    crate::actions::save_modified_forms(state);
    let project = project_of(state)?;
    let web = match arg(args, "target") {
        "" => state.borrow().run_target == aylis_tools::Target::Web,
        t => t.eq_ignore_ascii_case("web"),
    };
    let mut ui = crate::document::new_document_ui(false);
    ui.set_design_mode(false);
    let issues = aylis_project::check::project(&project, Some(&mut ui), web);
    let errors = issues.iter().filter(|i| i.error).count();
    let list: Vec<Value> = issues
        .iter()
        .map(|i| json!({ "level": if i.error { "error" } else { "warning" }, "form": i.form, "component": i.component, "message": i.message }))
        .collect();
    Ok(json!({ "project": project.name, "errors": errors, "warnings": issues.len() - errors, "issues": list }))
}

// ---------------------------------------------------------------------------
// Testes

/// A rodada de testes em andamento (uma por vez): o projeto e o resultado, quando sair.
struct TestRun {
    project: std::path::PathBuf,
    result: Arc<Mutex<Option<Result<aylis_tools::project_tests::TestReport, String>>>>,
}

static TESTS: Mutex<Option<TestRun>> = Mutex::new(None);

/// Há uma rodada de testes em andamento.
pub fn tests_running() -> bool {
    TESTS.lock().is_ok_and(|t| t.is_some())
}

/// O `test` na thread da interface: recusa no modo restrito (o MCP não confirma confiança) e,
/// sem rodada em andamento, salva os fontes e gera o código, como o Compilar.
fn prepare_tests(ui: &mut Ui, state: &Shared) -> Result<Value, String> {
    crate::mcp::restricted(state)?;
    project_of(state)?;
    if tests_running() {
        return Ok(Value::String("os testes já estão rodando".into()));
    }
    crate::codewindow::save_all(ui);
    crate::actions::generate_code(ui, state)?;
    Ok(Value::Null)
}

/// Começa os testes do projeto (ou, com uma rodada em andamento, fica nela) e espera até
/// `seconds`. `None`: ainda rodando (a próxima chamada espera a mesma rodada).
pub fn wait_tests(project: aylis_project::Project, seconds: u64) -> Option<Result<aylis_tools::project_tests::TestReport, String>> {
    let result = {
        let mut slot = TESTS.lock().ok()?;
        match slot.as_ref() {
            Some(run) => run.result.clone(),
            None => {
                let result: Arc<Mutex<Option<_>>> = Arc::default();
                let out = result.clone();
                let project_path = project.path.clone();
                std::thread::spawn(move || {
                    let report = aylis_tools::project_tests::run(&project, &aylis_tools::Control::default(), &mut |_| {});
                    if let Ok(mut r) = out.lock() {
                        *r = Some(report);
                    }
                });
                *slot = Some(TestRun { project: project_path, result: result.clone() });
                result
            }
        }
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
    loop {
        if let Some(done) = result.lock().ok().and_then(|mut r| r.take()) {
            if let Ok(mut slot) = TESTS.lock() {
                *slot = None;
            }
            return Some(done);
        }
        if std::time::Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

/// O projeto da rodada em andamento (para avisar quando o `test` é de outro).
pub fn running_project() -> Option<std::path::PathBuf> {
    TESTS.lock().ok().and_then(|t| t.as_ref().map(|r| r.project.clone()))
}

/// O resultado em JSON para o agente: os números, cada teste, o que os que falharam mostraram,
/// os erros de compilação e as últimas linhas.
pub fn report_json(report: &aylis_tools::project_tests::TestReport) -> Value {
    use aylis_tools::project_tests::Outcome;
    let cut = |text: &str| if text.chars().count() > 4000 { format!("{}…", text.chars().take(4000).collect::<String>()) } else { text.to_string() };
    json!({
        "success": report.success,
        "command": report.command,
        "passed": report.count(Outcome::Passed),
        "failed": report.count(Outcome::Failed),
        "ignored": report.count(Outcome::Ignored),
        "tests": report.cases.iter().take(500).map(|c| json!({ "name": c.name, "outcome": c.outcome.name() })).collect::<Vec<_>>(),
        "failures": report.failures.iter().take(40).map(|(name, text)| json!({ "name": name, "output": cut(text) })).collect::<Vec<_>>(),
        "compile_errors": report.compile_errors.iter().take(40).collect::<Vec<_>>(),
        "last_lines": report.last_lines,
    })
}

// ---------------------------------------------------------------------------
// Foto de um form

/// Um form do designer em PNG, do tamanho dele.
fn doc_png(doc: &mut Document) -> Result<Vec<u8>, String> {
    let root = doc.root;
    doc.ui.layout();
    let (w, h) = doc.ui.control(root).map_or((800.0, 600.0), |c| (c.width(), c.height()));
    let background = doc.ui.theme().color("background");
    let ui = &mut doc.ui;
    crate::host::render_png(w.ceil() as u32, h.ceil() as u32, background, |c| ui.paint(root, c)).ok_or_else(|| "não consegui desenhar o form (sem o renderizador nesta IDE)".to_string())
}

/// A foto de um form do projeto como o designer mostra (`form` vazio: o aberto; `variant`: a
/// variante de uma plataforma), sem abrir aba: o aberto numa aba sai como está (com o que não
/// foi salvo), e o outro é lido do arquivo. Devolve o título e o PNG.
pub fn form_png(state: &Shared, form: &str, variant: &str) -> Result<(String, Vec<u8>), String> {
    let mut st = state.borrow_mut();
    if form.is_empty() {
        if st.no_form {
            return Err("nenhum form aberto no designer".into());
        }
        let title = st.doc.form_name();
        return doc_png(&mut st.doc).map(|png| (title, png));
    }
    let project = st.project.clone().ok_or("abra um projeto antes (open_project)")?;
    let entry = project.form(form).ok_or_else(|| format!("o projeto não tem o form '{form}'"))?;
    let variant = Some(variant).filter(|v| !v.is_empty());
    let path = project.file(&variant.map_or_else(|| entry.file.clone(), |v| aylis_project::variant_file(&entry.file, v)));
    let title = match variant {
        Some(v) => format!("{} ({v})", entry.name),
        None => entry.name.clone(),
    };
    match st.find_tab_path(&path) {
        Some(i) if i == st.active => doc_png(&mut st.doc).map(|png| (title, png)),
        Some(i) => {
            let doc = st.tabs.get_mut(i).and_then(Option::as_mut).ok_or("a aba do form sumiu")?;
            doc_png(doc).map(|png| (title, png))
        }
        None => {
            if variant.is_some() && !path.is_file() {
                return Err(format!("o form {form} não tem a variante {}", variant.unwrap_or_default()));
            }
            let dark = st.doc.dark;
            drop(st);
            let mut doc = crate::actions::project_document(&project, &entry.name, variant)?;
            doc.set_dark(dark);
            doc_png(&mut doc).map(|png| (title, png))
        }
    }
}
