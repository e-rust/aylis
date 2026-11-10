//! Verificações do projeto que a IDE mostra antes de compilar (e o `aylis-tool check`): o que
//! funciona numa máquina e muda em outra, ou que só falharia rodando.

use crate::{Project, assets};

/// Um problema achado: erro bloqueia a compilação, aviso não.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Issue {
    pub error: bool,
    /// O form (`frmClientes`) e o componente (`edtNome`), quando o problema é de um.
    pub form: String,
    pub component: String,
    pub message: String,
}

impl Issue {
    fn warning(form: &str, component: &str, message: String) -> Self {
        Self { error: false, form: form.into(), component: component.into(), message }
    }
}

/// As famílias embutidas no projeto (`assets/fonts/<pasta>`): o nome do catálogo da pasta
/// ou, para uma fonte posta à mão, o nome da pasta e dos arquivos.
fn embedded_families(project: &Project) -> Vec<String> {
    let catalog = assets::font_catalog();
    let mut out = Vec::new();
    for folder in assets::project_fonts(project) {
        if let Some(entry) = catalog.iter().find(|e| e.folder == folder) {
            out.push(simplify(&entry.family));
        }
        out.push(simplify(&folder));
        if let Ok(dir) = std::fs::read_dir(project.dir().join("assets/fonts").join(&folder)) {
            for file in dir.flatten() {
                let name = file.file_name().to_string_lossy().to_string();
                let stem = name.split(['-', '[', '.']).next().unwrap_or_default();
                out.push(simplify(stem));
            }
        }
    }
    out
}

/// O nome para comparar: minúsculas, sem espaços (`"Fira Sans"` = `firasans`).
fn simplify(name: &str) -> String {
    name.chars().filter(|c| !c.is_whitespace() && *c != '_').flat_map(char::to_lowercase).collect()
}

/// Fontes do sistema: a do sistema muda de uma plataforma para outra, e com ela as medidas
/// e as quebras de linha (um texto que cabe no Windows corta no Android). Avisa quando o
/// projeto ou um componente usa uma família que não está em `assets/fonts/`.
pub fn fonts(project: &Project) -> Vec<Issue> {
    fonts_in(project, &layouts(project))
}

fn fonts_in(project: &Project, layouts: &[(&crate::FormEntry, serde_json::Value)]) -> Vec<Issue> {
    let embedded = embedded_families(project);
    let is_embedded = |family: &str| {
        // Um nome de fonte do projeto (`"titulo": "Montserrat"`) vale pela família dele.
        let family = project.fonts.get(family).map_or(family, String::as_str);
        embedded.contains(&simplify(family))
    };
    let mut out = Vec::new();
    if project.font.is_empty() {
        out.push(Issue::warning("", "", "o projeto usa a fonte do sistema, que muda de uma plataforma para outra (e com ela o tamanho dos textos): escolha uma fonte embutida em Projeto → Fontes".into()));
    } else if !is_embedded(&project.font) {
        out.push(Issue::warning("", "", format!("a fonte do projeto, {}, não está embutida (assets/fonts/): em outra plataforma ela pode não existir ou medir diferente", project.font)));
    }
    for (form, json) in layouts {
        let mut stack = vec![json.get("root").unwrap_or(json)];
        while let Some(node) = stack.pop() {
            let family = node.get("font_family").and_then(|v| v.as_str()).unwrap_or_default();
            if !family.is_empty() && !is_embedded(family) {
                let name = node.get("name").and_then(|v| v.as_str()).unwrap_or_default();
                out.push(Issue::warning(&form.name, name, format!("a fonte {family} não está embutida no projeto (assets/fonts/): em outra plataforma ela pode não existir ou medir diferente")));
            }
            if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
                stack.extend(children);
            }
        }
    }
    out
}

impl Issue {
    fn error(form: &str, component: &str, message: String) -> Self {
        Self { error: true, form: form.into(), component: component.into(), message }
    }
}

/// Os forms do projeto: (entrada, layout em JSON).
fn layouts(project: &Project) -> Vec<(&crate::FormEntry, serde_json::Value)> {
    project
        .forms
        .iter()
        .filter_map(|f| {
            let text = std::fs::read_to_string(project.dir().join(&f.file)).ok()?;
            Some((f, serde_json::from_str(&text).ok()?))
        })
        .collect()
}

