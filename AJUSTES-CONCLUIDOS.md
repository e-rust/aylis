# aylis — Ajustes concluídos

O que foi feito depois do [plano inicial](PLANO.md), já testado pelo autor e commitado. Os
que ainda faltam estão nos [ajustes pendentes](AJUSTES-PENDENTES.md). O porquê de cada
pedido, com as palavras do autor, está no [histórico de decisões](docs/historico-decisoes.md).

## Pendências de 25/10/2026 (lista oficial)

Combinado em 25/10/2026: fechar tudo isto antes de melhorias novas, nesta ordem. As linhas
"Ainda não" das fases do [plano](PLANO.md) são histórico (várias foram feitas depois); esta
lista foi conferida no código e vale no lugar delas.

**Desenho e componentes**
- [x] Giro (`rotation`): o clique respeitar a rotação.
- [x] Direct2D: sombra com desfoque de verdade e gradiente cônico.
- [x] ListBox com várias seleções.
- [x] Calendário: "hoje" no fuso local (hoje é UTC).
- [x] Proporção fixa (o `VResponsive` do Vuetify): `aspect_ratio` em todo controle.
- [x] Importar SVG com `<circle>` e `<rect>` (Projeto → Ícones): agora pelo leitor de SVG do aylis.
- [x] Vidro (Liquid Glass) no navegador: conferido (`examples/web` em `?vidro`, igual ao Skia); sem `filter` no canvas (Safari), desfoque por redução.

**3D**
- [x] Evento de fim de clipe no `Model3D` (`on_clip_end`; sem `looping`, para de pedir quadros no fim).
- [x] Texturas de normal, oclusão e metal/aspereza; luzes e câmeras do arquivo; sombras (`docs/3d.md`).

**Dados e grid**
- [x] DBGrid: caixa de marcar em campos lógicos, várias linhas selecionadas, soma no rodapé,
  busca incremental.
- [x] Índices nomeados múltiplos, filtro com parâmetros, situação de alteração por registro.

**Remoto**
- [x] Eventos de Field por nome, em lote e assíncronos (`on_get_text` com cache por valor e
  `field_texts`; `on_validate` com `field_error`); teste automático do `aylis_post`
  (`crates/aylis-capi/tests/post.rs`, janela de verdade); `wss://` direto no `aylis-host`
  (`--connect wss://` com as raízes do sistema e `--tls-ca`; `--listen` com `--tls-cert` e
  `--tls-key`; rustls, sem OpenSSL).

**IDE**
- [x] Editor de fonte em diálogo (propriedade `font`): "…" no `font_family` abre família, tamanho, negrito e prévia.
- [x] Lembrar a posição das janelas e dos painéis (`Ui::window_placement`, que a plataforma mantém; a IDE guarda no `config.json`).
- [x] Editor de código com realce: `Memo.syntax` (rust, python, json), `line_numbers`, `want_tabs` e recuo automático; na IDE, F12 e o → dos eventos abrem a janela de código (Ctrl+S, no handler; "Abrir no VS Code" continua).
- [x] Atualizar o `Cargo.toml` de projetos antigos (`codegen::upgrade_cargo` no "Gerar código": caminho do aylis, `aylis-build`, feature `inspect`; mantém comentários e outras dependências).
- [x] Inspetor e hot reload no Android e na web: canal de desenvolvimento (o app conecta na IDE por WebSocket; `adb reverse` + `debug.aylis.dev` no Android, `?aylis_dev=` no navegador), testado no emulador e no navegador (`docs/carregamento.md`).
- [x] Pontos de parada dentro da IDE: protocolo DAP com o CodeLLDB (Rust) e o debugpy (Python); F5, F7, F8, Shift+F8, F9, Ctrl+F5 (`docs/depuracao.md`).
- [x] Android: reabrir o app com o processo ainda vivo falhava ("EventLoop can't be recreated"): o aylis reabre a Activity num processo novo (JNI `startActivity` + saída), testado no emulador com `am start --activity-clear-task` (`docs/carregamento.md`).

**Tradução**
- [x] Mensagens do próprio aylis (diálogos padrão) traduzidas: catálogo `aylis_core::messages` (pt original, en, es) nos diálogos, validação do Edit, erros dos campos e do dataset, grid, placeholders e mídia; o `locales/` do app vence; `LocaleInfo::es()`. Testes `aylis-controls/tests/idiomas.rs` e `aylis-data/tests/idiomas.rs`; doc em `docs/funcionamento.md` (Idiomas).

**Celular** (testes no emulador)
- [x] Android: área de transferência, IME completo (sugestões, correção, emoji) e "voltar": `dev.aylis.AylisActivity` (Java, compilada na publicação) + JNI; testado no emulador com o Gboard (composição, sugestão, emoji, senha, copiar/colar, voltar). `docs/carregamento.md`.
- [x] Pinça (multitoque): `on_gesture`/`GestureEvent` (escala, giro, arrasto) no núcleo, `Model3D` e `Graph` tratam; pinça e giro do touchpad no macOS viram o mesmo gesto. Teste `aylis-core/tests/pinca.rs` (o emulador não injeta dois dedos pelo adb).
- [x] Cor das barras do sistema seguindo o tema (Android, testado no emulador com tema claro e escuro); teclado do iOS reduzindo a área segura (`UIKeyboardWillChangeFrameNotification`; compila para iOS, o teste precisa de Mac).
- [x] Teclado não cobre os campos: o campo com foco aparece acima do teclado (`Ui::reveal` rola os ScrollBox e o form), o form rola com o dedo quando não cabe, e não desenha sob a barra de status. Pedido do usuário em 26/10; teste `aylis-core/tests/teclado.rs` e no emulador.
- [x] Ícone e nome do app configuráveis no script: `build-android.ps1 -Name -Icon -Id` (testado no emulador, com a AylisActivity) e `build-ios.sh` com `NAME`, `ICON`, `BUNDLE_ID` (precisa de Mac).

**Mídia, navegador embutido e acessibilidade**
- [x] Vídeo sem fábrica externa: Android (MediaCodec, testado no emulador), web (`<video>`, testado no navegador), Linux (GStreamer por dlopen, teste `aylis-media/tests/sistema.rs` num container Debian), Apple (AVFoundation; compila para iOS, teste precisa de Mac). Áudio do vídeo pelo symphonia.
- [x] WebView no Android (android.webkit.WebView numa janela-filha da AylisActivity; `window.ipc.postMessage`, eval, testado no emulador), Linux (wry/WebKitGTK com o laço do GTK; exemplo `navegador` no Docker com Xvfb: carregar, mensagem, eval, resposta) e iOS (wry `build`; precisa de Mac para compilar o Objective-C e testar).
- [x] Acessibilidade no Android (accesskit_android na DecorView, conferida com o `uiautomator dump`), web (ARIA sobre o canvas, conferida no navegador, ativar clica no controle) e iOS (accesskit_winit; precisa de Mac para testar).

**Efeitos**
- [x] Transição de página e de form: `Transition` (fade, slide_*) no núcleo (`Ui::transition`, `translate_x`/`translate_y`), `TabControl.transition`, `Form.transition` nos forms empilhados (celular/navegador; `show` entra, `close`/voltar sai), a tela de baixo aparece durante a transição. O `PageControl` do aylis são só os pontos (sem páginas): a troca de páginas é o TabControl. Testes `aylis-controls/tests/transicoes.rs`; visto no emulador.

**Componentes que faltam**
- [x] Seletor de emoji (`EmojiPicker`): 1.898 emojis do Unicode (até a 15.1) com nomes e busca em português (CLDR) e inglês, categorias, recentes, `on_select`. Dados em `resources/emoji/emoji.tsv` (`scripts/gerar_emoji.py`).
- [x] Mapa (`MapView`): blocos do OpenStreetMap (ou `tile_url`), arrastar, roda/pinça/duplo clique para zoom, marcadores, atribuição, `on_map_click`/`on_move`; blocos pelo novo `HttpService` (ureq/rustls nos nativos, `fetch` no navegador), cache na memória. Exemplo `mapa`.
- [x] Visualizador de PDF/Office (`DocViewer`, crate `aylis-docview`, feature `docview`): PDF pelo hayro (Rust puro; páginas rasterizadas numa thread, na escala da tela), XLSX/XLS/XLSB/ODS pelo calamine e CSV (grade com abas), DOCX/ODT (parágrafos, títulos, tabelas). Rolagem, Ctrl+roda/pinça para zoom. Testes `aylis-docview/tests/docs.rs`; exemplo `documentos`.

**Pedidos novos (depois das pendências acima, antes da publicação)**
- [x] Limitar a taxa de quadros (economizar bateria) e um método para ler o FPS atual (o
  app mostra onde quiser).
- [x] Ícone da bandeja (tray): animação ou indicador, menu com cor própria (claro, escuro ou
  outra).
- [x] Notificação do Windows (toast do sistema): `Notification` (não visual; `Notification::show`), WinRT `ToastNotificationManager`, registra o AppUserModelID do app em HKCU na primeira vez; fora do Windows vira o `toast` dentro do app. Exemplo `notificacao`.
- [x] Barra de título personalizada: botões próprios, ou uma barra pronta com minimizar,
  restaurar e fechar que dá para alterar.
- [x] Barra de tarefas: botões de controle na miniatura (ao passar o mouse), ações no botão
  direito (jump list) e indicador (contador/ícone sobreposto, como os apps de mensagem).
- [x] DBGrid: evento de pintura personalizada da célula e da linha (como o `on_get_text`),
  recebendo os dados da linha (objeto/JSON) e o índice.
- [x] MCP na IDE: Executar → Servidor MCP (HTTP em 127.0.0.1:7341/mcp, "streamable HTTP", Origin conferido). Ferramentas: project_info, open_form, list_components, get_component, set_property, add_component, delete_component, get_layout, set_layout, list_classes, save, generate_code, run, stop, output e app_command (o protocolo remoto no app em execução). As mudanças são desfazíveis na IDE. Teste HTTP de ponta a ponta e conferido com a IDE aberta. Decidido pelo usuário em 25/10.
- [x] Depurador da IDE: ponto de parada com condição (e contagem de passagens) e "rodar
  até o cursor" (F4).
- [x] Componente de câmera (`CameraView`: ao vivo, foto PNG, escolher a câmera) e de
  microfone (`Microphone`: nível, gravação WAV, escolher o dispositivo). Câmera no Windows
  (Media Foundation, testada com a câmera real), Linux (GStreamer) e navegador
  (`getUserMedia`) e Android (Camera2 do NDK, testada no emulador; a `AylisActivity` pede
  as permissões em fila e o manifesto só declara CAMERA/RECORD_AUDIO se os forms usam os
  componentes); microfone pelo cpal, testado no Windows. Fica para quando houver Mac: a
  câmera no macOS/iOS (AVCaptureSession). Melhoria: gravar vídeo da câmera.
- [x] 3D: a cena de demonstração agora é um cubo que gira e anda e uma bola que quica
  (`sample::scene_glb`; o tubo continua nos testes). Ray tracing e path tracing avaliados
  (`docs/3d.md`): `ray_tracing_support()` mostra ray query na RX 9070 XT (DX12) e sem
  pipelines de RT no `wgpu` 30.
- [x] Paleta da IDE: os componentes que dependem de plataforma mostram o ícone das plataformas onde funcionam, com a lista na dica (`aylis_build::class_platforms`; TrayIcon, Taskbar, TitleBar, WebView, CameraView, Microphone). Pedido em 26/10.
- [x] Mensagens da janela: `aylis::platform::win::on_message(form, |m| ...)` intercepta qualquer mensagem do Windows (subclass da janela) e devolve `Default`, `Handled` (impede) ou `Replace` (troca); constantes em `win::msg`. Em todos os sistemas, `on_close_query` cancela o fechar. Pedido em 26/10; exemplo `barra_tarefas`.
- [x] Notificações do Android: o mesmo `Notification` ganhou canal, prioridade, botões (`on_action`), toque (`on_click`), progresso, fixa, atualizar pelo número, `cancel`, e `form`: um form inteiro desenhado como a imagem grande (serviço de desenho fora da tela, `Ui::render_png`). Permissão POST_NOTIFICATIONS pedida em fila e posta no manifesto só se usada. Testado no emulador (imagem do form, botão chegando ao app). No Windows: imagem grande, progresso e botões com o retorno pelo `Activated` do toast. Pedido em 26/10.
- [x] DataModule: raiz `DataModule` só com componentes não visuais, criada no início antes dos forms (`on_create` ao carregar); o form declara `"uses"` e fala com os componentes pelo caminho (`dmDados.cdsClientes`). IDE: Projeto → Novo DataModule / DataModules usados, designer com a bandeja ocupando tudo, Object Inspector lista os componentes dos DataModules usados, painel do projeto mostra "usa" e "usado por". Os forms continuam aceitando não visuais. Sem herança de forms nem de DataModules (pedido). Pedido em 26/10.
- [x] Variantes de form por plataforma: `forms/principal.<variante>.json` (windows, macos, linux, android, ios, desktop, mobile, web), escolhida na execução (a do sistema vence a do grupo, depois o padrão), mesmos handlers; o código gerado junta os handlers de todas e, havendo variantes, traz `Componentes::de(ui)` com `Option` nos componentes que não existem em todas. IDE: Projeto → Nova variante do form (copia o padrão), variantes listadas sob o form, cada uma na sua aba. Sem herança de form nem de DataModule.
- [x] IDE multi-idioma: português, inglês, chinês, hindi, espanhol, francês, árabe, bengali, russo, japonês, alemão e indonésio; o do sistema por padrão, Exibir → Idioma troca na hora e fica guardado. Catálogo em `scripts/ide_idiomas.py` (238 textos: menus, painéis, diálogos, avisos). Pedido em 26/10.
- [x] Formas geométricas (paleta "Shapes"): Rectangle, Ellipse, Circle, Line, Polygon (lados ou pontos), Star, Pie, Arc e Path (SVG), com `fill`, `stroke`, `stroke_width` e rotação; clique e hover acertam a figura (`Component::contains_point` no núcleo, `Path::contains`). Testes `aylis-controls/tests/formas.rs`, exemplo `formas`. Pedido em 26/10.
- [x] Componentes de BI (paleta "BI"): `KpiCard` (valor, variação com cor pelo "melhor", tendência), `PivotTable` (linhas × colunas, soma/contagem/média/mín/máx, totais, clique para detalhar), `Slicer`, `ChoroplethMap` (GeoJSON, escala de cor, hover e clique na região); o `Chart` ganhou dispersão, barras empilhadas, cascata, funil, treemap e radar (já tinha linha, barra, área, pizza, rosca, sparkline e mapa de calor; o medidor é o `Gauge`). Pedido em 26/10.
- [x] Componentes de trader (paleta "Trader"): `CandleChart` (candles com volume, SMA/EMA/Bollinger, IFR e MACD no painel de baixo, perfil de volume, zoom na roda, arrastar no tempo, cursor com OHLC, desenho de tendência e Fibonacci, `push` do candle em formação), `OrderBook` (acumulado, spread, clique dá o preço), `DepthChart`, `TickerTape`, `Watchlist` (pisca na alta/baixa) e `TimeAndSales`. Exemplo `painel`. Pedido em 26/10.
- [x] Lista com modelo de item e reciclagem (como o RecyclerView): `VirtualList` (modelo = primeiro filho desenhado no designer ou um form em `item_form`; só as linhas visíveis + 2 existem e são reaproveitadas; `on_bind(índice)`, `bindings` campo→componente com uma `RowSource`, `on_load_more` para lista infinita, `scroll_to`, `refresh`) e `DBVirtualList` ligada a um DataSource (acompanha o dataset). `Ui::clone_component` no núcleo. Exemplo `lista_infinita`. Pedido em 26/10 (o `Layout` já existia).
- [x] Outras linguagens para o app: TypeScript (Node e Bun), C#, Java, Go, C++ e Free Pascal, sobre o protocolo do
  aylis-host (como o Python). Clientes em `clients/` (avaliação de esforço no README de lá), e linguagens de projeto
  na IDE: gerador de handlers por form (bloco de registro refeito, funções do usuário mantidas), F9 compila e
  executa pelo aylis-host, publicação desktop (pasta com o aylis-host, o projeto, o backend e um atalho),
  Verificar ambiente e configurações do VS Code para anexar o depurador. Testes `aylis-host/tests/linguagens.rs`
  (cliente de cada uma) e `projetos_linguagens.rs` (projeto gerado, compilado e executado), todos passando aqui.

**Por último (o usuário testa no fim)**
- [x] Publicação: APK assinado com a chave do desenvolvedor (criar a chave na IDE ou com `aylis-tool keystore`; a senha
  nunca fica no projeto), AAB para a Play Store (`bundletool`; validado e o APK universal dele testado no emulador),
  instalador `.msi` (WiX 5; conferido por extração administrativa), e Python e as outras linguagens no celular e no
  navegador: app casca em Rust com os forms, ligado a um servidor com `aylis-host --bridge` (testado no emulador com
  um projeto TypeScript). Detalhes em `docs/carregamento.md`.


Fechadas e enviadas até o commit `53b81c0` (publicação).

## Ajustes de 26/10/2026

**Instalação e janela da IDE**
- [x] Pasta pronta para executar e depois o instalador NSIS (`aylis-tool installer-ide`),
  sem console no duplo clique, ícone próprio (`494fc25`, `20385c2`, `ad8cc4d`).
- [x] Layout da IDE: propriedades à esquerda, paleta à direita (`494fc25`).
- [x] Confirmações com o `MessageDialog` do aylis em vez da caixa do sistema (`ad8cc4d`).
- [x] Ctrl+Shift+N e Cancelar não deixam mais a tela escura; a GPU não estraga a janela
  principal depois de um diálogo (`b4c0d81`, `3b3c15e`).

