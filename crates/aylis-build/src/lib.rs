//! # aylis-build
//!
//! O `build.rs` de um projeto do aylis (gerado pela IDE) é só:
//!
//! ```text
//! fn main() {
//!     aylis_build::project();
//! }
//! ```
//!
//! Isso faz três coisas:
//!
//! 1. **Empacota o projeto** (`.aylisproj`, `forms/`, `themes/`, `assets/`, `locales/`) num arquivo que
//!    o executável inclui (`aylis::include_project!()`). O app publicado é um arquivo só,
//!    em qualquer plataforma (no Android e na web não há pasta do projeto para ler).
//! 2. **No Windows, põe o ícone no .exe** (o `publish.icon` do `.aylisproj`, um PNG de até
//!    256×256), sem precisar do `rc.exe`: o arquivo de recursos (`.res`) é escrito aqui
//!    e entregue ao linker.
//! 3. **Registra só as classes usadas** (app enxuto): lê os forms, junta as classes que
//!    aparecem neles e gera `aylis_classes.rs` em `OUT_DIR`, com uma função que registra
//!    só essas classes (`aylis::classes::Button(ui)`...). O `include_project!()` liga essa
//!    função ao app publicado. Como só os tipos usados são citados, o linker descarta o
//!    código dos outros componentes: um app com 4 tipos de componente leva só esses 4.
//!
//! O formato do pacote é simples ([`pack`] e [`unpack`]): o `aylis-project` lê com
//! [`unpack`].

use std::path::{Path, PathBuf};

/// Pastas do projeto que vão para o pacote.
pub const PACKED_DIRS: [&str; 5] = ["forms", "themes", "assets", "locales", "packages"];

/// Nome do pacote dentro de `OUT_DIR`.
pub const BUNDLE_FILE: &str = "project.aylispkg";

/// Nome do registro das classes usadas, dentro de `OUT_DIR`.
pub const CLASSES_FILE: &str = "aylis_classes.rs";