/// Cada componente do layout (a raiz e os de dentro).
fn nodes(layout: &serde_json::Value) -> Vec<&serde_json::Value> {
    let mut out = Vec::new();
    let mut stack = vec![layout.get("root").unwrap_or(layout)];
    while let Some(node) = stack.pop() {
        out.push(node);
        if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
            stack.extend(children);
        }
    }
    out
}

fn name_of(node: &serde_json::Value) -> &str {
    node.get("name").and_then(|v| v.as_str()).unwrap_or_default()
}

/// Handlers citados no form sem função no código da linguagem do projeto (o que mais vale
/// para Python, TypeScript e as outras, onde nenhum compilador pega).
pub fn handlers(project: &Project) -> Vec<Issue> {
    handlers_in(project, &layouts(project))
}

/// Cada arquivo de código é lido uma vez por form (antes, uma vez por handler).
fn handlers_in(project: &Project, layouts: &[(&crate::FormEntry, serde_json::Value)]) -> Vec<Issue> {
    let mut out = Vec::new();
    for (entry, layout) in layouts {
        let code = std::fs::read_to_string(crate::codegen::entry_source(project, entry)).ok();
        for node in nodes(layout) {
            let Some(obj) = node.as_object() else { continue };
            for (key, value) in obj {
                let Some(handler) = value.as_str().filter(|h| !h.is_empty() && crate::codegen::is_event_key(key)) else { continue };
                if code.as_deref().is_some_and(|text| !crate::sync::handler_in(project.language, entry, handler, text)) {
                    out.push(Issue::error(&entry.name, name_of(node), format!("o {key} chama {handler}, que não existe no código")));
                }
            }
        }
    }
    out
}

/// Arquivos do projeto citados nos forms (`"src": "assets/images/logo.png"`) que não existem.
pub fn assets(project: &Project) -> Vec<Issue> {
    assets_in(project, &layouts(project))
}

fn assets_in(project: &Project, layouts: &[(&crate::FormEntry, serde_json::Value)]) -> Vec<Issue> {
    let mut out = Vec::new();
    for (entry, layout) in layouts {
        for node in nodes(layout) {
            let Some(obj) = node.as_object() else { continue };
            for (key, value) in obj {
                let Some(path) = value.as_str().filter(|p| p.starts_with("assets/")) else { continue };
                if !project.dir().join(path).exists() {
                    out.push(Issue::error(&entry.name, name_of(node), format!("o {key} cita {path}, que não existe no projeto")));
                }
            }
        }
    }
    out
}

/// Os forms carregados num `Ui` (com as classes já registradas por quem chama): o form que
/// não carrega e a referência a outro componente que não resolve (`data_source`,
/// `popup_menu`, `master_source`...).
pub fn references(project: &Project, ui: &mut aylis_core::Ui) -> Vec<Issue> {
    references_in(project, ui, &layouts(project))
}

fn references_in(project: &Project, ui: &mut aylis_core::Ui, layouts: &[(&crate::FormEntry, serde_json::Value)]) -> Vec<Issue> {
    use aylis_core::{PropKind, PropValue};
    let mut out = Vec::new();
    // Os forms carregam como no app: com o tema e as cores do projeto (`"fill": "green"` é
    // uma cor do `.aylisproj`); sem eles, todo nome de cor próprio parecia errado.
    let files = crate::Files::Dir(project.dir());
    match crate::resolve_theme(project, &files) {
        Ok(theme) => ui.set_theme(theme),
        Err(e) => out.push(Issue::error("", "", e)),
    }
    ui.set_palette(project.palette());
    crate::install_font_names(ui, project);
    for (entry, layout) in layouts {
        let root = match aylis_json::load(ui, &layout.to_string(), None) {
            Ok(root) => root,
            Err(e) => {
                // A mensagem começa pelo caminho (`frmX > pnl > btnMenu: ...`): o componente é
                // o último nome antes dos dois-pontos.
                let text = e.to_string();
                let component = text.split(':').next().and_then(|p| p.rsplit(" > ").next()).filter(|c| *c != entry.name && !c.contains(' ')).unwrap_or_default();
                out.push(Issue::error(&entry.name, component, format!("o form não carrega: {text}")));
                continue;
            }
        };
        let top = layout.get("root").unwrap_or(layout);
        for node in nodes(layout) {
            let name = name_of(node);
            let id = if std::ptr::eq(node, top) { Some(root) } else { ui.find_in(root, name) };
            let (Some(id), Some(obj)) = (id, node.as_object()) else { continue };
            for (key, value) in obj {
                let Some(target) = value.as_str().filter(|t| !t.is_empty()) else { continue };
                if ui.prop_info(id, key).is_none_or(|p| p.kind != PropKind::Component) {
                    continue;
                }
                if !matches!(ui.get_prop(id, key), Ok(PropValue::Component(Some(_)))) {
                    out.push(Issue::error(&entry.name, name, format!("o {key} aponta para {target}, que não existe")));
                }
            }
        }
        ui.free(root);
    }
    out
}

