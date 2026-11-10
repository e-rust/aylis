# aylis — guia para agentes

Framework de interface em Rust: componentes com ancestral comum,
`Align`/`Anchors`/`Margins`/`Padding`, eventos por nome, forms em JSON, camada de dados
(`ClientDataSet`/`Field`/`DataSource`) e uma IDE feita com o próprio Aylis.

Onde está cada coisa:

| Arquivo | O que tem |
|---|---|
| [PLANO.md](PLANO.md) | O plano inicial |
| [AJUSTES-PENDENTES.md](AJUSTES-PENDENTES.md) | O que falta e o que espera o autor confirmar. Pedido novo entra aqui |
| [AJUSTES-CONCLUIDOS.md](AJUSTES-CONCLUIDOS.md) | O que o autor confirmou, com o commit |
| [docs/historico-decisoes.md](docs/historico-decisoes.md) | Os pedidos e as decisões, com o porquê |
| [docs/roteiro-de-conferencia.md](docs/roteiro-de-conferencia.md) | O que o autor confere, item por item |
| [docs/testes-em-dispositivos.md](docs/testes-em-dispositivos.md) | O que só dá para conferir num Mac, num celular ou noutro hardware |
| [medicoes/](medicoes/) | As medições de desempenho (as bases dos testes e os números de antes e depois) |

## Idioma e estilo

- Texto em **português do Brasil**: comentários, docs, mensagens, textos de tela e commits.
- **Nomes em inglês**, sempre: funções, tipos, variáveis, campos, propriedades, eventos,
  testes (funções e arquivos), exemplos, módulos, chaves de formatos gravados (`.aylisrec`,
  roteiros, config), variáveis de ambiente e subcomandos/opções de linha de comando.
  Nomes de componente em forms de exemplo e de teste (`edtNome`, `btnSalvar`) são dados de
  app e podem ficar em português.
- Em texto para pessoas (telas, docs, site, mensagens), **"Aylis"** com maiúscula; nomes de
  código, crate, comando e pasta ficam como são (`aylis-core`, `aylis-tool`).
- Siga o código ao redor: comentários de doc curtos (`///`, `//!`) explicando o porquê.
- Arquivos com LF (`core.autocrlf=false`); alguns antigos têm CRLF — preserve o que
  estiver no arquivo.

## Organização

| Crate | O que é |
|---|---|
| `aylis-core` | Núcleo sem sistema operacional: `Ui`, `Component`, `ControlBase`, propriedades, layout, eventos, tema, `Canvas`, serviços (traits) |
| `aylis-controls` | Os controles: junta os crates por família e registra as classes (`aylis_controls::register`) |
| `aylis-controls-base`, `-charts`, `-industrial`, `-device`, `-editors` | As famílias de controles (compilam em paralelo; só a base tem controles que usam outros) |
| `aylis-data`, `aylis-dbcontrols` | ClientDataSet/Field/DataSource e os controles DB* |
| `aylis-providers` | Os componentes ligados a bibliotecas de terceiros (código de barras, CSV/XLSX, PDF, mapas) |
| `aylis-json` | Layout e tema em JSON (carregar, salvar no formato canônico, hot reload) |
| `aylis-skia`, `aylis-d2d`, `aylis-web` | Renderizadores (Skia CPU/GPU, Direct2D, Canvas 2D do navegador) |
| `aylis-platform-winit` | Janelas e serviços de cada sistema (Windows, Linux, macOS, Android, iOS, web) |
| `aylis-media`, `aylis-3d`, `aylis-docview`, `aylis-vector`, `aylis-markdown`, `aylis-webview` | Vídeo/áudio/câmera/microfone, 3D (wgpu), PDF/Office, Lottie/SVG, Markdown, WebView |
| `aylis-remote`, `aylis-host`, `aylis-capi` | Protocolo JSON do backend em outro processo; o host (stdio, WebSocket, `--bridge`); a API C |
| `aylis-project`, `aylis-build`, `aylis-tools` | Projeto `.aylisproj` e gerador de código; empacotar no build; compilar, executar, publicar, verificar ambiente (`aylis-tool`) |
| `aylis-ide`, `aylis-demos` | A IDE; os componentes ao vivo da documentação (um wasm por categoria) |
| `aylis-migrate` | Converte projetos de uma versão anterior do formato |
| `aylis` | A fachada que os apps usam (`aylis::Application`, `aylis::classes`, `aylis::samples`) |
| `clients/` | Clientes do protocolo em TypeScript, C#, Java, Go, C++, Free Pascal e Python |