**Galeria e paleta**
- [x] Galeria de componentes vivos, cada um no tamanho padrão (`ad8cc4d`).
- [x] Busca da paleta, do Object Inspector e da galeria filtrando de verdade (`d23a7df`).
- [x] Categorias de visuais e não visuais; os não visuais fora da galeria, com ícone,
  descrição e janela própria (`640cdb5`).
- [x] Abas de plataforma na paleta (Windows / Apple / Android / aylis) e na galeria, com as
  versões; o componente entra com o tema da versão (`640cdb5`, `14221c2`).

**Designer**
- [x] Componente girado: seleção, alças, clique e redimensionar naturais (`c4957b1`,
  `bc1d67a`).
- [x] Duplo clique abre o editor do componente ou o código do evento padrão (`b5ec742`).
- [x] Janela de campos do ClientDataSet, como o Fields Editor (`3c1a5ed`).
- [x] X nos não visuais da bandeja; editor de menu (divisórias, ícones, marcas, evento por
  item ou do menu); lista suspensa do inspetor não fica órfã (`640cdb5`).
- [x] Ordem de tabulação entre níveis e o primeiro controle focado (`640cdb5`).
- [x] Fora do form aparece apagado e dá para pegar; listas suspensas acompanham a rolagem;
  `visible` desligado fica translúcido; desabilitado aparece na hora (`640cdb5`).
- [x] A roda só rola quando o form não cabe (`f6b51c2`).

**IDE**
- [x] Gravação do uso (`EventRecorder`), métricas (`Analytics`) e o visualizador
  (`f1fd560`).
- [x] Menus que dependem de projeto desabilitados sem projeto; tempo de compilação na
  barra de status (`14221c2`).
- [x] Seletor de ícones nas propriedades de ícone; os ícones aparecem na execução sem
  projeto (`14221c2`).
- [x] Demonstrador da suavização em loop (pedido de quadro por janela) (`640cdb5`).

**Componentes**
- [x] Correção e sugestão de texto (motor nativo do Windows, motor próprio do app),
  seleção e cursor por código, aparência do cursor (`640cdb5`).
- [x] Fonte em todo controle, herdada do pai; sublinhado e tachado nos três
  renderizadores (`14221c2`).
- [x] Cores próprias do botão (fundo, borda, texto); texto legível em Perigo e Sucesso em
  todos os temas (`14221c2`).
- [x] `Field`: `on_set_text` por nome e as funções `aylis::data::get_text`/`set_text`
  (`f6b51c2`).

**Acessibilidade e automação**
- [x] Nome do componente como AutomationId; abas, listas, menus e o grid expondo os seus
  itens (clicar e selecionar sem mouse); `ui.click_item` e os comandos `items` e
  `click_item` no inspetor (`f6b51c2`).

**Documentação**
- [x] O que depende de Mac ou celular em `docs/testes-em-dispositivos.md` (`5b231e7`).

## Lista de 26/10/2026 (fechada em 27/10/2026)

**Correções e lacunas**
- [x] Propriedades de referência listam só o tipo certo (datasets, DataSource, PopupMenu,
  controles focáveis, visuais), nunca o próprio componente nem o form; `Field` e
  `MenuItem` fora da paleta; a busca do Object Inspector limpa ao trocar de componente
  (`d180cf9`).
- [x] `post` espera as respostas pendentes do `on_set_text` remoto; o tamanho automático
  mede com a fonte do controle (`Ui::font_of`) (`9318633`).
- [x] Automação no TreeView, ListView, Calendar, chips, ButtonGroup, SegmentedControl e
  RadioGroup (`f252fbe`).
- [x] Textos novos da IDE nos 12 idiomas; `aylis-tool idiomas` acha os que faltam e aplica
  uma tabela; mensagens montadas com valores (`"Form {} criado"`) traduzidas pelo modelo
  (`b9742a9`).
- [x] Motor de correção e sugestão no Linux: Enchant 2 (Hunspell) por dlopen, conferido no
  WSL (`e1a52cf`).

**Melhorias**
- [x] `Model3D` com traçado de raios (`ray_traced`): sombras, oclusão e reflexos por ray
  query, e amostras acumuladas com a cena parada (`81a4635`); o Publicar leva o DXC
  (`80b1139`).
- [x] `VirtualList` com tamanho variável por item, horizontal e em grade (`e61eb73`).
- [x] Depurar na IDE as linguagens do aylis-host: Go (Delve), C# (netcoredbg), TypeScript
  (js-debug no Node), Java (adaptador DAP sobre o jdb), Free Pascal e C++ (CodeLLDB)
  (`5863964`).
- [x] Interface espelhada em árabe e hebraico (RTL), com `keep_left_to_right` para travar
  um controle (`6498bf2`).
- [x] Escolha da placa de vídeo (`gpu` no projeto, `AYLIS_GPU`, Opções do projeto)
  (`4797657`).
- [x] Cores e fonte próprias em todos os componentes (`fill`, `border_color`,
  `text_color`, `accent_color`, `success_color`, `warning_color`, `danger_color`)
  (`99e7f09`).

**Idioma do projeto, alvo de execução e Linux**
- [x] O idioma sugerido para o app segue o da IDE; o combo do Executar só mostra as saídas
  da linguagem; saída Linux pelo WSL (executar pelo WSLg com GPU d3d12, publicar executável
  e `.deb`, verificar ambiente) (`4e62880`).

**Gráficos e relatórios**
- [x] `Chart` com os tipos do Chart.js (bolhas, área polar, barra horizontal, linha e área
  empilhadas, degraus, suavizada, misto), vários eixos Y, escala logarítmica, eixo de
  tempo, mínimo e máximo, eixo invertido, legenda clicável, dicas em todos os tipos,
  `on_click` com série e índice, animação, zoom e arrasto, aparência por série, dados em
  JSON e o `DBChart` (`5be5f3c`).
- [x] Relatório de bandas (`Report`, `ReportBand`, `ReportPreview`): grupos, totais,
  visualização, impressão pela impressora do sistema e PDF (`c80c00d`).

**Código e depuração**
- [x] Realce de sintaxe em Pascal, TypeScript/JavaScript, C#, Java, Go e C/C++; Rust,
  Python e JSON revisados (`ddb4cba`).
- [x] Depuração de Rust e Python pela IDE revisada de ponta a ponta (testes `depurar` da
  IDE e `dap` das ferramentas) (`5863964`).

**3D: modelos do autor**
- [x] Os seis GLB abrem e desenham certo: cores por vértice, `KHR_texture_transform`,
  extensões obrigatórias desconhecidas ignoradas (`c4772a4`) e verniz
  (`KHR_materials_clearcoat`) (`4797657`).

## Apps de demonstração (pedido de 27/10/2026)

- [x] A Calculadora, o Paint e o Reprodutor Multimídia do Windows 11 em Rust, Python,
  TypeScript, C#, Java, Go, C++ e Free Pascal (`examples/apps`), com o mesmo form em todas
  as linguagens e o mesmo resultado conferido por roteiros de teste.
- [x] Roteiros de teste sem janela (`aylis_remote::roteiro`, `aylis-tool roteiro`), o ping
  dos clientes, `aylis-tool novo` e `aylis-tool gerar`.
- [x] `Component::invoke` (métodos pelo nome, também pelo protocolo) e o `PaintCanvas`
  (ferramentas do Paint, seleção com as setas, copiar/colar, desfazer, PNG/JPEG/BMP).
- [x] Ícones de desenho e de player; o estilo `round` do botão; botão marcado sem estilo
  próprio fica apertado.

## Revisão de segurança (31/10/2026)

Decidida item a item com o autor e implementada no mesmo dia (fases 1 a 7 e P2 a P6);
testada pelo autor e enviada nos commits `6df1fb1` e seguinte. Também entrou: Rust mínimo
1.88 (a correção do `time`, RUSTSEC-2026-0009), `publish = false` nos projetos gerados,
`aylis-tool fetch`/`update-tools`/`audit`/`update-catalog`, `resources/tools.lock.json` e
`deny.toml`; o `focus_text`/`target_text` das gravações deixou de levar o texto de campos de
senha (vazava os 60 primeiros caracteres). O roteiro dos testes de apps saiu do
`AYLIS_SCRIPT`: vai na linha do stdin do app de desenvolvimento (a mesma do aylis-host).
Achado no teste de verdade (31/10) e corrigido: a pergunta de confiança e a de recuperação
abriam atrás da IDE (o modal agora tem a janela principal como dono); a abertura (lingote)
junto com a recuperação travava sem janela nenhuma; os dois diálogos não redesenhavam a IDE
ao fechar; a captura pelo canal (`snapshot`) não existia num app só com Direct2D; os testes
da IDE gravavam pastas temporárias na lista de projetos confiáveis (agora tiram no fim, e a
IDE limpa pastas que não existem mais ao abrir).
- [x] 1. Modo de desenvolvimento: feature `dev` única (inspeção, hot reload, rastro); publicar =
  sem `dev`; Executar pela IDE (debug e release) usa `dev`. O app conecta na IDE (endereço nas
  opções do projeto, padrão 127.0.0.1, porta fixa separada do MCP), sem variáveis de ambiente,
  parâmetros ou `?aylis_dev=`. Sai o `inspect.rs`. Canal com Noise `NKpsk0` (`snow`): chave
  X25519 da IDE por máquina, segredo por projeto em `%LOCALAPPDATA%\aylis`, fora do git. Sem a
  IDE, o app segue tentando em silêncio; rastro pelo canal; várias instâncias. `aylis-host`
  recebe endereço e segredo pelo stdin. Testes controlam o app pelo canal (snapshot, bench,
  roteiro). Ajustes de suporte (renderizador, GPU, zoom, idioma...) fixos no projeto. Sai o
  `AYLIS_ROOT`; `AYLIS_UPDATE_CATALOG` vira `aylis-tool update-catalog`; caminhos dos
  compiladores (`AYLIS_GO`...) vão para as Opções da IDE.
- [x] 2. Nomes de handler e de form validados (`is_identifier`) em `Ui::set_prop`, no
  `aylis-json` e em `codegen::form_handlers`: não viram código injetado.
- [x] 3. `--bridge`: 50 conexões por IP e 200 no total, prazo de 10 s antes do handshake,
  mensagem até 1 MB, filas com limite, inatividade de 30 min com ping, `--allow-origin`; seção
  "Segurança do backend" nos clientes (eventos podem ser forjados; validar no servidor).
- [x] 4. Remover o `--listen` do `aylis-host` e o `Aylis(url=...)` do cliente Python (a tela
  conecta no servidor, nunca o contrário); testes `websocket.rs`/`wss.rs` com `--connect`.
- [x] 5. (resolvido pelo 1: o app não abre mais porta de inspeção.)
- [x] 6. MCP da IDE com token Bearer (`%LOCALAPPDATA%\aylis`, "Gerar novo token"), limites de
  cabeçalho e conexões, e `aylis-tool mcp` (ponte stdio→HTTP que lê o token sozinha).
- [x] Tela **Opções da IDE**: Geral (gravação de uso), uma aba por linguagem (compilador,
  depurador e servidor de linguagem achados no PATH e locais padrão, editáveis, com Instalar),
  Editor externo (VS Code), MCP (Configurar: Claude Code roda `claude mcp add`; Claude Desktop
  grava sozinho no `claude_desktop_config.json`, inclusive na cópia do pacote MSIX; no Linux só
  Claude Code). Windows, macOS e Linux.
- [x] 7. Sem `cmd /C`: `aylis_tools::shell::open` (`ShellExecuteW`, `open`, `xdg-open`) e o
  `code` chamado direto (PATH e local padrão de cada sistema); `%` → `%%` no `compilar.bat`.
- [x] 8. Projetos confiáveis: lista em `%LOCALAPPDATA%\aylis` (criados na IDE e exemplos entram
  sozinhos), pergunta ao abrir, modo restrito (sem servidor de linguagem, compilar, executar,
  depurar e publicar), o MCP não confirma confiança, `pylsp` com `-P`, forms do `.aylisproj` sem
  caminho absoluto ou `..`.
- [x] 9. Eventos reentrantes como no Delphi: receptor, handlers por nome e closures viram
  `Rc<dyn Fn>`; limite de 64 níveis de aninhamento com mensagem; API C com `catch_unwind` e
  callback `extern "C-unwind"`.
- [x] 10. Ferramentas com versão fixa e SHA-256 conferido (CodeLLDB, jdtls, bundletool, clangd,
  netcoredbg, js-debug), versão fixa nos gerenciadores (gopls, Delve, pylsp, debugpy, npm com
  lock, `dotnet tool`, commit do pasls), download e extração em Rust (sem `cmd`/`sh`), cache em
  `%LOCALAPPDATA%\aylis\downloads`, `aylis-tool update-tools`. Não vão no instalador.
- [x] 11. DocView: DOCX/ODT até 64 MB descompactados, planilhas conferidas pelo índice do ZIP
  (256 MB, razão 100:1), PDF até 64 megapixels, leitura fora da thread da tela, limites como
  propriedades.
- [x] 12. Expressões: profundidade até 256 (conferida durante a montagem), texto até 64 KB;
  documentar parâmetros (`:nome`) no `filter` em vez de concatenar.
- [x] 13. WebView: propriedade `message_url`, `"info": {"url": ...}` no JSON do evento,
  `allowed_origins` (padrão: a origem da URL inicial), `on_navigating` com cancelar, `sandbox`
  no iframe da web.
- [x] 14. Imagens: `aylis_core::image::checked_size` (sem estouro de `u32`), até 16.384 px por
  lado e 100 MP; na decodificação do Skia e do WIC, no `render_png` e na captura.
- [x] 15. Gravador: `sensitive` (LGPD) nos controles de texto e no `Field`, alterável em
  execução; `record_sensitive` global no `EventRecorder` (padrão desligado); `password` sempre
  mascarado; mudanças entram na gravação. Na IDE: opção visível e "Mascarar o texto do editor
  de código" (desligada).
- [x] 16. `process_events` até 10.000 eventos por chamada (o resto na próxima volta), aviso de
  laço com 1.000 repetições, profundidade máxima de 256 componentes.
- [x] P2. Caminho relativo do aylis no `Cargo.toml` de projetos dentro do repositório (exemplos
  regerados, teste contra caminho absoluto); tirar `lottie-ref` do `.claude/launch.json`.
- [x] P3. `deny.toml` + `aylis-tool audit` (bloqueia o `installer-ide`); alinhar `windows`
  0.62, `quick-xml` 0.42, `zip` 9 e o `cpal`.
- [x] P4. IDE: corrigir os `expect("projeto aberto")` depois de diálogo assíncrono; recuperação
  como a `__recovery` do Delphi (cópia a cada 2 min, gancho de pânico, pergunta ao abrir).
- [x] P5. Mídia: decodificador aberto dentro da thread (sem `unsafe impl Send`), `OnceLock` na
  câmera do Android, áudio lido por partes na thread de áudio (hoje o gst/avf/mediacodec lê o
  arquivo inteiro na thread da tela).
- [x] P6. `mf.rs` sem supor alinhamento de `f32`; `avf.rs` conferindo o tamanho da linha.

## Plano de 01/11/2026 (`c475abd` e `48081bc`)

Montado pergunta a pergunta com o autor, em 9 fases. As fases 2 a 9 entraram no `c475abd`; as
dúvidas resolvidas depois, o teste no ZenFone e a lista "faça tudo" do fim do dia, no
`48081bc`. O que sobrou de cada item continua nos [ajustes pendentes](AJUSTES-PENDENTES.md).

**Fase 1 — Android no ZenFone** (`0270a2b`, `48081bc`)
- [x] `examples/TesteAparelho` e `examples/Estresse3D` no aparelho: teclado, câmeras, microfone,
  vídeo, notificações, tema, área segura, giro e volta do segundo plano; a queda ao abrir no
  Android 9, as cores do MediaCodec nos Snapdragon, o 3D na Adreno e o 3D a 60 quadros/s (o
  corte do brilho em 8 px, decidido pelo autor). O halo, o nível do microfone e o 3D por alguns
  minutos conferidos pelo autor.
- [x] Gestos padrão do 3D (um dedo gira, pinça aproxima, dois dedos deslocam, toque duplo
  centraliza), conferidos no aparelho; o Inspetor e o hot reload pelo cabo; a barra de rolagem
  das listas arrastável.
- [x] Câmera: `CameraView.quality` (`low`, `medium`, `high`, `max`), `resolution`, os limites do
  aparelho (`sizes`, `frame_size`, `has_flash`) e `torch`, na proporção do sensor. Android,
  Windows, Linux (V4L2) e navegador; as quatro qualidades e o flash conferidos no ZenFone.
- [x] Aparelhos Android na IDE: a lista do adb ao lado da saída Android e a reconexão sozinha
  ao pôr o cabo de volta (conferida no ZenFone).

**Fase 2 — IDE no dia a dia** (`c475abd`)
- [x] Tela inicial limpa, com o menu só do que faz sentido sem projeto.
- [x] Mensagens de compilação copiáveis (`MessageList`, Ctrl+C, "Copiar tudo") e a barra de
  progresso para toda linguagem; Ctrl+Z no `Edit`, no designer e no editor.
- [x] Marcadores numerados e sem número (`.aylis/bookmarks.json`, Ctrl+Alt+B); regions e dobra
  por nível.
- [x] Pesquisa no arquivo (regex, `$1`, preservar maiúsculas, só na seleção) e no projeto
  (Ctrl+Shift+F, também nos forms, substituir com desfazer).
- [x] Extensões registradas (`.aylisproj`, `.aylispkg`, `.aylisrec`, `.aylisreport`) com um ícone
  por tipo; uma IDE por projeto e o MCP com várias IDEs (`ide_list`, `ide_select`).