/// As classes do aylis que podem aparecer num form, com a feature do crate `aylis` que as
/// traz ("" = sempre presente). Cada uma tem uma função de mesmo nome em `aylis::classes`.
pub const CLASSES: &[(&str, &str)] = &[
    ("Form", ""),
    ("Label", ""),
    ("Button", ""),
    ("Edit", ""),
    ("CheckBox", ""),
    ("RadioButton", ""),
    ("Switch", ""),
    ("ProgressBar", ""),
    ("Panel", ""),
    ("Layout", ""),
    ("Frame", ""),
    ("Rectangle", ""),
    ("ComboBox", ""),
    ("ScrollBox", ""),
    ("Splitter", ""),
    ("Image", ""),
    ("MainMenu", ""),
    ("PopupMenu", ""),
    ("MenuItem", ""),
    ("Memo", ""),
    ("CodeEditor", ""),
    ("TabStrip", ""),
    ("NumberBox", ""),
    ("TrackBar", ""),
    ("ListBox", ""),
    ("RadioGroup", ""),
    ("GroupBox", ""),
    ("DateEdit", ""),
    ("TimeEdit", ""),
    ("DateTimeEdit", ""),
    ("Popover", ""),
    ("Calendar", ""),
    ("ListView", ""),
    ("TreeView", ""),
    ("Chart", ""),
    ("TabControl", ""),
    ("TabItem", ""),
    ("Expander", ""),
    ("StatusBar", ""),
    ("ToolBar", ""),
    ("Timer", ""),
    ("FrameTimer", ""),
    ("DragImage", ""),
    ("ApplicationEvents", ""),
    ("EventInterceptor", ""),
    ("BusyWatcher", ""),
    ("OpenDialog", ""),
    ("SaveDialog", ""),
    ("Permissions", ""),
    ("Biometrics", ""),
    ("AppLink", ""),
    ("Location", ""),
    ("Battery", ""),
    ("Network", ""),
    ("MotionSensor", ""),
    ("Compass", ""),
    ("ProximitySensor", ""),
    ("LightSensor", ""),
    ("Vibration", ""),
    ("Gamepad", ""),
    ("Screen", ""),
    ("BackgroundService", ""),
    ("Phone", ""),
    ("LockScreen", ""),
    ("Nfc", "nfc"),
    ("Bluetooth", "bluetooth"),
    ("PushNotifications", ""),
    ("PushTrigger", ""),
    ("WakeAppAction", ""),
    ("NotifyAction", ""),
    ("AnimatedText", ""),
    ("FloatAnimation", ""),
    ("ColorAnimation", ""),
    ("Icon", ""),
    ("Badge", ""),
    ("Banner", ""),
    ("Avatar", ""),
    ("SegmentedControl", ""),
    ("ActivityIndicator", ""),
    ("Breadcrumb", ""),
    ("HotkeyEdit", ""),
    ("DirectionPad", ""),
    ("Joystick", ""),
    ("LevelMeter", ""),
    ("RecordButton", ""),
    ("ScreenLayout", ""),
    ("MediaCard", ""),
    ("FrameView", ""),
    ("CropBox", ""),
    ("SeekBar", ""),
    ("PaintCanvas", ""),
    ("NodeGraph", ""),
    ("ChatView", ""),
    ("Waveform", ""),
    ("Card", ""),
    ("Knob", ""),
    ("InnerLoading", ""),
    ("MarkupTable", ""),
    ("SlideItem", ""),
    ("Parallax", ""),
    ("Uploader", ""),
    ("PopupEdit", ""),
    ("OptionGroup", ""),
    ("AjaxBar", ""),
    ("ChatBubble", ""),
    ("RichEditor", ""),
    ("GridLayout", ""),
    ("FlowLayout", ""),
    ("Divider", ""),
    ("NavigationDrawer", ""),
    ("BottomNavigation", ""),
    ("BottomSheet", ""),
    ("Carousel", ""),
    ("Chip", ""),
    ("ChipGroup", ""),
    ("Rating", ""),
    ("RangeSlider", ""),
    ("OtpInput", ""),
    ("SplitButton", ""),
    ("AutoComplete", ""),
    ("ColorPicker", ""),
    ("ColorEdit", ""),
    ("FileInput", ""),
    ("FileDropZone", ""),
    ("Snackbar", ""),
    ("SkeletonLoader", ""),
    ("ProgressCircle", ""),
    ("EmptyState", ""),
    ("Pagination", ""),
    ("Stepper", ""),
    ("Timeline", ""),
    ("SpeedDial", ""),
    ("AvatarGroup", ""),
    ("EventCalendar", ""),
    ("CommandPalette", ""),
    ("TimePicker", ""),
    ("AppBar", ""),
    ("NavigationRail", ""),
    ("SideSheet", ""),
    ("PageControl", ""),
    ("UpDown", ""),
    ("WheelPicker", ""),
    ("ButtonGroup", ""),
    ("Gauge", ""),
    ("Manometer", ""),
    ("Thermometer", ""),
    ("TankLevel", ""),
    ("FlowMeter", ""),
    ("SevenSegment", ""),
    ("SignalLight", ""),
    ("Valve", ""),
    ("Motor", ""),
    ("Pump", ""),
    ("Conveyor", ""),
    ("Fan", ""),
    ("Silo", ""),
    ("EmergencyStop", ""),
    ("SelectorSwitch", ""),
    ("PushButtons", ""),
    ("Pipe", ""),
    ("Wire", ""),
    ("ElectricalSymbol", ""),
    ("ActivityRings", ""),
    ("LoadingIndicator", ""),
    ("AngleDial", ""),
    ("ShadowEditor", ""),
    ("EasingEditor", ""),
    ("TimingEditor", ""),
    ("EmojiPicker", ""),
    ("MapView", ""),
    ("FpsMeter", ""),
    ("TitleBar", ""),
    ("Notification", ""),
    ("MessageDialog", ""),
    ("EventRecorder", ""),
    ("Analytics", ""),
    ("Taskbar", ""),
    ("DataModule", ""),
    ("Ellipse", ""),
    ("Circle", ""),
    ("Line", ""),
    ("Polygon", ""),
    ("Star", ""),
    ("Pie", ""),
    ("Arc", ""),
    ("Path", ""),
    ("VirtualList", ""),
    ("KpiCard", ""),
    ("PivotTable", ""),
    ("Slicer", ""),
    ("ChoroplethMap", ""),
    ("CandleChart", ""),
    ("OrderBook", ""),
    ("DepthChart", ""),
    ("TickerTape", ""),
    ("Watchlist", ""),
    ("TimeAndSales", ""),
    ("ClientDataSet", "data"),
    ("Field", "data"),
    ("DataSource", "data"),
    ("DataNavigator", "dbcontrols"),
    ("DataGrid", "dbcontrols"),
    ("Column", "dbcontrols"),
    ("DBImage", "dbcontrols"),
    ("DataRepeater", "dbcontrols"),
    ("DBChart", "dbcontrols"),
    ("Report", "dbcontrols"),
    ("ReportBand", "dbcontrols"),
    ("ReportPreview", "dbcontrols"),
    ("Barcode", "providers"),
    ("Export", "providers"),
    ("AylisBarcodeProvider", "providers"),
    ("CsvProvider", "providers"),
    ("XlsxProvider", "providers"),
    ("AylisPdfProvider", "providers"),
    ("TileServerProvider", "providers"),
    ("MediaPlayer", "media"),
    ("VideoPlayer", "media"),
    ("Microphone", "media"),
    ("AudioOutput", "media"),
    ("Sound", "media"),
    ("AudioMixer", "media"),
    ("CameraView", "media"),
    ("Lottie", "vector"),
    ("Svg", "vector"),
    ("Markdown", "markdown"),
    ("DocViewer", "docview"),
    ("TrayIcon", "tray"),
    ("WebView", "webview"),
    ("Model3D", "3d"),
    ("McpServer", "mcp"),
    ("McpTool", "mcp"),
];