Onde mexer em cada caso:

- **Texto novo na IDE:** `aylis-tool translations` lista os que faltam; grave uma tabela (o
  português e as 11 traduções por linha, separados por tabulação, na ordem en es fr de ru zh
  ja hi ar bn id) e rode `aylis-tool translations --apply tabela.tsv`. A fonte são os
  `crates/aylis-ide/resources/locales/*.json`; o teste
  `the_new_texts_and_the_messages_with_values_are_translated` confere. Rótulos de
  propriedade: `crates/aylis-ide/resources/locales/properties.tsv`.
- **Mensagem do próprio Aylis para o usuário final:** `aylis_core::messages::t`, mostrada pelo
  `MessageDialog`/`message_dlg` (nunca a caixa do sistema; o `rfd` fica só para escolher
  arquivos).
- **Projeto:** o visual (forms, temas, assets) é um só e o código fica numa pasta por
  linguagem (`<projeto>/rust`, `<projeto>/python`...; `Project::code_dir`). A geração mantém
  todas com os handlers do form, e `aylis_project::sync` diz o que falta portar
  (`code-sync.json`).
- **Acesso tipado** (`frmX.comp.Prop`, `FrmX::create`): `aylis_project::bindings`, a partir de
  `resources/components.json`. Classe ou propriedade nova: `aylis-tool update-catalog` regrava
  o catálogo (`cargo test -p aylis-remote --test catalog` confere). Mudou o que o gerador
  escreve: suba o `GENERATOR_VERSION`, senão os projetos não regeneram. Os exemplos usam só o
  acesso tipado, sem procurar componente pelo nome (`examples_typed_access` confere).

## Diretrizes do projeto

Decididas com o autor (o porquê de cada uma está em
[docs/historico-decisoes.md](docs/historico-decisoes.md)). Algo que precise sair delas não se
faz por conta própria: vira pergunta ao autor e, aprovado, entra no histórico.

### Arquitetura

- **Núcleo sem sistema:** o `aylis-core` não importa `winit`, `skia`, `web-sys`, `windows` nem
  nada de um sistema só. O que é do sistema vira trait de serviço em `platform.rs`, e cada
  plataforma implementa.
- **A tela é o frontend:** o backend pode estar em qualquer linguagem. Toda operação tem de se
  expressar como classe + propriedade + valor (`PropValue`). Recurso só com API Rust tipada
  não basta: precisa de nome no registro (propriedade, evento ou método do `invoke`), para
  valer no JSON, no protocolo, na API C e nos 7 clientes.
- **Nada de banco no frontend:** sem conexão, SQL ou driver. O `ClientDataSet` guarda os dados
  e o delta, e o backend recebe o delta e aplica.
- **Biblioteca de terceiros por provedor:** o componente que o app usa liga-se a um componente
  provedor (como o dataset ao `DataSource`), para trocar de biblioteca sem mexer em quem usa
  (`aylis_core::providers`).
- **Sem linguagem própria:** nada de script, comandos, laços ou funções do usuário. Expressões
  só de valor, sem efeito colateral (o `filter`, o campo calculado, o texto `[...]` do
  relatório); regra de negócio é código do app, e o declarativo simples (`display_format`,
  `edit_mask`, `required`) pode.
- **Formatos em JSON** (`{"aylis": 1, "root": {...}}`), sem nomes antigos nem "obsoletos". Os
  forms são gravados no formato canônico (`aylis_json::to_form_text`), bom para merge.
  Mudança incompatível sobe a versão, e a conversão vai para o `aylis-migrate`.
- **Eventos:** reentrantes, um handler por vez, nunca em paralelo. Thread é
  escolha do programador e volta à tela por `post`/`post_to`. Na web, `aylis::background`
  (`std::thread` compila para wasm, mas falha ao rodar).
- **Erros:**
  - handler e método devolvem `Result`, e a recusa é por erro;
  - mensagem ao usuário pelo `MessageDialog` e pelo `aylis_core::messages::t`;
  - pânico não derruba a tela.
- **Gerado × do usuário:** o que a IDE gera fica em `generated/` (somente leitura). O código do
  usuário só ganha handlers novos no fim do `impl`.

### Componentização

- **Ancestral comum:**
  - todo componente é `ControlBase` + `impl Component`, com `properties!` (categoria de cada
    propriedade) e eventos por nome;
  - criar, liberar e trocar de pai em execução, e mexer em propriedade por nome, valem para
    todos.
