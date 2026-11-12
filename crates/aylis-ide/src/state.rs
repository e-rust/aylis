//! Estado compartilhado entre os painéis da IDE.

use std::cell::RefCell;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::rc::Rc;

use aylis_project::Project;

use crate::document::Document;

pub struct State {
    /// O form da aba ativa (o que o designer mostra).
    pub doc: Document,
    /// Todas as abas, na ordem; a ativa está em `doc` (aqui fica `None` no lugar dela).
    pub tabs: Vec<Option<Document>>,
    pub active: usize,
    /// A última aba foi fechada: nenhum form aberto (o `doc` vazio fica só por dentro, sem
    /// aba nem form no designer, até abrir outro).
    pub no_form: bool,
    /// Projeto aberto (sem projeto, a IDE edita forms soltos).
    pub project: Option<Project>,
    /// O projeto aberto é de fora e ainda não foi confiado: nada dele roda (veja `trust.rs`).
    pub restricted: bool,
    /// Classe escolhida na paleta, esperando o clique no form.
    pub pending: Option<String>,
    /// Object Inspector mostrando eventos (em vez de propriedades).
    pub events_tab: bool,
    /// Busca no Object Inspector e na paleta.
    pub inspector_filter: String,
    /// De quem o Object Inspector mostrava as propriedades: trocar limpa a busca.
    pub inspector_for: Vec<aylis_core::ComponentId>,
    pub palette_filter: String,
    /// Classes abertas na paleta (mostrando os subtipos).
    pub palette_open: std::collections::HashSet<String>,
    /// Os grupos do Object Inspector recolhidos (Identidade, Posição e tamanho...), pela
    /// categoria: valem para todos os componentes, como no Object Inspector do Delphi.
    pub inspector_closed: std::collections::HashSet<String>,
    /// A aba da paleta: "aylis" ou uma plataforma ("Windows", "Apple", "Android").
    pub palette_tab: String,
    /// Itens de menu e botões que só valem com um projeto aberto (desabilitados sem).
    pub project_only: Vec<aylis_core::ComponentId>,
    /// Zoom do designer (1.0 = 100%).
    pub zoom: f32,
    /// Mensagem da barra de status.
    pub status: String,
    /// Mensagens do "Executar" (compilação e saída do programa).
    pub output: Vec<crate::jobs::OutputLine>,
    /// Muda a cada alteração das mensagens (o painel só é refeito quando muda).
    pub output_version: u64,
    pub output_shown: std::cell::Cell<u64>,
    /// Projetos e forms abertos recentemente (o mais recente primeiro).
    pub recent: Vec<PathBuf>,
    /// A tela inicial pode aparecer (sem projeto e com o form vazio de partida; o "Novo form
    /// solto" a desliga, fechar o projeto a liga de novo).
    pub home: bool,
    /// O centro mostra o código embutido (senão, o designer).
    pub code_view: bool,
    /// Idioma escolhido para a IDE (`""`: o do sistema).
    pub language: String,
    /// Diálogos do sistema em andamento (ver [`crate::actions::spawn`]).
    pub tasks: Vec<Task>,
    /// Acorda a interface para ela avançar as tarefas (só existe com a janela aberta).
    pub waker: Option<std::task::Waker>,
    /// O que uma tarefa quer fazer no `Ui` quando termina (ela mesma não tem acesso a ele).
    pub after_task: Option<AfterTask>,
    /// Executar/Publicar em andamento.
    pub job: Option<crate::jobs::Job>,
    /// Verificação ou correção do ambiente em andamento.
    pub env_job: Option<crate::jobs::Job>,
    /// Acorda a interface quando chega andamento de um trabalho (de outra thread).
    pub output_waker: Option<aylis_platform_winit::Waker>,
    /// Plataforma do Executar (F9).
    pub run_target: aylis_tools::Target,
    /// Os aparelhos Android que o adb vê (atualizados sozinhos enquanto a saída é Android) e
    /// o escolhido para o Executar.
    pub devices: Vec<aylis_tools::env::AndroidDevice>,
    pub run_device: Option<String>,
    pub device_watch: Option<std::sync::Arc<crate::devices::Watch>>,
    /// Última verificação do ambiente.
    pub checks: Vec<aylis_tools::env::Check>,
    /// Diálogo "Verificar ambiente" aberto e a última linha da correção em andamento.
    pub env_view: Option<crate::tooldialogs::EnvView>,
    pub env_log: String,
    /// O assistente "Configurar linguagens" aberto.
    pub languages_view: Option<crate::tooldialogs::LanguagesView>,
    /// As linhas da instalação das linguagens (o log do assistente).
    pub setup_log: Vec<String>,
    /// Cada linguagem, se o compilador está instalado e o que ainda falta (o depurador, o
    /// servidor de linguagem...). Vazio: ainda não verificou.
    pub language_status: Vec<(aylis_project::Language, bool, Vec<String>)>,
    /// A IDE acabou de abrir: se nenhuma linguagem estiver instalada, abre o assistente.
    pub wizard_on_start: bool,
    /// O canal de desenvolvimento: o app que a IDE executa conecta nele (desktop, celular,
    /// navegador); o inspetor, o hot reload, o rastro e o MCP passam por ele.
    pub dev: Option<std::sync::Arc<crate::devlink::DevServer>>,
    /// O servidor MCP (agentes de IA comandando a IDE), quando ligado.
    pub mcp: Option<crate::mcp::McpServer>,
    /// As sessões de agente ligadas a esta IDE pelo MCP (a ponte avisa), na ordem em que
    /// entraram: a marca da sessão, o programa cliente e o título que o agente deu à conversa.
    /// Aparecem na barra da janela; mais de uma quando várias trabalham no mesmo projeto.
    pub agents: Vec<(String, String, String)>,
    /// O último aviso de letras de `&` repetidas no form aberto (só avisa quando muda).
    pub mnemonic_warning: String,
    /// O servidor de linguagem do editor de código (sugestões, definição, erros).
    pub intel: crate::intel::Intel,
    /// Pontos de parada: arquivo (caminho normalizado) → linhas a partir de 1.
    pub breakpoints: std::collections::BTreeMap<std::path::PathBuf, std::collections::BTreeSet<usize>>,
    /// Condição e contagem de passagens de cada ponto de parada que tiver (arquivo, linha).
    pub bp_conditions: std::collections::BTreeMap<(std::path::PathBuf, usize), (String, String)>,
    /// A depuração dentro da IDE.
    pub debugger: crate::debugger::Debugger,
    /// Executar com o rastro de eventos (cada evento com handler vai para Mensagens).
    pub trace_events: bool,
    /// Painel de mensagens aberto.
    pub show_messages: bool,
    /// Exibir → Mostrar componentes escondidos (vale para todos os forms abertos).
    pub show_hidden: bool,
    /// Propriedades expandidas no Object Inspector (margins, padding).
    pub expanded: std::collections::HashSet<String>,
    /// Pedido de fechar a IDE já confirmado (não pergunta de novo).
    pub quit_confirmed: bool,
    /// Já avisou que há forms alterados ao fechar ou trocar de projeto: a segunda vez descarta.
    pub leave_warned: bool,
    /// Form da linha do gerenciador de projeto em que o menu de contexto foi aberto.
    pub context_form: Option<String>,
    /// As pastas abertas no painel do projeto (o painel é refeito a cada mudança): começam
    /// recolhidas, e abre-se o caminho até o item atual quando ele muda.
    pub project_tree_open: std::collections::HashSet<String>,
    /// O item atual da última vez (o form no designer ou o fonte no editor).
    pub project_tree_current: String,
    /// O que o Object Inspector e a paleta mostram (um resumo): igual, não são refeitos.
    pub inspector_key: u64,
    pub palette_key: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            doc: Document::new(),
            tabs: vec![None],
            no_form: false,
            active: 0,
            project: None,
            restricted: false,
            pending: None,
            events_tab: false,
            inspector_filter: String::new(),
            inspector_for: Vec::new(),
            palette_filter: String::new(),
            palette_open: Default::default(),
            inspector_closed: Default::default(),
            palette_tab: "aylis".into(),
            project_only: Vec::new(),
            zoom: 1.0,
            status: String::new(),
            output: Vec::new(),
            output_version: 0,
            output_shown: std::cell::Cell::new(u64::MAX),
            recent: Vec::new(),
            home: true,
            code_view: false,
            language: String::new(),
            tasks: Vec::new(),
            waker: None,
            after_task: None,
            job: None,
            env_job: None,
            output_waker: None,
            run_target: aylis_tools::Target::Desktop,
            devices: Vec::new(),
            run_device: None,
            device_watch: None,
            checks: Vec::new(),
            env_view: None,
            env_log: String::new(),
            languages_view: None,
            setup_log: Vec::new(),
            language_status: Vec::new(),
            wizard_on_start: false,
            dev: None,
            mcp: None,
            agents: Vec::new(),
            mnemonic_warning: String::new(),
            intel: crate::intel::Intel::default(),
            breakpoints: Default::default(),
            bp_conditions: Default::default(),
            debugger: Default::default(),
            trace_events: false,
            show_messages: false,
            show_hidden: false,
            expanded: Default::default(),
            quit_confirmed: false,
            leave_warned: false,
            context_form: None,
            project_tree_open: Default::default(),
            project_tree_current: String::new(),
            inspector_key: 0,
            palette_key: 0,
        }
    }
}

