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
    no meio de um "Compilar em release". *Esperado:* nas MENSAGENS, "Gerando o código", "Verificando
    o projeto" e a compilação, com os erros clicáveis como antes; o Gerenciador de Tarefas mostra um
    `aylis-tool.exe` enquanto compila; o Parar encerra o `aylis-tool` e o `cargo` juntos. Num
    terminal, `aylis-tool build examples/Paint/Paint.aylisproj` faz o mesmo, e a Calculadora (Free Pascal)
    só gera e verifica.
98. **O Design no código de um form sem aba** (relatado em 10/11). Abra um projeto com dois
    forms, só com o principal aberto; abra o fonte do outro (pelo Pesquisar no projeto ou pela
    lista de arquivos) e clique em Design (ou F12). *Esperado:* abre o form desse fonte numa aba
    nova, e não o último design mostrado. (O mínimo do Rust no "Verificar ambiente" também passou
    a 1.92 desde a 0.5.0, o `rust-version` dos crates: um Rust mais velho aparece como
    desatualizado.)
99. **O aviso de versão nova** (pedido de 10/11, como no NoOBS). Na IDE, Ajuda → Verificar
    atualizações. *Esperado:* "O Aylis está atualizado (versão <a da IDE, hoje 0.9.2>)." Ajuda → "Avisar de novas
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
102. **O relatório do InsightDB e o compilador com o app aberto** (11/11; detalhes nos pendentes).
    - Com o NoOBS instalado aberto, compile o NoOBS do repositório: compila. Com o NoOBS do
      repositório aberto: recusa dizendo o caminho (ou encerra só ele, com "Encerrar o app aberto").
    - Feche e reabra a IDE com o app aberto e clique em Parar: o app fecha.
    - Rótulos ancorados à direita e embaixo acompanham a janela em tela cheia e a célula do
      GridLayout.
    - Pelo Claude: `move_form`, `new_form` com `file`, `designer_snapshot`, `compare_image` com a
      tela antiga, `search_icons`, um `app_command` por um nome repetido (recusa com os caminhos).
    - Chart: `value_format: full`, `label_angle: 45`, `legend_position: bottom`, rosca com
      `slice_labels: outside` e `colors`; barra deitada com `stacked`; DataGrid com `indicator` e
      `highlight_current` desligados.
    - Um botão de aba com `radius: [8, 8, -8, -8]`: cantos de cima redondos e os de baixo abrindo
      para fora.
    - Um form numa subpasta (`move_form` para `forms/painel/x.json`): o código vai para
      `rust/src/forms/painel/x.rs` e o projeto compila.
    - Tirar o evento de um botão cujo handler ninguém mexeu: na geração, o handler sai; com
      código, fica, e o Compilar avisa "não é chamado por nenhum evento".

103. **Renomear no MediaCard** (pedido de 11/11). Num MediaCard com `editable_title` e o
    `on_rename` ligado: o clique no nome abre o campo com o nome selecionado; Enter grava e o
    handler recebe o nome novo; Esc volta o nome; o clique fora (no fundo ou na miniatura) grava,
    e esse clique na miniatura não abre o item. A miniatura, depois, abre normalmente.

104. **A placa de vídeo de cada janela** (11/11; detalhes em `docs/gpu-por-monitor.md`). Com
    `set AYLIS_GPU_SWITCH=warp`, rode `cargo run -p aylis-platform-winit --example gpu_monitor
    --features d2d` e arraste a janela com o mouse de um monitor para o outro: a janela troca de
    placa (o texto mostra "Microsoft Basic Render Driver" ou a Radeon) só ao soltar, e o desenho
    continua igual. Sem a variável, não troca. Nas Opções do projeto, Placa de vídeo tem
    "Seguir o monitor".
    No Windows também: `cargo run -p aylis-platform-winit --example gpu_monitor --features d2d
    -- --toggle 4` troca a `gpu` do form entre a Radeon e o WARP (4 trocas impressas).

105. **R01 e R02 do relatório do Tabernáculo** (11/11). Copie `reproducoes/` do relatório para a
    raiz do Aylis e rode o `rodar.ps1` (as variáveis agora são `AYLIS_*`): nenhum caso cai nem
    fecha com erro, o form secundário fica em tela cheia no monitor dele (com o segundo monitor
    ligado, o `segunda-outro-monitor` vai para ele), e nenhum caso fica `PARADO` depois do
    `request_close`. Num app de verdade: uma projeção sem borda, por cima e sem dona, em tela
    cheia ao nascer, abre direto, sem o Timer que esperava o HWND.

106. **G01 e G08 do relatório do Tabernáculo** (11/11). Num projeto em Rust: ponha no
    `Cargo.toml`, acima do bloco do renderizador, um comentário `# aylis:render: lembrete` e
    uma dependência sua, e mude o renderizador nas Opções do projeto: a dependência fica. Rode
    `cargo fmt` no projeto (o `impl FrmX {}` vazio fica numa linha), ligue um evento novo no
    designer e salve: o handler aparece dentro do `impl` e o projeto compila.

107. **Relatório do Tabernáculo, lotes 1 e 2** (11 e 12/11; detalhes nos pendentes). Arraste um
    arquivo do Explorer sobre uma `FileDropZone` com filtro: o recusado chama `on_reject` e o
    `on_change` traz só os novos. Feche a IDE com um form alterado: avisa uma vez. Num form, um
    Button com `default_button` e outro com `cancel_button`: Enter e Esc clicam neles. Um Timer
    com `one_shot` dispara uma vez. Um form com `close_action` free some ao fechar.

108. **Relatório do Tabernáculo, lote 3: controles e tema** (12/11). Na galeria: o Popover abre
    ao lado do botão e fecha com o clique fora e com Esc; o DateTimeEdit troca o dia com ↑/↓ na
    data e os minutos na hora; o ComboBox com `groups` mostra as cabeças e pula a opção de
    `disabled_items`; o markup do Label com `<i>`, `<small>` e `<color=text_muted>`; três Buttons
    `checkable` com o mesmo `group_index` se comportam como abas. Num tema, `"border_sides":
    [0, 0, 0, 3]` num estilo de Label sublinha; `"border_style": "dashed"` traceja.

109. **Relatório do Tabernáculo, lote 4: layout e texto** (12/11). Um Layout com `gap` 8 e três
    filhos em cima; um FlowLayout com `v_align` center e um Layout com `grow` 1 no meio (empurra o
    resto para a direita); uma GridLayout com um filho em `grid_row` 1, `grid_column` 2; um
    ScrollBox com `gutter` desligado (o conteúdo usa a largura toda). Um Label com `line_height`
    1.6 e `wrap`, e outro com `text_shadow`.

110. **Relatório do Tabernáculo, lotes 5 e 6: mídia, WebView e projeto** (12/11). Um VideoPlayer
    com `file` `https://...mp4` toca no Windows; no fim, a barra fica no fim. Um WebView com
    `background_color` escuro não pisca branco. Crie um projeto novo: o `Cargo.toml` usa o Aylis
    do crates.io (`version = "..."`), e trocar "Aylis do projeto" nas Opções para uma pasta troca
    as dependências. Num form, `"class": "ImageView"`: o Gerar código avisa e sugere `Image`.

111. **Relatório do Tabernáculo, lote 7: IDE e MCP** (12/11). Pelo MCP: `set_property` com um
    argumento errado (`propery`) é recusado com a lista; `set_properties` e `move_component`
    funcionam e o Ctrl+Z desfaz. Na IDE, a árvore do projeto abre fechada, e as pastas que você
    abrir continuam abertas ao reabrir o projeto. Um fonte sem form no editor deixa o Design
    desligado.

112. **Lote 7, o resto: o build e o MCP** (12/11). Compile o Tabernáculo pela IDE e rode `git
    status`: só muda o que a linha "ℹ A compilação mudou…" das Mensagens explicar. Pelo Claude
    Code: `menu` com um texto repetido é recusado com os caminhos, e pelo caminho funciona; no
    "Verificar ambiente", o `dialog_state` mostra cada "Corrigir…" com um rótulo diferente; um
    `build` longo responde "em andamento" e o `task_status` traz o fim; o `output` com `source`
    `program` só traz o app; com dois apps abertos, `app_command` com `app` escolhe um, e o
    `snapshot` aparece como imagem.

113. **Lote 7: form em pasta, frame, tema e testes pelo MCP** (12/11). Na IDE, "Novo form" com a
    pasta `biblia` cria `forms/biblia/...json`. Pelo MCP: `new_frame` com `host` põe o Frame no
    outro form; `set_theme` muda um token e os forms abertos acompanham; `check`, `test` e
    `screenshot` (a foto aparece como imagem) num projeto confiado.

114. **Modo sem plataforma** (12/11). No Tabernáculo, troque o `ForaDaTela` e o Timer de 500 ms
    do `mostruario` por `aylis::headless::Headless` (`open`, `click`, `type_text`, `save_png`): a
    foto sai igual à da janela.

115. **Vídeo por textura e som por endereço** (12/11). Rode `cargo run --release -p aylis-media
    --example video --features d2d -- <um 4K> --stats`: o rodapé diz "textura" e a CPU fica baixa;
    com `--cpu-frames`, "pixels". No Tabernáculo, as projeções pintando por `current_video_frame`
    e `Frame::draw`. Um MediaPlayer com `file` `https://.../musica.mp3` toca, e arrastar a
    posição funciona.

116. **Abas laterais, gap do tema e rodapé do modal** (12/11). Um TabControl com `tab_position`
    `left`: as abas em coluna, Cima/Baixo trocam. Um tema com `"ButtonGroup": {"gap": 0}` cola os
    botões. Um `MessageDialog.footer` com `padding` menor encurta o rodapé. Mudar `width` do form
    com o app rodando muda a janela; `client_width` define a área sem a borda. Redimensionar a
    janela de uma grade: nenhum quadro com a largura de antes. O ícone da bandeja a 150% nítido.

117. **Opacidade de grupo no navegador e o create_in** (12/11). Publique um app para a web com um
    painel em `opacity` 0.5 e dois filhos sobrepostos: o de baixo não aparece através do de cima.
    No Tabernáculo, os 66 livros e as 150 teclas por `create_in` na máquina da igreja (antes, ~6,8
    ms). Editar à mão um handler e rodar `cargo fmt`: o "Sincronizar código" não mostra pendência.

118. **Cantos da janela com a barra própria** (12/11). `cargo run -p aylis-platform-winit --example
    titlebar --features d2d`: no Windows 11 a janela sai com os cantos arredondados, a borda fina
    e a sombra do sistema; maximizada, os cantos ficam retos. Num form com `border_style` custom,
    `window_corners` `small` (raio pequeno) e `square` (retos), também mudando com o app rodando.
    Pare o mouse no botão maximizar da barra própria: o Windows 11 mostra o menu de layouts, e
    escolher um posiciona a janela; o clique no botão maximiza e restaura, com o realce da barra.

119. **Área de transferência central** (12/11; detalhes nos pendentes). No Paint (qualquer
    linguagem), selecione um pedaço, Ctrl+C e cole no Paint do Windows ou no Word; copie uma
    imagem no navegador e cole no Paint do Aylis. Na IDE, copie um Panel com filhos, cole noutra
    aba de form e noutra IDE aberta; cole no Bloco de Notas (sai o JSON), mude um `text` lá,
    copie e cole de volta no designer. Num form com um `Clipboard`, `copy_html` e cole no Word
    (sai formatado) e no Bloco de Notas (sai o texto). Os campos de texto copiam e colam como
    antes.