**Fase 3 — Núcleo** (`c475abd`, `48081bc`)
- [x] Propriedades só de pintura (sem refazer o layout); o contorno `turn` do lingote saiu.
- [x] `Timer.precision` (`normal`, `high`, `exact`), `background` e `drift`.
- [x] Gramáticas TextMate e temas do VS Code nas 10 linguagens, "Importar linguagem…" e
  "Importar tema…" (`.vsix`); o motor passou para o Oniguruma (o fancy-regex fica no wasm): o
  C++ em ~1,2 ms por tecla em 5 mil linhas, as outras em 0,2 a 0,6 ms.

**Fase 4 — Dados** (`c475abd`, `48081bc`)
- [x] "Ligar a campo…" em qualquer propriedade (formato, expressão, mão dupla, metadados e o
  erro); `ComboBox.values`/`value`.
- [x] Paleta de dados unificada: `DataGrid`, `DataNavigator`, `DataRepeater`; os `DB*` antigos
  saíram, sem compatibilidade (decidido em 01/11); exemplos convertidos e o código das 8
  linguagens regerado; o `DBEdit` virou o `CellEditor` da grade.
- [x] `Chart` ligado a dataset (`labels` e `series` pelas ligações).

**Fase 5 — Pacotes** (`c475abd`, `48081bc`)
- [x] `.aylispkg` composto (tipo A): assinado (Ed25519), instalado por versão, um por projeto, na
  paleta; propriedades e eventos expostos; versão maior com notas e `renamed`; Projeto →
  Pacotes…; remover vai para a Lixeira.
- [x] Pacote de código (tipo B): `kind = "code"` com `crate` e `component`, `aylis-tool
  pack-code`, só em projeto Rust (dependência marcada no `Cargo.toml`, `register` no código
  gerado, compilação na hora); exemplo `examples/pacotes/Medidor`.

**Fase 6 — Componentes** (`c475abd`, `48081bc`)
- [x] Industriais e elétricos (16 componentes e o `ElectricalSymbol` com 12 subtipos); canos e
  fios presos aos pontos de ligação, e ligados arrastando no designer.
- [x] Áudio:
  - `AudioOutput` (lista de saídas, `on_devices_changed`);
  - `output` no `MediaPlayer`, `VideoPlayer` e no `Sound` novo, com várias saídas
    sincronizadas e volume e atraso por saída;
  - o loopback de cada saída na lista de entradas, `monitor` no `Microphone`;
  - o `AudioMixer` (volume, mudo e panorâmica por fonte; saídas, WAV e amostras).

**Fase 7 — Projeto** (`c475abd`, `48081bc`)
- [x] Ícone do app de um PNG ou SVG, gerado ao publicar para cada plataforma, com a prévia.
- [x] Projeto → Android… (versões, SDK, orientação, permissões travadas com o motivo,
  `uses-feature`, deep links, `manifest-extra.xml`, o manifesto final).
- [x] Vistas por largura: `views` no form, aplicadas pela largura real; no designer "Desktop
  (base) / Tablet / Celular" com a moldura, o inspector com a vista e "Voltar ao base".
  Conferidas na tela pela IDE no navegador.

**Fase 8 — Ferramentas** (`c475abd`, `48081bc`)
- [x] Relatar problema: o arquivo `.aylisreport` (problema, quem relata, máquina, projeto e
  anexos), o visualizador com dois cliques e a pergunta ao abrir depois de um erro fatal.
- [x] Profiler (Executar → Profiler…): faixas de handlers, eventos, layout, pintura, timers,
  animações, dados, recursos, marcas e memória; a tabela de métodos; `profile("nome")` nas 8
  linguagens.
- [x] Depurador: o tempo de processador do passo, a espera e, no Rust, a memória, no fim da
  linha amarela.

**Fase 9 — Web** (`c475abd`, `48081bc`)
- [x] Animações `appear` (ao aparecer na tela) e `keyframes`; o `appear` conferido na landing.
- [x] `examples/Landing` e `examples/Galeria`, publicadas e conferidas no navegador (desktop e
  celular); `aylis-tool serve`; o `auto_size` de grade e fluxo.
- [x] IDE lite no navegador: a mesma IDE sem a feature `desktop` (`lite.rs`, `files.rs`,
  `host.rs`, `aylis-ide-web`, `aylis-tool ide-web`), menus pela capacidade, o form guardado no
  navegador, o ZIP do projeto e a prévia; o CI compila para wasm.

**Também em 01/11** (`48081bc`)
- [x] Ponte com sessões (`aylis-host --bridge --sessions`) e o cliente Python com
  `aylis_client.Session`.
- [x] `TreeView.full_expand`/`full_collapse` com os eventos de cada nó.
- [x] FBX de verdade nos testes (modelos do Assimp; o `huesitos.fbx` em `aylis-3d/tests/data`).
- [x] CI no GitHub Actions (auditoria, Windows, Linux, wasm, Android); o `colors_and_fonts`
  corrigido.
- [x] Os scripts Python viraram `aylis-tool` (`resources`, `emoji`, `bench`, `test-docs`).

## Ajustes de 02/11/2026

- [x] Rolagem lenta do editor de código em modo de depuração (pedido em 02/11; `216b353`).
  A roda rola 8 linhas por dente (o
  `wheel_lines` do editor) e o Ctrl+roda uma página, como no Delphi. Medido com `tests/scroll_profile.rs` da IDE (o `aylis.pas` do
  `ProcessamentoAssincrono`, a IDE inteira, a amostragem da pilha): a pintura do Aylis era
  ~20 ms por quadro. As causas:
  - o Skia montava cada texto do zero a cada quadro (HarfBuzz e a busca das fontes ao
    desenhar e ao medir): agora guarda o parágrafo de cada linha (`line_paragraph`);
  - o código sem otimização: as dependências com `opt-level = 2` e o núcleo, os controles,
    os renderizadores e a plataforma com `opt-level = 1` no modo de depuração (o passo a
    passo por linha continua); os projetos fora do repositório ganham o mesmo para o Aylis
    no `Cargo.toml` gerado (o código do app continua sem otimização);
  - o editor rola até a última linha ficar no alto, como no Delphi, e só desliza com
    quadros rápidos (até 20 ms).
  Ficou em ~6,6 ms com o Skia na CPU; na GPU (a IDE no Windows), 2,7 ms.
  - Repintar só o que mudou: o `ctx.repaint()` e a mudança de estado (passar o mouse,
    apertar, foco) marcam a área do controle (`Ui::damage`, `Ui::take_damage`); o Skia na GPU
    guarda a imagem do quadro anterior e repinta só essa área (recorte alinhado ao pixel).
    Na dúvida, a janela inteira (`request_repaint`, `invalidate`, controle girado ou
    deslocado, espelhado, que desenha por cima de tudo, form fora de uma janela). Medido
    com `AYLIS_SCROLL_PROFILE=1 cargo test -p aylis-ide --test scroll_gpu`: 234 de 240
    quadros parciais na rolagem, 2,7 → 2,2 ms, e a imagem montada aos pedaços igual à
    pintada inteira (0 pixels diferentes). Falta no Direct2D, no Skia na CPU e no navegador
    (repintam inteiro, como antes).

## Conferidos pelo autor em 07/11/2026 (Sessão 1 do roteiro: a IDE com o NoOBS)

Os itens 1 a 22 do [roteiro de conferência](docs/roteiro-de-conferencia.md), confirmados pelo autor
em 07/11; os commits de cada um estão no texto (os sem número entram no próximo commit).

- [x] Imagens soltas pelo app presas no cache dos renderizadores (pedido em 03/11, medindo o
  NoOBS): o Direct2D guardava na placa os pixels crus de toda imagem desenhada até 96 MB
  (`FRAMES_BYTES`, do traço do `PaintCanvas` em `d192d1a`), mesmo as que o app já tinha
  trocado. As prévias das telas do NoOBS (480×270 a cada meio segundo) levavam o commit de
  154 a ~340 MB. Feito em 03/11 (falta o autor confirmar): `Image::downgrade`/`WeakImage` no
  núcleo; o Direct2D solta a cada pintura o que é de imagem morta (bitmaps, quadros crus e
  decodificadas), o Skia e o web ao guardar uma nova. Os tetos ficam só como proteção para
  quem segura muitas imagens vivas. Medido: o exemplo que troca duas prévias ficou em 47 MB
  de commit (ia de 95 a 199 MB); o NoOBS, em 129 MB estáveis e 50 MB privados (`a7a967e`).
  Depois (pedido em 03/11): passando do teto, sai a imagem usada há mais tempo (o Direct2D
  apagava as 256 de arquivo de uma vez, e uma tela com mais de 256 decodificava tudo a cada
  pintura; o Skia e o web tiravam a primeira a entrar). Um cache só para os três
  (`aylis_core::ImageCache`), e os tetos configuráveis por projeto: `image_cache` no
  `.aylisproj` (`pixels_mb`, `files`), em Opções do projeto → Memória, com o que cada um
  guarda e quando aumentar (`docs/funcionamento.md`, "Imagens prontas") (`023ba9d`).
- [x] Métrica de memória no app em depuração (pedido em 03/11): no app compilado com `dev`,
  mostrar quanto de memória o processo usa e quantas imagens estão no cache (e quanto ocupam);
  depois, também a taxa de quadros. Feito em 03/11 (falta o autor confirmar): o painel de
  memória (`aylis_core::memory_panel`) no canto do form principal, a cada segundo: quadros por
  segundo (e o teto), memória comprometida e working set do processo (Windows, Linux,
  Android), o heap do Rust e as imagens de cada cache com nome (`image_cache::stats`).
  Ctrl+Shift+M mostra e esconde; fora das capturas. As imagens também no Profiler da IDE.
  Conferido no NoOBS com `dev` (`e9e4620`).
- [x] A imagem junto do cursor no arrasto (pedido em 03/11, para o NoOBS: a miniatura do
  vídeo até a lixeira; "um componente que permita definir algo junto do arrasto"). Feito em
  03/11 (falta o autor confirmar; os vídeos ele já conferiu): o `DragImage` (System), que o
  controle aponta em `drag_image`; a imagem por código, a de arquivo, o desenho de um controle
  ou o do próprio controle arrastado (`Ui::snapshot`); `ui.set_drag_image` troca num arrasto.
  No Windows, uma janela em camadas (`UpdateLayeredWindow`) segue o cursor pela tela inteira;
  nos outros sistemas, o form pinta dentro da janela. No NoOBS: a miniatura do vídeo e, na
  pasta, o desenho do cartão. (Decidido pelo autor em 05/11: fazer o arrasto para fora do app
  e a imagem em janela própria no macOS e no Linux.) Fora deste passo: o arrasto do sistema entre programas (OLE
  `DoDragDrop`, soltar o vídeo no Explorer), que tem laço próprio e precisaria casar com o
  arrasto interno; e a janela própria no macOS e no Linux (`2f2a79e`; no NoOBS, a pasta em `9cca356`).
- [x] A janela sumia ao entrar em tela cheia (relatado em 04/11, no player do NoOBS): o app
  mostra a janela pela API do Windows (ao voltar da bandeja), o winit achava que ela estava
  escondida e, ao trocar o estilo para a tela cheia, tirava o `WS_VISIBLE` (sumia até da barra
  de tarefas). Feito em 04/11 (falta o autor confirmar; sem commit): antes da tela cheia, o
  `win32::sync_shown` avisa o winit que a janela está à vista. Conferido no NoOBS: entra e sai
  da tela cheia.
- [x] Como o quadro chega à janela, por projeto (pedido em 04/11: o aviso "Alt+R" do Radeon
  Software e o "Alt+Z" da NVIDIA; "configurável por projeto"; depois, o mouse parando quase 1 s
  ao entrar em tela cheia). Feito em 04/11 (falta o autor confirmar; sem commit): `present` no
  `.aylisproj` (vazio, a swap chain da janela, como antes; `composition`, DirectComposition, como
  os navegadores), em Opções do projeto → Geral → "Apresentação (Windows)", `AYLIS_PRESENT` nos
  diagnósticos (`aylis_core::gpu::PresentMode`, `aylis-d2d` `compose_into`,
  `docs/renderizadores.md`). Medido: o app recebia o mouse e desenhava normalmente durante a
  parada (laço de eventos sem buraco acima de 14 ms); com o DirectComposition o mouse não para
  mais (conferido pelo autor). O aviso da AMD continua em tela cheia nos dois modos; o NoOBS
  ficou com `composition`. A NVIDIA falta conferir.
- [x] Duas listas de fontes na tela (pedido em 04/11): a janela de código tinha o painel
  "Fontes" à esquerda, com os mesmos arquivos da árvore do Projeto, só que agrupados por
  pasta. Feito em 04/11 (falta o autor confirmar): o painel saiu, inclusive na janela de
  código destacada; a navegação fica na árvore do Projeto, e o Ctrl+F12 (Abrir fonte) acha
  pelo nome em qualquer janela.
- [x] Editor de código embutido com cara de janela à parte (pedido em 05/11): tinha a própria
  barra de menu (Arquivo, Editar, Navegar, Exibir, Depurar), o botão Salvar e uma faixa de
  abas separada da dos forms. Feito em 05/11 (falta o autor confirmar): embutido, os mesmos
  itens ficam no menu do botão direito do editor (os atalhos continuam valendo), o Salvar é
  o da IDE (no código, salva o arquivo ativo e o form, se mudou; Salvar tudo salva também o
  código) e as abas são uma faixa só, como no Delphi: cada form é uma aba (Design | Código
  alterna entre o form e o código dele; clicar na aba, no código, mostra o código daquele
  form) e os outros fontes abertos vêm depois, cada um na sua. A janela de código destacada
  continua com menu, Salvar e abas próprias.
- [x] App aberto pela IDE caía com "failed printing to stderr: O pipe está sendo fechado"
  (relatado em 03/11, no NoOBS, num `on_resize`): com a IDE fechada ou a execução parada, o
  `eprintln!` do aylis entrava em pânico. Feito em 03/11 (falta o autor confirmar): as
  bibliotecas do aylis escrevem pelo `aylis_core::stderr_line!`, que ignora o stderr fechado
  (`9f4bfd4`).
- [x] Minimapa do editor de código sem as cores até o fim (relatado em 03/11): num arquivo
  grande o realce vem aos poucos e o minimapa guardava os traços do primeiro quadro (só o
  começo colorido). Feito em 03/11 (falta o autor confirmar): enquanto o realce não termina,
  os traços são refeitos a cada quadro (`63cf1d2`). Na primeira tentativa a IDE aberta era
  anterior à correção (o `target\debug` das 20:30): falta conferir com a IDE recompilada.
  Conferido por teste em 07/11 (`the_minimap_gets_the_colors_down_to_the_end`, no
  `code_editor_extras`: o quarto de baixo do minimapa ganha as cores); falta só o autor ver.
- [x] Fechar o último form abria um Form1 novo (relatado em 03/11). Feito em 03/11 (falta o
  autor confirmar): a IDE fica sem form aberto (`State::no_form`): sem aba, estrutura e
  inspetor vazios e o designer com o aviso "Nenhum form aberto"; abrir ou criar um form volta
  (`900d4fb`).
- [x] Tirar o "Depurar no VS Code" da IDE (pedido em 03/11). Feito em 03/11 (falta o autor
  confirmar): saiu o item do menu Executar (Shift+F5); depura-se na IDE (Ctrl+F5). O
  "Depurar no aparelho" (Android, anexa pelo VS Code) ficou (`900d4fb`).
- [x] NoOBS não compilava (relatado em 03/11): a verificação antes de compilar dizia que
  `green`, `alert` e `amber` não eram cores. São cores do projeto (`colors` do `.aylisproj`), mas
  a verificação carregava os forms sem o tema e as cores dele. Feito em 03/11 (falta o autor
  confirmar): `check::references` carrega como o app; `aylis-tool check` no NoOBS: 0 erros (fica
  o aviso da fonte do sistema) (`2c7daa0`).
- [x] Troca de abas lenta nas Opções do projeto (pedido em 03/11). Medido com o NoOBS em
  release: a Geral levava 213 ms (perguntava as placas de vídeo ao wgpu a cada vez), a
  Traduções 71 ms e as outras de 10 a 30 ms (cada troca refazia a IDE atrás duas vezes). Feito em
  03/11 (falta o autor confirmar): as placas guardadas na primeira consulta (e perguntadas numa
  thread ao abrir o projeto), o `refresh` adiado até a janela fechar e a Traduções salvando só
  os forms alterados e sem gravar o arquivo do idioma só por passar pela aba. Agora: Geral
  0,7 ms, Traduções 16 ms, Fontes e Android ~10 ms, as outras 1 a 3 ms (`c7d8232`).
- [x] Ctrl+B no código (pedido em 03/11): marca a linha com o próximo número livre, começando
  pelo 1 (1 a 9, depois o 0; com os dez em uso, um marcador sem número); em cima de um
  marcador, tira. Feito em 03/11 (falta o autor confirmar): `Memo::toggle_next_bookmark`, a
  tecla no editor e o item "Marcar com o próximo número" no menu Exibir da janela de código
  (`564605b`; o marcador acompanhar a edição seguinte, `a8d3064`).