- **Estender por composição e delegação** (o `inherited`, como o `GlowButton`), não copiando
  um controle para mudar a pintura. Comportamento fica separado da aparência.
- **Aparência pelo tema:**
  - tokens, estados e variantes;
  - cores de `ctx.colors` (o `ColorOverride` do controle) e fontes de `ctx.font`;
  - nada de cor ou fonte fixa no código (o teste de cores de todos os controles confere).
- **Repintura é do núcleo:** o componente só diz o que mudou (`ctx.repaint()`). Área, imagem
  guardada e rolagem por cópia são do núcleo. Declare `paint_is_pure` quando a pintura não
  tem efeito colateral.
- **Composto:** o pai lê os filhos; o filho avisa o pai por evento.
- **Família:** o controle vai para a família certa (`aylis-controls-base`, `-charts`,
  `-industrial`, `-device`, `-editors`; o que depende de biblioteca de terceiros, para o
  `aylis-providers`). Só a base tem controles que usam outros controles.
- **Classe nova completa:**
  - registro em `aylis_controls::register`, em `aylis_build::CLASSES` e em `aylis::classes`
    (o teste `crates/aylis/tests/classes.rs` confere as três);
  - `aylis_build::CLASS_PLATFORMS` se depender de plataforma;
  - `aylis-tool update-catalog`;
  - rótulos traduzidos;
  - conteúdo de exemplo em `aylis::samples` (galeria e documentação ao vivo);
  - a documentação refeita (`aylis-tool docs` regrava as páginas da Landing);
  - teste.

  Guia: [docs/criar-componente.md](docs/criar-componente.md).

### Plataformas

- **Alvos:** Windows, Linux, macOS, Android, iOS e navegador (wasm). Tudo compila em todos.
  Recurso que não existe numa plataforma degrada: o componente existe e não liga, e vai para
  `CLASS_PLATFORMS`.
- **Renderizadores:** Direct2D no Windows (o padrão), Skia (CPU/GPU) nas outras e Canvas 2D
  no navegador. O desenho tem de sair igual nos três: nada que só um saiba fazer.
- **Na web e na IDE lite:** sem disco, processo, `std::thread` nem `Instant` direto. Recurso
  de desktop novo na IDE passa por `lite::can` ou pelos módulos `files`/`host`, e o trabalho
  em segundo plano, por `aylis::background`.
- **O que precisa de Mac ou de celular real** vai para
  [docs/testes-em-dispositivos.md](docs/testes-em-dispositivos.md) e não é dado como feito.

### Desempenho

- **Sem regressão:**
  - `giant_screen`: a área e os controles pintados sempre; os tempos em release, antes do
    commit;
  - pintura por parte igual à inteira (`partial_paint`);
  - a base só é regravada de propósito, com o motivo nos pendentes.
- **Custo por quadro proporcional ao que aparece, não ao total:**
  - listas e grades virtualizadas (o DBGrid custa o mesmo com 2 mil ou 1 milhão de linhas);
  - achar componente por índice, não percorrendo a árvore;
  - nada que percorra o projeto inteiro (os forms, o código, o disco) a cada clique ou a
    cada refresh da IDE: guarde pela marca do arquivo (`FileStamp`) e leia só o que mudou.
- **Parado não gasta:** timer e animação só enquanto algo muda, sem repintura contínua (a
  bateria do celular).
- **Memória por instância:** o que é igual em todas (temas, catálogos) é compartilhado
  (`Arc`), não copiado: a IDE abre um `Ui` por aba de form.
- **Remoto:** nunca uma ida por célula ou por tecla; use lote, cache por valor e chamada
  assíncrona.
- **Medir antes e depois** (`AYLIS_BENCH`, `aylis-tool bench`, os testes de desempenho abaixo,
  `medicoes/`), e os números entram nos pendentes. Otimização sem medição não entra.
- **Tamanho:** o app liga só as features e as classes que os forms usam. Dependência nova e
  pesada fica atrás de feature e precisa de motivo (veja [docs/medidas.md](docs/medidas.md)).

### Segurança

- **Nenhuma porta para fora:**
  - servidores (MCP da IDE, `McpServer` do app, canal de desenvolvimento) só em `127.0.0.1`,
    com token ou segredo, e recusando `Origin` de outro site;
  - segredo em `%LOCALAPPDATA%\aylis`, nunca no repositório nem no projeto;
  - a tela conecta no backend, nunca o contrário (sem `--listen`).