120. **Verificação de ambiente rápida** (12/11; detalhes nos pendentes). Abra Ferramentas →
    Verificar ambiente duas vezes: a segunda abre na hora. Com o WSL desligado e um projeto sem
    Linux, não pode demorar; as plataformas fora do projeto mostram "guardado há…" e "Conferir".
    Rode `rustup target remove wasm32-unknown-unknown` e Executar no navegador: o F9 percebe.

121. **Pacotes de ferramentas na IDE** (12/11). Monte uma pasta com `aylis-tool packages pack
    --out <pasta> node jdk`, ponha-a nas Opções → Pacotes e clique em "Testar as origens". Em
    Ferramentas → Pacotes de ferramentas, instale o Node e o JDK (o andamento, o Cancelar) e
    remova; o Corrigir do JDK ou do NDK numa máquina sem eles: sem pasta nem espelho, baixa do
    fabricante; com `package_official = off` no `tools.json` e sem origem, o erro traz o passo a
    passo (o endereço, o SHA-256, o comando que confere e a pasta). Suba a pasta no Google Drive, monte o `pacotes.json` com os ids
    (`"drive"`) e use-o como espelho.

122. **Instalador com os pacotes** (12/11). Gere o kit com `aylis-tool installer-ide
    --with-packages node,codelldb` e, numa máquina de teste sem internet, rode o setup ao lado da
    pasta `pacotes`: o Rust e o CodeLLDB vêm do kit; depois, na primeira abertura, o "Configurar
    linguagens" instala o Node do kit. Desinstale e confira que os pacotes, o cache e as compilações saíram rápido.

123. **Automação OLE (`OleObject`)** (12/11; detalhes nos pendentes). Num form, ponha um
    `OleObject` (paleta System), um `ClientDataSet` com alguns registros e o `DataSource`. Num
    botão: `open()`, `set_value("Visible", "true")`, `call("Workbooks.Add")`, `put_data(
    "ActiveSheet.Range(\"A1\")", dataSource)` e `get_value("ActiveSheet.Range(\"A2\").Value")`. O
    Excel abre com os títulos e os registros (datas e valores como número, não texto); a tela
    espera a resposta (a primeira abertura do Excel leva alguns segundos). Com
    `prog_id = WScript.Shell`, o `open` recusa com a mensagem.
    Feche o form sem o `call("Quit")`: o Excel aberto continua com você (largado, não fechado).

124. **Transição de aba sem repintar as páginas** (12/11; detalhes nos pendentes, "Compositor
    em outra thread"). Num form, um `TabControl` cheio (duas ou três abas com muitos controles)
    com `transition` em `slide_left` e depois em `fade`; execute e troque de aba várias vezes,
    também com a janela maximizada num monitor 4K. As páginas andam e esmaecem lisas, sem
    tremer meio pixel nem deixar resto, e no fim ficam iguais a antes (o que passa da borda da
    página pode sumir durante os 250 ms). Com o Profiler aberto, o "quadro · N% repintado"
    durante a troca não pinta os controles das páginas.
    A memória: no painel de memória do app em depuração (ou no Profiler da IDE), a linha
    "· camadas guardadas" sobe durante a troca e volta a zero quando ela acaba, mesmo depois de
    muitas trocas. Em Projeto → Opções do projeto → Memória, desmarque "Camadas automáticas nas
    transições e animações": a troca continua lisa, mas a linha fica em zero (as páginas são
    repintadas). Com a caixa marcada, ponha o `layer_cache` do form em `off` (Object Inspector,
    "Camadas automáticas"): só esse form deixa de guardar. "Camadas guardadas" em 0 MB desliga
    também o `cache_layer` dos controles.

125. **Prefixo de URL próprio (`UrlScheme`)** (12/11; detalhes nos pendentes). Num terminal fora
    do Claude: `cargo run -p aylis-platform-winit --example url_scheme --features cpu`. Com ele
    aberto, digite `aylisexemplo://pedido/42` no Executar (Win+R) e num link do navegador: o
    endereço aparece na janela, ela vem para a frente e não abre outra. Feche, chame o link de
    novo: o app abre com o endereço. Num app publicado com instalador e um `UrlScheme` ligado, o
    link funciona logo depois de instalar, e desinstalar tira a chave
    (`HKCU\Software\Classes\<prefixo>`). Apague a chave do exemplo no fim.

126. **WMI (`WmiQuery`)** (12/11; detalhes nos pendentes). Num form, ponha um `WmiQuery` (paleta
    System), um `ClientDataSet` sem campos ligado no `data_set`, um `DataSource` e um `DBGrid`.
    Com a `query` `SELECT Name, Size, FreeSpace FROM Win32_LogicalDisk`, um botão chama
    `execute()`: a grade mostra os discos, com tamanho e espaço livre como números. Troque para
    `SELECT * FROM Win32_Process` e use `execute_async()` com o `on_done`: a tela não para, e
    todas as colunas aparecem. Em outro `WmiQuery`, `watch` ligado e a `query`
    `SELECT * FROM __InstanceCreationEvent WITHIN 2 WHERE TargetInstance ISA 'Win32_Process'`,
    com o `on_event` mostrando o texto: abra o Bloco de Notas e o evento chega com o
    `TargetInstance.Name` `notepad.exe`. Com `namespace` `\\outro\root\cimv2`, o `execute`
    recusa com a mensagem.

127. **Escala de um componente (`scale`)** (13/11; detalhes nos pendentes). Num form, um `Panel`
    com botões, um `Edit` e um `ComboBox` dentro; ponha `scale` 1.5 no painel (categoria Layout):
    no designer e no app rodando, o painel ocupa o mesmo lugar e tudo dentro sai 1,5 vez maior.
    Clique nos botões perto das bordas (o clique acerta o certo), digite no `Edit` (o cursor fica
    no lugar do texto), abra o `ComboBox` (a lista aparece logo abaixo dele) e passe o mouse para
    ver a dica. No designer, selecione um botão de dentro: as alças ficam em volta dele. Com
    `scale` 0.5 no form inteiro, a tela toda sai pela metade, como um zoom.

128. **A documentação da Landing no celular** (13/11; detalhes nos pendentes). Publique a Landing
    e abra no celular: no topo da página principal aparece "Documentação"; toque. A página ocupa
    a tela toda, com o texto e as tabelas legíveis. O botão ☰ do topo abre o índice na tela
    inteira; tocar numa seção abre a seção, tocar numa página fecha o menu e mostra a página.
    Digite na busca: o menu abre com os achados, e um toque leva à página. "Anterior" e
    "Próxima" ficam um embaixo do outro. Gire para paisagem ou abra no PC: o índice volta à
    esquerda, sem o ☰.

129. **O instalador da IDE e a escolha na primeira abertura** (13 e 16/11; detalhes nos
    pendentes). Rode o `aylis-ide-<versão>-setup.exe`: uma página de componentes só, com "Aylis
    IDE e Rust (obrigatório)" cinza e marcado e o atalho, sem linguagens nem plataformas. Nos
    detalhes, "Configurando Rust…", com os acentos certos. Abra a IDE: o "Configurar
    linguagens" abre sozinho, com as linguagens e as "Plataformas de destino" (Android,
    Navegador). Marque Python e Android e instale: o andamento mostra os dois, e no fim Rust e
    Python aparecem prontos e o Android também. Feche e abra a IDE de novo: o assistente não
    abre mais (fica em Ferramentas → Configurar linguagens). Com uma conversa do Claude ligada
    ao MCP e a IDE aberta, rode o instalador de novo por cima: a IDE fecha sozinha, a instalação
    termina sem perguntar se quer reiniciar e não sobra `*.antigo*` na pasta instalada; na
    conversa, a próxima chamada do MCP já fala com a ponte nova (se o Claude não religar, o MCP
    dessa conversa fica desligado até religar). Em Ferramentas → Opções da IDE → Pacotes,
    sem espelho (a origem oficial ligada); com `/ESPELHO=<url>` no setup, o espelho aparece lá. Na pasta instalada
    (`%LOCALAPPDATA%\Programs\aylis\aylis`), não há `examples`, e um projeto novo em Rust
    compila. Pelo MCP, `component_docs` do Button responde.

130. **ComboBox: linhas da lista e a barra** (13/11; detalhes nos pendentes). Num form, um
    `ComboBox` com 30 opções e, no Object Inspector, "Linhas da lista aberta"
    (`drop_down_count`) em 5. Rode: a lista abre com 5 linhas e uma barra fina à direita.
    Role com a roda e arraste a barra até o fim: aparece a última opção, e soltar a barra não
    escolhe nem fecha. Ponha o ComboBox perto do pé do form, com 20 linhas: a lista abre para
    cima ou encolhe para caber, sem sair da janela. Com 5 opções e 10 linhas, não há barra.

131. **`app_command`: arrastar e Ctrl/Shift** (13/11; detalhes nos pendentes). Num app rodando
    pela IDE, com uma `ListBox` de seleção múltipla (`lstNomes`) e dois painéis, um
    `draggable` com `drag_data` e outro `accepts_drops` com um `on_drag_drop` que mostra o
    texto: peça ao agente, pelo MCP, `{"cmd":"click","target":"lstNomes","y":14}` e depois
    `{"cmd":"click","target":"lstNomes","y":70,"modifiers":"Ctrl"}`: a lista fica com o 1º e o
    3º marcados na tela. `{"cmd":"drag","from":"pnlA","to":"pnlB"}`: o `on_drag_drop` mostra o
    texto. `{"cmd":"click","target":"btnX","button":"right"}` abre o menu de contexto do botão.

132. **Marca d'água em negativo (`blend_mode`)** (13/11; detalhes nos pendentes). Num form com
    um `VideoPlayer` tocando um vídeo, ponha por cima um `Label` grande branco (ou um `Svg` de
    logo branco) e, no Object Inspector, "Modo de mistura" em `difference`: onde o vídeo é
    claro a marca fica escura, onde é escuro fica clara, e acompanha o vídeo quadro a quadro,
    sem piscar. Teste também `exclusion` e `multiply`, com `opacity` 0,5. Confira com o
    renderizador Direct2D e com o Skia (Projeto → Opções) e, publicado para a web, no
    navegador.
133. **Recortar, Copiar e Colar no editor de código** (13/11). Com um fonte aberto na IDE,
    clique com o botão direito: no topo do menu vêm Recortar, Copiar, Colar e Selecionar tudo,
    antes de Arquivo, Editar... Selecione um trecho, recorte e cole em outro lugar (o Ctrl+Z
    desfaz cada um); com vários cursores, o colar põe uma linha em cada. Na janela própria
    (Arquivo → Abrir em outra janela), os mesmos itens ficam no menu Editar.