/// Onde funcionam as classes que dependem da plataforma (a paleta da IDE mostra os ícones).
/// As que não estão aqui funcionam em todas: Windows, macOS, Linux, Android, iOS e web.
pub const CLASS_PLATFORMS: &[(&str, &[Platform])] = {
    use Platform::*;
    &[
        ("TrayIcon", &[Windows, MacOs, Linux]),
        ("Taskbar", &[Windows]),
        ("TitleBar", &[Windows, MacOs, Linux]),
        ("WebView", &[Windows, MacOs, Linux, Android, Ios]),
        ("CameraView", &[Windows, Linux, Android, Web]),
        ("Microphone", &[Windows, MacOs, Linux, Android, Ios]),
        // No iOS ainda não (o Core NFC e o Core Bluetooth pedem um Mac para compilar e testar).
        ("Nfc", &[Windows, MacOs, Linux, Android, Web]),
        ("Bluetooth", &[Windows, MacOs, Linux, Android, Web]),
        // Um servidor em 127.0.0.1: só onde o agente roda na mesma máquina.
        ("McpServer", &[Windows, MacOs, Linux]),
        ("McpTool", &[Windows, MacOs, Linux]),
    ]
};

/// Uma plataforma de destino.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
    Android,
    Ios,
    Web,
}

impl Platform {
    pub const ALL: [Platform; 6] = [Platform::Windows, Platform::MacOs, Platform::Linux, Platform::Android, Platform::Ios, Platform::Web];

    pub fn name(self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::MacOs => "macOS",
            Platform::Linux => "Linux",
            Platform::Android => "Android",
            Platform::Ios => "iOS",
            Platform::Web => "Web",
        }
    }
}

/// As plataformas onde a classe funciona; `None`: todas.
pub fn class_platforms(class: &str) -> Option<&'static [Platform]> {
    let base = class.split('.').next().unwrap_or(class);
    CLASS_PLATFORMS.iter().find(|(c, _)| *c == base).map(|(_, p)| *p)
}

const MAGIC: &[u8; 8] = b"AYLISPK1";

/// Para o `build.rs`: empacota o projeto da pasta do crate e, no Windows, embute o ícone.
///
/// # Panics
///
/// Quando não acha o `.aylisproj` ou não consegue gravar em `OUT_DIR` (o build para com a
/// mensagem).
pub fn project() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("rode pelo cargo (build.rs)"));
    // A pasta do projeto dita no `Cargo.toml` (o crate noutro lugar que o `.aylisproj`; B06 do
    // Tabernáculo), ou a do crate, ou a de cima.
    let declared = std::fs::read_to_string(manifest.join("Cargo.toml")).ok().and_then(|t| metadata_project(&t)).map(|p| manifest.join(p));
    println!("cargo:rerun-if-changed=Cargo.toml");
    project_at(&declared.or_else(|| project_root(&manifest)).unwrap_or(manifest));
}

/// O `project = "..."` da seção `[package.metadata.aylis]` de um `Cargo.toml`: a pasta do
/// `.aylisproj`, relativa ao crate.
pub fn metadata_project(cargo: &str) -> Option<String> {
    let mut inside = false;
    for line in cargo.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            inside = t == "[package.metadata.aylis]";
            continue;
        }
        if inside {
            if let Some(rest) = t.strip_prefix("project") {
                let value = rest.trim_start().strip_prefix('=')?.trim();
                return Some(value.trim_matches('"').to_string()).filter(|v| !v.is_empty());
            }
        }
    }
    None
}

/// O canal de desenvolvimento embutido, dentro de `OUT_DIR` (vazio fora da feature `dev`).
pub const DEV_FILE: &str = "aylis_dev.json";

/// Onde a IDE grava, antes de compilar o crate em `crate_dir` com a feature `dev`, o endereço
/// dela, a chave pública e o segredo do projeto (fora do projeto e do git:
/// `%LOCALAPPDATA%\aylis\dev\<crate>.json`). O `build.rs` embute no app.
pub fn dev_target_path(crate_dir: &Path) -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from).or_else(|| std::env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join("AppData").join("Local")))?.join("aylis")
    } else {
        PathBuf::from(std::env::var_os("HOME")?).join(".local/share/aylis")
    };
    let dir = crate_dir.canonicalize().unwrap_or_else(|_| crate_dir.to_path_buf());
    let key = dir.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/").to_lowercase();
    // FNV-1a: um nome de arquivo estável por pasta.
    let hash = key.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3));
    Some(base.join("dev").join(format!("{hash:016x}.json")))
}

/// A pasta do projeto de um crate: a dele ou a de cima (o código de cada linguagem fica
/// numa subpasta do projeto, como `rust/`).
pub fn project_root(crate_dir: &Path) -> Option<PathBuf> {
    std::iter::once(crate_dir).chain(crate_dir.parent()).find(|d| find_project(d).is_some()).map(Path::to_path_buf)
}

