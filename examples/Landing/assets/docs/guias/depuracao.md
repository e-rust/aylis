# Depurar na IDE

A IDE depura o projeto: pontos de parada no código, o programa para, a linha
fica em amarelo, e dá para ver a pilha e as variáveis e andar passo a passo.

## Teclas

| Tecla | O quê |
|---|---|
| Ctrl+F5 | Depurar na IDE (compila e roda no depurador) |
| F5 (na janela de código) ou clique nos números | Liga/desliga o ponto de parada da linha |
| F9 | Executar: com ponto de parada, no depurador; depurando, continua |
| F8 | Próxima linha (passa por cima das funções) |
| F7 | Entrar na função |
| Shift+F8 | Sair da função |
| F4 | Executar até o cursor |
| Ctrl+F2 | Parar |
| Ctrl+F4 | Fechar o arquivo (a aba) na janela de código |

O código fica numa janela só, com uma aba por arquivo. À
esquerda, o painel **Fontes** lista os fontes do projeto (na linguagem ativa, por pasta);
clique abre na aba. **Navegar → Abrir fonte…** (Ctrl+F12, o "View Unit") acha um fonte
pelo nome, e **Ctrl+clique** numa palavra (ou Alt+Seta acima) vai à declaração dela —
procedure, função, classe, tipo —, mesmo em outro fonte; de novo, alterna entre a
interface e a implementação. A declaração vem de um índice dos fontes (as palavras de
declaração de cada linguagem), sem precisar do compilador. Quando o programa para:
- a linha fica em amarelo, no meio da tela, na aba do arquivo dela (entrar ou sair de uma
  função noutro fonte troca de aba, na mesma janela);
- o painel da direita mostra os botões (Continuar, Próxima linha...), a pilha (clique num
  quadro para ver a linha e as variáveis dele), as variáveis com um nível de campos, e o
  **Avaliar**, que calcula uma expressão no quadro escolhido; fora da depuração o painel some e o código ocupa a janela;
- a saída do programa continua no painel de Mensagens.

Os pontos de parada valem durante a sessão da IDE, e dá para mudar com o programa rodando.

## Tempo e memória no fim da linha

Ao parar depois de F7, F8, F9 ou F4, o fim da linha amarela mostra o que o passo gastou desde
o último continuar ou passo, por exemplo `⏱ 12,0 ms  ·  espera 1,60 s  ·  +2,0 KB`:

- **Processador**: o tempo da thread que andou, lido pela IDE no sistema
  (`QueryThreadCycleTime` no Windows, `/proc/<pid>/task/<tid>/schedstat` no Linux). O tempo
  parado no depurador não conta, porque o programa parado não gasta processador. Se o depurador
  não dá a thread do sistema (Go, Python, Java), vale o processo todo, com `(processo)` no fim.
  Sem nem o processo, só o relógio, com `(relógio)` no fim.
- **Espera** (disco, rede, sleep: o relógio menos o processador) aparece só quando passa de
  500 ms. O tempo de relógio vai sempre no status, embaixo.
- **Memória**: a mudança nos bytes alocados, só onde é exata. No Rust, o app com `dev` conta
  no alocador (`aylis::dev_memory`, duas operações atômicas por alocação). O CodeLLDB lê o
  contador ao continuar e ao parar. Um app com o próprio `#[global_allocator]` precisa tirá-lo
  com `cfg(not(feature = "dev"))`. Nas outras linguagens, ainda não.

## Profiler

**Executar → Profiler…**, com o app rodando pelo F9. **Gravar** liga a medição no app (pelo
canal de desenvolvimento, só com `dev`); a IDE busca os blocos a cada 250 ms.

- **A linha do tempo** tem uma faixa por tipo: handlers (form e método), eventos sem handler
  nomeado, layout, pintura, timers, animações, dados (abrir o dataset, filtro e ordem), recursos
  (decodificar imagens), as marcas à mão e a memória do app. A roda dá zoom no ponteiro,
  arrastar anda no tempo e o duplo clique mostra tudo. A dica embaixo diz o nome e a duração
  do bloco sob o ponteiro.
- **A tabela de métodos** traz faixa, nome, chamadas, total, próprio (sem os blocos de dentro),
  média e máximo. Clicar no título ordena, e o duplo clique num handler abre o código dele.

Marcar um trecho à mão (a faixa "Marcas"):

| Linguagem | Como |
|---|---|
| Rust | `let _p = aylis_core::profile::profile("consulta");` (termina no fim do escopo) |
| Python | `with aylis.profile("consulta"): ...` |
| TypeScript | `aylis.profile("consulta", () => ...)` |
| C# | `using (aylis.Profile("consulta")) { ... }` |
| Java | `try (var p = aylis.profile("consulta")) { ... }` |
| Go | `defer app.Profile("consulta")()` |
| C++ | `auto p = app.profile("consulta");` |
| Free Pascal | `Aylis.ProfileBegin('consulta'); ... Aylis.ProfileEnd;` |

