# aylis — Ajustes pendentes

O que apareceu depois do [plano inicial](plano.md) e ainda falta fazer. Quando um item
ficar pronto e o autor confirmar no teste, ele sai daqui: o que foi feito fica no commit, e o
porquê, no [histórico de decisões](historico-decisoes.md) (até 09/11 havia também a lista dos
concluídos, que saiu a pedido do autor). O que só dá para conferir num Mac, num celular ou num
PC com outro hardware fica em [testes-em-dispositivos.md](testes-em-dispositivos.md).

O plano de 01/11/2026 (9 fases) foi feito em `c475abd` e `48081bc`. Abaixo, o que sobrou dele e o que ainda não começou.

Decidido pelo autor em 05/11: o que precisa de hardware ou de outra máquina (o ZenFone, um
leitor NFC, um aparelho Bluetooth, um controle, um Mac, uma placa NVIDIA, outro PC) fica para
depois; os itens feitos que só esperam a confirmação dele são conferidos primeiro, pelo
[roteiro de conferência](roteiro-de-conferencia.md), e saem daqui para os concluídos.

**Relatório do Tabernáculo (11/11/2026)**

O console do Tabernáculo (51 forms, projeções em tela cheia) foi feito no Aylis 0.1.0, e o
relatório dele traz 209 itens (R01 a R04 com reprodução, mais P, J, C, W, L, T, G, I, M, B, O e
F). Conferido contra a versão atual em 11/11: 21 resolvidos, 57 parciais, 129 abertos e 2 que
não se aplicam mais. A análise item a item ficou com o autor (`ANALISE-versao-atual.md`, na
pasta do relatório). Pedido do autor: registrar os defeitos abertos e começar pela R01 e pela
R02.

Feitos em 11/11 (sem commit; roteiros 105 e 106), conferidos com as reproduções do relatório
e com testes:
- R01: o form secundário que nasce em tela cheia, sem dona e por cima, tinha a swap chain
  recusada (`0x887A0001`) e o `run` voltava com erro (na 0.1.0 caía no `dxgi.dll`). No Windows, a
  tela cheia agora é aplicada depois de a janela e o alvo de desenho existirem (`open_window`):
  os 11 casos saem limpos, em tela cheia (o `segunda-outro-monitor` em (-3840, 11) 3840×2160, no
  segundo monitor).
- R02: o `request_close()` num `on_timer` não encerrava (o laço dormia até outra mensagem). O
  `App::quit` põe o `ControlFlow::Poll` antes do `exit()`, e o pedido é conferido de novo depois
  dos relógios: o `run` volta 70 a 120 ms depois do pedido (antes, até passar de 9 s).
- G01: um comentário do `Cargo.toml` que citasse o marcador `aylis:render` abria o bloco do
  renderizador, e a geração apagava as dependências até o fim dele. Agora os marcadores só
  valem em linha própria, a abertura é a última antes do fim, e o que há entre eles tem de ser
  do bloco (`render_span`). Teste `a_comment_naming_the_render_marker_keeps_the_dependencies`.
- G08: depois do `cargo fmt` (`impl FrmX {}` numa linha), o handler novo caía fora do `impl`.
  Agora entra antes da chave que fecha, que vai para a linha de baixo. Teste
  `a_new_handler_goes_inside_an_impl_joined_by_rustfmt`.

Lote 1, os defeitos (feito em 11/11, sem commit; testes `aylis-controls/tests/tabernaculo.rs`,
`aylis-ide/tests/tabernaculo.rs`, `aylis-core/tests/theme_tokens.rs`):
- R04: o arquivo solto só vai a uma zona à vista e habilitada (F01); o `*.*` fica no filtro (o
  "Todos os arquivos"; no macOS o painel já mostra todos) e o recusado dispara `on_reject` (F02);
  o `on_change` do `FileDropZone` e do `FileInput` leva só os que entraram (F04, decisão do autor).
- G06: o `main.rs` gerado usa o nome do pacote com `_`. G07: o `generated/mod.rs` sem o `use
  form_handler` (o caminho inteiro em cada registro). `GENERATOR_VERSION` 10.
- I07: fechar ou trocar de projeto com forms alterados avisa uma vez (a segunda descarta). M05: o
  `close_project` do MCP diz que não fechou, com o motivo.
- M07: o `usages` (MCP e IDE) lê os forms abertos da memória. M08: renomear pelo `set_property`
  faz o mesmo que o Object Inspector (o código e os outros forms).
- C07: `Theme::unknown_color_tokens`, e a verificação do projeto avisa a cor que o tema não tem.
- C04: o `letter_spacing` também com `wrap` e `markup`. C05: o Label soma o `padding` do controle.
- J01: os atalhos de um PopupMenu valem só com ele ligado ao controle com o foco ou a um pai (o
  `IsMenuKey` do VCL).
- T08: o BGRA é pré-multiplicado nos três renderizadores (o contrato do tipo).
- M04: a ponte `aylis-tool mcp` sai quando o processo que a abriu some. M22: o `add_component`
  com `parent` põe nele, não num filho sob o canto.
- I01: a IDE pega o `CARGO_HOME` e o `RUSTUP_HOME` do usuário (registro). I02: a verificação
  mostra o erro do cargo. I03: com o rustup já instalado, o "Corrigir" instala o toolchain por
  ele. I04: a porta do canal ocupada, a IDE abre na próxima livre e avisa o app
  (`AYLIS_DEV_PORT`; no Android, o `adb reverse`).
- W01: o WebView só pega o foco ao nascer se for o primeiro da ordem de tabulação do form.
- W05 e W06: o VideoPlayer e o MediaPlayer abrem em segundo plano (`on_ready`, `on_error`); o
  `play` antes de abrir toca quando ficar pronto.
- B02: o `render_png` refaz o layout até assentar (o que os `on_resize` mudam entra na foto).
- R03: o evento `on_window_created` do Form, com a janela (e o HWND) já existindo.
- M13: os forms e fontes abertos guardam a marca do arquivo; ao voltar o foco e antes de cada
  chamada do MCP, o que mudou por fora recarrega (sem alteração na IDE) ou vira conflito: a IDE
  pergunta; o MCP recusa com o diff até o `resolve_conflict` (`keep`: disk ou ide).
- Não se aplicam: L03 (o encaixe é o do Delphi e do FMX: `Left` depois de `Top` fica com o que
  sobrou; o `MostLeft` pega a altura inteira) e W03 (o wry só cria o WebView2 na hora, na thread
  da janela; como o W02, é do WebView).

Lote 2, janelas e plataforma (feito em 12/11, sem commit; teste
`aylis-controls/tests/tabernaculo_janelas.rs`; o resumo em `docs/tempo-de-execucao.md`, "Forms"):
- Forms: `on_window_created` (R03, do lote 1); `show_owned` (J19); o principal é o `main_form`
  do projeto (J20); `close_action` hide/free e `ui.free_later` (J21, J22); `activate_on_show`
  (J05); `position` nos cantos da área útil (J10); a tela cheia no monitor de `monitor` (J07); o
  modal preso ao form que o abriu (J24); a barra de título clara ou escura pelo tema, também ao
  trocar (J04); `on_minimize`, `on_restore` e `ui.is_minimized` (P06); `frame_rate` por janela e
  o laço no ritmo da mais rápida (P13).
- Teclado e foco: `default_button` e `cancel_button` no Button (J23); `shortcut_text` (J13);
  `ui.focus_visible` (J08); `ui.open_popups` (J09); o toque longo só pelo dedo (J18).
- Arrasto: `on_drag_move` e `on_drag_leave`, `ui.drop_point` no `on_drag_drop`, os cursores
  `grab` e `grabbing` (J15, J16). Os ganchos de mensagem com número (`remove_message_hook`, J12).
  O diálogo de arquivo com o form como dona, e o bloqueio do modal reaplicado quando ele fecha
  (J02).
- Plataforma: `on_terminate` no `ApplicationEvents` (P04); `ui.advance` para testes (P08); a pasta
  do `placements.json` (`Application::set_placements_dir`, P09); o Timer com `one_shot` e
  `restart` (P10); o app vive na bandeja sem form aberto (P12); o DPI documentado (P14); a classe
  `FrameTimer` (P05); o `tag_string` (P07); o `width`/`height` de um controle alinhado é o do layout
  com o app rodando (L13); o arquivo passando por cima (`on_file_enter`, `on_file_leave`, F03); o
  `FileDropZone` com `with_filter`/`with_multiple`/`with_title` (F05).

Lote 3, controles e tema (feito em 12/11, sem commit; teste
`aylis-controls/tests/tabernaculo_controles.rs`):
- Tema: `text_case`, `letter_spacing`, `border_sides` (a borda por lado, C11) e `border_style`
  (`dashed`, `dotted`; C12) no estilo; o Label pinta o fundo, a borda e o raio do estilo (C10) e
  usa a caixa-alta e o espaçamento dele (C09); a sombra `2xl` (C32); o estilo `Edit.empty` muda a
  borda do campo vazio (C37, o padrão como antes); o estilo `TabControl.tab` desenha as abas
  (normal e escolhida, C17); a dica com o atraso pelo tema (`durations.hint`) ou pelo
  `ui.set_hint_delay`, e com a borda do estilo `Hint` (C19).
- Button: `h_align` (C13), `group_index` (o grupo exclusivo, C03), `auto_size` medindo o texto e o
  ícone (C14). CheckBox, RadioButton e ComboBox com `auto_size` (C21).
- O tema vê o `checked` posto direto no campo (`Component::checked_state`, C02); `inherit_state`
  pinta o filho com o estado do pai (C25).
- PopupMenu: `on_close` (C16) e o item `header` (C15). Classe nova `Popover` (ancorado, fecha com
  o clique fora ou o Escape, `on_close`; C15). Classe nova `DateTimeEdit` (C30).
- `TabItem.on_show` (C18); ComboBox com `groups` e `disabled_items` (C20); `on_paste` no Edit e no
  Memo (C22); o markup do Label com `<sb>`, `<i>`, `<u>`, `<small>`, `<big>`, `<size=N>` e
  `<color=...>` (C23); a bandeja com `popup_menu` (os MenuItem, C26) e `show_balloon` (C27);
  `accessible_name` (C29); `auto_select` no Edit (C36); `translate` (G19).
- O `tag_string` e o `accessible_name` dividem uma caixa no `ControlBase` (8 bytes por controle).

Lote 4, layout e texto (feito em 12/11, sem commit; teste `aylis-controls/tests/tabernaculo_layout.rs`):
- `gap` no Layout e no Panel (L05); o FlowLayout com `v_align`, `baseline` (L12) e o `grow` dos
  filhos (o espaçador, L04); a GridLayout com `grid_row`, `grid_column` e `row_span` (G12); o
  ScrollBox com `gutter` e a barra só com o que rolar (L01); o form com `auto_size`, desligado por
  padrão (L07); `ui.preferred_size` (L11).
- Texto: o serviço de texto do renderizador do projeto já na montagem dos forms
  (`install_text_service`, T01); `line_height`, `font_weight`, `font_size` e `italic` no estilo
  (T04, T05); no Label, `line_height`, `text_shadow` e os métodos `min_content_width` e
  `max_content_width` (T02); o traço por dentro documentado no `Canvas` (T09).

Lote 5, mídia e WebView (feito em 12/11, sem commit; testes em `aylis-media/tests/media.rs` e
`aylis-webview/tests/environment.rs`):
- No fim do vídeo a posição fica no fim (W04); vídeo por endereço `http(s)`/`rtsp` no Media
  Foundation, no GStreamer e no AVFoundation (W11; o macOS e o Linux por conferir); o VideoPlayer
  como fonte de quadros (`show_video`, `on_frame`, `current_frame`) com o relógio e a fila como API
  (`clock_state`, `queued_frames`, `frames_ahead`; W12, W13); o WebView com `background_color`,
  `data_dir` e `browser_args`, sem a depuração remota (W07); a foto do `snapshot` com o WebView
  pela captura da janela (`WindowCaptureService`, no Windows; W10).

Lote 6, gerador e projeto (feito em 12/11, sem commit; testes em `aylis-project/tests/project.rs`):
- A chave `sdk` do `.aylisproj`: projetos novos com o Aylis do crates.io, a pasta fixa como opção,
  e vazio como antes, pela junção (G11, G03; também resolve G15 e G16 nos projetos novos); a
  opção nas Opções do projeto.
- O "Gerar código" (IDE e MCP) aponta a classe com nome errado e a parecida (G02; a classe do app
  continua valendo); o `create_in` transparente como o Frame (G05, `GENERATOR_VERSION` 11); o
  `output.bin` escolhe o binário do Compilar, do Executar e do Publicar, e o gerador não cria
  `src/main.rs` num crate de biblioteca (G13, I08); o `mod generated` fora do `cargo fmt` (G14); o
  `recorded` e a lista do que foi tocado só mudam com mudança (G17); `[package.metadata.aylis]
  project` diz a pasta do projeto (B06); o perfil das dependências também nos `Cargo.toml` antigos
  (B09); o texto com contexto (`arquivo|Abrir`) e as traduções seguindo a correção do texto no
  designer (O02).

Lote 7, IDE e MCP (em parte, 12/11, sem commit):
- O argumento desconhecido é recusado com os que valem (M06); `ide_list` com o pid e `ide_select`
  por ele (M03); o `set_property` com `margins.left` em texto (M09); `set_properties` (M15) e
  `move_component` (M16); o `read_code` pelo caminho relativo ao projeto, com qualquer barra e
  dizendo onde procurou (M24, M26); a descrição do `generate_code` diz que salva (I09); o aviso da
  fonte do sistema uma vez por projeto (I11); a árvore do projeto fechada, lembrando o que o
  usuário abriu (I10); o Design desligado num fonte sem form (I05); a foto sem o cursor de texto e
  sem o hover (B03).

Lote 7, o resto (feito em 12/11, sem commit; roteiros 112 a 117):
- I06: a geração e o `Project::save` só gravam o que mudou (`codegen::write_changed`); a
  `.aylis/.gitignore` tem uma regra só, `*` (`aylis_project::local_dir_ignored`), em vez de ganhar
  uma linha por recurso; o build diz nas mensagens o que mexeu no projeto e por quê
  (`aylis_tools::touched`, no `aylis-tool build`, no Executar e no Publicar). O `Cargo.lock` quem
  reescreve é o cargo, quando o `Cargo.toml` muda (features pelas classes dos forms); a causa
  exata no Tabernáculo segue a conferir.
- M10: `ide_menus` com `id` e `count`; `menu` por `id`, por caminho (`/` ou `›`) e, no Object
  Inspector, pela propriedade; o ambíguo é recusado com os candidatos. M11: `dialog_state` com
  `id`, `hint` e `label` único (`Corrigir… — Android SDK (Android)`); `dialog_click` por eles.
- M12: passados 45 s, a chamada responde `in_progress` com o número da tarefa e continua;
  `task_status` espera e traz o resultado; o `status` lista as pendentes (`mcp_tasks.rs`).
- M20: `output` com `source` (build/program) e `level` em cada linha, filtros e `previous` (o
  histórico de 2000 linhas, mantido ao trocar de projeto). M21: o `app_command` descreve todos os
  comandos e a feature `dev`, escolhe o app (`app`: pid ou projeto; `{"cmd":"apps"}` lista), e o
  `snapshot` volta como imagem.
- M17: o "Novo form" da IDE e o `new_form` com a pasta dentro de `forms/` (`folder`, sem absoluto
  nem `..`). M18: `new_frame`, `get_theme`/`set_theme`, `translations`/`set_translations`,
  `set_app_icon` e o `publish` com AAB e instalador, esperando como o `build` (a senha da chave
  fica em `AYLIS_KEYSTORE_PASSWORD`). M19: `check`, `test` (cargo test, unittest, go test,
  bun/node, dotnet test; recusado no modo restrito) e `screenshot` (qualquer form pelo designer,
  ou o app em execução); `docs/proposta-llm-ci.md` atualizado. Falta: o executor de testes de
  Java, C++ e Pascal; a pasta no "Novo DataModule"; o `set_theme` chegando ao app em execução.
- B04: `aylis::headless::Headless` monta o `Ui` sem janela nem `platform::run` (as classes, o texto
  e o desenho fora da tela do renderizador do projeto; `install_offscreen_service`), abre o projeto
  ou um form, simula `click`, `type_text`, `press`, `advance` e tira a foto (`render_png`).
  Documentado em `docs/funcionamento.md`; teste `crates/aylis/tests/headless.rs` (Skia e Direct2D).

Mídia (feito em 12/11, sem commit; testes `aylis-media/tests/texture.rs` e `http.rs`):
- W15: com o canvas no Direct2D da mesma placa, o Media Foundation entrega o quadro na placa, o
  conversor do Direct3D 11 escreve BGRA numa textura compartilhada e o Direct2D a desenha sem
  passar pela CPU; o Skia, o navegador e o `current_frame` recebem os pixels copiados uma vez (o
  teste compara: diferença média abaixo de 3 por canal). API: `Frame::gpu`, `Frame::draw`,
  `GpuFrame`, `VideoPlayer::current_video_frame`. Medido com um 4K a 60 quadros/s (RX 9070 XT,
  release): o decodificador de 13–17 ms para 1,6 ms por quadro; a pintura no p95 de 8,6–9,0 para
  0,62 ms; a CPU de 79–83% para 13–18% de um núcleo; a memória privada de 1,1–1,3 GB para 0,65 GB.
  Com o quadro na placa, `frame.image` vem vazia: quem pinta usa `current_video_frame` e
  `Frame::draw`. Não conferido: duas placas de vídeo.
- W11: o som sozinho por endereço, lido aos poucos com `Range` (`HttpSource`, feature `http`, a
  padrão, com o `ureq` que a plataforma já usa) e, no Windows, pelo Media Foundation como reserva;
  vale no MediaPlayer, no VideoPlayer só de som e no `open_audio_location`. Não conferido no Linux
  e no macOS.

Controles, layout e projeto (feito em 12/11, sem commit; testes
`aylis-controls/tests/tabernaculo_rendering.rs`, `tabernaculo_followup.rs`, `partial_paint.rs`,
`aylis-core/tests/input.rs` e `ico_frames.rs`, `aylis-project/tests/frame_instances.rs` e
`sync_hand_edits.rs`):
- C17: as abas laterais do TabControl (`tab_position`, `tab_width`; o estilo `TabControl.tab` com a
  variante do lado). C38: o `gap` no estilo do tema (ButtonGroup e SegmentedControl; 0 cola). L15:
  o rodapé da caixa de mensagem pelo estilo `MessageDialog.footer` (antes, 36 + 14 fixos).
- J11: `client_width`/`client_height` no Form, e a janela acompanhando o `width`/`height` mudado
  com o app rodando (`WindowCommand::ApplySize`); a moldura opcional já era o `border_style`.
- L02: o `update_layout` refaz até assentar (no máximo 8 vezes), e a janela de novo depois dos
  relógios: o que o `on_resize` muda sai no mesmo quadro.
- C41: o quadro do `.ico` do tamanho da bandeja na escala (`image::ico_frame_for_size`; 24 px a
  150%), refeito quando a escala muda.
- C24: o Skia e o Direct2D já faziam a opacidade de grupo; o Canvas 2D só multiplicava o
  `globalAlpha` e agora desenha o grupo num canvas à parte. Falta conferir num navegador.
- Pintura por parte: o Skia na CPU refazia as curvas cortadas pela borda da área (até ~100 tons na
  beirada, até 4 px para dentro; o Direct2D não tem isso). A área pequena e compacta é pintada numa
  superfície 16 px maior. O Popover e o DateTimeEdit voltaram ao lugar natural no `register`;
  quadros com diferença acima de 4 tons, de 77 para 32. `giant_screen` igual dentro do ruído.
- G21: o `create_in` pela pasta (feature `dev`, IDE) relia e interpretava o arquivo a cada
  cartão; o `ProjectForms` guarda o JSON por form (`CachedForm`, com o `FileStamp`, conferido uma
  vez por instante do `Ui::time`). Em release, 200 cartões de 7 controles: pasta de 136–142 para
  22 µs; pacote de 24 para 22 µs; em código, 13–14 µs (o resto são os nomes do acesso tipado e a
  tradução). O "18×" do relatório não se confirmou no pacote (era ~2×). Falta a máquina da igreja.
- G22: o `sync` acha a definição de verdade (fora de comentários e strings), conta só as chaves
  de código, e a impressão `v2:` ignora comentários e a vírgula do `cargo fmt`; a renomeação à
  mão leva o histórico.
- Não se aplicam: C08 (o `with_opacity` no `fill` aparece por todos os caminhos, nos dois
  renderizadores; sem `ui.damage` a tela não repinta) e J03 (o núcleo exige o mesmo id; o
  controle liberado ou escondido não passa a contagem; o teste fica como guarda).

As duas falhas da suíte que vinham desde o 0.2.0-beta.3 (corrigidas em 12/11):
- `apps` (`every_app_has_all_languages_and_nothing_to_port`): a Calculadora, o Paint, o Player
  e o IBGE tinham handlers "a portar" sem nenhum fonte mudado desde 09/11. A impressão gravada
  em 09/11 em Python, TypeScript, C#, Java e Pascal era de uma regra mais velha que a "antiga"
  do G22; o `record` a tomava por edição e subia a revisão, e as outras linguagens ficavam
  atrasadas. Agora a impressão sem versão que nenhuma regra reconhece só é refeita pelo fonte,
  sem revisão nem porte (`hashes_from_before_still_match`); os `code-sync.json` dos cinco apps
  voltaram ao estado de 09/11 e foram regravados com a regra nova, sem nada pendente.
- `documentacao` da Landing: não era instabilidade. Na suíte do workspace o cargo une as
  features do `aylis`, o Gauge fica registrado e a página o monta ao vivo; sozinha, a Landing
  não o registra e mostra a imagem. O teste confere o registro e espera o caso certo.

Cantos da janela (pedido de 12/11, sem commit; roteiro 118): a propriedade `window_corners` do Form
(`default`, `round`, `small`, `square`). No Windows 11 o DWM desenha os cantos
(`DWMWA_WINDOW_CORNER_PREFERENCE`), com a borda suave e a sombra dele; o `default` com
`border_style = custom` arredonda como uma janela comum, e com `none` fica como o sistema deixa
(reto). Muda com o app rodando (`WindowCommand::UpdateCorners`). No Windows 10 o atributo não
existe e os cantos ficam retos. No macOS, a barra própria passa a ser a barra do sistema
transparente com o conteúdo por baixo (mantém os cantos e a sombra); escrito, não compilado nem
conferido num Mac (testes-em-dispositivos). No Linux, no celular e no navegador, sem efeito.
Conferido na tela com o Direct2D e o Skia na CPU (a captura por GDI não mostra o Skia em OpenGL).

O menu de layouts do Windows 11 na barra própria (pedido de 12/11, sem commit; roteiro 118): a
TitleBar diz ao `Ui` onde fica o botão maximizar (`Ui::set_maximize_button`,
`Ui::maximize_button`), e a subclass da janela responde `HTMAXBUTTON` no `WM_NCHITTEST` sobre
ele: o Windows passa a tratá-lo como o botão da janela e, com o mouse parado, mostra o próprio
menu de layouts (Snap). O mouse ali chega como mensagens da moldura (`WM_NCMOUSEMOVE`,
`WM_NCLBUTTONDOWN`/`UP`, `WM_NCMOUSELEAVE`), que voltam ao `Ui` como mouse na barra: o realce e o
clique (maximizar e restaurar) continuam os da barra. Conferido na tela (Direct2D, 150%): o menu
aparece e os dois cliques maximizam e restauram. Os ganchos do app (`on_message`) vêm antes e
podem mudar isso. Fora do Windows, sem efeito.

O instalador e o desinstalador mais rápidos (pedido de 12/11, sem commit: "os projetos de exemplo
estão custando muito ao instalador e demorando muito para desinstalar, fica rodando arquivo por
arquivo"):
- O que demorava era a lista de detalhes do NSIS, uma linha por arquivo. Medido com 20 mil
  arquivos: `RMDir /r` em 28,3 s com a lista, 3,1 s sem ela (`SetDetailsPrint textonly`) e 2,6 s
  com o `rd /s /q` do Windows. A instalação e a desinstalação passam a mostrar só a etapa
  ("Removendo o Aylis…", "Removendo as compilações dos projetos…"); com a pasta de compilação
  (91 mil arquivos na máquina do autor), a remoção cai de uns 2 minutos para uns 15 s (estimado
  pela medida; a desinstalação de verdade não foi rodada, para não apagar a pasta do autor).
- Os exemplos vão sem o `generated/` (o Aylis refaz ao gerar, compilar ou executar; conferido com
  a Calculadora: o `generate` refaz as 8 linguagens e o `check` passa). Instalado: 2.513 → 1.834
  arquivos (os exemplos, 1.368 → 689); a instalação silenciosa, 32 → 20 s. O instalador quase
  não muda (73,5 MB: o código gerado comprime bem). A documentação da Landing fica: o
  `component_docs` do MCP lê as páginas dela.
- Teste `the_script_leaves_generated_out_and_does_not_list_every_file`.

Pacotes de ferramentas e a verificação de ambiente rápida (pedido de 12/11, sem commit; o
desenho em `docs/pacotes.md`, a decisão no histórico; roteiro 120 a 122):
- **Verificação de ambiente:** o resultado fica em `%USERPROFILE%\.aylis\environment.json`
  (`aylis_tools::env_cache`); cada plataforma anota do que dependeu (o programa achado com a
  marca, as pastas do SDK e do rustup, o `installed.json` dos pacotes, o `tools.json`, as
  variáveis), e sem mudança nenhum processo roda; um resultado guardado vale no máximo 24 h
  (`MAX_AGE_SECS`: o que muda dentro do WSL ou num `pip` feito fora não deixa marca). As
  plataformas rodam em paralelo, com prazo nos processos (20 s; 45 s no WSL). A tela abre com o
  guardado e reconfere só as do projeto (a do Executar à frente); as outras mostram "guardado
  há…" e "Conferir". O F9 usa o guardado. Medido em debug: abrir, de 1,1 s (4,5–5 s com o WSL
  desligado) para ~1 ms mais a reconferência em segundo plano (~130 ms sem mudança); o F9, de
  ~300 para ~50 ms; "Verificar de novo", de 1,1 para 0,34 s (3,9 s com o WSL desligado).
- **Núcleo dos pacotes** (`aylis_tools::toolpacks`, `resources/packages.lock.json`, sucessor do
  `tools.lock.json`): instala em `%USERPROFILE%\.aylis\tools\<nome>\<versão>` (registro em
  `installed.json`), com as dependências antes, a troca atômica, o progresso e o cancelamento.
  Os arquivos vêm do cache, da pasta `pacotes` ao lado do executável, das pastas e dos espelhos
  das Opções (um `pacotes.json` com `url` ou `drive`, com a confirmação do Drive, ou o link de
  uma pasta compartilhada do Drive, listada pela vista embutida; conferido em 12/11 com a pasta
  do autor: o nsis e o jdk de 183 MB baixados dela em 26 s e o java rodando), conferidos
  pelo SHA-256 do manifesto embutido; prazo, proxy das variáveis e do Windows (sem PAC/WPAD), e
  nunca a URL oficial (só o `packages pack`/`update` a usam). Pacotes no Windows: rust 1.99.0
  (+ rust-android, rust-wasm), jdk 17.0.21+9, android-sdk, android-ndk 27.2.12479018,
  cargo-ndk 4.1.2, wasm-bindgen 0.2.129, python 3.12.10, node 26.13.1, dotnet 8.0.428, go
  1.27.4, nsis 3.11, codelldb, netcoredbg, js-debug, clangd, bundletool e jdtls. Conferido: o
  `pack` de jdk, node, codelldb e rust (500 MB, 66 s) e o `install` sem rede (22 s), com java,
  node, cargo, rustc, rust-analyzer e codelldb rodando e um projeto compilado (host e wasm).
- **Na IDE:** Ferramentas → Pacotes de ferramentas (a lista, instalar e remover com o
  andamento e o Cancelar, "já existe no sistema"); a aba Pacotes nas Opções (pastas, espelhos,
  "Testar as origens"); o "Corrigir…" instala o pacote que resolve o item, sem trazer o que a
  máquina já tem, e sem origem diz isso e leva às Opções. O `tools.json` passou para
  `%USERPROFILE%\.aylis\tools.json` (o do `AppData` é copiado uma vez).
- **No instalador:** uma página com os pacotes do manifesto para marcar (nenhum, ou o `rust` no
  kit offline sem Rust na máquina) e a do espelho; no fim, o `aylis-tool` instalado instala os
  marcados da pasta `pacotes` ao lado do setup (o que falhar é avisado, código 3, e a IDE
  fica). Silencioso: `/S /PACOTES=rust,jdk|all /ESPELHO=https://...`. O desinstalador remove
  também os pacotes, o cache e o `environment.json`. O kit offline: `aylis-tool installer-ide
  --with-packages <nomes|all>`. Testado em modo silencioso por fora do Claude (WMI): o kit com
  node e codelldb instalado em 37,8 s; com o `go` sem origem, saída 3 e a IDE instalada (o
  registro, o PATH e os atalhos restaurados depois).
- **Falta:** o Free Pascal (só há instalador Inno Setup), o WiX e o Build Tools da Microsoft (o
  Rust do Windows depende dele para ligar; o Rust `gnu` com MinGW foi testado e funciona sem
  ele, mas sem o Skia, e o autor decidiu manter o `msvc`, veja o histórico); pacotes para pylsp, tsls, gopls, delve, csharp-ls
  e pasls (continuam pelos gerenciadores); android, dotnet, python e nsis no Linux e no macOS;
  o JDK do macOS (testes-em-dispositivos); o proxy por PAC/WPAD; o "Configurar linguagens" ainda
  cai no winget/rustup quando não há origem; as páginas do instalador vistas na tela e o
  desinstalador numa máquina de teste.

Quadros de câmera e a repintura (relatado em 13/11 por um app com várias câmeras e duas janelas;
feito, sem commit; teste `aylis-media/tests/repaint.rs`):
- Conferido no código: o `FrameSink::push_frame` (e o `CameraView`) pedia `request_repaint` a
  cada quadro — a época do dano subia e todas as janelas repintavam inteiras —, com uma
  mensagem por quadro para a thread da interface; o Direct2D soltava a textura não desenhada
  nos últimos 2 quadros de tela, e o vídeo (até 12 texturas girando, 15 quadros de câmera
  contra 89 de tela) a reabria a cada quadro; e uma janela sem marca própria, quando só outra
  mudava, também pintava inteira (`take_frame` devolve `Whole` sem marca).
- Agora: o quadro marca só o controle (`ui.damage`) e um aviso fica pendente até a tela atender
  (`video::notify_frame`; 30 quadros com a tela ocupada viram 1 mensagem); o quadro pedido por
  hora (`request_frame_at`: o vídeo tocando, o cursor) marca só quem pediu
  (`Ui::take_due_frames`); a plataforma pede a pintura só das janelas com o que pintar
  (`Ui::forms_to_paint`; na dúvida — layout, animação, `request_repaint`, marca num form sem
  janela, como o designer da IDE — todas, como antes); o Direct2D mantém a textura aberta
  enquanto ela for desenhada nos últimos 2 s, com teto de 64 e a chave pelo id e pelo handle.
- Conferido: os testes de núcleo, mídia (o antigo `frames_pushed_from_another_thread_are_shown`
  esperava uma mensagem por quadro e passou a esperar uma), Direct2D, pintura por parte, IDE e
  telas da IDE; o `giant_screen` (relógio 6,1 ms contra a base 4,57, dentro do limite; a
  máquina com uma IDE aberta), wasm e Android; o exemplo `video` com um MP4 a 30 quadros por
  segundo, pela textura, sem pular. Não medido no app das câmeras (as ~950 mensagens por
  segundo, os ~8% das texturas): falta o autor rodar o app.
- Para quem cria componente ("o desenvolvimento de componentes está fácil ainda?" — "Sim"):
  `Headless::check_partial_paint(nome, mudança)` (o mesmo teste das classes do Aylis, para o
  componente do app) passa o componente pelo mouse, pelo foco, por um deslocamento e pela
  mudança dada, compara a pintura só da área com a inteira e aponta o desenho que mudou sem
  aviso (o `ctx.repaint()` que faltou); o `OffscreenService` ganhou `render_area_rgba` (o
  Skia e o Direct2D). O guia (`docs/criar-componente.md`, e a cópia da Landing) diz o que
  avisa sozinho (o que passa pelo `Ui`, inclusive o `ui.get_mut`) e o que não (o dado de fora,
  o `on_frame`), e que o `request_frame_at` repinta só o próprio componente. Testes em
  `crates/aylis/tests/headless.rs` (Skia e Direct2D).
