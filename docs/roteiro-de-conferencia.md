# Roteiro de conferência

Os itens dos [ajustes pendentes](ajustes-pendentes.md) que estão feitos e só esperam a
confirmação do autor (os que precisam de hardware ou de outra máquina ficaram para depois,
decidido em 05/11). Cada item diz onde ir, o que fazer e o que deve acontecer. Responda pelo
número (ok, ou o que deu diferente); o que for confirmado sai dos pendentes e deste roteiro.

Antes de começar, a IDE do repositório recompilada: `cargo run --release -p aylis-ide`.

As Sessões 1 (a IDE com o NoOBS) e 2 (a IDE com o ERP) foram conferidas e confirmadas pelo autor
em 07/11, e as Sessões 3 a 7 em 08/11 (com o Claude pelo MCP da IDE): o que passou saiu dos pendentes
(está no commit). Na noite de 08/11, depois dos ajustes, voltaram e
foram aprovados os itens 38, 41, 48, 59 e 67. Os números dos itens continuam os mesmos.

## Para conferir de novo

95. **Duas sessões no mesmo projeto** (item 71). Numa conversa do Claude, abra um projeto pelo
    Aylis; noutra conversa (o Claude Desktop, ou outro Claude Code), peça o mesmo projeto.
    *Esperado:* as duas ficam na mesma IDE, com os dois títulos na barra da janela, e a mudança de
    uma aparece para a outra; fechando uma conversa, o título dela sai e a outra continua. Peça
    outro projeto na segunda: ele abre noutra IDE, e o primeiro continua aberto na primeira.
    Em 08/11 à noite: a mesma IDE, certo, mas o registro da IDE só tinha a sessão do Claude Code.
    **Próximo passo:** na conversa do Claude Desktop, chamar o `session_title` com "Teste Desktop" e
    dizer o que a barra da IDE mostra (os dois títulos, só o novo, ou só o do Claude Code).
92. **A sessão do Claude abre a IDE** (item 68). Com a IDE fechada, numa conversa no Claude
    Desktop (ou numa sessão do Claude Code), peça para usar o Aylis. *Esperado:* a IDE abre
    sozinha, a resposta diz "Abri a IDE do Aylis para esta conversa", com o título da conversa na
    barra da janela; as chamadas vão para ela, com todas as ferramentas da IDE desde a primeira;
    outra conversa abre outra IDE. (No Claude Code, a abertura e o aviso passaram em 08/11 à noite.)

97. **O compilador separado da IDE** (pedido de 09/11). Antes, recompile também o `aylis-tools`
    (`cargo build -p aylis-tools`, com as conversas do Claude fechadas: a ponte do MCP prende o
    `aylis-tool.exe`). Abra o Paint (Rust) e dê Ctrl+F9; depois F9; depois Parar (Ctrl+F2)
    no meio de um "Compilar (release)". *Esperado:* nas MENSAGENS, "Gerando o código", "Verificando
    o projeto" e a compilação, com os erros clicáveis como antes; o Gerenciador de Tarefas mostra um
    `aylis-tool.exe` enquanto compila; o Parar encerra o `aylis-tool` e o `cargo` juntos. Num
    terminal, `aylis-tool build examples/Paint/Paint.aylisproj` faz o mesmo, e a Calculadora (Free Pascal)
    só gera e verifica.
98. **O Design no código de um form sem aba** (relatado em 10/11). Abra um projeto com dois
    forms, só com o principal aberto; abra o fonte do outro (pelo Pesquisar no projeto ou pela
    lista de arquivos) e clique em Design (ou F12). *Esperado:* abre o form desse fonte numa aba
    nova, e não o último design mostrado. (O mínimo do Rust no "Verificar ambiente" também passou
    a 1.88, o `rust-version` dos crates: um Rust 1.86 ou 1.87 agora aparece como desatualizado.)
99. **O aviso de versão nova** (pedido de 10/11, como no NoOBS). Na IDE, Ajuda → Verificar
    atualizações. *Esperado:* "O Aylis está atualizado (versão 0.2.0-beta.1)." Ajuda → "Avisar de novas
    versões" vem marcado e desmarca/marca. Com uma release mais nova no GitHub, a IDE instalada
    (não a do `target/`, que só consulta pelo menu) mostra, uma vez por dia depois das 6 h, a faixa
    "Nova versão do Aylis disponível" abaixo da barra, com "Abrir página" e o ✕.