/// Como [`project`], com o projeto em outra pasta (o app casca de um projeto em outra
/// linguagem, gerado em `.aylis/shell`).
pub fn project_at(dir: &Path) {
    let dir = dir.to_path_buf();
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("rode pelo cargo (build.rs)"));
    let project = find_project(&dir).unwrap_or_else(|| panic!("nenhum .aylisproj em {}", dir.display()));
    // O cargo refaz o build quando algo nestas pastas (ou no .aylisproj) muda.
    println!("cargo:rerun-if-changed={}", project.display());
    for sub in PACKED_DIRS {
        let path = dir.join(sub);
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    let bundle = pack(&dir).unwrap_or_else(|e| panic!("não consegui empacotar o projeto: {e}"));
    std::fs::write(out.join(BUNDLE_FILE), bundle).expect("gravar o pacote do projeto");
    let mut used = used_classes(&dir.join("forms"));
    // As classes dos forms dos pacotes de componente (`packages/`).
    for class in used_classes(&dir.join("packages")) {
        if !used.contains(&class) {
            used.push(class);
        }
    }
    // As classes que o código cria pelo nome (`ui.create("MediaCard", ...)`) também:
    // sem elas, o release (que só registra as usadas) criaria nada em silêncio.
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let src = Path::new(&manifest).join("src");
        if src.exists() {
            println!("cargo:rerun-if-changed={}", src.display());
            for class in code_classes(&src) {
                if !used.contains(&class) {
                    used.push(class);
                }
            }
        }
    }
    std::fs::write(out.join(CLASSES_FILE), classes_source(&used)).expect("gravar o registro das classes");

    // O canal de desenvolvimento: só com a feature `dev` do app (a IDE liga ao executar; a
    // publicação nunca). Sem ela, o app publicado não leva endereço nem chave nenhuma.
    let dev = match (std::env::var_os("CARGO_FEATURE_DEV"), std::env::var("CARGO_MANIFEST_DIR")) {
        (Some(_), Ok(manifest)) => match dev_target_path(Path::new(&manifest)) {
            Some(path) => {
                println!("cargo:rerun-if-changed={}", path.display());
                std::fs::read_to_string(&path).unwrap_or_default()
            }
            None => String::new(),
        },
        _ => String::new(),
    };
    std::fs::write(out.join(DEV_FILE), dev).expect("gravar o canal de desenvolvimento");

    let windows = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows");
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc");
    if windows && msvc {
        if let Some(icon) = icon_of(&project) {
            let png = dir.join(&icon);
            println!("cargo:rerun-if-changed={}", png.display());
            // O `.ico` com todos os tamanhos que a IDE gera do ícone (PNG grande ou SVG); sem
            // ele, o PNG direto (até 256×256).
            let generated = dir.join(APP_ICON);
            println!("cargo:rerun-if-changed={}", generated.display());
            let res = match std::fs::read(&generated) {
                Ok(ico) => ico_res(&ico),
                Err(_) => std::fs::read(&png).map_err(|e| e.to_string()).and_then(|png| icon_res(&png)),
            };
            match res {
                Ok(res) => {
                    let path = out.join("icon.res");
                    std::fs::write(&path, res).expect("gravar icon.res");
                    // O link.exe aceita .res direto (converte sozinho).
                    println!("cargo:rustc-link-arg-bins={}", path.display());
                }
                Err(e) => println!("cargo:warning=ícone {icon} ignorado: {e}"),
            }
        }
    }
}

/// As classes do aylis citadas no código Rust da pasta (e subpastas): todo texto entre
/// aspas que é o nome exato de uma classe conhecida (`"MediaCard"`, `"CheckBox"`). Pode
/// sobrar alguma (registrar a mais só custa tamanho); não falta a que o código cria.
///
/// Uma classe que liga uma feature (`Bluetooth`, `Nfc`...) só conta criada de fato
/// (`create("Bluetooth"`, `"class": "Bluetooth"`): a mais, ela traria dependências pesadas
/// para o app, e o nome aparece em outros textos (o nome de um fone "Bluetooth").
pub fn code_classes(src: &Path) -> Vec<String> {
    fn files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                files(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }
    let mut list = Vec::new();
    files(src, &mut list);
    let mut set = std::collections::BTreeSet::new();
    for f in list {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        // Uma leitura por arquivo: procurar cada uma das ~200 classes no texto inteiro
        // custava ~370 ms num projeto de 140 fontes, a cada "Gerar código" (e F12).
        let (quoted, created) = quoted_names(&text);
        for (name, feature) in CLASSES {
            let found = if feature.is_empty() { quoted.contains(name) } else { created.contains(name) };
            if found {
                set.insert(name.to_string());
            }
        }
    }
    set.into_iter().collect()
}