134. **Painel deslizando por cima de uma página** (13/11; detalhes nos pendentes, "Compositor em
    outra thread"). Rode `cargo run --release -p aylis-platform-winit --example transitions
    --features d2d` (com `AYLIS_RENDERER=d2d`; no monitor 4K a 150%, acrescente `1920 1080
    0.6667`): a janela troca de aba deslizando e esmaecendo e passa um painel cheio por cima
    da página, sozinha, e fecha mostrando a tabela no terminal. Olhe se tudo anda liso e se,
    durante e depois de cada movimento, nada fica com resto, borrado ou meio pixel fora (os
    textos e as bordas da página de trás iguais a quando está parada). Repita com
    `AYLIS_RENDERER=gpu` e `cpu`.
135. **O hífen no nome do pacote e a árvore sem "(fechado)"** (13/11; detalhes nos pendentes).
    Na IDE, Projeto › Forms mostra só os nomes, sem o "(fechado)" dos forms não abertos. No
    RTSPlayer, volte o pacote de `client/rust/Cargo.toml` para `rtsplayer-app` e publique para
    Android (e para a web): o APK sai e o app abre no aparelho; o executável do Windows continua
    `rtsplayer-app`.
136. **O motivo exato quando um pacote não vem** (13/11; detalhes nos pendentes). Gere o
    instalador e rode-o (o Rust e o CodeLLDB) em três situações: sem internet e sem a
    pasta `pacotes` ao lado; com a pasta `pacotes`, mas com um arquivo trocado por outro de mesmo
    nome; sem internet e com a pasta `pacotes` sem um dos arquivos. *Esperado:* no fim, a caixa
    diz em cada linguagem a causa ("sem internet", "os arquivos não estão ao lado do instalador",
    "um arquivo não confere com o SHA-256 do Aylis"), e a lista de detalhes começa pela mesma
    frase, com o caminho da pasta e o nome do arquivo. Na IDE, Ferramentas → Configurar
    linguagens, sem internet: a linha do ⚠ diz "Sem internet" antes da lista das origens.
138. **O relatório do RTSPlayer de 13/11** (detalhes nos pendentes). No RTSPlayer, tire o
    contorno do `on_resize` (o `set_height` da lista) e ponha o modelo da linha com `align: top`:
    numa janela alta, a lista mostra todos os registros que cabem e rola até o último, com as
    alturas de `sizes`. Num app com um botão que troca `topmost`, a janela vai para a frente das
    outras e sai. Com `publish.android.abis` = `["arm64-v8a"]` (pelo MCP, `set_project_setting`),
    publique para Android: o APK traz só `lib/arm64-v8a`. Feche a IDE e, num Claude Desktop com o MCP da ponte compilada, peça
    `open_project`: abre a IDE instalada (ou o erro lista onde procurou). Um `build` com
    `max_wait` 200 responde em até ~50 s ("ainda compilando").
139. **Android de 32 bits** (14/11; detalhes nos pendentes, relatório do RTSPlayer, item 5). Em
    Opções do projeto → Android, marque "Também 32 bits" e publique: o publicar instala o pacote
    "Skia para Android de 32 bits" (Ferramentas → Pacotes mostra) e o APK traz `lib/armeabi-v7a`,
    `lib/arm64-v8a` e `lib/x86_64`. Desmarque e publique de novo: o APK volta às duas. Se tiver um
    TV box ou celular de 32 bits, instale e abra o app.
140. **Barras do sistema, tecla Voltar, gestos e fim da rolagem no celular** (14/11; detalhes nos
    pendentes, "Pedidos de 14/11"). Num app de teste no emulador `aylis_pixel` ou no ZenFone:
    - um `SystemBars` com `status_bar_color` vermelho e `icons` `light`: a barra fica vermelha com
      ícones claros; `status_bar_visible` falso: a tela fica cheia;
    - um `on_back` no form que troca de frame e responde `handled`: a tecla Voltar volta ao frame
      anterior em vez de tirar o app da frente; sem tratar, o de antes;
    - um `NavigationDrawer` fechado: deslizar da borda esquerda abre, deslizar para a esquerda fecha;
      um `TabControl` com `swipe`: o dedo troca de página; um `Image` com `zoomable`: a pinça
      aproxima e o dedo arrasta; uma `VirtualList` com `swipe_actions` "Excluir": deslizar o item
      mostra o botão, e o toque chega no `on_swipe_action`;
    - uma lista rolada até o fim: no Android 12 ou mais o conteúdo estica e volta; num Android 11, o
      brilho na cor do tema; `overscroll` `none` desliga.
141. **Testes do projeto em Java, C++ e Pascal** (14/11). Na Calculadora, troque a linguagem para
    Java, C++ e Pascal e rode a ferramenta `test` do MCP da IDE (não há item de menu nem
    subcomando do `aylis-tool` para isso): os testes de `tests/` rodam e o
    resultado aparece por teste; quebre um de propósito e ele aparece como falha, com a linha.
142. **Novo DataModule com pasta e o tema chegando ao app** (14/11). Projeto → Novo DataModule…:
    o diálogo pede o nome e a pasta, e o arquivo vai para ela. Com o app aberto pelo F9, troque o tema
    pelo MCP (`set_theme`): o app muda sem reiniciar.
143. **Clique com Ctrl, arrastar e teclas no `McpServer` do app** (14/11). Num app com `McpServer`, peça
    a um agente: Ctrl+clique em dois itens de uma lista de seleção múltipla (`ui_click` com
    `modifiers`), arrastar um item (`ui_drag`) e Ctrl+A (`ui_key`).
144. **Compartilhar no Windows** (14/11). Um `AppLink` com `share` de um texto e de um arquivo: abre a
    folha "Compartilhar" do Windows sobre a janela, e o e-mail ou outro app recebe o texto e o
    arquivo. Um arquivo que não existe dá a mensagem em vez de abrir a folha.
145. **Um form em vários monitores** (14/11; detalhes em `docs/gpu-por-monitor.md`). Na máquina dos
    seis monitores: `cargo run -p aylis-platform-winit --example span_monitors --features d2d` com o
    form entre dois monitores de placas diferentes: duas janelas sem moldura, uma só na barra de
    tarefas; o mouse passando de um para o outro sem piscar; o ComboBox perto da divisa abrindo no
    monitor dele; `-- --full` cobrindo todos; desligar um monitor com o app aberto refaz as janelas.
146. **CI com agente, lint, cobertura e IDE sem janela** (14/11; detalhes nos pendentes, item 38).
    - `aylis-tool lint` não lista nada; um `Color::rgb(1, 2, 3)` num `paint` aparece com a linha.
    - Novo projeto: a caixa "Guia para agentes de IA" vem marcada e o projeto ganha `AGENTS.md`,
      `CLAUDE.md` e `.mcp.json`; o Claude Code aberto na pasta vê o `aylis` no `/mcp`.
    - F9, clique num botão e Projeto → Cobertura de handlers…: o clicado em RODARAM; o clique num
      handler abre o código; Limpar zera. Repita em Python ou Pascal.
    - Profiler: Salvar rastro… como `.json` abre no ui.perfetto.dev; `.aylisprof` reabre.
    - Pelo MCP: `profile` start/stop com o app aberto, `recording` `source: ide` e `coverage`.
    - `aylis-tool recording-to-test <gravação>` e `aylis-tool ide-script roteiro.json --headless`.
    - No GitHub: o segredo `ANTHROPIC_API_KEY` e um pull request de teste (vem a revisão; um teste
      quebrado traz a triagem); na primeira CI, gravar as bases com `aylis-tool coverage-check
      ... --update`.
    - Um app com `Analytics` e `performance` ligado: o `analytics.json` traz os tempos.
147. **Pacotes do Free Pascal e do WiX** (14/11). Ferramentas → Pacotes de ferramentas: instale `fpc`
    e `wix` (o `wix` do fabricante; o `fpc`, montado pelo Aylis, da pasta de pacotes ou do
    espelho do Aylis). Um projeto em Pascal compila sem
    o Lazarus; o publicar com instalador `.msi` usa o WiX do pacote.
148. **O editor com um fonte enorme** (14/11; detalhes nos pendentes, item 26). `cargo run --release
    -p aylis-controls --example large_text 1000000`: digitar, apagar, Ctrl+Z e colar sem travar; a
    cor da sintaxe acompanhando ao rolar.
149. **A documentação do site sem comparações com outros frameworks** (14/11). Na Landing publicada,
    as páginas dos componentes trazem o "O que faz" de toda propriedade, e nenhuma compara o Aylis com
    outro framework.
150. **Dados de versão e assinatura no Windows** (14/11; detalhes nos pendentes e em
    `docs/publicar-windows.md`). Publique um app (Executar → Publicar → Windows, com instalador) e
    abra Propriedades → Detalhes do `.exe` em `dist/desktop`. *Esperado:* empresa, produto, versão
    e nome original do projeto. Sem certificado: a publicação avisa uma vez "sai sem assinatura" e
    termina. Com o certificado (Ferramentas → Opções → Assinatura, a impressão digital do
    `certmgr.msc`): as mensagens mostram o `signtool` (com `*` no lugar de uma senha de `.pfx`), e
    Propriedades → Assinaturas Digitais do `.exe` e do instalador mostram a empresa e o carimbo de
    tempo; `signtool verify /pa /v <arquivo>` passa. Ferramentas → Verificar ambiente: o item
    "Assinatura de código". Depois, o Cortex XDR e o SmartScreen com o app assinado.
151. **O ponteiro no `on_click`** (14/11; detalhes nos pendentes). Num `on_click` de um Button, leia
    `ui.click_pointer()`: clicando com o botão esquerdo e com Ctrl, vêm a posição, o botão e o
    Ctrl; pelo Enter, `None`. No celular, `kind` é toque. Pelo `aylis-host`, um backend em Python
    recebe `event["pointer"]` no clique do mouse e não no Enter.
152. **Muitas câmeras no Direct2D** (14/11). No RTSPlayer com a grade de câmeras (ou
    `cargo run --release -p aylis-d2d --example camera_frames 600 1920 1080 36`): o uso de CPU e a
    fluidez com 16 ou mais câmeras, e a memória da placa estável com o tempo.
153. **Configurações de pintura em execução e por form** (14/11; detalhes nos pendentes e em
    `docs/video-e-pintura.md`). Num app com câmeras no Direct2D, ponha um `RenderSettings` (paleta
    System) e, num botão, troque `frame_reuse_kb` para `0` e de volta para `1024`: o uso de CPU
    muda na hora, sem reabrir. Num segundo form, `frame_reuse_kb = 0` no Object Inspector: só ele
    deixa de reaproveitar. Troque `auto_layers` para falso e veja uma transição de abas sem a
    camada; `layers_mb = 0` e as camadas somem no próximo quadro. No designer, trocar o
    `RenderSettings` não muda nada na IDE, e as propriedades não aparecem no Object Inspector nem
    no JSON do form.
154. **A tela Sobre** (14/11). Ajuda → Sobre o Aylis: o logo da IDE à esquerda, ao lado "Aylis IDE" e
    "Versão 0.9.2" (a da IDE) em destaque, e a descrição embaixo; em English (Exibir → Idioma), o texto
    traduzido.
155. **A orientação da tela no celular** (14/11; detalhes nos pendentes e em
    `docs/funcionamento.md`, "Orientação da tela"). Na IDE, Opções do projeto → Geral →
    *Orientação da tela*: "Em pé" grava `"orientation": "portrait"` no `.aylisproj`, e Opções do
    projeto → Android mostra que a orientação está em Geral; "Ver o manifesto final" traz
    `android:screenOrientation="portrait"`. Execute no Android (emulador `aylis_pixel` ou celular):
    girando o aparelho, o app fica em pé. Num segundo form com `orientation = sensor_landscape`
    (Object Inspector, categoria Layout), aberto por um botão: a tela deita ao abrir e volta em pé
    ao fechar (e com o Voltar). Num botão, `frmX.orientation = landscape` e depois `project`: gira
    na hora e volta. `Screen.set_orientation("any")`: o app passa a girar com o aparelho. No app de
    desenvolvimento, `adb shell setprop debug.aylis.orientation landscape` e reabrir: deitado,
    mesmo com o projeto em pé; publicado em release, a propriedade não muda nada. No iPhone e no
    iPad: veja `docs/testes-em-dispositivos.md`.
156. **O menu de texto dos campos e as alças no celular** (14/11; detalhes nos pendentes). Num
    form com um `Edit`, um `Edit` com `password`, um com `read_only` e um `Memo`, no Windows:
    o botão direito no texto abre Recortar, Copiar, Colar e Selecionar tudo (Recortar e Copiar
    apagados sem seleção; no `read_only`, só Copiar e Selecionar tudo; na senha, sem Copiar e
    Recortar). Colar texto copiado de outro app; Ctrl+Z desfaz a colagem. Com um `popup_menu` no
    form, abre o do app. No Android (emulador `aylis_pixel` e celular): segurar o dedo numa
    palavra seleciona a palavra, aparecem as duas alças na cor de destaque e o menu embaixo; soltar
    o dedo não fecha o menu; Colar traz a senha copiada de outro app (o gerenciador de senhas, o
    navegador); arrastar as alças aumenta e diminui a seleção sem rolar a tela; tocar fora fecha o
    menu e a seleção fica com as alças.
157. **A anotação de texto sobre um print (Memo só texto que cresce)** (14/11; detalhes nos
    pendentes, pedido do NoOBS, itens 2 e 3). `cargo run -p aylis-platform-winit --example
    annotation --features cpu`: o "Clique no print para anotar" aparece só com as letras, sem
    fundo, borda nem barra. Clique no print (na metade escura e na clara) e digite: a caixa nasce no
    clique e cresce para a direita; Shift+Enter desce uma linha e ela cresce para baixo; Backspace
    na linha mais larga encolhe. Selecione com o mouse: o destaque é translúcido (o print aparece
    por baixo); o cursor aparece nos dois lados (o contorno claro no escuro). Repita no Direct2D:
    `--features cpu,d2d` com `AYLIS_RENDERER=d2d`. Na IDE, num form, um `Memo` e um `Edit` com `auto_size` e
    `plain` (rótulo "Só o texto" no Object Inspector): crescem enquanto se digita no app rodando.
    As imagens de referência estão em `crates/aylis-controls/tests/visual/plain_memo-*.png`.
158. **O recorte em RGBA, o arrasto a 4K e o form que vem para a frente** (14/11; detalhes nos
    pendentes, pedido do NoOBS, itens 4, 5 e 6).
    - Item 6: `cargo run -p aylis-platform-winit --example foreground --features cpu`; clique
      noutro app (o navegador) e continue clicando nele: em 5 s o "Print" abre na frente, com o
      cursor no campo (digite e o Esc fecha) e a linha `na frente: sim`. Repita com
      `AYLIS_FORCE_FOREGROUND=0`: o form fica atrás, piscando na barra (se o Windows deixar ir
      para a frente mesmo assim, foi porque a máquina estava parada). No NoOBS, com
      `force_foreground: true` no form do print, abra pelo atalho global com outro app à frente:
      o Esc fecha o print, e não o app de trás.
    - Item 5: `cargo run --release -p aylis-platform-winit --example drag_4k --features d2d`:
      as linhas `_parte` e `paintcanvas` abaixo de 5 ms nos dois renderizadores, e `pixels
      diferentes da pintura inteira: 0`. No Paint da galeria (ou no NoOBS), com uma imagem
      grande, o lápis e a seleção arrastada sem engasgar no Skia (`AYLIS_RENDERER=cpu`).
    - Item 4: no NoOBS, copiar o recorte pelo `ui.render_rgba(form, rect, 1.0)`: a imagem colada
      igual ao que estava na tela (as anotações e o Memo), sem a moldura nem um pixel deslocado.
159. **O print sobre todos os monitores, em pixels físicos e aberto na hora** (14/11; detalhes nos
    pendentes, pedido do NoOBS, item 1).
    - `set AYLIS_SPAN_SPLIT=2` e `cargo run --release -p aylis-platform-winit --example
      print_overlay --features d2d -- --auto`: o print cobre a tela inteira (as duas metades) 20
      vezes seguidas, sem moldura, sem botão na barra de tarefas e sem aparecer antes num pedaço
      só; no fim, `clique no pixel certo: sim`, as mesmas janelas antes e depois e a mediana de
      abrir perto de 75 ms. Sem o `AYLIS_SPAN_SPLIT` (uma janela), o mesmo.
    - `AYLIS_SPAN_SPLIT=2 cargo run -p aylis-platform-winit --example span_monitors --features d2d
      -- --auto`: no fim, `em pixels físicos: form 1500 × 780` (a 150%) e de volta a 1000 × 520.
    - Na IDE, num form, `physical_pixels` (rótulo "Pixels físicos", em Layout) no Object
      Inspector; rodando, o form com 1 unidade = 1 pixel (um Label com `font_size` 12 sai menor
      num monitor a 150%).
    - No NoOBS, o form do print com a receita dos pendentes: abre pelo atalho em menos de ~100 ms,
      a seleção bate com o print pixel a pixel; o caso de dois monitores de escalas diferentes está
      em [testes em dispositivos](testes-em-dispositivos.md).
163. **A janela sem o branco da abertura e a tela cheia** (19/11; detalhes nos pendentes). Num
    tema escuro, abra a IDE e um app de exemplo: a janela aparece já pintada, sem um quadro
    branco antes. `cargo run -p aylis-platform-winit --example window_start --features d2d --
    full --stay`: nasce em tela cheia cobrindo a barra de tarefas; `full_later`: abre comum e
    vai para a tela cheia; `back`: nasce em tela cheia e volta ao tamanho de antes.
162. **O acesso tipado e os handlers tipados** (19/11; detalhes nos pendentes). Num projeto novo,
    um Button, um DateEdit e um TrackBar com dois cliques em cada: os handlers nascem com quem
    disparou na classe (`sender: Button`) e o valor no tipo (a data, o número); no código,
    `self.form.set_window_state(ui, WindowState::FullScreen)`, a cor como `Color` e as margens
    como `Bounds` (nada de texto). Um handler do jeito de antes (`args: &EventArgs`) continua
    funcionando. Troque a linguagem nas Opções do projeto e confira o mesmo nas outras.
    Os nomes do projeto: crie um tema em `themes/` com uma variante `Button.perigo` e um
    segundo form; no código, `self.btnOk.set_style(ui, ButtonStyle::Perigo)`,
    `set_theme(ui, ThemeName::...)` e, num Frame, `set_form(ui, FormName::FrmOutro)` (o
    autocompletar mostra os do projeto). Acrescente outra variante ao tema e gere o código: ela
    aparece no `ButtonStyle`. No Object Inspector, o `item_form` de um VirtualList ganha a lista
    dos forms, como o `form` do Frame.
    As listas em estrutura: num ListView, num TreeView e num Chart, o `items`/`series` no form é
    JSON (`[{"title": "Ana", "detail": "Vendas"}]`, nós com `children`, séries com `values` e
    `kind`); no código, `ListViewItem`, `TreeNode` e `ChartSeries` com os campos tipados. No
    DataGrid, `columns` é `[{"field": "sigla", "width": 44}]` e o arraste da coluna no designer
    regrava o JSON. Abra o IBGE e o Relatórios: as grades e o mestre-detalhe funcionam como
    antes. No Object Inspector, o "…" do `items` de um TreeView abre o editor de coleção:
    acrescente um nó, troque o texto, marque `expanded` e, em "Editar… (0)", acrescente um
    filho; ao confirmar, a árvore no designer mostra o nó com o filho.
161. **Reveal Highlight** (18/11; detalhes nos pendentes). `cargo run -p aylis-platform-winit
    --example reveal --features d2d`: passe o mouse pela grade de botões; o fundo do botão sob
    ele acende com uma luz que segue o mouse, e as bordas dos vizinhos acendem do lado mais
    perto; o painel em volta acende a borda quando o mouse chega perto dela. Tire o mouse da
    janela: a luz some. Na IDE, num Button, `style` = `reveal`; num tema do projeto,
    `"Button": { "reveal": "both" }` em `styles` liga em todos os botões.
160. **Form transparente, redondo e com vidro fosco** (18/11; detalhes nos pendentes).
    `cargo run -p aylis-platform-winit --example shaped_forms --features d2d`, com
    `AYLIS_RENDERER=d2d`: o "Vidro fosco" com os cantos arredondados, o fundo branco translúcido
    e o que está atrás desfocado (arraste-o por cima de um texto); o form redondo azul, sem
    moldura, arrastado pelo fundo. As bordas sem serrilhado. Clique fora do círculo, dentro do
    quadrado dele: o clique vai para a janela de trás. `--no-blur`: sem o desfoque, o fundo
    só translúcido. Na IDE, num Form, `window_shape` = `ellipse` (rótulo "Forma da janela"): o
    designer mostra o form redondo; `transparent` e `shape_radius` no Object Inspector. No
    exemplo, a estrela (`window_shape = path`) translúcida com o desfoque e as pontas suaves, e
    o vidro com o canto de baixo à esquerda quase reto (`shape_radii` = `32,32,32,4`). Na IDE,
    `shape_path` com `M 5 0 L 10 5 L 5 10 L 0 5 Z`: o form vira um losango no designer.

164. **A Landing no celular** (11/11; detalhes nos pendentes, item 72). Publique de novo a página e
    os demos (`aylis-tool publish ... --target web`, `ide-web`, `demos`) e abra no celular: a
    rolagem acompanha o dedo sem engasgar (de novo em 22/11: o `ScrollBox` rola por cópia; no
    ZenFone a inércia foi de 12 para 30 quadros/s, veja "Ajustes de 22/11" nos pendentes; antes,
    republique a página, porque o service worker guarda o `pkg/` pelo nome); o 3D do topo não aparece no celular (escondido pela
    vista) e as ferramentas do navegador não mostram o `demos/3d` baixado; no PC o lingote aparece.
    Na documentação, a página do `Model3D` num navegador sem WebGPU (no Chrome, `chrome://flags`
    → WebGPU desligado, ou um iPhone antes do iOS 26) mostra o modelo girando.
    Na documentação, a página de um componente leve (Button, Valve, KpiCard...) mostra o
    componente ao vivo sem baixar nada além da página (aba Rede do navegador); a de um
    componente pesado (Gauge, DataGrid, Model3D) baixa o wasm da família dele (o Additional
    ~3,1 MB comprimido, antes ~4,5 MB). A página inicial baixa ~2,6 MB comprimido (antes ~3,2 MB).