/// O que fazer no `Ui` quando uma tarefa termina.
pub type AfterTask = Box<dyn FnOnce(&mut aylis_core::Ui)>;

/// Uma tarefa que roda na thread da interface (um diálogo do sistema e o que fazer com a
/// resposta).
pub type Task = Pin<Box<dyn Future<Output = ()>>>;

pub type Shared = Rc<RefCell<State>>;

pub fn shared(doc: Document) -> Shared {
    Rc::new(RefCell::new(State { doc, ..Default::default() }))
}

impl State {
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Documento da aba `i` (a ativa vem de `doc`).
    pub fn tab(&self, i: usize) -> Option<&Document> {
        if i == self.active { Some(&self.doc) } else { self.tabs.get(i)?.as_ref() }
    }

    /// Torna a aba `i` a ativa.
    pub fn switch_to(&mut self, i: usize) {
        if i == self.active || i >= self.tabs.len() {
            return;
        }
        let Some(next) = self.tabs[i].take() else { return };
        let current = std::mem::replace(&mut self.doc, next);
        self.tabs[self.active] = Some(current);
        self.active = i;
        self.pending = None;
    }

    /// Abre um documento numa aba nova e ativa ela. Com uma só aba vazia e intocada
    /// (o `Form1` do início), a nova toma o lugar dela.
    pub fn open_tab(&mut self, doc: Document) -> usize {
        let blank = self.tabs.len() == 1 && (self.no_form || self.doc.path.is_none() && !self.doc.modified);
        if blank {
            self.no_form = false;
            self.doc = doc;
            return 0;
        }
        self.tabs.push(Some(doc));
        let index = self.tabs.len() - 1;
        self.switch_to(index);
        index
    }

