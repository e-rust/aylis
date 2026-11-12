//! Os eventos do form frmDocumentacao: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).
//!
//! A documentação ocupa a página inteira (o form maximizado), no jeito dos sites de
//! documentação: no topo, a busca e a volta ao site; à esquerda, as seções em árvore (cada uma
//! abre e fecha, com a página aberta marcada); no meio, a trilha, a página e "Anterior" /
//! "Próxima"; à direita, "Nesta página" (os títulos da página, que levam até eles).
//!
//! O conteúdo vem de `assets/docs` (`aylis-tool docs` refaz: os guias de `docs/` e a referência
//! de cada componente); o `indice.json` diz as seções e as páginas.

use aylis::controls::{TreeNode, TreeView};
use aylis::prelude::*;
use serde_json::Value;

use crate::generated::FrmDocumentacao;

/// A pasta da documentação, dentro do projeto.
const DOCS: &str = "assets/docs/";

/// Uma página do índice.
#[derive(Clone, Default)]
struct Pagina {
    secao: String,
    titulo: String,
    arquivo: String,
}

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {
    /// As seções do índice, na ordem, com as páginas de cada uma.
    secoes: Vec<(String, Vec<Pagina>)>,
    /// Todas as páginas na ordem do índice ("Anterior" e "Próxima" andam por ela).
    ordem: Vec<Pagina>,
    /// O texto de cada página, em minúsculas (a busca olha também dentro), lido na primeira
    /// busca.
    textos: Vec<String>,
    /// Os achados da busca, na ordem da lista.
    achados: Vec<Pagina>,
    /// O que foi buscado (destacado na página aberta pelos achados).
    busca: String,
    /// O texto a destacar na próxima página aberta (só a aberta por um achado da busca).
    destaque: String,
    /// Os títulos da página aberta ("Nesta página"), na ordem da lista.
    titulos: Vec<String>,
    /// O arquivo aberto (para os links relativos).
    atual: String,
    /// As páginas por onde a pessoa passou (o arquivo e o título), para voltar e avançar com
    /// os botões extras do mouse ou Alt+← e Alt+→ (`navegar`), e a posição atual nele.
    historico: Vec<(String, String)>,
    posicao: usize,
    /// `abrir` chamado por `navegar`: não mexe no histórico.
    navegando: bool,
    /// A demonstração ao vivo montada na própria página (sai ao abrir outra).
    demo: Option<ComponentId>,
}