/// O código do projeto (os arquivos da linguagem dele), com o caminho.
fn code_files(project: &Project) -> Vec<(std::path::PathBuf, String)> {
    let mut out = Vec::new();
    let mut stack = vec![project.code_dir()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let path = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                // As saídas da compilação e as dependências não são do usuário.
                // O gerado também não (no ERP de 1009 forms, 10 MB que ninguém escreveu à mão).
                if !["target", "bin", "obj", "node_modules", "lib", "__pycache__", "build", "generated"].contains(&name.as_str()) {
                    stack.push(path);
                }
            } else if let Ok(text) = std::fs::read_to_string(&path) {
                out.push((path, text));
            }
        }
    }
    out
}

/// Caminhos em texto no código (`find_path("frmClientes.edtNome")`) que não existem nos
/// forms: o compilador não pega, e o código só falha rodando. É aviso, não erro: o código
/// pode criar o componente rodando (os cartões da Galeria).
pub fn code_paths(project: &Project) -> Vec<Issue> {
    code_paths_in(project, &layouts(project), &code_files(project))
}

fn code_paths_in(project: &Project, layouts: &[(&crate::FormEntry, serde_json::Value)], code: &[(std::path::PathBuf, String)]) -> Vec<Issue> {
    let mut known = std::collections::HashSet::new();
    for (entry, layout) in layouts {
        known.extend(paths_of(entry, layout));
    }
    code_paths_known(project, &known, code)
}

/// Os caminhos do form (`frmX` e `frmX.componente`), como o `find_path` os escreve.
fn paths_of(entry: &crate::FormEntry, layout: &serde_json::Value) -> Vec<String> {
    std::iter::once(entry.name.clone()).chain(nodes(layout).into_iter().map(|n| format!("{}.{}", entry.name, name_of(n)))).collect()
}

fn code_paths_known(project: &Project, known: &std::collections::HashSet<String>, code: &[(std::path::PathBuf, String)]) -> Vec<Issue> {
    let call = "find_path(\"";
    let mut out = Vec::new();
    for (path, text) in code {
        for (at, _) in text.match_indices(call) {
            let rest = &text[at + call.len()..];
            let Some(end) = rest.find('"') else { continue };
            let target = &rest[..end];
            // Só os caminhos de um form do projeto (frmX.y) que não existem.
            let form = target.split('.').next().unwrap_or_default();
            if project.form(form).is_some_and(|f| f.name == form) && !known.contains(target) {
                let line = text[..at].matches('\n').count() + 1;
                out.push(Issue::warning("", "", format!("{}:{line}: find_path(\"{target}\") não existe nos forms (só vale se o código criar esse componente)", path.display())));
            }
        }
    }
    out
}

/// O que só aparece rodando: `panic = "abort"` (um erro num handler fecha o app, sem a
/// proteção que mostra o erro e continua) e, na web, `std::thread::spawn` (o navegador não
/// tem threads: `aylis::background` ou `spawn_local`).
pub fn platform(project: &Project, web: bool) -> Vec<Issue> {
    platform_in(project, web, &|| code_files(project))
}

fn platform_in(project: &Project, web: bool, code: &dyn Fn() -> Vec<(std::path::PathBuf, String)>) -> Vec<Issue> {
    let mut out = Vec::new();
    if project.language != crate::Language::Rust {
        return out;
    }
    let cargo = std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap_or_default();
    let abort = cargo.lines().any(|l| l.split('#').next().unwrap_or_default().replace(' ', "").contains("panic=\"abort\""));
    if abort {
        out.push(Issue::warning("", "", "o Cargo.toml usa panic = \"abort\": um erro num handler fecha o aplicativo em vez de aparecer na caixa de erro e continuar".into()));
    }
    if web {
        for (path, text) in code() {
            if path.extension().is_some_and(|e| e == "rs") && text.contains("thread::spawn(") {
                out.push(Issue::error("", "", format!("{}: std::thread::spawn não funciona no navegador: use aylis::background (trabalho pesado) ou spawn_local", path.display())));
            }
        }
    }
    out
}