/// Os nomes entre aspas do texto (`"MediaCard"`: letras, números e `_`, com aspas dos dois
/// lados, em qualquer lugar) e os criados (`create("Bluetooth"` e `"class": "Bluetooth"`,
/// com ou sem espaços).
fn quoted_names(text: &str) -> (std::collections::HashSet<&str>, std::collections::HashSet<&str>) {
    let bytes = text.as_bytes();
    let ident = |at: usize| -> Option<(&str, usize)> {
        // `at` é a aspa de abertura: o nome e a posição depois da aspa de fechamento.
        let start = at + 1;
        let len = bytes[start..].iter().take_while(|b| b.is_ascii_alphanumeric() || **b == b'_').count();
        (len > 0 && bytes.get(start + len) == Some(&b'"')).then(|| (&text[start..start + len], start + len + 1))
    };
    let skip_ws = |mut i: usize| {
        while bytes.get(i).is_some_and(|b| b.is_ascii_whitespace()) {
            i += 1;
        }
        i
    };
    let mut quoted = std::collections::HashSet::new();
    let mut created = std::collections::HashSet::new();
    for (i, _) in text.match_indices('"') {
        let Some((name, after)) = ident(i) else { continue };
        quoted.insert(name);
        // `"class": "X"`.
        if name == "class" {
            let j = skip_ws(after);
            if bytes.get(j) == Some(&b':') {
                let k = skip_ws(j + 1);
                if bytes.get(k) == Some(&b'"') {
                    if let Some((target, _)) = ident(k) {
                        created.insert(target);
                    }
                }
            }
        }
    }
    // `create("X"` (com espaços depois do parêntese, se houver).
    for (i, m) in text.match_indices("create(") {
        let k = skip_ws(i + m.len());
        if bytes.get(k) == Some(&b'"') {
            if let Some((target, _)) = ident(k) {
                created.insert(target);
            }
        }
    }
    (quoted, created)
}

/// As classes que aparecem nos forms da pasta (e subpastas), sem repetir, em ordem. Um
/// subtipo (`"Button.icon"`) conta como a classe (`"Button"`). O `Form` sempre entra.
pub fn used_classes(forms: &Path) -> Vec<String> {
    fn walk(value: &serde_json::Value, out: &mut std::collections::BTreeSet<String>) {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(class) = map.get("class").and_then(|c| c.as_str()) {
                    let base = class.split('.').next().unwrap_or(class);
                    out.insert(base.to_string());
                }
                for v in map.values() {
                    walk(v, out);
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|v| walk(v, out)),
            _ => {}
        }
    }
    fn files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                files(&p, out);
            } else if p.extension().is_some_and(|x| x == "json") {
                out.push(p);
            }
        }
    }
    let mut set = std::collections::BTreeSet::new();
    set.insert("Form".to_string());
    let mut list = Vec::new();
    files(forms, &mut list);
    for f in list {
        if let Ok(Ok(value)) = std::fs::read_to_string(&f).map(|t| serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}'))) {
            walk(&value, &mut set);
        }
    }
    set.into_iter().collect()
}

/// O código Rust do registro das classes usadas (o `aylis_classes.rs`). Classes que não são
/// do aylis (do próprio app) ficam num comentário: o app as registra.
pub fn classes_source(used: &[String]) -> String {
    let mut known = Vec::new();
    let mut custom = Vec::new();
    for class in used {
        match CLASSES.iter().find(|(name, _)| name == class) {
            Some((name, _)) => known.push(*name),
            None => custom.push(class.as_str()),
        }
    }
    // Uma expressão (bloco que define a função e a devolve), para o `include!` dentro do
    // `include_project!()`.
    let mut out = String::from(
        "// Gerado pelo aylis-build: registra só as classes usadas pelos forms do projeto.\n\
         {\n\
         fn aylis_register_used(ui: &mut ::aylis::core::Ui) {\n",
    );
    for name in &known {
        out.push_str(&format!("    ::aylis::classes::{name}(ui);\n"));
    }
    if !custom.is_empty() {
        out.push_str(&format!("    // classes do app (registradas pelo próprio app): {}\n", custom.join(", ")));
    }
    out.push_str("}\naylis_register_used\n}\n");
    out
}

/// A feature do crate `aylis` que uma classe exige ("" = nenhuma); `None` = não é do aylis.
pub fn feature_of(class: &str) -> Option<&'static str> {
    CLASSES.iter().find(|(name, _)| *name == class).map(|(_, f)| *f)
}

/// O `.aylisproj` da pasta (o primeiro em ordem alfabética, se houver mais de um).
pub fn find_project(dir: &Path) -> Option<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "aylisproj"))
        .collect();
    found.sort();
    found.into_iter().next()
}

/// O `.ico` do Windows que a IDE gera do ícone do projeto (todos os tamanhos).
pub const APP_ICON: &str = ".aylis/app-icon.ico";