Nas outras linguagens, cada handler já entra como um bloco (o tempo inclui a ida e a volta ao
backend). Desligado, cada ponto de medição custa uma leitura atômica.

## Como funciona

A IDE fala o protocolo de depuração do VS Code (DAP) com um adaptador por linguagem:

| Linguagem | Adaptador | De onde vem |
|---|---|---|
| Rust (Windows, Linux, macOS) | CodeLLDB | Ferramentas → Verificar ambiente baixa a última versão do GitHub e guarda em `%LOCALAPPDATA%\aylis\tools\codelldb` (ou usa o da extensão do VS Code, se instalada) |
| Python | `debugpy` | Verificar ambiente instala com o `pip` |
| Go | Delve (`dlv dap`) | Verificar ambiente instala com o `go install` |
| C# | netcoredbg | Verificar ambiente baixa do GitHub (não há versão para macOS) |
| TypeScript | js-debug (o do VS Code) | Verificar ambiente baixa a v1.140.0; o backend roda no Node 22.6+ (`--experimental-strip-types`), não no Bun, para o depurador entrar |
| Java | `aylis-tool jdb-dap` | um adaptador DAP do próprio Aylis sobre o `jdb` do JDK |
| Free Pascal e C++ | CodeLLDB | o mesmo do Rust |

- **Rust:** a IDE compila o executável de desenvolvimento (o mesmo do F9) e o CodeLLDB roda
  o programa. No Windows o LLDB lê os símbolos da Microsoft (PDB): variáveis, tipos e
  passos funcionam. Uma diferença do Visual Studio: na última linha de um bloco (o `}`), as
  variáveis daquele bloco já saíram de escopo.