165. **Uma pasta de compilação para todo o Rust** (11/11; detalhes nos pendentes). Abra a IDE
    recompilada e veja `%USERPROFILE%\.cargo\config.toml`: `[build] target-dir` com a pasta das
    Opções e `[profile.dev.package."*"] debug = false`, cada um com o comentário "Gravado pelo
    Aylis". Num projeto Rust qualquer, fora do Aylis, `cargo build` no terminal compila nessa
    pasta, e não num `target/` do projeto. Num projeto do Aylis já compilado pela IDE, `cargo
    build` no terminal termina em segundos, sem recompilar as dependências. Em Ferramentas →
    Opções → Rust e Python, desmarque "Usar esta pasta em todo o Rust desta máquina" e dê OK: as
    duas chaves saem (o arquivo some, se só tinha elas); marque de novo e elas voltam.
166. **O gerador e o projeto** (23/11). Num projeto Rust novo (a IDE recompilada):
    - um form `fraMenu` com um botão, e o principal com um `Frame` de `form: "fraMenu"` numa
      `TabStrip`; num handler, `ui.create_form_instance("frmPrincipal")` e `ui.show` dela, e
      depois arraste uma aba para fora. *Esperado:* a segunda janela vem com o menu dela (o
      botão responde nela), e a aba arrancada abre o form, não uma janela genérica (a aba
      arrancada da segunda janela depende também da busca do form pela classe, no núcleo);
    - um DataModule com "Criar no início" e um `on_create` que escreve no log. *Esperado:* a
      linha sai ao executar, antes da janela;
    - pelo acesso tipado, uma coluna `ColumnSpec { field, title, ..Default::default() }` posta
      num `DataGrid` com `set_columns`, e `set_items` de um `ComboBox` com `["", "A", "B"]`.
      *Esperado:* o texto da coluna aparece com a cor do tema; o `items()` volta os três, e
      `item_index` 1 é o "A";
    - `self.lblX.set_font_weight(ui, Some(FontWeight::SemiBold))` com `use aylis::prelude::*`
      compila; `cargo clippy -- -D warnings` no crate do projeto não reclama do `generated/`;
    - num botão, `set_icon(ui, "material:info")` com um ícone que nenhum form usa, e compile no
      terminal com `cargo build` (sem a IDE). *Esperado:* o ícone aparece; um nome que não existe
      (`"material:nao_existe"`) dá um aviso do cargo na compilação e, em debug, uma linha no
      stderr ao pintar.