/// O ícone declarado no `.aylisproj` (`publish.icon`).
fn icon_of(project: &Path) -> Option<String> {
    let text = std::fs::read_to_string(project).ok()?;
    let value: serde_json::Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).ok()?;
    value["publish"]["icon"].as_str().filter(|s| !s.is_empty()).map(str::to_string)
}

// ---------------------------------------------------------------------------
// Pacote

/// Empacota o `.aylisproj` e as pastas [`PACKED_DIRS`] de `dir`. Os nomes ficam relativos
/// à pasta, com `/` (`forms/clientes.json`); o `.aylisproj` vai como está.
pub fn pack(dir: &Path) -> Result<Vec<u8>, String> {
    let mut files = Vec::new();
    if let Some(project) = find_project(dir) {
        let name = project.file_name().and_then(|n| n.to_str()).ok_or("nome do .aylisproj")?.to_string();
        files.push((name, project));
    }
    for sub in PACKED_DIRS {
        collect(&dir.join(sub), sub, &mut files)?;
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = Vec::from(&MAGIC[..]);
    out.extend_from_slice(&(files.len() as u32).to_le_bytes());
    for (name, path) in files {
        let data = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        out.extend_from_slice(&(name.len() as u32).to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&(data.len() as u64).to_le_bytes());
        out.extend_from_slice(&data);
    }
    Ok(out)
}

fn collect(dir: &Path, prefix: &str, files: &mut Vec<(String, PathBuf)>) -> Result<(), String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Ok(()) };
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        // Arquivos ocultos e temporários de editores ficam de fora.
        if name.starts_with('.') || name.ends_with('~') {
            continue;
        }
        let relative = format!("{prefix}/{name}");
        // O crate de um pacote de código (`packages/<Nome>/crate`) é compilado no app, não
        // embutido (só o `package.json` dele vai).
        if path.is_dir() && name == "crate" && prefix.starts_with("packages/") && prefix.matches('/').count() == 1 {
            continue;
        }
        if path.is_dir() {
            collect(&path, &relative, files)?;
        } else {
            files.push((relative, path));
        }
    }
    Ok(())
}

/// Lê um pacote feito por [`pack`]: (nome, conteúdo) de cada arquivo.
pub fn unpack(bytes: &[u8]) -> Result<Vec<(String, &[u8])>, String> {
    let bad = || "pacote do projeto inválido".to_string();
    let rest = bytes.strip_prefix(&MAGIC[..]).ok_or_else(bad)?;
    let (count, mut rest) = take_u32(rest).ok_or_else(bad)?;
    let mut files = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let (len, r) = take_u32(rest).ok_or_else(bad)?;
        let (name, r) = r.split_at_checked(len as usize).ok_or_else(bad)?;
        let name = std::str::from_utf8(name).map_err(|_| bad())?.to_string();
        let (len, r) = r.split_at_checked(8).ok_or_else(bad)?;
        let len = u64::from_le_bytes(len.try_into().expect("8 bytes")) as usize;
        let (data, r) = r.split_at_checked(len).ok_or_else(bad)?;
        files.push((name, data));
        rest = r;
    }
    Ok(files)
}

fn take_u32(bytes: &[u8]) -> Option<(u32, &[u8])> {
    let (n, rest) = bytes.split_at_checked(4)?;
    Some((u32::from_le_bytes(n.try_into().ok()?), rest))
}

// ---------------------------------------------------------------------------
// Ícone do Windows

/// Tamanho de um PNG (do cabeçalho IHDR).
pub fn png_size(png: &[u8]) -> Option<(u32, u32)> {
    if png.len() < 24 || &png[..8] != b"\x89PNG\r\n\x1a\n" || &png[12..16] != b"IHDR" {
        return None;
    }
    let w = u32::from_be_bytes(png[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(png[20..24].try_into().ok()?);
    Some((w, h))
}

/// Um arquivo `.ico` com o PNG dentro (o Windows aceita PNG em ícones desde o Vista).
pub fn ico(png: &[u8]) -> Result<Vec<u8>, String> {
    let (w, h) = check_icon(png)?;
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0, 1, 0]); // ICONDIR: reservado, tipo 1 (ícone), 1 imagem
    out.extend_from_slice(&[dim(w), dim(h), 0, 0]);
    out.extend_from_slice(&1u16.to_le_bytes()); // planos
    out.extend_from_slice(&32u16.to_le_bytes()); // bits por pixel
    out.extend_from_slice(&(png.len() as u32).to_le_bytes());
    out.extend_from_slice(&22u32.to_le_bytes()); // a imagem começa depois dos 6 + 16 bytes
    out.extend_from_slice(png);
    Ok(out)
}

fn check_icon(png: &[u8]) -> Result<(u32, u32), String> {
    let (w, h) = png_size(png).ok_or("não é um PNG")?;
    if w > 256 || h > 256 {
        return Err(format!("o ícone tem {w}×{h}; o Windows aceita até 256×256"));
    }
    Ok((w, h))
}

/// Largura ou altura no formato do ícone (256 vira 0).
fn dim(n: u32) -> u8 {
    if n >= 256 { 0 } else { n as u8 }
}

