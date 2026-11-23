# Proposta: CI com LLM, automação da IDE, rastro com métricas e cobertura

Pedido do autor em 07/11: "verificar integração contínua do futuro com LLM, pensar em algo que
automatize a IDE e os projetos usando ela, code tracing com analytics, code coverage". Isto é
o estudo: o que já existe, o que falta e as decisões do autor (no fim). O estudo e as decisões
do autor (08/11) foram feitos em 14/11 (item 38 dos pendentes; falta o autor conferir no
GitHub, roteiro 146). Falta: as bases de cobertura e de tempos (a primeira CI), o tempo do
backend nas outras 7 linguagens no 3b, o 4c, o visualizador de gravações levando ao código e a
marca de tempo no editor (3d).

## O que já existe

- **CI** (`.github/workflows/ci.yml`):
  - auditoria das dependências (`cargo-deny`);
  - testes no Windows e no Linux, com as telas da IDE comparadas com as referências;
  - compilação para wasm e Android;
  - a cobertura por crate no Windows e no Linux, com a base `medicoes/cobertura.json` (cair mais
    de 1 ponto reprova; a base ainda vazia) e o lcov como artefato.
- **MCP da IDE:** 84 ferramentas (81 da IDE, em `aylis_tools::mcp::ide_tools`, e
  3 da ponte: `session_title`, `ide_list` e `ide_select`).
  - projeto e forms: abrir, criar (na pasta escolhida), mover, componentes, propriedades,
    layout, gerar código;
  - frame (`new_frame`), tema (`get_theme`, `set_theme`), traduções (`translations`,
    `set_translations`) e ícone do app (`set_app_icon`);
  - editor: digitar, teclas, hover, estado;
  - diálogos e menus;
  - depuração: pontos de parada, variáveis, quadro;
  - compilar, verificar (`check`), testar (`test`), executar, publicar (com AAB e instalador);
  - imagens: `designer_snapshot`, `screenshot` (qualquer form do projeto, ou o app em
    execução) e `compare_image`;
  - `app_command`, que comanda o app em execução pelo canal de desenvolvimento.

  `aylis-tool mcp` faz a ponte stdio, e as Opções da IDE configuram o Claude Code e o Claude
  Desktop.
- **No app:**
  - `McpServer` (o app como servidor MCP, com `expose_ui` e `McpTool`);
  - `EventRecorder` (`.aylisrec`, a caixa-preta);
  - `Analytics` (métricas somadas entre sessões, só no disco);
  - os observadores de atividade do núcleo.
- **Profiler** (Executar → Profiler): pelo canal de desenvolvimento, uma linha do tempo por
  faixa (handlers, eventos, layout, pintura, timers, dados) e a tabela de métodos (chamadas,
  total, próprio, média, máximo), que abre o código do handler.
- **Testes de apps:** o app de desenvolvimento aceita roteiro e `snapshot` pelo canal, e a
  verificação antes de compilar (`aylis-tool check`, o `check` do MCP) confere handlers,
  referências e assets. O `test` do MCP roda os testes do projeto na ferramenta da linguagem
  (`aylis_tools::project_tests`: `cargo test`, `unittest`, `go test`, `bun`/`node --test`,
  `dotnet test`) e devolve o resultado por teste; no modo restrito, não roda.

## 1. CI com LLM

O que é regra mecânica vira verificação determinística; o LLM fica com o que pede julgamento.
Um agente que "confere as diretrizes" erra de um jeito diferente a cada execução, e uma regra
que pode ser checada por programa não deve depender dele.

- **1a. `aylis-tool lint` (sem LLM, roda sempre):** **Feito (14/11).** As diretrizes do `CLAUDE.md` que dá para
  checar por programa. Entra como passo da CI e da verificação antes do commit.
  - `aylis-core` sem dependência de sistema;
  - cor fixa em `paint`;
  - `Command::new("cmd")`;
  - classe nova fora de uma das três listas (já há teste) ou sem exemplo em `aylis::samples`;
  - texto de tela sem tradução;
  - `unsafe` sem `// SAFETY:`.
- **1b. Revisão por agente no pull request:** **Feito (14/11)**, no `agent-review.yml`.
  - o `anthropics/claude-code-action` com o `CLAUDE.md` e o histórico de decisões;
  - pede: decisão contrariada, componente fora da família, recurso só em Rust sem nome no
    registro, texto em inglês, teste faltando;
  - comenta, não bloqueia.
- **1c. Triagem de falha:** **Feito (14/11)**, no `failure-triage.yml`. Quando a CI falha, um agente lê o log e as imagens das telas que
  mudaram (o artefato `telas-que-mudaram`). Ele comenta a causa provável e a sessão que mexeu
  ali. Hoje esse trabalho é do autor, que encaminha à sessão responsável.
- **1d. IDE sem janela na CI:** **Feito (14/11).** `aylis-ide --headless --mcp-stdio` abre a IDE sem desenhar a
  janela, com o MCP no stdio. Um agente (ou um roteiro fixo) cria um projeto, põe componentes,
  gera, compila, executa e comanda o app pelo `app_command`. Isso testa a IDE de ponta a ponta
  como um usuário, no lugar de só as funções. A foto de um form sai sem a janela do app (o
  `screenshot` pinta o form do designer em PNG), e o app também roda sem plataforma
  (`aylis::headless`, o B04).