- **Python:** o `aylis-host` roda o backend com o `debugpy` esperando, e a IDE conecta nele.
- **As outras linguagens do aylis-host** (Go, C#, TypeScript, Java, Free Pascal, C++): o
  `aylis-host` sobe o backend e informa o processo dele (`AYLIS_BACKEND_PID`); a IDE anexa o
  depurador da linguagem a esse processo. No Windows os caminhos dos pontos de parada vão
  com `\` (o netcoredbg compara com o caminho gravado no PDB). Testes:
  `cargo test -p aylis-host --test debug_languages -- --ignored`.

O código fica em `crates/aylis-tools/src/dap.rs` (o cliente do protocolo, testado com os
dois adaptadores em `crates/aylis-tools/tests/dap.rs`) e `crates/aylis-ide/src/debugger.rs`
(a parte da IDE). A IDE não tem mais o "Depurar no VS Code" (saiu em 03/11): depura-se nela.

Detalhes de robustez:
- O CodeLLDB no Windows às vezes não avisa que o programa terminou (fica mudo). A sessão
  vigia o processo depurado e, se ele sumiu sem aviso, encerra sozinha em ~1,5 s.
- O LLDB às vezes avisa duas vezes a mesma parada; antes de cada passo as paradas velhas
  são descartadas, para a linha amarela ser sempre a do passo atual.
- `AYLIS_DAP_TRACE=1` escreve no stderr cada mensagem trocada com o depurador.

**Condição e passagens.** Na janela de código, Depurar → "Condição do ponto de parada…"
(na linha do cursor) pede uma expressão (só para quando for verdadeira: `i == 3`,
`nome == "Ana"`) e a contagem de passagens (`5` para na quinta vez; `>= 5` da quinta em
diante). Vale para Rust (CodeLLDB) e Python (debugpy), e muda na hora com o programa
rodando.

**Executar até o cursor (F4).** Roda até a linha do cursor com um ponto de parada
temporário, que sai quando o programa para; sem depuração em andamento, começa uma.

Ainda não: depurar no Android e no navegador (lá vale o Inspetor do app e o hot reload).

## Editor de código

A janela de código tem o que se usa no VS Code. A inteligência vem do servidor de linguagem
(LSP) de cada uma, que Ferramentas → Verificar ambiente instala:

| Linguagem | Servidor |
|---|---|
| Rust | rust-analyzer (do rustup) |
| Python | python-lsp-server (pylsp) |
| TypeScript | typescript-language-server com o TypeScript 5 |
| C# | csharp-ls (0.16 no .NET 8; a última no .NET 9+) |
| Java | Eclipse JDT LS 1.38 (a IDE cria o `.project` e o `.classpath` com a pasta `src`) |
| Go | gopls |
| C++ | clangd 23 (a IDE gera o `compile_commands.json`) |
| Free Pascal | pasls (o da Castle Engine, compilado com o Lazarus) |

A barra de cima mostra o servidor em uso (ou que está iniciando, ou que falta). Sem servidor,
a IDE usa o próprio índice dos fontes (declarações e busca no texto).

| Tecla | O que faz |
|---|---|
| (digitando), Ctrl+Space | Sugestões; Enter ou Tab aceita |
| Ctrl+Shift+Space, `(` | Ajuda dos parâmetros (o atual em destaque) |
| Mouse parado num nome | A dica: o tipo, a documentação, o erro da linha (no Pascal, a declaração) |
| F12, Ctrl+clique | Ir para a definição |
| Shift+F12 | Referências (no painel de baixo) |
| F2 | Renomear em todos os fontes (sem suporte no servidor, como no pasls: troca a palavra inteira no texto) |
| Ctrl+. | Correção rápida |
| Shift+Alt+F | Formatar o documento |
| Ctrl+Shift+O, Ctrl+T | Ir para o símbolo do arquivo / do projeto |
| Ctrl+P, Ctrl+F12 | Abrir fonte |
| Ctrl+Shift+P | Paleta de comandos |
| Ctrl+Shift+M | Problemas (os erros e avisos vêm enquanto digita) |
| Ctrl+F, Ctrl+H, F3, Shift+F3 | Procurar e substituir (maiúsculas, palavra inteira) |
| Ctrl+G | Ir para a linha |
| Ctrl+Z, Ctrl+Y | Desfazer e refazer |
| Ctrl+/ | Comentar ou descomentar as linhas |
| Alt+↑/↓, Shift+Alt+↑/↓ | Mover ou duplicar as linhas |
| Ctrl+Shift+K | Apagar as linhas |
| Ctrl+Enter, Ctrl+Shift+Enter | Linha nova embaixo / em cima |
| Tab, Shift+Tab, Ctrl+], Ctrl+[ | Recuar e desfazer o recuo das linhas selecionadas |
| Ctrl+L | Selecionar a linha |
| Ctrl+D | A palavra; de novo, mais um cursor na próxima ocorrência |
| Ctrl+Shift+L | Um cursor em cada ocorrência |
| Alt+clique, Ctrl+Alt+↑/↓ | Mais um cursor (no clique, ou na linha de cima/baixo); Esc volta a um |
| Ctrl+Shift+[ e Ctrl+Shift+] | Dobrar e desdobrar o bloco (ou a seta na calha) |
| Ctrl+Shift+\ | Ir para o parêntese que fecha |
| Home | Início do texto da linha (de novo: a coluna 1) |
| Ctrl+=, Ctrl+-, Ctrl+0 | Zoom (o 0 volta ao tamanho normal) |

Parênteses, colchetes, chaves e aspas fecham sozinhos. Nos diálogos (ir para a linha,
renomear, a paleta), Enter confirma, Esc cancela e ↑/↓ andam na lista.

Com vários cursores, digitar, apagar, Enter, as setas, Home, End, copiar, recortar e colar
valem em todos (colar com uma linha para cada cursor distribui as linhas); o Ctrl+Z desfaz
tudo de uma vez. Os blocos dobram pelo recuo, como no VS Code: a linha que fecha (`}`, `end`)
fica à vista, e Exibir → Dobrar tudo / Desdobrar tudo vale para o arquivo. Ir para uma linha
escondida (busca, F12, o depurador) abre o bloco dela.

O minimapa à direita mostra o arquivo inteiro com as cores e a parte à vista; clicar ou
arrastar rola (Exibir → Minimapa liga e desliga). A trilha em cima do editor mostra o arquivo,
o tipo e a função onde o cursor está; clicar nela abre os símbolos do arquivo.

O editor é o componente `CodeEditor` (na paleta, ao lado do `Memo`): os apps também podem
usar, com as cores da linguagem (`syntax`), `minimap`, `folding` e `color_theme`. O `Memo`
comum fica só caixa de texto. O editor não tem borda (nem a do foco).

**Temas do editor.** Exibir → Tema do editor… troca as cores do código em todos os
editores (a escolha fica guardada): Dark+, Light+, Monokai, Dracula, One Dark, Solarized
escuro e claro, GitHub claro e escuro e Nord; "Automático" segue as cores da IDE. Nos
apps, a propriedade `color_theme` do `CodeEditor`.

**Abas.** Os arquivos abertos ficam em abas como as do navegador (o componente `TabStrip`):
o × fecha (com alteração não salva, um ponto aparece no lugar dele e fechar avisa antes de
descartar), o clique do meio também fecha, o "+" abre outro fonte e arrastar troca a ordem.