/// Um arquivo de recursos (`.res`) com o ícone do aplicativo: `RT_ICON` 1 e `RT_GROUP_ICON`
/// 1 (o primeiro grupo é o ícone que o Explorer mostra).
pub fn icon_res(png: &[u8]) -> Result<Vec<u8>, String> {
    const RT_ICON: u16 = 3;
    const RT_GROUP_ICON: u16 = 14;
    let (w, h) = check_icon(png)?;
    let mut group = Vec::new();
    group.extend_from_slice(&[0, 0, 1, 0, 1, 0]); // GRPICONDIR
    group.extend_from_slice(&[dim(w), dim(h), 0, 0]);
    group.extend_from_slice(&1u16.to_le_bytes());
    group.extend_from_slice(&32u16.to_le_bytes());
    group.extend_from_slice(&(png.len() as u32).to_le_bytes());
    group.extend_from_slice(&1u16.to_le_bytes()); // id do RT_ICON
    let mut out = Vec::new();
    // Todo .res começa com uma entrada vazia (assim o linker reconhece o formato de 32 bits).
    resource(&mut out, 0, 0, 0, &[]);
    resource(&mut out, RT_ICON, 1, 0x1010, png);
    resource(&mut out, RT_GROUP_ICON, 1, 0x1030, &group);
    Ok(out)
}

/// As imagens de um arquivo `.ico`: (largura, altura, bytes da imagem — PNG ou BMP).
pub fn ico_images(ico: &[u8]) -> Result<Vec<(u32, u32, &[u8])>, String> {
    let u16_at = |i: usize| ico.get(i..i + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let u32_at = |i: usize| ico.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if u16_at(0) != Some(0) || u16_at(2) != Some(1) {
        return Err("não é um arquivo .ico".into());
    }
    let count = u16_at(4).unwrap_or(0) as usize;
    (0..count)
        .map(|i| {
            let entry = 6 + 16 * i;
            let size = |b: u8| if b == 0 { 256 } else { b as u32 };
            let (w, h) = (size(*ico.get(entry).ok_or("ícone truncado")?), size(*ico.get(entry + 1).ok_or("ícone truncado")?));
            let len = u32_at(entry + 8).ok_or("ícone truncado")? as usize;
            let offset = u32_at(entry + 12).ok_or("ícone truncado")? as usize;
            let data = ico.get(offset..offset + len).ok_or("ícone truncado")?;
            Ok((w, h, data))
        })
        .collect()
}

/// A maior imagem PNG de um `.ico` (o ícone da janela, que pede PNG).
pub fn ico_largest_png(ico: &[u8]) -> Option<&[u8]> {
    ico_images(ico).ok()?.into_iter().filter(|(_, _, d)| png_size(d).is_some()).max_by_key(|(w, h, _)| w * h).map(|(_, _, d)| d)
}

/// O `.res` de um `.ico` com todos os tamanhos (o Explorer escolhe o de cada lugar).
pub fn ico_res(ico: &[u8]) -> Result<Vec<u8>, String> {
    const RT_ICON: u16 = 3;
    const RT_GROUP_ICON: u16 = 14;
    let images = ico_images(ico)?;
    let mut group = Vec::new();
    group.extend_from_slice(&[0, 0, 1, 0]); // GRPICONDIR: reservado, tipo 1
    group.extend_from_slice(&(images.len() as u16).to_le_bytes());
    let mut out = Vec::new();
    resource(&mut out, 0, 0, 0, &[]);
    for (i, (w, h, data)) in images.iter().enumerate() {
        let id = i as u16 + 1;
        let entry = 6 + 16 * i;
        // A entrada do grupo é a do .ico (14 bytes) com o id no lugar do deslocamento.
        group.extend_from_slice(&[dim(*w), dim(*h)]);
        group.extend_from_slice(&ico[entry + 2..entry + 12]);
        group.extend_from_slice(&id.to_le_bytes());
        resource(&mut out, RT_ICON, id, 0x1010, data);
    }
    resource(&mut out, RT_GROUP_ICON, 1, 0x1030, &group);
    Ok(out)
}

/// Uma entrada do `.res` com tipo e nome numéricos.
fn resource(out: &mut Vec<u8>, kind: u16, name: u16, flags: u16, data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_le_bytes()); // DataSize
    out.extend_from_slice(&32u32.to_le_bytes()); // HeaderSize
    for id in [kind, name] {
        out.extend_from_slice(&0xFFFFu16.to_le_bytes());
        out.extend_from_slice(&id.to_le_bytes());
    }
    out.extend_from_slice(&0u32.to_le_bytes()); // DataVersion
    out.extend_from_slice(&flags.to_le_bytes()); // MemoryFlags
    out.extend_from_slice(&0u16.to_le_bytes()); // LanguageId (neutro)
    out.extend_from_slice(&0u32.to_le_bytes()); // Version
    out.extend_from_slice(&0u32.to_le_bytes()); // Characteristics
    out.extend_from_slice(data);
    while out.len() % 4 != 0 {
        out.push(0);
    }
}