- **Escala de um componente com os filhos (`scale`, como o `Scale` do FMX)** (13/11; roteiro,
  item 127). Propriedade nova do `ControlBase` (categoria Layout, 1 = normal, de 0,05 a 20): o
  controle ocupa o mesmo retângulo no pai, e por dentro o espaço é o retângulo dividido pela
  escala (`inner_size`). Acompanham: o layout dos filhos, o `auto_size` (medido dentro, vezes a
  escala), o clique (`hit_node`, `local_point`), o `absolute_rect`, o `to_form` e o
  `Ui::inner_scale` (o produto das escalas até o form), a pintura (`canvas.scale` depois da
  rotação) e o dano (a parte e a rolagem por cópia dentro de um controle com escala viram o
  controle inteiro). No `Form` vale como zoom da tela toda. O nome é `scale` porque `zoom` já é
  do Chart, DocViewer, MapView, Model3D, PaintCanvas, ReportPreview e Form. Testes:
  `crates/aylis-controls/tests/scale.rs` e o painel com escala no `check_partial_paint`
  (`crates/aylis/tests/headless.rs`); `giant_screen` passou (relógio ~5 ms, base 4,57).
  Falta conferir numa janela de verdade: as alças do designer, as listas que abrem (popups), o
  cursor e o IME de texto dentro de um painel com escala, e a pintura no Canvas 2D do navegador
  (só compilado).
- **A documentação da Landing no celular** (13/11; roteiro, item 128). Pedido: "Documentação do
  landing page não é responsiva para funcionar no smartphone". Antes, na vista `phone` do
  `frmDocumentacao` o índice ficava à esquerda com 220 de largura e a página sobrava com ~100
  (uma palavra por linha), e o "Próxima" saía por cima do índice. Agora, no celular:
  - o índice sai da esquerda e vira um menu de tela inteira, aberto e fechado pelo botão novo
    `btnMenu` do topo (`alternar_menu`); escolher uma página fecha o menu, e uma seção só abre na
    árvore; a busca abre o menu com os achados, e escolher um fecha;
  - no topo, a marca, a busca ocupando o meio e "Site" no lugar de "Voltar ao site";
  - a página com a largura toda (margens de 16 e 12) e "Anterior" / "Próxima" um embaixo do
    outro;
  - fora do celular nada muda (a vista devolve o base ao sair, inclusive com o menu aberto).
  Na página principal, a vista `phone` escondia o `fluMenu` inteiro, e o "Documentação" do topo
  sumia (só restava o "Ver todos na documentação" lá embaixo); agora só o "Experimentar" some.
  Teste `on_the_phone_the_index_is_a_full_screen_menu` (`examples/Landing/rust/tests/
  documentacao.rs`) e a conferência do topo em `pagina.rs`. Conferido no navegador (o wasm
  publicado, 375×812 emulado): o menu, a seção e a página. Falta: publicar a Landing e ver num
  celular de verdade.
- **O instalador da IDE mais simples** (13/11; roteiro, item 129). Pedido: "Simplificar as opções
  do instalador, deve mostrar agrupado por linguagem/plataforma de destino, Rust deve sempre ter
  [...] ao marcar uma linguagem deve instalar tudo pra ela funcionar bem, e o link da pasta do
  meu drive não apareceu como provedor padrão [...] remover os projetos de exemplo".
  - Uma página de componentes só (saíram a dos pacotes e a do espelho): "Aylis IDE e Rust
    (obrigatório)", com o código do Aylis, os clientes e a documentação; os grupos "Outras
    linguagens" (Python, TypeScript, C#, Java, Go, C++, Free Pascal) e "Plataformas de destino"
    (Android, Navegador), desmarcados; e o atalho.
  - O Rust sempre se configura, e cada linguagem ou plataforma marcada também, no fim, pelo
    `aylis-tool setup` dela (o mesmo do "Configurar linguagens": compilador, Visual C++,
    depurador, servidor de linguagem; no Android, JDK, SDK, NDK e cargo-ndk; no navegador, o alvo
    wasm e o wasm-bindgen). O `setup` ganhou `--target android|web` (`setup::setup_targets`,
    que nessas saídas só roda o que falta de fato e nada que abra janela sem pacote) e
    `--utf16-output` (os acentos nos detalhes do instalador). O que não ficar pronto é dito no
    fim (código de saída 3), com o caminho para refazer na IDE.
  - Silencioso: `/LINGUAGENS=python,go|all`, `/PLATAFORMAS=android,web|all` e `/ESPELHO=` (no
    lugar do `/PACOTES=`). A pasta `pacotes` ao lado do instalador vale pela variável
    `AYLIS_PACKAGE_DIR` (sem ficar gravada nas Opções).
  - O espelho padrão: a pasta do Drive (`settings::DEFAULT_PACKAGE_MIRROR`) vale enquanto
    nada foi gravado em `package_mirrors`; apagada nas Opções, fica apagada (a lista vazia é
    gravada). Antes, sem informar o espelho na página do instalador, nada era gravado (o
    `tools.json` desta máquina não tinha espelho).
  - Os exemplos não vão: o `Cargo.toml` instalado sai sem nenhum `examples/` nos membros. As
    páginas de referência dos componentes (o `component_docs` do MCP) vão para
    `aylis\docs\referencia`, onde o MCP procura primeiro.
  - Testes em `installer.rs` (a página agrupada, o Rust sempre, sem shell, sem exemplos) e em
    `settings.rs` (o espelho padrão). O instalador foi gerado; não foi rodado nesta máquina (ele
    trocaria a instalação do autor). Falta: instalar numa máquina limpa (ou numa VM) com Python
    e Android marcados e conferir que ficam prontos.
- **ComboBox: quantas opções a lista mostra, a barra e o caber na tela** (13/11; roteiro, item
  130). Pergunta: "O combobox tem scroll para os itens com possibilidade de definir quantos itens
  deve exibir?". Rolava (roda, dedo, setas), mas eram 10 linhas fixas no código e sem barra.
  Agora:
  - `drop_down_count` (o `DropDownCount` do Delphi; padrão 10, mínimo 1), propriedade do
    registro (JSON, Object Inspector, protocolo, clientes), com o rótulo traduzido;
  - com mais opções que as à vista, uma barra fina à direita da lista (`text_muted` do tema):
    o polegar se arrasta, um clique na calha pula para lá, e soltar não escolhe nem fecha;
  - a lista abre embaixo se couber, senão em cima; sem lugar para todas as linhas pedidas em
    nenhum dos lados, abre no lado com mais lugar e mostra as que couberem (`drop_place`).
  Testes em `crates/aylis-controls/tests/combo_drop_down.rs` (as linhas, a roda, a barra
  arrastada, o encolher e a barra pintada só quando rola). O catálogo e a página do ComboBox
  na documentação foram refeitos.

Ficam para depois:
- T06 (o provedor de PDF que embute as fontes), decidido pelo autor.
- A pintura por parte no navegador (se o Canvas 2D tem o mesmo efeito do Skia na borda).

**Compositor em outra thread (12/11/2026)**

Pedido do autor: estudar um compositor como o do navegador e o Core Animation, em que as
camadas já pintadas (lugar, transparência, recorte) são compostas e apresentadas por outra
thread, e a animação de `translate_x`/`translate_y`/`opacity` segue lisa mesmo com a thread da
interface ocupada. Aprovado só registrar e medir; a mudança de arquitetura espera o autor.

Como seria, sem quebrar nem piorar o que existe:
- O `Ui` continua numa thread só (`Rc`/`RefCell`) e os controles e o `Canvas` não mudam: o
  compositor recebe um retrato (as imagens das camadas e as curvas) e não encosta no `Ui`.
- Só nos caminhos de GPU (a swap chain do Direct2D, o Skia GPU). O Skia na CPU e o navegador
  ficam como estão (compor noutra thread na CPU é copiar a tela inteira por quadro).
- O Direct2D hoje é `D2D1_FACTORY_TYPE_SINGLE_THREADED`: medir a fábrica multithread (trava em
  cada chamada) ou compor direto no Direct3D/DirectComposition.
- Textura só para a camada que anima ou tem `cache_layer`, liberada no fim (a IDE abre um `Ui`
  por aba). Sem animação, a thread dorme.
- O compositor só assume quando a interface atrasa um quadro, para não somar um quadro de
  latência no dia a dia. O `partial_paint` e o `giant_screen` seguem valendo.
- Ajuda só no que move ou esmaece desenho pronto (transições, animação de lugar e
  transparência, o vídeo e o 3D já na GPU, a rolagem se um dia houver ladrilhos); mudança de
  conteúdo (hover, texto, dados) segue na interface.

Medição de 12/11 (exemplo `transitions` do `aylis-controls`, Skia na CPU, sem janela; os
números em `medicoes/transicoes.json`): duas abas cheias de controles e um painel cheio
deslizando por cima, 250 ms a 60 Hz.

| Cenário | 1920 × 1080: quadro p50 / p95 | > 16,7 ms | 3840 × 2160: p50 / p95 | > 16,7 ms | Controles por quadro (1080p / 4K) |
|---|---|---|---|---|---|
| Aba deslizando | 5,9 / 6,7 ms | 0 de 136 | 26 / 29 ms | 136 de 136 | 394 / 1435 |
| Aba esmaecendo | 13,6 / 16,3 ms | 5 de 136 | 65 / 167 ms | 136 de 136 | 680 / 2654 |
| Painel animado | 4,9 / 5,3 ms | 0 de 136 | 50 / 73 ms | 136 de 136 | 420 / 1488 |
| Painel com `cache_layer` | 5,3 / 6,1 ms | 0 de 136 | 29 / 56 ms | 134 de 136 | 342 / 1329 |

O que a medição mostrou:
- Hoje o problema não é a interface ocupada: é o próprio quadro. As transições de aba repintam
  a tela inteira a cada quadro (área 100%) e pintam todos os controles das duas páginas; o
  esmaecer pinta as duas por inteiro com camada de transparência e é o mais caro. Em 4K na CPU,
  nada cabe em 16,7 ms.
- O painel com `cache_layer` tem a imagem guardada, mas o que ele descobre ao andar (a página
  de trás) é repintado: 342 controles por quadro em 1080p.
- Logo, a maior parte do ganho vem de compor camadas guardadas **na mesma thread** (as duas
  páginas da transição e o fundo atrás de quem anda, como imagens, sem repintar controles); a
  outra thread é o passo seguinte, menor.
- Faltam: a mesma medição no Direct2D com janela (o padrão no Windows; só a CPU foi medida),
  e quanto tempo os handlers de apps reais seguram a interface (o caso em que só a outra
  thread ajuda).
- Os tempos em 4K variaram até 2x entre rodadas (outras sessões compilando); a área e os
  controles são exatos.

As transições com camada guardada (feito em 12/11, sem commit; pedido do autor: "Sim, investigue"):
- Por que a camada não valia: (1) o `tick_transitions` chamava `request_repaint` a cada quadro,
  a janela inteira era repintada e a pintura inteira refaz as camadas (`layer_epoch`); (2) o
  `apply_transition` mudava o `translate`/`opacity` pelo `control_mut`, que marca o desenho de
  dentro (a versão da camada subia a cada quadro) e o layout do form; (3) a condição do
  `cached` exigia `clip_children`, e o `TabItem` não recorta.
- Agora (`aylis-core`: `transition.rs` e `paint.rs`): o `apply_transition` muda pelo `cb_mut` e
  marca só o lugar (`damage_placement`), sem `request_repaint`; quem está em transição vai
  para a camada guardada mesmo sem recortar os filhos (por 250 ms, o que passa da borda da
  página fica de fora); e a camada guardada anda em pixels inteiros do dispositivo (a imagem
  montada com a fração de um quadro, reposta arredondada noutro, saía meio pixel diferente da
  pintura inteira: 32 pixels no primeiro quadro, achados pelo teste novo).
- Teste `a_tab_transition_moves_the_pages_without_repainting_their_children` (`partial_paint`,
  Skia e Direct2D, deslizar e esmaecer): os filhos não são repintados durante a transição e
  cada quadro por parte sai igual à pintura inteira. Passam também o `partial_paint` inteiro,
  os testes do `aylis-core`, `transitions`, `tab_pages`, `tab_strip`, `tab_strip_drag` e o
  `cargo check` do `aylis-core` para wasm.
- Medido com o exemplo `transitions` (antes e depois compilados e rodados alternados, a máquina
  em 100% de CPU com outras sessões; tempos oscilaram até 2x entre rodadas, os controles são
  exatos):

| Cenário | Controles por quadro (1080p / 4K) | Quadro p50 1080p (melhor rodada) | Quadro p50 4K (melhor rodada) | 1º quadro 1080p |
|---|---|---|---|---|
| Aba deslizando | 394 → 4 / 1435 → 4 | 6,0 → 2,9 ms | 28 → 17 ms | 5 → 14 ms |
| Aba esmaecendo | 680 → 4 / 2654 → 4 | 13,7 → 9,4 ms | 57 → 45 ms | 5 → 9 ms |
| Painéis (não mudaram) | 420 e 342 | iguais | iguais | iguais |

- O que piorou: o primeiro quadro, que monta as duas camadas de página inteira (em 1080p
  ainda dentro de 16,7 ms; em 4K na CPU, de ~27 para 57 ms ou mais). E a memória: as duas
  imagens (8 MB cada em 1080p, 33 MB em 4K) ficam no cache do renderizador depois da transição
  até ele se limpar (16 imagens no Skia); falta soltá-las no fim da transição.
- O resto do quadro na CPU agora é compor duas imagens do tamanho da tela (largura de banda
  da memória); na GPU (Direct2D na janela) isso deve ficar perto de zero, mas não foi medido.
- O `giant_screen`: a área e os controles iguais à base; com a máquina livre, o teste passa
  (em 12/11 à tarde, com a CPU em 100%, os dois lados ficaram uns 3x acima da base).
- Os painéis animados por cima de uma página (`painel animado`, `painel cache_layer`) seguem
  repintando a página de trás que descobrem (342 a 420 controles por quadro): o fundo como
  camada é o passo seguinte, se valer.