167. **Dados, gráficos e desenho** (23/11). Num app de
    exemplo com um `DataGrid` de muitas linhas e um `Chart` (a IDE recompilada):
    - um dente da roda sobre a grade. *Esperado:* 3 linhas, com o passo padrão e com outro
      `wheel_step` no app; no touchpad, a grade acompanha os dedos sem saltar;
    - num botão, `display_width` de um campo para 320 e `display_label` para outro texto.
      *Esperado:* a coluna muda na hora, e o registro atual continua o mesmo;
    - no `Chart` de linhas, `border_color: transparent`. *Esperado:* some a moldura, e a grade
      fica; com uma cor, só a moldura muda. Um `Chart` `stacked_bar`: os pedaços de cada coluna
      encostam, sem o fundo entre as cores;
    - um `Label` com `⚠ Atenção ℹ 😀` no Direct2D (o padrão no Windows), no Skia e no navegador.
      *Esperado:* `⚠` e `ℹ` na cor do texto nos três, e o `😀` colorido; `⚠️` (com U+FE0F)
      colorido. No navegador, a conferir no Chrome e no Firefox.
168. **O núcleo e os controles** (23/11). Num app de exemplo (a IDE recompilada):
    - um form com uma grade de 64 frames e um `Timer` de 1 s que põe, em cada um, o mesmo
      `visible` num rótulo e lê `ui.bounds` do frame; o Profiler da IDE aberto. *Esperado:* o
      handler do timer em menos de 1 ms, sem "layout de form" a cada volta; mudando de fato o
      `visible` de um rótulo, só aquele frame é refeito;
    - num frame desenhado em 640×360, um `Label` ancorado à direita e embaixo com `visible`
      desligado; embuta o frame numa área grande e só então mostre o rótulo. *Esperado:* ele
      aparece no canto da área, e não perto de (640, 360);
    - uma `TreeView` com um `on_dbl_click` que escreve no log, e `leaf_layout: icon_in_arrow`
      sem ícones; dê um duplo clique num nó e ponha um ícone numa folha pelo código.
      *Esperado:* uma linha no log por duplo clique (com o caminho do nó); o ícone aparece no
      lugar da seta e nenhum texto anda;
    - um tema com o estilo `PopupMenu` escuro e um `TrayIcon` com `popup_menu`; clique direito
      no ícone da bandeja. *Esperado:* o menu da bandeja com o fundo, o texto e o destaque do
      estilo, como o menu dentro do app. Com `menu_theme: dark` e sem o estilo, o menu escuro do
      tema escuro do Aylis (o tom muda um pouco em relação ao de antes: diga se incomoda);
    - duas janelas do form principal (`create_form_instance`) com uma `TabStrip` de páginas com
      um `Edit`; arranque uma aba da segunda janela. *Esperado:* abre outra instância do form
      (com a faixa e os handlers), não a janela genérica, e dá para digitar no `Edit` sem
      clicar; com as duas janelas, clique na que não foi aberta por último e abra um modal por
      um atalho: ele fica preso a ela;
    - uma `VirtualList` que chama `refresh` no `on_item_click`, rolada até o meio; dê um duplo
      clique num item. *Esperado:* o `on_item_dbl_click` chega;
    - um `Edit` de altura fixa (32) com `error` preenchido. *Esperado:* a caixa continua com 32
      e o erro aparece embaixo dela; com `auto_size`, como antes;
    - um `Label` com `wrap`, `line_height: 1.35` e `highlight` de uma palavra do texto.
      *Esperado:* a palavra com o fundo marcado;
    - um projeto com a fonte "Roboto" que troca entre claro e escuro por `ui.set_theme` de um
      tema embutido. *Esperado:* a fonte continua a Roboto (depende da linha do `install_with`
      que a sessão do gerador põe; até lá, `ui.set_project_font("Roboto")` no início do app);
    - no app que relatou, o `Timer` de um frame posto por `create_in` num modal e o
      `minimap: false` do `CodeEditor` no layout: os testes passam pelo `set_prop`, então diga
      o que o app faz de diferente se ainda falharem.
169. **As janelas e a entrada no Windows** (23/11). O exemplo `window_checks` tem um cenário por item (`cargo run -p
    aylis-platform-winit --example window_checks -- <cenário>`, a lista no começo do arquivo), e o
    `multi_monitor` serve para o arrasto. O que não deu para conferir aqui:
    - com os dois monitores ligados, o `multi_monitor` (o do segundo monitor em tela cheia):
      arraste um item da tela cheia para uma célula da janela comum do principal, e o contrário.
      *Esperado:* o soltar chega nos dois sentidos, e o rótulo do arrasto (o texto junto do mouse)
      acompanha de um monitor para o outro, por cima da tela cheia;
    - um app publicado com o form principal `window_state: maximized`, aberto por um clique
      duplo no Explorer. *Esperado:* a janela fica ativa, e o teclado (um atalho) vale sem clicar
      nela;
    - `window_checks topmost` e o Gerenciador de Tarefas com "Sempre visível" por cima dela.
      *Esperado:* em até 2 s ela volta para cima do Gerenciador, sem tirar o foco de onde ele
      está;
    - `window_checks blur`, posto para abrir no logon (um atalho na pasta `shell:startup`), e a
      máquina reiniciada (nos primeiros 8 s ele desliga e religa o vidro, de 2 em 2 s; confira
      no começo e depois). *Esperado:* o vidro, e não o preto. Com ele aberto, desligue e ligue os
      "Efeitos de transparência" (Configurações → Personalização → Cores). *Esperado:* o vidro
      volta sozinho, sem desligar o `blur` nem reabrir;
    - `window_checks text`, e no `Edit` o teclado virtual do Windows (Win+Ctrl+O), o painel de
      emoji (Win+.) e o preenchimento do gerenciador de senha que você usa. *Esperado:* o texto
      chega inteiro, os emojis também;
    - `window_checks grab`. *Esperado:* a mão aberta sobre o form e a fechada com o botão
      apertado; diga se o desenho precisa de ajuste.
170. **Projeto, publicação e acesso tipado** (23/11). Num projeto Rust
    novo, numa pasta com git (a IDE recompilada):
    - em Projeto → Publicar, preencha "Descrição" e "Direitos autorais", marque "Versão da tag
      do git" e publique para Windows sem nenhuma tag. *Esperado:* a publicação para com a
      mensagem "o repositório não tem tag" e o que fazer. Crie a tag (`git tag v1.4.0`) e publique
      de novo. *Esperado:* em Propriedades → Detalhes do `.exe`, a descrição, os direitos e a
      versão 1.4.0; o instalador (NSIS ou Inno) com os mesmos direitos; num label, o
      `aylis::app_version()` mostra 1.4.0 no app publicado e no executado pela IDE;
    - desmarque a opção. *Esperado:* volta a versão do campo; com a descrição e os direitos
      vazios, o nome do app e "© fabricante", como antes;
    - publique para Android e Linux (`.deb`) com a opção marcada. *Esperado:* o `versionName` do
      APK e a versão do `.deb` são os da tag;
    - num handler de um botão do próprio form, chame `FrmX::get(ui)`. *Esperado:* `None` e, no
      Console da IDE (debug), uma linha que manda usar o `self` ou o `FrmX::components(ui)`, uma
      vez só;
    - com `use aylis::prelude::*;` e `use crate::generated::*;` no mesmo arquivo, um
      `ColumnSpec { field: "nome".into(), ..Default::default() }`. *Esperado:* compila (o tipado);
      o mesmo com um `TreeNode`, um `MapMarker` e um `ListViewItem` do acesso tipado num form
      com `TreeView`, `MapView` e `ListView`: compila sem o caminho. Num app que montava a
      `TreeView` pela struct do controle com `use aylis::prelude::*`, o `TreeNode` pede o caminho
      `aylis::controls::TreeNode` (diga se algum outro nome que o app usava sumiu do `prelude`);
    - um form com um `TabControl` (com `TabItem`) e um `TabStrip`, e "Gerar código" em Rust e
      em C#. *Esperado:* compila nas duas; as abas do `TabStrip` pelo acesso tipado são
      `TabStripItem { title: "Início".into(), ..Default::default() }`;
    - com `aylis = { ..., features = ["serde", "chrono"] }` no `Cargo.toml`, uma struct com
      `#[derive(serde::Deserialize)]` e um campo `Date`, lida de `{"criada": "2026-11-23"}`, e
      `let d: Date = chrono::NaiveDate::from_ymd_opt(2026, 11, 23).unwrap().into();`.
      *Esperado:* compila e dá 23/11/2026 nos dois.
171. **O `DataGrid`, os dados e os gráficos** (23/11). Num app de exemplo (a IDE recompilada):
    - abra a IDE e um form com um `DataGrid` e um `Chart`, sem tema próprio. *Esperado:* a grade
      e o gráfico iguais aos de antes (a zebra, a seleção, o cabeçalho, a moldura da célula, o
      espaço de dentro do gráfico);
    - num tema do projeto, `"DataGrid.row": { "focused": { "fill": "#DBEAFE", "text": "#1E3A8A" } }`,
      `"DataGrid.stripe": { "fill": "transparent" }`, `"DataGrid.header": { "fill": "surface" }`
      e `"DataGrid.cursor": { "border_width": 1 }`; e a grade com `frame: false` dentro de um
      `Panel` com borda. *Esperado:* a linha atual azul com o texto escuro, sem zebra, o cabeçalho
      na cor do fundo, a moldura da célula fina, e uma moldura só (a do `Panel`); diga se falta
      alguma parte que as telas web tinham;
    - `"ScrollBar": { "width": 6, "thumb": "primary" }` no tema. *Esperado:* as duas barras da
      grade finas e na cor de destaque, e a grade usa o espaço que sobrou;
    - uma grade `read_only`: Enter numa célula e selecione um pedaço do texto com o mouse,
      Ctrl+C. *Esperado:* o pedaço copiado, e o dataset continua em Browse; digitar uma letra na
      grade não abre nada;
    - uma `Column` com `"title": null` e `hint`, a grade com `empty_text` e `sortable: false` e um
      `on_title_click`, sem registros. *Esperado:* o cabeçalho da coluna vazio, a dica ao parar o
      mouse no título, o texto pedido no lugar de "Sem registros", e o evento no clique do título
      sem ordenar;
    - no `on_pointer_move` da grade, `grade.row_at(ui, e.position.y)` e `value_at` posto na dica
      da grade. *Esperado:* o valor da linha sob o mouse, sem o registro atual mudar (também pelo
      Python, `invoke`);
    - um `ClientDataSet` sem `Field`, ligado a uma grade, com `load_json` de uma lista de objetos
      (números, datas `2026-11-23`, `true`/`false`, texto) e depois outra consulta com outras
      colunas. *Esperado:* as colunas com o tipo certo (números à direita, a caixa de marcar
      nos lógicos), e trocadas na segunda carga;
    - dois `Chart` de linha com rótulos de horário (um com um horário a mais), `sync` apontando
      para um `ChartSync`, um com `select_range` e o outro com `zoom`. Passe o mouse num.
      *Esperado:* a linha tracejada e a dica no mesmo horário nos dois; arraste uma faixa no
      primeiro (e a roda no segundo). *Esperado:* os dois mostram a mesma janela; duplo clique
      volta ao todo; o `on_sync` chega com o tipo, os valores e o nome do gráfico. Um terceiro
      com `time_axis` no mesmo `ChartSync`: ignora o que veio dos outros (no Console, uma linha
      em debug);
    - a linha de filtro (`filter_row`) de uma grade com cidades acentuadas: digite `sao`.
      *Esperado:* aparece "São Paulo";
    - `highlight_index: 2` num `Chart` pelo código, e `border_width: 0` com `padding: 0`.
      *Esperado:* a coluna 2 destacada com a dica; sem moldura, e o gráfico até a borda.