/// As classes do `classes! { ... }` do crate `aylis` (o texto de `crates/aylis/src/lib.rs`) com a
/// categoria da paleta de cada uma, na ordem (uma por linha:
/// `Button: aylis_controls::Button => "Standard",`). As demonstrações da documentação
/// (`aylis-demos`) fazem um wasm por categoria.
pub fn class_categories(facade: &str) -> Vec<(&str, &str)> {
    let body = facade.split_once("classes! {\n").map_or("", |(_, b)| b);
    let mut out: Vec<(&str, &str)> = body
        .lines()
        .take_while(|l| l.trim() != "}")
        .filter_map(|line| {
            let line = line.trim();
            let line = if line.starts_with("#[") { line.split_once("] ").map_or(line, |(_, rest)| rest) } else { line };
            let (name, rest) = line.split_once(':')?;
            let category = rest.split_once("=> \"")?.1.split('"').next()?;
            Some((name.trim(), category))
        })
        .collect();
    // As que têm função própria depois da lista (com subtipos, como o `ElectricalSymbol`):
    // `pub fn Nome(ui: &mut Ui) {` e, na linha de baixo, `ui.register_class::<...>("Categoria")`.
    let mut lines = body.lines().skip_while(|l| l.trim() != "}").peekable();
    while let Some(line) = lines.next() {
        let Some(name) = line.trim().strip_prefix("pub fn ").and_then(|r| r.strip_suffix("(ui: &mut Ui) {")) else { continue };
        let category = lines.peek().and_then(|next| next.split_once(">(\"")).and_then(|(_, r)| r.split('"').next());
        if let Some(category) = category {
            out.push((name, category));
        }
    }
    out
}

/// Para o `build.rs` de uma página com a documentação ao vivo (a Landing): grava em
/// `OUT_DIR/demo_classes.rs` a função `register(ui)` com as classes das famílias `categories`
/// (as categorias da paleta) que rodam no navegador e não pedem feature do `aylis`. A página
/// as mostra no próprio wasm, sem baixar o wasm da família (o núcleo viria de novo); as outras
/// famílias continuam no `aylis-demos`. `facade` é o `crates/aylis/src/lib.rs`.
pub fn demo_classes(facade: &Path, categories: &[&str]) {
    println!("cargo:rerun-if-changed={}", facade.display());
    let text = std::fs::read_to_string(facade).unwrap_or_else(|e| panic!("ler {}: {e}", facade.display()));
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("rode pelo cargo (build.rs)"));
    std::fs::write(out.join("demo_classes.rs"), demo_classes_source(&text, categories)).expect("gravar demo_classes.rs");
}

/// O código de [`demo_classes`].
pub fn demo_classes_source(facade: &str, categories: &[&str]) -> String {
    let mut out = String::from("// Gerado pelo aylis-build: as classes das famílias da documentação ao vivo.\npub fn register(ui: &mut ::aylis::core::Ui) {\n    let _ = &ui;\n");
    for (class, category) in class_categories(facade) {
        if categories.contains(&category) && feature_of(class) == Some("") && demo_on_web(class) {
            out.push_str(&format!("    ::aylis::classes::{class}(ui);\n"));
        }
    }
    out.push_str("}\n");
    out
}

/// A feature do `aylis-demos` que liga o que a classe pede no `aylis` (`None`: nenhuma).
pub fn demo_feature(class: &str) -> Option<&'static str> {
    match CLASSES.iter().find(|(c, _)| *c == class).map_or("", |(_, f)| *f) {
        "data" | "dbcontrols" | "providers" => Some("data"),
        "media" => Some("media"),
        "vector" => Some("vector"),
        "markdown" => Some("markdown"),
        "docview" => Some("docview"),
        "3d" => Some("3d"),
        _ => None,
    }
}

/// A classe pode ser mostrada nas demonstrações do navegador: roda na web e não depende de uma
/// feature do `aylis` que não existe lá (bandeja, MCP, NFC e Bluetooth do desktop, WebView).
pub fn demo_on_web(class: &str) -> bool {
    let feature = CLASSES.iter().find(|(c, _)| *c == class).map_or("", |(_, f)| *f);
    let web = class_platforms(class).is_none_or(|p| p.contains(&Platform::Web));
    web && (feature.is_empty() || demo_feature(class).is_some())
}

#[cfg(test)]
mod metadata_tests {
    #[test]
    fn the_project_folder_comes_from_the_cargo_metadata() {
        let cargo = "[package]\nname = \"console\"\n\n[package.metadata.aylis]\nproject = \"../app\"\n\n[dependencies]\nproject = \"x\"\n";
        assert_eq!(super::metadata_project(cargo).as_deref(), Some("../app"));
        assert_eq!(super::metadata_project("[package]\nname = \"a\"\n"), None);
    }
}