/// Todas as verificações, para "Executar", "Publicar" e o `aylis-tool check`. Com `ui` (as
/// classes registradas), também as referências entre componentes; `web`: o que só vale no
/// navegador.
pub fn project(project: &Project, ui: Option<&mut aylis_core::Ui>, web: bool) -> Vec<Issue> {
    // Os forms e o código lidos uma vez, para todas as verificações.
    project_all(project, ui, web, &layouts(project))
}

/// A ligação a um campo que o dataset não tem (`"data_field": "nmoe"` num DBEdit cujo
/// `data_source` aponta para um ClientDataSet sem esse Field). Só quando o dataset declara os
/// campos (os `Field` filhos, que são as colunas dele); sem nenhum, ele nem abre, e o
/// carregamento já avisa.
pub fn data_fields(project: &Project) -> Vec<Issue> {
    data_fields_in(&layouts(project))
}

fn data_fields_in(layouts: &[(&crate::FormEntry, serde_json::Value)]) -> Vec<Issue> {
    use std::collections::HashMap;
    // Por nome (e pelo caminho `form.nome`): o `data_set` de cada DataSource e os campos de
    // cada ClientDataSet.
    let mut sources: HashMap<String, String> = HashMap::new();
    let mut fields: HashMap<String, Vec<String>> = HashMap::new();
    for (entry, layout) in layouts {
        for node in nodes(layout) {
            let name = name_of(node).to_ascii_lowercase();
            if name.is_empty() {
                continue;
            }
            let keys = [name.clone(), format!("{}.{name}", entry.name.to_ascii_lowercase())];
            match node.get("class").and_then(|c| c.as_str()) {
                Some("DataSource") => {
                    if let Some(ds) = node.get("data_set").and_then(|v| v.as_str()) {
                        for k in &keys {
                            sources.insert(k.clone(), ds.to_ascii_lowercase());
                        }
                    }
                }
                Some("ClientDataSet") => {
                    let list: Vec<String> = node
                        .get("children")
                        .and_then(|c| c.as_array())
                        .into_iter()
                        .flatten()
                        .filter(|c| c.get("class").and_then(|v| v.as_str()) == Some("Field"))
                        .map(|c| c.get("field_name").and_then(|v| v.as_str()).filter(|s| !s.is_empty()).unwrap_or(name_of(c)).to_ascii_lowercase())
                        .collect();
                    for k in &keys {
                        fields.insert(k.clone(), list.clone());
                    }
                }
                _ => {}
            }
        }
    }
    let local = |form: &str, name: &str| -> [String; 2] { [format!("{}.{}", form.to_ascii_lowercase(), name.to_ascii_lowercase()), name.rsplit('.').next().unwrap_or(name).to_ascii_lowercase()] };
    let mut out = Vec::new();
    for (entry, layout) in layouts {
        for node in nodes(layout) {
            let (Some(field), Some(source)) = (node.get("data_field").and_then(|v| v.as_str()), node.get("data_source").and_then(|v| v.as_str())) else { continue };
            if field.is_empty() {
                continue;
            }
            let Some(ds) = local(&entry.name, source).iter().find_map(|k| sources.get(k)).cloned() else { continue };
            let Some(list) = local(&entry.name, &ds).iter().find_map(|k| fields.get(k)).cloned() else { continue };
            if !list.is_empty() && !list.contains(&field.to_ascii_lowercase()) {
                out.push(Issue::error(&entry.name, name_of(node), format!("o data_field cita {field}, que o dataset {ds} não tem")));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Só o que mudou

/// Muda o que a verificação confere: mudar faz conferir tudo de novo.
const CHECK_VERSION: u32 = 1;

/// O que a última verificação achou em cada form, em `generated/components/checked.json`.
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Checked {
    version: String,
    forms: std::collections::BTreeMap<String, CheckedForm>,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
struct CheckedForm {
    issues: Vec<Issue>,
    /// Os outros forms que ele cita (DataModules do `uses`, frames, `dm.componente`): se um
    /// deles muda, este é conferido de novo.
    mentions: Vec<String>,
    /// Os caminhos dele (`frmX.componente`), para o `find_path` no código.
    paths: Vec<String>,
    /// As marcas do JSON dele e do código dele quando foi conferido: outra marca, confere de
    /// novo, venha a mudança de onde vier (o F12 marcava o form em dia e a verificação mostrava
    /// o erro de antes; conferência de 08/11).
    #[serde(default)]
    stamps: Vec<Option<crate::FileStamp>>,
}

/// As marcas de agora do JSON do form e do arquivo de código dele (na linguagem do projeto).
fn form_stamps(project: &Project, entry: &crate::FormEntry) -> Vec<Option<crate::FileStamp>> {
    let stamp = |p: std::path::PathBuf| std::fs::metadata(p).ok().and_then(|m| crate::FileStamp::of(&m));
    let code = crate::codegen::form_source(project, &entry.name);
    vec![stamp(project.dir().join(&entry.file)), code.and_then(stamp)]
}

fn checked_path(project: &Project) -> std::path::PathBuf {
    project.code_dir().join("generated/components/checked.json")
}

/// A versão do que vale para todos os forms: a verificação, o aylis e o projeto sem a lista dos
/// forms (o tema e as cores mudam o que carrega).
fn checked_version(project: &Project) -> String {
    use std::hash::{Hash, Hasher};
    let mut settings = project.clone();
    settings.forms.clear();
    let mut h = std::collections::hash_map::DefaultHasher::new();
    serde_json::to_string(&settings).unwrap_or_default().hash(&mut h);
    format!("{CHECK_VERSION} {} {:016x}", env!("CARGO_PKG_VERSION"), h.finish())
}

/// Os nomes dos forms do projeto (em minúsculas → como está no projeto).
type FormNames = std::collections::HashMap<String, String>;

fn form_names(project: &Project) -> FormNames {
    project.forms.iter().map(|f| (f.name.to_ascii_lowercase(), f.name.clone())).collect()
}

/// Os forms do projeto que o layout cita: um valor que é o nome de um deles (`"uses":
/// ["dmDados"]`, o `form` de um frame) ou começa por ele (`"dmDados.pmAcoes"`).
fn mentions_in(names: &FormNames, own: &str, layout: &serde_json::Value) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut stack = vec![layout];
    while let Some(v) = stack.pop() {
        match v {
            serde_json::Value::String(s) => {
                let head = s.split('.').next().unwrap_or_default();
                if let Some(name) = names.get(&head.to_ascii_lowercase()).filter(|n| !n.eq_ignore_ascii_case(own)) {
                    if !out.contains(name) {
                        out.push(name.clone());
                    }
                }
            }
            serde_json::Value::Array(a) => stack.extend(a),
            serde_json::Value::Object(o) => stack.extend(o.values()),
            _ => {}
        }
    }
    out
}

/// O [`project`] conferindo só os forms em `changed` (os que mudaram desde a última geração e
/// os de código mexido), os que citam um deles e os que a última verificação não viu; dos
/// outros vale o que ela achou. `None` (ou sem verificação anterior, ou outro aylis, tema ou
/// configuração do projeto): todos. O código (`find_path`, `thread::spawn` na web) é conferido
/// inteiro, lido uma vez.
pub fn project_changed(project: &Project, mut ui: Option<&mut aylis_core::Ui>, web: bool, changed: Option<&[String]>) -> Vec<Issue> {
    let version = checked_version(project);
    let old: Option<Checked> = std::fs::read_to_string(checked_path(project)).ok().and_then(|t| serde_json::from_str(&t).ok()).filter(|c: &Checked| c.version == version);
    let (old, changed) = match (old, changed) {
        (Some(old), Some(changed)) => (old, changed),
        _ => {
            // Tudo: e o resultado de cada form fica guardado para a próxima.
            let layouts = layouts(project);
            let issues = project_all(project, ui, web, &layouts);
            save_checked(project, &version, &layouts, &issues, None);
            return issues;
        }
    };
    // Mudou: o que a geração disse, ou outra marca no JSON ou no código dele.
    let moved: Vec<String> = project
        .forms
        .iter()
        .filter(|e| old.forms.get(&e.name).is_some_and(|c| c.stamps != form_stamps(project, e)))
        .map(|e| e.name.clone())
        .collect();
    let is_changed = |name: &str| changed.iter().chain(moved.iter()).any(|c| c.eq_ignore_ascii_case(name));
    // O que conferir de novo: os que mudaram, os que citam um deles e os que não foram vistos.
    let recheck: Vec<&crate::FormEntry> = project
        .forms
        .iter()
        .filter(|e| is_changed(&e.name) || old.forms.get(&e.name).is_none_or(|c| c.mentions.iter().any(|m| is_changed(m))))
        .collect();
    let rechecked = |name: &str| recheck.iter().any(|e| e.name == name);
    let fresh: Vec<(&crate::FormEntry, serde_json::Value)> = recheck
        .iter()
        .filter_map(|f| {
            let text = std::fs::read_to_string(project.dir().join(&f.file)).ok()?;
            Some((*f, serde_json::from_str(&text).ok()?))
        })
        .collect();
    // Os citados por eles também carregam (o dataset de um DataModule, um frame), mas só os
    // problemas dos conferidos contam.
    let names = form_names(project);
    let mut cited: Vec<&crate::FormEntry> = Vec::new();
    for (entry, layout) in &fresh {
        for m in mentions_in(&names, &entry.name, layout) {
            if let Some(e) = project.form(&m).filter(|e| !rechecked(&e.name) && !cited.iter().any(|c| c.name == e.name)) {
                cited.push(e);
            }
        }
    }
    let mut loaded: Vec<(&crate::FormEntry, serde_json::Value)> = cited
        .iter()
        .filter_map(|f| {
            let text = std::fs::read_to_string(project.dir().join(&f.file)).ok()?;
            Some((*f, serde_json::from_str(&text).ok()?))
        })
        .collect();
    loaded.extend(fresh.iter().map(|(e, v)| (*e, v.clone())));
    // Na ordem do projeto (os DataModules antes, como ao abrir o app).
    loaded.sort_by_key(|(e, _)| project.forms.iter().position(|f| f.name == e.name));
    let mut found = handlers_in(project, &fresh);
    found.extend(assets_in(project, &fresh));
    found.extend(data_fields_in(&loaded));
    if let Some(ui) = ui.as_deref_mut() {
        found.extend(references_in(project, ui, &loaded));
    }
    found.extend(fonts_in(project, &fresh));
    // Os de antes dos que não mudaram, os novos dos conferidos e os do projeto todo.
    let mut issues: Vec<Issue> = found.iter().filter(|i| i.form.is_empty() || rechecked(&i.form)).cloned().collect();
    for entry in project.forms.iter().filter(|e| !rechecked(&e.name)) {
        issues.extend(old.forms.get(&entry.name).map(|c| c.issues.clone()).unwrap_or_default());
    }
    let mut known: std::collections::HashSet<String> = std::collections::HashSet::new();
    for entry in &project.forms {
        match fresh.iter().find(|(e, _)| e.name == entry.name) {
            Some((e, layout)) => known.extend(paths_of(e, layout)),
            None => known.extend(old.forms.get(&entry.name).map(|c| c.paths.clone()).unwrap_or_default()),
        }
    }
    let code = code_files(project);
    issues.extend(code_paths_known(project, &known, &code));
    issues.extend(platform_in(project, web, &|| code.clone()));
    save_checked(project, &version, &fresh, &issues, Some(old));
    issues
}

/// Todas as verificações com os forms e o código lidos uma vez.
fn project_all(project: &Project, ui: Option<&mut aylis_core::Ui>, web: bool, layouts: &[(&crate::FormEntry, serde_json::Value)]) -> Vec<Issue> {
    let code = code_files(project);
    let mut out = handlers_in(project, layouts);
    out.extend(assets_in(project, layouts));
    out.extend(data_fields_in(layouts));
    if let Some(ui) = ui {
        out.extend(references_in(project, ui, layouts));
    }
    out.extend(code_paths_in(project, layouts, &code));
    out.extend(platform_in(project, web, &|| code.clone()));
    out.extend(fonts_in(project, layouts));
    out
}

/// Guarda o que os forms de `layouts` têm agora (os outros ficam como em `old`).
fn save_checked(project: &Project, version: &str, layouts: &[(&crate::FormEntry, serde_json::Value)], issues: &[Issue], old: Option<Checked>) {
    let names = form_names(project);
    let mut checked = old.unwrap_or_default();
    checked.version = version.to_string();
    checked.forms.retain(|name, _| project.form(name).is_some());
    for (entry, layout) in layouts {
        checked.forms.insert(
            entry.name.clone(),
            CheckedForm {
                issues: issues.iter().filter(|i| i.form == entry.name).cloned().collect(),
                mentions: mentions_in(&names, &entry.name, layout),
                paths: paths_of(entry, layout),
                stamps: form_stamps(project, entry),
            },
        );
    }
    let path = checked_path(project);
    if let (Some(dir), Ok(text)) = (path.parent(), serde_json::to_string(&checked)) {
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(&path, text);
    }
}