impl FrmDocumentacao {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let indice: Value = ler(ui, "indice.json").and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(Value::Null);
        let texto = |v: &Value, k: &str| v[k].as_str().unwrap_or_default().to_string();
        self.data.secoes = indice["secoes"]
            .as_array()
            .map(|s| {
                s.iter()
                    .map(|secao| {
                        let nome = sem_marcas(&texto(secao, "secao"));
                        let paginas = secao["paginas"]
                            .as_array()
                            .map(|p| p.iter().map(|p| Pagina { secao: nome.clone(), titulo: sem_marcas(&texto(p, "titulo")), arquivo: texto(p, "arquivo") }).collect())
                            .unwrap_or_default();
                        (nome, paginas)
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.data.ordem = self.data.secoes.iter().flat_map(|(_, p)| p.iter().cloned()).collect();
        self.abrir(ui, "guias/README.md", "");
    }

    /// Abre a página `arquivo` (no título `titulo`, se houver) e acerta o resto da tela: a
    /// árvore (a seção aberta e a página marcada), a trilha, "Nesta página" e os botões.
    fn abrir(&mut self, ui: &mut Ui, arquivo: &str, titulo: &str) {
        if !self.data.navegando {
            let d = &mut self.data;
            // Uma página nova depois de voltar: o "avançar" que havia some, como num navegador.
            d.historico.truncate(if d.historico.is_empty() { 0 } else { d.posicao + 1 });
            if d.historico.last().map(|(a, t)| (a.as_str(), t.as_str())) != Some((arquivo, titulo)) {
                d.historico.push((arquivo.to_string(), titulo.to_string()));
            }
            d.posicao = d.historico.len() - 1;
        }
        self.data.atual = arquivo.to_string();
        // A página de um componente visual: o componente de verdade no lugar da imagem. Das
        // famílias leves, montado aqui mesmo (as classes vêm no wasm da página: veja o
        // `build.rs`); das outras, no navegador, o wasm da família num `<iframe>` (carregado
        // só agora; veja `demo_de`). Sem nenhum dos dois, a imagem.
        if let Some(id) = self.data.demo.take() {
            ui.free(id);
        }
        let texto = ler(ui, arquivo);
        let classe = texto.as_deref().and_then(|t| classe_visual(arquivo, t));
        let na_pagina = classe.and_then(|c| aylis::samples::demo(ui, c, self.pnlDemo.id, self.form.id));
        let no_quadro = if na_pagina.is_none() && cfg!(target_arch = "wasm32") { texto.as_deref().and_then(|t| demo_de(arquivo, t)) } else { None };
        let _ = ui.set_prop(self.pnlDemo.id, "visible", na_pagina.is_some() || no_quadro.is_some());
        let _ = ui.set_prop(self.wbDemo.id, "visible", no_quadro.is_some());
        let _ = ui.set_prop(self.wbDemo.id, "url", no_quadro.clone().unwrap_or_default());
        if let Some((id, (_, altura))) = na_pagina {
            self.data.demo = Some(id);
            let _ = ui.set_prop(self.pnlDemo.id, "height", (altura + RODAPE_DEMO).min(ALTURA_DEMO_MAX));
        } else if no_quadro.is_some() {
            // A altura do quadro: a da imagem do componente (o tamanho dele) até a página da
            // demonstração mandar a dela (`demo_mensagem`).
            let classe = arquivo.trim_start_matches("componentes/").trim_end_matches(".md");
            if let Some((_, altura)) = ui.services().resources.as_ref().and_then(|r| r.read(&format!("{DOCS}componentes/img/{classe}.png"))).as_deref().and_then(tamanho_png) {
                let _ = ui.set_prop(self.pnlDemo.id, "height", (altura as f32 + RODAPE_DEMO).min(ALTURA_DEMO_MAX));
            }
        }
        match texto.filter(|_| na_pagina.is_some() || no_quadro.is_some()) {
            // Com o componente ao vivo, a página sem a imagem dele.
            Some(texto) => {
                let sem_imagem: Vec<&str> = texto.lines().filter(|l| !(l.starts_with("![") && l.contains("](img/"))).collect();
                let _ = ui.set_prop(self.mdPagina.id, "source", String::new());
                let _ = ui.set_prop(self.mdPagina.id, "text", sem_imagem.join("\n"));
            }
            None => {
                let _ = ui.set_prop(self.mdPagina.id, "source", format!("{DOCS}{arquivo}"));
            }
        }
        let _ = ui.set_prop(self.mdPagina.id, "anchor", titulo.to_string());
        // O achado da busca: o texto buscado destacado (a página rola até ele); aberta de outro
        // jeito, sem destaque.
        let _ = ui.set_prop(self.mdPagina.id, "highlight", std::mem::take(&mut self.data.destaque));
        let pagina =self.data.ordem.iter().position(|p| p.arquivo == arquivo);
        // A árvore: as seções fechadas, menos a da página aberta, com a página marcada.
        let atual = pagina.map(|i| self.data.ordem[i].clone()).unwrap_or_default();
        let mut marcada = None;
        let nos: Vec<TreeNode> = self
            .data
            .secoes
            .iter()
            .enumerate()
            .map(|(s, (nome, paginas))| {
                let aberta = *nome == atual.secao;
                let filhos = paginas
                    .iter()
                    .enumerate()
                    .map(|(p, pagina)| {
                        if aberta && pagina.arquivo == arquivo {
                            marcada = Some(vec![s, p]);
                        }
                        TreeNode { text: pagina.titulo.clone(), children: Vec::new(), expanded: false, data: pagina.arquivo.clone() }
                    })
                    .collect();
                TreeNode { text: nome.clone(), children: filhos, expanded: aberta, data: String::new() }
            })
            .collect();
        if let Some(tree) = ui.get_mut::<TreeView>(self.tvIndice.id) {
            tree.set_nodes(nos);
            tree.set_selected_path(marcada);
        }
        ui.damage(self.tvIndice.id);
        // A trilha: a seção e a página.
        let trilha = if atual.titulo.is_empty() { "Documentação".to_string() } else { format!("{}  ›  {}", atual.secao, atual.titulo) };
        let _ = ui.set_prop(self.lblTrilha.id, "text", trilha);
        // "Nesta página": os títulos de segundo e terceiro nível (os de terceiro, recuados).
        let texto = ler(ui, arquivo).unwrap_or_default();
        let titulos = titulos_de(&texto);
        let rotulos: Vec<String> = titulos.iter().map(|(nivel, t)| format!("{}{}", if *nivel >= 3 { "    " } else { "" }, t.replace(';', ","))).collect();
        let _ = ui.set_prop(self.lbTitulos.id, "items", rotulos.join(";"));
        self.data.titulos = titulos.into_iter().map(|(_, t)| t).collect();
        // "Anterior" e "Próxima", com o título de cada uma.
        let vizinha = |i: Option<usize>| i.and_then(|i| self.data.ordem.get(i)).map(|p| p.titulo.clone());
        let anterior = vizinha(pagina.and_then(|i| i.checked_sub(1)));
        let proxima = vizinha(pagina.map(|i| i + 1));
        for (botao, alvo, rotulo) in [
            (self.btnAnterior.id, anterior.clone(), anterior.map(|t| format!("‹  Anterior: {t}"))),
            (self.btnProxima.id, proxima.clone(), proxima.map(|t| format!("Próxima: {t}  ›"))),
        ] {
            let _ = ui.set_prop(botao, "visible", alvo.is_some());
            if let Some(rotulo) = rotulo {
                let _ = ui.set_prop(botao, "text", rotulo);
            }
        }
    }

    /// A página da demonstração disse o tamanho natural: o quadro fica na altura dela. Tamanho
    /// zero: a classe não tem demonstração ao vivo (a câmera, um relatório...), e a página
    /// volta a mostrar a imagem.
    pub fn demo_mensagem(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let Some((_, altura)) = args.text().and_then(tamanho_demo) else { return };
        if altura > 0.0 {
            let _ = ui.set_prop(self.pnlDemo.id, "height", (altura + RODAPE_DEMO).min(ALTURA_DEMO_MAX));
        } else {
            let _ = ui.set_prop(self.pnlDemo.id, "visible", false);
            let _ = ui.set_prop(self.wbDemo.id, "url", String::new());
            let _ = ui.set_prop(self.mdPagina.id, "source", format!("{DOCS}{}", self.data.atual));
        }
    }

    pub fn voltar(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ui.close(self.form.id);
    }

    /// Os botões extras do mouse, Alt+← e Alt+→: a página anterior ou a seguinte do
    /// histórico. Voltar da primeira é voltar ao site.
    pub fn navegar(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let posicao = match args.text() {
            Some("forward") if self.data.posicao + 1 < self.data.historico.len() => self.data.posicao + 1,
            Some("back") if self.data.posicao > 0 => self.data.posicao - 1,
            Some("back") => return self.voltar(ui, sender, args),
            _ => return,
        };
        self.data.posicao = posicao;
        let (arquivo, titulo) = self.data.historico[posicao].clone();
        self.data.navegando = true;
        self.abrir(ui, &arquivo, &titulo);
        self.data.navegando = false;
    }

    /// A busca: no título e no texto das páginas; com algo escrito, os achados ficam no lugar
    /// da árvore.
    pub fn buscar(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let busca = args.text().unwrap_or_default().trim().to_lowercase();
        self.data.busca = busca.clone();
        let _ = ui.set_prop(self.tvIndice.id, "visible", busca.is_empty());
        let _ = ui.set_prop(self.lbBusca.id, "visible", !busca.is_empty());
        if busca.is_empty() {
            return;
        }
        if self.data.textos.len() != self.data.ordem.len() {
            self.data.textos = self.data.ordem.iter().map(|p| ler(ui, &p.arquivo).unwrap_or_default().to_lowercase()).collect();
        }
        // Primeiro os que batem no título, depois os que batem só no texto.
        let (mut no_titulo, mut no_texto) = (Vec::new(), Vec::new());
        for (pagina, texto) in self.data.ordem.iter().zip(&self.data.textos) {
            if pagina.titulo.to_lowercase().contains(&busca) {
                no_titulo.push(pagina.clone());
            } else if texto.contains(&busca) {
                no_texto.push(pagina.clone());
            }
        }
        no_titulo.extend(no_texto);
        let rotulos: Vec<String> = no_titulo.iter().map(|p| format!("{} — {}", p.titulo, p.secao).replace(';', ",")).collect();
        let _ = ui.set_prop(self.lbBusca.id, "items", rotulos.join(";"));
        self.data.achados = no_titulo;
    }

    /// Um nó da árvore: uma página abre; uma seção, a primeira página dela.
    pub fn abrir_no(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let arquivo = ui.get::<TreeView>(self.tvIndice.id).and_then(|tree| {
            let no = tree.selected()?;
            Some(if no.data.is_empty() { no.children.first().map(|c| c.data.clone()).unwrap_or_default() } else { no.data.clone() })
        });
        if let Some(arquivo) = arquivo.filter(|a| !a.is_empty() && *a != self.data.atual) {
            self.abrir(ui, &arquivo, "");
        }
    }

    /// Um achado da busca.
    pub fn abrir_achado(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let indice = match ui.get_prop(self.lbBusca.id, "item_index") {
            Ok(PropValue::Int(i)) if i >= 0 => i as usize,
            _ => return,
        };
        if let Some(pagina) = self.data.achados.get(indice).cloned() {
            self.data.destaque = self.data.busca.clone();
            self.abrir(ui, &pagina.arquivo, "");
        }
    }

    /// "Nesta página": rola até o título.
    pub fn ir_para_titulo(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let indice = match ui.get_prop(self.lbTitulos.id, "item_index") {
            Ok(PropValue::Int(i)) if i >= 0 => i as usize,
            _ => return,
        };
        if let Some(titulo) = self.data.titulos.get(indice).cloned() {
            let _ = ui.set_prop(self.mdPagina.id, "anchor", titulo);
        }
    }

    pub fn anterior(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.andar(ui, -1);
    }

    pub fn proxima(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        self.andar(ui, 1);
    }

    fn andar(&mut self, ui: &mut Ui, passo: isize) {
        let Some(i) = self.data.ordem.iter().position(|p| p.arquivo == self.data.atual) else { return };
        if let Some(pagina) = i.checked_add_signed(passo).and_then(|j| self.data.ordem.get(j)).cloned() {
            self.abrir(ui, &pagina.arquivo, "");
        }
    }

    /// Um link da página: um título desta página (`#secao`), outra página da documentação
    /// (relativa à atual, com ou sem `#secao`) ou um site.
    pub fn seguir_link(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let url = args.text().unwrap_or_default().to_string();
        let (alvo, titulo) = url.split_once('#').unwrap_or((url.as_str(), ""));
        if alvo.is_empty() {
            let _ = ui.set_prop(self.mdPagina.id, "anchor", titulo.to_string());
            return;
        }
        if url.contains("://") || !alvo.ends_with(".md") {
            let _ = aylis::controls::AppLink::open_url(ui, &url);
            return;
        }
        let mut partes: Vec<&str> = self.data.atual.rsplit_once('/').map_or(Vec::new(), |(pasta, _)| pasta.split('/').collect());
        for parte in alvo.split('/') {
            match parte {
                ".." => {
                    partes.pop();
                }
                "." | "" => {}
                p => partes.push(p),
            }
        }
        let caminho = partes.join("/");
        self.abrir(ui, &caminho, titulo);
    }
}

/// O que o quadro "Ao vivo" tem além da demonstração (o rótulo em cima e a borda).
const RODAPE_DEMO: f32 = 34.0;
/// A altura máxima do quadro (um componente muito alto rola dentro dele).
const ALTURA_DEMO_MAX: f32 = 620.0;

/// A largura e a altura de um PNG (o cabeçalho `IHDR`).
fn tamanho_png(bytes: &[u8]) -> Option<(u32, u32)> {
    let w = u32::from_be_bytes(bytes.get(16..20)?.try_into().ok()?);
    let h = u32::from_be_bytes(bytes.get(20..24)?.try_into().ok()?);
    bytes.starts_with(b"\x89PNG").then_some((w, h))
}

/// O tamanho que a página da demonstração mandou (`{"aylis_demo_size":[640,240]}`).
fn tamanho_demo(mensagem: &str) -> Option<(f32, f32)> {
    let valor: Value = serde_json::from_str(mensagem).ok()?;
    let par = valor["aylis_demo_size"].as_array()?;
    Some((par.first()?.as_f64()? as f32, par.get(1)?.as_f64()? as f32))
}

/// O endereço da demonstração de uma página de componente (`componentes/Button.md`), se o
/// componente é visual: `demos/<família>/index.html?class=Button&theme=dark`. A família é a
/// categoria da paleta, que a página diz na linha `**Paleta:** Standard · **controle visual**`,
/// com `-` no lugar dos espaços (a mesma pasta que o `aylis-tool demos` grava).
fn demo_de(arquivo: &str, texto: &str) -> Option<String> {
    let classe = classe_visual(arquivo, texto)?;
    let linha = texto.lines().find(|l| l.starts_with("**Paleta:**"))?;
    let categoria = linha.trim_start_matches("**Paleta:**").split('·').next()?.trim();
    Some(format!("demos/{}/index.html?class={classe}&theme=dark", categoria.to_lowercase().replace(' ', "-")))
}

/// A classe de uma página de componente (`componentes/Button.md` → `Button`), se ele é visual
/// (a linha `**Paleta:** Standard · **controle visual**`).
fn classe_visual<'a>(arquivo: &'a str, texto: &str) -> Option<&'a str> {
    let classe = arquivo.strip_prefix("componentes/")?.strip_suffix(".md")?;
    texto.lines().find(|l| l.starts_with("**Paleta:**"))?.contains("**controle visual**").then_some(classe)
}

/// Um título do índice sem as marcas do Markdown (`` `ClientDataSet` ``, `**negrito**`): a
/// árvore, a trilha, a busca e os botões "Anterior" / "Próxima" mostram texto puro.
fn sem_marcas(titulo: &str) -> String {
    titulo.replace('`', "").replace("**", "").trim().to_string()
}

/// O texto de um arquivo da documentação.
fn ler(ui: &Ui, arquivo: &str) -> Option<String> {
    let bytes = ui.services().resources.as_ref()?.read(&format!("{DOCS}{arquivo}"))?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// Os títulos de segundo e terceiro nível de um Markdown (fora dos blocos de código), sem as
/// marcas de código e de negrito: (nível, texto).
fn titulos_de(texto: &str) -> Vec<(u8, String)> {
    let mut dentro_de_codigo = false;
    let mut titulos = Vec::new();
    for linha in texto.lines() {
        let linha = linha.trim_end();
        if linha.trim_start().starts_with("```") {
            dentro_de_codigo = !dentro_de_codigo;
            continue;
        }
        if dentro_de_codigo {
            continue;
        }
        let (nivel, resto) = if let Some(r) = linha.strip_prefix("### ") {
            (3, r)
        } else if let Some(r) = linha.strip_prefix("## ") {
            (2, r)
        } else {
            continue;
        };
        titulos.push((nivel, resto.replace('`', "").replace("**", "").trim().to_string()));
    }
    titulos
}

#[cfg(test)]
mod tests {
    use super::{demo_de, sem_marcas};

    /// Os títulos do índice vêm do Markdown dos guias: a crase e o negrito saem.
    #[test]
    fn index_titles_lose_the_markdown_marks() {
        assert_eq!(sem_marcas("O `ClientDataSet` e o **delta**"), "O ClientDataSet e o delta");
        assert_eq!(sem_marcas("Temas"), "Temas");
    }

    /// A página de um componente visual leva ao wasm da família dele (a pasta que o
    /// `aylis-tool demos` grava: a categoria com `-`); um componente sem tela, a nenhum.
    #[test]
    fn a_visual_component_page_points_to_its_family_demo() {
        let botao = "# Button\n\n![Button](img/Button.png)\n\n**Paleta:** Standard · **controle visual** · também tem as propriedades";
        assert_eq!(demo_de("componentes/Button.md", botao).as_deref(), Some("demos/standard/index.html?class=Button&theme=dark"));
        let grade = "**Paleta:** Data Controls · **controle visual**";
        assert_eq!(demo_de("componentes/DataGrid.md", grade).as_deref(), Some("demos/data-controls/index.html?class=DataGrid&theme=dark"));
        assert_eq!(demo_de("componentes/Timer.md", "**Paleta:** System · componente não visual"), None);
        assert_eq!(demo_de("guias/README.md", botao), None);
    }
}
