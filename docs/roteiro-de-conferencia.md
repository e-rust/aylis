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
    remova; o Corrigir do JDK ou do NDK numa máquina sem eles, com e sem origem (sem origem, a
    mensagem leva às Opções). Suba a pasta no Google Drive, monte o `pacotes.json` com os ids
    (`"drive"`) e use-o como espelho.

122. **Instalador com os pacotes** (12/11). Gere o kit com `aylis-tool installer-ide
    --with-packages node,codelldb` e, numa máquina de teste, rode o setup pela tela: a página dos
    pacotes, a do espelho, a instalação deles no fim; marque um pacote que não está no kit e veja
    o aviso. Desinstale e confira que os pacotes, o cache e as compilações saíram rápido.

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

129. **O instalador da IDE agrupado** (13/11; detalhes nos pendentes). Rode o
    `aylis-ide-<versão>-setup.exe`: uma página de componentes só, com "Aylis IDE e Rust
    (obrigatório)" cinza e marcado, "Outras linguagens" e "Plataformas de destino" abertos e
    desmarcados, e o atalho. Marque Python e Android e instale: nos detalhes, "Configurando
    Rust…", "Configurando Python…" e "Configurando Android…", com os acentos certos. Na IDE,
    Ferramentas → Configurar linguagens mostra Rust e Python prontos, e a verificação de
    ambiente do Android não tem falta (só o aparelho). Em Ferramentas → Opções → Pacotes, o
    espelho é o link da sua pasta do Drive. Na pasta instalada
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
    instalador e rode-o (marcando, por exemplo, Python) em três situações: sem internet e sem a
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
      um `TabControl` com `swipe`: o dedo troca de página; um `ImageView` com `zoomable`: a pinça
      aproxima e o dedo arrasta; uma `VirtualList` com `swipe_actions` "Excluir": deslizar o item
      mostra o botão, e o toque chega no `on_swipe_action`;
    - uma lista rolada até o fim: no Android 12 ou mais o conteúdo estica e volta; num Android 11, o
      brilho na cor do tema; `overscroll` `none` desliga.
141. **Testes do projeto em Java, C++ e Pascal** (14/11). Na Calculadora, troque a linguagem para
    Java, C++ e Pascal e rode Projeto → Testes (ou o `test` do MCP): os testes de `tests/` rodam e o
    resultado aparece por teste; quebre um de propósito e ele aparece como falha, com a linha.
142. **Novo DataModule com pasta e o tema chegando ao app** (14/11). Arquivo → Novo DataModule:
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
    e `wix` (da pasta de pacotes ou do espelho, depois de subidos). Um projeto em Pascal compila sem
    o Lazarus; o publicar com instalador `.msi` usa o WiX do pacote.
148. **O editor com um fonte enorme** (14/11; detalhes nos pendentes, item 26). `cargo run --release
    -p aylis-controls --example large_text 1000000`: digitar, apagar, Ctrl+Z e colar sem travar; a
    cor da sintaxe acompanhando ao rolar.
149. **A documentação do site sem comparações com outros frameworks** (14/11). Na Landing publicada,
    as páginas dos componentes trazem o "O que faz" de toda propriedade, e nenhuma compara o Aylis com
    outro framework.

105. **A Landing no celular** (11/11; detalhes nos pendentes, item 72). Publique de novo a página e
    os demos (`aylis-tool publish ... --target web`, `ide-web`, `demos`) e abra no celular: a
    rolagem acompanha o dedo sem engasgar; o 3D do topo não aparece no celular (escondido pela
    vista) e as ferramentas do navegador não mostram o `demos/3d` baixado; no PC o lingote aparece.
    Na documentação, a página do `Model3D` num navegador sem WebGPU (no Chrome, `chrome://flags`
    → WebGPU desligado, ou um iPhone antes do iOS 26) mostra o modelo girando.
    Na documentação, a página de um componente leve (Button, Valve, KpiCard...) mostra o
    componente ao vivo sem baixar nada além da página (aba Rede do navegador); a de um
    componente pesado (Gauge, DataGrid, Model3D) baixa o wasm da família dele (o Additional
    ~3,1 MB comprimido, antes ~4,5 MB). A página inicial baixa ~2,6 MB comprimido (antes ~3,2 MB).
106. **Uma pasta de compilação para todo o Rust** (11/11; detalhes nos pendentes). Abra a IDE
    recompilada e veja `%USERPROFILE%\.cargo\config.toml`: `[build] target-dir` com a pasta das
    Opções e `[profile.dev.package."*"] debug = false`, cada um com o comentário "Gravado pelo
    Aylis". Num projeto Rust qualquer, fora do Aylis, `cargo build` no terminal compila nessa
    pasta, e não num `target/` do projeto. Num projeto do Aylis já compilado pela IDE, `cargo
    build` no terminal termina em segundos, sem recompilar as dependências. Em Ferramentas →
    Opções → Rust e Python, desmarque "Usar esta pasta em todo o Rust desta máquina" e dê OK: as
    duas chaves saem (o arquivo some, se só tinha elas); marque de novo e elas voltam.

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