172. **Os campos, o texto e o desenho** (23/11). Na
    IDE recompilada e num app de exemplo (o `giant_screen` em release já passou com a máquina
    parada; os números nos pendentes):
    - abra forms com `Edit`, `ComboBox`, `Memo`, `ListBox`, `VirtualList` e `ScrollBox`, sem
      tema próprio.
      *Esperado:* tudo igual ao de antes (a moldura, o rótulo, o ícone e as barras);
    - um `ComboBox` com `label: "Tabela"`, `style: outlined` e `clear_button`, ao lado de um
      `Edit` com o mesmo `label` e `style`. *Esperado:* sem opção, o rótulo dentro do campo; ao
      escolher (ou com foco), ele sobe para a borda como no `Edit`, e o "×" antes da seta limpa a
      escolha sem abrir a lista (o `on_change` chega vazio);
    - no tema do projeto, `"Edit": { "icon_size": 20, "icon_color": "primary", "icon_gap": 10,
      "focused": { "icon_color": "danger" } }` e um `Edit` com `icon: search`. *Esperado:* o ícone
      maior, na cor de destaque e mais longe do texto; com o foco, na cor de perigo;
    - um `CodeEditor` com `syntax: sql` e um `SELECT ... FROM ... WHERE` com comentário `--`.
      *Esperado:* palavras-chave, textos, números e comentários coloridos, em maiúsculas ou não.
      Depois `keyword_color: #C00000` e `syntax_tokens: [{"name": "variavel", "prefix": "@",
      "color": "warning"}]` com um `@total` no texto. *Esperado:* as palavras-chave em vermelho
      escuro e o `@total` na cor de aviso, menos dentro de um texto `'...'`;
    - um `Rectangle` com `stroke_width: 2` e `stroke_dash: "6;4"`. *Esperado:* a borda
      tracejada, igual no Direct2D e no Skia (Opções → renderizador) e no navegador;
    - um `Label` com `markup`, `wrap` e `<tag=warning/20,warning>um termo</tag>` no meio de um
      texto longo, estreitando o rótulo. *Esperado:* o termo numa caixinha com fundo claro e
      borda, que desce inteira para a linha de baixo quando não cabe (nunca partida);
    - no tema, `"ScrollBar": { "width": 4, "thumb": "primary", "hover": { "thumb": "danger" } }`.
      *Esperado:* as barras do `ScrollBox`, do `Memo`, do `ListBox`, do `TreeView`, do
      `VirtualList` (com `scroll_bar: always`), da lista aberta do `ComboBox` (e da grade) finas
      e na cor de destaque, vermelhas com o mouse em cima; a calha do `ScrollBox` mais estreita,
      e as linhas do `VirtualList` mais largas (ocupam o que a barra deixou). Depois `"ScrollBox.fina": { "scroll_bar":
      { "width": 2 } }` e um `ScrollBox` com `style: fina`. *Esperado:* só ele com a barra de 2;
    - um `PaintCanvas` com `tool: arrow` (o exemplo Paint ainda não tem os botões dessas duas):
      arraste. *Esperado:* a prévia e a
      seta com a ponta cheia, na espessura do pincel; Ctrl+Z desfaz. Ferramenta `text`: clique,
      escreva "Câmera 3" (Shift+Enter quebra a linha) e Enter. *Esperado:* o texto na fonte do
      controle e na cor 1, que vira pixel (gravar o PNG leva o texto; não se edita depois); Esc
      desiste. No navegador (a IDE lite), o mesmo (conferido no item 176).

173. **Os diálogos, as abas, a paleta e os avisos** (23/11). Num
    app de exemplo (o `Abas` serve para as abas) e na galeria da IDE:
    - `prompt(ui, "Novo perfil", "Nome do perfil:", "Padrão", |ui, nome| ...)` num botão.
      *Esperado:* a caixa do `message_dlg` com o campo embaixo do rótulo, o texto "Padrão"
      selecionável e o cursor nele; Enter confirma (chega `Some("...")`), Esc e Cancelar dão
      `None`. Com `set_prompt_required(ui, caixa, true)` e o campo vazio (ou só espaços), o OK
      fica desligado e o Enter não fecha;
    - `let c = confirm(...); set_dialog_icon(ui, c, "delete", Some(ColorRef::Token("danger".into())))`
      e `set_dialog_icon(ui, show_message(ui, "Pronto."), "", Some(Color::rgb(0x6A, 0x1B, 0x9A).into()))`.
      *Esperado:* o círculo vermelho com a lixeira na cor sobre o perigo; no segundo, o "i" de
      sempre num círculo roxo, legível. Sem a chamada, tudo como antes. O mesmo pelo Object
      Inspector, num `MessageDialog` com `icon`, `icon_color`, `input` e `required`;
    - num form com duas abas (`TabStrip` com páginas), `ui.show_modal_in(frm_pergunta,
      página_da_aba_1, |ui, r| ...)`. *Esperado:* o form centrado sobre a página, com o fundo
      escurecido só nela; clicar na página atrás não faz nada, o Tab fica no form e o Esc fecha
      como `cancel`. Enquanto isso, a aba 2, a barra do form e as outras janelas respondem.
      Arranque a aba 1 para outra janela com a pergunta aberta: *Esperado:* ela vai junto; feche
      essa janela: *Esperado:* a função recebe `cancel`;
    - uma `CommandPalette` com `items` com `id`, `category` e `description` e `Ctrl+K` chamando
      `CommandPalette::open`. *Esperado:* abre por cima com a busca limpa e o foco; os comandos
      agrupados pela categoria, a descrição embaixo do nome; "configuracoes" acha "Configurações";
      Enter escolhe (o `on_select` com o `id` e o nome), a paleta some, e ao abrir de novo o
      escolhido está em "Recentes", em cima. Com um `on_query_change` que escreve no console,
      digitar rápido "clientes" dá um aviso só, ~200 ms depois de parar;
    - num form com `TitleBar` (`window_settings`/barra própria), `notify(ui, "Salvo", "...",
      ToastKind::Success, 4000)`. *Esperado:* o aviso abaixo da barra, sem cobrir os botões
      dela. Depois um `ToastStack` posto pelo designer no canto de baixo: *Esperado:* os avisos
      nele. Na galeria, o `ToastStack` aparece com dois avisos;
    - no tema do projeto, `"TabStrip.tab": { "fill": "surface_2", "text": "text_muted",
      "hover": { "fill": "surface" }, "checked": { "fill": "primary/20", "text": "primary" } }`
      e uma `TabStrip` com `fill` próprio. *Esperado:* a aba ativa com o fundo e o texto do
      `checked` (não some mais no `fill` da faixa), as outras com os seus; sem o estilo, como
      antes. Igual no Direct2D e no Skia (Opções → renderizador) e no navegador;
    - `hover_preview: true` numa `TabStrip` com páginas: passe por todas as abas e pare o mouse
      sobre uma que não é a ativa. *Esperado:* depois de meio segundo, o título e a miniatura da
      página embaixo da aba (a de quando ela foi vista por último); some ao sair da aba ou
      clicar. Uma aba ainda não mostrada não tem prévia;
    - `hit_tabs_only: true` numa `TabStrip` por cima de uma `TitleBar` (o `Abas`). *Esperado:*
      arrastar pela parte vazia da faixa move a janela; as abas continuam clicáveis e
      arrastáveis. Com `drop_types: "aylis-tab"` na barra e o `on_drop` dela chamando
      `TabStrip::receive_drag(ui, faixa, &dados, None)`, a aba de outra janela solta na parte vazia
      entra no fim;
    - um `EmptyState` novo pelo designer. *Esperado:* sem o botão "Limpar filtros".

174. **Janelas e sistema** (23/11). O exemplo novo `desktop_checks` tem um cenário por item (`cargo run
    -p aylis-platform-winit --example desktop_checks -- <cenário>`, a lista no começo do
    arquivo). O que não deu para conferir aqui (a máquina estava em uso):
    - `desktop_checks keys`: aperte uma letra na janela de cima, Caps Lock, outra letra, e clique
      na de baixo e aperte outra. *Esperado:* `caps true` depois do Caps Lock, `num` conforme o
      Num Lock, e o `form` e o `ativo` da janela em que a tecla chegou;
    - `desktop_checks tray` (com `--features tray`): clique direito no ícone duas vezes, e espere
      os 3 s. *Esperado:* o primeiro clique escreve `on_context` e não abre o menu; o segundo
      abre; aos 3 s o menu abre sozinho junto do mouse;
    - `desktop_checks transparency`, e desligue e ligue os "Efeitos de transparência"
      (Configurações → Personalização → Cores). *Esperado:* `on_transparency_change` com
      `false` e depois `true` (em até 1 s);
    - `desktop_checks single`, e com ele aberto, `desktop_checks single a b` noutro terminal.
      *Esperado:* a segunda escreve que sai, e a primeira vem para a frente e escreve
      `on_second_instance` com `a` e `b`;
    - com os dois monitores ligados, `desktop_checks monitors`, troque o cabo de um monitor de
      porta e rode de novo. *Esperado:* o `device` (`\\.\DISPLAYn`) pode mudar; o `device_path`
      de cada aparelho não muda;
    - `desktop_checks border` e `desktop_checks border none full custom` (no monitor com a cor
      de destaque nas bordas ligada, em Configurações → Personalização → Cores). *Esperado:*
      nenhuma linha colorida em volta da tela cheia;
    - `desktop_checks topmost`, e o Gerenciador de Tarefas com "Sempre visível" por cima dela.
      *Esperado:* `on_topmost_lost`, e 1 s depois ela volta para cima, sem tirar o foco do
      Gerenciador.