    /// Fecha a aba `i` (se for a última, fica sem form aberto: [`State::no_form`]). Devolve o
    /// documento fechado.
    pub fn close_tab(&mut self, i: usize) -> Option<Document> {
        if i >= self.tabs.len() {
            return None;
        }
        if self.tabs.len() == 1 {
            if self.no_form {
                return None;
            }
            // A última: nenhum form aberto (antes aparecia um Form1 novo no lugar).
            let dark = self.doc.dark;
            let mut fresh = Document::new();
            fresh.set_dark(dark);
            self.no_form = true;
            return Some(std::mem::replace(&mut self.doc, fresh));
        }
        if i == self.active {
            let next = if i + 1 < self.tabs.len() { i + 1 } else { i - 1 };
            self.switch_to(next);
        }
        let closed = self.tabs.remove(i);
        if self.active > i {
            self.active -= 1;
        }
        closed
    }

    /// Fecha todas as abas (ao trocar de projeto).
    pub fn close_all(&mut self) {
        let dark = self.doc.dark;
        self.doc = Document::new();
        self.doc.set_dark(dark);
        self.tabs = vec![None];
        self.active = 0;
        self.no_form = false;
    }

    /// Aba do form `name`, se estiver aberto.
    /// A aba do arquivo `path` (as variantes de um form têm o mesmo nome e arquivos
    /// diferentes).
    pub fn find_tab_path(&self, path: &std::path::Path) -> Option<usize> {
        (0..self.tabs.len()).find(|&i| self.tab(i).is_some_and(|d| d.path.as_deref() == Some(path)))
    }

    pub fn find_tab(&self, form: &str) -> Option<usize> {
        (0..self.tabs.len()).find(|&i| self.tab(i).is_some_and(|d| d.form_name().eq_ignore_ascii_case(form)))
    }

    /// Algum form aberto tem alterações não salvas.
    pub fn any_modified(&self) -> bool {
        (0..self.tabs.len()).any(|i| self.tab(i).is_some_and(|d| d.modified))
    }

    /// Guarda nos recentes (o mais recente primeiro, sem repetir, até 10).
    pub fn remember(&mut self, path: PathBuf) {
        // O mesmo arquivo escrito de outro jeito (com "..", outra caixa) vale uma entrada só.
        let path = crate::debugger::norm(&path);
        self.recent.retain(|p| crate::debugger::norm(p) != path);
        self.recent.insert(0, path);
        self.recent.truncate(10);
    }

    /// Acrescenta uma linha às mensagens.
    pub fn log(&mut self, line: impl Into<String>) {
        self.push(crate::jobs::OutputLine::new(line, crate::jobs::OutKind::Info));
    }

    /// Acrescenta uma linha às mensagens (mantém as últimas 2000).
    pub fn push(&mut self, line: crate::jobs::OutputLine) {
        self.output.push(line);
        let excess = self.output.len().saturating_sub(2000);
        self.output.drain(..excess);
        self.output_version += 1;
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
        self.output_version += 1;
    }
}