- **1e. Gravação vira teste:** **Feito (14/11).** `aylis-tool recording-to-test arquivo.aylisrec` transforma uma
  gravação de uso (a que reproduziu o problema) em roteiro de teste do app. O agente da
  triagem usa o mesmo caminho para provar a correção.

## 2. Automatizar a IDE e os projetos feitos nela

- **2a. Guia do agente no projeto novo:** **Feito (14/11).** O Novo projeto, com a opção "Guia
  para agentes de IA" (marcada), grava o `AGENTS.md` com as regras para quem escreve o app, um
  `CLAUDE.md` de uma linha apontando para ele e o `.mcp.json`.
  - Conteúdo:
    - o que é gerado e o que é do usuário;
    - o acesso tipado (`self.btnSalvar`, `FrmX::components`);
    - nada de SQL na tela;
    - `MessageDialog`;
    - thread com `post_to`/`aylis::background`;
    - como rodar e testar.
  - Grava também um `.mcp.json` apontando para `aylis-tool mcp`: o Claude Code aberto na pasta
    do projeto já enxerga a IDE.
  - O texto vem de um modelo no Aylis (atualizado com ele), uma versão por linguagem do
    projeto.
- **2b. Ferramentas que faltavam no MCP da IDE** (o `check`, o `test` e o `screenshot` já
  existiam): **Feito (14/11).**
  - `profile` (ligar, pegar, resumir);
  - `recording` (ler a gravação mais recente, resumida);
  - `coverage` (abaixo).

  Com elas, o ciclo "mudar, conferir, testar, medir" roda inteiro pelo agente.
- **2c. Roteiros da IDE:** **Feito (14/11).** A mesma lista de ferramentas do MCP como roteiro JSON executável
  pela linha de comando (`aylis-tool ide-script roteiro.json`). Serve para tarefas repetidas
  (atualizar todos os projetos de exemplo, gerar a documentação) sem um agente no meio.

## 3. Rastro do código com métricas

O Profiler já mede tudo, mas só ao vivo e só na IDE. Falta guardar, comparar e somar.

- **3a. Exportar:** **Feito (14/11).** "Salvar rastro" grava o formato de eventos do Chrome (abre no Perfetto e no
  `chrome://tracing`), além do formato próprio que a IDE reabre.
- **3b. Rastro em teste:** **Feito (14/11)** (a base em `medicoes/handler_times.json`, que só
  avisa). Um teste de app pode pedir o rastro (`profile` pelo canal) e
  reprovar um handler acima de um tempo. Na CI, os tempos dos handlers dos exemplos entram numa
  base em `medicoes/` (como o `giant_screen`), e regressão reprova.
- **3c. Métricas de desempenho no `Analytics`:** **Feito (14/11)** (a propriedade `performance`,
  desligada por padrão). Além de quantas vezes, quanto tempo cada
  handler levou (média, p95, máximo) e os quadros lentos, somados entre sessões. Continua só no
  disco do usuário (diretriz de segurança). O app decide se e como manda; o Aylis não manda nada
  para a rede.
- **3d. Ligar ao código:** na tabela do Profiler e no visualizador de gravações, o handler leva
  ao arquivo e à linha (a geração já sabe onde cada handler está). No editor, uma marca discreta
  ao lado do handler mostra quantas vezes e quanto tempo ele levou na última execução.
  Feito em parte (14/11): o clique no handler do Profiler leva ao arquivo e à linha; falta o
  visualizador de gravações e a marca no editor.

## 4. Cobertura

- **4a. No Aylis:** **Feito (14/11)** (o `aylis-tool coverage-check`).
  - a cobertura da CI ganha uma base por crate em `medicoes/cobertura.json`; cair mais que um
    ponto reprova, como o `giant_screen`;
  - entra também o job do Windows (hoje é só o Linux);
  - o `lcov` vira artefato.
- **4b. Cobertura de handlers, em qualquer linguagem:** **Feito (14/11)** (Projeto → Cobertura
  de handlers…). Os observadores de atividade já sabem
  que handler rodou. Junto com a lista de handlers dos forms, isso dá "handlers que nenhum teste
  ou uso tocou", sem ferramenta de cobertura da linguagem. Funciona igual nas 8 linguagens e
  aparece na IDE (Projeto → Cobertura) e no MCP.
- **4c. Cobertura por linha nos projetos:**
  - "Executar com cobertura" usa a ferramenta de cada linguagem: `cargo llvm-cov` em Rust,
    `coverage.py` em Python, `c8` em TypeScript e o coletor do .NET em C#;
  - as linhas aparecem na margem do editor;
  - funciona com testes e também com uma sessão de uso manual: executar, clicar no app,
    fechar e ver o que rodou.

## Decisões do autor (08/11)

1. **Ordem:** 2b primeiro, depois 1a → 2a → 4b → 3a/3b → 4a → 1d → 1b/1c → o resto.
2. **Agente na CI (1b, 1c):** só comenta.
3. **Guia do projeto (2a):** opção do Novo projeto marcada por padrão, com `AGENTS.md`, um
   `CLAUDE.md` de uma linha e o `.mcp.json`.
4. **Tempos na CI (3b, 4a):** a cobertura reprova (cair mais de 1 ponto) e o tempo só avisa.
5. **Métricas de desempenho no app publicado (3c):** no `Analytics`, só quando o app pede.