- **Texto não vira código nem comando:**
  - nome de handler e de form passa por `is_identifier`;
  - nada de `cmd /C` ou shell: `aylis_tools::shell::open` e processos com argumentos
    separados; a exceção são os comandos antes e depois da compilação do projeto (cmd e
    PowerShell), que só rodam em projeto confiado e aparecem no "Confiar neste projeto?";
  - caminhos de forms sem absoluto nem `..`.
- **Dependências e ferramentas:** as dependências passam pelo `cargo-deny` (`deny.toml`,
  `aylis-tool audit`). Ferramenta baixada tem versão fixa e SHA-256
  (`resources/tools.lock.json`).
- **Projeto de fora é não confiável até o usuário dizer:** no modo restrito, sem compilar,
  executar, servidor de linguagem nem publicar. O MCP não confirma confiança.
- **Dados sensíveis:**
  - campo de senha não entra em gravação, rastro nem log (vira `*`);
  - gravação e métricas não vão para a rede;
  - testes não salvam imagens da câmera.
- **Protocolo:** limites de conexões, tamanho de mensagem e prazos. Evento que chega do
  cliente pode ser forjado, então o backend valida.
- **`unsafe`:** só em FFI de plataforma e renderizador, com `// SAFETY:` dizendo por que vale.
  O `aylis-core` não tem `unsafe`.

## Antes do commit

Commit só quando o autor pedir; push também. Quem faz o commit roda, nesta ordem:

1. `cargo test --workspace --exclude aylis-platform-winit` (alguns testes pulam sem a
   ferramenta da linguagem).
2. `cargo test --release -p aylis-controls --test giant_screen` (os tempos contra a base).
3. `cargo check -p <crate> --target wasm32-unknown-unknown` dos crates mexidos (a IDE lite:
   `-p aylis-ide --no-default-features`).
4. `cargo ndk -t x86_64 -P 26 check -p <crate>` dos crates mexidos, com `ANDROID_NDK_HOME`
   (`%LOCALAPPDATA%\Android\Sdk\ndk\<versão>`).
5. Mudou tela da IDE de propósito: `AYLIS_UPDATE_VISUAL=1 cargo test -p aylis-ide --test
   visual`, e as imagens entram no commit.

Uma falha no que não é seu não se corrige ali: relate ao autor, com a saída, para ele
encaminhar à sessão responsável. O `bridge` do `aylis-host` às vezes falha com a máquina
carregada; rodado sozinho, passa.

## Comandos

```bash
cargo run -p aylis-ide                                    # a IDE
cargo run -p aylis-platform-winit --example shapes --features cpu   # exemplos (veja examples/ do crate)
cargo run -p aylis-tools --bin aylis-tool -- <subcomando>  # as ferramentas
```

**Ver sem janela**
- Captura de um exemplo: `AYLIS_SNAPSHOT=pasta AYLIS_BENCH=5 cargo run ... --example x` grava
  `pasta/form.png`.
- A IDE em PNG: `cargo run -p aylis-ide -- --png saida.png` (`--gallery`: a galeria de
  componentes; `--recording arquivo`: o estado de uma gravação); idioma: `AYLIS_IDE_LANG=en`.
- Gravações de uso: a IDE grava o próprio uso em `%LOCALAPPDATA%\aylis\recordings\*.aylisrec`
  (JSON Lines; `aylis_controls::Recording` lê). Para investigar um problema relatado, leia a
  gravação mais recente.

**Testes de imagem e de desempenho**
- Telas da IDE comparadas com referências: `cargo test -p aylis-ide --test visual` (as imagens
  em `crates/aylis-ide/tests/visual/`).
- Pintura por parte × inteira: `cargo test -p aylis-controls --test partial_paint`; o caso
  grande: `cargo run --release -p aylis-controls --example large_ui`.
- `giant_screen`: uma tela de 19.200 × 6.480 com ~18 mil componentes, sem janela, comparada
  com `medicoes/giant_screen.json`. Melhorou de propósito: `AYLIS_UPDATE_BENCH=1` em release
  regrava a base, que entra no commit.