175. **A rede e o `McpServer`** (23/11; os guias
    [rede.md](rede.md) e [mcp.md](mcp.md)). Os testes com servidores locais passam; falta o que
    só dá para ver com um cliente de verdade:
    - `cargo run -p aylis-providers --features http,websocket --example network`. *Esperado:* o
      `GET` de `https://example.com/` com 200 e o eco do WebSocket (`wss://ws.postman-echo.com/raw`)
      devolvendo "olá" (conferido aqui em 23/11). Com outro endereço:
      `-- https://httpbin.org/status/404 wss://...`: o 404 chega como resposta, não como erro;
    - o mesmo `HttpClient` num app publicado para a web (`aylis-tool publish ... --target web`):
      o pedido sai pelo `fetch` do navegador (a aba Rede do navegador mostra), e um `http://` de
      outro site é recusado antes de sair;
    - `cargo run -p aylis-remote --example mcp_app`, e no Claude Desktop a entrada que ele
      escreve (`claude_desktop_config.json`, o `mcp_app.exe --mcp-stdio estoque-exemplo`),
      reiniciando o Claude Desktop. *Esperado:* o servidor aparece com a versão 1.2.0; no chat,
      "qual o saldo do produto A3?" chama a ferramenta "Saldo de um produto" (sem pedir
      confirmação: só leitura), o terminal do exemplo escreve a chamada e o tempo; o recurso
      "Leia-me" e o prompt "Relatório de um produto" aparecem no menu de anexos e de prompts. Com
      o exemplo fechado, o Claude Desktop diz que o app não está aberto. Também confere que o
      Claude Desktop (um pacote MSIX) enxerga o arquivo da porta e do token em
      `%LOCALAPPDATA%\aylis\mcp-apps` gravado pelo app aberto fora dele;
    - a mesma linha do `claude mcp add` no Claude Code: `/mcp` mostra as ferramentas, os
      recursos e os prompts; desligar uma ferramenta em execução (`enabled`) atualiza a lista
      sem reconectar.
176. **O desenho: SVG, texto e camadas** (23/11). Num app de exemplo, no Direct2D (o padrão no Windows), no Skia (Opções →
    renderizador) e no navegador (publicado para a web), no Chrome e no Firefox:
    - um `Svg` com um SVG que tenha `<clipPath>` (um círculo recortando uma foto em vetor),
      `<mask>` (um degradê do branco ao preto apagando uma forma aos poucos) e `<pattern>` (um
      fundo de listras ou de pontos). *Esperado:* os três efeitos iguais nos três
      renderizadores e no Firefox/Chrome abertos lado a lado com o arquivo; redimensionar a
      janela redesenha nítido;
    - numa tela com uns 300 ícones SVG com máscara (um `Svg` repetido), role e passe o mouse.
      *Esperado:* sem travar; parado, sem gastar (a pintura vem da imagem guardada);
    - no projeto, Projeto → Fontes → Noto Emoji. Num `on_paint` próprio,
      `TextStyle { color_glyphs: false, .. }` com `⚠ ✅ 😀 ❤️ 👍🏽`. *Esperado:* todos na cor do
      texto, do mesmo tamanho e no mesmo lugar nos três; com `color_glyphs: true`, coloridos (o
      `⚠` sem U+FE0F continua na cor do texto). Sem a Noto Emoji no projeto: o Direct2D tira a
      cor, o navegador continua colorido (o esperado, documentado);
    - no mesmo `on_paint`, um título com `letter_spacing: 3` alinhado à esquerda, no meio e à
      direita, e um texto longo medido com `canvas.measure_text_wrapped(texto, &estilo, 200.0)`
      e desenhado com `wrap` num retângulo desse tamanho. *Esperado:* as letras afastadas, sem
      espaço sobrando depois da última (o centrado fica no meio de verdade); o texto quebrado
      cabe exato no retângulo, sem "…" e sem linha cortada, igual nos três;
    - a marca d'água: num `on_paint`, a imagem de uma câmera, `save_layer(0.9)`, dentro
      `save_layer_blend(1.0, BlendMode::Difference, ...)` com um texto branco, e os dois
      `restore`. *Esperado:* igual no Direct2D, no Skia e no navegador (a marca isolada na
      camada); com `save_layer_blend(0.9, Difference, ...)` sem a camada de fora, o texto
      inverte o vídeo por baixo nos três. Com uma grade de câmeras e a marca d'água em cada
      célula, o quadro no Direct2D mais leve que antes;
    - na IDE lite (no navegador), um `PaintCanvas` com `tool: text`: clique, escreva e Enter.
      *Esperado:* o texto vira pixel na imagem, como no desktop (Ctrl+Z desfaz); o `draw_text`
      pelo código devolve `true`.
177. **A instância única, a captura de monitor e a tecla nas outras linguagens** (23/11). Na IDE recompilada:
    - Projeto → Opções do projeto: "Instância única" com Quantas abrirem, Uma por usuário e Uma
      na máquina, e a caixa "Permitir capturar a imagem de um monitor (capture_monitor)".
      Escolha "Uma por usuário", marque a caixa e dê OK. *Esperado:* o `.aylisproj` com
      `"single_instance": "user"` e `"allow_screen_capture": true`; reabrindo a tela, as duas
      como ficaram; voltando a "Quantas abrirem" e desmarcando, as duas chaves somem do arquivo.
      Com a IDE em inglês, os rótulos em inglês;
    - num projeto em Python (ou outra linguagem que não Rust), "Gerar código" e um
      `on_key_down` que escreve `form`, `form_id`, `caps_lock` e `num_lock` do evento.
      *Esperado:* o `KeyEvent` gerado tem os campos, e a tecla mostra o nome do form em que
      chegou e o estado do Caps Lock.
178. **O Reveal com o raio do controle** (24/11). Num form, um `Rectangle` com `radius: 12`,
    `fill` e `style: reveal`, e no tema do projeto `"Rectangle.reveal": { "reveal": "border",
    "border_width": 2 }` (sem `radius`). Rode com Direct2D, depois com Skia (Opções do projeto →
    Aparência) e no navegador (publicar para web). Passe o mouse perto do cartão. *Esperado:* a
    borda acesa acompanha os cantos arredondados nos três (sem quina reta por cima); com
    `radius: [12, 12, 0, 0]`, os de baixo ficam retos também na luz. Um controle dentro do
    cartão continua aparecendo inteiro (o raio não recorta os filhos).
179. **As medidas do menu da bandeja pelo tema** (24/11). Um app com `TrayIcon` e
    `menu_items` (exemplo `tray`). Sem nada no tema, o menu igual ao de antes (220 de largura,
    itens de 32). Depois, no tema, `"PopupMenu": { "item_height": 26, "separator_height": 7,
    "width": 260, "font": "small" }`. *Esperado:* o menu da bandeja com 260 de largura, itens
    mais baixos e a fonte menor, e os cliques no item certo; o `PopupMenu` do form também com
    os itens de 26 e a divisória de 7. Tirando o `width` e pondo um item de texto bem longo, o
    menu da bandeja alarga até o texto caber.
180. **O painel sem ativar volta sem tirar o teclado** (24/11). Abra o Bloco de Notas e
    deixe-o com o cursor; num terminal, `cargo run -p aylis-platform-winit --example
    window_checks -- hide_show` e volte a clicar no Bloco de Notas antes de 1,5 s. Continue
    digitando por uns 8 s. *Esperado:* o form `topmost` com `no_activate` some (`Hide`), volta
    (`Show`), some de novo e volta noutro lugar (`Place` sem ativar), sempre por cima e sem
    nunca tirar o teclado do Bloco de Notas (nada do que se digita se perde, e o console não
    escreve `ativado`). O mesmo vale para um form só com `activate_on_show` desligado.
181. **O teclado volta ao form quando o `WebView` some** (24/11). `cargo run -p
    aylis-platform-winit --features webview --example webview_focus`; assim que a janela
    abrir, clique dentro do campo grande da página. *Esperado:* no console, `foco antes: uma
    janela filha (a vista)`; o WebView some, `foco depois: a janela do form`, e o que se digita
    em seguida aparece no `Edit` de cima (`texto: ...`), sem precisar de Alt+Tab. Com
    `-- --free` (o WebView liberado em vez de escondido), o mesmo.
182. **O texto em cinza simétrico e o ClearType do projeto** (24/11). Na IDE recompilada, um
    projeto com rótulos de letra pequena (11 a 13 px, com "w", "v", "x" e "/"), o modo de
    texto `native` e o fundo do form sólido:
    - rode com Direct2D. *Esperado:* em cinza, as diagonais suaves, sem degraus (compare com a
      0.10.0); a largura dos rótulos com `auto_size` não muda;
    - Projeto → Opções do projeto, marque "Texto em ClearType (Direct2D e Skia, sobre fundo
      opaco)" e dê OK. *Esperado:* o `.aylisproj` com `"text_antialias": "cleartype"`
      (desmarcando, a chave some). Rode de novo com Direct2D e com Skia: com a lupa do Windows,
      a borda das letras com franjas coloridas; um rótulo dentro de um painel com cantos
      arredondados também em ClearType; um controle com `opacity` abaixo de 1 em cinza;
    - publique para web. *Esperado:* no navegador, em cinza (a diferença aceita);
    - a própria IDE continua em cinza.
183. **A IDE aberta por um agente: só leitura e o F12 sem perder linha** (24/11). Numa conversa
    do Claude, peça para abrir um projeto pelo Aylis (a IDE abre pela ponte do MCP):
    - abra o código de um form (F12). *Esperado:* em cima do editor, a faixa "Aberto por um
      agente (MCP): só leitura..." com o botão **Liberar edição**; o que se digita não entra; o
      Ctrl+Z não desfaz nada;
    - peça ao agente uma troca no mesmo fonte (`edit_code`). *Esperado:* a troca aparece no
      editor, que continua travado para você;
    - clique em **Liberar edição**. *Esperado:* a faixa some e dá para digitar; o Ctrl+Z desfaz
      só o que você digitou, nunca a troca do agente; outro fonte aberto pelo agente continua
      travado (o botão libera a aba à vista);
    - peça ao agente que grave uma linha no fonte por fora (a ferramenta Edit dele, não o
      `edit_code`) e, logo em seguida, sem trocar de janela, aperte F12 no designer e depois
      Ctrl+S no código. *Esperado:* a linha do agente continua no editor e no disco;
    - com alteração sua não salva no fonte, peça ao agente outra linha por fora e aperte Ctrl+S.
      *Esperado:* não grava por cima: a IDE pergunta se recarrega a versão do disco;
    - Opções da IDE → MCP, desmarque "Abrir os fontes só leitura quando aberta por um agente" e
      abra outro fonte. *Esperado:* ele abre livre, sem a faixa. Com a IDE aberta por você (sem
      a conversa), nenhum fonte abre travado.
184. **A versão pela série de tags e o `-dev`** (24/11). Num projeto num repositório git, crie
    as tags `Client-v1.0.0.28` e, num commit depois, `Server-v3.1.0`. Em Projeto → Publicar,
    marque "Versão da tag do git", ponha "Client-v" em "Prefixo das tags", marque "-dev fora da
    tag" e escreva um "Produto" diferente do nome do aplicativo; publique para Windows.
    *Esperado:* Propriedades → Detalhes do `.exe` com a versão `1.0.0.29-dev` (há um commit
    depois da tag do cliente), o produto escrito e o nome do arquivo pelo nome do aplicativo.
    Crie a tag `Client-v1.0.0.29` no commit atual, sem alteração pendente, e publique de novo:
    `1.0.0.29`, sem o `-dev`. Com o prefixo `Beta-v` (sem nenhuma tag assim), a publicação para
    dizendo que o repositório não tem tags `Beta-v*`.
185. **O runtime do Visual C++ dentro do `.exe`** (24/11). Num projeto com Direct2D no Windows
    e um `WebView`, Projeto → Opções → Windows, marque "Runtime do Visual C++ dentro do
    executável" e publique para Windows. *Esperado:* a pasta publicada só com o `.exe` (sem
    `vcruntime140.dll`, `vcruntime140_1.dll` nem `msvcp140.dll`), o
    `<projeto>/rust/.cargo/config.toml` com o `+crt-static`, e o `.exe` abrindo numa VM com o
    Windows limpo (sem o Visual Studio nem o redistribuível). Com o Skia no Windows e sem o
    LLVM instalado, a publicação para antes de compilar e explica por quê.