- [x] IDE lenta com o NoOBS (pedido em 03/11: clicar num componente congelava a tela, o tema
  do NoOBS não aparecia, a abertura demorava; "tudo na IDE deve ser instantâneo"). Na gravação
  de uso, cada clique no designer levava de 3 a 7 s. Feito em 03/11 (falta o autor confirmar):
  - cada `refresh` refazia o painel de estrutura com um `free` por linha, e cada `free`
    remontava o índice de referências do app inteiro (1.692 componentes na IDE): o
    `Ui::free_many` libera de uma vez, o índice perde só o que saiu e o de nomes só cai se
    saiu alguém com nome;
  - todo `get_prop`/`set_prop` montava a lista das ~90 propriedades para achar uma (e a lista
    tirava repetidos comparando com todas): `prop_info` procura direto, e a lista usa um
    conjunto;
  - a árvore do projeto lia o disco a cada clique (o JSON de cada form, as variantes, os
    fontes): lê de novo numa thread e redesenha só se algo mudou;
  - o tema próprio (`themes/noobs-dark.json`) trocava pelo Aylis Light ao abrir (valia a
    escolha anterior da IDE): abre no modo do tema, e o claro/escuro procura a mesma família
    nos temas do projeto.
  Medido: o `refresh` de um clique foi de 662 ms para 3,3 ms; na IDE de verdade, o clique
  leva de 12 a 25 ms e a janela abre com o NoOBS em 0,44 s (`b6f0aba`).
  Depois (pedido em 03/11: "o F12 demorou bastante para mostrar o código"; ~4 s na gravação):
  - o F12 gerava o projeto inteiro: agora `codegen::generate_form` faz só a unit do form, os
    renomeados e o acesso tipado (o Cargo, as features e os ícones ficam para o "Gerar código" e
    o Executar), e só regrava o `.aylisproj` se algum form foi salvo;
  - o `aylis_build::code_classes` procurava cada uma das ~200 classes no texto inteiro de cada
    fonte (e remontava o texto sem espaços para as classes com feature): lê cada arquivo uma vez;
  - o `Cargo.toml` (que lê fontes e forms para as features) era montado mesmo existindo.
  Medido no NoOBS: o F12 foi de 448 ms para 28 ms; o "Gerar código" inteiro, de 410 ms para
  85 ms (`71fec43`).
- [x] "Gerar código" punha a feature `bluetooth` no NoOBS (achado em 03/11): o
  `aylis_build::code_classes` contava todo `"Bluetooth"` entre aspas no código Rust do app, e
  o NoOBS tem a palavra num comentário (o nome de um fone). Corrigido em 03/11 (falta o autor
  confirmar): uma classe que liga uma feature (`Bluetooth`, `Nfc`...) só conta criada de fato
  (`create("Bluetooth"`, `"class": "Bluetooth"`); as outras seguem como antes (`e9e4620`).
- [x] Opções do projeto com abas laterais (pedido em 03/11: "muitos botões azuis"). Feito em
  03/11 (falta o autor confirmar): uma janela só, as seções numa lista à esquerda e a escolhida
  à direita, montada pelo próprio editor dela (`dialogs::embed`: o `new_dialog` e o `run` da
  seção montam na página, sem janela). Trocar de seção aplica a aberta (com erro, fica nela);
  OK aplica e fecha; Cancelar descarta a aberta. "Ícone do app" e "Confiar neste projeto"
  ficam como um botão na página (um pede o arquivo, o outro só pergunta) (`0b07d07`).
- [x] Tela inicial da IDE sem a borda de foco na lista de recentes (pedido em 01/11, feito;
  falta o autor confirmar): a variante `flat` das listas (`ListBox`, `ListView`,
  `TreeView`, `Memo`, `Calendar`) tira a borda e a do foco, em todos os temas.
- [x] Dispositivos no designer (feito; falta o autor confirmar): 20 aparelhos em
  `aylis_core::devices` (Galaxy, Pixel, iPhone SE a 16 Pro Max, iPad, Moto, telas HD a 4K e
  ultrawide) no combo de vista, com a moldura, girar, recorte e barra de status; executar no
  desktop dentro da moldura (`AYLIS_DEVICE`, que a IDE passa ao executar; `--device` no
  `--png`).
- [x] Menus da IDE organizados (pedido em 03/11: "tem muita coisa que era pra ser propriedade do
  projeto, coisas que não são alteradas sempre"; feito, falta o autor confirmar): o que é do projeto
  e muda pouco (geral, aparência, fontes, ícone do app, Android, DataModules usados, traduções,
  confiar no projeto) saiu do menu Projeto e ficou na janela "Opções do projeto" (Ctrl+Shift+F11,
  `projectsettings.rs`), uma seção por assunto, como as "Project Options" do Delphi; a seção Geral
  ganhou as plataformas de destino. O menu Projeto ficou com forms, código, ícones e animações; o
  ambiente da máquina (Verificar ambiente, Configurar linguagens), os pacotes de componentes (num
  submenu) e o servidor MCP foram para Ferramentas. O atalho Ctrl+Shift+A (Aparência) saiu.
- [x] Profiler (feito; falta o autor confirmar): a faixa "GPU" por quadro (o tempo do quadro
  menos o da pintura); a amostragem da pilha no Rust (Windows x64: suspende a thread, desenrola
  com `RtlVirtualUnwind` e dá nome pelo dbghelp), com a árvore de chamadas e o flame graph na
  aba "Chamadas (amostras)"; o cliente C++ compilado e testado nas sessões. Falta a
  amostragem no Linux e no macOS.
- [x] Depurador (feito; falta o autor confirmar): a memória no fim da linha em C++, Free
  Pascal, C# e Java pela memória do processo (`cputime::process_memory`, quando o adaptador
  não dá); falta o tempo no macOS (precisa de Mac).
- [x] `aylis-build` (feito; falta o autor confirmar): as features do `Cargo.toml`
  (`codegen::aylis_features`) vêm das mesmas classes que o registro enxuto usa (forms, pacotes
  e as citadas pelo nome no código Rust); `"Column"` no código liga `dbcontrols` e `data`.

## Conferidos pelo autor em 07/11/2026 (Sessão 2 do roteiro: a IDE com o ERP)

Os itens 23 a 28 do [roteiro de conferência](docs/roteiro-de-conferencia.md), confirmados pelo autor
em 07/11. O item 16 (o delta mestre-detalhe) teve 16.1 e 16.2 confirmados aqui e continua nos
pendentes pela 16.3 (o guia na documentação da Landing, Sessão 4).

- [x] 21. A IDE com um projeto grande. A medição de 12 foi de compilação com 100 forms; a IDE em si
  (abrir o projeto, a árvore, abrir vários forms, gerar o código, a verificação do 1b,
  memória) nunca foi medida num projeto de ERP. Decidido pelo autor em 04/11: sem gerador no
  `aylis-tool`; um exemplo de verdade, feito uma vez e guardado em `examples/`, com pelo menos
  1000 forms, todos com datasets, um banco SQLite por baixo e mestre-detalhe; módulos de RH,
  Financeiro, Contábil, Compras, Vendas e Fiscal, com tabelas que façam sentido para cada um e
  uma lógica mínima de uso (sem regras de negócio detalhadas). Pronto quando o exemplo abrir,
  compilar e rodar, e os tempos da IDE (abrir o projeto, abrir forms, gerar o código, a
  verificação, a busca do 20, a memória) estiverem em `medicoes/`, com um limite num teste.
  Em Rust (decidido pelo autor em 04/11).

  Feito em 04/11 (falta o autor confirmar): `examples/ERP`, 1009 forms (252 entidades, cada uma
  com cadastro mestre-detalhe, lista, consulta e os itens de todas, e o menu com a busca), 504
  tabelas num SQLite que o exemplo cria e preenche (`rust/src/erp/banco.rs`, a gravação pelo
  `Delta` numa transação); as telas chamam o código comum (`rust/src/erp/mod.rs`). Fica fora do
  workspace (`exclude` no `Cargo.toml` da raiz). O teste `telas` do exemplo abre as 1008 telas
  (limite de 60 ms por tela; medido 7,7 ms em debug) e grava um pedido novo com dois itens, que
  voltam com o id do banco. O teste `check` do `aylis-project` passa a conferir o projeto (os
  exemplos todos). Medições em `medicoes/erp_1000_forms.json`; o que elas mostram:
  - o `components.rs` gerado tem 10 MB (220 linhas por form) num arquivo só; em release, mudar um
    form recompila o app inteiro (2 min 50 s; debug, 9 s): vale um item à parte, gerar um arquivo
    por form ou dividir em crates por módulo;
  - gerar o código 4,7 s, verificar o projeto 5,6 s, a IDE abrir e desenhar 2,7 s (as ferramentas
    em debug).
  Faltam as medições da IDE com vários forms abertos e da memória, e a busca do item 20.
  Medido em 07/11 (IDE em release, sem janela; `erp_ide_cost` no `perf` do `aylis-ide`, com os
  limites; números em `medicoes/erp_1000_forms.json`): abrir o ERP e desenhar 250 ms; abrir um
  form 356 ms, dos quais 324 ms registrando os ícones citados no projeto inteiro (os 1009 forms
  e o código lidos de novo a cada form). Corrigido: o designer registra só os ícones do form e
  dos forms que ele cita (frames e DataModules), e as citações de cada arquivo ficam guardadas
  pela marca dele para a geração. Agora abrir um form leva 47–89 ms (o documento, 5–8 ms);
  memória privada: IDE vazia 19 MB, o ERP aberto 62 MB, com 20 forms 83 MB (~1 MB por form).
  A busca de texto no projeto (2054 fontes e 1009 forms, 9080 achados) leva 2,7–7,4 s e o "onde é
  usado" 1,8–2,7 s, as duas numa thread (a IDE não trava; varia com o OneDrive).

  Pedido do autor em 07/11: "abrir o projeto do erp com todos os forms abertos em abas ao
  mesmo tempo para medir a troca de abas e a velocidade de mudar para código e design".
  Medido com os 1009 forms abertos (`erp_all_forms_in_tabs`, em release, com os limites;
  `memory_per_document` mostra de onde vem a memória de cada aba) e corrigido:
  - trocar de aba, de 218 para 42 ms: o refresh das abas normalizava o caminho do código de cada
    aba pelo disco (`canonicalize`, 1009 por troca, lento no OneDrive), e agora guarda o
    resultado (`debugger::norm`); a ordem das abas procurava cada aba numa lista (1 milhão de
    comparações), agora por mapa;
  - Design → Código, de 510 para 88 ms; Código → Design, de 359 para 36 ms (o mesmo refresh);
  - memória, de 1137 para 368 MB (de 1,07 para 0,30 MB por aba): cada `Ui` guardava três temas
    inteiros (~0,2 MB cada) e o documento mais uma cópia; os temas do `Ui` viraram `Arc`
    (`Ui::set_shared_theme`; quem muda um tema copia antes), e o Aylis Light e o Aylis Dark de
    fábrica e o tema do projeto são um só para todas as abas. Abrir os 1009 forms leva 4 s.
  Números em `medicoes/erp_1000_forms.json`.

  Relatado pelo autor em 04/11: abrir o ERP e clicar em Executar travou a IDE (na gravação de
  uso, ~8 s logo depois do clique e ~27 s durante a compilação). Medido e corrigido no mesmo dia
  (falta o autor confirmar):
  - o Executar gerava o código (2,9 s em release) e verificava o projeto (1,6 s) na thread da
    tela: agora rodam no trabalho (`jobs::prepare`), com as etapas "Gerando o código" e
    "Verificando o projeto" nas mensagens e os problemas voltando por `Msg::Issues`; o Publicar
    também;
  - o `save_all` relia todos os forms para os ícones e as animações (0,35 s, duas vezes por
    Executar): o Executar, o Compilar e o Publicar usam o `save_for_run`, que só grava, e a
    sincronização foi para o `prepare` (a parte na tela caiu para 0,5 ms);
  - o painel Projeto comparava cada fonte com a lista das units dos forms e procurava cada form
    pelo nome (quadrático): 165 ms a cada refresh da IDE, que é refeita a cada mensagem da
    compilação; agora um conjunto e o `codegen::entry_source`, 6,5 ms;
  - o `examples/ERP/rust/target` (353 MB, o aylis compilado do zero) vinha de uma ferramenta
    rodando o cargo na pasta do exemplo, fora do workspace (provavelmente o servidor de
    linguagem): o exemplo ganhou `.cargo/config.toml` com o `target` do repositório.

  Relatado pelo autor em 04/11: o F12 (ir para o código) ainda demorava; pedido em seguida:
  achar tudo que varre o projeto inteiro. Medido com o ERP (IDE em release) e corrigido (falta o
  autor confirmar):
  - F12, de 1,4–1,7 s para 0,11 s (0,25 s na primeira vez): renomear e registrar no
    `code-sync.json` só o form aberto (`sync::apply_renames_in`, `sync::record_in`; o projeto
    inteiro continua no Gerar código e no Executar) e conferir o `import` só no arquivo dele
    (`bindings::generate_in`);
  - as variantes de cada form eram 8 consultas ao disco por form (8 mil no ERP, lentas no
    OneDrive): `FormFiles` lê cada pasta uma vez (`read_entry_variants`, `entry_variants`), no
    acesso tipado, nas pendências, na geração e no painel do projeto;
  - `is_data_module` e `uses_of` liam e interpretavam o JSON inteiro do form: a classe da raiz
    e os `uses` ficam guardados pela data e pelo tamanho (`form_meta`);
  - o acesso tipado relia os 1009 forms e, para filtrar cada componente, montava a lista de
    propriedades da classe: o modelo de cada form fica guardado pela data e pelo tamanho dos
    arquivos dele, e o filtro é `Catalog::has`;
  - o `symbols::find` (a dica e o Ctrl+clique sem servidor de linguagem) lia e interpretava todos
    os fontes a cada vez: as declarações ficam guardadas por arquivo (97 → 3 ms);
  - os ícones e as animações do Salvar varriam os forms e todo o código (o gerado de 10 MB
    incluído), duas vezes: as citações ficam guardadas por arquivo, com a data e o tamanho da
    listagem da pasta (130 + 230 ms → 4 + 4 ms);
  - o Gerar código do menu roda no trabalho (1,7 s no ERP).
  Ficaram como estão, por serem ações pedidas e já fora da tela ou raras: a verificação do
  projeto e a publicação (leem cada form uma vez, no trabalho), as Pendências entre linguagens e
  as traduções (pelo menu), as referências e o renomear sem servidor de linguagem (leem os
  fontes ao pedir).

  Revisto em 04/11 a pedido do autor ("muito cuidado em colocar cache e thread... e não por
  soluções paliativas"): os caches em memória e a thread do Salvar saíram, no lugar de cada um a
  operação de verdade (decidido item a item em 04/11; falta o autor confirmar):
  - o servidor de linguagem é obrigatório: sem ele, a dica, ir para a declaração, as
    referências, renomear, os símbolos e as sugestões avisam que ele falta (saíram os substitutos
    que liam todos os fontes, o `symbols::find` e as ferramentas `find_declaration` e
    `go_to_declaration` do MCP);
  - o Salvar acrescenta em `assets/` só os ícones e as animações do form salvo
    (`codegen::add_assets_cited_in`); tirar os que ninguém usa é do Gerar e do Publicar;
  - o `.aylisproj` anota em cada form `data_module`, `uses` e `modified` (a hora do arquivo),
    gravados ao salvar o form e ao acrescentar um; ao abrir o projeto, só os forms com outra hora
    são relidos (`Project::refresh_forms`); quem escreve forms por fora chama o `refresh_forms`;
  - o acesso tipado é um pedaço por form e por classe em `generated/components/`, refeito só
    quando o JSON do form é mais novo que o pedaço (como no `make`; `bindings::stale_forms`), e
    um número de versão do gerador e do catálogo faz refazer tudo. Em Rust o `components.rs`
    inclui os pedaços (`include!`); nas outras 7 o arquivo de sempre é montado juntando os
    pedaços (a API não muda: o Java não tem classe parcial, e o código usa `Components.Button`).
    A saída é a mesma de antes nas 8 linguagens (conferido no IBGE);
  - a geração (`codegen::generate`) só lê os forms que mudaram e os de unit mexida pelo usuário
    depois do último registro (`sync::changed_since_record`); o que os arquivos que juntam todos
    precisam (nome, handlers, eventos-pergunta, classes e citações de cada form) fica em
    `generated/components/forms.json`; as features do `Cargo.toml` e o serviço em segundo plano
    saem das classes guardadas, e os ícones e as animações das citações guardadas mais o código
    do usuário, numa passada;
  - a verificação lê cada form e cada arquivo de código uma vez (antes, cada conferência relia;
    os handlers liam o arquivo a cada handler) e não lê o código gerado; com
    `check::project_changed`, só os forms que mudaram, os que citam um deles e os não vistos são
    conferidos, e dos outros vale o resultado guardado (`generated/components/checked.json`).
  Medido no ERP (release), o Executar antes de compilar: 2,8 s → 0,49 s sem nada mudado (gerar
  0,38 s, verificar 0,09 s); a verificação inteira 1,56 → 0,53 s; o F12 1,5 s → 0,11 s; o Salvar
  0,35 s → poucos ms. O que sobra é ler o código do usuário (as classes criadas por código, as
  citações de ícones e os `find_path`), que depende de qualquer arquivo de código: próximo passo,
  guardar isso por arquivo pela hora, como os forms. Aprovado pelo autor em 05/11 ("segue").

## Conferidos pelo autor em 08/11/2026 (Sessões 3 a 7 do roteiro)

Os itens 29 a 96 do [roteiro de conferência](docs/roteiro-de-conferencia.md) conferidos pelo autor
em 08/11 (com o Claude pelo MCP da IDE). Os ajustes achados na conferência ficaram nos pendentes
("Conferência de 08/11/2026"); estes entram no próximo commit.

- [x] `current_cell_border_is_the_theme_accent_or_cursor_color`
  (`crates/aylis-dbcontrols/tests/grid_extras.rs:332`, pedido em 03/11): depois do clique numa
  célula, o `DBGrid` não desenha a borda de 2 px da célula atual (o teste recebe nenhuma
  borda). A borda só sai com a grade em foco (`grid.rs`, `ctx.state.focused`), então o
  provável é que o clique do teste não dê mais o foco à grade. Já falhava antes de `37ceed8`;
  o commit que quebrou não foi achado (as versões antigas não compilam mais: `alsa-sys`).
  Pronto quando a causa estiver achada e o teste passar.
  Feito em 03/11 (`cafc5c8`; falta o autor confirmar): a causa era a pintura sem anel de foco
  no clique (`216b353`), que tira o `focused` do `ctx.state` de quem não é campo de texto; a
  borda da célula atual é o cursor da grade e agora lê o foco do próprio controle.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 33: `grid_extras` 15/15 e a borda de 2 px da célula atual na tela).