- A IDE com o ERP de exemplo (1009 forms), em release e **um teste por vez** (a memória
  medida é a do processo): `cargo test -p aylis-ide --release --test perf -- --ignored
  --nocapture --exact <teste>`, com `erp_ide_cost` (abrir o projeto, abrir forms, a busca, a
  memória), `erp_all_forms_in_tabs` (os 1009 abertos: trocar de aba, Design ↔ Código) e
  `memory_per_document`. Os limites estão nos próprios testes; os números, em
  `medicoes/erp_1000_forms.json`.
- O editor com um fonte enorme: `cargo run --release -p aylis-controls --example large_text
  [linhas]`.

**Outras linguagens**
- `cargo test -p aylis-host --test languages --test language_projects` e `cargo test -p
  aylis-tools --test apps` (Calculadora, Paint, Player e IBGE no mesmo roteiro nas 8
  linguagens).
- Ferramentas fora do PATH: a aba da linguagem nas Opções da IDE, que grava
  `%LOCALAPPDATA%\aylis\tools.json` (`aylis_tools::settings`).

**Plataformas**
- Android: `aylis-tool run app.aylisproj --target android` (emulador `aylis_pixel`; `adb` em
  `%LOCALAPPDATA%\Android\Sdk\platform-tools`).
- Linux: Docker `rust:1-bookworm` com Xvfb.
- Web: a IDE lite é a mesma IDE sem a feature `desktop` (`crates/aylis-ide/src/lite.rs`).
  O site, nesta ordem (o `publish` limpa a pasta): `aylis-tool publish
  examples/Landing/Landing.aylisproj --target web --out <página>`, depois `ide-web --out
  <página>/ide` e `demos --out <página>/demos` (crate `aylis-demos`: um wasm por categoria da
  paleta, carregado num `<iframe>` só na página do componente); `aylis-tool serve <página>`
  serve.

**Ferramentas do `aylis-tool`** (nada de scripts Python: ferramenta nova vira subcomando)
- `docs` regrava as ~250 páginas da documentação da Landing (componentes e guias).
- `resources` baixa ícones, fontes e animações; `emoji` refaz o seletor de emoji; `bench`
  mede os exemplos; `test-docs` refaz os documentos de teste do `aylis-docview`.
- `installer-ide` gera `dist/aylis-ide-<versão>-setup.exe` (NSIS). A pasta do Aylis é achada em
  tempo de execução (`aylis_project::codegen::aylis_root`: `aylis/` ao lado do executável,
  depois o repositório onde foi compilado); instalada, as compilações vão para
  `%LOCALAPPDATA%\aylis\target` (`aylis_tools::shared_target_dir`).
- `clean [--project app.aylisproj]`: o `target` é limpo sozinho, sem mexer durante uma
  compilação. As compilações pelo Aylis anotam em `target/.aylis-index.json` que projeto usou
  que unidade (`aylis_tools::build_index`), e sai o que nenhum projeto vivo usa mais; fora do
  índice, o substituído ou sem uso há 7 dias (`aylis_tools::cleanup`).

## Várias sessões na mesma árvore

Várias sessões trabalham ao mesmo tempo neste repositório, então a árvore costuma ter
alterações de outras sessões (e não há commit por assunto, por enquanto).

- Não reverta nem "arrume" alterações que não são suas, nem derrube processos de outra
  sessão (um `aylis-tool serve`, por exemplo): se algo estiver preso (um executável em uso),
  avise o autor.
- Antes de criar uma função, procure se ela já existe: outra sessão pode ter acabado de
  escrever a mesma.

## Armadilhas conhecidas (Windows)

- No Git Bash desta máquina, um heredoc com aspas (`<<'EOF'`) ainda come barras
  invertidas: para editar código com `\n`, `\\` ou `\"`, use a ferramenta de edição.
- Não use o `perl -pi` com `|` como delimitador (`s|...|...|`) num padrão que tenha `|`: ele
  já inseriu texto no topo de arquivos.
- O `cmd /C` não entende argumentos com aspas escapadas: grave um `.bat`.
- Ferramentas .NET (WiX) precisam de `DOTNET_ROOT` quando o .NET não está instalado no
  sistema.
- Ao mexer na `AylisActivity.java`, recompile o `aylis-tool` (o Java vai embutido nele).
- O repositório está no OneDrive: cada consulta ao disco (`metadata`, `canonicalize`) é
  lenta. Em laço sobre muitos arquivos, guarde o resultado.

## Testes com hardware do autor

Câmera e microfone são reais nesta máquina. Os testes **não salvam nem abrem imagens da
câmera**: só conferem que o arquivo foi escrito e apagam em seguida.