100. **Duas conversas, dois projetos** (relatado em 10/11). Antes, feche o Claude Desktop e
    recompile o `aylis-tools` (as pontes antigas prendem o `aylis-tool.exe`). Numa conversa do
    Claude Desktop, peça para trabalhar no NoOBS; noutra, no RTSPlayer, com as duas IDEs abertas.
    *Esperado:* cada conversa passa o `project` nas chamadas e mexe só na IDE do seu projeto; o
    título de cada uma aparece só na janela da IDE dela; uma chamada sem `project` volta recusada,
    com a lista das IDEs abertas; pedir para abrir outro projeto abre noutra IDE, sem trocar o
    projeto da IDE da outra conversa.
101. **O relatório do MCP do RTSPlayer** (10/11, os 16 itens; detalhes nos pendentes). Com a IDE
    e o `aylis-tools` recompilados e o RTSPlayer aberto:
    - os bindings e os cliques nas linhas: `bindings` no `DataRepeater` (o texto aparece nas linhas);
      um botão de linha chama o handler do form;
    - pelo Claude: `class_info` de um Label (os valores de `trimming`); um `set_layout` com dois
      erros devolve os dois; `patch_layout` de um painel; `app_command` `snapshot` mostra a tela;
    - `run_without_debug` com um erro no código responde com o erro; a primeira compilação
      responde "ainda compilando" e o `build_wait` traz o fim;
    - Projeto → Novo fonte… cria e abre o arquivo;
    - o RTSPlayer sem o `[workspace]` à mão no `Cargo.toml` compila depois do "Gerar código", e com
      `"features": ["media"]` no `.aylisproj` mantém a feature sem componente de mídia.

## Os ajustes de 08/11 que faltam conferir

Cada um tem, nos pendentes ("Conferência de 08/11/2026" e "Achados e feitos na conferência de 08/11
à noite"), a nota "Feito em 08/11" com o que mudou. Conferir na IDE recompilada, por grupo:

- **IDE:** F12 volta onde estava; Ctrl+Z depois do `edit_code`; marcadores numerados (um por linha);
  a dica do código como no VS Code e a dica do rodapé do servidor de linguagem; as dicas nos botões da
  barra; as abas dos forms com muitas abertas (o × só onde aparece, o nome inteiro, a roda rola);
  ESTRUTURA recolhendo pela seta e por ←/→; a busca do Object Inspector com os grupos abertos; o
  designer pelo teclado (Shift+seta, Ctrl+seta, Alt+↑/↓); o clique nos resultados do "Pesquisar no
  projeto"; Traduções (a lista dos idiomas) e Fontes (só a falta); a IDE em English (o roteiro 84 e o
  idioma marcado no menu); o Profiler (a roda e o "quadro · N% repintado"); MENSAGENS com vermelho só
  para erro; "Aylis" com maiúscula; a categoria "Dados" da paleta; os diálogos com a IDE minimizada; a
  IDE no Direct2D; o aviso quando a IDE nasce presa ao cliente do MCP.
- **Execução e projeto:** encerrar o app aberto antes de executar e o aviso antes de compilar; os
  comandos antes e depois da compilação (e o "Confiar" mostrando cada um); o `.aylisproj` sem mudar a
  cada salvar de form; a cópia da Landing compilando fora do repositório; o `allow(non_snake_case)`;
  o `on_change` disparado pelo código, como na VCL; o modelo Web e o Desktop (o Smartphone passou).
- **Componentes:** o Enter que repete o clique e o Espaço que clica ao soltar; o anel de foco com folga;
  o Memo no fim do texto; o QR com o nível de correção; o MapView só pelo provedor; os métodos tipados
  com nome e `Result`; o `write_pdf` com a memória plana; a GradeGigante (a memória, o "Montando…" e o
  Parar).
- **MCP:** `dialog_state` com as listas e os botões, `dialog_select`, `ide_menus` sem repetidos; o `set`
  do `app_command` nas duas formas; o Esc fechando a Galeria; o guarda recusando o que não é JSON; o
  caminho do `claude` nas Opções da IDE.
- **Web e documentação:** a Landing (o título "Aylis", o favicon, o "Aylis" e o "Baixar a IDE" maiores); o
  cache sem servidor (F5 várias vezes); a prévia da IDE lite; a busca com destaque e os títulos sem
  crases; as páginas novas (a coluna "Quando", o `on_server_query`, o `Barcode` no relatório, Sankey e
  Waffle na galeria, o `on_close_query` no ProcessamentoAssincrono).

O que precisa de Mac, celular ou outro hardware está em
[testes-em-dispositivos.md](testes-em-dispositivos.md).