As camadas guardadas soltas e configuráveis (feito em 12/11, sem commit; pedido do autor: "pode
fazer, mais esse comportamento de cache que faz com que armazene em memória eu preciso que fique
configurável por projeto ou por formulário"):
- Antes, cada renderizador guardava até 16 imagens e só limpava tudo ao chegar nas 16; as das
  transições ficavam mortas lá (a chave muda a cada pintura inteira): até 16 páginas inteiras,
  ~130 MB em 1080p e ~530 MB em 4K, por janela.
- Agora os três (Skia, Direct2D, Canvas 2D) usam o mesmo `aylis_core::LayerCache`: limite em
  megabytes (sai a usada há mais tempo; uma maior que o limite inteiro nem é montada) e no
  painel de memória ("· camadas guardadas"). O núcleo solta a camada de um controle quando ela
  deixa de valer (`Canvas::forget_cached`): a chave mudou, a transição acabou, o controle foi
  liberado.
- No projeto: `image_cache.layers_mb` (padrão 128, de 0 a 4096; 0 = nada guardado) e
  `image_cache.auto_layers` (padrão ligado), em Projeto → Opções do projeto → Memória. No
  form: a propriedade `layer_cache` (`project`, `on`, `off`), que troca as camadas automáticas
  (transição e contêiner animando) só nele. O `cache_layer` pedido num controle vale sempre.
- Testes: `the_transition_layers_leave_the_renderer_when_it_ends` e
  `the_form_and_the_project_turn_the_automatic_layers_off` (`aylis-controls/tests/transitions.rs`;
  o primeiro falha sem o soltar: ficavam 2 imagens),
  `the_layer_cache_keeps_the_byte_limit_and_forgets_by_owner` (`aylis-core/tests/image_cache.rs`)
  e o `project_memory_saves_the_image_cache_limits` da IDE com o campo novo. Passam o
  `aylis-core`, o `aylis-project` e o `aylis-controls` inteiros, o `giant_screen` em release, o
  `catalog` do `aylis-remote` e o `cargo check` para wasm do núcleo, dos controles, do
  `aylis-web` e da IDE lite. Catálogo regravado (`update-catalog`), rótulo em
  `properties.tsv`, os 4 textos novos da IDE traduzidos e a linha na página do `Form` da
  Landing (à mão; o `aylis-tool docs` regrava tudo e não foi rodado).
- Falha na IDE, de outra sessão: `every_inspector_property_has_a_readable_translated_name`
  (`UrlScheme.bring_to_front`, `UrlScheme.scheme`, `WmiQuery.namespace` sem rótulo no
  `properties.tsv`).
- Medido com o exemplo `transitions` (agora com a coluna "guardado"), a máquina livre, o HEAD
  contra o de agora (`medicoes/transicoes.json`):

| Cenário | Quadro p50 1080p | Quadro p50 4K | Acima de 16,7 ms em 4K | 1º quadro 1080p / 4K | Guardado depois |
|---|---|---|---|---|---|
| Aba deslizando | 5,8 → 2,1 ms | 22,4 → 13,4 ms | 136 → 24 de 136 | 5,3 → 12,5 / 20,8 → 49,1 ms | 0 MB |
| Aba esmaecendo | 13,0 → 7,7 ms | 52,3 → 33,9 ms | 136 → 136 | 4,5 → 6,9 / 21,9 → 30,9 ms | 0 MB |
| Painel animado | 4,7 → 4,6 ms | 21,2 → 19,2 ms | 136 → 136 | igual | 0 MB |
| Painel `cache_layer` | 4,8 → 4,7 ms | 24,4 → 21,5 ms | 125 → 113 | igual | 4 MB / 16 MB (a dele) |

- Segue mais caro o primeiro quadro da transição (monta as duas camadas); quem preferir o
  primeiro quadro ao resto desliga com `layer_cache = off` no form ou `auto_layers` no projeto.

**Área de transferência central (12/11/2026)**

Pedido do autor: "pode implementar e ajustar os lugares onde tiveram que fazer diretamente em
cada projeto, vamos centralizar isso para usar o do Aylis atendendo todos os casos". Feito,
sem commit; falta o autor conferir (roteiro 119):
- `aylis_core::clipboard`: texto, HTML, imagem e os formatos do app (`ClipboardContent`),
  gravados de uma vez. O que a plataforma não guarda fica no processo até outro app copiar.
  O `ClipboardService` ganhou `set_content`, `get_html`, `get_image`, `get_data`, `has` e
  `sequence`, com padrões (um serviço só de texto continua valendo).
- Por plataforma: Windows com o `clipboard-win` (tudo, e o formato do app vai para outra
  instância; o `arboard` saiu do Windows); Linux e macOS com o `arboard` e a `image-data`
  (texto, HTML com texto ou imagem: um por vez); Android com `newHtmlText`; navegador com o
  `ClipboardItem` (texto e HTML).
- Componente não visual `Clipboard` (System): `copy_text`, `copy_html`, `copy_data`,
  `copy_image` (de um `Image` ou a foto de outro controle), `paste_*`, `has_format` e `clear`.
- Saíram as cópias próprias: o PaintBox (a imagem vai para os outros apps e ele cola a de fora,
  também PNG/JPEG) e o designer da IDE (os componentes em
  `application/x-aylis-components+json` e como texto JSON: colam noutra aba, noutra IDE e de
  um editor). `Edit`, `Memo`, `MemoEditor`, `RichEditor`, `DBGrid`, Markdown, o rótulo de
  copiar e a IDE passam pelo `aylis_core::clipboard`.
- Testes: `crates/aylis-controls/tests/clipboard.rs` (5), o do designer em
  `crates/aylis-ide/tests/document.rs` e, no `aylis-platform-winit`, o DIB (2) e o da área de
  transferência real do Windows (`#[ignore]`: grava e devolve o texto que estava; passou).
- Falta: o iOS ainda não tem `ClipboardService` (copia e cola só dentro do app; o
  `UIPasteboard` pede um Mac); no macOS a `image-data` traz o `image` com TIFF (o tamanho não
  foi medido); o Linux só foi checado compilando o `clipboard.rs` à parte (o Docker estava
  desligado); o macOS não foi compilado.

**Prefixo de URL próprio: o `UrlScheme` (12/11/2026)**

Pedido do autor: um componente que, ativado, registra no Windows um prefixo próprio, e o link
chamado por qualquer app chega no evento dele. Feito, sem commit; falta o autor conferir
(roteiro 125):
- Componente não visual `UrlScheme` (System): `scheme`, `description`, `active`,
  `bring_to_front`; `on_link` e `on_error`; `register()`, `unregister()`, `is_registered()`.
  Cada um recebe só os links do seu prefixo; o `AppLink` continua recebendo todos.
- Núcleo: `check_scheme` (o prefixo válido e fora dos do sistema), a fila dos links que abriram
  o app (`push_startup_link`/`take_startup_links`) e o `LinkService` com `register_scheme`,
  `unregister_scheme`, `scheme_registered`, `receive_links` e `can_register_schemes`.
- Windows (`aylis-platform-winit/src/url_scheme.rs`): o registro em
  `HKCU\Software\Classes\<prefixo>` (o `unregister` só tira se for deste executável) e o pipe
  `\\.\pipe\aylis-links-<executável e usuário>` (`PIPE_REJECT_REMOTE_CLIENTS`; a próxima
  instância do pipe nasce antes de a atual ser lida). Conferido de verdade com o exemplo
  `url_scheme`: com o app aberto, dois links chegaram nele e a cópia nova fechou (um processo
  só); com o app fechado, o link abriu o app e chegou no `on_link`.
- Publicação: os prefixos dos `UrlScheme` ligados nos forms (`publish::url_schemes`) vão para o
  `.msi` (um componente com `ForceDeleteOnUninstall`), o NSIS e o Inno (registram e tiram na
  desinstalação) e para o manifesto do Android (um filtro de intent cada). O `.msi` só foi
  conferido como texto (o WiX não está nesta máquina); o NSIS, o Inno e o Android, por teste.
- Testes: `crates/aylis-controls/tests/url_scheme.rs` (4),
  `url_schemes_are_registered_and_removed_by_the_installers` e o do manifesto do Android.
- A conferir: o teste real rodou de dentro do Claude Desktop (pacote MSIX); o registro foi
  visto pelo `reg` e pelo `Start-Process`, mas falta ver de fora (o navegador chamando o link).
- Falta: macOS e iOS (`CFBundleURLTypes` no `Info.plist`), Linux (o `.desktop` com
  `x-scheme-handler`) e o navegador (`registerProtocolHandler`, só `web+`) na publicação; com o
  app aberto, o repasse para a cópia aberta só existe no Windows e no Android.

**WMI pela API nativa (12/11/2026)**

Pedido do autor: um componente que fale com o WMI do Windows pela API nativa (o `OleObject`
chega só à API de script, sem laço sobre o resultado nem eventos). Feito, sem commit; falta o
autor conferir (roteiro 126):
- `WmiQuery` (System, `aylis-providers`, só Windows no `CLASS_PLATFORMS`): `namespace`
  (`root\cimv2`), `query` (WQL), `data_set` (um `ClientDataSet`), `timeout` (s) e `watch`;
  `busy`, `row_count`, `result` (JSON) e `last_event` só leitura; métodos `execute` (espera),
  `execute_async` (`on_done`/`on_error`), `get_value(consulta, propriedade)` e
  `get_json(consulta)`; eventos `on_done`, `on_event` e `on_error`. Sem campos, o dataset ganha
  um `Field` por coluna, no tipo do CIM (64 bits como inteiro, `CIM_DATETIME` como data e hora
  local, listas como memo em JSON); com campos, cada um recebe a coluna do mesmo nome.
- Núcleo: `WmiService` (assíncrono: o fim pelo `done`, os eventos pelo `listener`, de outra
  thread; o componente leva à tela pelo `post_to`), `WmiValue`, `WmiTable`, `WmiWatch` (para ao
  ser largada), `wmi_namespace` (recusa outro computador), `wmi_datetime` e `wmi_select_list`
  (as colunas na ordem do `SELECT`).
- Windows (`aylis-platform-winit/src/win_wmi.rs`, feature `Win32_System_Wmi`): `IWbemLocator` →
  `ConnectServer` → `CoSetProxyBlanket` (só na conexão; nada global do processo) →
  `ExecQuery`, numa thread com o COM multithread e as conexões guardadas por namespace; cada
  assinatura tem a thread dela (`ExecNotificationQuery`, esperando em fatias de 1 s).
- Só consulta: nada de métodos do WMI (`Win32_Process.Create` abriria programas) nem de outra
  máquina. O `watch` só liga com o app rodando, e o timer do componente existe só enquanto a
  assinatura muda (parado, nenhum).
- Testes: `crates/aylis-providers/tests/wmi.rs` (8, com um WMI falso: dataset com e sem
  campos, JSON, assíncrono vindo de outra thread, eventos, recusas, designer) e, no
  `aylis-platform-winit`, o WMI real (5: `Win32_OperatingSystem` com as colunas na ordem e os
  tipos, `Win32_Process` com `*`, erros, assinatura e um processo novo chegando como evento).
- Como no `OleObject`: o `aylis-project` não compila agora por uma mudança de outra sessão
  (`ImageCacheLimits.layers_mb`), então o `aylis-tool update-catalog` rodou pelo exemplo do
  `aylis-remote` (o mesmo que ele chama), o que também trouxe as mudanças de outras sessões no
  catálogo (`OleObject` renomeado, `UrlScheme`, `Clipboard`, `Form.layer_cache`); a página da
  documentação (`componentes/WmiQuery.md` e o `indice.json`) foi escrita à mão, no formato do
  gerador, com o texto de `docs/componentes/WmiQuery.md`. Não rodaram a suíte inteira nem os
  testes da IDE (o das traduções dos rótulos); rodaram os do componente, do serviço, das
  mensagens, do catálogo e de `crates/aylis/tests/classes.rs`, mais o wasm e o Android.
- Falta: exemplo na galeria (componente não visual, como o `OleObject`).

**Automação OLE (12/11/2026)**

Pedido do autor: um componente para falar com o Excel e outros programas por OLE/ActiveX; a
automação "pode fazer" (embutir controle ActiveX no form ficou de fora, o porquê no histórico).
Feito, sem commit; falta o autor conferir (roteiro 123):
- `OleObject` (System, `aylis-providers`, só Windows no `CLASS_PLATFORMS`): `prog_id`,
  `use_running`, `call_timeout` (ms) e `opened`; métodos `open`, `release`, `get_value`,
  `get_number`, `set_value`, `call`, `get_values`/`set_values` (matriz JSON) e `put_data`
  (títulos e registros de um `DataSource` ou `Export` numa chamada só, pelo `Resize` do Excel).
  Os caminhos são os do VBA, só com nomes e literais (`aylis_providers::ole::parse_path`).
- Núcleo: `OleService`, `OleValue` (com `Date` e `Array`), `allow_ole`/`ole_allowed` (o Office
  vem permitido; outro programa só pelo código do app) e `ole_date`/`ole_date_parts`.
- Windows (`aylis-platform-winit/src/win_ole.rs`): `IDispatch` numa thread própria com o COM em
  STA; a tela espera pelo canal com o `call_timeout`, sem processar mensagens. VARIANT para
  texto, números, datas, booleanos, objetos e SAFEARRAY de 2 dimensões; os erros do Excel numa
  célula (`#N/A`, `#DIV/0!`...) voltam como texto; o `EXCEPINFO` vira a mensagem.
- Testes: `crates/aylis-providers/tests/ole.rs` (7, com um programa falso: caminhos, lista de
  permitidos, um `set` só para 500 registros, largar ao liberar) e, no `aylis-platform-winit`,
  o COM real com o `Scripting.Dictionary` do Windows (texto com acento, data, matriz 2D de ida e
  volta, erro do objeto) e a recusa do `WScript.Shell`.
- Falta: esta máquina não tem o Office, então o `excel_round_trip` (`#[ignore]`) não rodou de
  verdade: a ordem linha × coluna da matriz no Excel e o `Resize` do `put_data` esperam o
  roteiro 123 numa máquina com o Excel. A tela espera enquanto o programa responde (não há
  versão assíncrona). O backend remoto continua podendo comandar o Excel da máquina (decidido:
  basta a lista de programas).
- Na mesma rodada, o `aylis-tool update-catalog` também gravou no catálogo o `Clipboard` e o
  `UrlScheme` (de outras sessões), que faltavam nele.
- A página da documentação (`componentes/OleObject.md` e a entrada no `indice.json`) foi
  ajustada à mão, no formato do gerador, com o texto de `docs/componentes/OleObject.md`: o
  `aylis-tool docs` não rodava de novo porque o `aylis-project` estava sem compilar por uma
  mudança de outra sessão (`ImageCacheLimits.layers_mb`). Pelo mesmo motivo não rodaram a suíte
  inteira nem os testes da IDE (o das traduções); rodaram os do componente, do serviço, do
  catálogo e de `crates/aylis/tests/classes.rs`, mais o wasm e o Android dos três crates.

**A placa de vídeo de cada janela (11/11/2026)**

Pedido do autor: uma janela arrastada de um monitor da placa dedicada para um da integrada
trava (o Windows copia cada quadro entre as placas). Veio num patch escrito sem compilar;
analisado, aplicado e corrigido em 11/11 (sem commit; roteiro 104). Tudo em
[gpu-por-monitor.md](gpu-por-monitor.md):
- `GpuChoice::FollowMonitor` (`monitor`), no projeto e na propriedade nova `gpu` do `Form`;
  o evento `on_gpu_change` e os métodos `gpu_adapter()`/`gpu_adapters()`.
- No Direct2D, a janela nasce na placa do monitor e troca ao mudar de monitor
  (`HwndTarget::follow_monitor`, `rebuild` soltando a swap chain antiga antes).
- Corrigido do patch: arrastando, troca ao soltar (`WM_EXITSIZEMOVE`), não no meio; o
  `WM_DISPLAYCHANGE` confere de novo; a lista de placas só sai quando pedida; a janela fechada
  sai do `Ui`; o comentário do `store_placement` voltou ao lugar; "placa" traduzida como placa
  de vídeo, não "card".
- Testado numa placa só e dois monitores, com `AYLIS_GPU_SWITCH=warp` (alterna com o WARP):
  5 trocas em 6 movimentos, na swap chain e no DirectComposition (exemplo `gpu_monitor`).
- Falta: o arraste com o mouse e a máquina com as duas placas
  ([testes-em-dispositivos](testes-em-dispositivos.md)); antes, ver se um app na CPU também
  trava (aí a causa não é a cópia).
- Linux (pedido do autor: "preciso que funcione no linux também"), feito em 11/11 no Wayland
  (`gpu_linux.rs`): o feedback do `linux-dmabuf` v4 diz a placa do monitor da janela, e ela
  passa a desenhar num `EGLDisplay` daquela placa (`EGL_EXT_explicit_device`), com o Skia
  refeito; a `gpu` do form com o nome de uma placa também vale por janela. No X11 não há o que
  o app fazer (o servidor desenha a tela toda na placa principal). Testado no WSL (Radeon pelo
  D3D12 e llvmpipe): a troca de contexto e o quadro depois dela; o WSLg não tem o feedback v4,
  e o `GALLIUM_DRIVER` força o D3D12 nas duas, então falta um Linux com duas placas.
- Não começado: um form ocupando vários monitores (uma janela por monitor, todas como um form).

**Pedido de 11/11/2026**
- [ ] **Uma pasta de compilação para todo o Rust da máquina:** "armazenamento está sendo o
  maior problema do rust, e o aylis deveria corrigir isso" — "por padrão é pra tudo ir pra um
  só lugar", e quem quiser desmarca. Feito em 11/11 (sem commit; roteiro 106):
  - `aylis_tools::cargo_config`: a IDE grava no `config.toml` global do cargo (`CARGO_HOME`
    ou `~/.cargo`) o `build.target-dir` com a pasta das Opções e o
    `profile.dev.package."*".debug = false`, ao abrir e no OK das Opções. Cada chave leva o
    comentário "Gravado pelo Aylis"; só essas são atualizadas ou tiradas, e uma chave que o
    autor escreveu vence (o arquivo é editado com o `toml_edit`, que já vinha no Cargo.lock).
  - Opções → Rust e Python: "Usar esta pasta em todo o Rust desta máquina", marcada por
    padrão (`global_cargo_config = off` no `tools.json` desliga).
  - Corrigido: o `.cargo/config.toml` do projeto levava só a pasta, e o cargo do terminal (e o
    rust-analyzer) compilava as dependências com o debug, ao lado das da IDE. Agora leva o
    mesmo perfil (`process::DEPENDENCY_DEBUG`).
  - Medido (`medicoes/debug_dependencias.json`, o OlaNome fora do repositório): sem debug nas
    dependências, o `target/` cai de 2.722 para 1.328 MB e a compilação limpa de 118 para 60 s;
    com os perfis misturados na mesma pasta, ela ia a 4.012 MB.
  - O sccache foi descartado pelo autor: acelera, mas não diminui o disco.
  - Com uma pasta só, dois projetos com executável de mesmo nome gravam o mesmo
    `target/debug/<nome>.exe`, e um `cargo clean` em qualquer projeto limpa a pasta de todos.
  - Ativado na máquina do autor em 11/11. "Seguir a pasta única": o repositório do Aylis
    também compila em `%LOCALAPPDATA%\aylis\target`. O `default_target_dir` não tem mais o
    caso "rodando do repositório"; o instalador, o `bench` e o teste `apps` acham os
    executáveis por `aylis_tools::cargo_target_dir` (o `cargo metadata`) ou pelo
    `CARGO_TARGET_TMPDIR`. Falta: a ponte do MCP do Claude Desktop
    (`claude_desktop_config.json`) ainda aponta para `aylis\target\debug\aylis-tool.exe`,
    e o Claude Desktop aberto desfaz a troca.
  - 12/11: a pasta era única só no nome. O Claude Desktop é um pacote MSIX, e tudo o que ele
    abre (as sessões, o `cargo`, a IDE aberta pelo MCP) gravava o `%LOCALAPPDATA%\aylis\target`
    numa cópia privada (`%LOCALAPPDATA%\Packages\Claude_*\LocalCache\Local\aylis\target`, 65
    GB); de fora do Claude, a pasta real estava vazia, e uma compilação de fora começaria do
    zero noutra pasta. Decisão do autor: a pasta padrão passa a ser
    `%USERPROFILE%\.aylis\target`, fora do `AppData` (`default_target_dir`), sem conversão da
    pasta de antes (só a máquina do autor a usava). Na máquina do autor, os 65 GB foram movidos para a pasta nova (o mesmo disco: uma
    renomeação), e o `config.toml` do cargo, o `tools.json` (as duas cópias) e a ponte do MCP
    do Claude Desktop (trocada com ele fechado, por um script aberto pelo WMI) apontam para ela.
    O teste `visual` da IDE gravava as falhas em `<repositório>/target/visual`; agora, no
    `CARGO_TARGET_TMPDIR`. Ainda na cópia do Claude: o `mcp.json`, os segredos (`dev-key`,
    `dev-secrets`), o `trusted.json` e as ferramentas baixadas (`tools`), que uma IDE aberta fora
    do Claude não vê.
- [ ] **Compilar com o app aberto em outro lugar:** "O compilador do Aylis não aceita compilar
  enquanto qualquer NoOBS.exe estiver aberto, inclusive o instalado." O autor: não é para ser
  assim. O Aylis deve conferir se o executável que a compilação vai gerar está em uso e não pode
  ser substituído, e não se há um processo com aquele nome rodando: pode haver mais de um na
  máquina, e o que roda pode não ser o da pasta de saída. Feito em 11/11 (falta o autor
  confirmar; sem commit): antes de compilar, `aylis_tools::run` confere os executáveis que a
  compilação grava (o do `target` e o da pasta de saída) abrindo-os para escrita; um em uso
  recusa (ou, com "Encerrar o app aberto", encerra só os processos daquele arquivo, pelo
  caminho). Outra cópia do app, de outra pasta, não atrapalha mais. O Parar sem nada rodando na
  IDE encerra o app do projeto pelo executável (o L-20 abaixo). Teste:
  `only_the_written_exe_counts_as_busy` (uma cópia do ping.exe rodando).

**Relatório do MCP na migração do InsightDB (11/11/2026)**

O agente que refez a página inicial do InsightDB (monitoramento de banco, de web para o Aylis) só
pelo MCP anotou as lacunas (L-01 a L-28). Pedido do autor em 11/11: "pode colocar e já começar a
fazer". Os números são os do relatório.

Feitos em 11/11 (falta o autor confirmar; sem commit; roteiro 102):
- **L-24:** a causa era o `auto_size`, que regravava a altura a cada arrumação e zerava a
  referência das âncoras; os setters só zeram quando o valor muda, e a medida do layout
  (`set_measured_height`/`width`) mantém a referência. Teste `aylis-controls/tests/anchors.rs`
  (falha com o comportamento antigo).
- **L-01:** o caminho `new_form` → `set_layout` → `save` grava no form novo (teste); o que levava ao
  form errado é um layout com a raiz de outro form (copiado do principal), agora recusado com a
  explicação. **L-03/L-18:** `new_form` com `file` e a ferramenta `move_form`
  (`Project::move_form`: o JSON, as variantes e o fonte de cada linguagem; a aba aberta acompanha).
- **L-13/L-27:** o `target` por nome que está em mais de um lugar é recusado com os caminhos (o
  clique ia para o Panel de outro frame; o clique num Panel funciona, com teste). **L-17:** `rect`
  em cada nó do `tree` e o comando `rect`. **L-11:** o form de um Frame recebe o `on_show` junto
  com o de fora. **L-20:** o Parar sem trabalho na IDE encerra o app do projeto pelo executável.
- **L-05/L-14:** `designer_snapshot` e `compare_image` (a referência em PNG; a diferença em % e
  em imagem). **L-06:** `search_icons`. **L-04/L-09:** o `class_info` (beta.2) lista os eventos do
  Form. **L-02:** a beta.2 já responde em 45 s.
- **L-22/L-26/L-25:** Chart com `value_format` (short, full, percent), `label_angle`,
  `title_align`, `title_bold`, `area_opacity`, `legend_position` (top_right, top_left, top,
  bottom), `stacked` (barras deitadas empilhadas), `colors` (as fatias não pegam mais a cor da
  série), `center_text` e `slice_labels` (percent, outside, none); DataGrid com `indicator` e
  `highlight_current`. Os padrões são o desenho de antes.
- **L-07:** Frame com filhos no JSON é recusado (o `set_layout` diz por quê), e o
  `add_component` num Frame também. **L-10:** tema sem `"dark"` decide pelo fundo. **L-12:**
  `#hex/NN`. **L-15:** Label com `auto_size` e `align` left/right encolhe ao texto (os soltos
  ficam com a largura desenhada). **L-23:** os itens aceitam quebra de linha, e o `class_info`
  diz o formato dos textos com estrutura. **L-16/L-21:** nas notas do `class_info` e nas
  instruções do MCP.
- Decididos pelo autor em 11/11 e feitos (falta confirmar; sem commit):
  - **L-19** ("fazer igual ao Delphi, o design junto do código" → "a subpasta do form se repete no
    código"): `forms/painel/inicio.json` → `rust/src/forms/painel/inicio.rs` (e
    `python/src/forms/painel/inicio.py`, com o `__init__.py` da pasta), módulo `painel_inicio`; os
    forms da raiz ficam como eram, e o fonte que estava direto em `src/forms` vai para a subpasta
    na geração. Nas outras 6 linguagens o fonte fica na pasta de sempre, com o nome único
    (`painel_inicio`); a subpasta nelas pede o gerador de cada uma (pacotes, namespaces).
  - **L-08** ("remover o handler somente se estiver vazio"): a geração tira o handler que nenhum
    evento liga se ele continua exatamente como a IDE gerou e nada no arquivo o chama
    (`codegen::drop_unused_stubs`, Rust e Python). E a pergunta "um handler que não é chamado é um
    método morto: a IDE ou o compilador valida?": o compilador não (os handlers são públicos), e a
    verificação do projeto passou a avisar o handler com código que nenhum evento chama (o form e
    as variantes).
  - **L-28** (`radius: [8, 8, 0, 0]`, e o negativo para o canto invertido, ideia do autor): Panel,
    Rectangle e Button com o raio por canto (cima-esquerda, cima-direita, baixo-direita,
    baixo-esquerda); negativo curva o canto para fora (`aylis_core::Path::corners`); `radius: 8`
    continua valendo para os quatro.
- Ficou: a cor no markup do Label (L-15, a segunda parte).
- **A explicação das propriedades** (pedido de 11/11: "o mcp consegue explicar as propriedades
  como funcionam, o negativo do radius por exemplo"; decidido: a explicação fica na documentação,
  tirada do comentário `///` do campo no código, mais um `docs/componentes/<Classe>.md` opcional, e o
  MCP a devolve sob demanda, só o essencial). Feito em 11/11 (sem commit): a página de cada
  componente ganhou a coluna "O que faz" (`reference::source_field_docs`, pelas camadas da classe e
  o `ControlBase`), o `.md` escrito à mão entra no fim da página, e a ferramenta `component_docs`
  (com `property`, só a linha dela e a seção `### propriedade` do texto à mão). Comentários novos e
  curtos nos campos do `ControlBase` (`align`, `anchors`, `margins`, `padding`, `left`...) e no
  `radius`. Ainda sem texto: as propriedades óbvias (`text`, `items`, `value`, `min`, `max`...) e
  as classes cujo tipo no código tem outro nome (o `Image` é o `ImageView`).
- **Renomear embutido no MediaCard** (pedido de 11/11, do NoOBS: "o card ganha uma propriedade
  como editable_title e cuida sozinho do campo de edição, dispara um on_rename com o nome novo, e o
  app só grava"). Feito em 11/11 (sem commit; roteiro 103): `editable_title` (o clique no nome abre
  o campo; a miniatura continua no `on_click`), `on_rename` com o nome novo (já gravado no `title`;
  vazio ou igual não dispara), Esc cancela, e o método `rename()` abre pelo código (o F2). O campo
  é uma camada sobre o form que segue o card (como o calendário do DateEdit). No núcleo: a camada
  que fecha com o clique fora perde o foco antes de sair, para o campo gravar. Teste
  `aylis-controls/tests/media_card_rename.rs`; texto em `docs/componentes/MediaCard.md`.

- Altos:
  - [ ] **L-01.** Depois do `new_form`, o designer ficou fora de sincronia: `set_layout`/`save`
    gravaram no form errado.
  - [ ] **L-02.** O `build` com espera estourou o prazo da chamada e a IDE fechou (na 0.2.0-beta.2
    o `build` responde em até 45 s; falta conferir que a IDE não fecha).
  - [ ] **L-18.** Não há como mover ou renomear o arquivo de um form; o "Salvar como…" abre a janela
    do Windows, fora do alcance dos `dialog_*`.
  - [ ] **L-24.** `anchors` à direita/embaixo não acompanham o pai que muda de tamanho (a célula do
    GridLayout, a janela em tela cheia).
- Médios:
  - [ ] **L-03.** `new_form` não escolhe o nome do arquivo nem a subpasta.
  - [ ] **L-04.** Consulta das propriedades, enums e eventos (o `class_info` da beta.2; conferir).
  - [ ] **L-05.** Captura do designer (a do app é o `snapshot` da beta.2).
  - [ ] **L-11.** O form dentro de um Frame não recebe `on_show`.
  - [ ] **L-13.** `app_command click` num Panel não dispara o `on_click` dele.
  - [ ] **L-14.** Comparação visual com uma imagem de referência.
  - [ ] **L-17.** `app_command` não dá a posição e o tamanho calculados.
  - [ ] **L-19.** O código Rust dos forms fica sempre em `rust/src/forms/<nome>.rs`, sem subpastas.
  - [ ] **L-20.** Depois de reabrir a IDE, o `stop` não alcança o app aberto antes, e o build falha
    com "o app.exe já está aberto".
  - [ ] **L-22.** Chart: eixo só no formato curto, rótulos sem girar, título fixo à esquerda e em
    negrito, área com a cor da linha.
  - [ ] **L-26.** Chart: barra deitada empilhada; pizza/rosca com a cor da fatia pela série, o total
    sempre no meio e o nome da fatia sem ir para fora.
- Baixos:
  - [ ] **L-06.** Busca de ícones. **L-07.** O Frame aceita filhos, e excluir o Frame move os filhos
    para o form. **L-08.** Tirar um evento não tira o handler do código. **L-09.** Os eventos do
    Form (`on_show`) fora do catálogo. **L-10.** `ui.theme().dark` começa `false` com o tema escuro.
    **L-12.** `cor/15` só vale para nome de cor, não para hex. **L-15.** `auto_size` do Label com
    `align: left` não encolhe; o markup sem cor. **L-16.** A borda padrão do Panel aparece como
    `stroke_width: null`. **L-21.** Os tipos gerados têm o nome dos controles do `aylis::prelude`.
    **L-23.** `ComboBox.items` separa por `;` e o catálogo só diz texto. **L-25.** DataGrid sempre
    com a coluna indicadora e a linha atual; legenda do Chart fixa no alto à direita. **L-27.**
    `app_command` por nome pega o primeiro do app inteiro. **L-28.** Raio por canto e canto
    invertido no Button/Panel (o agente resolveu com um Path; o autor aprovou o resultado).

**Relatório do MCP na migração do RTSPlayer (10/11/2026)**

O agente que migrou o cliente do RTSPlayer só pelo MCP anotou o que travou
(`RTSPlayer/relatorio-mcp-aylis.md`). Os números são os do relatório.

Feitos todos em 10/11 (pedido do autor: "faça todos"; falta o autor confirmar; sem commit;
roteiro 101):
- **14:** o `call_named` sobe de escopo em escopo até o form que tem o handler (a linha clonada
  é um escopo sem os handlers). **16:** a causa era o carregador do JSON: `bindings` é chave
  reservada (as ligações a campo, um objeto) e a propriedade de texto da lista era pulada; agora
  o texto vale como propriedade (no `load` e no hot reload). **11:** `bindings` aceita
  `componente.propriedade=campo` (convertido pelo tipo: booleano, número, cor); a lista ganhou
  `index_of(componente)` (o registro da linha de um clique) e `part(índice, componente do
  modelo)` (a peça da linha pelo acesso tipado), no `invoke` (valem em todas as linguagens).
  Testes em `aylis-dbcontrols/tests/virtual_list.rs`.
- **12:** a geração guarda as classes de cada form (`generated/components/classes.json`) e tira o
  acesso das que nenhum usa mais (`GENERATOR_VERSION` 8); o `.aylisproj` aceita `"features":
  ["media"]` para o que o código usa sem componente. **4:** o app excluído do workspace de cima
  também ganha o `[workspace]` próprio. **5:** `Project::bin_name` (o primeiro `[[bin]]`) no
  `--bin` do Executar, do depurar e do WSL.
- **13:** `margins`/`padding` (texto `"12, 4, 12, 0"`) e `anchors` (`"left,top"`) no catálogo e
  no acesso tipado; o `set_prop` aceita o texto. **8:** o gancho grava `.claude/.gitignore` e a
  resposta do `new_project` avisa. **10:** Projeto → Novo fonte… e o `edit_code` com `old` vazio
  num arquivo que não existe criam o fonte (dentro da pasta do código, fora do `generated/`).
  **1:** `new_project` com `here` cria direto numa pasta que já tem outras coisas
  (`aylis_project::create_at`), e as mensagens dizem que mover o projeto conta como de fora.
- **2:** `set_layout` confere tudo antes (`aylis_json::validate`) e devolve todos os erros de
  uma vez; `dry_run`, `layout_file` e `patch_layout` (troca um componente pelo nome). **3:**
  `class_info` (propriedades com tipo, valores dos enums e exemplos, eventos e métodos); a sombra
  pelo nome do tema (`"md"`) vale no form. **15:** o `snapshot` do app (com `target`, só um
  componente) volta como imagem do MCP; `tree` com `depth` e `visible_only`.
- **6 e 17:** `build`, `run`, `run_without_debug` e `debug` esperam até 45 s (`max_wait`) e
  respondem "ainda compilando"; `build_wait` espera o resto. O `run` devolve os erros da
  compilação, ou que o app está rodando, ou que abriu e caiu. **7:** sem a chamada passar do
  prazo do cliente, a ponte não é reiniciada no meio (a causa provável); `ide_list` mostra também
  as IDEs com o MCP desligado. **9:** resolvido pelo `project` em cada chamada (roteiro 100).

- MCP — o vínculo da conversa com a IDE:
  - [ ] **7.** Depois do `Request timed out` de um `build`, a IDE com o projeto sumiu da conversa e
    a seguinte abriu outra, vazia. Suspeita a conferir: a IDE nasceu presa ao job do cliente
    (`in_kill_on_close_job`), e o cliente reiniciou a ponte depois do prazo.
  - [ ] **9.** Numa conversa retomada, a IDE do projeto aparecia sem a marca de fixada. Em 10/11 a
    ponte passou a escolher a IDE pelo `project` de cada chamada (o Claude Desktop divide uma ponte
    entre conversas); falta conferir se isso resolve (roteiro 100).
  - [ ] **6.** A primeira compilação estoura o prazo do cliente MCP: o `build` responder "ainda
    compilando" antes do prazo, e uma ferramenta para esperar o resultado (`build_wait`).
  - [ ] **17.** `run`/`run_without_debug` respondem "Executando…" na hora, e a compilação que falha
    depois não é avisada: esperar a compilação e devolver os erros como o `build`; avisar também o
    app que abre e cai.
- MCP — forms e classes:
  - [ ] **2.** `set_layout` para no primeiro erro (um layout de 24 KB reenviado cinco vezes):
    devolver todos os erros, um `dry_run`, o layout por arquivo e um `patch_layout` por componente.
  - [ ] **3.** Não há como saber pelo MCP as propriedades, os valores de enum e os eventos de uma
    classe: um `class_info` (`trimming: "ellipsis"`, `radius` no `Label`, `on_double_click` e a
    sombra `"md"` só no tema foram tentativas às cegas).
  - [ ] **15.** O `app_command` não tira captura da janela, e o `tree` de um app médio passa de
    130 mil caracteres: um `screenshot` e o `tree` com filtro.
- Projeto e geração:
  - [ ] **1.** `new_project` não cria numa pasta que já existe; mover o projeto criado conta como
    "outra máquina" (modo restrito). Criar em pasta existente, ou a mensagem explicar.
  - [ ] **4.** O app dentro de outro workspace do Cargo não compila (o `aylis` herda o
    `rust-version` do workspace errado): o gerador escrever um `[workspace]` vazio no
    `Cargo.toml` do app.
  - [ ] **5.** O `build` supõe que o executável tem o nome do pacote: ler o `[[bin]]`.
  - [ ] **8.** O `new_project` grava `.claude/settings.local.json` (o gancho do guarda, com caminho
    absoluto) sem dizer: avisar na resposta e pôr no `.gitignore` do projeto.
  - [ ] **10.** Não há como criar um fonte novo (nem pelo MCP nem pelo menu): `new_source`, ou o
    `edit_code` criar com `old` vazio, e o item de menu.
  - [ ] **12.** Gerar o código tira a feature `media` quando nenhum form usa classe de mídia, mas
    deixa o acesso gerado do `VideoPlayer` (o build quebra no gerado); o código do usuário que usa
    `aylis::media` também perde a feature. Apagar as classes sem uso e o projeto poder declarar
    features (`"features"` no `.aylisproj`).
  - [ ] **13.** O acesso tipado não tem `margins`, `padding` nem `anchors`.
- Framework (`DataRepeater`):
  - [ ] **14.** Os eventos de dentro de uma linha não chegam ao handler do form: a linha é um
    escopo de nomes, e o `call_named` procura no "form" vazio (`form_of` para na linha). Subir até
    o form de verdade, como o `instance_of`.
  - [ ] **16.** Os `bindings` não preenchem as linhas (o `count` e o `on_bind` vêm certos, o texto
    fica o do modelo).
  - [ ] **11.** Os `bindings` só preenchem `text`/`source`/`value`/`checked`; a linha só é achada
    pelo nome em texto no `on_bind`; o clique dentro da linha não diz o registro. Propriedade nos
    `bindings`, acesso tipado à linha e o índice do registro nos eventos.

**Pedidos de 09/11/2026**
- [x] O instalador não leva os exemplos IBGE e ERP (grandes, só de demonstração); o `Cargo.toml`
  instalado sai sem eles entre os membros.
- [ ] **Compilador separado da IDE:** um executável que gera o app passando pelas validações
  (gerar o código, verificar o projeto, compilar), usado pela linha de comando e pela IDE; a IDE
  deixa de gerar e verificar dentro do próprio processo e passa a chamar esse executável e mostrar
  o progresso e as mensagens dele. Hoje a geração e a verificação rodam numa thread da IDE
  (`jobs.rs`, `aylis_project::check`) e só o `cargo`/as ferramentas das linguagens rodam fora.
  Decidido pelo autor em 09/11: um subcomando do `aylis-tool`. Feito em 09/11 (falta o autor
  confirmar; sem commit; roteiro 97): `aylis-tool build <projeto> [--target t] [--release]
  [--no-generate] [--no-compile] [--language l] [--json]` gera, verifica e compila, parando no
  primeiro erro (`aylis_tools::build`; compila o desktop em Rust, as outras plataformas e linguagens
  só geram e verificam, e compilam ao executar ou publicar). Com `--json`, uma linha por evento
  (`start` com o protocolo, `step`, `line`, `diagnostic`, `generated`, `issues`, `built`, `done`). A
  IDE chama o subcomando em outro processo no Compilar (Ctrl+F9, que agora também verifica), no
  preparo do Executar e do Publicar e no Depurar, e mostra os eventos nas MENSAGENS; o editor
  acompanha a geração pelo evento `generated` (o Ctrl+F9 e o Depurar geravam por trás do editor).
  Interromper encerra o `aylis-tool` e, com ele, o cargo (`process::children_die_with_me`, um Job
  Object no Windows). Um `aylis-tool` de protocolo diferente é recusado; sem o `aylis-tool` ao lado
  da IDE (os testes) ou com um de antes do `build`, a IDE faz dentro dela, com um aviso. O
  `aylis-tool check` e o `build` registram o 3D (`aylis-3d` no `aylis-tools`), como o designer: o
  `check` acusava os forms com `Model3D`. Ficaram na IDE o "Gerar código" do menu e a geração do
  F12 (rápidas, e o F12 precisa do resultado na hora). Testes: o protocolo
  (`the_events_go_and_come_back_through_the_json`); à mão, um projeto novo compilado pelo
  subcomando e pelo `build_external`, e interrompido no meio de uma compilação release (nenhum
  cargo ou rustc sobrou).
- [ ] **Publicar no crates.io** (pedido do autor; os nomes `aylis`, `aylis-core`... e
  `aylis-accesskit-android` estavam livres em 09/11). Preparado em 09/11 (sem commit; falta o autor
  publicar): `cargo publish --dry-run` passou nos 27 crates, cada um empacotado e compilado a partir
  do pacote. O que mudou: (1) o `emoji.tsv` foi para dentro do `aylis-controls-base`
  (`data/emoji.tsv`; o `aylis-tool emoji` grava lá); fontes, ícones e animações não vão nos crates
  (são copiados para o projeto), e cada crate tem menos de 1,5 MB; (2) a cópia do
  `accesskit_android` vai como `aylis-accesskit-android` 0.9.1 (a biblioteca continua
  `accesskit_android`; `third_party/accesskit_android/AYLIS.md`); (3) as 64 dependências por caminho
  ganharam `version = "0.1.0"` (subir a versão do workspace pede subir essas também); (4)
  `publish = false` na IDE, no `aylis-tools`, no `aylis-host`, no `aylis-capi` e no `aylis-migrate`
  (os demos, a IDE web e os exemplos já tinham); (5) as descrições com "Aylis" maiúsculo e sem
  Delphi/FMX/VCL (como no site, item 28). O repositório ficou público em 09/11 (com a release
  v0.1.0), e o `repository` está nos manifestos (`repository.workspace`; a cópia do AccessKit
  também aponta para o Aylis, onde o código dela fica); o dry-run passou de novo sem avisos. Para publicar (o token é do autor, e publicar é permanente: só dá para "yank"):
  `cargo login`, depois `cargo publish` (depois do commit) com os 27
  `-p`: `aylis-accesskit-android aylis-core aylis-3d aylis-build aylis-controls-base
  aylis-controls-charts aylis-controls-device aylis-controls-editors aylis-controls-industrial
  aylis-controls aylis-data aylis-dbcontrols aylis-devlink aylis-docview aylis-json aylis-markdown
  aylis-media aylis-skia aylis-web aylis-d2d aylis-platform-winit aylis-vector aylis-project
  aylis-providers aylis-webview aylis-remote aylis` (o cargo publica na ordem das dependências).
  Fica para depois: o gerador continuar apontando os projetos para `.aylis/sdk` (decisão do autor
  ao trocar para a versão publicada) e um README por crate (a página do crates.io sai vazia).

**Arrastar**
- [ ] Abas que saem e entram em janelas, como as do navegador (pedido em 06/11): arrastar a
  aba para fora cria outra janela, arrastar uma janela de fora para dentro também, com a
  imagem de prévia junto do mouse. Feito em 06/11 (falta o autor confirmar; sem commit) no
  `TabStrip`: puxada para fora da faixa, a aba vira um arrasto do núcleo com a prévia (o
  título e a miniatura do conteúdo, que no Windows segue o cursor fora da janela); solta noutra
  faixa do mesmo `drop_group` (em qualquer janela) entra no lugar marcado; de volta na sua,
  só muda de lugar; longe de todas, vira janela onde o mouse soltou. Com páginas
  (`TabStrip::add_page`) a faixa leva o conteúdo junto, abre a janela nova (faixa igual, os
  mesmos handlers, independente da outra) e fecha a que ficou vazia (`close_empty`); a janela
  de uma aba só vai inteira (solta longe, muda de lugar; solta numa faixa, junta-se a ela).
  Sem páginas, só os títulos e os avisos (`on_tab_drop` com `last_drop`, `on_tear_off`, como a
  IDE já usa). No núcleo: os ganchos `drop_accepts`/`drop_hover`/`drop_leave`/`drop_receive`/
  `drag_ended` no `Component`, `Ui::begin_drag_now`, `Ui::screen_pointer`, `Ui::copy_events`,
  `Ui::set_form_owner` e `WindowCommand::MoveTo`. Num projeto, a janela nova é outra
  instância do mesmo form (`create_form_instance`, os handlers por nome falam com ela); sem
  projeto, um form com uma faixa igual. Testes em `crates/aylis-controls/tests/tab_strip_drag.rs`
  e no exemplo `examples/Abas` (o teste `arrastar`). Depois (pedido em 06/11): dentro da
  faixa, o arrasto é como o do Edge e do Chrome: a aba pega segue o mouse (sem sair das
  pontas), troca de lugar quando o meio dela passa o da vizinha, as vizinhas deslizam até o
  lugar novo e a solta assenta no seu, no tempo do quadro (`Component::frame`; o desvio cai
  pela metade a cada ~40 ms). Teste `dragging_follows_the_mouse_and_the_neighbors_slide`.
  Depois (pedido em 06/11): fechar pelo × (ou pelo clique do meio) com o mouse na faixa
  mantém a largura das abas, e o × da próxima vem para debaixo do mouse; quando o mouse sai
  da faixa, elas vão até a largura natural deslizando. Teste
  `closing_keeps_the_widths_until_the_mouse_leaves`. Depois (pedido em 06/11): a última aba
  pode ser empurrada para a direita além do lugar dela, até a ponta da faixa, e o "+" vai
  junto (fica sempre depois da aba mais à direita na tela); ao soltar, os dois voltam
  deslizando. Teste `pushing_the_last_tab_right_takes_the_plus_along`.
  Depois (pedido em 06/11): a IDE usa essas abas em cima do designer e do código
  (`panels::tabs`, no lugar dos `Rectangle` refeitos a cada atualização): uma `TabStrip` só,
  atualizada por `TabStrip::set_tabs` (que não mexe quando nada mudou, para o deslize e a
  largura presa continuarem). Arrastar troca a ordem e ela fica guardada enquanto a IDE está
  aberta (os forms pelo nome, o código pelo caminho; as abas novas vão para o fim); a aba de
  código puxada para fora vai para a janela de código.
  Fora deste passo: o `TabControl` (abas sublinhadas, de páginas `TabItem`) não ganhou o
  arrastar; a janela não acompanha o mouse durante o arrasto (como no Chrome), quem acompanha
  é a prévia; juntar todas as abas de uma janela arrastando a faixa vazia.

**IDE**
- [ ] Recuo do Object Inspector (pedido em 09/11: "as propriedades deveriam estar identadas à
  frente do agrupador, e o agrupador parece um espaço à frente do alinhamento lateral"). Feito em
  09/11 (falta o autor confirmar; sem commit): o nome do grupo na borda do painel, alinhado com a
  busca e com o título do componente, e as propriedades 16 px à frente (`GROUP_INDENT` em
  `inspector.rs`; os lados de `margins`, mais 14). A aba Eventos, sem grupos, ficou como estava.
  Referência `designer-windows.png` regravada (a de Linux, no Docker).
- [ ] Aparelhos do seletor de vistas por categoria (pedido em 06/11): o item "Vistas…" vira
  "Telas…" e os aparelhos saem da lista plana para um menu com submenus — PC (HD, Full HD,
  2,5K, 4K, 8K), Tablet (iPad, iPad mini/Air/Pro, Galaxy Tab, Pixel Tablet, Xiaomi Pad) e
  Smartphone › Android / iOS. Feito em 06/11 (falta o autor confirmar): `DeviceOs` em
  `aylis_core::devices` e a lista nova; na IDE, `screens_menu` em `app.rs`. O editor das vistas
  ficou como "Editar vistas…" e o aparelho em uso aparece no combo.

**Android (com o ZenFone conectado)**
- [ ] Fase 1, feito e conferido no ZenFone em 01/11 (falta o autor confirmar e desinstalar
  `TesteAparelho` e `Estresse3D` do aparelho):
  - o limite de 30 quadros/s: 33,3 ms entre quadros;
  - a depuração: `aylis-tool debug-android` põe o `lldb-server` do NDK no app (depurável com
    `--debug` ou `dev`), grava "aylis: anexar (Android)" para o CodeLLDB e, com `--lldb`, abre
    o `lldb` do NDK já anexado; o ponto de parada no handler parou na linha certa;
  - a abertura da IDE com o lingote em `aylis_3d::ingot`;
  - "Executar → Depurar no aparelho" na IDE (projetos em Rust): o mesmo do `debug-android`,
    com o aparelho escolhido, e abre o VS Code.
- [ ] Lingote (pedido em 01/11, feito; falta o autor confirmar): a luz vermelha no pé vinha do
  chanfro (a tira fina usava o mipmap mais reduzido da brasa); o pé agora some no fundo no
  próprio modelo (paredes transparentes, alfa por vértice), como na abertura da IDE, que deixou
  de pintar o degradê por cima; o lingote é curto como no logo; a câmera de órbita do `Model3D`
  enquadra pela abertura mais fechada (em pé, a horizontal). Conferido no ZenFone, no
  navegador e no PNG da abertura.
- [ ] Câmera no `max` no ZenFone, feito e conferido em 01/11 (falta o autor confirmar):
  - os quadros vêm reduzidos ao tamanho da tela, convertidos em inteiros, em blocos e em
    várias threads (`camera::yuv420_to_rgba`), e o tamanho cheio fica só para a foto
    (`VideoDecoder::full_frame`);
  - na tela, de 6,5 para 18 quadros/s no `max` e de 25 para 32 no `medium`;
  - a foto sai em 3024x4032 em cerca de 1 s (antes, 20 s; agora com a compressão rápida do PNG);
  - a foto com a câmera desligada não devolve mais o último quadro;
  - no navegador, com a webcam de verdade (Edge): 640x480, 1280x960 e 1920x1080 em `low`,
    `medium` e `max`, a imagem mudando; a permissão negada agora aparece na própria
    `CameraView` ("o acesso foi negado"), em vez de ficar em "…".
  - Falta a câmera no Linux: passar a webcam USB para o WSL pede o `usbipd-win` (instalar,
    como administrador, e a câmera sai do Windows enquanto estiver anexada).
- [ ] Aparelhos na IDE: o autor ver a lista (com dois aparelhos, se tiver).
- [ ] `AppLink` (decidido em 01/11, feito; falta o autor confirmar): componente não visual
  (categoria System) com `open_url`, `open_app`, `share(texto, arquivos)` e `request(ação,
  dados)` (resposta em `on_result`, erro em `on_error`); `on_link` e `on_shared`.
  - Núcleo: o `LinkService` e a fila do que chega de fora (`platform::push_incoming`), que
    espera um `AppLink`: o app aberto pelo próprio link ainda não tem o form.
  - Android (conferido no emulador): `ACTION_VIEW`, pacote, folha de compartilhar,
    `startActivityForResult`; o provedor `AylisActivity$Files` para compartilhar arquivos (sem
    androidx); recebe link e compartilhar com o app frio ou aberto (`singleTask`, uma activity
    só). A publicação põe o provedor com o `AppLink` e o `SEND` com o `on_shared`.
  - Desktop: `ShellExecuteW` / `open` / `xdg-open`; o link na linha de comando vira
    `on_link`. Navegador: relativo na mesma aba, de fora noutra; `navigator.share`.
  - Falta: iOS (`UIApplication.openURL`, a folha de compartilhar), compartilhar arquivo no
    Android conferido com outro app, e compartilhar no desktop (não há folha do sistema).
    Decidido pelo autor em 05/11: compartilhar em todos os sistemas onde der (no Windows, a
    folha "Compartilhar" do sistema, o `DataTransferManager`; no macOS, o `NSSharingServicePicker`;
    no Linux, onde houver um portal).
- [ ] `BackgroundService` (feito; falta o autor confirmar): a lógica em `src/background.rs`
  (o gerador cria; `aylis_core::background`: uma thread e as duas filas `send`/`on_message`),
  rodando no Android num serviço em primeiro plano (`AylisActivity$Background`, notificação
  fixa, tipo certo no Android 14+), com `keep_alive` (`START_STICKY`, o receptor do boot e o
  pedido de isenção da bateria). Conferido no emulador: a contagem seguiu com a tela fechada,
  voltou depois de `kill -9` e depois de reiniciar o emulador. Guia em
  [docs/segundo-plano.md](segundo-plano.md) (listas dos fabricantes, push como reserva).
  Falta o iOS (os modos permitidos) e conferir no ZenFone (a lista da ASUS).
- [ ] Ligações e tela de bloqueio (feito; falta o autor confirmar): componentes `Phone`
  (`dial`, `call`, `state` e `on_state` pelo `TelephonyCallback`; com `dialer`, o
  `InCallService` `AylisActivity$Calls`, o pedido do papel de telefone, número de quem liga,
  `answer`/`reject`/`hang_up` e o histórico) e `LockScreen` (`show_when_locked`,
  `turn_screen_on`, `dismiss`); a `Notification` ganhou `full_screen`. A publicação põe as
  permissões e os filtros de discagem. Conferido no emulador: discar e o estado tocando e
  livre. Falta conferir o papel de telefone e a tela de bloqueio num aparelho.
- [ ] Sensores (feito; falta o autor confirmar): `Location`, `Battery`, `Network`,
  `MotionSensor`, `Compass`, `ProximitySensor`, `LightSensor` e `Vibration` (categoria
  "Device", `available` falso onde não houver), pelo `SensorService` do núcleo. Android:
  SensorManager, BatteryManager, ConnectivityManager, LocationManager e Vibrator (conferidos
  no emulador, com os valores mudando). Windows: bateria, rede e localização (respondeu, com
  50 km de precisão); Linux: `/sys`. Falta o iOS.
- [ ] Áudio no Android: o loopback pelo `AudioPlaybackCapture` (feito; falta conferir no
  ZenFone): a entrada `loopback:Android` no `AudioInput`, com a autorização do
  MediaProjection e o serviço `AylisActivity$Projection`. No emulador as amostras chegam
  (96 mil/s), mas em silêncio; no aparelho deve sair o som dos outros apps.
- [ ] `Nfc` (pedido em 01/11; não começou): componente não visual (categoria "Device",
  `available` falso onde não houver) para ler e gravar tags NDEF e dar o id da tag, com
  `on_tag` e `write(registros)`. Android: `NfcAdapter` em modo leitor (a publicação põe a
  permissão `NFC` e o `uses-feature`). iOS: Core NFC (ler; gravar no iOS 13+), com a
  permissão da Apple no app e só com o app aberto. Desktop: leitores USB pelo PC/SC
  (ACR122U). Navegador: Web NFC (só no Chrome do Android).
  Feito em 03/11 (falta o autor confirmar e conferir com hardware, em
  `docs/testes-em-dispositivos.md`): `aylis_core::ndef` (ler e montar NDEF, a memória das tags do
  tipo 2), o `NfcService`, o componente (`records` em texto, `text:`/`uri:`/MIME por linha);
  Android pelo `NfcAdapter` (Java na `AylisActivity`), desktop pelo PC/SC (feature `nfc`), web
  pela Web NFC. iOS ainda não (pede um Mac). Testado com serviço falso; aqui não há leitor USB.
- [ ] `Bluetooth` (pedido em 01/11; não começou): suporte completo num componente não visual
  (categoria "Device", `available` falso onde não houver). BLE: procurar aparelhos (filtro
  por serviço e nome, RSSI), conectar e desconectar, listar serviços e características, ler,
  gravar (com e sem resposta) e ser avisado das mudanças (`notify`/`indicate`), com os eventos
  de achado, conectado, desconectado, valor e erro. Clássico (SPP/RFCOMM, para impressoras e
  leitores serial): parear e trocar bytes onde o sistema deixa. Android pelo Java
  (`BluetoothLeScanner`, `BluetoothGatt`, as permissões `BLUETOOTH_SCAN`/`CONNECT`); Windows,
  Linux, macOS e iOS pelo `btleplug` (no iOS, só BLE); navegador pela Web Bluetooth (só
  Chrome, com o seletor do navegador). Hoje o aylis só tem a permissão `bluetooth` e a saída
  de áudio para fone ou caixa.
  Feito em 03/11 (falta o autor confirmar e conferir com hardware): o `BluetoothService`, o
  componente (procurar, conectar, características, ler, gravar, `subscribe`, o clássico com
  `open`/`send`/`close`; uuids curtos do SIG); Android pelo `BluetoothGatt`/`BluetoothSocket`
  (Java, operações GATT em fila), Windows/Linux/macOS pelo `btleplug` (feature `bluetooth`, só
  BLE), web pela Web Bluetooth. iOS ainda não. No Windows o adaptador é achado, mas nenhum
  aparelho BLE anunciava por perto (o exemplo do próprio `btleplug` também viu 0).
- [ ] `Gamepad` e `Joystick` (pedido em 01/11, feito; falta o autor confirmar com um
  controle de verdade): o `Gamepad` (categoria "Device") lê o controle `index` a cada quadro,
  com botões (`on_button_down`/`on_button_up` com o nome: `a`, `b`, `x`, `y`, `lb`, `rb`,
  `lt`, `rt`, `back`, `start`, `l3`, `r3`, o direcional e `guide`), analógicos, gatilhos,
  `dead_zone`, `on_connect`/`on_disconnect`, `pressed(nome)` e `rumble(força, ms)`. Desktop
  e navegador pelo `gilrs` (no Linux, o `libudev-dev` entrou no CI e no WSL). Android: a
  `NativeActivity` entrega a entrada ao winit, então os botões e o analógico esquerdo vêm dos
  eventos dele (separados pelo id do aparelho, que o Java lista) e o analógico direito e os
  gatilhos analógicos ficam de fora até o winit expor o eixo. Falta o iOS (GameController).
  O `Joystick` (categoria "Additional") é a alavanca na tela: `x`/`y` de -1 a 1, `dead_zone`,
  `spring`. Conferidos por teste e no PNG; nesta máquina não há controle conectado.

**Interface responsiva sem código (análise de 06/11/2026)**
- [ ] A IDE só grava na vista por largura `left`, `top`, `width`, `height`, `visible` e
  `align` (`Document::VIEW_PROPS`): outra propriedade mexida no inspetor dentro da vista
  (`font_size`, `padding`, `columns`...) ia para o base, em todas as larguras. Pedido em 06/11:
  qualquer propriedade mexida na vista vale só nela. Feito em 06/11 (sem commit; falta o autor
  confirmar): ao entrar na vista, o designer guarda o base de toda propriedade gravada de cada
  componente com nome e do próprio form (menos a posição e o tamanho do form, que são da
  prévia); o que ficar diferente vira mudança da vista. Mudança no próprio form vai com o nome
  vazio (`"": { "padding": 6 }`), que o núcleo e o `aylis-json` entendem.
- [ ] Vistas personalizáveis (pedido em 06/11): hoje a IDE só tem Celular (até 599) e Tablet
  (até 1023). Criar, renomear, apagar e mudar o limite de cada vista pelo designer. Feito em
  06/11 (sem commit; falta o autor confirmar): o seletor da barra mostra as vistas do form
  (Celular e Tablet sempre) e "Vistas…", que abre o editor (`viewsdialog.rs`): nome, largura e
  altura de/até, orientação, plataformas e as escalas; com desfazer (`Document::set_views`).
- [ ] A vista escolhida por mais que a largura (pedido em 06/11): altura, orientação
  (retrato/paisagem) e plataforma/tipo de aparelho, sem código. Feito em 06/11 (sem commit;
  falta o autor confirmar): `FormView` com `min_width`, `max_width`, `min_height`,
  `max_height`, `orientation` e `platforms` (windows, linux, macos, android, ios, web); vale
  uma vista por vez, a primeira da mais estreita para a mais larga cujas condições todas valem.
  O tipo de aparelho fica pela largura e pela plataforma (não há como saber "tablet" de outro
  jeito sem o sistema).
- [ ] Escala de fonte e de densidade por vista (pedido em 06/11): mudar o tamanho das
  fontes e os espaços do form inteiro numa vista, sem mexer componente a componente. Feito em
  06/11 (sem commit; falta o autor confirmar): `font_scale` e `spacing_scale` na vista (0,25 a
  4). O layout carimba as escalas nos controles do form; a fonte da pintura e a medida
  (`font_of`) saem escaladas, e `margins`/`padding` e o `gap` do `GridLayout`/`FlowLayout`
  também; as propriedades continuam com os valores do desenho. O espaçamento interno de cada
  controle (o do tema: a altura de um botão, por exemplo) não muda.
- [ ] `GridLayout` com um filho ocupando mais de uma coluna (o `colspan`; pedido em 06/11).
  Feito em 06/11 (sem commit; falta o autor confirmar): `column_span` em todo controle
  (categoria Layout); o que não cabe no resto da linha vai para a próxima, e mais que a grade
  tem ocupa a linha inteira (numa grade responsiva que encolheu).
- [ ] Ver por que a vista não muda um componente de contêiner (pedido em 06/11): se o motivo
  for forte, fica como está e o motivo fica escrito; se não, a vista passa a mover. Visto em
  06/11: não havia motivo registrado; a vista só foi modelada como "propriedade → valor", e o
  núcleo já move um componente dentro do mesmo form sem problema de nomes (`set_parent`).
  Feito (sem commit; falta o autor confirmar): as pseudo-propriedades `parent` (nome do
  contêiner; vazio: o form) e `index` na vista; ao sair dela, o componente volta ao lugar e à
  ordem; o `aylis-json` grava a árvore do base (`Ui::base_children`) mesmo com a vista ativa;
  no designer, arrastar para outro contêiner com a vista à mostra grava a mudança, e "Voltar ao
  base" devolve o lugar. Só componentes com nome.

**Componentes e projeto**
- [ ] Projeto de exemplo com relatórios mestre-detalhe, com campos de texto e números (pedido
  em 06/11, feito; falta o autor confirmar): `examples/Relatorios` (pedidos → itens e
  parcelas). O relatório de bandas não tinha mestre-detalhe (só grupos num dataset): ganhou
  as bandas `sub_header`, `sub_detail` e `sub_footer` com o `data_source` do detalhe; depois
  de cada registro do mestre saem os do detalhe, com os totais dele, e o campo que o
  detalhe não tem vem do mestre. Testes: `cargo test -p aylis-dbcontrols --test report` e
  `cargo test -p relatorios`.
- [ ] Notificações push para todas as plataformas (pedido em 06/11, feito; falta o autor
  confirmar): o componente `PushNotifications` (categoria "Device"; guia em
  [docs/push.md](push.md)) registra sozinho com `active`, dá o `token` e o `provider` no
  `on_token` e as mensagens no `on_message` e no `on_opened` (toque na notificação, também
  com o app fechado), com `title`, `body`, `data` e `data_value`. O serviço é um trait no
  núcleo (`PushService`, com a fila `push_event` para o que chega antes do form):
  - Android: FCM sem o SDK do Firebase (o build não usa Gradle): a `AylisActivity` cria a
    instalação no Firebase Installations e pede o token ao Play Services; o receiver
    `AylisActivity$Push` recebe e mostra a notificação; a publicação põe o receiver, a
    permissão do c2dm e o `<queries>` do Play Services;
  - iOS e macOS: APNs, com os métodos acrescentados ao delegado do app do winit e o delegado
    da `UNUserNotificationCenter`; a publicação do iOS põe o `aps-environment` e o
    `remote-notification`;
  - navegador: Push API com VAPID; o `sw.js` da publicação recebe, mostra e repassa à página;
  - Windows: WNS (só com o app em MSIX); Linux: UnifiedPush pelo D-Bus. Nos dois, o
    componente mostra a notificação pelo `NotificationService` (`show_in_foreground`).
  Conferido por teste (`crates/aylis-controls/tests/push.rs`, o manifesto no `publish.rs`, o
  `sw.js` no Node) e compilando para Windows, web, Android e iOS (o Linux num crate à parte;
  o macOS não foi compilado aqui). Pronto quando receber de verdade em cada plataforma (o
  que depende de aparelho está em [docs/testes-em-dispositivos.md](testes-em-dispositivos.md)).
- [ ] Peças para montar fluxos com o push, como um app de chamadas (pedido em 06/11, feito;
  falta o autor confirmar; sem áudio nem vídeo, e quem liga as peças é o app): o
  `PushTrigger` (filtro sobre os dados → executa uma ação), o `WakeAppAction` (abre o app por
  cima da tela bloqueada, tocando, com botões e tempo limite) e o `NotifyAction` (notificação
  montada com modelos `{chave}`). No Android as regras ficam gravadas no aparelho e o Java
  executa a ação com o app fechado (o `ActionService` do núcleo); os eventos (`on_trigger`,
  `on_open`, `on_button`, `on_timeout`, `on_click`) chegam quando o app roda. Nas outras
  plataformas, com o app aberto. Guia em [docs/push.md](push.md); testes em
  `crates/aylis-controls/tests/push.rs`. Falta: o iOS por cima do bloqueio (CallKit e push de
  VoIP pelo PushKit) e o toque no desktop.
- [ ] Assistente de novo projeto (pedido em 01/11; não começou): no lugar do diálogo único
  de hoje (nome, pasta, linguagem, renderizador e modelo), um passo a passo: linguagem (com a
  instalação do que faltar), plataformas alvo (Windows, Linux, macOS, Android, iOS, web),
  modelo e as configurações iniciais (tema, renderizador por plataforma, ícone, nome e id do
  app, permissões), e um resumo antes de criar.
  Feito em 03/11 (falta o autor confirmar): `crates/aylis-ide/src/wizard.rs`, em 5 passos
  (projeto e linguagem, com "Instalar…" quando falta o compilador; plataformas; modelo;
  configurações: tema, renderizador, nome, id e ícone do app, permissões; resumo), com Voltar e
  Avançar e a validação de cada passo. `aylis_project::new_project::create_with` aplica as
  escolhas: `platforms` no `.aylisproj` (vazio = todas), o ícone copiado para `assets/images`, o
  id conferido e, com permissões, um `Permissions` (`prmApp`) no form principal. Testes no
  `aylis-project` e no `aylis-ide` (`the_new_project_wizard_goes_step_by_step`).
- [ ] Áudio: testar duas saídas reais tocando juntas (alto-falante e fone Bluetooth) e o
  `delay`; conferir o monitor do PipeWire no Linux.
- [ ] Ícones por tipo de arquivo no macOS e no Linux (feito; falta conferir nesses sistemas):
  `aylis-tool ide-package --target linux|macos` monta o `.desktop`, o MIME e os ícones hicolor
  com `instalar.sh`, ou o `.app` com `Info.plist` (tipos de documento e exportados) e um
  `.icns` por tipo.

**Ferramentas**
- [ ] Pasta de compilação global e limpeza por projeto (pedido em 05/11: "se cada projeto novo
  que eu criar ficar compilando tudo de novo, ocupa muito espaço"; "um índice que indique quais
  projetos estão gerando quais arquivos"; "arquivos de bibliotecas compartilhadas manteria por
  mais tempo"). Feito em 05/11, falta o autor confirmar:
  - a pasta é configurável em Opções da IDE → Rust e Python (`target_dir` no `tools.json`; vazio
    é a de antes: o `target` do repositório ou `%LOCALAPPDATA%\aylis\target`);
  - toda compilação pelo aylis anota em `<target>/.aylis-index.json` que unidade do cargo cada
    projeto usou (`aylis_tools::build_index`, pelas mensagens JSON do cargo, também as unidades já
    prontas);
  - a limpeza (`aylis_tools::cleanup`) separa: o que nenhum projeto vivo usa mais (outra versão do
    Rust ou de uma dependência, projeto apagado) sai logo; o que é só de um projeto (código,
    testes, exemplos) sai 14 dias depois da última compilação dele (`output.cleanup_days` no
    `.aylisproj`, campo "Limpeza (dias)" nas Opções do projeto); o compartilhado (aylis,
    dependências), 30 dias depois do último projeto (os dois prazos nas Opções). Cada projeto
    guarda a sua cópia do aylis: dois projetos com features diferentes não se apagam mais. O
    que está fora do índice (cargo no terminal) segue a regra de antes (7 dias ou substituído);
    a pasta `examples/` (7 GB aqui) passou a ser limpa também;
  - o `.cargo/config.toml` do projeto (fora do repositório do aylis) aponta para a pasta: o cargo
    no terminal e o servidor de linguagem não compilam o aylis de novo num `target` do projeto;
  - "Limpar agora" nas Opções; `aylis-tool clean --project app.aylisproj` apaga o que é só do
    projeto. Testes: `cargo test -p aylis-tools --test build_index` (um projeto compilado de
    verdade) e `cleanup::tests`.
- [ ] Depuração remota (feito; falta conferir com outro PC e com o ZenFone): `aylis-tool
  debug-agent` (canal Noise `NKpsk0` sobre WebSocket, pareamento por `host:porta#chave`),
  `agent-run` envia e executa, `agent-dap` repassa o DAP com os caminhos trocados; Android
  pelo Wi-Fi com `aylis-tool android-wifi` (`adb tcpip` + `adb connect`). Conferido nesta
  máquina, agente e cliente no mesmo PC.

**Web**

**Backend**
- [ ] Sessões nos clientes TypeScript, C#, Java, Go, C++ e Free Pascal (feito; falta o autor
  confirmar): `Session`, `User` e `Server` como no Python (`login`/`logout`, `broadcast` às
  telas do mesmo usuário, `post` de outra thread), uma thread ou goroutine por tela. O teste
  `sessions_*` do `aylis-host --test bridge` roda o mesmo roteiro nas 7 linguagens: duas telas
  com contadores separados, o erro de uma não derruba a outra, a lenta não trava a outra e as
  duas do mesmo usuário somam juntas.

**Arquitetura para software grande (análise de 02/11/2026)**

Levantado numa revisão do código: o que pode quebrar ou atrapalhar um app com centenas de
forms e uma equipe grande. Cada item diz o problema e quando está resolvido; a ordem é a da
gravidade. A discussão item por item ainda pode mudar a solução.

- [ ] Itens 1, 2, 12 e 3, as outras linguagens, a repintura por parte e o desempenho (as notas de 02/11 a
  05/11 que estavam embaixo do item 1b; o 1b foi conferido em 08/11 e está nos concluídos):
  Feito em 02/11 para o Rust (itens 1, 2 e 12; falta o autor confirmar). No núcleo:
  `register_form_class`/`bind_form` montam a instância de cada form ao criá-lo (o form que não
  bate com o código não abre; os criados antes do registro, como o principal, ganham a
  instância no registro, e um que não bate vira o erro do app); `form_handler` chama o método
  da instância do form de quem disparou; um evento do mesmo form dentro de um handler dele
  espera ele terminar (com aviso em modo dev); o mesmo form abre duas vezes
  (`create_form_instance`; entre as raízes o nome pode repetir); `ui.with_form` chega à
  instância de fora (um `post` de uma thread). Na geração: a pasta `rust/generated/` é da IDE
  (`components.rs`: um handle por classe, com o `ComponentId`, lendo e gravando sem falha
  calada pelo `aylis_core::handles` e com `try_get`/`try_set`; a struct de cada form, com os
  componentes resolvidos, os de só algumas plataformas como `Option`, o estado `data` e
  `get`/`get_mut`/`find_x`/`sender_as_x`; `mod.rs`: o registro e os arquivos dos forms); o
  `src/` é do usuário (o `lib.rs` e o `main.rs` criados uma vez; o arquivo de cada form nunca é
  refeito: a geração só acrescenta o handler novo no fim do `impl`). Os métodos dos handles
  recebem `self` por valor (são `Copy`): `self.lbl.set_text(ui, self.calcula(ui))` compila.
  Os projetos foram convertidos por um passo do `aylis-migrate` (handlers, funções que usam o
  form e as que chamam essas viram métodos; `frmX.` → `self.`; `find_path("frmX.y")` →
  `find_in(self.form.id, "y")`; o `Data`); os 9 exemplos em Rust e o NoOBS (16 forms e frames)
  compilam; à mão, só a GradeGigante (os `post` das threads usam `ui.with_form`), a
  Calculadora (a lista de botões virou método, a conta foi do `thread_local!` para o `Data`) e
  o `lib.rs` do NoOBS (os módulos vêm de uma macro). Depois, a pedido do autor (02/11: "não vai
  sobrar nenhum projeto fora daqui"), o passo saiu do migrador e o formato voltou a 1.

  Pascal, também feito em 02/11 (falta o autor confirmar): a unit gerada (`ayliscomponents`)
  tem os componentes por id (herdando `TAylisComponent`) e uma classe base por form
  (`TFrmXBase = class(TForm)`, com os componentes `published`, montados ao abrir; o que não
  bate levanta `EAylis`); a unit do form é do usuário: `TFrmX = class(TFrmXBase)` com os
  handlers `published` `(Sender: TAylisComponent; Event: TJSONObject)` (o cliente acha pelo
  nome com `MethodAddress`; o `Aylis` dos corpos é a propriedade herdada), o estado em campos
  `private`, a variável global `frmX` (a primeira instância, como no Delphi) e o
  `RegisterForm` na `initialization`; a geração só acrescenta os handlers novos. O protocolo
  manda o `form_id` no evento e ganhou o `components` e o `create_form` com `new`; o cliente
  ganhou `TAylisForm`, `CreateForm` e a instância por janela. Os 5 exemplos Pascal compilam, e o
  roteiro dos 4 apps dá o mesmo estado nas 8 linguagens.
  As outras 6 linguagens, também em 02/11 (falta o autor confirmar): o cliente de cada uma
  ganhou a base dos componentes e dos forms (`AylisComponent`/`AylisForm`; em Go,
  `Component`/`Form`; em C++, `aylis::Component`/`aylis::FormBase`), o registro das classes
  dos forms e a instância por janela (montada com o `components` na primeira vez e tornada a
  atual antes de cada handler, pelo `form_id` do evento), o `CreateForm` com `another` e o
  `exception` na falha do handler (item 9). O gerado tem os componentes por id e uma classe
  por form; a variável global `frmX` é a janela do evento (a primeira, fora dos handlers): em
  Python um `CurrentForm` que repassa à instância; em C++ a instância é copiada para o
  global, então `&frmX.btn` continua valendo. O registro dos handlers saiu das units do
  usuário (sem blocos marcados) para o arquivo gerado de cada linguagem (`index.ts`,
  `forms/__init__.py`, `AllForms.cs`, `Forms.java`, `forms.go`, `forms.hpp`). Os exemplos
  compilam e o roteiro dos 4 apps dá o mesmo estado nas 8 linguagens. O cliente vindo da
  instalação e o tempo de compilação com 100 forms: no item 12.

  Decisões do item 3, uma a uma (02/11):
  - 3.1 Prazo (decidido): sem prazo automático enquanto a conexão estiver viva; o segundo
    clique no X durante a espera pergunta "O programa não está respondendo. Encerrar o
    programa? As alterações não salvas serão perdidas." (`MessageDialog`) e encerra o
    backend junto (no desktop, mata o processo filho e fecha o app; no servidor da
    `--bridge`, só a sessão daquela tela, que fica abandonada e recebe `Aylis.Cancelled`); a
    conexão caída fecha. A espera tem um componente próprio, o `BusyWatcher` (aprovado em
    02/11; conveniência para quem não pôs o trabalho pesado numa thread, prioridade baixa):
    - não visual (System), no form principal; sem `scope`: ocupado é sempre do backend;
    - `delay` (1000 ms), `action` (`default`, `form`, `none`), `busy_form` (frame por cima
      do form que originou a espera, ou form em janela), `block_input` (`true`),
      `min_visible` (300 ms);
    - `on_busy_start` (antes de mostrar; `set_handled` pula o form) e `on_busy_end` (depois
      de esconder), rodando do lado da tela: em Rust ou no próprio `busy_form`; o backend
      ocupado não roda handler, mas pode mandar progresso de outra thread (`post`);
    - o evento leva um `seq` e o cliente responde `done` sozinho quando o handler termina;
      ocupado = evento sem `done` há mais de `delay`;
    - sem o componente, o padrão é bloquear a entrada com o cursor de espera, continuar
      pintando e, depois de uns 3 s, um indicador discreto do aylis;
    - uma janela aberta pelo próprio pedido (`message_dlg`, `show_modal`) põe o pedido em
      "esperando o usuário": o relógio para e o `busy_form` some; volta do zero na
      resposta;
    - em Rust, só funciona com o trabalho fora da thread da tela (thread com `post` ou
      tarefa, item 14).
  - 3.2 Ações durante a espera (decidido). Ocupado é do backend (no servidor, da sessão):
    enquanto um handler roda, todos os forms ligados a ele recusam a entrada desde o
    primeiro instante (o visual só depois do `delay`); continuam pintando, animando,
    movendo, redimensionando e minimizando, e o X leva à saída de emergência. Os eventos
    repetidos se juntam por (componente, evento): um `on_timer` não sai enquanto o anterior
    não teve `done`; `on_resize`, slider e semelhantes ficam só com o último valor; clique e
    tecla nunca se juntam; o catálogo marca quais se juntam. Paralelismo é escolha do
    programador (decidido em 02/11): sem handlers em paralelo, sem `Aylis.Background` e sem
    thread de comunicação; o trabalho pesado vai para uma thread da própria linguagem, com
    um form em `show_modal` mostrando o progresso e um botão Cancelar comum (o handler dele
    roda, porque a thread dos handlers está livre). Para isso funcionar com segurança, o
    aylis dá, em todos os clientes e também no modo desktop (hoje o `TAylis` do Pascal
    escreve sem trava e só o `TAylisSession` tem `Post`):
    - trava na escrita das mensagens: gravar propriedades de qualquer thread; ler (o que
      espera resposta, como o `Call`) só na thread dos handlers, porque a resposta chega
      pela mesma leitura (em debug, avisar quem ler de outra thread);
    - `Post` para rodar código na thread dos handlers (o `TThread.Queue` do Delphi), por
      exemplo para fechar o modal no fim; no desktop, o laço está parado lendo a entrada,
      então o `Post` pede à tela um eco (`{"cmd": "wake"}` → `{"event": "__post"}`) que o
      acorda;
    - o guia com esse padrão, em cada linguagem.

    Feito em 02/11 (falta o autor confirmar): o comando `wake` no `aylis-remote` (devolve
    `{"event": "__post"}`), e no cliente Pascal a trava na escrita, o `Post` do `TAylis`
    (desktop) e o erro claro num `Call` de outra thread; o exemplo
    `examples/ProcessamentoAssincrono` (só em Pascal: a thread em `fechamento.pas`, o modal
    de progresso e o Cancelar), com os roteiros `processamentoassincrono` e
    `processamentoassincrono-cancel` e o teste
    `processamento_assincrono_runs_the_work_in_a_thread` do `aylis-tools` (não rodou pelo cargo: o
    `aylis-tool.exe` estava preso por um `serve` de outra sessão; os roteiros, tocados direto
    no `aylis-host`, deram "Mês fechado: 100 notas" e "Cancelado na nota 27 de 100").
    Depois, também em 02/11: o mesmo nos clientes TypeScript, C#, Java, Go, C++ e
    Python (a trava na escrita, o `Post` com o `wake` no desktop, o erro do `Call` de outra
    thread) e o guia em `docs/threads.md`. Falta:
    - os projetos já criados guardam a própria cópia do cliente (`src/aylis.pas`, que a
      geração não sobrescreve): eles não recebem o `Post` sozinhos.
  - 3.3 O que a tela mostra (decidido): coberto pelo 3.1 (cursor de espera, a pintura
    continua e, depois de uns 3 s, o indicador discreto; com o `BusyWatcher`, o declarado).
  - 3.4 Diálogo dentro do handler (decidido em 02/11, opção C). Nos clientes remotos,
    `MessageDlg` e `ShowModal` esperam como no Delphi (`if FrmCliente.ShowModal = mrOk`,
    `CanClose := MessageDlg(...) = mrYes`), e a versão com callback continua:
    - o protocolo ganha `"wait": true` no `message_dlg` e no `show_modal`: a resposta
      `{"id": .., "result": "yes"}` só sai quando o diálogo fecha (resposta adiada na tela);
    - enquanto espera, o cliente trata os eventos que chegam (os do modal e os `Post`), como
      o laço do `ShowModal` do Delphi; num `Call` comum, os eventos continuam na fila;
    - o pedido fica em "esperando o usuário" (3.1), sem prazo;
    - em Rust, só callback (a thread da tela não pode esperar, e o winit e o navegador não
      têm laço aninhado), explicado no guia;
    - o ProcessamentoAssincrono passa a usar o `ShowModal` que espera.
  - 3.5 `before_post` com backend remoto (decidido em 02/11). `before_post` e
    `before_delete` viram eventos do form, por nome, em todas as linguagens (hoje só closure
    em Rust); `before_insert`, `before_edit` e `before_scroll` só se precisar:
    - recusar como no Delphi: exceção no handler (`raise EAylis.Create('Informe o CPF')` em
      Pascal, `throw`/`raise` nas outras, `error` em Go) recusa, e a mensagem vai para a
      tela; terminar normalmente aceita; em Rust, o handler devolve `Result<(), String>`
      (`Err` recusa, o `?` repassa) e a IDE gera o esqueleto já com esse tipo;
    - a tela espera naturalmente, como no Delphi: enquanto o `post` roda, a entrada fica
      bloqueada com o cursor de espera (3.2) e a tela continua pintando; o registro só
      grava (e dispara o `after_post`) com a resposta; recusado, continua em edição, com a
      mensagem e o foco no campo; os controles DB* respeitam o `post` pendente;
    - o código do backend que chama `Post` só segue depois de o `post` terminar, como no
      Delphi: um evento-pergunta roda mesmo durante um `Call` (senão o `Post` do próprio
      backend esperaria a si mesmo), com limite de profundidade;
    - uma exceção inesperada também recusa; em modo dev, o log mostra a pilha.
  - 3.6 Teclas (decidido em 02/11). Já existem: `allowed_chars`, `blocked_chars`,
    `only_allowed`, `chars_override` (com herança dos contêineres), `mask`, o `edit_mask` e o
    `char_case` do Field, `tab_keys` e os atalhos. Entra:
    - `char_map` em todos os controles que aceitam texto (e nos contêineres, com a mesma
      herança dos filtros): pares "caractere digitado → caractere entregue" (`,` → `.`),
      editados na IDE como uma lista de pares; evita a programação só para trocar caractere;
    - `char_case` (`normal`, `upper`, `lower`) no Edit e nos outros controles de texto;
      ligado a um Field, o `char_case` (e o `char_map`, se o Field tiver) do Field é que manda;
    - `char_case` e `char_map` ficam separados (decidido em 02/11): o `char_case` é uma
      regra sobre o valor (vale ao digitar, ao colar e quando o código grava o `text`; usa
      as regras do Unicode e do idioma, como `ß` → `SS`), o `char_map` só troca o que o
      teclado produz (digitar e colar); na IDE, os dois e os filtros (`allowed_chars`,
      `blocked_chars`, `only_allowed`) ficam num só editor "Entrada de texto", na categoria
      Teclado;
    - a ordem de um caractere digitado: `char_map` → `char_case` → filtros → `on_key_press`
      (vê o caractere já transformado, pode trocar ou engolir) → o controle;
    - `key_preview` no form (recebe o `on_key_down` antes do controle com foco, como o
      `KeyPreview` do Delphi) e um evento de teclado da aplicação inteira (antes de todos, como
      o `TApplicationEvents`): aplicação → form (`key_preview`) → controle → os pais;
    - o handler engole ou troca a tecla (o `Key := 0` e o `Key := #0`/`Key := '.'` do
      Delphi): em Rust na hora; nas linguagens remotas também (casos legítimos existem), com
      `on_key_down` e `on_key_press` como eventos-pergunta quando ligados a um handler remoto:
      cada tecla daquele controle espera a resposta, e as teclas digitadas enquanto isso
      ficam numa fila da tela, em ordem, sem se perder; quem não liga o evento não paga
      nada; o guia avisa do atraso por tecla pela `--bridge` na internet.
  - 3.7 Arrastar e soltar (decidido em 02/11). Hoje o `on_drag_start` precisa de
    `begin_drag` e o `on_drag_over` de `reject_drop` na hora (só Rust; no remoto o arrasto
    nem começa), o protocolo não tem nada de arrastar e o arrasto é por form. Entra:
    - declarativo: `drag_data` (`"cliente:123"`, o tipo antes do primeiro `:`) e
      `drag_label` na origem, que começam o arrasto sem handler; listas e grades mandam o
      item (a chave do registro, o texto) sozinhas; `drop_types` (`["cliente", "produto"]`)
      no destino, que aceita ou recusa sem perguntar;
    - pergunta nas linguagens remotas: `on_drag_start` (a resposta traz dados e rótulo, ou
      cancela) e `on_drag_over` (uma ida e volta por destino; o destino fica neutro até a
      resposta; soltar antes espera a resposta); recusar com exceção (em Rust, `Err`);
    - o arrasto passa a ser do app: a origem num form e o destino em outro (desktop);
    - arrastar do app para fora (Explorer, outros programas) fica para um item à parte.
  - 3.8 A forma da API nos clientes (decidido em 02/11). Eventos-pergunta:
    `on_close_query`, `before_post`, `before_delete`, `on_key_down` e `on_key_press` (onde
    ligados), `on_drag_start`, `on_drag_over` e o `on_validate` do Field (o `raise` recusa o
    valor na hora e o foco fica no campo, como o `OnValidate` do Delphi; o `field_error`
    continua para a validação que termina mais tarde). Mais o `ShowModal` e o `MessageDlg`
    que esperam (3.4), com a mesma resposta adiada:
    - recusar com mensagem é sempre exceção (`raise`, `throw`; `Err` em Rust, `error` em Go);
    - responder um valor é do jeito de cada linguagem: em Pascal, `var` como no Delphi
      (`var CanClose: Boolean`, `var Key: Char`, `var Data, Caption: string`); nas outras,
      um objeto de argumentos com campos graváveis (`e.Cancel`, `e.KeyChar`, `e.Handled`,
      `e.Data`; em Rust, `args` com `&mut`);
    - a resposta é automática: o cliente lê o respondido quando o handler termina e manda à
      tela;
    - o catálogo (`components.json`) marca os eventos-pergunta e os campos de resposta; a
      IDE gera o esqueleto com a assinatura certa em cada linguagem e atualiza a assinatura
      dos projetos existentes ao gerar o código.

  Feito em 02/11 (falta o autor confirmar):
  - perguntas no núcleo (`question.rs`): `ui.ask` dispara e devolve a resposta; em Rust o
    handler responde na hora (`ui.refuse`, `ui.reply`; o handler do form pode devolver
    `Result<(), String>`, e o `Err` recusa); quem não responde já adia
    (`defer_question`/`answer`), e quem perguntou diz com `on_answer` o que fazer com a
    resposta. O `on_close_query` (o form só fecha com a resposta; o segundo X durante a espera
    abre "O programa não está respondendo. Encerrar o programa?", que encerra o backend junto),
    o `before_post` e o `before_delete` (eventos do ClientDataSet, por nome: o registro só grava
    ou sai com a resposta; recusado, a mensagem vai para a tela e continua em edição), o
    `on_validate` (recusado em Rust, o valor não entra; remoto, marca o campo e o `post`
    espera), as teclas e o arrasto. Um handler do mesmo form que espera o outro terminar
    responde quando rodar;
  - protocolo: o evento-pergunta vai com `ask` e volta com `answer` (`values` ou `error`); os
    clientes respondem sozinhos quando o handler termina e tratam um evento-pergunta mesmo
    durante um `Call`; o `data post`/`delete` do backend só responde quando a gravação termina, e
    a operação que esbarra numa gravação pendente (navegar, incluir, aplicar) espera e roda
    depois; `message_dlg` e `show_modal` com `wait: true` respondem quando a caixa fecha
    (`MessageDlg`/`ShowModal` que esperam nos 7 clientes, tratando os eventos e os `Post`
    enquanto isso);
  - ocupado (`busy.rs`): cada evento leva `seq` e o cliente responde `done` (só os que anunciam
    `done` no `hello`, para os projetos com cliente antigo não travarem); enquanto houver um sem
    `done`, o clique é recusado e as teclas esperam numa fila; passado o `delay`, o cursor de
    espera e, depois de uns 3 s, o aviso "Aguardando o programa…"; o `BusyWatcher` (System) com
    `delay`, `action` (`default`, `form`, `none`), `busy_form`, `block_input`, `min_visible`,
    `on_busy_start`/`on_busy_end`; o diálogo aberto pelo pedido põe em "esperando o usuário".
    Os eventos que se juntam (`on_timer`, `on_resize`, `on_scroll`, um slider...: o catálogo
    marca com `coalesce`) não saem de novo antes do `done`, e só o último vai;
  - teclas (3.6): `char_map` (pares; nos controles de texto e nos contêineres, com a herança
    dos filtros), `char_case` (`normal`, `upper`, `lower`; no Edit e no Memo, também no texto
    gravado pelo código), num editor só "Entrada de texto" na IDE; `on_key_press` (vê o
    caractere já trocado; `ui.replace_key_char`, `ui.suppress_key`); `key_preview` no form; o
    `ApplicationEvents` (System: `on_key_down`, `on_key_press` antes de todos e o
    `on_exception`, que substitui a caixa de erro); remotos, cada tecla espera a resposta e as
    seguintes ficam numa fila, em ordem;
  - arrastar (3.7): `drag_data`/`drag_label` na origem e `drop_types` no destino, sem handler;
    a ListBox manda o item e a DataGrid a chave do registro; `on_drag_start`/`on_drag_over`
    como pergunta (o destino remoto fica neutro até a resposta; soltar antes espera); entre
    janelas do app (a plataforma passa o form sob o mouse);
  - assinaturas (3.8): o catálogo marca `question:campos`; a geração faz o esqueleto em Pascal
    com `var` (`var CanClose: Boolean`, `var Key: string`, `var Data, Caption: string`, `var
    Accept: Boolean`) e em Rust com `Result<(), String>`, e troca a assinatura comum de um
    projeto de antes; nas outras linguagens, o `reply` do evento (`ev.Reply["can_close"]`);
  - o guia em `docs/threads.md`; testes: `questions` (aylis-controls e aylis-data), `drag_drop`,
    o selftest das 7 linguagens fechando o form pelo backend (`on_close_query`) e o Python com
    `before_post` durante o `data post`.
  Ficou de fora: a sessão abandonada da `--bridge` receber `Aylis.Cancelled` (fechar a tela já
  encerra a sessão).

  Revisto em 02/11 (o que outra sessão fez, ainda sem commit, com a rolagem do editor de
  código): já existe parte da etapa 2 no núcleo: `Ui::damage(id)` (o `ctx.repaint()` passou a
  marcar só a área do controle, com folga fixa de 24), `take_damage` e `paint_region`; o
  Skia na GPU guarda a imagem do quadro anterior e repinta só a área (o Skia na CPU, o
  Direct2D e a web ainda repintam inteiro); o Skia guarda os parágrafos de uma linha já
  montados (`LINES`); o `Memo` desliza a roda pelo `frame()` do núcleo e marca a área pelo
  `ctx.repaint()`, sem lógica de repintura própria (os testes `damage` e `memo_scroll`
  passam). Para a repintura não se duplicar entre o núcleo e os componentes:
  - o componente só diz o que mudou (`ctx.repaint()`, `Ui::damage`); decidir a área, guardar
    imagem, recortar e copiar é só do núcleo e do renderizador; nenhum componente guarda
    pixels nem faz a própria cópia de rolagem;
  - a camada de rolagem (etapa 4) precisa servir também quem rola por dentro (o `Memo`, o
    `DataGrid`, o `VirtualList`, as listas): uma API do núcleo em que o componente diz a
    área visível e o deslocamento, e o núcleo guarda e copia, em vez de cada um fazer a sua;
  - o cache de parágrafos do Skia vira o do motor de texto do núcleo quando os modos
    `layout`/`core` do item 10 chegarem (um cache só, valendo também para o Direct2D e a
    web), e o do Skia sai;
  - a folga fixa de 24 troca pela extensão real dos efeitos (sombra, desfoque, anel de
    foco), como decidido;
  - falta o teste de pintar por parte × por inteiro e comparar as imagens, que pega o
    controle que desenha fora da própria área depois da mudança do `ctx.repaint()`.

  Feito em 02/11 (falta o autor confirmar): o Skia na CPU também repinta só a área (quando o
  `softbuffer` guarda o quadro anterior); a folga fixa de 24 saiu: o núcleo mede, na pintura,
  onde cada controle desenhou de verdade (`ink.rs`, um canvas que repassa tudo e anota a
  extensão de cada desenho) e soma a sombra mais larga dos estados do estilo (a "lg" do tema
  passava de 24); quem desenha além de onde desenhava antes e fora da área faz o próximo
  quadro ser inteiro; o descarte da pintura não pula mais um controle que não recorta os
  filhos nem um cujo desenho passou da borda até a área. O teste `partial_paint` do
  `aylis-controls` passa o mouse e o foco por todas as classes e compara pixel a pixel (aceita
  só a beirada anti-aliasing de um desenho cortado pela área, que o Skia rasteriza um pouco
  diferente; o resto de verdade que ele já pegou mudava mais de 200 tons). Medido no teste: a
  área média é 17% da janela, porque o mouse indo de um controle para o fundo do form
  repinta tudo (o form muda de estado); isso some com o desenho gravado (etapa 3), que diz
  quando o desenho não mudou. O Direct2D também (na swap chain, a pintura vai para um
  bitmap guardado, por parte, que é copiado inteiro para o buffer de trás, porque no modelo
  flip o buffer de trás não guarda o quadro anterior); o `partial_paint` roda nele também, e
  numa janela de verdade (o exemplo `components`, o mouse varrendo a tela e depois um quadro
  inteiro) a única diferença foi o relógio que andou. A web também (o canvas guarda a
  imagem; mudar de tamanho repinta inteiro; com janelas flutuantes por cima, inteiro); na
  IDE no navegador, o mouse varrendo a tela e depois um quadro inteiro deram 0 pixels
  diferentes, depois de o `Canvas2d` ignorar um `restore` sem `save` (como o Skia já fazia),
  que fechava o recorte da área. O layout é por form: mexer num controle (`set_prop`,
  `control_mut`, `ctx.invalidate()`, trocar de pai) refaz só o form dele
  (`Ui::invalidate_form`); o `invalidate()` sem controle continua refazendo todos.

  O caso grande (02/11): o exemplo `large_ui` do `aylis-controls` (8 forms, 6.768 controles,
  Skia na CPU, 1280x800, release) mede digitar num Edit, o relógio na barra de status, a
  barra de progresso e rolar um Memo de 2.000 linhas. A primeira medição mostrou que só
  digitar repintava por parte: mudar uma propriedade só de pintura (o valor da barra) e
  mudar o texto (que refaz o layout do form) pediam a janela inteira. Agora a propriedade só
  de pintura marca o controle, e o layout de um form marca o controle que mudou e quem o
  layout mudou de lugar, de tamanho ou de visibilidade; o `take_damage` faz o layout pendente
  antes; um controle nunca pintado usa o próprio retângulo. Resultado, por parte × inteiro:
  digitar 0,51 × 4,95 ms (0,2% da janela), relógio 0,69 × 4,93 ms (0,4%), progresso 0,45 ×
  4,88 ms (0,7%), rolar 1,43 × 5,55 ms (20,6%, a área do Memo; a camada de rolagem da etapa
  4 tira isso). O layout de todos os forms custa 0,7 ms e achar um form pelo nome 0,0003 ms
  (o índice do item 6): o layout por parte dentro do form não compensa e fica de fora. O
  teste `large_ui` é o limite na CI, pela área repintada (2% nos três primeiros, 30% no
  rolar), que vale em qualquer máquina. Na web (03/11, a IDE lite em release, Canvas 2D, no
  navegador embutido do app): o tempo de cada quadro, medido em volta do `requestAnimationFrame`
  ao passar o mouse pela paleta e digitar no inspetor, deu mediana 3,4 ms e 90% abaixo de
  19,8 ms (o pior, 20,6 ms, é o quadro que repinta a janela inteira ao trocar o foco). A área
  repintada é a mesma do desktop (é do núcleo); o custo maior vem do Canvas 2D.

  Pela regra decidida (parar na etapa 2 se a medição mostrar que o nível da VCL basta), as
  etapas 3 (desenho gravado) e 4 (camadas, com a API de rolagem interna) ficam esperando:
  o que elas resolveriam hoje é o mouse indo de um controle para o fundo do form (repinta
  tudo, porque o form muda de estado) e a rolagem do Memo (20% da janela, 1,4 ms). Voltam
  se uma medição num app de verdade mostrar quadro lento nesses casos.

  Decidido pelo autor em 05/11 ("pode fazer todas as melhorias para o desempenho"): fazer as
  etapas 3 (desenho gravado por controle, regravado só quando ele muda) e 4 (camadas na rolagem
  e nos controles animando posição ou opacidade, e o `cache_layer`), nos três renderizadores.

  Feito em 05/11 (sem commit; falta o autor confirmar na janela de verdade):
  - Etapa 3: o desenho próprio de um controle vira uma "impressão digital" (`trace.rs`); quando
    só o estado (hover, foco...) de um contêiner muda, o `Ui` ensaia o desenho dele e, igual,
    não repinta nada. Vale para quem declara `Component::paint_is_pure` (Form, Panel,
    Rectangle, Layout, Frame, ScrollBox, GridLayout, FlowLayout, GroupBox). A transição de
    estilo só começa se o estilo do novo estado for outro.
  - Etapa 4: controle deslocado (`translate`) repinta o lugar antigo e o novo (dele e dos
    filhos), não mais a janela inteira; a propriedade `cache_layer` (e, automático, o contêiner
    com posição ou opacidade animando) guarda o desenho dele e dos filhos numa imagem,
    recomposta enquanto nada muda por dentro (`begin_cached` no Skia CPU/GPU e na web, que
    não tinham; o Direct2D já tinha); a rolagem do `Memo` vira cópia dos pixels já pintados
    (`Ui::scroll_damage`/`damage_part`, `Ui::take_frame` → `FrameDamage` com cópias e áreas),
    feita no Skia (CPU e GPU), no Direct2D (no quadro guardado) e no canvas da web; as áreas
    de um quadro são pintadas de uma vez, com o recorte na união delas.
  - Medido (`large_ui`, Skia CPU, 1280×800): mouse do botão para o fundo, de janela inteira
    para 0,2%; hover e foco no teste por parte, área média de 17,4% para 3,6%; rolar o Memo,
    de 20,6% para 6,2% da janela; painel deslizando com 30 filhos, os filhos não são
    repintados. Os testes `partial_paint` comparam a imagem por parte com a inteira (Skia,
    texto core, Direct2D) em todas as classes, também deslocadas, e na rolagem.
  - Tela gigante (pedido do autor em 05/11: 19.200 × 6.480, `cargo run --release -p
    aylis-controls --example giant_screen`, 17.822 componentes, Skia CPU sem janela): montar
    37 ms, layout 2,3 ms, pintura inteira 715 ms; por quadro, digitar 1,1 ms, relógio 4,4 ms,
    60 barras espalhadas 2,9 ms, mouse 1,1 ms, painel deslizando 2,9 ms, rolar um Memo da
    largura da tela 21 ms. Corrigido no caminho: o byte alto era zerado na tela inteira a
    cada quadro (34 ms fixos), as áreas viravam o retângulo que contém todas e os controles
    eram escolhidos por ele (60 barras: 345 ms; rolar: 469 ms); agora são pintados só os
    que tocam alguma área (`Ui::paint_regions`).
  - O teste `giant_screen` entrou na bateria (pedido do autor em 05/11: "o resultado deve ser
    tido como base para não aceitar regressão"): compara com `medicoes/giant_screen.json` a
    área e os controles pintados por quadro (sempre) e os tempos (em release, folga de 50%
    + 1 ms); `AYLIS_UPDATE_BENCH=1` regrava a base. Conferido que pega as duas regressões
    corrigidas hoje.
  - A conferir na janela: rolar um Memo comprido (roda e deslize) no Skia CPU, na GPU, no
    Direct2D e na web, sem resto na tela.

    Feito em 02/11 (falta o autor confirmar): o `on_apply_updates` por nome é pergunta (a
    resposta traz os `results` por registro, ou o handler manda o `reconcile` antes; a exceção
    recusa a gravação inteira); o dataset fica "aplicando" até a resposta (`is_applying`; o
    `DataNavigator` desabilita enquanto a gravação espera o backend); o `data apply` do backend
    só responde com tudo gravado e devolve o número de registros que não foram
    (`apply_errors`). Em Rust, o `on_apply` síncrono continua; para a gravação demorada, o
    handler do `on_apply_updates` adia (`ui.defer_question()`), grava numa thread e entrega com
    `post` (`ui.answer(n, Answer::Accepted(vec![("results", ...)]))`). O `reconcile` continua
    aceito.
- [ ] 9. (Feito em 02/11 em `f5f3b51`; o aviso do `panic = "abort"` foi com a verificação do
  1b, os outros 6 clientes mandando o `exception` com os forms com instância, e o evento de
  exceção da aplicação é o `on_exception` do `ApplicationEvents` (item 3.6); falta o autor
  confirmar.) Um erro no handler não derruba o app, como no Delphi (`Application.HandleException`).
  Hoje, em Rust, um `panic` derruba o app (só a C ABI usa `catch_unwind`); nas linguagens
  remotas, a exceção só vai para o log, e o usuário não sabe que falhou; na web, o wasm para.
  Decidido em 02/11. Pronto quando:
  - em Rust, handlers e métodos de componente rodam protegidos no laço da plataforma
    (desktop e Android), e a proteção devolve à árvore o componente que estava fora
    (item 4);
  - o erro aparece no `MessageDialog`; em modo dev, também o caminho do handler e a pilha,
    no log, no `activity` e na gravação (`.aylisrec`);
  - há um evento de exceção da aplicação (como o `TApplicationEvents.OnException`), no
    componente de eventos da aplicação do 3.6; tratado, a caixa padrão não aparece;
  - nas linguagens remotas, o cliente avisa a tela, que mostra a mesma caixa (ou chama o
    evento);
  - um erro na pintura marca o componente (um retângulo de erro no lugar dele), registra
    uma vez e o resto continua;
  - na web, a tela "o aplicativo encontrou um erro" com recarregar (o wasm não continua
    depois de um `panic`);
  - a IDE avisa quando o projeto usa `panic = "abort"` (sem a proteção); o guia recomenda
    transação nas operações de banco;
  - há teste.
  Conferido em 08/11 (roteiro 48): não aprovado. O `panic!` vira a caixa de erro com o lugar e o
  processo segue, mas o form fica marcado como "handler rodando" para sempre e todo evento
  seguinte dele é adiado e nunca roda (veja os ajustes de 08/11, grave).

  Feito em 02/11 (o "agora", falta o autor confirmar): o projeto novo vem com a Inter em
  `assets/fonts/inter/` como fonte do projeto (`aylis_project::DEFAULT_FONT`); a verificação
  `aylis_project::check::fonts` avisa quando o projeto usa a fonte do sistema ou um componente
  pede uma família que não está embutida (a IDE mostra junto com a verificação antes de
  compilar, do item 1b); o teste `text_metrics` do `aylis-controls` mede a mesma lista de
  textos no Skia e no DirectWrite: com a fonte embutida, a mesma largura e as mesmas quebras;
  só a altura da linha muda onde o caractere cai na fonte do sistema (árabe, emoji,
  japonês: 19 × 16,9); o guia (`docs/fontes-e-icones.md`) pede folga e `auto_size`. Ficaram:
  - a web no teste de medidas (mede no navegador; precisa de um teste em wasm);
  - o designer no renderizador da plataforma escolhida: a IDE desenha tudo num renderizador
    só, e mostrar o form em outro seria pintar fora da tela e mostrar a imagem; com a fonte
    embutida as medidas já batem entre o Skia e o DirectWrite, então a diferença que sobra é
    a da fonte de fallback. Decidido pelo autor em 03/11: fica como está (o `core` e o `layout` já dão as mesmas medidas no designer); se surgir a necessidade, faz-se;
  - o "depois" (os modos de texto `native`/`layout`/`core` por plataforma).

  O "depois", feito em 03/11 (falta o autor confirmar): o motor de texto do núcleo
  (`aylis_core::textcore`, com o `ttf-parser` e o `rustybuzz`, em Rust puro e também no wasm)
  e o `"text"` por plataforma no `.aylisproj` (`windows`, `linux`, `macos`, `android`, `ios`,
  `web`), aplicado pelo app ao abrir e escolhido em Projeto → Opções do projeto. No `layout`
  e no `core` o núcleo mede (o `Ui::measure_text` e o canvas) e decide as quebras, com a mesma
  regra de linhas dos renderizadores; no `layout` o renderizador desenha cada linha, no
  `core` o núcleo desenha o contorno das letras (a fonte variável no peso pedido). Texto com
  um caractere que as fontes do projeto não têm fica com o renderizador. Projeto novo: `core`
  no desktop e no celular, `layout` na web; o de antes fica em `native`. O teste
  `text_modes`: as medidas iguais com o Skia, o DirectWrite e sem renderizador; no `core`,
  0,01% de pixels diferentes entre o Skia e o Direct2D (1,09% no `native`); e o
  `partial_paint` roda também no `core`. Com o `layout` na web, a medida no navegador é a do
  núcleo (o mesmo código em wasm), o que resolve o teste de medidas da web para os projetos
  novos; o `native` na web continua medido pelo navegador. O guia em
  `docs/fontes-e-icones.md`.

  Feito em 02/11 (11.3 e 11.4 em parte; falta o autor confirmar): o teste `visual` do
  `aylis-ide` desenha sem janela a tela inicial, a galeria de componentes e o designer com o
  exemplo Calculadora aberto, e compara com as imagens de `crates/aylis-ide/tests/visual/`
  (por sistema; falha se mais de 0,5% dos pixels mudarem mais de 48 tons;
  `AYLIS_UPDATE_VISUAL=1` regrava); roda na CI do Windows, que guarda as imagens novas como
  artefato quando falha; o `partial_paint` (Skia e Direct2D) e o `text_metrics` rodam no
  mesmo job. A CI ganhou o job `coverage` (`cargo llvm-cov` nos crates do núcleo, no resumo
  da execução, sem meta). Depois, em 03/11: as referências do Linux (gravadas no Ubuntu do WSL;
  o teste desenha sem janela, então o Xvfb não é preciso), e o teste também roda no job do Linux
  da CI; os forms principais dos exemplos (Calculadora, Paint, IBGE, Player) e o visualizador
  de gravações de uso, com uma gravação de exemplo sem dados de ninguém
  (`tests/visual/exemplo.aylisrec`). O 11.2 está abaixo.

  11.2, medido antes (02/11, esta máquina): o `aylis-controls` (42 mil linhas) compila do zero
  em 15,7 s (debug) e 15,5 s (release); depois de mudar um arquivo, 2,1 s (debug) e 19 s
  (release); a IDE inteira depois de mudar um controle, 12,5 s (debug, quase tudo o próprio
  `aylis-ide` e o link). A divisão por família ajudaria o release e a compilação do zero, não
  a de todo dia em debug; e ela move todos os arquivos do crate, que outras sessões editam
  ao mesmo tempo.

  11.2, feito em 03/11 (aprovado pelo autor no mesmo dia; falta confirmar): o código saiu do
  `aylis-controls` para crates por família que só dependem do `aylis-core` e compilam em
  paralelo: `aylis-controls-base` (o resto: forms, edição, listas, menus, sistema, Quasar,
  código), `aylis-controls-charts` (Chart, BI, trader, grafo, mapa), `aylis-controls-industrial`,
  `aylis-controls-device` (sensores, permissões, biometria, AppLink) e `aylis-controls-editors`
  (editores visuais, PaintCanvas, AnimatedText). O `aylis-controls` ficou como fachada: reexporta
  tudo (a mesma API, `aylis_controls::bi`, `aylis_controls::industrial`...), guarda o `register`
  na mesma ordem e os testes. A macro `leaf_control!` foi para o `aylis-core`. Medido depois
  (mesma máquina): do zero 11,8 s (debug) e 11,4 s (release), contra 15,7 e 15,5; mudar um
  arquivo de gráfico 0,85 s (debug) e 4,3 s (release); mudar um arquivo da base 1,4 s (debug)
  e 10,4 s (release), contra 2,1 e 19. O guia `docs/criar-componente.md` diz onde fica um
  controle novo.

  Feito em 02/11 (falta o autor confirmar): o cliente do protocolo não é mais copiado para o
  projeto: vem da pasta `clients` do aylis (a instalação, ou o repositório; relativo quando o
  projeto está dentro do aylis, como os exemplos). TypeScript, Python, C++ e Pascal têm um
  arquivo-ponte gerado (`src/aylis.ts` reexporta, `src/aylis_client.py` carrega, `src/aylis.hpp`
  e `src/aylis.pas` incluem o da instalação), que funciona também fora da IDE; o C# compila o
  `clients/csharp/Aylis.cs` pelo `App.csproj` (o de antes ganha a linha); o Go aponta o módulo
  `aylis.dev/cliente` para a instalação no `go.mod` (o código que importava `app/src/aylis` passa
  a importar o módulo); o Java junta o `dev.aylis` da instalação na compilação (fora da IDE, é
  preciso o caminho). A geração apaga as cópias antigas (só os arquivos com a marca do cliente),
  e um aylis novo vale para todos os projetos. Na IDE, os arquivos-ponte não aparecem, e o
  arquivo gerado (a pasta `generated/`, o acesso tipado, o registro) abre somente leitura, com
  a faixa "Gerado pela IDE do Aylis a cada 'Gerar código'". O tempo com 100 forms (99 com 14
  componentes e 2 handlers), em `medicoes/compilacao_100_forms.json`: gerar o código 0,4 s; em
  debug, o crate inteiro 4,4 s, um form mudado 2,1 s, um componente novo 3,5 s; em release, 16 s.
  Corrigido no caminho: um form sem componentes não compilava em Rust (o `problems` sem tipo).
  A árvore do projeto agrupa por dono: Forms (cada form com a unit e o JSON), Programa, Units e
  "Gerado pelo aylis (somente leitura)", recolhido; com o modelo do item 2 as units não têm mais seção
  gerenciada (nada a sombrear). Decidido pelo autor em 03/11: só a linguagem escolhida, com
  possibilidade de trocar depois. É o que já acontece: o projeto novo nasce com a pasta da
  linguagem escolhida; Projeto → Opções do projeto troca a linguagem, a geração cria a pasta
  dela e a anterior fica intacta (com as pendências em "Pendências entre linguagens"). Os
  exemplos do aylis mantêm as 8, para testar todas as linguagens.

  Medido em 03/11 (falta o autor confirmar; `medicoes/protocolo.json`, o teste `protocol_cost`
  do `aylis-remote` em release): numa tela comum, um comando custa uns 8 µs (69 bytes) e um
  evento 3 µs (155 bytes), fora a ida e volta do processo: o JSON não pesa. A carga de 100 mil
  registros com 5 campos: 8,3 MB e 368 ms (montar o texto 122, ler 89, carregar 158). Só a
  carga pesa, então entrou o formato colunar só para ela (`data load_columns`, com `fields` e
  uma lista de valores por campo): 4,5 MB e 205 ms (33, 24 e 147); a conversão do `load`
  também passou a achar cada campo uma vez, não a cada valor. O que sobra é montar o dataset
  (147 ms); um formato binário pouparia só a leitura (24 ms), então fica o colunar em JSON.

**ERP e projetos corporativos (análise de 04/11/2026)**

Pergunta do autor em 04/11: onde daria errado ou ficaria difícil fazer um ERP corporativo no
aylis. Fica fora daqui o que a análise de 02/11 já resolveu ou decidiu (nomes de evento e
handler conferidos antes de compilar: itens 1 e 1b; código gerado só da linguagem escolhida e
tempo com 100 forms: itens 11.2 e 12). Ainda falta discutir item por item, e a discussão pode
mudar a solução.

- [ ] 16. Provider de banco de referência. O `aylis-data` não sabe de banco (de propósito): o
  delta vai ao backend e cada equipe escreve a gravação (o `clients/python/registration_backend.py`
  faz à mão, num SQLite). Falta o que o `TDataSetProvider`/FireDAC dá: transformar o delta em
  SQL, controlar a concorrência (`UpdateMode`: chave, campos alterados, todos), mestre-detalhe
  gravado em cascata numa transação, devolver ao cliente as chaves geradas e os valores do
  banco. Pronto quando houver um provider (pelo menos Postgres) num crate próprio e, nas
  linguagens remotas, o mesmo no cliente ou um guia com o padrão.

  Revisto em 04/11 pelo autor: o foco do aylis é a interface; nada de provider nem de SQL no
  aylis (muda o foco e aumenta a superfície de testes e os riscos). A ideia dele: um método que
  entregue as alterações com filtro por incluídos, alterados e excluídos, e o programador
  converte em SQL do jeito dele. O que já existe: o `delta` (comando `data delta` e o
  `on_apply_updates`) já vem com `op` (`insert`, `update`, `delete`), `ref`, `key` (a chave
  original), `old` e `new` (só os campos alterados no update), e o `reconcile` devolve erro ou
  valores (a chave gerada) por registro; o `status_filter` (como o `StatusFilter`) mostra os
  registros por situação. Em discussão (resposta de 04/11): o que falta é pequeno e sem banco:
  - o filtro no próprio delta (`delta(kinds)`, ou `inserted()`/`updated()`/`deleted()` em
    cada cliente);
  - mestre-detalhe: o delta do mestre com os detalhes junto, na ordem segura para gravar
    (excluir os detalhes antes do mestre, incluir o mestre antes dos detalhes), e a chave
    gerada do mestre (devolvida no `reconcile`) repassada sozinha ao campo de ligação dos
    detalhes incluídos (`master_fields`); hoje não repassa;
  - um guia com o exemplo de gravação em cada linguagem.

  Decidido pelo autor em 04/11 (os três aprovados):
  - 16.1 o filtro por tipo no delta, no Rust e nos 7 clientes;
  - 16.2 mestre-detalhe: o delta do mestre leva os detalhes, na ordem segura, cada alteração
    dizendo o dataset e, no detalhe incluído sob um mestre incluído, o mestre de que depende;
    o resultado vai ao dataset certo pelo `ref`; a chave gerada do mestre passa sozinha aos
    detalhes; nos clientes, um ajudante que, ao aceitar o mestre com a chave gerada, já troca a
    chave nas alterações seguintes dos detalhes;
  - 16.3 o guia vai para a página do aylis (`examples/Landing`), numa seção de documentação
    completa: os componentes, os modos de usar e exemplos de cada coisa (pedido de 04/11).

  Feito em 04/11 (falta o autor confirmar):
  - 16.1 e 16.2 no `aylis-data`: o `Change` ganhou `data_set` e `master` (`MasterLink`); o
    `delta_with_details` (o que o `apply_updates` e o comando `data delta` mandam) leva os
    detalhes sem `apply_separately` (propriedade nova do ClientDataSet), na ordem segura; o id dos
    registros passou a ser único no processo, e o `reconcile` acha o dataset de cada resultado
    pelo `ref`; a chave gerada de um mestre passa aos campos de ligação dos detalhes na tela. O
    ajudante `Delta` (filtro por tipo, `accept` que troca a chave nas alterações seguintes) no
    Rust e nos 7 clientes (`Delta`, `TAylisDelta`, `NewDelta`, `aylis::Delta`), com o mesmo roteiro
    em todos (`SELFTEST <lang> delta 2 1 1 501 1`, nos testes `languages` e `python_backend`; o
    C++ conferido à parte com o Visual C++, que o teste não acha sem o `vcvars` nas Opções).
    Testes: `master_detail_delta` do `aylis-data`;
  16.1 e 16.2 conferidos e confirmados pelo autor em 07/11 (Sessão 2 do roteiro); falta a 16.3
  (Sessão 4).
  - 16.3: os guias `docs/gravar-alteracoes.md` (o delta em cada linguagem) e
    `docs/interceptador.md`; a documentação na página: `aylis-tool docs` (o exemplo `docs` do
    `aylis-ide`, `aylis_ide::reference`) copia os guias de `docs/` e gera uma página por componente
    registrado (paleta, descrição, propriedades com o nome do Object Inspector e o tipo, eventos e
    quais perguntam, um trecho de código), as comuns a todo controle e o índice, em
    `examples/Landing/assets/docs` (235 páginas); o form `frmDocumentacao` da Landing (botão
    "Documentação"): seções, páginas, busca em todas e os links entre páginas (teste
    `documentacao` da Landing). Falta: exemplos de uso escritos para cada componente (hoje a página
    traz só o trecho gerado) e um teste que avise quando a documentação ficar velha. Os exemplos
    escritos: adiados pelo autor em 05/11 ("para outro momento").

  Aprovado pelo autor em 04/11, como sugerido: com dados parciais, ordenar, filtrar e
  localizar viram um evento ao backend, que recarrega o dataset; sem handler ligado, a tela
  avisa ("ordenando só os 500 carregados"); o total de registros pode vir do backend; limite
  de memória opcional.

  Feito em 07/11 (pedido do autor: "faça" os aprovados; sem commit; falta o autor
  confirmar):
  - com mais registros no backend (`has_more`), `set_index`, `set_index_name`, `set_filter` e
    o `locate` que não acha entre os carregados disparam o `on_server_query` do dataset, com o
    texto JSON `{"kind": "sort"|"filter"|"locate", "index", "filter", "loaded"}` (mais
    `field` e `value` no "locate"); o backend recarrega (`load_rows` + `set_has_more`), e o
    que ele faz no dataset durante o evento não pergunta de novo (`DataSet::server_query`);
  - sem quem trate, os dados ficam como estão e o aviso vai para `partial_notice`, que a grade
    mostra numa faixa de atenção sobre as linhas ("Ordenado só entre os 50 registros
    carregados: há mais no servidor"; em inglês e espanhol também); some quando tudo chegou;
  - `record_total` (o total que o backend diz) dá o tamanho do polegar da barra da grade;
    `max_records` para a busca sob demanda no limite, com o aviso.
  Teste `server_query.rs` do `aylis-dbcontrols`.
- [ ] 18. Interceptador de eventos (no lugar da herança visual de forms). Decidido pelo autor em
  04/11: não haverá herança visual. O que um ERP precisa é tratar num lugar só qualquer evento
  de qualquer componente (registrar, conferir permissão, padronizar, cancelar), recebendo o
  form, o componente que disparou e os argumentos do evento; talvez um componente próprio, que
  vale para um form ou para a aplicação inteira. O que existe hoje: o `ApplicationEvents`
  (`on_key_down`, `on_key_press`, `on_exception`) e o `key_preview` do form; o `Ui::fire` só
  chega a quem tem handler ligado (um clique num botão sem `on_click` não passa por nada).
  Decidido pelo autor em 04/11 ("deve ser feito"), como proposto:
  - um componente não visual `EventInterceptor` (System), com `scope` (`form` ou
    `application`), `events` e `classes` (filtros opcionais; vazio = todos);
  - `on_intercept` antes do handler, com o form, o componente, o nome do evento, os argumentos
    e o handler ligado; é evento-pergunta (item 3), e o `cancel` faz o handler não rodar;
    `after_intercept` depois do handler (auditoria);
  - ordem: os da aplicação, os do form, o handler; vários na ordem de criação; o primeiro que
    cancela encerra; passa também pelos eventos sem handler ligado;
  - no "todos", ficam de fora os eventos contínuos (`coalesce`, movimento do mouse, pintura),
    com aviso em modo dev; só entram se pedidos pelo nome em `events`;
  - os controles com ação própria (navegador de dados, botão com `modal_result`) consultam o
    interceptador antes da ação;
  - em modo dev, o evento cancelado vai para o log com o nome do interceptador.

  Feito em 04/11 (falta o autor confirmar): `aylis_core::intercept` (o `fire` passa pelos
  interceptadores antes do handler, também sem handler ligado; `EventArgs::Intercept` com o
  `Intercepted`; `ui.cancel_event()`; um evento-pergunta cancelado é recusado; com o handler
  remoto, o evento e a pergunta dele esperam a resposta; o botão com `modal_result` não fecha o
  form se o clique foi cancelado); o componente `EventInterceptor` (System, nas três listas de
  classes, no catálogo e na paleta, com os rótulos e a descrição traduzidos); o `before_action`
  do `DataNavigator` (o `BeforeAction` do Delphi, que passa pelo interceptador); no protocolo, o
  evento interceptado em JSON (`form`, `sender`, `event`, `handler`, `args`) e a resposta
  `cancel`; no Pascal, a assinatura `(Sender; Info: TJSONObject; var Cancel: Boolean)` no
  cliente e na geração. Testes: `interceptor` do `aylis-controls` (cancelar, sem handler, ordem
  aplicação → form, filtros, contínuos fora do "todos", pergunta recusada, backend que responde
  depois) e `navigator_asks_before_action_and_a_refusal_keeps_the_record` do `aylis-dbcontrols`.
- [ ] 19. Renomear na IDE em todo o projeto. O `rename_component` do designer só troca o nome no
  form: o código que usa o componente (`self.edtNome`, `frmX.edtNome`), os handlers com o nome
  dele, as referências em outros forms e frames e os nomes em texto ficam para trás. O 1b acha
  o problema, mas não corrige. Pronto quando renomear componente, form ou handler atualizar
  form, código de cada linguagem e as outras referências, com uma prévia do que vai mudar.
  Aprovado pelo autor em 04/11: nas outras linguagens, usar o LSP de cada uma para renomear
  no código (`textDocument/rename`).
  Conferido em 08/11 (roteiro 59): não aprovado. Renomear o `cdsPedidos` para `cdsVendas` pelo
  Object Inspector renomeou também o form (a raiz e o `main_form` do `.aylisproj`); o handler
  renomeou certo (veja os ajustes de 08/11, grave).

  Feito em 07/11 (sem commit; falta o autor confirmar; `crate::rename`). No código, pelo
  servidor de linguagem, como aprovado (a primeira versão, por texto, foi mostrada ao autor,
  que confirmou em 07/11: "LSP como o planejado"): o `textDocument/rename` perguntado no
  primeiro uso do código do usuário acha só o que é de fato aquele símbolo; a prévia leva a
  troca exata de cada trecho que ele deu, sem o código gerado (a geração refaz). Sem o
  servidor pronto (abrindo, sem servidor, ou se ele não acha o símbolo), as ocorrências por
  texto, com o aviso na prévia. Nos forms, pelo JSON:
  - renomear um componente no Object Inspector: o próprio form já seguia (as referências são
    pelo componente) e agora as vistas também (guardavam o nome); se outros forms
    (`frmClientes.cdsCliente`) ou o código falam do nome antigo, a janela da pesquisa abre com
    cada ocorrência marcada (a prévia), e o "Substituir" troca as marcadas (o "Desfazer a
    substituição" e o Ctrl+Z voltam);
  - renomear o form: o mesmo, e o tipo dele no código (`frmX` → `FrmX`), cada um pelo
    servidor;
  - digitar outro nome no evento, com o handler antigo no código e o novo ainda sem método:
    é renomear o método e os outros eventos ligados a ele (escolher "Novo" no ▾ continua
    ligando a um handler novo).
  Testes `rename.rs` da IDE; com o rust-analyzer de verdade no exemplo Abas
  (`the_language_server_finds_the_code_to_rename`, ignorado: 47 s indexando), o uso no código
  do usuário com a troca exata e a declaração no gerado fora da prévia.

  Feito em 07/11 (sem commit; falta o autor confirmar): "Onde é usado…" no menu de contexto do
  designer e em Editar (`crate::usages`), na janela da pesquisa (a lista com clique que leva
  ao código na linha ou ao form com o componente):
  - componente: as propriedades dos forms que apontam para ele (também de outro form, pelo
    nome completo), as ligações (`bindings`), as vistas e o nome no código;
  - campo (um `Field` escolhido): os controles ligados ao dataset dele pelo `data_field`, as
    colunas, as expressões, o filtro e o índice do dataset, o `[campo]` dos textos do
    relatório e o nome entre aspas no código;
  - handler: os eventos ligados a ele e o nome no código;
  - o código gerado fica de fora; a ferramenta `usages` no MCP da IDE dá o mesmo aos agentes.
  Testes `usages` (no Relatórios de exemplo) e os do módulo.

  Feito em 07/11 (sem commit; falta o autor confirmar):
  - o formato fixo (`aylis_json::to_form_text`): uma chave por linha, listas curtas numa linha
    (`"margins": [8, 4, 8, 8]`), a ordem da classe, `children` por último; a IDE e o
    `aylis_json::save` gravam assim (antes, o `to_string_pretty` abria cada número numa
    linha);
  - `aylis-ide --format-forms projeto.aylisproj` passa um projeto para o formato uma vez (só a
    forma, nunca o conteúdo): os 1.023 forms dos 14 exemplos foram convertidos;
  - corrigido no caminho: o `value` gravado antes do `max` (a ordem da classe) era cortado no
    máximo padrão ao abrir (o NumberBox do GradeGigante, 10.000.000 → 1000): os limites
    (`min`, `max`...) carregam antes; as vistas gravavam `599.0` no lugar de `599`;
  - teste `form_roundtrip` (abrir e salvar dá o mesmo arquivo, byte a byte, em todos os
    exemplos; do ERP, uma amostra de 1 em 20) e o do `aylis-json`;
  - o guia `docs/forms-em-equipe.md` (o formato, resolver um conflito, evitar conflitos).

  Feito em 07/11 (sem commit; falta o autor confirmar; o subrelatório mestre-detalhe já
  existia, nas bandas `sub_header`/`sub_detail`/`sub_footer`):
  - código de barras e QR Code: o controle `Barcode` (paleta Reports; o dado em `text`, então
    `[campo]` numa banda dá o código de cada registro), pelo provedor ligado ou o do Aylis:
    Code 128, EAN-13, EAN-8, UPC-A, Code 39, ITF (o do boleto) e QR Code (crate `qrcode`);
    um formato novo é outro provedor (o teste liga um);
  - exportar: o componente `Export` (paleta Data: `data_source`, `fields`, `sheet_name`,
    `save_to(caminho)`), pelo `CsvProvider` (o padrão: `;`, vírgula decimal, BOM) ou pelo
    `XlsxProvider` (crate `rust_xlsxwriter`: títulos em negrito e congelados, números com o
    formato do campo, datas, largura pelo conteúdo);
  - relatório grande: `report::write_pdf(ui, report, arquivo)` grava o PDF página por página
    (o novo `aylis_core::pages::PdfWriter`, que o `to_pdf` também usa), sem guardar as páginas;
    com `[PAGES]`, monta duas vezes (a primeira só conta). Mesmo PDF do `to_pdf`, byte a byte,
    no teste com 3.000 registros;
  - impressora térmica (ESC/POS): fora, como decidido.
  Testes `providers.rs` do `aylis-providers` e `a_big_report_is_written_page_by_page...`.

  Feito em 07/11 (sem commit; falta o autor confirmar):
  - no núcleo, `aylis_core::providers`: `BarcodeProvider`, `ExportProvider`, `PdfProvider` e
    `MapTileProvider`; o componente provedor devolve o seu em `Component::provided`;
  - a propriedade `provider` no `Barcode`, no `Export`, no `Report` (o PDF) e no `MapView`
    (o endereço dos blocos e o crédito); sem provedor, o padrão do Aylis;
  - o crate `aylis-providers` (feature `providers` da fachada, ligada pela IDE quando um form
    usa as classes): `Barcode`, `Export` e os provedores `AylisBarcodeProvider`, `CsvProvider`,
    `XlsxProvider`, `AylisPdfProvider` e `TileServerProvider` (com `{s}` para dividir os
    pedidos), na aba "Providers" da paleta (as abas da paleta ficam em inglês, como as
    outras);
  - nas três listas de classes, no catálogo e com os rótulos das propriedades nos 12 idiomas.
- [ ] 26. Fonte enorme no editor de código (pedido do autor em 05/11: "no projeto ERP está lento
  a navegação no fonte mod.rs"; o minimapa, arrastado até embaixo, não chegava ao fim do
  fonte; "garanta que fique rápido com quantidade infinita de linhas, limitado apenas pela
  memória do computador").
  Feito em 05/11 (sem commit; falta o autor confirmar):
  - O minimapa: a escala tinha um piso de 0,25 px por linha; com 8 mil linhas o fim ficava
    fora da faixa e o arrasto parava por volta da linha 3.200. Agora o arquivo inteiro cabe e
    o arrasto chega à última linha (teste `dragging_the_minimap_to_the_bottom_reaches_the_last_line`).
  - Navegar sem ler o texto inteiro: a chave dos caches deixou de ser o hash do texto (um
    contador de versões); a posição em bytes, o tamanho, a linha e a coluna do cursor, as setas,
    a palavra do cursor, as ocorrências, o par do colchete (guardado), a calha (só as linhas
    à vista) e o minimapa saem da tabela das linhas; as linhas visuais ficam guardadas até o
    texto mudar; o realce sem gramática de um texto grande vem aos poucos.
  - Medido (`cargo run --release -p aylis-controls --example large_text <linhas>`, Skia CPU,
    1600×900), com 8 mil / 100 mil / 1 milhão de linhas: rolar com a roda 2,5 / 2,6 / 2,6 ms por
    quadro; arrastar o minimapa 7,5 / 7,5 / 7,5 ms; primeiro quadro 20 / 24 / 150 ms. Antes (o
    mod.rs de 8 mil linhas): roda 0,3 ms no evento + 3,2 ms no quadro. Na primeira medição
    com 100 mil e 1 milhão (já com o minimapa e as linhas visuais guardadas): roda 8,8 / 59,6
    ms (mais 15 ms no evento com 1 milhão), minimapa 31 / 252 ms, primeiro quadro 165 /
    1.598 ms.
  - Falta: editar. Digitar ainda cresce com o tamanho (8 mil linhas: 15 ms por tecla; 1 milhão:
    270 ms): o texto é um `String` só (inserir move o resto), a tabela das linhas e as linhas
    visuais são refeitas a cada tecla, o realce recomeça do início e o desfazer guarda uma
    cópia do texto inteiro a cada alteração. Proposta: texto em pedaços (rope), tabela das
    linhas e linhas visuais atualizadas só na linha editada, realce retomado da linha editada
    (o estado de cada linha guardado) e desfazer pelas diferenças. Espera a decisão do autor.
- [ ] 27. Tabela da documentação com o texto da primeira coluna por cima da segunda (achado na
  conferência do roteiro em 05/11, na Landing publicada para a web): na página "Fontes e
  ícones offline", os caminhos `resources/fonts/...` e `resources/icons/...` passam da
  coluna "Recurso" e ficam por cima de "O que é". A coluna precisa quebrar o texto (ou ter a
  largura do conteúdo).
  Feito em 05/11 (sem commit; falta o autor confirmar): no `aylis-markdown`, cada coluna tem
  de piso a palavra mais larga dela (medida com a fonte do trecho, a mono no código) e o que
  sobra é dividido pelo quanto cada uma ainda queria (antes todas encolhiam na mesma
  proporção); uma palavra mais larga que a linha quebra por caractere. Teste
  `table_cells_never_overlap_the_next_column`.
- [ ] 28. Sem referências ao Delphi no site (pedido do autor em 05/11: "pode remover
  referencias ao Delphi"). Feito em 05/11 (sem commit): na página da Landing, o título
  ("montadas visualmente"), o cartão do Free Pascal ("Units, classes e eventos com var, no
  jeito do Pascal") e o "E os dados?"; nos guias que vão para a documentação do site
  (`docs/criar-componente.md`, `gravar-alteracoes.md`, `interceptador.md`, `threads.md`) e na
  cópia deles na Landing. Os comentários do código e os documentos internos continuam com as
  comparações.
- [ ] 29. Na documentação da Landing (web), um clique na lista de guias não abriu a página; com
  dois cliques abriu (visto na conferência de 05/11). A lista saiu no item 30 (agora é uma
  árvore); conferido em 05/11: um clique na árvore abre a página.
- [ ] 30. A documentação em tela cheia, no jeito do Docusaurus (pedido do autor em 05/11: "a
  documentação tem que abrir em tela cheia, seguindo um layout de navegação igual ao
  https://docusaurus.io/docs"). Feito em 05/11 (sem commit; falta o autor confirmar):
  - O form `frmDocumentacao` maximizado; no navegador, um form maximizado (ou em tela cheia)
    ocupa a página inteira, sem a barra de janela (`web_windows.rs`), como no desktop.
  - No topo, a marca, a busca e "Voltar ao site"; à esquerda, as seções em árvore (só a da
    página aberta fica aberta, com a página marcada); no meio, a trilha ("Guias › página"), a
    página e "Anterior" / "Próxima" com os títulos; à direita, "Nesta página" (os títulos de
    nível 2 e 3, que rolam até eles). Abaixo de 1.100 px de largura, sem "Nesta página".
  - A busca olha o título e o texto das páginas; os achados ficam no lugar da árvore.
  - A `Markdown` ganhou a propriedade `anchor` (rola até um título, pelo texto ou pelo
    identificador), e os links `#titulo` e `pagina.md#titulo` funcionam.
  - Teste `examples/Landing/rust/tests/documentacao.rs` refeito para o layout novo.
  - A barra de rolagem da página não respondia ao mouse (pedido do autor em 05/11: "a scroll
    não é clicavel"): a `Markdown` só rolava com a roda. Agora clicar na faixa da barra (os
    12 px da direita) pula para lá e arrastar o polegar rola junto (teste
    `the_scrollbar_is_clickable_and_draggable`; conferido na Landing publicada). A barra da
    árvore já respondia (teste `the_scroll_bar_of_a_long_tree_can_be_dragged`).
  - O hover da árvore ficava algumas linhas abaixo do mouse depois de abrir uma seção
    (relatado pelo autor em 05/11). A causa, nas listas (ListBox, ListView, TreeView): a
    lista rolada que encolhia tinha a rolagem limitada só na pintura, e o mouse e o clique
    contavam a linha com a rolagem velha. Agora a pintura grava a rolagem limitada (teste
    `a_tree_that_shrinks_keeps_the_mouse_on_the_row_under_it`; conferido na Landing).
- [ ] 33. Documentação com tutoriais e imagens (pedido do autor em 05/11: "sinto que a
  documentação está chata e com muito texto e pouca imagem [...] falta um tutorial simples
  para criar um novo projeto e um para criar um novo componente"; "não entendi o que significa
  o check e o NEW do ícone.. não vejo necessidade"). Decidido pelo autor: código em Rust com
  abas das outras linguagens; o tutorial de projeto faz o "Olá, nome"; o de componente, um
  botão com contador; imagens geradas automaticamente. Feito em 05/11 (sem commit; falta o
  autor confirmar):
  - Guia do Vuetify sem a coluna de ícones (✅, 🆕, ◐).
  - Seção "Comece aqui" no índice: a visão geral e os tutoriais `docs/tutorial-primeiro-
    projeto.md` (o projeto `examples/OlaNome`, com teste) e `docs/tutorial-componente.md` (o
    pacote `examples/pacotes/BotaoContador`, com teste; o código entra na página direto do
    arquivo, `<!-- incluir: caminho -->`, então não fica velho).
  - Imagens geradas pelo `aylis-tool docs` (`aylis_ide::reference`), sem janela: a tela inicial
    da IDE, o designer com o projeto, o app rodando e o botão com contador (em `docs/img`, que
    vão para a Landing), e o desenho de cada componente visual no topo da página dele (168).
    A IDE do gerador usa uma configuração à parte (`actions::use_private_config`), sem mexer
    nos recentes de quem roda.
  - `Markdown`: blocos de código seguidos em linguagens diferentes viram abas (a escolha vale
    para a página e as próximas); imagens com caminho relativo à página; no navegador, a
    imagem guardada por caminho (cada `Image` nova recomeçava a decodificação e nunca
    aparecia) e o layout refeito quando ela termina de carregar.
  - Atenção: outra sessão regenerou `examples/Landing/assets/docs` com um gerador anterior
    (às 22:35) e a pasta perdeu os tutoriais e as imagens; regerei depois. Antes do commit,
    conferir que a pasta está com a seção "Comece aqui".
- [ ] 24. (Adiado pelo autor em 04/11: cedo demais, nada foi lançado ainda.) Versões e suporte.
  Quando o projeto ficar público, o suporte é pela comunidade, com o autor como mantenedor e
  revisor. A política de versões (estável, por quanto tempo uma versão recebe correção, como
  uma mudança incompatível é anunciada, o changelog) volta a ser discutida no lançamento.
- [ ] 31. Redimensionar a janela pela borda (achado no NoOBS em 05/11: "os componentes são
  empurrados mesmo tendo espaço pra eles, só depois que eu solto o clique que eles se
  redimensionam"; depois, "deveria ter uma largura mínima onde parasse de tentar quebrar
  linha"). Feito em 05/11 (sem commit; falta o autor confirmar):
  - Arrastando a borda no Windows, o laço do sistema não passa pelo `about_to_wait`, e os
    `set_timeout` do `on_resize` só rodavam ao soltar. Agora o quadro refaz o layout e roda
    os timers vencidos antes de pintar (`aylis-platform-winit`, `redraw`).
  - Propriedades novas em todo controle: `min_width`, `min_height` e `max_height` (com a
    `max_width` que já havia), como o `Constraints` da VCL. O layout não deixa o controle
    sair delas em nenhum alinhamento; com a mínima maior que a área, ele encosta à esquerda e
    o resto fica cortado (ou rola, num ScrollBox), e um texto com `wrap` para de quebrar ali.
    No form, viram o tamanho mínimo e o máximo da janela. Testes
    `min_width_stops_shrinking_and_the_rest_is_cut` e `a_form_is_never_smaller_than_its_minimum`.
  - No NoOBS, as Configurações acertam só a coluna a cada quadro do arrasto (a aba inteira,
    150 ms depois de parar) e a página tem largura mínima (coluna de 280 px).
  - Trocar de aba depois de redimensionar deixava a coluna estreita (relatado em 06/11; o
    autor pediu a correção no core: "não ter que fazer isso em todo componente"). A causa: o
    layout é adiado para o quadro e pula o que está escondido, então a página que acabou de
    aparecer ainda tinha a largura antiga. Agora `Ui::bounds` (`&mut self`) roda o layout
    pendente antes de responder, como o realinhamento imediato da VCL (sem nada pendente,
    só confere uma flag); `Ui::laid_out_bounds` lê o do último layout, para quem só tem
    `&Ui` (pintura). Teste `bounds_read_right_after_a_change_are_already_laid_out`; a tela
    gigante segue na base.
  - O fundo do seletor de tema (e outros controles) mudava de tamanho ao redimensionar, longe
    da borda (relatado em 06/11): o app fazia "largura própria encostada à esquerda" numa
    pilha `top` com margem direita = coluna − largura, conta que fica velha quando a coluna
    muda. Propriedade nova `self_align` (`stretch`, o padrão; `start`, `center`, `end`), como
    o `align-self` do CSS: encaixado em cima ou embaixo, o controle fica com a própria
    `width` no começo, meio ou fim (à esquerda ou à direita, a `height`). Teste
    `self_align_keeps_the_own_size_at_the_start_center_or_end_of_the_band`. No NoOBS, o
    fundo do tema usa `self_align: start` e os textos criados em código, `auto_size` na
    largura da coluna.
- [ ] 34. Texto copiável no `Markdown` (pedido do autor em 06/11: "é possível deixar a
  documentação com os textos copiáveis?"). Hoje o componente não tem seleção, e o navegador
  não tem área de transferência (só desktop com `arboard` e Android). Proposta: arrastar
  seleciona (na ordem de leitura, atravessando parágrafos, tabelas e código), duplo clique
  pega a palavra e triplo o parágrafo, Ctrl+C e Ctrl+A, menu do botão direito com Copiar, e
  um botão Copiar no canto de cada bloco de código; na web, `navigator.clipboard.writeText`
  (e o colar pelo evento `paste`, que também serve ao `Edit` e ao `Memo`). Feito em 06/11 (sem
  commit; falta o autor confirmar), com o realce de sintaxe nos blocos de código (o do `Memo`,
  `aylis_controls_base::syntax`) e o menu Copiar do botão direito desenhado pelo próprio
  componente; testes em `crates/aylis-markdown/tests/markdown.rs` e no `layout.rs`.
- [ ] 35. Componentes de verdade na documentação, com carregamento em partes (pedido do autor em
  06/11: "vamos colocar os componentes de verdade na documentação, precisamos ter o
  carregamento em mais de um wasm para não ficar gigante e não travar o carregamento, o
  carregamento inicial deve ser rápido"). Hoje a Landing é um wasm só de ~11,8 MB (3D, Markdown
  e vetores; sem os controles das outras famílias). Aguardando a decisão do formato (ver a
  conversa de 06/11). Decidido em 06/11: um wasm por família (a categoria da paleta). Feito em
  06/11 (sem commit; falta o autor confirmar): o crate `aylis-demos` (registra só as classes de
  `AYLIS_DEMO_FAMILY`, com `aylis_build::class_categories`) e `aylis-tool demos --out pasta` (as 18
  famílias, de 3 a 10 MB cada); a página de um componente visual, no navegador, mostra o
  componente de verdade no painel "Ao vivo" (um `WebView`, que agora também funciona nos forms
  abertos por cima no navegador). O lingote 3D do topo também ficou sob demanda (pedido do autor
  em 06/11): no navegador vem do wasm da família 3D num `<iframe>`, e o wasm da página caiu de
  11,4 MB para 5,8 MB (2,2 MB comprimido); fora do navegador, o `Model3D` de verdade (a feature
  `3d` só fora do wasm). De quebra: a vista nativa dentro de uma caixa rolada andava o dobro
  (a rolagem era descontada duas vezes) e agora leva o recorte da caixa (`NativeViewSlot::clip`,
  `clip-path` no navegador); e o brilho do `aylis-3d` no navegador não acompanhava a mudança de
  tamanho (o WebGPU recusava todo quadro e o 3D sumia).
- [ ] 36. Página do Aylis: seções com contadores (pedido do autor em 07/11). Feito em 07/11 (sem
  commit; falta o autor confirmar): saiu a fileira de números (Componentes, Linguagens,
  Plataformas, Idiomas) e cada seção ganhou o contador em cima do título ("8 linguagens", "6
  plataformas", "220 componentes", "12 idiomas"); as plataformas viraram cards com ícone como os
  das linguagens (Devicon e Simple Icons, licenças em `assets/images/plataformas/LICENSE.txt`); 11
  componentes em cards com a imagem e o "e muito mais" levando à documentação; os 12 idiomas da
  IDE, cada um escrito na própria língua; o Free Pascal com o símbolo do Lazarus (Simple Icons,
  CC0); o texto do topo sem "no Windows, no Linux, no...". Depois: o TimePicker no lugar do
  Manometer, o Tux colorido, os contadores maiores e sem as perguntas frequentes.
- [ ] 32. O hover de botão transparente piscava escuro e "voltava ao normal" (achado no NoOBS em
  06/11: "o hover dos botões dão só uma piscada com o efeito e depois ficam normais"). O estado
  de hover ficava certo; a transição do estilo misturava as cores canal a canal, e o
  `transparent` (preto com alfa 0) passava por cinzas escuros meio opacos até chegar no
  cinza-claro do hover. Feito em 06/11 (sem commit; falta o autor confirmar): `Color::lerp`
  mistura com alfa pré-multiplicado (entre cores opacas, igual a antes). Teste
  `mixing_from_transparent_does_not_go_through_black`.
- [ ] 37. Cada componente aparece direito na demonstração ao vivo (pedido do autor em 07/11:
  "alguns componentes não cabem dentro do container dele e outros não mostram nada"). Feito em
  07/11 (sem commit; falta o autor confirmar): `aylis::samples` (conteúdo de exemplo por classe,
  em `crates/aylis/src/samples.json` e por código: o dataset do DataGrid, as mensagens do
  ChatView, os nós do NodeGraph, a árvore aberta, o lingote no Model3D, os arquivos do Svg e do
  Lottie); a demonstração tem o tamanho do componente mais a folga e encolhe para caber (nunca
  passa da borda), e a página manda o tamanho natural para o quadro "Ao vivo" (`postMessage`);
  sem demonstração possível (câmera, relatório, documento, peças de outro componente:
  `aylis_demos::WITHOUT_DEMO`), o quadro some e volta a imagem. A galeria da IDE usa os mesmos
  exemplos, e as imagens da documentação foram refeitas com eles. O teste
  `crates/aylis-demos/tests/visible.rs` desenha cada demonstração sem janela e reprova a que sai
  lisa ou passa do quadro (`AYLIS_DEMO_SHOTS=pasta` grava as imagens). O contador da página foi
  corrigido para 222 componentes (eram 220; a página do DocViewer não sai no gerador).
- [ ] 33. Instaladores NSIS e Inno Setup no Publicar, e tradução fora da tela (pedidos do autor em
  07/11, ao alinhar o NoOBS ao padrão dos projetos Aylis: "se o aylis-tool não suporta o nsis deve
  começar a suportar, deve ter uma opção no projeto para configurar o tipo de instalador";
  "adicione suporte ao inno setup também"; e, sobre traduzir na bandeja e em threads, "esses
  componentes então já deveriam ter essa opção fácil usando o Aylis"). Feito em 07/11 (sem
  commit; falta o autor confirmar):
  - `publish.windows` no `.aylisproj` (`aylis_project::WindowsOptions`): `installer` (`msi`, o
    padrão; `nsis`; `inno`), `files` ao lado do exe (`runtime/*.dll`), `install_dir` e
    `data_dir` com `{localappdata}`/`{appdata}`/`{programfiles}`/`{documents}`, `exe_dir`,
    `autostart` e `desktop_shortcut` (`on`/`off`/vazio), `autostart_args`, `close_running`,
    `run_after_install`. O gerador fica em `aylis-tools/src/win_installer.rs`; os dois `.exe`
    instalam por usuário, com os dados de versão (sem eles o Defender marcava o instalador).
    `aylis-tool publish ... --installer` (o `--msi` continua valendo). Conferidos os dois com o
    NoOBS: geram, os dados de versão saem certos e o Defender não acusa.
  - IDE: Projeto → Opções do projeto → Windows (`windowsview.rs`); no Publicar, "Windows:
    instalador (o tipo do projeto)". Textos traduzidos nas 11 línguas.
  - `aylis::tr("...")` e `aylis::tr_with("{0} de {1}", &[..])`: o mesmo dicionário do `ui.tr`,
    de qualquer lugar e thread; o extrator de textos acha os dois. Teste `global_tr.rs`.

- [ ] 34. Acesso tipado fora dos handlers e a geração que perdia mudanças (pedidos do autor em
  07/11: "a forma de usar a interface ... estava localizando componentes pelo nome, mais
  deveria estar tipado"; e "verifique a falha do teste e corrija o problema").
  - `FrmX::components(ui)`: a cópia dos componentes do form (struct `FrmXComponents`, `Copy`),
    guardada pelo `bind` gerado (`Ui::set_form_components`/`form_components`). Vale também
    enquanto um handler do próprio form roda, quando o `FrmX::get(ui)` dá `None` (a instância
    está emprestada). Teste em `aylis-core/tests/instances.rs`; o NoOBS inteiro passou a usar.
  - NSIS: numa atualização, "iniciar com o Windows" e o atalho vêm como o usuário deixou
    (`AylisKeepChoices`), não no padrão de novo.
  - A geração decidia "mudou desde a última vez" pela hora em milissegundos com `>`/`>=`: um
    arquivo gravado no mesmo milissegundo do marcador (ou no mesmo segundo, numa pasta do
    OneDrive, que guarda a hora em segundos) parecia não ter mudado, e o form não era refeito
    (o `multi_language` falhava em ~4 de 10 rodadas). Agora cada arquivo tem a marca dele
    (`aylis_project::FileStamp`: a hora em nanossegundos e o tamanho), guardada ao gerar
    (`generated/components/recorded` para o código, `.../sources` para os JSON dos forms), e
    "mudou" é ser outra. O F12 só atualiza as marcas do form dele. 30 rodadas sem falha.
  - Peça repetida como frame (pedido do autor em 07/11: "acredito que essa parte deveria ser
    frame, devemos criar o mínimo possível em tempo de execução"): o gerador faz
    `FraX::create_in(ui, pai)` (mais uma instância dentro do pai, com os componentes tipados) e
    `FraX::components_of(ui, raiz)`. Como dois irmãos não podem ter o mesmo nome, a instância
    ganha um livre (`Ui::free_name_in`: `fraX`, `fraX2`...), e a instância do form guarda o
    nome com que foi criada (`Ui::form_class`), que é o que acha os handlers dela (antes, um
    form renomeado perdia os handlers). Teste em `aylis-core/tests/instances.rs`. O NoOBS passou
    a declarar em JSON o que montava em código: as telas inteiras (a Exportação era montada por
    ~550 linhas) e 25 frames para as peças repetidas; não sobrou `ui.create("...")` nem
    `set_prop`/`get_prop` por nome fora dos testes. Telas comparadas antes × depois
    (`NOOBS_TOUR`, 17 capturas): no máximo 0,05% dos pontos, nos medidores e no vídeo.
  - `ScrollBox` (pedido do autor em 07/11: "deveria avisar [quando o usuário rola] e com opção
    informar quais componentes estão visíveis dentro dele e com opção de posicionar em um
    componente para manter ele todo visível"): o evento `on_scroll` (só o usuário: roda, dedo,
    barra; o código posicionando não dispara), a propriedade `scroll_position`, os métodos
    `scroll_to(componente)` (rola o mínimo para ele ficar inteiro à vista) e
    `visible_components(lista, inteiros)` (os caminhos separados por `;`; em Rust,
    `ScrollBox::visible_children`). As contas usam a rolagem do último layout
    (`ControlBase::laid_out_scroll`), porque no `invoke` a caixa está fora do `Ui`. Testes em
    `aylis-controls/tests/scroll_box.rs`. O NoOBS usa na transcrição da lateral do player
    (acompanha a fala atual; o usuário rolando para longe, fica onde ele deixou).
  - Argumento de componente nos métodos tipados (pedido do autor em 07/11): o catálogo grava
    `component` (antes `string`) e cada linguagem pede o componente dela e manda o id
    (`scroll_to(self, ui, a1: impl Handle)` em Rust, `AylisComponent`, `Aylis.AylisComponent`,
    `aylis.Ref` — interface nova do cliente Go —, `const aylis::Component&`, `TAylisComponent`); o
    `Ui::invoke` entrega ao componente o argumento já como componente, venha pelo caminho ou
    pelo id numérico. Testes: `aylis-project/tests/multi_language.rs`
    (`a_component_argument_takes_the_component`), o `scroll_box.rs` pelo id, e o
    `aylis-host/tests/language_projects.rs` agora compila um `ScrollBox` em cada linguagem.
  - Quadros com a tela parada (pedido do autor em 07/11: "por que mesmo com a tela sem nenhuma
    alteração ele marca 120fps?"): o `LevelMeter` pedia um quadro por atualização do monitor
    na queda do pico mesmo sem `show_peak` (teste em `recording_widgets.rs`). No NoOBS, os
    medidores e a prévia das telas seguiam atualizando (e o `invalidate` refazia a janela
    inteira a cada 100 ms) com as Configurações cobrindo a tela principal: agora param
    enquanto ela está coberta e repintam só o controle (`ui.damage`). Medido com
    `AYLIS_WAKE_TRACE`: Configurações de 139 para 2–3 quadros por segundo; a tela principal
    continua em ~139, porque cada nível novo (a cada 100 ms) desliza em 90 ms na taxa do
    monitor.
  - Controle escondido atrás de outro não pinta nem pede quadros (pedido do autor em 07/11:
    "só não pode ficar pedindo repintura quando o componente não está visível... deve
    verificar se tem mais componentes com esse problema"). O `Component::opaque(&style)`
    diz que a pintura cobre o retângulo inteiro com cor opaca (`Rectangle`/`Panel` de cor
    sólida sem cantos, `Form` de fundo opaco); o `Ui` guarda quem cobre (ele ou um filho do
    tamanho dele, `Visual::covers`) e não pinta o irmão que fica inteiro atrás (com a sombra;
    quem não recorta os filhos só se a área pintada inteira estiver coberta). Se a cobertura
    deixa de cobrir, o quadro seguinte repinta a área. Teste
    `a_control_covered_by_an_opaque_sibling_does_not_ask_for_frames`; `giant_screen` em
    release sem regressão (mesma área e controles; rolar 21 ms). Revisados os ~45 pedidos de
    quadro dos controles: o alarme e a torre de sinalização (industrial) pediam um quadro por
    atualização do monitor para piscar 2 vezes por segundo (agora só na troca); o letreiro do
    trader pedia parado com o mouse em cima; o Markdown conferia as imagens carregando a cada
    quadro (agora 20 vezes por segundo, por até 10 s). Os outros só pedem enquanto algo anda.
    O `cargo test -p aylis-ide --test visual` falha no designer (0,74%): os cabeçalhos de
    categoria do Object Inspector mudaram, de outra sessão (falha igual sem a oclusão).
  - Roda e alças na prévia da exportação do NoOBS (pedido do autor em 07/11: "ao usar o
    scroll dentro para aumentar ou diminuir o zoom, a janela de fora scrolla também"; "os
    quadrados brancos nas bordas do vídeo deveria ser clicável para eu redimensionar"):
    propriedade comum `captures_wheel` (a roda em cima do controle fica com ele, os
    contêineres de fora não rolam; rótulo traduzido em `properties.tsv`), e
    `Component::hit_slop` (o controle recebe o mouse um pouco além da borda; fora dela decide o
    `contains_point`). O `CropBox` `framed` usa os dois: as alças desenhadas do lado de fora da
    seleção pegam o mouse, mostram o cursor de redimensionar e arrastam. Testes em
    `scroll_box.rs` e `crop_box.rs`.
  - A lista do ComboBox não fechava com o clique fora (pedido do autor em 07/11): só a perda
    de foco fechava, e clicar onde nada recebe foco (o fundo, um rótulo) não tira o foco.
    `Component::closes_on_outside_press`: a camada posta no form que pede isso é liberada
    quando o mouse aperta fora dela e de quem a abriu (`dies_with`), e o dono repinta; o
    clique segue para quem está embaixo. Vale para a lista do ComboBox e as sugestões do
    AutoComplete. Teste `combo_list_closes_on_a_press_outside` (falhava antes).
  - O projeto não depende de onde o Aylis está (pedidos do autor em 07/11: o `Cargo.toml` do
    NoOBS publicado tinha o caminho absoluto desta máquina; "não deveria depender estar lado
    a lado, quando o Aylis for instalado ele vai para AppData"; "o melhor é ter um registro que
    permita identificar a instalação do Aylis como qualquer outro app"). O instalador da IDE
    grava `HKCU\Software\Aylis` (`InstallDir`, `SdkDir`, `Version`; apaga ao desinstalar). O
    `Cargo.toml` de um projeto do usuário aponta para `<projeto>/.aylis/sdk` (relativo, igual
    em toda máquina) e o aylis mantém ali a junção até o aylis que está rodando (crate
    `junction`, só no Windows; link simbólico nos outros), com o `.aylis/.gitignore`. O
    `.cargo/config.toml` (a pasta de compilação, com o nome do usuário) ganha um
    `.cargo/.gitignore`. Dentro do repositório do aylis o caminho continua `../..`. O
    `scripts\build.bat` do NoOBS cria a junção pelo registro. Teste
    `a_user_project_points_to_its_sdk_link` (também confere que apagar o projeto não apaga o
    aylis). Falha à parte, de outra sessão: `the_landing_template_brings_the_form_the_theme_and_
    the_handlers` (o tema `"dark"` × `"Aylis Dark"`).
  - Para o frame repetido valer a pena: o índice de nomes (`aylis-core/src/names.rs`) era
    jogado fora a cada `add`/`set_name`/`set_parent` e remontado percorrendo a tela inteira, o
    que deixava montar N frames quadrático. Agora é um índice por escopo, atualizado no lugar
    (`set_name`, `set_parent`, `free`), e um componente que entra sem nome não mexe nele; o
    `free_name_in` continua do último número usado; a fábrica de forms guarda o JSON já lido
    dos forms do pacote embutido. Medido no NoOBS (500 cartões da biblioteca, release, refazer):
    360 ms → 17,5 → 13,8 → 10,3 ms com frames, contra 4,3 ms com o `MediaCard` solto (cerca de
    8 µs para criar e 8 µs para liberar uma instância de frame). `giant_screen` em release sem
    regressão (montar 44 ms, layout 2,9 ms).

- [ ] 38. CI com LLM, automação da IDE, rastro com métricas e cobertura (pedido do autor em
  07/11: "verificar integração contínua do futuro com LLM, pensar em algo que automatize a IDE
  e os projetos usando ela, code tracing com analytics, code coverage"). Estudo feito em 07/11:
  [docs/proposta-llm-ci.md](proposta-llm-ci.md), com o que já existe (CI com cobertura,
  54 ferramentas no MCP da IDE, Profiler, gravações, `Analytics`), as propostas (`aylis-tool
  lint` com as diretrizes checáveis, revisão e triagem por agente, a IDE sem janela na CI, o
  guia do agente e o `.mcp.json` nos projetos novos, ferramentas novas no MCP, rastro salvo e
  medido na CI, cobertura de handlers em qualquer linguagem e por linha no editor) e cinco
  decisões para o autor. Nada implementado: espera as decisões.
  Decisões do autor em 08/11 (roteiro 69): (1) ordem: 2b primeiro (as ferramentas que faltam no
  MCP, com os buracos achados na conferência), depois 1a → 2a → 4b → 3a/3b → 4a → 1d → 1b/1c → o
  resto; (2) o agente na CI só comenta; (3) o guia do projeto é opção do Novo projeto marcada por
  padrão, com `AGENTS.md`, um `CLAUDE.md` de uma linha apontando para ele e o `.mcp.json`; (4) na
  CI a cobertura reprova (cair mais de 1 ponto) e o tempo só avisa; (5) as métricas de desempenho
  no `Analytics` do app publicado só quando o app pede. Passar para a seção de decisões do
  `docs/proposta-llm-ci.md`.
- [ ] 44. Os exemplos só pelo acesso tipado (pedido do autor em 07/11: "todos os projetos de
  exemplo estão usando os componentes tipados? Sem localizar pelo nome"). Feito em 07/11 (sem
  commit; falta o autor confirmar):
  - Rust: GradeGigante, Galeria, Estresse3D, Relatorios, IBGE, Paint (as ferramentas como
    botões tipados), TesteAparelho (os sensores pelos getters tipados) e Landing (o
    `FrmDocumentacao::create(ui)` novo, gerado para todo form, no lugar do
    `create_form("frmDocumentacao")`);
  - ERP: o código comum das 1009 telas recebe as `Partes` de cada form (`self.cdsMestre.id`...)
    em vez de achar `cdsMestre`/`cdsItens`/`lblStatus` pelo nome; a lista acha o `cdsMestre`
    do cadastro que abriu pelas partes que ele deu ao abrir. Fica o `create_form(nome)` do
    menu: o form vem do nome que está na linha de dados (são 1009), como o
    `CreateForm(FindClass(...))` do Delphi;
  - IBGE nas outras 7 linguagens: os datasets pelo componente tipado (`frmIBGE.cdsEstados`,
    pelo id) no lugar do caminho em texto; Paint nas outras 7: as ferramentas e as cores como
    componentes tipados (o botão do evento pelo `SenderAsButton` gerado); Calculadora em
    Pascal (os botões sem erro); ProcessamentoAssincrono em Pascal (o `CreateForm(TFrmProgresso)`
    novo no cliente, pela classe, e o `modal_result` pelo id);
  - corrigido no caminho: o `Model3D` e o `DocViewer` não estavam no catálogo
    (`resources/components.json`), então não tinham acesso tipado em nenhuma linguagem; o C++
    ganhou `Component::id()`;
  - teste `examples_typed_access` do `aylis-project`: o código do usuário dos exemplos, nas 8
    linguagens, não tem `find_in`, `find_path`, `FindButton(...)` nem nome de componente entre
    aspas.
  Conferido em 08/11 (roteiro 67): o teste `apps` igual nas 8 linguagens (Calculadora, Paint,
  Player e IBGE) e a Calculadora à mão em Pascal, Python e Rust; não aprovado no ERP, que cai ao
  fechar a segunda de duas telas abertas pelo menu (pânico no winit, veja os ajustes de 08/11,
  grave). E o Relatórios ainda procura componente pelo nome (`relatorios.rs:33`, `find_path`).
- [ ] 47. A roda não rolava o ScrollBox do form no designer (relatado pelo autor em 07/11, na
  Landing: "a scroll interna quando o componente está no design não funciona, impedindo eu ver
  o que tem abaixo"). A roda só movia a vista do designer e nunca chegava ao form desenhado.
  Feito em 07/11 e confirmado pelo autor no mesmo dia (sem commit): sem Ctrl nem Shift, a roda vai primeiro
  aos controles do form sob o ponteiro (`Ui::wheel_controls`, novo no núcleo: a roda sobe pelos
  pais sem rolar o form), sem deslize (o `Ui` do form não anda de quadro em quadro no designer:
  salta direto), e o layout refeito põe o clique e a seleção no lugar novo; sem nada para rolar
  ali, move a vista como antes. A rolagem não vai para o JSON do form. Teste
  `the_wheel_scrolls_a_scroll_box_of_the_designed_form`.
- [ ] 49. No designer, a seleção aparecia fora da área visível depois de rolar, e o retângulo de
  seleção não começava dentro do form (relatados pelo autor em 07/11, na Landing). Feito em
  07/11 e confirmado pelo autor no mesmo dia (sem commit): o contorno e as alças só aparecem na parte
  visível dos contêineres que recortam (`Document::visible_area`), somem quando o componente
  sai da vista e a alça escondida não se pega; clicar num componente que não se move (alinhado:
  o ScrollBox e os layouts que cobrem o form) seleciona ele e o arrasto abre o retângulo, que
  seleciona o que ele toca, como no Delphi, a partir do contêiner onde começou: um contêiner
  coberto inteiro entra como um só; o que ele só corta dá lugar aos de dentro que ele toca (no
  NoOBS, um retângulo fino do campo de busca até os botões de uma barra da largura toda) e entra
  ele mesmo se não tocar nenhum; o miolo de um frame fica de fora. Testes
  `the_selection_area_starts_over_an_aligned_container_and_hides_when_scrolled_away` e
  `the_selection_area_picks_what_fits_inside_a_wider_bar`.
- [ ] 51. A tela inicial da IDE com a lista de projetos de ponta a ponta numa tela larga (pedido
  do autor em 07/11: "deveria estar centralizada a lista com largura máxima"). Feito em 07/11
  e confirmado pelo autor no mesmo dia (sem commit): o conteúdo numa coluna de no máximo 960, no meio da
  janela (`max_width`). A referência `principal-windows.png` regravada.
- [ ] 68. O MCP do Aylis ligado à sessão do Claude (pedidos do autor em 08/11: "caso o Aylis não
  esteja aberto o MCP deve abri-lo para a sessão do Claude que pediu, e o MCP deve relacionar a
  Sessão com a instância da IDE aberta"; "no Aylis deve mostrar o título da sessão do Claude que
  está conectada nele"; "o Claude deve conseguir pelo MCP pedir para compilar, e com isso a
  sessão tem que ficar aguardando o Aylis compilar e o Aylis deve avisar quando terminar"). Feito
  em 08/11 (sem commit; falta o autor confirmar):
  - a ponte (`aylis-tool mcp`) tem uma marca por sessão (por conexão: no Claude Code, uma por
    sessão; no Claude Desktop, pelo que se sabe, uma para o app inteiro); na primeira chamada de
    ferramenta sem IDE para ela, abre o `aylis-ide` ao lado do `aylis-tool` com o MCP ligado
    (`AYLIS_IDE_MCP=0`) e a marca (`AYLIS_IDE_MCP_SESSION`, que a IDE anota no registro das IDEs)
    e espera até 90 s; uma IDE aberta pelo usuário serve, a aberta por outra sessão não; a IDE da
    sessão fechada: a próxima chamada abre outra. O aperto de mão e a lista não abrem a IDE;
  - `session_title` (da ponte): o título que o agente dá à conversa; a ponte manda à IDE, com o
    nome do cliente (`aylis/session`), e a IDE mostra na barra da janela ("Aylis IDE — NoOBS ·
    Claude Desktop: título"); ao sair, tira. A instrução do MCP pede o `session_title` no começo;
  - `build` compila e espera terminar (até 30 min, numa thread da conexão: a IDE segue
    respondendo) e responde se compilou, com os erros e as últimas linhas; `wait: false` só
    começa. A ponte espera até 35 min e manda o andamento a cada 10 s (`notifications/progress`)
    ao cliente que pede.
  Conferido pela ponte: sem IDE, o `session_title` abriu a IDE com a marca e as chamadas
  seguintes foram para ela. Falta conferir no Claude Desktop e no Claude Code, o título na janela
  e o `build` esperando.
  Conferido em 08/11 (roteiros 92 e 93): no Claude Code, com a IDE fechada, o `session_title` abriu
  a IDE com o título na barra e as chamadas foram para ela; o `build` espera e diz se compilou, com
  os erros (aprovado). Não aprovado por inteiro: uma conversa no modo Claude do Claude Desktop pegou
  a mesma IDE em vez de abrir outra (veja o ajuste das duas conversas, na conferência de 08/11).
- [ ] 70. A auditoria das dependências barrava o instalador da IDE pelo RUSTSEC-2026-0436 no
  `calamine` (o leitor de planilhas do `aylis-docview`: um `.xls`/`.xlsb` malicioso causa pânico ou
  alocação sem limite; só negação de serviço), achado em 08/11 ao gerar o instalador para o
  plugin (item 69). Feito em 08/11 (sem commit; falta o autor confirmar): o `calamine` subiu de
  0.37 para 0.38.0 (`crates/aylis-docview/Cargo.toml` e `Cargo.lock`), sem mudança de API no que o
  `office.rs` usa. Conferido: `cargo test -p aylis-docview`, o wasm e o Android (`cargo ndk -t
  x86_64 -P 26`) passam, e o `aylis-tool audit` não acusa mais o `calamine`.
  Depois, a pedido do autor ("Sim, corrija"), os outros quatro avisos que ainda barravam:
  - `pulldown-cmark` 0.13.5 → 0.13.6 (RUSTSEC-2026-0433, análise quadrática de wikilinks de
    imagem aninhados; o `aylis-markdown`) e `async-executor` 1.14.0 (retirado do crates.io; vem do
    `accesskit_unix`) → 1.14.1, só no `Cargo.lock`;
  - o motor de texto do núcleo (`crates/aylis-core/src/textcore.rs`, os modos `layout` e `core`)
    trocou o `rustybuzz` 0.20 e o `ttf-parser` 0.25, sem manutenção (RUSTSEC-2026-0206 e -0192),
    pelo `harfrust` 0.15 (o porte do HarfBuzz que o próprio aviso indica), que traz o
    `read-fonts` 0.46: fonte, métricas, mapa de caracteres, eixo `wght` e contorno das letras.
    As métricas da linha seguem a mesma regra do `ttf-parser` (tipográfica se o OS/2 pede, senão
    `hhea`, tipográfica ou Windows); o plano de formação continua guardado por fonte, direção,
    escrita e idioma. De quebra, os bytes da fonte deixam de ser vazados (`Box::leak`): a fonte
    divide o `Arc` do registro. O `ttf-parser` continua no `Cargo.lock` por bibliotecas de
    terceiros, o que a regra (`unmaintained = "workspace"`) aceita.
  Conferido: `aylis-tool audit` passa (advisories, bans, licenses e sources ok); os testes do
  `aylis-core` e o `text_modes` (o peso da fonte variável, o mesmo desenho no Skia e no Direct2D)
  e o `partial_paint` passam, e a imagem do modo `core` saiu igual à do `native`; wasm e Android
  do `aylis-core`, `aylis-markdown` e `aylis-docview` compilam; `giant_screen` em release dentro da
  base (pintura inteira 799 ms; quadros de 1,8 a 21 ms). `cargo test --workspace` passou em tudo,
  menos no `aylis-tools` e em quem depende dele (`aylis-ide`, `aylis-host`, `aylis-demos`), que no
  momento não compilam pela mudança em andamento de outra sessão (o `Instance` sem o campo
  `session`, itens 67 e 68); antes disso, o `the_bridge_adds_and_answers_its_own_tools` falhava
  por não esperar a ferramenta `session_title`. Falta rodar esses quatro quando a outra sessão
  terminar, e gerar o instalador (`installer-ide`).
  Conferido em 08/11 (roteiro 96): o texto com o `harfrust` igual ao de antes (acentos, negrito,
  emoji, o código no editor) e `cargo test -p aylis-ide --test visual` 4/4. Os testes do
  `aylis-tools` (`mcp`, `apps` inteiro nas 8 linguagens) e o `--test mcp` do `aylis-ide` também
  passaram em 08/11; falta o resto do `aylis-tools`/`aylis-host`/`aylis-demos` e gerar o instalador.
- [ ] 71. Duas sessões do Claude no mesmo projeto (pergunta do autor em 08/11: "se tiver duas
  sessões abertas e o usuário pedir para fazer alteração no mesmo projeto, oque acontece?";
  escolha: "Na mesma IDE"). Antes, cada sessão ficava numa IDE, e as duas podiam abrir o mesmo
  projeto (uma gravava por cima da outra); numa IDE aberta pelo usuário, as duas caíam juntas e
  a barra mostrava só uma. Feito em 08/11 (sem commit; falta o autor confirmar):
  - o registro das IDEs guarda as sessões ligadas a cada uma (`Instance::sessions`; a IDE
    atualiza pelo `aylis/session`, que agora leva a marca da sessão): IDE com sessão está ocupada
    para as outras;
  - um projeto, uma IDE: o `open_project` de um projeto já aberto noutra IDE leva a sessão para
    lá (a que ela deixou fica livre), e a IDE não reabre um projeto que já está aberto; um
    projeto que não está aberto, pedido numa IDE dividida, vai para uma IDE livre na tela
    inicial ou uma nova, sem tirar o projeto da outra sessão;
  - a barra da janela mostra todas as sessões ligadas; quem sai sai só ele;
  - a marca da sessão leva o pid da ponte: uma ponte encerrada à força (sem avisar) é tirada
    da IDE na próxima escolha, e a IDE volta a ficar livre.
  As chamadas das sessões rodam uma de cada vez na IDE; se uma muda o trecho que a outra ia
  editar, o `edit_code` da segunda falha (o texto exato não bate) e ela relê.
  Conferido com duas e três pontes e IDEs de verdade: as duas sessões no mesmo projeto, as duas
  na barra, a saída de uma, a outra sessão indo para outro projeto (reaproveitando a IDE livre)
  e a marca de uma ponte morta limpa. Achado no caminho: a IDE caía com "Identificador de
  monitor inválido" (o `name`/`size` do monitor no `winit` 0.30 dão `unwrap` quando o monitor
  some), no refresh dos monitores a cada 2 s; agora o `aylis-platform-winit` confere o monitor
  antes (`monitors::valid`), e não caiu mais. Corrigido também o teste
  `the_bridge_adds_and_answers_its_own_tools`, que não esperava o `session_title`.
  Conferido em 08/11 (roteiro 95): não aprovado. A mudança de uma conversa (Claude Desktop)
  aparece para a outra (Claude Code) na mesma IDE, mas a conversa do Claude Desktop não aparece no
  título e, pedindo outro projeto, trocou o projeto da IDE da outra conversa (a IDE só conhecia
  uma sessão: a ponte do Claude Desktop não se anunciou; veja os ajustes de 08/11).
- [ ] 72. A Landing no celular: rolagem lenta, 3D que não aparece e ~2 MB por componente da
  documentação (relato do autor em 11/11: "no smartphone o 3D não mostra e também a scroll é
  bem lenta [...] cada componente que vejo na documentação ele baixa o componente
  aproximadamente 2mb cada"). Feito em 11/11 (sem commit; falta o autor confirmar no celular;
  roteiro 105):
  - **Rolagem:** o `ScrollBox` refazia o layout do form inteiro a cada passo (na web, ~300
    `measureText` por quadro, cada texto umas 4 vezes; 12,6 ms de mediana e 38 ms no p90 por
    quadro num PC, em tamanho de celular). Agora `Ui::scroll_content` só anda os filhos diretos
    e repinta a caixa (com layout pendente, faz o layout como antes); no nativo, 0,074 → 0,003
    ms por passo (`scrolling_on_the_phone_does_not_lay_out_the_page_again`, no teste da
    Landing; `scrolling_moves_the_children_without_a_new_layout` em `scroll_box.rs`). E o
    Canvas 2D guarda as larguras medidas por fonte e texto (zeradas quando uma fonte termina
    de carregar). Falta medir no navegador e no celular de verdade.
  - **3D:** no celular a vista "phone" esconde o `layLingote` de propósito, mas o `<iframe>`
    nascia no primeiro ciclo (o layout ainda no tamanho de projeto, 1200) e baixava os ~10 MB
    do 3D escondido. Agora a vista nativa só nasce com o layout em dia
    (`NativeViews::sync`); conferido no navegador em 375 × 812: nenhum pedido ao `demos/3d`.
    Decidido pelo autor em 11/11: o lingote continua escondido no celular.
  - **3D sem WebGPU** (pedido do autor em 11/11: "resolva"): o fallback WebGL do `aylis-3d`
    pedia o adaptador sem `compatible_surface`, e o wgpu não dá adaptador GL assim; num navegador
    sem WebGPU (iPhone antes do iOS 26, muitos Android) o 3D nunca aparecia, e calado. Agora
    `Engine::new_web` tenta o WebGPU e, sem ele (ou sem adaptador), cria o canvas, a superfície
    WebGL 2 e pede o adaptador por ela; a primeira vista usa esse canvas (no WebGL, uma vista 3D
    por página: o dispositivo é do contexto daquele canvas). Corrigido no caminho: o passe usava
    o formato BGRA fixo, que só coincidia com o WebGPU (o canvas WebGL é RGBA); e a falha ao
    abrir a GPU vai para o console do navegador. Conferido no Edge sem janela (SwiftShader),
    com o `navigator.gpu` escondido: o lingote sai igual ao do WebGPU. Falta num iPhone de
    verdade (testes-em-dispositivos).
  - **Tamanho dos wasm:** a publicação web (página, IDE lite e demos) compilava em release
    puro, com 1,4 MB de nomes de funções em cada wasm. Agora `WEB_RELEASE` no `publish.rs`
    (LTO, uma unidade, `opt-level = "s"`, `strip`), pelas variáveis do cargo: Standard 6,6 →
    3,7 MB (1,91 → 1,35 MB comprimido), Landing 9,0 → 6,9 MB sem o strip. Números em
    `medicoes/web_wasm.json`. A compilação de cada wasm fica mais lenta (~3 min a Standard).
  - **O núcleo uma vez só** (decidido pelo autor em 11/11: "A + os cortes 1 e 3"): as famílias
    leves da documentação ao vivo (Standard, Layouts, Menus, Shapes, Animations, Electrical,
    Industrial, Trader, BI, Device, System, Quasar: 136 classes) vêm no wasm da própria página
    (`aylis_build::demo_classes` no `build.rs` da Landing; a montagem do demo foi para
    `aylis::samples::demo`, que o `aylis-demos` também usa); a página de um desses componentes
    não baixa nada a mais (antes, 1,2 a 1,9 MB comprimido). As pesadas (Additional, 3D, mídia,
    dados, relatórios, provedores) continuam no `<iframe>` da família. A Landing ficou com
    6,2 MB (2,64 MB comprimido). Teste `a_component_page_shows_the_live_component_inside_the_page`.
  - **Corte 3:** o `harfrust` (~520 KB de código, 28% do núcleo mínimo) virou a feature
    `text-shaping` do `aylis-core`: a fachada liga por padrão, a IDE, o host e a API C também;
    o gerador põe no `Cargo.toml` do projeto só quando há fontes em `assets/fonts`
    (`GENERATOR_VERSION` 9). Sem fonte do projeto o texto já era do renderizador; nada muda na
    tela. Additional 13,4 → 7,6 MB (4,48 → 3,14 MB comprimido); 3D 9,9 → 4,3 MB (2,93 → 1,65).
  - **Corte 1 (`wasm-opt`) descartado depois de medido:** depois do LTO com `opt-level="s"` e
    `strip`, o `wasm-opt` tira 4 a 6% do wasm cru e o comprimido não cai (fica até 1% maior)
    em nenhum nível; o passo e o Binaryen saíram (números em `medicoes/web_wasm.json`).


**Conferência de 08/11/2026 (Sessões 3 a 7 do roteiro): ajustes achados**

Achados conferindo os itens 29 a 96 com o autor (o Claude pelo MCP da IDE, os projetos de teste no
scratchpad da sessão). "(autor)" é pedido ou relato dele; o resto foi achado na conferência. O que já
tinha bloco acima ganhou lá a nota "Conferido em 08/11".

*Graves*
- [ ] A IDE caiu no driver de vídeo (roteiro 96, autor): APPCRASH 08/11 16:34:27, módulo
  `atio6axx.dll` (OpenGL da AMD 32.0.32015.7029), 0xc0000005 (dump no WER). A IDE desenha com o Skia
  GPU (OpenGL); caiu depois de fechar o Profiler e voltar à IDE (a janela tinha ficado minimizada e
  janelas dela tinham sido movidas por fora). Ver o contexto GL com a janela minimizada/restaurada; a
  IDE no Windows com o Direct2D (o padrão dos apps) ou o Skia por D3D, e cair para a CPU quando o
  driver falha, sem derrubar a IDE.
  Feito em 08/11 (falta o autor confirmar): a IDE no Windows desenha com o Direct2D (o padrão dos
  apps), sem o OpenGL que caiu no driver da AMD; `AYLIS_RENDERER` ainda escolhe outro
  (`aylis-ide/src/main.rs`, feature `d2d` no `Cargo.toml` da IDE). Falta conferir a IDE aberta no
  Direct2D.
- [ ] A geração de código grava por trás do editor aberto (roteiros 41 e 70): o "Gerando o código"
  do Executar (Python) e o `generate_code` (Rust) acrescentam handlers no disco, e o editor da IDE
  continua com o texto antigo (`unsaved` falso); salvar o editor apaga o que a geração pôs. Com o
  arquivo aberto, a geração tem de passar pelo buffer do editor.
  Feito em 08/11 (falta o autor confirmar): a geração (Executar, Gerar código, F12 e o
  `generate_code` do MCP) guarda o texto dos arquivos abertos antes e, depois, o editor acompanha o
  que ela pôs, como uma edição dele (o Ctrl+Z desfaz): igual ao disco, continua salvo; com alteração
  não salva, o trecho novo entra no mesmo lugar do texto do editor, e se não der para juntar a
  mensagem avisa (`codewindow::follow_generation`, `Msg::Generated`); teste
  `the_editor_follows_what_the_generation_added`.
- [ ] Diálogos da IDE abrindo fora da tela (roteiros 59, 80 e 87, autor): o "Confiar neste
  projeto?", as Opções do projeto e o Profiler abriram em ≈ -21000,-21000; a IDE fica bloqueada pelo
  modal invisível e o Windows toca o som de janela ocupada. Causa: a janela principal estava
  minimizada (em -21333,-21333, 158×26) e o diálogo se posiciona sobre o retângulo dela. Usar o
  retângulo restaurado (`GetWindowPlacement` → `rcNormalPosition`) ou restaurar a dona; o diálogo
  abre centralizado sobre a IDE, no monitor dela, na frente e com o foco. E o `build` do MCP num
  projeto restrito só responde o erro, sem abrir o modal.
  Feito em 08/11 (falta o autor confirmar): com a dona minimizada, o diálogo se posiciona pelo
  retângulo normal dela (`GetWindowPlacement`, `win32::normal_rect`), e um modal restaura a janela
  principal antes de abrir. O `build`, o `run_without_debug`, o `run` e o `publish` do MCP num
  projeto restrito respondem um erro que diz para pedir ao usuário que confie (`mcp::restricted`).
- [ ] A IDE sumiu sem registro (roteiro 40): logo depois de "Confiar" na Calculadora, o MCP mandou
  `run_without_debug` e, com a compilação rodando, `build`; a gravação
  (`Aylis-IDE_2026-11-08_14-40-10`, t≈3422) acaba sem `form_closed` e o Windows não registrou queda.
  Sem o `build` no meio, não caiu. O `build` com outra compilação ou execução em andamento devia
  esperar ou recusar.
  Feito em 08/11 (falta o autor confirmar): não reproduzido. Achado no caminho: com o app
  executando, o `build` do MCP achava que a compilação tinha começado (havia um trabalho) e esperava
  o app terminar; agora recusa com "já há um trabalho em andamento", e o Compilar (Ctrl+F9) não
  salva nem começa com outro trabalho rodando.
  Depois, também em 08/11: pista forte para o sumiço: a IDE aberta pela ponte fica no job do cliente
  quando ele recusa o `CREATE_BREAKAWAY_FROM_JOB`, e um job que encerra os processos ao fechar mata
  a IDE sem erro nem registro quando o cliente reinicia a ponte (um `build` longo pode fazer o
  cliente desistir). Agora a ponte avisa no log do MCP, e a IDE grava na gravação de uso
  (`ide_no_job_do_cliente`) e avisa na situação (`dap::in_kill_on_close_job`). Se a próxima queda
  tiver essa marca, a saída é a IDE nascer fora do job.
- [ ] Duas conversas na mesma IDE (roteiro 95): a ponte do Claude Desktop não se anuncia à IDE
  (`aylis/session`), então ela não aparece no título e, pedindo outro projeto, trocou o projeto da
  IDE em uso pela outra conversa. Havia 4 pontes `aylis-tool mcp` vivas, duas de horas antes. E o
  título mostra "local-agent-mode-aylis", o nome interno do cliente, que não diz nada ao usuário.
  Repetido em 08/11 depois de reiniciar o computador (roteiro 92): com a IDE desta conversa aberta
  (pid 7836, sessão `15392-…`), uma conversa no modo Claude do Claude Desktop pediu o Aylis e usou
  essa mesma IDE (respondeu pelo `ide_list`/`project_info`), sem abrir outra e sem aparecer no
  registro (`sessions` só com a `15392`; a ponte dela é a `22404`). Não houve `ide_select`: a
  primeira chamada dela foi o `ide_list`, que já respondeu `* (sem projeto)` (a IDE fixada nela).
  Pista: o `target_port` fixa sozinho uma IDE "livre" (sem sessão no registro); a IDE é aberta
  com a marca de quem a abriu, mas há uma janela entre ela aparecer no registro e o anúncio da
  sessão chegar, e qualquer ponte que olhe nesse meio a toma; depois a ponte que tomou não se
  anuncia. Fechar a janela (a IDE já nasce com a sessão de quem a abriu no registro) e conferir
  por que o `announce` da segunda ponte não grava a sessão dela.
  Feito em 08/11 (falta o autor confirmar): a ponte só escolhe a IDE numa chamada de ferramenta (o
  aperto de mão, a lista e os avisos ela responde sozinha; antes fixava uma IDE "livre" já ali);
  pega a IDE livre anunciando-se e conferindo no registro que chegou primeiro (duas ao mesmo tempo:
  a segunda abre a sua); a cada chamada confere se a sessão está no registro da IDE e anuncia de
  novo se não estiver; o `process_alive` passou a contar o acesso negado como vivo (uma ponte noutro
  nível de integridade ou sandbox era dada como morta, e outra ponte tirava a sessão dela do
  registro); o cliente "local-agent-mode-aylis" aparece como "Claude Desktop"; a resposta da chamada
  que abriu a IDE diz "Abri a IDE do Aylis para esta conversa", e o `ide_list` sem IDE diz que a
  próxima chamada abre uma. Testes em `aylis-tools` (`bridge_tests`). Falta conferir com o Claude
  Desktop (roteiro 92 e 95).

*IDE: projeto, designer e editor*
- [ ] F12 de volta ao Código volta onde estava (roteiro 59, autor): hoje vai ao topo do arquivo;
  manter o cursor, a seleção e a rolagem, como no Delphi.
  Feito em 08/11 (falta o autor confirmar): o F12 sem handler volta à aba com o cursor, a seleção e
  a rolagem de antes (só a primeira vez vai ao topo).
- [ ] Ctrl+Z depois do `edit_code` (roteiro 72, autor): desfaz a mudança certa, mas o cursor pula
  para uma edição antiga; depois do desfazer, o cursor e a rolagem vão ao trecho desfeito.
  Feito em 08/11 (falta o autor confirmar): o desfazer e o refazer põem o cursor no trecho que mudou
  quando o cursor guardado está fora dele (uma troca feita de fora, como o `edit_code`); a rolagem
  acompanha (`Memo::restore`); teste `undo_puts_the_caret_on_what_was_undone`.
- [ ] Marcadores numerados (roteiro 77, autor): uma linha tem um marcador só, o último atribuído
  (Ctrl+Shift+1 e depois Ctrl+Shift+2 deixam hoje os dois na linha).
  Feito em 08/11 (falta o autor confirmar): a linha fica com um marcador só, o último número dado a
  ela; teste em `memo_marks`.
- [ ] A dica do código (roteiro 56, autor): sai como texto cru (sem cor, Markdown à mostra, a
  assinatura em fonte proporcional, cortada no meio). Mostrar como o VS Code: o código em
  monoespaçada com as cores do editor, os separadores, a documentação formatada e rolagem.
  Feito em 08/11 (falta o autor confirmar): a dica é uma caixa por cima do editor com o `Markdown`
  (`hovertip.rs`): os erros do ponto, o caminho e a assinatura, a documentação, com linhas entre as
  partes; os blocos de código na monoespaçada e com o realce e as cores do editor, quebrando a linha
  longa; até 600 × 340, depois rola; fica aberta com o mouse em cima (seleciona e copia) e fecha
  ~350 ms depois de sair, numa tecla, clique, roda ou digitação. Falta conferir na IDE.
  Depois, também em 08/11: a cara do editor nos blocos de código do `Markdown` virou propriedade
  (`editor_look`, `code_theme`, `code_language`, no catálogo e com os rótulos traduzidos); teste
  `the_editor_look_comes_from_the_properties`.
- [ ] A dica do rodapé do servidor de linguagem não traz o detalhe (roteiro 58).
  Feito em 08/11 (falta o autor confirmar): o rótulo do servidor recebe o mouse (o `Label` não
  recebia) e a dica diz o servidor e a versão, o estado (iniciando, preparando, pronto, erro com a
  mensagem), o projeto, a pasta, o programa e o que ele faz (pelo que ele informa); sem servidor ou
  no modo restrito, diz por quê. Traduzida.
- [ ] Dicas nos botões de texto da barra da IDE (roteiro 57, autor): Novo projeto, Abrir…, Salvar,
  Salvar tudo, Desfazer, Refazer, + Form, Código (F12), Publicar…, com o atalho ("Salvar (Ctrl+S)").
  Feito em 08/11 (falta o autor confirmar): as dicas com o atalho (Novo projeto, Abrir, Salvar,
  Salvar tudo, Desfazer, Refazer, + Form, Código, Publicar), traduzidas.
- [ ] Abas dos forms com muitas abertas (roteiro 71, autor): (a) com a aba encolhida, o X de fechar
  não aparece mas a área dele vale, e o clique na borda direita fecha a aba; a área do X só vale com
  o X desenhado; (b) todas viram "fr…": o nome sempre inteiro, a mesma largura, e a roda rola a barra
  na horizontal. Medido: ~0,49 MB por aba (o roteiro esperava ~0,3).
  Feito em 08/11 (falta o autor confirmar): (a) a área do × só vale onde ele é desenhado; (b) todas
  as abas têm a mesma largura, a que cabe o nome mais comprido (até o `max_tab_width`), e passando
  da faixa a roda rola as abas na horizontal; a aba ativa nova vem para a vista. A largura é medida
  só quando os nomes mudam. Os ~0,49 MB por aba não foram mexidos.
- [ ] Painel ESTRUTURA (roteiro 60, autor): recolher e abrir os filhos de cada componente (a seta,
  →/← no teclado), lembrando por form.
  Feito em 08/11 (falta o autor confirmar): a seta (o começo da linha) recolhe e abre os filhos; o
  form lembra o que está recolhido; o escolhido dentro de um recolhido abre os de cima. Falta o ←/→
  do teclado.
  Depois, também em 08/11: com o foco no painel (clicar numa linha), ↑/↓ andam, → abre (aberto, vai
  ao primeiro filho) e ← recolhe (recolhido ou sem filhos, vai ao pai); teste
  `the_structure_panel_follows_the_arrows`.
- [ ] Object Inspector (roteiro 74, autor): a busca de propriedades abre os grupos recolhidos
  enquanto há texto (hoje o achado fica escondido).
  Feito em 08/11 (falta o autor confirmar): o código já abre os grupos enquanto há busca
  (`searching`); conferir de novo na IDE recompilada.
- [ ] Designer pelo teclado como no Delphi (autor): Shift+seta muda o tamanho (1 px), Ctrl+seta move
  (1 px), com vários selecionados.
  Feito em 08/11 (falta o autor confirmar): seta e Ctrl+seta movem 1 px, Ctrl+Shift+seta move 8 px,
  Shift+seta muda o tamanho 1 px, Alt+↑/↓ muda a ordem entre os irmãos (era o Ctrl+↑/↓); vale para
  todos os escolhidos (`Document::resize_by`); teste `designer_keys`.
- [ ] "Pesquisar no projeto" / prévia do renomear (roteiro 60, autor): clicar ou dar duplo clique
  num resultado não abre o lugar (mais de 30 cliques na gravação sem nenhum evento).
  Feito em 08/11 (falta o autor confirmar): o TreeView ganhou `on_click` (também no nó já escolhido,
  que não dava `on_change`) e `on_dbl_click`; a pesquisa abre o lugar no clique, e o duplo clique
  traz a IDE para a frente.
- [ ] Opções do projeto → Traduções (roteiro 82, autor): a lista dos idiomas do projeto (com quanto
  falta em cada), escolher, adicionar e remover, no lugar do campo de texto; e tirar da lista o que
  não se traduz ("-", ".00", "0", "0,25×", "00:00", "0.0.0", "+").
  Feito em 08/11 (falta o autor confirmar): a lista dos idiomas do projeto com quanto falta em cada
  (novo, completo, N sem tradução), Adicionar e Remover (com confirmação; apaga o
  `locales/<idioma>.json`); os textos sem nenhuma letra saem da lista (`i18n::translatable`).
- [ ] Opções do projeto → Fontes (roteiro 49, autor): o nome da família alinhado à esquerda; sai o
  "em uso · N" e aparece só a falta ("usada, mas não embutida"), decidido pelo autor.
  Feito em 08/11 (falta o autor confirmar): o nome da família à esquerda (um rótulo clicável no
  lugar do botão) e só a falta: "usada, mas não embutida", que some ao marcar a família.
- [ ] A IDE em English (roteiro 84): ficaram em português "Executar sem depurar", "Compilar e
  executar em release", "Compilar", "Compilar em release", "Mostrar componentes escondidos", "Abrir em
  outra janela", "Ir para o Design", a situação "Projeto X aberto (Rust)", e nas Opções o título,
  "Navegador (web)", a descrição do renderizador, "(troque em Opções do projeto → Aparência)", "TEXTO
  POR PLATAFORMA" e a nota, "Nome do executável"; o "Language/English" não fica marcado. Ver por que o
  teste `the_new_texts_and_the_messages_with_values_are_translated` não pegou.
  Feito em 08/11 (falta o autor confirmar): os textos do roteiro traduzidos nas 11 línguas (55
  linhas pelo `translations --apply`), o texto do renderizador traduzido por partes, o "(troque
  em…)" pela tradução, e o idioma escolhido fica marcado no menu Exibir → Idioma. O teste só
  conferia dois textos fixos; agora confere também os do roteiro. A lista do `aylis-tool
  translations` ainda tem ~390 textos (a maioria não é de tela: dados de exemplo, nomes de fonte,
  mensagens para o agente, textos de teste), acumulados de outras sessões.
- [ ] Profiler (roteiros 54 e 87, autor): a roda invertida (o zoom da linha do tempo e a lista de
  Métodos); mostrar a área repintada de cada quadro (hoje só o tempo, então o "a pintura do mouse no
  fundo quase some" não dá para conferir); e conferir se mover o mouse no fundo, sem trocar de botão,
  gera quadro (918 quadros em ~15 s de mouse no TestePintura).
  Feito em 08/11 (falta o autor confirmar): a roda corrigida nos dois lugares (para cima aproxima; a
  lista de Métodos sobe). Conferido no núcleo, com rastro: mover o mouse no fundo de um form não
  marca nada para repintar (só ao entrar no form); os quadros do relato vêm de outro lugar (falta
  achar com o app). A área repintada de cada quadro ainda não aparece no Profiler.
  Depois, também em 08/11: cada quadro aparece na faixa da GPU como "quadro · N% repintado" (a
  tabela de Métodos junta todos em "quadro"). Pista para os quadros do mouse: um quadro pedido sem
  área marcada vira a janela inteira (`take_frame` dá `Whole` sem nada marcado), então um
  `request_repaint` à toa custa a tela toda; conferir com o app aberto o que pede esses quadros.
- [ ] MENSAGENS: vermelho só para erro (roteiros 42 e 53, autor): o clique cancelado pelo
  interceptador (uma linha vermelha por clique) e o "frmX recarregado" saem em vermelho.
  Feito em 08/11 (falta o autor confirmar): as notas do próprio Aylis no erro padrão do app (`aylis:
  …`: o form recarregado, o clique cancelado pelo interceptador, o evento adiado) saem apagadas;
  vermelho só o que diz que falhou (`jobs::aylis_note_kind`, teste).
- [ ] Linha de situação depois de trocar a linguagem do projeto diz a antiga ("aberto (Free
  Pascal)" com o projeto em Rust) (roteiro 40).
  Feito em 08/11 (falta o autor confirmar): o `set_project_setting` do MCP que troca a linguagem
  gera o código dela e põe a situação com a nova (pelas Opções já era assim); e aceita o valor como
  texto ou como JSON.
- [ ] "aylis" minúsculo para pessoas (roteiros 40, 51): "Gerado pelo aylis (somente leitura)" na
  árvore, "versão anterior do aylis" e o `MIGRATE_HINT`.
  Feito em 08/11 (falta o autor confirmar): "Gerado pelo Aylis (somente leitura)", "versão anterior
  do Aylis", o `MIGRATE_HINT` e a mensagem do formato no `aylis-json` (e as traduções).
- [ ] Os forms com "criar no início" abrem todos visíveis ao executar (roteiro 70): como o Delphi,
  criar sem mostrar (só o principal aparece).
  Feito em 08/11 (falta o autor confirmar): não abriam: o `ui_forms` do app dizia `visible: true`
  pela propriedade de forms criados e não mostrados. Agora diz `open` e `visible` só para o aberto.

*Execução e projeto (pedidos do autor, roteiro 88)*
- [ ] Propriedade do projeto para encerrar o processo do app se ele já estiver aberto, antes de
  executar (o NoOBS da IDE terminava na hora com o instalado aberto).
  Feito em 08/11 (falta o autor confirmar): `output.close_running` (Opções do projeto → Geral,
  "Encerrar o app aberto antes de executar"): com o app já aberto (o instalado também), encerra
  antes de compilar; sem ela, o Executar e o Compilar param logo com a mensagem
  (`run::close_or_refuse_running`).
- [ ] Antes de compilar, conferir se o `.exe` de saída pode ser sobrescrito, e avisar logo (ou
  encerrar pela propriedade acima).
  Feito em 08/11 (falta o autor confirmar): o app aberto com o mesmo nome de executável para a
  compilação antes de começar, com a mensagem (ou é encerrado, com a propriedade acima).
- [ ] Comandos antes e depois da compilação (cmd e PowerShell), como os Build Events do Delphi. Decidido
  pelo autor: só em projeto confiado, e a janela "Confiar neste projeto?" mostra esses comandos. É
  exceção à diretriz "Texto não vira código nem comando" (ajustar o `CLAUDE.md` e o histórico).
  Feito em 08/11 (falta o autor confirmar): `output.before_build`/`after_build` (cmd ou PowerShell;
  nas Opções, um por linha, `ps: ` na frente para o PowerShell). Só rodam em projeto confiado; o
  texto vai para um `.bat`/`.ps1` em `.aylis/` (sem `cmd /C` com aspas), na pasta do projeto, com
  `AYLIS_PROJECT_DIR` e `AYLIS_OUTPUT_EXE`; um que falha para a compilação. O "Confiar neste projeto?"
  lista os comandos. A diretriz do `CLAUDE.md` ganhou a exceção.
- [ ] O `.aylisproj` muda a cada salvar de form (roteiro 61): o `"modified"` do form é regravado, e duas
  pessoas mexendo em forms diferentes conflitam no `.aylisproj`; tirar essa marca do arquivo do projeto
  (para fora do controle de versão) e citar no `docs/forms-em-equipe.md`.
  Feito em 08/11 (falta o autor confirmar): a hora de cada form saiu do `.aylisproj`
  (`skip_serializing`) e foi para `.aylis/form-stamps.json`, com a regra no `.aylis/.gitignore`; um
  `.aylisproj` de antes ainda é lido. Citado no `docs/forms-em-equipe.md`; teste em `project.rs`.
- [ ] O modelo/cópia da Landing não compila fora do repositório (roteiros 36 e 38): o bloco à mão do
  `features = ["3d"]` no `Cargo.toml` não é gerado nem reescrito pelo `publish` ("Dependency 'aylis'
  has different source paths"); as features do projeto saem do gerador.
  Feito em 08/11 (falta o autor confirmar): a geração põe toda dependência do aylis no `Cargo.toml`
  (também o bloco do 3D escrito à mão) na mesma pasta (`sync_aylis_paths`); teste. O modelo "Landing
  page" saiu (decidido pelo autor: Desktop, Smartphone e Web).
- [ ] Rust: os handlers dão aviso `non_snake_case` (roteiro 42): o `impl` do form no código do
  usuário com `#[allow(non_snake_case)]`.
  Feito em 08/11 (falta o autor confirmar): o `impl` do form ganha `#[allow(non_snake_case)]` no
  arquivo novo e num de antes que recebe um handler com maiúscula; teste.
- [ ] Mudar o valor de um controle pelo código dispara o `on_change`, como na VCL (roteiro 53;
  decidido pelo autor): `text`, `checked`, `value`, `item_index`..., também pelo protocolo; conferir
  os controles e exemplos que mudam o próprio valor no `on_change`.
  Feito em 08/11 (falta o autor confirmar): central no núcleo: o `set_prop` de uma propriedade de
  valor (`text`, `checked`, `value`, `item_index`, `date`... na categoria Data ou Runtime) dispara o
  `on_change` com o mesmo argumento do usuário (`Component::change_args`), só quando o valor mudou e
  alguém escuta; não dispara no designer, ao carregar o form, no hot reload, no `create` do
  protocolo, nas variantes, traduções e vistas (`set_prop_quiet`, `without_change_events`). Vale pelo
  acesso tipado e pelo `set` do protocolo; o registro que chega num controle DB dispara sem gravar de
  volta. Laço: o limite de eventos aninhados para com erro, sem travar. Três diálogos da IDE (ângulo,
  cor, sombra) passaram a gravar sem disparar. `giant_screen` igual à base. Testes em
  `change_by_code.rs`, `change_events.rs` e `bindings.rs`. Diferença da VCL: lá o `ItemIndex` da
  ComboBox pelo código não dispara; aqui dispara (a lista do autor inclui o `item_index`).

*Componentes*
- [ ] Botão com Enter seguro repete o clique, como na VCL (roteiro 42, autor), com uma propriedade
  para ligar ou desligar; o Espaço clica ao soltar.
  Feito em 08/11 (falta o autor confirmar): `repeat_enter` (ligada por padrão, rótulo traduzido): o
  Enter clica ao apertar e repete seguro; o Espaço afunda ao apertar e clica ao soltar (sair do foco
  com ele apertado não clica). Testes.
- [ ] O anel de foco do botão primário quase invisível no Aylis Light (roteiro 54, autor): contrastar
  com o próprio controle (uma folga da cor do fundo entre o botão e o anel), em todos os temas e
  variantes.
  Feito em 08/11 (falta o autor confirmar): com o anel, o fundo do botão encolhe e sobra uma folga
  com o fundo de trás entre o botão e o anel, em todos os temas e variantes; teste
  `the_button_focus_ring_has_a_gap_in_every_theme`.
- [ ] Memo no fim do texto (roteiro 54, autor): continuar girando a roda desloca a imagem sem a
  posição mudar e deixa cópias cortadas da última linha e o "L" virando traço; igual no Direct2D, no
  Skia CPU e no GPU, não na web (é a rolagem por cópia do núcleo).
  Feito em 08/11 (falta o autor confirmar): a roda calculava o fim com as linhas inteiras e a
  pintura com a altura real: passava do fim, pedia a cópia e a pintura voltava. O Memo guarda a
  altura da área de texto, o fim é arredondado para baixo, e o núcleo não copia uma rolagem que não
  andou. Teste `the_wheel_past_the_end_of_a_memo_copies_nothing` (falhava antes: 14.483 pixels
  diferentes).
- [ ] Duas janelas do mesmo form com o mesmo nome e caminho no protocolo (roteiro 42): a segunda ganha
  nome livre (`frmPrincipal2`), como o `create_in`.
  Feito em 08/11 (falta o autor confirmar): o `create_form_instance` dá à segunda o nome livre
  (`frmPrincipal2`); os handlers continuam achando a instância (com ou sem instância registrada), e
  no protocolo o `form` do evento continua o dos handlers, com o `sender.path` dizendo a janela.
  Testes `second_window.rs` e `change_events.rs`.
- [ ] `Barcode`: o nível de correção do QR (roteiro 63, autor; hoje sempre M), passando pelo
  provedor.
  Feito em 08/11 (falta o autor confirmar): a propriedade `error_correction` (low, medium, quartile,
  high; padrão medium) chega ao provedor (`BarcodeOptions`); rótulo traduzido; testes.
- [ ] `MapView`: o `tile_url` e a `attribution` dele confundem com um provedor ligado (roteiro 66);
  o servidor vem só do provedor.
  Feito em 08/11 (falta o autor confirmar): saíram o `tile_url` e a `attribution` do `MapView`: o
  servidor e o crédito vêm do provedor ligado (sem provedor, o OpenStreetMap, `TileServer`). Nenhum
  form do repositório usava; um form de fora com essas propriedades passa a dar erro ao abrir (não
  subiu a versão do formato).
  Decidido pelo autor em 08/11: fica assim (sem subir a versão do formato nem conversão no
  aylis-migrate); um form de fora com `tile_url` ou `attribution` no MapView dá erro ao abrir.
- [ ] Os métodos no acesso tipado (roteiro 64): o parâmetro sai `a1` (`Export::save_to(ui, a1)`) e a
  falha só como `false`; nome nos parâmetros e o erro devolvido.
  Feito em 08/11 (falta o autor confirmar): os parâmetros têm nome (`MethodInfo::params`, no
  catálogo `save_to(path:string)`) nas 8 linguagens (em Pascal com `A` na frente) e, em Rust, os
  métodos devolvem `Result` (`handles::try_call`); `GENERATOR_VERSION` 7; os 12 exemplos do
  workspace regerados e compilando.
- [ ] `report::write_pdf` ainda sobe a memória com as páginas (roteiro 65): 3.000 registros +1,0 MB,
  30.000 +1,9 MB (pico +4,8), 300.000 +18,3 MB (pico +47,3, o PDF tem 33 MB); o teste não mede
  memória.
  Feito em 08/11 (falta o autor confirmar): agora fica plana: 3.000 registros +0,00 MB (pico +0,06),
  30.000 +0,00 (pico +0,08), 300.000 +0,00 (pico +0,24; antes +16,0 e +20,2). As causas: as marcas
  de repintura dos rótulos acumulando sem quadro (`push_recent`), os totais juntando os valores numa
  lista e o fecho do PDF montado num texto só. O teste mede a memória (3.000 e 30.000; o de 300.000
  é `#[ignore]`).
- [ ] GradeGigante (roteiro 52, autor): a memória que sobra depois do Parar (103 → 100 → 523 → 581
  MB nos três primeiros); o rótulo fica com um "Montando…" velho depois do Parar; o `terminou()` apaga
  o Parar de uma carga nova. O comportamento de mostrar tudo no fim está certo (decidido pelo autor);
  corrigir o texto do roteiro, que dizia "aos pacotes".
  Feito em 08/11 (falta o autor confirmar): o Parar não valia depois da montagem, e o lote montado
  entrava no dataset assim mesmo (a memória que sobrava); agora o lote é conferido antes de entrar.
  Cada carga tem um número: o andamento atrasado não escreve na tela, e o `terminou` só mexe nos
  botões da carga atual; o Parar mostra "Parando…". Medido: 1 milhão de registros, 298 MB carregado
  e 4 MB depois de esvaziar. Falta conferir na janela.

*MCP e agentes*
- [ ] A ponte sem a IDE (feito nesta conferência, sem commit; conferir no roteiro 92): a lista das
  ferramentas da IDE passou para `aylis_tools::mcp::ide_tools()`, e a ponte responde a lista completa
  com a IDE fechada (o Claude Code não pede a lista de novo pelo `tools/list_changed`, que a ponte
  também passou a mandar depois do `session_title`); teste `the_list_without_the_ide_has_the_ide_tools`.
- [ ] Buracos do MCP da IDE (roteiro 69, o item 2b): o `dialog_state` sem os ComboBox e os botões, e
  nenhum `dialog_select` (o agente não escolhe o Modelo do Novo projeto); o `ide_menus` com ~64 "Ligar a
  campo…" repetidos e itens de contexto sem caminho; o `set` do `app_command` recusa o formato que a
  descrição da ferramenta mostra (`prop`/`value`, pede `props`); o `set_project_setting` recusa o
  texto com aspas; a Galeria não fecha pelo `dialog_key`.
  Feito em 08/11 (falta o autor confirmar): o `dialog_state` traz as listas (`combos`, com rótulo,
  itens e o escolhido), os botões e só o que está à vista; ferramenta nova `dialog_select` (pelo
  texto ou pelo índice); o `ide_menus` mostra cada item de menu de contexto uma vez, com o lugar
  ("Object Inspector › (propriedade) › Ligar a campo…"); o `set` do `app_command` aceita
  `prop`/`value` e `props`; o Esc fecha a Galeria (e os Componentes não visuais e as Gravações); o
  `set_project_setting` aceita o valor como texto ou JSON. Teste
  `an_agent_fills_the_dialogs_and_reads_the_menus`. O `menu` ainda acha um item de contexto só pelo
  texto (clica no primeiro).
- [ ] O guarda de edição deixa passar um pedido que não é JSON válido (roteiro 72): registrar ou
  recusar com mensagem.
  Feito em 08/11 (falta o autor confirmar): um pedido que não é JSON é recusado com mensagem clara e
  registrado em `%LOCALAPPDATA%\aylis\edit-guard.log` (só o tamanho e o erro); teste.
- [ ] A chamada que abre a IDE não diz que abriu (roteiro 91): no Claude Desktop, o `ide_list`
  respondeu "Nenhuma IDE do Aylis aberta com o MCP ligado", o `project_info` em seguida abriu a IDE
  e respondeu sem avisar, e o Claude concluiu "não encontrei nenhuma IDE aberta". Quando a ponte
  abre a IDE, a resposta diz isso ("abri a IDE do Aylis para esta conversa"); e o `ide_list` sem
  IDE diz que a próxima chamada abre uma.
  Feito em 08/11 (falta o autor confirmar): veja "Duas conversas na mesma IDE" acima.
- [ ] O `claude` não está no PATH (roteiro 70): a linha do `client_command` não roda num terminal
  comum; dizer isso ou configurar o Claude Code por um botão.
  Feito em 08/11 (falta o autor confirmar): `claude_executable()` acha o Claude Code no PATH, em
  `~/.local/bin`, no npm e no que vem com o Claude Desktop; a linha do comando usa o caminho
  completo, e o botão "Configurar o Claude Code" (Opções da IDE → MCP) roda o `claude mcp add` por
  ele, sem shell.

*Web, Landing e documentação*
- [ ] Landing (roteiro 34, autor): o "Aylis" do topo e o texto do "Baixar a IDE" maiores; a aba do
  navegador diz "Landing" (devia ser "Aylis"); sem favicon.
  Feito em 08/11 (falta o autor confirmar): o "Aylis" do topo de 24 para 32, o "Baixar a IDE" maior,
  a aba do navegador diz "Aylis" e há favicon (o ícone da IDE; a publicação web gera o `favicon.ico`
  e os ícones do celular).
- [ ] Cache da web (roteiro 36, autor): sem servidor, abriu nos primeiros F5 e depois deu ERR_FAILED
  (o service worker não achou a página no cache).
  Feito em 08/11 (falta o autor confirmar): a página, o `.js` e o `.wasm` entram no cache já na
  instalação do service worker, o cache velho só sai com o novo completo, e a navegação acha a
  página com query, com `index.html` e sem a barra final; teste
  `the_service_worker_keeps_the_page_for_offline_use`. Falta conferir no navegador.
- [ ] IDE lite (roteiro 39, autor): a prévia do form corta o fim (a barra "Voltar ao designer" sem a
  janela crescer).
  Feito em 08/11 (falta o autor confirmar): a janela da prévia cresce os 40 px da barra "Voltar ao
  designer". Falta conferir no navegador.
- [ ] Documentação (roteiro 35): a busca destaca o texto achado na página (autor); "aylis" minúsculo
  em textos ("O aylis traz…", "Como o aylis funciona", "Componentes do Vuetify no aylis"); crases cruas
  nos títulos do índice e nos botões Anterior/Próxima.
  Feito em 08/11 (falta o autor confirmar): a busca destaca o texto achado na página aberta (o
  `Markdown` ganhou `highlight`); "Aylis" com maiúscula nos guias; as crases e o negrito saem dos
  títulos do índice, da trilha, da busca e dos botões Anterior/Próxima. Falta rodar o `aylis-tool
  docs` (as páginas de componente).
- [ ] Faltam exemplos e documentação: o `on_server_query` (só o nome do evento na página do
  `ClientDataSet`), o `Barcode` numa banda de relatório, o `on_close_query` no
  `ProcessamentoAssincrono`, e as variantes "Chart · Sankey" e "Chart · Waffle" na galeria.
  Feito em 08/11 (falta o autor confirmar): o `on_server_query` explicado no guia "Gravar as
  alterações" e na tabela de eventos das páginas de componente (coluna "Quando"); o `Barcode` numa
  banda no exemplo do `ReportPreview`; o `on_close_query` no `ProcessamentoAssincrono` (o X cancela
  a thread, e o fim dela fecha), com roteiro de teste; as variantes "Chart · Sankey" e "Chart ·
  Waffle" na galeria. Falta rodar o `aylis-tool docs`.
- [ ] O roteiro: tirar "o menu leva a seção ao topo" (os itens saíram a pedido do autor) e o "aos
  pacotes" do GradeGigante. (Feito em 08/11 na reescrita do roteiro: os dois itens saíram com os
  aprovados.)
  Feito em 08/11 (falta o autor confirmar): feito na reescrita do roteiro.

*Achados e feitos na conferência de 08/11 à noite (sem commit; falta o autor conferir o que não foi
testado na hora)*
- [ ] Renomear sem confirmar (decidido pelo autor, item 59): com o servidor de linguagem, a troca é
  automática (sem a prévia); ainda não aberto, a caixa de erro e o nome não muda; aberto e respondendo
  vazio enquanto prepara o projeto, o nome volta e a caixa de erro diz por quê; pronto e sem usos no
  código, só os outros forms trocam; a janela (por texto) só abre sem servidor de linguagem
  (`rename::refused_while_loading`, `revert`, `intel::not_started`/`preparing`). Conferido na hora
  (componente e handler, logo depois de abrir e com o servidor indexando).
- [ ] Dois renomear seguidos sem salvar: o segundo não trocava o código (o "Substituir" partia do disco e
  desistia do arquivo alterado no editor); agora parte do texto do editor. E a pesquisa lê da memória os
  forms abertos e alterados e deixa o `code-sync.json` de fora (os falsos achados depois de substituir);
  testes em `projectsearch`.
- [ ] O registro dos handlers depois de renomear um handler e dar F12: o `generate_form` marcava o form em
  dia sem atualizar o índice dos forms, e o Executar registrava o nome antigo (não compilava); teste
  `the_handler_registry_follows_a_rename_after_f12`. Conferido na hora.
- [ ] A verificação reaproveitava o erro de antes (o mesmo F12): cada form guarda a marca do JSON e do
  código, e confere de novo quando uma muda; teste `the_check_follows_the_files_even_when_nothing_is_marked_changed`.
- [ ] Executar, Compilar, Depurar e Publicar salvam antes os fontes alterados nos editores (o app rodava
  com o código velho) e gravam o campo do Object Inspector em edição (o Ctrl+Shift+F9 com o cursor no
  campo executava sem a mudança); o Salvar do editor de código salva junto o form dele; Salvar e Salvar
  tudo também gravam o campo em edição. Testes `save_before_build` e `designer_keys`. Conferido na hora.
- [ ] O duplo clique (ou o Enter) num erro das MENSAGENS abre o editor da IDE na linha (abria o VS Code).
  Conferido na hora.
- [ ] Os erros e avisos de compilação e de verificação das MENSAGENS entram na gravação de uso
  (`mensagem`), para o "veja o log" achar o que o painel já limpou; a saída do programa fica de fora.
- [ ] MCP: `set_property` com `""` ou `"null"` num evento desliga o handler (dava erro ou ligava a um
  handler "null").
- [ ] Duas conversas na mesma IDE (roteiro 95), a investigar: o Claude Desktop (modo Claude) abriu o
  projeto já aberto e usou a mesma IDE, certo, mas o registro da IDE só tinha a sessão do Claude Code. As
  duas pontes rodam o binário novo e são filhas do mesmo `claude.exe`, e o anúncio (`aylis/session`)
  mandado à mão grava. Próximo teste: a conversa do Desktop chama `session_title` com "Teste Desktop" e
  o autor diz o que a barra da IDE mostra (dois títulos: duas pontes que se anunciam; só o novo: as duas
  conversas pela mesma ponte; só o do Claude Code: a ponte do Desktop não se anuncia). Se for o último,
  pôr um log por ponte (`%LOCALAPPDATA%\aylis\mcp-bridge-<pid>.log`) com o fixar e o anúncio.


*Diretrizes do `CLAUDE.md` (roteiro 68, decididas pelo autor em 08/11)*
- [ ] "Sem linguagem própria nem expressões" vira: sem linguagem própria (nada de script, comandos,
  laços ou funções do usuário); expressões só de valor, sem efeito colateral (o `filter`, o campo
  calculado, o texto `[...]` do relatório); regra de negócio continua sendo código do app.
  Feito em 08/11 (falta o autor confirmar): feito no `CLAUDE.md`.
- [ ] "Eventos como no Delphi:" vira só "Eventos:" (a única menção ao Delphi no `CLAUDE.md`).
  Feito em 08/11 (falta o autor confirmar): feito no `CLAUDE.md`.
- [ ] A exceção dos comandos antes e depois da compilação (cmd e PowerShell só em projeto confiado,
  mostrados no "Confiar neste projeto?") na diretriz "Texto não vira código nem comando".
  Feito em 08/11 (falta o autor confirmar): feito no `CLAUDE.md`.