- [x] Vários monitores (pedido em 02/11, feito; falta o autor confirmar):
  - a lista no `Ui` (`ui.monitors()`, `ui.form_monitor(form)`): nome do aparelho ("XB273K V5",
    pela configuração de vídeo do Windows), nome do sistema, principal, posição, tamanho,
    área útil sem a barra de tarefas, escala, taxa de atualização e modos de vídeo;
    conferida a cada 2 s;
  - o componente `Screen` (categoria "Device") leva tudo às outras linguagens:
    `count()`, `primary()`, `monitors()` e `monitor(i)` em JSON, `refresh_rate(i)`,
    `form_monitor(form)` e `on_change`;
  - no form: `monitor` (em qual abrir; com `position = designed`, o `left`/`top` contam do
    canto dele), `remember_placement` (grava o monitor, a posição, o tamanho e se estava
    maximizada em `<dados do usuário>\<app>\placements.json` e reabre igual; com o monitor
    desligado ou a barra fora da tela, abre como se não houvesse) e `move_to_monitor(i)`;
  - o exemplo `monitors` (lista, um botão por monitor). Conferido nesta máquina: os dois
    monitores com os nomes e as taxas (160 e 60 Hz), mover para o segundo, reabrir nele e
    voltar. No navegador e no celular, a tela do aparelho (sem a taxa no navegador).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 30: os dois monitores com nome e taxa (XB273K 160 Hz, PHL 288P6L 60 Hz), mover e reabrir no segundo).
- [x] Gráficos novos (pedido em 01/11; não começou): **Sankey** (fluxos entre nós, a largura
  da faixa pelo valor; dados `origem;destino;valor`) e **Waffle** (grade de 10x10 quadrados,
  cada um 1% do total, uma cor por categoria). Radar (teia de aranha) e Treemap já existem
  como `kind` do `Chart` (`radar`, `treemap`).
  Feito em 03/11 (`cafc5c8`; falta o autor confirmar): `kind` `sankey` (rótulos `origem>destino`,
  a 1ª série com os valores; os nós em colunas pelo caminho mais longo, um ciclo não trava) e
  `waffle` (as sobras do arredondamento vão para os maiores restos, para somar 100); testes e o
  guia em `docs/funcionamento.md`.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 31: o Sankey e o Waffle (100 quadrados, 7/5/3 somando 100%), conferidos num form de teste).
- [x] `TreeView.full_expand` e `full_collapse` como métodos no protocolo (feito; falta o autor
  confirmar): pelo `invoke`, abrem logo depois pela fila, com o `on_expand` de cada nó; no
  catálogo (`components.json`), que também ganhou métodos do `AudioMixer` e do `Sound` que
  faltavam.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 32: chamados de um backend em Python, a árvore abre e fecha inteira).
- [x] Revisão da Landing e da web (pedido em 02/11, feito; rolagem confirmada pelo autor, o
  resto falta confirmar):
  - Landing: sem o "Ver os componentes"; "Baixar a IDE" abre as versões do GitHub; logotipos
    das linguagens (Devicon, MIT; o do Free Pascal desenhado); conteúdo com largura máxima
    (a propriedade nova `max_width`, que limita e centraliza); chips sem cara de botão;
    dúvidas recolhidas; o menu leva a seção ao topo (`Ui::reveal_top`); celular sem o vão
    embaixo do botão. Conferida de ultrawide a celular pelas capturas.
  - O anel de foco só pelo teclado (como o `:focus-visible`): o clique num botão, expansor ou
    chip não mostra a borda; os campos de texto mostram sempre.
  - "Aylis" com maiúscula nos textos de tela, nas 12 traduções, no instalador e nos
    cabeçalhos do código gerado (nomes de comando, crate, pasta e o marcador
    `# pacote do aylis` ficam como estão).
  - O lingote gira com o mouse arrastando (`Model3D` com `turntable`); o cursor de arrastar
    só aparece quando arrastar faz algo.
  - Rolagem: 100 pontos por dente com deslize (o padrão), o deslize anda a cada quadro
    (`Component::frame`, `Ui::request_frames`) sem arredondar no meio, a roda do navegador
    vira dente mesmo com a escala do Windows, e a web não fica mais presa a 60 quadros por
    segundo (medido no Edge: de 17 para 2,3 ms entre quadros rolando).
  - No navegador numa tela larga, os forms de cima viram janelas no canvas (barra de título,
    fechar, arrastar, o modal escurece o fundo); no celular, tela cheia como antes.
  - Cache: o `.js` e o `.wasm` publicados levam o hash do conteúdo no nome e um service
    worker (`sw.js`) guarda os dois; o `index.html` pergunta ao servidor e traz a versão
    nova; sem rede, abre a última. Falta conferir o cache num Edge ou Chrome de verdade (o
    painel embutido não registra o service worker).
    Conferido em 07/11 no painel embutido (agora registra): o service worker controla a página,
    e o cache tem o `/`, o `.js` e o `.wasm` da Landing e os das demos abertas (cada família no
    próprio escopo). Recarregar com o servidor parado não deu para testar ali (o painel recusa
    a navegação); falta isso num Edge ou Chrome de verdade.
  - Exemplos para testar no PC: `gamepad` (o controle e a alavanca) e `outputs` (várias
    saídas de áudio com metrônomo, para ajustar o atraso do fone Bluetooth).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 34 e 36: aprovada; os ajustes (fontes do "Aylis" e do "Baixar a IDE", o cache sem servidor) estão nos pendentes).
- [x] Na web, a roda sem movimento antes (feito; falta o autor confirmar): a posição vem do
  próprio evento `wheel` (um ouvinte na captura da janela, antes do winit).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 37: rola na hora).