186. **O WebView2 de versão fixa** (24/11; falta o pacote, veja os pendentes). Extraia o `.cab`
    da versão fixa (x64) com `expand <arquivo.cab> -F:* <pasta>` e copie a pasta de dentro
    para `WebView2`, ao lado do `.exe` publicado de um app com `WebView`. *Esperado:* o app abre
    a página, e o Gerenciador de Tarefas mostra o `msedgewebview2.exe` rodando dessa pasta (não
    do `Program Files (x86)\Microsoft\EdgeWebView`). Sem a pasta, o do Windows.
187. **O `WebView` só com `html`** (24/11). Num form, um `WebView` com a propriedade `html`
    (`<h1>Olá</h1>`) e o `url` apagado no Inspetor (ou nunca preenchido). Execute.
    *Esperado:* a página aparece; antes ficava vazia, e o console dizia `WebView2 error ...
    0x80070057`.
187. **A instância única configurável** (24/11). Na IDE recompilada, Projeto → Opções do projeto
    → Geral: *Esperado:* abaixo de "Instância única", "Segunda abertura" (entregar os argumentos
    ou sair calada) e "Nome da trava". Num projeto Rust com `user`, "Sair calada" e o nome
    `Global\MeuTeste`, abra o app publicado duas vezes: a segunda some sem trazer a primeira e
    sem `on_second_instance`. Com um handler que chama `ui.release_single_instance()` (num botão)
    e depois abre o próprio exe: o novo abre como primeiro. O mesmo num projeto Python (o
    `release_single_instance` do `ApplicationEvents`). No terminal, `cargo run -p
    aylis-platform-winit --example desktop_checks -- quiet` e, antes de 5 s, de novo: a segunda
    escreve "sai calada"; depois de 5 s, a próxima fica aberta como primeira.
188. **A sessão do Windows bloqueada e conectada** (24/11). `cargo run -p aylis-platform-winit
    --example desktop_checks -- session`; bloqueie com Win+L e volte. *Esperado:*
    `on_session_change: bloqueada true, conectada true` ao bloquear e `bloqueada false` ao
    voltar. Por RDP (ou na VM do Windows 11): feche a janela do RDP sem sair e reconecte; o
    console mostra `conectada false` e depois `true`.
189. **Iniciar com o sistema** (24/11). `cargo run -p aylis-platform-winit --example
    desktop_checks -- login on`. *Esperado:* `start_with_login: true`, e o Gerenciador de
    Tarefas → Aplicativos de inicialização mostra o `desktop_checks`; saia da conta e entre: o
    exemplo abre sozinho. `-- login off`: some da lista e não abre mais. Num app com um
    `CheckBox` "Abrir com o Windows" ligado ao `set_start_with_login`, o mesmo.
190. **O Markdown formatado no Design** (pedido de 24/11). Na IDE, abra um `.md` do projeto (o
    `AGENTS.md`, um README: no painel do projeto, grupo "Documentos", nas pastas deles) e clique em Design, ou F12. *Esperado:* o
    texto formatado (títulos, listas, tabelas, código colorido, as imagens com caminho relativo
    à pasta do arquivo) no lugar do editor, com o Design em destaque; Código (ou F12) volta ao
    texto. Digite algo sem salvar e volte ao Design: já aparece. Um link para outro `.md` abre ele
    numa aba, formatado (com `#título`, rola até ele); um `https://` abre no navegador. Cada
    arquivo lembra a visão escolhida ao trocar de aba.
191. **A tecla física no `KeyEvent`** (pedido de 24/11). `cargo run -p aylis-platform-winit
    --example desktop_checks -- keys`, com o teclado ABNT2. Aperte `/` (a tecla ao lado do Shift
    direito), `ç`, `´`, `~` e `\` (a tecla ao lado do Shift esquerdo). *Esperado:* `física IntlRo`
    (scancode `0x73`), `Semicolon` (`0x27`), `BracketLeft` (`0x1A`), `Quote` (`0x28`) e
    `IntlBackslash` (`0x56`), com o caractere certo no `Char`. O Ctrl direito mostra
    `ControlRight` (`0xE01D`); o `1` da fila de cima, `Digit1`, e o do teclado numérico, `Numpad1`.
    Troque o layout do Windows para US: a física e o scancode não mudam, só o `Char`.
192. **O `on_key_down` tipado** (pedido de 24/11). Na IDE recompilada, num projeto em Rust, ligue
    o `on_key_down` do form a um handler novo e gere o código. *Esperado:* o esqueleto é
    `fn h(&mut self, ui: &mut Ui, sender: Form, value: KeyEvent) -> Result<(), String>`; com
    `ui.suppress_key()` dentro, a tecla não chega ao `Edit` em foco (com `key_preview` ligado).
    Faça o mesmo num projeto em Python: `def h(aylis, sender: "Form", value: "KeyEvent")`, e
    `aylis.reply("key", "")` engole a tecla. Rode a Calculadora (Rust e outra linguagem) e use o
    teclado: os números e as operações apertam os botões. Pelo MCP, `set_property` de um
    evento com `"\"btn_click\""` liga o handler `btn_click` (antes, nome inválido).
194. **A escala, o fim do redimensionamento, o trazer para a frente e o mouse sobre a janela**
    (pedido de 24/11). `cargo run -p aylis-platform-winit --example window_reports`. *Esperado:*
    a escala do monitor na primeira linha (1,5 com 150%); arraste a borda por um tempo e solte:
    um `on_resize_end` só, ao soltar; maximize pelo botão: outro, logo depois; leve a janela a
    um monitor de outra escala: `on_scale_change` com a escala nova. A cada segundo, o mouse:
    sobre a janela, `form sob ele "frmReports"` e `pointer_over true`; fora, ou com outra
    janela por cima dela, `None` e `false`. Com `-- auto`: a largura muda e sai o
    `on_resize_end` 300 ms depois; a janela minimiza e volta pelo `bring_to_front` (com o foco,
    mesmo com outro programa clicado no meio). Num app com `FrameView` e escala 125% ou 150%,
    um quadro do tamanho da tela em pixels sai nítido, sem borrão nas letras.
193. **O teclado preso, o cursor de imagem e o Scroll Lock** (pedido de 24/11). `cargo run -p
    aylis-platform-winit --example desktop_checks -- grab`. *Esperado:* `available: Ok(Bool(true))`
    e, com a janela em foco, `on_capture_change: Value(Bool(true))`. Aperte a tecla Windows: o
    menu Iniciar não abre, e o console mostra `on_key_down Meta (física MetaLeft)` e o
    `on_key_up`. Alt+Tab, Ctrl+Esc, Alt+Espaço e PrintScreen: nada do sistema acontece, e as
    teclas aparecem no console; Alt+F4 não fecha (aparece `F4` com `alt true`). Clique noutra
    janela: `on_capture_change: Value(Bool(false))`, e a tecla Windows volta a abrir o Iniciar.
    O mouse sobre a janela vira a mira branca com contorno, com o clique no meio. Ligue o
    Scroll Lock: a próxima tecla mostra `scroll true`, e a linha `lock_keys` seguinte também
    (sem apertar nada). Ctrl+Alt+Del continua do sistema.
195. **Vídeos com marca d'água e o compositor da mistura** (pedidos de 24/11). `cargo run -p
    marca_dagua`. *Esperado:* o vídeo de exemplo tocando com "CÂMERA 01" repetido, inclinado,
    cada letra na cor oposta à do vídeo por baixo (amarelo → azul, preto → branco). Escolha 16,
    64 e 100 na barra: cada célula com o próprio número e o próprio vídeo; "Vídeo" desmarcado
    troca por uma imagem fixa, "Mover" faz a marca deslizar cobrindo a célula inteira, "Inverter"
    desmarcado deixa a marca branca. Abra outro vídeo: vai para todas. Depois, com
    `"blend_compositor": "direct3d"` no `examples/MarcaDagua/MarcaDagua.aylisproj` (ou a
    propriedade `blend_compositor` do form em `direct3d`), o mesmo desenho, sem diferença visível;
    para medir os dois: `AYLIS_BLEND_COMPOSITOR=direct2d` ou `direct3d` com `AYLIS_BENCH=100` e
    `--grid 256 --still --move`.
196. **O tamanho pelo código e o monitor no Wayland** (pedido de 24/11). No Windows, `cargo run
    -p aylis-platform-winit --example window_reports -- size`. *Esperado:* aos 1 s a janela
    cresce para 900 × 560 (a linha "1 s depois" com o form e a janela iguais) e aos 2,5 s
    estreita para 520, com um `on_resize_end` a cada mudança; o `form_monitor` com número desde
    a primeira linha. No Linux, o mesmo no GNOME pelo Wayland (veja testes-em-dispositivos). Num
    app com `KeyboardCapture` rodado sem instalar, no Linux pelo Wayland, o stderr avisa uma vez
    que falta o `<app>.desktop`.
197. **A parede de vídeos no Direct2D: o `Wait`, a cópia parcial e a `exclusion`** (pedido de
    25/11). `cargo run --release -p marca_dagua -- --grid 16`. *Esperado:* os 16 vídeos tocando
    com a marca em negativo, agora pela `exclusion` (o mesmo pixel da `difference`), sem lixo nem
    pedaço velho em nenhuma célula, também ao redimensionar, maximizar, minimizar e voltar, e com
    a janela meio coberta por outra; o menu e os cliques respondem. Compare com `--blend
    difference` (a mesma imagem) e `--blend normal`. Depois, com `"present": "composition"` e
    `"present": "composed"` no form (ou `AYLIS_PRESENT`), o mesmo. Uma janela que rola (o
    `examples/GradeGigante`, a rolagem da grade): nenhum rastro ao rolar rápido. Na máquina com a
    Intel integrada (testes-em-dispositivos), `gpu` do form em `Intel`: o 3D da Intel e a pintura
    pelo `profile` da IDE (o resumo traz `blend_layers`).
198. **O ritmo das pinturas (`max_paint_rate` do form)** (decisão D1 de 25/11). `cargo run
    --release -p marca_dagua -- --grid 16 --paint-rate 15`. *Esperado:* os vídeos andam, mais
    "travados" (15 pinturas por segundo), e o contador de FPS da barra perto de 15; passar o mouse
    nos botões, clicar em "Pausar" e digitar no texto da marca respondem na hora, sem esperar a
    vez. Com `--paint-rate 0` (ou sem a opção), como antes. No Object Inspector da IDE, o `Form`
    mostra "Pinturas por segundo (sem entrada)" em Comportamento.
199. **A bandeja e o sempre por cima com o shell** (pedidos de 25/11). No Windows:
    - `cargo run -p aylis-platform-winit --example desktop_checks --features tray,cpu --
      tray_id 1`. *Esperado:* dois ícones no "^"; aos 2 s as dicas terminam em "trocado" e os
      indicadores aparecem; aos 4 s sai o do GUID e aos 6 s o do número. Arraste o do número
      para a barra, feche e abra de novo: ele volta na barra (o lugar lembrado pelo `uID` 1);
    - `... desktop_checks topmost`, arraste o form para perto da bandeja e abra o "^", o Iniciar,
      a central de notificações (Win+N), a pesquisa e o Alt+Tab por cima dele. *Esperado:* cada
      um escreve `on_topmost_lost` com `system true` e o form fica por baixo (não volta ao topo);
      com o `desktop_checks cover` (ou o Gerenciador de Tarefas com "Sempre visível"), `system
      false`, o nome do exe, e o form volta ao topo 1 s depois.

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
  o `on_change` disparado pelo código; o modelo Web e o Desktop (o Smartphone passou).
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
