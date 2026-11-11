//! # aylis-ide
//!
//! A IDE do Aylis, feita com o próprio aylis: projetos com vários forms (gerenciador de
//! projeto e abas), paleta, form designer (seleção múltipla, alinhar, guias, zoom),
//! estrutura, Object Inspector (propriedades e eventos, com editores por tipo), menus e
//! atalhos, desfazer/refazer, copiar/colar, geração e abertura do código dos eventos,
//! execução (desktop, Android, navegador, iOS) com a lista de erros clicável, verificação
//! do ambiente e publicação.

pub mod actions;
pub mod app;
pub mod assetdialogs;
pub mod code_sync;
pub mod codegen;
pub mod codewindow;
pub mod androidview;
pub mod windowsview;
pub mod grammars;
pub mod reportview;
pub mod iconview;
pub mod packagesview;
pub mod columnsview;
pub mod debugger;
pub mod devices;
#[cfg(feature = "desktop")]
pub mod devlink;
#[cfg(not(feature = "desktop"))]
#[path = "devlink_lite.rs"]
pub mod devlink;
pub mod dialogs;
pub mod document;
/// O guarda que recusa o agente gravando direto num projeto aberto na IDE.
#[cfg(feature = "desktop")]
pub mod edit_guard;
pub mod fieldsview;
pub mod files;
pub mod gallery;
pub mod home;
pub mod host;
pub mod hovertip;
pub mod inspector;
pub mod instance;
pub mod intel;
pub mod jobs;
pub mod lang;
pub mod lite;
pub mod live;
pub mod profiler;
pub mod mcp;
pub mod messages;
pub mod menuview;
pub mod nonvisual;
pub mod options;
pub mod panels;
pub mod projectsearch;
pub mod usages;
pub mod rename;
pub mod projectsettings;
pub mod propeditors;
pub mod recordings;
#[cfg(feature = "desktop")]
pub mod reference;
pub mod recovery;
pub mod splash;
pub mod state;
pub mod surface;
pub mod taborder;
pub mod tooldialogs;
pub mod trust;
pub mod updates;
pub mod viewsdialog;
pub mod widgets;
pub mod wizard;