- [x] IDE lite (feito; falta o autor confirmar): "Arquivo → Abrir projeto (ZIP)…" (o form
  principal, os temas e as traduções; imagens ficam de fora, com aviso; aceita ZIP com `\` do
  Windows); o IndexedDB no lugar do `localStorage` (carrega tudo antes de abrir e migra o que
  havia); o tamanho inicial já estava certo. Conferido no navegador: a migração, o ZIP da
  Calculadora e o form guardado entre visitas. O `std::fs` que sobrou na IDE fica nos
  caminhos que a lite não abre (compilar, processos, depurar); passar para o `Workspace`
  pede arquivos binários nele (as imagens).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 39: abre o ZIP da Calculadora e guarda no navegador ao recarregar (o ajuste da prévia cortada está nos pendentes)).
- [x] 1. Acesso tipado sem falha silenciosa. O gerado (`frmX.comp.set_text`) guarda o caminho
  em texto, procura o componente a cada chamada e descarta o erro (`let _ = ui.set_prop`); a
  leitura devolve o padrão quando o componente não existe (`read_only()` falso = "não é" ou
  "não achou"); `ui.on(id, "on_clik", …)` aceita evento que a classe não tem; handler por nome
  sem registro não avisa. (O carregamento do JSON já recusa propriedade e evento
  desconhecidos.) Pronto quando:
  - o gerado resolve os componentes uma vez, ao abrir o form (handles com o `ComponentId`),
    com erro claro dizendo o que não bate entre o form e o código; o form não abre se não
    bater (decidido em 02/11: a falha dura é a esperada; a mudança na API dos projetos
    existentes tem impacto baixo);
  - os acessos são por classe (`Ref<Label>`), não um par get/set por instância;
  - falha de `set`/`get` aparece (aviso em debug, `activity` em release), e há `try_` para
    quem quer tratar;
  - `ui.on` confere o nome contra os eventos da classe;
  - o hot reload refaz os handles: ele mantém o id dos componentes casados por nome e
    classe no mesmo pai, e só o renomeado, o que trocou de classe ou de pai e o removido
    ficam com o handle morto.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 40: em Rust o nome que não bate para na compilação, com arquivo e linha).
- [x] 2. Form com instância e estado. O form gerado é um estático com caminho fixo,
  `find_path` acha sempre o primeiro form com o nome, e o estado do form vai para
  `thread_local!` (o IBGE faz assim). Pronto quando:
  - um form aberto duas vezes (duas janelas de cliente, um frame embutido duas vezes) tem
    cada handler falando com a própria instância;
  - o gerado tem uma struct por form com o estado do usuário, e os handlers recebem `&mut`
    dela;
  - o modelo é o aprovado em 02/11 no exemplo em Pascal: a classe do form
    (`class(TAylisForm)`) com os componentes como campos `published` geridos pela IDE, o
    estado em campos `private`, os handlers achados pelo nome nos métodos `published` e um
    `RegisterForm` no lugar do bloco de registro; em Rust, a struct gerada com os campos e o
    `impl` no arquivo do usuário;
  - um evento do mesmo form disparado dentro de um handler dele (um `next` no dataset
    dispara o `after_scroll` na hora) fica adiado até o handler terminar, também no Rust
    (decidido em 02/11). As outras linguagens já fazem assim: o cliente guarda numa fila os
    eventos que chegam enquanto espera uma resposta (o `FQueued` do Pascal); só o Rust
    entrega na hora. Em modo dev, avisar quando um evento for adiado;
  - o IBGE sai do `thread_local!`;
  - o protocolo remoto e as outras linguagens têm o mesmo (a instância vem no evento).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 42: duas janelas do mesmo form, cada uma com o próprio estado (ajustes nos pendentes)).
- [x] 3. Eventos que perguntam, no backend remoto. O `on_close_query` depende de um
  `cancel_close` síncrono, e o receptor de eventos só envia: um backend em outra linguagem não
  consegue impedir um fechamento. Levantado em 02/11: `on_close_query` (`cancel_close`),
  `on_drag_start` (`start_drag`) e `on_drag_over` (`reject_drop`) só respondem em Rust;
  `before_post` e `before_delete` existem só como closure em Rust (nem são eventos do form);
  o `on_key_down` só avisa (nenhum handler segura a tecla, como o `Key := 0` do Delphi); o
  `on_validate` do Field já responde depois (`field_error`). Pronto quando:
  - as ações que podem esperar (fechar, gravar, excluir, aplicar) mandam o evento com id do
    pedido e ficam suspensas até a resposta ou o prazo, com um padrão definido; em Rust, a
    resposta dentro do handler continua valendo na hora, sem suspender;
  - `before_post` e `before_delete` viram eventos do form, por nome;
  - o que é rápido demais para esperar a rede (arrastar por cima, cada tecla) ganha
    propriedades declarativas (tipos aceitos no soltar, filtro de caracteres) e um jeito de o
    handler em Rust segurar a tecla;
  - o catálogo marca quais eventos são pergunta;
  - há teste nas 7 linguagens.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 43: o `on_close_query` do backend em Pascal decide, e o Cancelar do modal funciona com a thread rodando).
- [x] 4. (Feito em 02/11; falta o autor confirmar: `Error::Busy` com o caminho e a dica em
  `get_prop`, `set_prop`, `invoke` e `send_message`, o log em modo dev, o teste `busy` do
  `aylis-core` e a regra do composto em `docs/criar-componente.md`.) Componente ocupado
  (reentrância). `with_component` tira o componente do lugar
  enquanto o método dele roda; `invoke` e `send_message` nele falham com "ocupado" sem aviso,
  e `get_prop`/`set_prop` dizem "propriedade desconhecida" (o `bounds` e o layout também o
  perdem). Só afeta quem escreve componentes em Rust: o handler de uma tela roda depois que
  o método do componente termina (mudar o texto do botão no `on_click` funciona). Decidido
  em 02/11. Pronto quando:
  - um erro próprio (`Error::Busy`) com o caminho e a dica: "frmPedido.grdItens está
    ocupado: um método dele está rodando. Dentro do componente, use `self`; num componente
    composto, o pai lê os filhos, e o filho avisa o pai por evento", em `get_prop`,
    `set_prop`, `invoke` e `send_message`; em modo dev, também no log, com quem tentou;
  - o guia de componentes explica a regra do composto (o pai pergunta aos filhos na hora;
    o filho avisa o pai com `ctx.fire`; o filho nunca chama o pai no meio de um método do
    pai), com o exemplo da grade com editor na célula;
  - nada de adiar a mensagem para o componente ocupado: adiar trocaria um erro visível por
    um dado gravado no lugar errado (o texto do editor chegando depois de a grade descer de
    linha);
  - tirar o `ControlBase` de dentro do componente (para o layout nunca o perder) fica
    guardado, só se os avisos mostrarem necessidade real.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 44: o teste `busy` e a regra em `docs/criar-componente.md`).
- [x] 5. Layout e pintura por parte. `invalidate` marca o `Ui` inteiro, `layout` refaz todos
  os forms abertos e a pintura é do form inteiro a cada quadro (como o FMX; parado, não
  pinta nada). Decidido em 02/11: o modelo do Flutter, em etapas, cada uma útil sozinha (se
  a medição mostrar que o nível da VCL basta, para na etapa 2). Pronto quando:
  - 1, medir: um caso de 5 a 10 mil controles em vários forms no `aylis-tool bench` (digitar
    num Edit, um relógio na barra de status, uma barra de progresso animando, rolar uma
    grade), também na web, com limite na CI;
  - 2, nível da VCL: layout sujo por form e por parte (do controle que mudou até o primeiro
    pai de tamanho fixo, como o `AlignControls`); pintura só do retângulo que mudou (como o
    `Invalidate`) nos três renderizadores, com a área incluindo sombra, desfoque, rotação e
    anel de foco; na GPU (Skia GPU, Direct2D), a imagem da janela guardada ou os recursos do
    sistema (`Present1`, `swap_buffers_with_damage`); na web e no Skia CPU, a imagem já fica
    guardada; um modo de teste pinta por parte e por inteiro e compara as imagens (a
    galeria e os exemplos passam por ele);
  - 3, desenho gravado por controle (`SkPicture` no Skia, `ID2D1CommandList` no Direct2D,
    comandos próprios na web), regravado só quando muda, com a revisão de todos os controles
    para avisarem quando o desenho deles mudou (cursor, hover, animação, relógio);
  - 4, camadas no estilo Flutter: automáticas nas áreas de rolagem e nos controles animando
    `translate` ou `opacity` (rolar e animar movem a camada, sem repintar) e a propriedade
    `cache_layer` para o programador marcar um controle pesado (gráfico, mapa);
  - sem compositor numa thread separada: no aylis a thread da tela raramente fica ocupada
    (backend remoto em outro processo; em Rust, o trabalho pesado vai para uma thread).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 45 e 54: o `large_ui` repinta 0,2% (digitar), 0,4% (relógio) e 0,7% (progresso); o resto ao rolar no fim do Memo e o Profiler sem a área estão nos pendentes).
- [x] 6. (Feito em 02/11 em `fdb58ee`, menos a medição, que vai com o caso grande do item 5;
  falta o autor confirmar.) Busca por nome sem percorrer a árvore. `find`, `find_in` e `find_path` percorrem a
  árvore comparando textos; o nome qualificado do handler sai de um `format!` a cada evento;
  os eventos de um componente são uma lista de textos. O aviso de um dataset (`notify`, a
  cada registro que muda, cada carga) percorre todos os componentes do `Ui` duas vezes,
  procurando os `DataSource` e lendo o `data_source` de cada um por texto. Decidido em
  02/11, nesta ordem. Pronto quando:
  - o dataset guarda os seus `DataSource` e cada `DataSource` os controles ligados, como os
    `TDataLink` do Delphi (listas mantidas ao ligar, desligar e liberar), e o aviso percorre
    só elas;
  - há um índice (form, nome) → id mantido em `set_name`, `free`, troca de pai e hot reload,
    usado por `find`, `find_path` e pela resolução de referências (`data_source`);
  - o despacho de eventos não monta texto (`format!`) a cada disparo;
  - em modo debug, uma verificação compara as listas e o índice com a busca antiga, e há
    testes para cada caminho (renomear, mudar de pai, hot reload, liberar no meio de um
    evento);
  - há medição antes e depois, no caso grande do item 5.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 45: achar um form em 0,0007 ms entre 6.799 controles).
- [x] 7. (Feito em 02/11 em `86e35f6`; falta o autor confirmar.) Agrupar os ganchos raros do trait `Component` (60 métodos; prioridade baixa).
  Reavaliado em 02/11: o `TControl` do Delphi também tem dezenas de métodos virtuais, e um
  ancestral único é o modelo familiar; dividir o trait inteiro não compensa. Pronto quando:
  - os ganchos de plataforma (`taskbar`, `tray`, `native_view`, `take_native_commands`,
    `connection`, `files_dropped`) ficam num `PlatformHooks` e os do designer
    (`design_grip`, `design_drag`) num `DesignHooks`, que o componente devolve se usar (como
    o `control()` devolve o `ControlBase`);
  - o resto do `Component` fica como está;
  - o guia de componentes explica quando chamar a base (o `inherited`).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 46: a seção 7 de `docs/criar-componente.md`).
- [x] 8. Dados para software de negócio. Pronto quando:
  - 8.1 (Feito em 02/11 em `a47f810`; nos clientes, também em 02/11: `FieldAsCurrency` e
    `FieldAsBCD` no Pascal, `FieldDecimal` com `decimal` no C#, `fieldDecimal` com `BigDecimal`
    no Java, `field_decimal` com `Decimal` no Python, `FieldDecimal` com `*big.Rat` no Go; no
    TypeScript e no C++, sem decimal na linguagem, o texto exato (`fieldDecimal`,
    `field_decimal`); falta o autor confirmar.) `Decimal`
    (decidido em 02/11). Hoje as expressões já somam, subtraem e multiplicam
    exato e o protocolo já manda o decimal como texto (fica assim); falta:
    - nas expressões (campos calculados, filtros), a divisão continua decimal (hoje vira
      `Double`: 0,30 / 0,10 = 2,9999999999999996): com as casas do campo da expressão ou,
      sem campo, as dos valores mais 4 (como o `Currency`); `round(x, casas, modo)` para
      escolher; misturar com `Float` dá `Float`, documentado;
    - a propriedade `rounding` no Field (`half_up`, o padrão de hoje; `half_even`, o
      bancário; `truncate`; `up`), ao gravar no campo (da tela, do backend ou de uma
      expressão) e na exibição;
    - no Rust, os operadores `+ - * /` no `Decimal` e `div(b, casas, modo)`;
    - estouro é erro (o campo mostra; no Rust, a operação devolve o erro), nunca valor
      saturado;
    - nos clientes, os campos devolvem o tipo exato da linguagem (`AsCurrency`/`AsBCD` no
      Pascal, `decimal` no C#, `BigDecimal` no Java, `Decimal` no Python...), com o texto
      escondido;
  - 8.2 (Feito em 02/11 em `8479d89`, `ce37522` e `e9d7e77`, e o campo "Fuso horário" nas
    opções do projeto na IDE; falta o autor confirmar.) Data e hora com fuso (decidido
    em 02/11). Defeitos achados: o `today()` das
    expressões usa UTC (em Brasília, depois das 21h já é o dia seguinte); a leitura ISO
    recusa `Z` e `±hh:mm` e, com fração de segundo, ignora o `Z` (o `toISOString` do
    JavaScript vira hora local 3 h adiantada, sem aviso). Pronto quando:
    - `today()` dá a data local da tela, e há `now()`;
    - a leitura aceita `Z` e `±hh:mm`; num `DateTime`, converte para o horário local da
      tela; nunca ignora o fuso;
    - dois tipos, como no FireDAC: `DateTime` (hora de parede, sem fuso, como o
      `TDateTime`; fica como hoje) e `Timestamp` (um instante, guardado em UTC, mostrado no
      fuso de quem vê, como o `ftTimeStampOffset`; no protocolo vai com o fuso);
    - o "local" é o do aparelho; a propriedade `time_zone` do projeto fixa um fuso com nome
      (`America/Sao_Paulo`), com a tabela de fusos (IANA) só quando usada;
    - o guia tem a regra: aconteceu num momento → `Timestamp`; data ou hora do calendário →
      `DateTime`;
  - 8.3 `apply_updates` (decidido em 02/11, o mesmo modelo do 3.5). Hoje o `on_apply` (Rust)
    roda na thread da tela e congela a gravação demorada, e o `on_apply_updates` (remoto)
    volta "ok" na hora, antes de o banco gravar. Pronto quando:
    - nas linguagens remotas, o `on_apply_updates` é evento-pergunta: a resposta são os
      resultados por registro; a tela espera como no 3.5; o `ApplyUpdates` do backend só
      volta com tudo gravado e devolve o número de erros (`ApplyUpdates(MaxErros): Integer`,
      como no Delphi); exceção no handler recusa a gravação inteira; os erros por registro
      vão para o `on_reconcile_error`; o comando `reconcile` continua aceito por um tempo,
      para compatibilidade;
    - em Rust, o `on_apply` síncrono continua, e entra uma variante para gravação demorada:
      o handler devolve "pendente", grava numa thread própria e entrega os resultados com
      `post`; enquanto isso, o dataset fica "aplicando" (sem edição, e os controles DB*
      mostram que está gravando).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 47: 0,30 / 0,10 = 3 exato e o `today()` local (12 testes do `aylis-data`)).
- [x] 10. O mesmo texto nos três renderizadores. Skia, DirectWrite e o navegador medem e
  quebram linha cada um a seu modo, e o layout depende disso (`auto_size`); a maior causa é
  a fonte do sistema diferente em cada plataforma (como no FMX). Decidido em 02/11. Pronto
  quando:
  - agora: o projeto novo vem com uma fonte embutida como padrão, e a IDE avisa quando a
    tela usa a fonte do sistema num projeto para mais de uma plataforma; o designer mostra a
    tela no renderizador da plataforma escolhida; um teste compara medidas e quebras de uma
    lista de textos (acentos, árabe, emoji, japonês) nos três renderizadores, com
    tolerância; o guia pede folga nos textos e `auto_size` onde o texto varia;
  - depois: o modo de texto por plataforma no `.aylisproj`, como o `render`
    (`"text": {"windows": "core", "web": "layout", ...}`): `native` (como hoje: cada
    plataforma mede e desenha), `layout` (o núcleo decide quebras e tamanhos, a plataforma
    desenha cada linha) e `core` (o núcleo mede e desenha letra por letra; sem custo no
    desktop e no celular, com o motor e as fontes no download da web); padrão nos projetos
    novos: `core` no desktop e no celular e `layout` na web; os existentes ficam em
    `native` até o autor trocar;
  - sem opção por form (decidido em 02/11: mediria o mesmo texto de dois jeitos na mesma
    janela, e o custo da web é do app inteiro);
  - o teste de medidas e o de comparar imagens (item 5) rodam em cada modo de cada
    renderizador.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 49 e 96: projeto novo com a Inter e o texto por plataforma; o `harfrust` com o texto igual e o teste visual passando (ajustes do catálogo de fontes nos pendentes)).
- [x] 11. Escopo, testes e CI. Pronto quando:
  - (11.1, descartado em 02/11: o aylis é usado pela IDE, e o projeto gerado já liga só as
    features e registra só as classes que os forms usam; as features padrão do crate `aylis`
    ficam como estão);
  - 11.2 (aprovado em 02/11: "hoje a compilação demora muito"): o `aylis-controls` (cerca de
    49 mil linhas) fica dividido por família (básicos, editores, gráficos, financeiro,
    industrial...), compilando em paralelo; medir o tempo antes e depois;
  - 11.3 (aprovado em 02/11, com foco na IDE: "é quase impossível testar tudo
    visualmente"): a CI (Windows e Linux com Xvfb) grava imagens e compara com as de
    referência: as telas da IDE (`aylis-ide --png`, a galeria com `--gallery` e as gravações
    de uso com `--recording`), os exemplos e a galeria de componentes; inclui o teste de
    pintar por parte × por inteiro (item 5) e o de medidas de texto (item 10);
  - 11.4 (aprovado em 02/11: "única forma de garantir que foi testado"): a cobertura é
    medida por crate (`cargo llvm-cov`) e aparece na CI, sem meta obrigatória no começo;
  - 11.5 (não se aplica por enquanto, 02/11): várias sessões trabalham ao mesmo tempo na
    mesma árvore, então não há commit por assunto; a sessão que faz o commit roda os testes
    e, se algo falhar, relata ao autor, que encaminha à sessão responsável.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 50: `cargo test -p aylis-ide --test visual` 4/4).
- [x] 12. Código gerado separado do código do usuário. Pronto quando:
  - o registro dos handlers e o acesso tipado ficam num arquivo só gerado, e o arquivo do
    usuário nunca é reescrito (sem blocos marcados);
  - o gerado é pequeno (por classe, como no item 1);
  - há o tempo de compilação medido com 100 forms;
  - está decidido se todo projeto mantém as 8 linguagens ou só as escolhidas;
  - (pedido em 02/11: "muita dificuldade em entender o que é o aylis que gera e o que é
    programação própria da tela") fica claro quem é dono de cada arquivo, em três partes:
    - no disco, uma pasta por dono: o código do usuário em `src/` (as units dos forms e as
      dele) e o programa (`app.lpr`) na raiz da linguagem; o que o aylis refaz a cada
      "Gerar código" em `generated/` (o acesso tipado, o registro dos forms), que pode ir no
      `.gitignore`;
    - o cliente do protocolo (`aylis.pas` e os equivalentes) sai do projeto: vem da
      instalação do aylis (`-Fu` e semelhantes, achado por `aylis_root`), como a VCL no
      Delphi; resolve o projeto antigo preso a uma cópia velha (sem o `Post`); quem compila
      fora da IDE precisa do caminho, e a IDE migra o projeto antigo ao abrir;
    - nas units de form, com o modelo do item 2, somem os blocos de registro: a IDE só
      mantém a seção `published` (componentes e métodos ligados no form), como no Delphi;
      em Rust e C#, o gerado fica num arquivo à parte, sem seção gerenciada;
    - na IDE, a árvore de fontes agrupa por dono: Forms (cada form com a unit e o JSON),
      Units, Programa e "Gerado pelo aylis" (recolhido, com cadeado); o gerado abre somente
      leitura com uma faixa explicando; a seção gerenciada aparece sombreada e avisa ao
      digitar; o cliente do aylis não aparece.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 40: "Gerado pelo aylis (somente leitura)" separado na árvore do projeto).
- [x] 13. (Feito em 02/11: o hello em `e69d69a`, a compactação em `c7713fb`, os formatos
  estritos e o `aylis-migrate` em `7ca6b55`; a medição do protocolo fica com a do item 5;
  falta o autor confirmar.) Evolução do formato e do protocolo. Hoje o form tem versão (`"aylis": 1`) mas não
  há migração, e uma propriedade renomeada faz o form antigo não abrir. Decidido em 02/11
  ("não quero ficar arrastando coisas antigas de uma versão pra outra"): versões
  incompatíveis não abrem, e um migrador à parte converte. Pronto quando:
  - sem nomes antigos, sem propriedades obsoletas e sem "Ignorar" no núcleo, na IDE e no
    runtime;
  - o número de formato (o `"aylis"` dos forms, o `"aylis_project"` do projeto) só sobe numa
    mudança incompatível; a IDE e o runtime só abrem o próprio formato e dizem qual
    migrador rodar;
  - o `aylis-migrate.exe` (crate próprio, no instalador; exceção deliberada à regra de
    subcomando do `aylis-tool`, para a lógica antiga ficar fora do `aylis-tool` e da IDE)
    converte forms e projeto em cadeia (1→2→3), refaz o código gerado, faz uma cópia (ZIP)
    antes e gera um relatório do código do usuário com arquivo e linha de cada uso a trocar
    (troca automática só dos casos exatos, com confirmação);
  - a IDE detecta o formato antigo, não abre, e oferece rodar o migrador;
  - o protocolo começa com um "olá" (versão do protocolo e capacidades) e só aceita a mesma
    versão; a tela antiga recebe "há uma versão nova do aplicativo" com o link da loja (o
    caso do celular pela `--bridge`, quando o servidor é atualizado antes do app);
  - o protocolo continua em JSON (decidido em 02/11; Protobuf descartado: sem suporte
    oficial para Free Pascal e Delphi, o protocolo é dinâmico e cairia no `Struct`, e a
    depuração e as gravações perderiam a leitura); medir o tempo do protocolo numa tela
    comum e numa carga grande (os municípios do IBGE, 100 mil registros) e, se só a carga
    de dados pesar, um caminho binário colunar só para ela;
  - compactação no WebSocket (`--connect`, `--bridge`, a tela na web), anotado em 02/11:
    hoje não há nenhuma; o `tungstenite` não tem a `permessage-deflate` (nem o 0.30), então
    a compactação é do aylis: a tela anuncia `deflate` na resposta do `hello`, e a ponte
    (`--bridge`), ao ver essa resposta, manda compactadas (quadro binário com deflate) as
    mensagens acima de 1 KB para aquela tela; a tela (o app ligado ao servidor e a casca do
    navegador) descompacta; no stdio e no `--connect`, não (os clientes das linguagens não
    precisam de compactação);
  - (corrigido em 02/11: o limite de 1 MB por mensagem, `MAX_MESSAGE`, só vale para o que a
    tela manda à ponte, no `accept`; as cargas que vêm do backend não passam por ele, e os
    clientes não precisam dividi-las.)
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 51: o projeto de formato 0 não abre e a IDE oferece o `aylis-migrate`).
- [x] 14. (Feito em 02/11 em `7e7a802`; o `PostTo(form, trabalho)` nos 7 clientes remotos, que
  descarta o trabalho se o form foi liberado (a tela avisa com `__freed`), também em 02/11;
  falta o autor confirmar.) Trabalho em segundo plano
  (decidido em 02/11). O `post` do Rust já existe (o
  `TThread.Queue`), mas roda mesmo com o form dono já fechado, e na web o Rust não tem
  threads (só `async`). Pronto quando:
  - `post_to(dono, ...)`: ligado a um componente ou form, descartado se o dono foi liberado;
    com o item 2, o trabalho recebe a instância do form; o mesmo no `Post` dos clientes
    remotos (o form como dono);
  - `ui.spawn_local(dono, async ...)` na thread da tela, cancelado quando o dono é liberado,
    com versões `async` dos serviços (HTTP, arquivos, permissões, impressão); é o caminho na
    web e para quem usa bibliotecas `async` (`sqlx`);
  - no desktop e no celular, o padrão continua thread + `post_to`, como no Delphi;
  - `aylis::background(dono, trabalho, fim)` é o jeito documentado em Rust (acrescentado em
    02/11: `std::thread::spawn` compila para a web mas falha quando roda, e na web a lógica
    Rust vai para o navegador): no desktop e no celular, uma thread e o `post_to`; na web, o
    trabalho roda na thread da tela, em seguida (funciona, mas a tela espera num cálculo
    pesado; espera de rede vai pelo `spawn_local`); um Web Worker só se aparecer um caso
    real; a verificação antes de compilar (1b) avisa de `std::thread::spawn` num projeto
    que publica para a web; nas linguagens remotas nada muda (o backend roda no servidor);
  - o guia diz quando usar cada um e que o `async` é para esperar, não para calcular (um
    cálculo pesado numa tarefa `async` trava a tela).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 52: a carga do GradeGigante em threads e fechar no meio sem erro (a memória depois do Parar está nos pendentes)).
- [x] 15. (Feitos em 02/11: 15.1 em `7f24ad6`, 15.2 em `fdb58ee`, 15.3 em `31f2c1b` (o nome já
  ia junto na cópia; o defeito era a cor trocada direto), 15.4 em `2ea27f1`; falta o autor
  confirmar.) Menores (decididos em 02/11):
  - 15.1 passar do limite de profundidade da árvore (`MAX_TREE_DEPTH`, 256) ou de eventos
    aninhados (`MAX_EVENT_NESTING`, 64) dá erro sempre, em modo dev e em produção (hoje é um
    aviso e o evento não chega), pela caixa do item 9, com o caminho ("evento recursivo:
    frmPedido.edtQtd.on_change → ... 64 níveis"), e passa pelo evento de exceção da
    aplicação; os dois limites são configuráveis no `.aylisproj`, e a IDE avisa acima de um
    máximo seguro para a pilha;
  - 15.2 as propriedades do tipo componente (`data_source`, `popup_menu`...) que apontam
    para um componente liberado se limpam sozinhas, pelo índice de ligações do item 6; o
    `notification` fica para os casos especiais;
  - 15.3 o nome da cor (`success`, `brand`) fica no próprio componente, junto com a cor, e
    vai junto ao clonar, copiar e colar e desfazer; teste: colar um controle com cor por
    nome e trocar o tema;
  - 15.4 um roteiro de teste com leitor de tela (Narrador e NVDA no Windows, TalkBack no
    Android, VoiceOver no Mac e no iPhone) e IME (japonês, chinês) em
    [docs/testes-em-dispositivos.md](docs/testes-em-dispositivos.md): ler um form, navegar
    com Tab, preencher um Edit, a grade.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 53: o limite de recursão (teste do `aylis-core`) e a cor por nome colada mudando com o tema).
- [x] 17. Consulta grande no servidor. A busca sob demanda já existe (`on_fetch`, `has_more`,
  o grid pede mais ao rolar), mas ordenar pelo título do grid (`DBGrid::sort_by`), filtrar e
  localizar só usam o que já foi carregado, mesmo com `has_more`: o resultado sai errado sem
  aviso. Pronto quando, com dados parciais, ordem, filtro e localizar viram um pedido ao
  backend (ou a tela avisa), houver um limite de memória para o dataset e o total de registros
  puder vir do backend (para a barra de rolagem).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 62: `cargo test -p aylis-dbcontrols --test server_query` 3/3 (falta um exemplo e a documentação, nos pendentes)).
- [x] 20. Busca e referências no projeto inteiro. (Corrigido em 04/11: a busca de texto no projeto
  já existe, `projectsearch.rs`, Ctrl+Shift+F, também nos forms.) Falta o "onde este
  campo/componente/handler é usado", por nome (`cdsCliente.Saldo` em forms, expressões e código). Pronto quando a IDE tiver busca no projeto (texto e por nome de componente,
  propriedade e campo) e uma lista de usos com clique que leva ao lugar. Aprovado pelo autor em
  04/11.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 60: "Onde é usado" no dataset, no Field e no handler (clicar no resultado não abre: nos pendentes)).
- [x] 22. Forms em JSON bons para merge. O JSON já guarda a ordem (`preserve_order`), mas não há
  teste de que abrir e salvar sem mudança dá os mesmos bytes, nem de que mudar uma propriedade
  muda uma linha só. Com várias pessoas no mesmo form, a diferença precisa ser legível.
  Pronto quando a ordem e o formato do que a IDE salva forem fixos e testados (abrir e salvar
  sem mudança = arquivo igual, em todos os exemplos) e o guia disser como resolver um conflito
  de form. Aprovado pelo autor em 04/11 ("muito necessário").
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 61: só a linha da propriedade muda no form (o `modified` no `.aylisproj` está nos pendentes)).
- [x] 23. Relatório para ERP. Já existe o relatório de bandas (`aylis-dbcontrols/src/report.rs`:
  grupos, totais, visualização, PDF, impressora). Falta o que ERP pede: subrelatório
  (mestre-detalhe), código de barras e QR Code (nota fiscal, etiqueta, boleto), impressora
  térmica (ESC/POS, cupom), exportar planilha/CSV e relatório grande sem todas as páginas na
  memória. Pronto quando cada um existir, ou estiver decidido que fica de fora.
  Aprovado pelo autor em 04/11, com a exportação num componente por formato (CSV, XLSX...),
  para o código de um formato só entrar no app que usa aquele componente.
  Decidido pelo autor em 05/11:
  - códigos de barras: os mais comuns do mercado, com um jeito fácil de acrescentar outro
    formato depois;
  - planilha: uma biblioteca pronta, desacoplada do aylis (trocar de biblioteca sem mexer em
    quem usa);
  - impressora térmica (ESC/POS): sem suporte por enquanto;
  - ideia do autor, para estes e para tudo que dependa de projeto de terceiros: o componente
    se liga a um componente provedor, como o dataset ao `DataSource` (o `Export` a um provedor
    de XLSX; o código de barras a um provedor de formatos), para ficar fácil criar outro
    provedor depois. Proposta em discussão (resposta de 05/11), no item 25.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 63 a 65: códigos de barras e QR lidos pelo celular, um por registro (teste), CSV e XLSX no Excel, o PDF grande com "Página X de Y" certo (a memória que ainda sobe está nos pendentes)).
- [x] 25. Componentes ligados a provedores (ideia do autor, 05/11). O que depende de projeto de
  terceiros (planilha, códigos de barras, e daqui em diante PDF, mapas,
  reconhecimento de texto...) vira um par: o componente que o app usa (o que faz, com as
  propriedades e os eventos de sempre) e um componente provedor (quem faz, com a biblioteca),
  ligado por uma propriedade do tipo componente, como o `data_source`. Trocar de biblioteca é
  trocar o provedor; um formato novo é um provedor novo; o app só leva o código dos provedores
  que os forms usam (o `aylis-build` já faz isso por classe). Em discussão: a forma (o trait do
  provedor por assunto no núcleo; o componente sem provedor ligado usa o padrão ou avisa?; o
  provedor numa paleta própria; quais componentes de hoje passam a ter provedor).
  Decidido pelo autor em 05/11, como proposto: um contrato (trait) por assunto no núcleo
  (`XlsxProvider`, `BarcodeProvider`...); o componente aponta o provedor por uma propriedade
  `provider`; sem provedor ligado, vale um provedor padrão do aylis (não é erro); os provedores
  numa aba própria da paleta ("Provedores"). Passam a ter provedor: a planilha (XLSX), o código
  de barras, o PDF do relatório, a exportação CSV e o mapa. Banco local não: acesso a banco é
  só do app host (backend), não é componente do aylis.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 63, 64 e 66: o Barcode, o Export com o XlsxProvider e o CsvProvider, o MapView com outro servidor (OpenTopoMap) e a atribuição dele).
- [x] 32. O app como servidor MCP (pedido do autor em 05/11: "um componente MCP que possa dar a
  LLM a capacidade de usar o app host sem depender da tela de forma automatizada"). Feito em
  05/11 (sem commit; falta o autor confirmar): `McpServer` e `McpTool` (paleta System, em
  `aylis_remote::mcp`, feature `mcp` do `aylis`). O servidor liga com `active` (nunca no
  designer), HTTP em 127.0.0.1 com token (vazio: gerado) e checagem de `Origin`; o método
  `client_command` dá a linha do `claude mcp add`. Cada `McpTool` filha é uma ferramenta do
  app (`params` em JSON Schema ou na forma curta `cliente: string, obs?: string`), respondida
  pelo `on_execute` (evento que pergunta: vale para backends em outras linguagens). Com
  `expose_ui`, as ferramentas de tela (`ui_forms`, `ui_tree`, `ui_get`, `ui_items`, `ui_rows`,
  `ui_data`, `ui_set`, `ui_click`, `ui_click_item`, `ui_type`, `ui_key`, `ui_invoke`) pelo
  protocolo remoto; `read_only` deixa só as que leem, `forms` limita os forms, e o próprio
  servidor nunca é alcançável. `on_call` vê (e pode recusar) cada chamada. O transporte HTTP é
  o mesmo do MCP da IDE (`aylis_remote::mcp::http`). Guia em `docs/mcp.md`; testes em
  `crates/aylis-remote/tests/mcp.rs`. Falta conferir com um cliente de verdade (Claude Code
  ligado a um app aberto).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 70: o Claude Code viu as 12 ferramentas, leu a árvore e clicou no app; sem o token, 401; de outro site, 403).
- [x] 39. Diretrizes do projeto no `CLAUDE.md` (pedido do autor em 07/11: "explicando a forma
  correta de trabalhar (segurança, definições de arquitetura, componentização, plataformas
  suportadas, requisito de desempenho) para ninguém desenvolver algo fora das diretrizes").
  Feito em 07/11 (sem commit; falta o autor confirmar): a seção "Diretrizes do projeto", tirada
  das decisões já registradas (histórico, revisão de segurança de 31/10, análise de
  arquitetura de 02/11), com a regra de que sair delas é pergunta ao autor.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 68: conferidas; dois ajustes do texto nos pendentes (as expressões e tirar o Delphi)).
- [x] 40. Voltar e avançar pelos botões extras do mouse (pedido do autor em 07/11: "atalho de
  voltar nos botões extras do mouse"). Feito em 07/11 (sem commit; falta o autor confirmar):
  - núcleo: o evento `on_navigate` do form (texto `back`/`forward`), disparado pelos botões
    extras do mouse e por Alt+← / Alt+→ (`Ui::navigate`), do controle com foco para cima até
    quem trata (um form embutido antes da janela). Sem quem trate, o botão e a tecla seguem o
    caminho normal;
  - navegador: os botões extras sobre o app não tiram mais da página;
  - documentação da Landing: o histórico das páginas abertas (voltar da primeira volta ao
    site). Teste `os_botoes_extras_do_mouse_voltam_e_avancam_pelas_paginas`;
  - editor de código da IDE: os lugares por onde passou (abrir arquivo, ir para a declaração,
    achados), como no VS Code. Teste `code_navigation.rs`.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 55: pelo mouse e por Alt+setas, e a Landing no Edge não sai da página).
- [x] 41. Mãozinha com Ctrl sobre um nome no editor de código (pedido do autor em 07/11: "quando
  eu pressiono Ctrl e passo o mouse sobre uma variável, função, tipo ele deveria aparecer a
  mãozinha para indicar que é clicável"). Feito em 07/11 (sem commit; falta o autor confirmar):
  no `CodeEditor` (o `Memo` com `code_editing`), com Ctrl sobre um nome o cursor vira a
  mãozinha e o nome fica sublinhado na cor de destaque, como no VS Code; aparece também ao
  apertar o Ctrl com o mouse parado e some ao soltar, ao sair do nome ou do editor. Teste
  `ctrl_over_a_name_shows_the_hand_cursor`.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 56 (a aparência da dica do código está nos pendentes)).
- [x] 42. A dica (`hint`) aparecia rápido demais (pedido do autor em 07/11: "deve ter um delay";
  depois "os 500 é pouco, aumente para 1 s"). Feito em 07/11 (sem commit; falta o autor
  confirmar): o tempo contava desde a entrada no controle, então quem atravessava um botão já
  via o balão; agora é 1 s com o mouse parado em cima (`HINT_DELAY`), e o tempo recomeça
  enquanto o mouse anda (um tremido de até 3 px não conta). Teste
  `hints_wait_for_the_mouse_to_rest_one_second`.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 57: o atraso de 1 s (dar dica aos botões de texto da barra está nos pendentes)).
- [x] 43. O servidor de linguagem demora para ficar pronto e a IDE não dizia (pedido do autor em
  07/11: "se ele [é] lento tem que ter um progresso ou informar na IDE que ele ainda não está
  pronto"). Medido na Calculadora (Rust): o `initialize` responde em 0,1 s, mas o rust-analyzer
  ainda busca os metadados, monta o grafo dos crates, compila os build scripts e indexa — de
  20 s (com cache) a 50 s (sem) — e depois roda o `cargo check`; a IDE mostrava "rust-analyzer"
  (pronto) já no 0,1 s, e um pedido durante a abertura dizia "instale o servidor". Feito em
  07/11 (sem commit; falta o autor confirmar): o cliente lê o `$/progress` do LSP (`Client::work`,
  `lsp::work_text`), e o rótulo do servidor na janela de código mostra a fase e a porcentagem
  ("rust-analyzer: Indexing 48%…"), com o detalhe na dica; durante a abertura, os pedidos
  dizem que ele ainda está abrindo; declaração ou referência não encontrada enquanto ele
  indexa avisa isso. Vale para todo servidor que manda o `$/progress` (rust-analyzer, clangd,
  gopls, jdtls...). Testes `server_progress_shows_what_is_running_and_how_much` e, com o
  servidor de verdade, `the_rust_server_reports_its_progress_until_ready` (ignorado: demora).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 58: o rodapé mostra as fases do servidor (a dica do rodapé sem detalhe está nos pendentes; o aviso do Ctrl+clique antes de pronto não deu para pegar)).
- [x] 45. O agente edita o projeto pela IDE, não gravando os arquivos (pedido do autor em 07/11:
  "deveria impedir edições diretamente em arquivo para não bagunçar a IDE"). A IDE guarda os
  forms e o código abertos na memória e não vê a mudança no disco: a edição do agente sumia ao
  salvar. Decidido pelo autor: as partes 1 e 2; a 3 (a IDE conferir a data dos arquivos
  abertos e recarregar) ficou de fora, pelo receio de deixar a IDE lenta (estimado: só as abas
  abertas, ao ganhar o foco, uns 2 ms). Feito em 07/11 (sem commit; falta o autor confirmar):
  - 1, editar pelo MCP: `read_code` (o texto do editor, com o que não foi salvo, ou um trecho
    por linhas) e `edit_code` (troca um trecho exato pelo editor da IDE: o Ctrl+Z desfaz, o
    servidor de linguagem fica sabendo, a aba fica alterada; recusa o ambíguo, o ausente, o
    código gerado e os forms, que mudam por `set_property`/`set_layout`); o `save` com `all`
    também salva o código. A instrução do MCP diz para não gravar os arquivos do projeto
    direto. Teste `an_agent_edits_the_code_through_the_ide_editor`;
  - 2, barrar no agente: com o MCP ligado e um projeto aberto, a IDE grava no
    `.claude/settings.local.json` do projeto (o local, fora do git; o resto do arquivo fica) um
    gancho `PreToolUse` do Claude Code para `Write`, `Edit`, `MultiEdit` e `NotebookEdit`, que
    roda `aylis-ide --guard-edit` (`edit_guard`). O guarda olha as IDEs abertas e recusa
    (código 2, com o motivo e as ferramentas a usar) gravar nos forms, nos temas, no
    `.aylisproj` e no código de um projeto aberto numa IDE com o MCP ligado; assets, docs e
    `target` ficam livres, e com o projeto fechado tudo fica livre. O gancho vale para o
    Claude Code aberto na pasta do projeto (não para uma sessão na raiz do repositório do
    Aylis). Testes do módulo e conferido pelo executável (recusa o código, deixa os assets).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 72: o gancho, a recusa com o projeto aberto, a liberação com ele fechado, o `edit_code` na aba sem ir ao disco e o Ctrl+Z (o cursor depois do Ctrl+Z está nos pendentes)).
- [x] 46. Selecionar componente no designer lento com a Landing aberta (relatado pelo autor em
  07/11, conferindo o item 1 do roteiro). A gravação de uso mostrou ~420 ms entre apertar e
  soltar o botão em cada clique, com a IDE de debug (a de release foi compilada depois). Medido
  com o `landing_selection_cost` (o `perf` do `aylis-ide`; a Landing tem 231 componentes): por
  seleção, 473 ms em debug e 8 ms em release, dos quais a árvore Estrutura refeita inteira 300 ms
  em debug (3 ms em release) e o Object Inspector 143 ms (3,5 ms). Corrigido: a Estrutura
  reaproveita as linhas quando a árvore não mudou e só troca o destaque (`set_row_selected`).
  Agora 168 ms em debug e 5,5 ms em release. O `opt-level = 1` no `aylis-ide` em debug só levava
  a 156 ms e deixava a compilação do crate em 1 min 44 s: ficou de fora. Para conferir a
  velocidade, a IDE em release (`cargo run --release -p aylis-ide`). Sem commit; falta o autor
  confirmar.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 73).
- [x] 48. Os grupos do Object Inspector com letra pequena e sem recolher (pedido do autor em
  07/11: "o texto da fonte deles está muito pequeno e recolhível igual a lista de
  componentes"). Feito em 07/11 (sem commit; falta o autor confirmar): o título do grupo em 13
  pontos e negrito à esquerda, com a seta à direita como na paleta; clicar recolhe e abre, e vale para todos os componentes
  (`inspector_closed`); com busca, os recolhidos abrem. A chave que evita refazer o Inspector
  passou a contar os grupos recolhidos. Teste `the_inspector_groups_collapse_and_open`; a
  referência `designer-windows.png` regravada (a `designer-linux.png` fica para a próxima
  rodada no Linux).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 74 (a busca que não abre os recolhidos está nos pendentes)).
- [x] 50. Opções do projeto (relatado pelo autor em 07/11): na Geral, os combos encostavam na
  barra de rolagem; em Fontes, uma barra sem nada para rolar; em Traduções, a lista sem borda
  com a barra solta. Medido: a página embutida recebia a altura pedida pelo diálogo para a
  janela própria, que sobrava (Fontes) ou faltava (o fim da Geral e da Memória ficava cortado,
  sem rolar até lá). Feito em 07/11 (sem commit; falta o autor confirmar): a página ocupa a área
  visível e cresce só o que o conteúdo precisa (`fit_page`); 18 de recuo para a barra; a lista
  das Traduções numa moldura com a cor de borda do tema. Conferido em imagem e no teste
  `the_project_options_pages_scroll_only_when_needed_and_to_the_end`.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 81 e 82).
- [x] 52. O F12 parava de alternar Design ↔ Código depois de uma ida e volta (relatado pelo
  autor em 07/11). Causa: ao voltar ao Design, o foco ficava no editor de código escondido, e o
  atalho, procurado a partir do foco, caía no "Ir para o Design" do menu do código (que não faz
  nada no Design) em vez do "Abrir código do form" da IDE. Feito em 07/11 (sem commit; falta o
  autor confirmar; compilado em debug a pedido do autor, sem rodar os testes): ao voltar ao
  Design (F12, botão Design ou a aba de um form sem código), o foco vai para o designer.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 75).
- [x] 53. O marcador (Ctrl+B) cobria o número da linha no editor de código (relatado pelo autor
  em 07/11). Feito em 07/11 (sem commit; falta o autor confirmar; sem rodar os testes, a pedido
  do autor): a calha do editor de código ganhou uma coluna de 16 à esquerda dos números para o
  marcador. As imagens de referência com o editor podem mudar na próxima rodada de testes.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 77 (uma linha com um marcador só está nos pendentes)).
- [x] 54. A dica do código (o tipo e a documentação, do servidor de linguagem) aparecia na hora
  (relatado pelo autor em 07/11). Ela não passava pelo atraso das dicas da tela: o editor pedia
  ao servidor assim que o mouse mudava de palavra. Feito em 07/11 (sem commit; falta o autor
  confirmar; sem rodar os testes, a pedido do autor): o pedido fica agendado pelo mesmo
  `HINT_DELAY` (1 s, agora exportado pelo `aylis-core`) e é cancelado se o mouse sair da palavra.
  Conferir na próxima rodada de testes: a ferramenta `editor_hover` do MCP e os testes que
  pedem a dica passam a esperar esse 1 s.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 78).
- [x] 55. O F12 num fonte sem form levava ao designer de outro form (relatado pelo autor em 07/11,
  no NoOBS: no `app_settings.rs`, o F12 mostrou o `frmMain`). Feito em 07/11 (sem commit; falta
  o autor confirmar; sem rodar os testes, a pedido do autor): num fonte sem form, o F12 fica no
  código e a linha de situação diz "Este fonte não tem form: não há Design para ele"
  (traduzido nos 11 idiomas da IDE).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 76: fica no código e a linha de situação diz que não há form).
- [x] 56. O minimapa ficava vazio na janela de código criada arrastando uma aba (relatado pelo
  autor em 07/11, com o Skia na GPU). O renderizador Skia é um só para todas as janelas, e o
  cache das imagens prontas (`begin_cached`, que o minimapa usa) guardava texturas do contexto
  da janela que as desenhou: a janela nova achava a imagem da outra pela mesma chave e não
  desenhava nada. Feito em 07/11 (sem commit; falta o autor confirmar; sem rodar os testes, a
  pedido do autor): na GPU, a chave leva o contexto da janela (`DirectContext::id`). No Direct2D
  o cache já era por janela.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 79).
- [x] 57. Listas sem borda nas Opções do projeto, com a barra de rolagem solta (pedido do autor
  em 07/11): o catálogo de Fontes e o "quem usa qual tema" da Aparência. Feito em 07/11 (sem
  commit; falta o autor confirmar): `widgets::framed_list` (a lista que rola numa moldura com a
  borda do tema e o recuo para a barra), usada nas duas e nas Traduções. Conferido em imagem.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 82).
- [x] 58. A tela parava ~2,8 s ao trocar de seção nas Opções do projeto (relatado pelo autor em
  07/11, na gravação de uso, com o NoOBS e a IDE em debug). Medido (`project_options_sections_cost`
  no `perf` do `aylis-ide`): sair das Traduções (2557 componentes) levava 2,8 s em debug e 8 ms em
  release. Causa: ao liberar, o núcleo perguntava quem apontava para cada componente que saía, um
  por um, e em debug cada pergunta conferia o índice refazendo a busca completa em todas as
  propriedades do app. Feito em 07/11 (sem commit; falta o autor confirmar; sem rodar a suíte, a
  pedido do autor): uma pergunta só para todos os que saem (`Ui::referrers_of_all`), com uma só
  conferência. Em debug, trocar de seção leva agora de 1 a 43 ms (sair das Traduções, 7 ms;
  Aparência de 48 para 5 ms, Fontes de 177 para 22 ms). Avisar só os componentes que guardam
  referências (o `notification`) foi tentado e desfeito: custava ~1 ms ali.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 80).
- [x] 59. Executar → Profiler… não fazia nada sem o app em execução (relatado pelo autor em
  07/11): só a linha de situação pedia para executar, e passava despercebida. Feito em 07/11
  (sem commit; falta o autor confirmar; sem rodar os testes, a pedido do autor): a janela abre
  sempre; sem o app, diz isso e o botão vira "▶ Executar" (executa sem depurar, já que a janela
  é modal e o F9 da IDE fica bloqueado), e um timer de meio segundo, ligado só enquanto espera,
  liga o Profiler ao app quando ele abre ("● Gravar" de novo). Textos traduzidos nos 11 idiomas.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 87 (a roda invertida no Profiler está nos pendentes)).
- [x] 60. "aylis: evento de frmMain.lyLibrary adiado: um handler do mesmo form está rodando" em
  vermelho a cada execução do NoOBS (relatado pelo autor em 07/11). Era o `on_resize` do
  `lyLibrary`, causado pelo próprio handler ao mexer no layout: esperar o handler terminar é o
  normal, e o aviso de desenvolvimento (item 2) só assustava. Feito em 07/11 (sem commit; falta o
  autor confirmar; sem rodar os testes, a pedido do autor): o `Ui` anota o evento em andamento
  (`running_events`) e o aviso pula os automáticos (`COALESCED_EVENTS`: `on_resize`, `on_timer`,
  `on_scroll`, movimento do mouse, roda...); continua para os do usuário (clique, tecla...).
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 88).
- [x] 61. O título "MENSAGENS" colado nas linhas do painel de mensagens (pedido do autor em
  07/11). Feito em 07/11 (sem commit; falta o autor confirmar): 8 de espaço entre a barra do
  título e a lista.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 90).
- [x] 62. Os títulos dos grupos nas Opções do projeto com a letra pequena (pedido do autor em
  07/11, na Geral: "RENDERIZADOR POR PLATAFORMA"). Feito em 07/11 (sem commit; falta o autor
  confirmar): `widgets::section` nas páginas Geral, Aparência e Fontes, com o título em 13 pontos
  e negrito, na largura do próprio texto, e a explicação ao lado, apagada (o texto até o
  primeiro " (" ou " · "); traduzido inteiro antes de dividir. A `caption` (11, apagada) continua
  nos painéis da IDE. Conferido em imagem, em português e em inglês. Achado no caminho, de antes:
  em inglês a seção "Fontes" sai "Sources" (a mesma palavra para fonte de letra e código-fonte),
  e "EM USO NO PROJETO (n)", "+ Nova fonte nomeada" e "(manter)" ficam em português.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 83).
- [x] 63. As traduções da página Fontes das Opções do projeto (achadas em 07/11, conferindo o item
  62 em inglês; corrigir pedido pelo autor). Feito em 07/11 (sem commit; falta o autor confirmar;
  sem rodar os testes, a pedido do autor):
  - "Fontes" era traduzido como código-fonte ("Sources", "Quelltexte"...), de quando nomeava o
    painel dos fontes, que saiu: agora como fonte de letra nos 11 idiomas;
  - traduzidos "(manter)", "(herdar)", "+ Nova fonte nomeada", o modelo "EM USO NO PROJETO ({}) ·
    ...", os nomes "Fonte do sistema (...)" e "Monoespaçada do sistema (...)" e os modelos dos
    lugares ("{} lugar(es): {}", "tema {}", "{}, {} e mais {}"), cada lugar traduzido antes de
    entrar na frase;
  - achado no caminho: os itens dos combos vêm traduzidos na tela, e a página Fontes comparava o
    texto escolhido com "(a do tema)" e "(manter)" (em inglês, ao dar OK, a fonte do projeto viraria
    a família "(the theme's)"); a Aparência, com "(herdar)". Agora pela posição do item (o primeiro).
  Conferido em imagem, em inglês.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 84 (os textos que ainda ficaram em português estão nos pendentes)).
- [x] 64. O painel de memória (o app compilado com `dev`) abria visível em toda execução (pedido do
  autor em 07/11: "por padrão deve vir oculto"). Feito em 07/11 (sem commit; falta o autor
  confirmar; sem rodar os testes): começa disponível e escondido (`set_memory_panel(Some(false))`),
  e o Ctrl+Shift+M mostra e esconde. O item 16 do roteiro de conferência foi ajustado.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 89).
- [x] 65. O ERP aberto pela IDE (F9) lento (relatado pelo autor em 07/11). Medido: compilar sem
  mudança 0,7 s, gerar o código 0,7 s, verificar 0,9 s e abrir o app 0,2 s, mas cada quadro custava
  415 ms em debug (2 quadros/s), quase tudo na grade do menu (1009 telas). O ERP usa o texto pelo
  núcleo (`text` em `core`, item 10), e o `textcore` formava cada texto de novo a cada medida e a
  cada desenho, montando o plano de formação do `rustybuzz` toda vez (a parte cara, segundo a
  própria documentação dele): 1,7 ms por texto em debug. Feito em 07/11 (sem commit; falta o autor
  confirmar): o plano fica guardado por fonte, direção, escrita e idioma, e as letras formadas de
  cada texto, em unidades da fonte (servem em qualquer tamanho; esvazia ao passar de 4096
  textos). O quadro do ERP foi de 415 para 36 ms na janela (28 quadros/s em debug); a pintura sem
  janela, de 430 para 22 ms. Os testes de texto do `aylis-core` passam; a suíte não rodou.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 86: o menu suave em debug).
- [x] 66. Aplicar as Opções do projeto sem mudar nada regravava o `.aylisproj` (achado em 07/11, na
  suíte antes do commit: o teste das páginas das Opções, que abria a Landing do repositório,
  trocou o tema "Aylis Dark" por "dark" e acrescentou um bloco `text`). Feito em 07/11 (sem
  commit; falta o autor confirmar): a Aparência mantém o tema como estava escrito quando a família
  e o modo não mudam; a Geral só grava `text` se ele já existia ou mudou do padrão; o teste usa uma
  cópia da Landing (o modelo "Landing page") numa pasta temporária; o `Landing.aylisproj` voltou
  ao que era. No caminho, mais combos que liam o texto traduzido do item (em inglês davam o valor
  errado): o modo Claro/Escuro da Aparência, a placa de vídeo e a apresentação da Geral, agora
  pela posição do item.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 84 e 85: o `.aylisproj` do NoOBS idêntico depois de OK sem mudar; o Dark pela Aparência só grava o `theme`).
- [x] 69. O plugin do Aylis para o Claude (pedido do autor em 08/11: "pode fazer o plugin").
  Feito em 08/11 (sem commit; falta o autor confirmar): o catálogo na raiz do repositório
  (`.claude-plugin/marketplace.json`) e o plugin em `plugins/aylis`: o servidor MCP (`aylis-tool
  mcp`, pelo PATH), a skill `aylis` (como trabalhar com o Aylis pela IDE) e os comandos
  `/aylis:build`, `/aylis:run` e `/aylis:open` (skills só de barra), com o README. O instalador da
  IDE põe a pasta dela no PATH do usuário e tira ao desinstalar (conferido compilando o trecho no
  NSIS; o instalador inteiro não foi gerado: a auditoria barrou, veja abaixo). Limite: o Chat do
  Claude Desktop ignora servidor MCP local de plugin (só a skill e os comandos entram); lá, o
  servidor continua em Desenvolvedor.
  **Trocado em 08/11, a pedido do autor** ("não quero ter que configurar na mão nada no claude
  desktop... se o MCP sozinho conseguir tratar das instruções corretas, então deixe só ele"): o
  plugin saiu (`.claude-plugin/` e `plugins/`). O que a skill ensinava agora vai nas instruções
  do próprio servidor MCP (`aylis_tools::mcp::INSTRUCTIONS`, mandadas no `initialize` pela IDE e
  pela ponte, também quando a IDE está fechada), e vale em qualquer cliente sem instalar nada
  além do servidor, que a IDE configura (Ferramentas → Opções da IDE → MCP). O PATH do
  instalador ficou (os terminais chamam `aylis-tool` pelo nome).
  Achado no caminho, não desta mudança: a auditoria (`aylis-tool audit`) falha pelo
  RUSTSEC-2026-0436 no `calamine` (o leitor de planilhas do `aylis-docview`: um `.xls`/`.xlsb`
  malicioso derruba ou estoura a memória), corrigido no `calamine` 0.38.0; sem isso, o
  `installer-ide` não gera o instalador.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 94: o Claude Desktop chamou o `session_title` primeiro e mudou pelo `set_property`/`edit_code`).
- [x] 67. O MCP do Aylis no Claude Desktop ficava "Falhou" (relatado pelo autor em 08/11: "a IDE do
  Aylis não está aberta com o servidor MCP ligado"). O Claude Desktop liga a ponte (`aylis-tool
  mcp`) ao abrir; com a IDE fechada, a ponte respondia erro até ao `initialize`, e o cliente dava
  o servidor como falho até reiniciar. Feito em 08/11 (sem commit; falta o autor confirmar): sem a
  IDE, a ponte responde o `initialize` e a lista de ferramentas com a última resposta da IDE
  (guardada em `%LOCALAPPDATA%\aylis\mcp-initialize.json` e `mcp-tools.json`) ou o padrão, o `ping`,
  e uma chamada de ferramenta volta como erro da ferramenta, dizendo para abrir a IDE e ligar o
  MCP; quando a IDE volta a responder, avisa o cliente (`notifications/tools/list_changed`) para
  buscar a lista completa. Conferido pela ponte sem a IDE (initialize, tools/list e tools/call);
  com a IDE abrindo depois, falta conferir no Claude Desktop.
  Conferido e confirmado pelo autor em 08/11/2026 (roteiro 91: depois de reiniciar o Claude
  Desktop, o servidor "aylis" conectado, sem "Falhou"; com a IDE fechada, a primeira conversa
  pediu o Aylis e a IDE abriu sozinha, sem reiniciar o Claude).

## Conferidos pelo autor em 08/11/2026 à noite (o que voltou ao roteiro depois dos ajustes)

- [x] Landing page (feito; falta o autor confirmar): "Novo projeto" ganhou o combo "Modelo"
  (`aylis_project::TEMPLATES`: "Em branco" e "Landing page", que copia form, assets, tema e,
  em Rust, os handlers do `examples/Landing`). Conferida no designer: o título saía apagado
  porque o designer abria em "Form claro" num projeto "Aylis Dark"; agora abre no tema do
  projeto. O "Experimentar" abre a IDE lite em `ide/` (o `AppLink`).
  Conferido em 08/11 (roteiro 38): não aprovado. Decisão do autor: os modelos do Novo projeto
  passam a ser Desktop, Smartphone e Web, em branco e já configurados (tamanho de tela, linguagem,
  plataformas e renderizador; no Smartphone, a moldura do aparelho); o "Landing page" sai (a
  Landing fica só como exemplo). Achado: o projeto criado pelo modelo nem compila no desktop
  ("cannot find `scene3d` in `aylis`"; veja os ajustes de 08/11).
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 38; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).

- [x] 1b. Verificação do projeto antes de compilar (ideia do autor): hoje a IDE salva, gera o
  código e confere o ambiente, mas não confere o projeto. Pronto quando "Executar",
  "Publicar" e um `aylis-tool check` carregam cada form num `Ui` sem janela, com as classes
  registradas, e listam:
  - handler citado no form sem função no código da linguagem (o que mais vale para Python,
    TypeScript e as outras, onde nenhum compilador pega);
  - referência a componente que não resolve (`data_source`, `popup_menu`, `master_source`);
  - ligação a campo que o dataset não tem;
  - asset citado que não existe;
  - nomes em texto no código (`find_path("frmX.y")`, nome de campo) que não existem no form;
  - erro bloqueia a compilação, aviso não; clicar no item abre o form ou a linha.
  Conferido em 08/11 (roteiro 41): não aprovado. Em Python, apagar do código um handler que o form
  cita não deu aviso: a geração recriou a função vazia (`pass`) no disco. Decisão do autor:
  avisar, não recriar — a geração só cria handler para evento ligado agora no form; um handler
  citado e ausente é erro da verificação (a mensagem, o clique leva ao lugar e não executa).
  Achado junto: a geração grava por trás do editor aberto (veja os ajustes de 08/11).

  Feito em 02/11 (falta o autor confirmar): `aylis_project::check` (`project`, e cada
  verificação separada), usado pela IDE antes de Executar e de Publicar (`jobs::verify`: os
  problemas vão para as mensagens; o clique abre o form com o componente selecionado, ou o
  código na linha; com erro, não compila) e pelo `aylis-tool check <projeto.aylisproj> [--web]`
  (sai com erro se houver). Confere: o handler citado no form sem função no código da
  linguagem do projeto; o asset citado (`"assets/..."`) que não existe; o form que não
  carrega (classe desconhecida, propriedade que a classe não tem, referência como
  `data_source` ou `popup_menu` que não resolve: o próprio carregamento recusa, e o
  componente sai do caminho da mensagem); `find_path("frmX.y")` no código que não existe
  (aviso: o código pode criar o componente rodando, como os cartões da Galeria); o
  `panic = "abort"` no Cargo.toml (aviso, do item 9); `std::thread::spawn` publicando para a
  web (erro, do item 14); as fontes do sistema (aviso, do item 10). Os exemplos passam sem
  erro (o teste `check` do `aylis-project`). Depois, também em 02/11: a ligação a um campo que o
  dataset não tem (`data_field`, erro quando o ClientDataSet declara os campos; sem campos
  declarados, passa).
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 41; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).

- [x] Pânico num handler em Rust deixa o form inutilizado (roteiro 48): a caixa de erro aparece e o
  processo segue, mas o form fica marcado como "handler rodando" para sempre; todo evento seguinte dele
  é adiado e nunca roda ("aylis: evento de frmPrincipal.btnContar adiado: um handler do mesmo form está
  rodando", um por clique). A marca tem de ser desfeita também na saída por pânico (um guard com
  `Drop`), com um teste: pânico num handler, depois outro evento do mesmo form roda.
  Feito em 08/11 (falta o autor confirmar): o `with_form` (`aylis-core/src/instances.rs`) segura o
  pânico do handler, devolve a instância ao form, roda os eventos que esperavam e só então deixa o
  pânico seguir para a caixa de erro; teste
  `a_panic_in_a_handler_gives_the_instance_back_to_the_form`.
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 48; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).

- [x] Renomear um componente renomeou também o form (roteiro 59, autor): no Relatórios, `cdsPedidos` →
  `cdsVendas` pelo `name` do Object Inspector (gravação `Aylis-IDE_2026-11-08_15-37-53`, t≈7767)
  trocou também a raiz do `forms/principal.json` e o `main_form` e o nome do form no `.aylisproj`;
  ficaram o form e o dataset com o mesmo nome. O renomear só muda o componente escolhido; um teste.
  Feito em 08/11 (falta o autor confirmar): a causa era o "Substituir marcados" repetido: a pesquisa
  relia o form do disco (ainda com `cdsPedidos`), não achava o componente na memória (já renomeado)
  e caía na raiz do form. Agora um achado cujo componente não está mais no form não troca nada
  (`projectsearch::hit_target`); teste `a_hit_of_a_component_that_is_gone_does_not_touch_the_form`.
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 59; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).

- [x] O ERP cai ao fechar a segunda de duas telas (roteiro 67, autor): pânico no winit, "Unexpected
  GetClientRect failure" (winit 0.30.13, `window.rs:217`), chamado por
  `aylis_platform_winit::paint_stack` ← `App::title` ← `App::size_limits` ← `App::window_event`: o
  Aylis pede o `inner_size` de uma janela já destruída, e fora de handler o pânico derruba o app.
  Reproduzir: abrir o ERP, abrir duas telas pelo menu, fechar uma e depois a outra. O
  `aylis-platform-winit` (`lib.rs`, `monitors.rs`) estava alterado sem commit por outra sessão.
  Feito em 08/11 (falta o autor confirmar): a segunda tela era possuída pela primeira (o dono é o
  form ativo ao abrir), e o Windows destrói a janela possuída junto com a dona; o Aylis seguia com a
  janela morta e o próximo `inner_size` caía. Antes de fechar uma janela, as que ela possui passam
  para a do form principal (`reown_windows_of`, `win32::set_owner`), e uma janela destruída por fora
  fecha o form (`WindowEvent::Destroyed`). Conferido com um exemplo temporário: sem a correção a
  janela de B morria ao fechar A; com ela, continua viva.
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 67; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).

- [x] Fechar ou trocar de projeto fecha tudo (roteiro 38, autor): hoje ficam as abas de código do
  projeto anterior (até um `.py` num projeto Rust), o painel MENSAGENS com a execução anterior e o
  servidor de linguagem anterior rodando (o rust-analyzer do ERP depois de abrir o Relatórios); ao
  criar ou abrir um projeto, a IDE mostra o Design do form principal do novo.
  Feito em 08/11 (falta o autor confirmar): fechar ou abrir outro projeto fecha as abas de código
  (com código não salvo, avisa uma vez, como ao fechar a IDE), limpa as MENSAGENS, para o servidor
  de linguagem e volta ao Design (`actions::leave_project`, `codewindow::close_all_tabs`,
  `intel::stop`, que nunca era chamado).
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 38; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).

- [x] O MessageDialog (roteiro 48, autor): o X do ícone centralizado, sem espaço sobrando, a janela no
  tamanho do texto com altura máxima e rolagem, e Ctrl+C copia o texto.
  Feito em 08/11 (falta o autor confirmar): o × do ícone desenhado no centro do círculo (dois
  traços, sem a letra da fonte), a caixa no tamanho do texto (240 a 520 de largura; acima de 360 de
  altura, rola) e o Ctrl+C copia (título, texto e botões, como o Windows); a cor sobre o círculo vem
  do tema. Testes em `keyboard.rs` e `more_controls.rs`.
  Conferido e aprovado pelo autor em 08/11 à noite (roteiro 48; commit 4254190 e os ajustes
  da própria conferência, ainda sem commit).
