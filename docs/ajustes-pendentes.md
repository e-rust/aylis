# aylis — Ajustes pendentes

O que apareceu depois do [plano inicial](plano.md) e ainda falta fazer. Quando um item
ficar pronto e o autor confirmar no teste, ele sai daqui: o que foi feito fica no commit, e o
porquê, no [histórico de decisões](historico-decisoes.md) (até 09/11 havia também a lista dos
concluídos, que saiu a pedido do autor). O que só dá para conferir num Mac, num celular ou num
PC com outro hardware fica em [testes-em-dispositivos.md](testes-em-dispositivos.md).

O plano de 01/11/2026 (9 fases) foi feito antes do primeiro commit público (`3256ba3`, 09/11). Abaixo, o que sobrou dele e o que ainda não começou.

Decidido pelo autor em 05/11: o que precisa de hardware ou de outra máquina (o ZenFone, um
leitor NFC, um aparelho Bluetooth, um controle, um Mac, uma placa NVIDIA, outro PC) fica para
depois; os itens feitos que só esperam a confirmação dele são conferidos primeiro, pelo
[roteiro de conferência](roteiro-de-conferencia.md), e saem daqui para os concluídos.

**Ajustes de 23/11/2026: as notas da versão e as medições**

Os recursos e as correções de 23/11 (as decisões no [histórico](historico-decisoes.md), seção
17; as conferências no roteiro, itens 166 a 177), sem commit.

- [ ] **Para as notas da versão** (mudanças que quebram ou mudam o que o app já fazia):
  - nos registros tipados em Rust, a cor e as medidas são `Option` (`text_color: Some(cor)`,
    `width: Some(120.0)`); o `font_weight` é o `FontWeight` do núcleo
    (`Some(FontWeight::SemiBold)`); o `GENERATOR_VERSION` subiu, e os projetos regeneram;
  - o `aylis::prelude` exporta os controles por uma lista explícita: as structs com o nome de
    um registro tipado (`ColumnSpec`, `TreeNode`...) saem dele e ficam pelo caminho completo
    (`aylis::dbcontrols::ColumnSpec`);
  - o registro das abas do `TabStrip` passou de `TabItem` a `TabStripItem`;
  - o `on_select` da `CommandPalette` recebe `PaletteSelection { id, name }` (era o nome);
  - o `EmptyState` novo vem sem o botão "Limpar filtros" (`action` vazio);
  - a linha de filtro do `DataGrid` grava `fold([campo]) like ...` no `filter` (era
    `[campo] like ...`);
  - o menu da bandeja `light`/`dark` usa as cores dos temas do Aylis (o tom muda um pouco).
- [ ] **Medições** (release):
  - `set_prop` com o mesmo valor e o layout só do contêiner alcançado: a volta do timer de 64
    frames de 20 controles, 13,76 ms → 0,014 ms (mesmo valor) e 0,27 ms (valor que muda); no
    `giant_screen`, o relógio de 4,55 para 0,66 ms por quadro;
  - SVG com máscara e recorte (`svg_effects`, 600 ícones de 32 px em 1280 × 800): Skia CPU
    3,8 ms sem efeito, 5,6 ms com efeito na primeira pintura e 2,9 ms guardado; Direct2D (alvo
    WIC) 10,1 × 11,1 ms;
  - camada de opacidade com mistura dentro no Direct2D (52 células em 1920 × 1080, o melhor
    de 10): a imagem fora da camada 106,7 → 35,6 ms; dentro 129,7 → 29,5 ms (a camada da
    mistura do tamanho da célula, não da tela);
  - `giant_screen` com a máquina parada, duas rodadas dentro da base (não regravada): pintura
    inteira 704 e 707 ms (base 696), digitar 0,81/0,82 ms (1,31), relógio 0,65/0,69 (4,57),
    60 barras 2,28/2,15 (3,09), deslizar 2,59/2,52 (2,99), rolar 18,45/18,31 (19,55).

**Ajustes de 22/11/2026: a bateria de testes sem repetir o que não mudou (`aylis-tool test-all`)**

Pedido do autor ("um app que consiga executar todos os testes de uma vez, que produza um
resultado que apresentaria diferença caso algo não esteja correto, sem precisar passar por cada
teste toda vez", depois "faça o test-all").

- [ ] Feito, esperando a conferência do autor (sem commit):
  - `aylis-tool test-all` (`aylis_tools::test_all`): compila os testes (`cargo test --no-run`),
    e a chave de cada alvo é a marca do executável mais as do que ele lê em execução,
    declarado em `resources/test-inputs.json` (329 alvos levantados no código: `pure`,
    `ignored`, uma lista de pastas e arquivos, ou `always` com o motivo: as ferramentas das
    linguagens, o hardware, compilar projetos, medir tempo). Chave igual à de uma rodada que
    passou: vem do cache (`<target>/.aylis-tests.json`). Pacotes em paralelo, os alvos de um
    pacote em sequência, os `always` e os testes da documentação (`workspace/doc`, um
    `cargo test --doc` do workspace) no fim; falha roda de novo e, se passar, sai instável.
    O relatório `medicoes/testes.json` não tem tempos: duas rodadas boas deram o mesmo arquivo;
  - `CLAUDE.md`: o passo 1 do "Antes do commit" passou a ser o `test-all` (`--all` antes de
    uma release);
  - acertos para rodar fora do `cargo test`: o teste do `cleanup` só exige o cargo como pai
    quando a variável `CARGO` está definida; o `no_delphi` aceita o nome dele no
    `test-inputs.json`; os alvos sem o harness do libtest valem pelo código de saída.
- Medido nesta máquina (22/11; a pasta de compilação no F:, recém-criada):
  - 1ª rodada (cache vazio): 44 min (34 de testes), 375 alvos, 1.649 testes; as 5 falhas eram
    do jeito de rodar (acima), não do código;
  - 2ª (depois de mudar o `aylis-tools` e um teste do `aylis`): 283 s (74 s compilando), 191 do
    cache e 140 rodaram, nenhuma falha;
  - 3ª, sem mudança nenhuma: 241 s (63 s compilando), 176 do cache e 155 rodaram (28 `always`).
- Falta:
  - **a recompilação entre rodadas sem mudança**: ~60 s de compilação e ~130 alvos que
    poderiam vir do cache rodam de novo, porque o cargo relinka os executáveis (o `rcgen` e o
    que depende dele: o `aylis-devlink`, o `aylis`, a IDE, os exemplos). Provável causa, não
    confirmada: os testes `always` que compilam projetos chamam o cargo pelo `aylis-tool` e pela
    IDE com outro ambiente (as variáveis do compilador C), e os build scripts que as anotam
    fazem o cargo refazer a próxima compilação da bateria. Caminhos: conferir com
    `CARGO_LOG=cargo::core::compiler::fingerprint=info` logo depois de uma rodada; dar a esses
    testes uma pasta de compilação própria; ou chavear pelo conteúdo do executável sem a data
    do cabeçalho PE;
  - os doctests do workspace levam vários minutos quando uma biblioteca muda.

**Ajustes de 22/11/2026: a rolagem da Landing no celular**

Pedido do autor ("investigue a lentidão ao scrolar a landing page", depois "Sim, corrija"),
medido no ZenFone 5 (Snapdragon 660, Edge 138, canvas de 1080 × 2030) pelo DevTools remoto,
com toques do próprio DevTools (o `adb input swipe` chega sem `pointerType` e o winit trata
como mouse: sem inércia, não serve para medir).

- [ ] Feito, esperando a conferência do autor no celular:
  - **o `ScrollBox` rola por cópia** (`Ui::scroll_content` → `Ui::scroll_children_damage`): os
    filhos andam e os pixels deles com eles; só a faixa que aparece e a imagem velha e a nova
    do polegar da barra são pintadas. Antes, só o `Memo` copiava, e cada passo de um
    `ScrollBox` repintava a caixa inteira. A caixa pode ser transparente sobre um fundo liso:
    o `InkCanvas` anota se o desenho de cada controle foi só um retângulo de cor única
    (`ink::Plain`), e a cópia procura esse fundo nos ancestrais (com um irmão desenhado por
    baixo, ou um fundo que não é liso, repinta como antes). Onde cada descendente desenhou anda
    junto (`shift_painted`), senão a pintura seguinte achava que vazou da área;
  - **a pintura por área pula o que está fora**: um contêiner que não recorta os filhos, fora
    da área, não desenha o próprio fundo e a sombra, e não é percorrido quando nenhum
    descendente chega à área (`Ui::descendants_reach`, só pela geometria). Na faixa de 36
    pontos da Landing, de ~90 controles percorridos para ~18;
  - o começo da inércia não pede mais a janela inteira (`request_repaint` → só um quadro);
  - **web**: a sombra é montada de uma pronta em nove pedaços (o `shadowBlur` refazia o
    desfoque a cada card, a cada quadro, e segurava a GPU do celular); o espelho de
    acessibilidade em HTML só é refeito quando a árvore para de mudar (rolando, as posições
    mudavam a cada 250 ms e refazer os elementos travava um quadro de 50–60 ms), e a
    assinatura dele é um hash;
  - teste `scrolling_a_scroll_box_copies_the_pixels_already_painted` (`partial_paint`: com e
    sem a calha da barra, e com um irmão por baixo, a imagem por partes igual à inteira).
- Medidas (inércia de um arremesso, quadros desenhados; `scratchpad` da sessão, fora do repositório):
  - antes: cada quadro repintava a página (~1.600 chamadas ao canvas, 15–34 ms de JS+wasm) e,
    na inércia, um quadro a cada 83 ms (p50; 12 fps);
  - depois: ~150–200 chamadas e 8–10 ms de JS+wasm por quadro (p50), um quadro a cada 33 ms
    (p50; 30 fps). Com a página parada 68% do tempo na inércia, o resto até 60 fps está na
    composição do navegador ou na GPU (sem texto e sem imagens a cadência quase não muda).
- Falta:
  - a cópia só vale com o deslocamento em pixels inteiros: com escala quebrada (2,625 em muitos
    Android), o `ScrollBox` rola em pontos inteiros e cada passo continua repintando a caixa.
    Rolar em pixels inteiros pede que o `Ui` saiba a escala da tela;
  - a Landing publicada tem um service worker que guarda o `pkg/` pelo nome: um build com o
    mesmo nome de arquivo não chega a quem já abriu (os publicados levam o hash no nome);
  - medir no Direct2D e no Skia (a mudança vale para os três; o `partial_paint` confere a
    imagem) e publicar a Landing de novo para o autor conferir no celular.

**Relatório de um player de música portado para o Aylis (21/11/2026)**

Um player (um form, `MediaPlayer`, `AudioMixer`, `VirtualList`, `OpenDialog`, `FileDropZone` e
um visualizador feito de painéis) portado de Tauri para o Aylis 0.9.1 no Linux Mint 20.3. O
relatório trouxe 19 problemas e 31 sugestões; conferidos no código em 21/11, e o autor decidiu
os que dependiam dele ("fazer tudo", na ordem abaixo). As decisões estão no histórico (16).

- Feito em 21/11, esperando a conferência do autor:
  - correções: o `.deb` do app abre pelo `/usr/bin/<id>` (o `Exec` com espaço não abria) e pede
    a `libasound2` com mídia; o `.deb` do WSL (Rust) usa o mesmo montador
    (`aylis_tools::app_linux`); o `Label` com `line_height` e `wrap` para na altura e põe "…"
    (`label_lines`); trocar o `file` do `MediaPlayer` não desliga mais o `AudioMixer` (os
    pontos de escuta são do componente); `playing()`/`ready()` no acesso tipado do player e
    `take_samples()` no `AudioMixer`; no Linux e no macOS compilar não recusa o app aberto; MCP:
    o `stop` espera o app fechar, o `set_theme` aceita a string com JSON, o `invoke` de
    `play`/`pause`/`seek` num player vai pelo `media` (que entrou na ajuda), o `trace` com
    `events`/`exclude`; o designer registra os ícones do `set_layout`/`set_property` antes de
    salvar; os stubs entram no `impl` que já tem handlers; `aylis-tool --version` e a versão no
    `status` do MCP. O item 5 do relatório (ícone só no código) não se reproduziu: a geração já
    leva os do código (teste `generating_keeps_the_icons_of_the_code_and_of_the_project_list`);
  - recursos: `Image.set_image_file`/`set_image_bytes`/`dominant_color` (e o aviso em modo dev
    quando o arquivo não está no projeto); `OpenDialog.kind` = `folder`; `pre_volume` nas fontes
    do `AudioMixer`; o aviso de ícone que falta e a lista `icons` do projeto (nas Opções do
    projeto → Geral); `Panel.hover_fill`/`pressed_fill`/`interactive` (com o mouse num filho,
    pelo `ControlBase::set_hover_within`); `Form.on_files_drop` com `drop_x`/`drop_y`; a linha
    tipada do `VirtualList` (`lstX_row(ui, i)`, `lstX_row_of(ui, c)`); `dialog_result` e
    `drop_files` no `app_command` (só com a feature `dev`); `Label.max_lines`;
    `SeekBar.show_labels`;
  - Linux: `publish.linux` (Projeto → Opções → Linux): o formato (`deb`, `rpm`, `appimage`,
    `flatpak`), o nome do pacote, as categorias, a descrição, o mantenedor, as dependências, os
    tipos de arquivo e as permissões do Flatpak; os ícones em 8 tamanhos e o SVG; o
    `StartupWMClass` igual ao nome que a janela declara (`Options::app_id`); o atalho no menu
    ao publicar sem instalador (`menu_shortcut`) e o `--install-shortcut`/`--remove-shortcut`
    no app publicado. O `.rpm` montado de verdade no WSL (Ubuntu com `rpmbuild`);
  - mídia: `MediaInfo` (tags, duração, capa, `probe`) com o provedor padrão (Symphonia,
    `SymphoniaTagsProvider`) e o `LoftyTagsProvider` (`aylis-providers`, feature `tags`; no app,
    a feature `media-tags`); `Playlist` (`next`, `previous`, `play_index`, `shuffle`, `repeat`,
    `on_track_change`) com a troca sem silêncio (`AudioPlayer::queue_next`); `AudioSpectrum`
    (barras, espelhado, onda, picos; para quando o som para).
  - o layout por largura pelas vistas que o núcleo já tinha (`views`; decidido de novo em
    21/11, sem form por faixa): o `on_view_change` no `Form` (o nome da vista que entrou) e a
    seção das vistas na página do `Form` (`docs/componentes/Form.md`).
  - as listas do player (22/11, decididas depois da discussão do `DataGrid`): no
    `VirtualList`, a seleção (`selection`, `item_index`, `selected`, a linha no estado
    `checked`, o `Panel.selected_fill`), os eventos com o índice (`on_item_click`,
    `on_item_dbl_click`, `on_item_activate`, `on_change`), o teclado, a `scroll_bar`, os
    métodos `select`/`scroll_to`/`refresh_item`/`refresh` e vários modelos de linha (`kinds`,
    `kind_field`, `set_kind`), com o gancho novo do núcleo `ChildPointer` (o contêiner sabe do
    mouse nos de dentro); o `ListHeader` (alinha títulos a peças do modelo pelo nome, larguras
    e `on_sort`); a linha tipada do `DataRepeater` e uma por modelo; o `ImageList` e as
    colunas `kind: icon`/`image` do `DataGrid` (`images`; a altura é o `row_height`).
  - as animações nas linhas do `VirtualList` (22/11): a linha reciclada recomeça as animações
    (`Ui::reset_animations`), o estado do item chega sem animar
    (`Ui::set_checked_state_instant`), o `appear` uma vez por item (`Ui::finish_appear`), os
    métodos `start`/`stop` das animações e o gatilho `hover_within`. Testes em
    `crates/aylis-controls/tests/list_animations.rs`; o ListaPlayer usa os quatro. Dois
    defeitos achados no exemplo e corrigidos: o `on_bind` entregue com a lista fora do `Ui` (no
    meio do `scroll_to`, por um `with_component` de outro componente; o handler tipado não
    rodava e a linha ficava com o item velho): os eventos de quem está no meio de um método
    agora esperam ele voltar (`Ui::process_events`); e a lista zerava o `translate_x` das
    linhas a cada rolagem mesmo sem `swipe_actions`.
  - o `MediaSession` (21/11): o serviço `MediaSessionService` no núcleo (começar, atualizar,
    parar; os comandos chegam por uma função), o componente no `aylis-media` (com `player`
    num `MediaPlayer` ou `VideoPlayer`, `playlist`, `info`, `active`, `forward`, `on_command`,
    `on_seek`), o SMTC no Windows (compila; sem conferir na tela), o MPRIS no Linux (testado
    pelo D-Bus de verdade no WSL: o painel lê o título e o estado, o `PlayPause` e o
    `SetPosition` chegam ao app) e a Media Session API no navegador (compila).
- Não conferido aqui (precisa de máquina ou ferramenta): o AppImage (o `appimagetool`) e o
  Flatpak (o `flatpak-builder`) não foram montados de verdade, só o manifesto e o `AppRun` têm
  teste; o código só de Linux do app (`--install-shortcut`, a classe da janela) não compilou
  no WSL (faltam as bibliotecas do GTK lá); o som de verdade do `Playlist` sem silêncio e do
  `AudioSpectrum` (só a FFT tem teste).
- Falta fazer:
  - a IDE escolher pasta pelo `OpenDialog.kind` (hoje pelo `rfd` direto em 4 lugares, com
    blocos assíncronos: uma refatoração à parte);
  - o `MediaSession` no macOS e no iOS (`MPNowPlayingInfoCenter` e `MPRemoteCommandCenter`)
    e no Android (o `MediaSession` na `AylisActivity`): o componente existe e fica parado lá;
  - o tema do designer: o relatório viu o form claro no designer com o projeto em
    `"theme": "dark"` (o app saiu escuro); a conferir.
  - As seções escritas à mão estão em `docs/componentes/` (a ordem do `align` no `Panel`, o
    `VirtualList`, o `TrackBar`, o `Image`, o `OpenDialog` e as classes novas); as páginas da
    Landing saem no próximo `aylis-tool docs`.

**Ajustes de 20/11/2026: a instalação do ambiente e o `.deb` da IDE**

- Relato do autor em 20/11: quem usa o Aylis tem pouca experiência com Rust, e instalar e
  configurar o ambiente é a parte que mais falha: empresas bloqueiam o Google Drive (o espelho
  padrão dos pacotes, sem outra origem) e não se sabe se o Linux funciona. Levantado: os
  pacotes só vêm do cache, da pasta `pacotes` ou do espelho (o Drive); o Rust do Windows (MSVC)
  precisa das Build Tools do Visual Studio, que não são pacote (pelo `winget`); o Linux nunca
  foi testado numa máquina limpa (o WSL usado já tinha as bibliotecas).
  - Decidido e feito em 20/11 (veja [pacotes.md](pacotes.md) e o histórico): a URL oficial do
    fabricante é a última origem, ligada por padrão, com o SHA-256 do manifesto (o Drive saiu
    do padrão e ficou só para o que o Aylis monta); sem origem, o erro traz o passo a passo.
    As máquinas limpas foram testadas depois (o Fedora 45, o Windows 11, o Mint 22.3, o macOS).
  - Ainda em aberto (propostas sem decisão): o kit offline publicado como arquivo da release
    do GitHub, e tirar as Build Tools do Visual Studio do caminho do Rust no Windows (o
    `rust-lld` ou o Rust do MinGW, a pesquisar; empacotá-las não pode, pela licença).
- Pedido do autor em 20/11 ("prepare para ser compatível com .deb do linux"; ele instala o
  Linux numa VM para testar). Feito (`aylis-tools::ide_package::linux_deb`, `aylis-tool
  ide-package --target linux --binary aylis-ide --deb`): o `.deb` da IDE, montado em Rust (o
  `ar` e os `.tar.gz`, sem o `dpkg-deb`), instala em `/opt/aylis-ide` (a IDE, o `aylis-host`, o
  `aylis-tool`, o `aylis-migrate` e o código do Aylis), põe a IDE (pelo lançador do WSL) e o
  `aylis-tool` em `/usr/bin`, e o atalho, os tipos de arquivo e os ícones em `/usr/share`, com o
  `postinst`/`postrm` atualizando os bancos do sistema. As dependências (`Depends`) são as
  bibliotecas de compilação de `aylis_tools::wsl::PACKAGES` (as da CI): o `apt install
  ./aylis-ide_0.9.0_amd64.deb` as instala junto. A versão de pré-lançamento vira `0.9.0~beta.1`.
  Conferido no WSL (Ubuntu 26.04): o `dpkg-deb --info` lê o pacote; o `apt install` instala; a
  IDE de `/opt` abre (`--png`); um projeto novo compila com o `aylis-tool` do `/usr/bin` (70 s);
  o `apt remove` tira tudo. Teste `the_deb_installs_in_opt_with_the_launcher_and_the_dependencies`.
  - Testado numa VM limpa do Hyper-V com o Fedora 45 (20/11), pelo `.tar.gz` e o
    `instalar.sh` (sem `sudo`), por SSH e pelo MCP da IDE da VM: a IDE abre, um projeto novo
    compila e roda, e o ciclo inteiro (ligar o `on_click`, o esqueleto, o código, compilar,
    clicar e ver o texto mudar) funciona. Achado e corrigido no caminho:
    - a verificação de ambiente só conhecia o `apt`: agora `aylis_tools::linux_deps` reconhece
      a distribuição (`apt`, `dnf`, `zypper`, `pacman`), confere o compilador C e cada
      biblioteca pelo `pkg-config` e monta o comando com os nomes de cada uma; roda pelo `sudo`
      sem senha quando a máquina deixa, senão pelo `pkexec`;
    - o `aylis-tool build` (e os outros comandos que compilam) não achava o Rust dos pacotes do
      Aylis: agora chama o `setup::refresh_path`, como a IDE;
    - a IDE ficava segundos sem responder (o GNOME oferecia "Forçar saída"): sem placa de vídeo,
      o modo automático desenhava no OpenGL por software (llvmpipe). Agora, achando o OpenGL
      por software, a janela desenha na CPU (e as seguintes já vão direto): rápida;
    - o assistente "Configurar linguagens" da primeira abertura abria por cima da pergunta de
      confiar no projeto e a bloqueava, e no Wayland ficava atrás da IDE: o "Confiar" não
      fazia nada. Agora ele espera o modal fechar; e, no núcleo, clicar numa janela bloqueada
      por um modal traz o modal para a frente e o faz piscar (`Ui::blocking_modal`);
    - o `aylis-tool new` não marcava o projeto como confiável (o "Novo projeto" da IDE marca);
    - o GNOME mostrava a janela como "Desconhecido": agora o `app_id` (e o `WM_CLASS`) é o
      nome do executável, o mesmo do `.desktop`;
    - o OK das Opções apagava os espelhos dos pacotes (o padrão também) e as pastas: o campo
      (um `Memo`) era lido como `Edit` e vinha vazio. Teste `ok_keeps_the_package_sources`;
    - na CPU, no Wayland, passar o mouse nas abas (Propriedades × Eventos, Design × Código)
      deixava lixo piscando: o Wayland alterna buffers, e a pintura por parte só refazia as
      áreas do quadro. Agora o `cpu.rs` guarda as áreas dos últimos quadros e, pela idade do
      buffer, pinta também o que ele perdeu (teste `an_older_buffer_also_paints_what_it_missed`).
      Confirmado pelo autor na VM.
  - Decidido pelo autor em 20/11 ("eu não queria depender do meu google drive.. foi uma decisão
    ruim"): os pacotes vêm do fabricante (a URL oficial do manifesto, por último, com o mesmo
    SHA-256), e o espelho padrão do Drive sai. Feito (`toolpacks::fetch`, `settings`): a
    origem oficial ligada por padrão (`package_official = off` desliga, para a empresa que só
    aceita as origens dela); quando nenhuma origem dá, o erro traz o passo a passo para
    instalar à mão (o endereço, o SHA-256 e o comando que o confere em cada sistema, a pasta,
    tentar de novo). Conferido na VM com o Fedora, sem pasta `pacotes`, sem cache e sem
    ferramentas: o `setup rust` baixou o Rust do `static.rust-lang.org` e o CodeLLDB do GitHub
    dele, em 68 s. Testes `the_official_url_is_the_last_source_and_the_error_says_how_to_do_it_by_hand`
    e `the_official_url_off_asks_nothing`.
  - Falta: instalar o `.rpm` da IDE num Fedora (ele sai desde a 0.9.1; veja abaixo); a chave `package_official` só pelo
    `tools.json` (sem caixa nas Opções nem opção no `aylis-tool packages sources`).
  - Testado numa VM limpa com o Ubuntu 26.04 (21/11) e no Mint 22.3 (22/11); falta o Debian. O Rust não vem no `.deb` (o `cargo` do Ubuntu é velho
    para a edição 2024): vem pelos pacotes de ferramentas ou pelo rustup. Distribuições antigas
    (Ubuntu 20.04, Debian 10) não têm o `libfreetype-dev` com esse nome.
  - O "Corrigir" das linguagens no Linux mandava sempre o `apt` (no Fedora falhava): agora o
    `setup::linux_toolchain` tem os nomes de cada linguagem no apt, dnf, zypper e pacman (o
    `golang`, o `nodejs-npm` e o `java-latest-openjdk-devel` do Fedora 45, conferidos na VM),
    e o JDK do Android também. Teste `each_distribution_gets_its_own_package_names`.
  - Testado do zero na VM do Fedora 45 em 21/11 (sem a IDE, sem os pacotes do Aylis e sem as
    bibliotecas de desenvolvimento): o `.tar.gz` e o `instalar.sh`; o `setup rust` (o Rust e o
    CodeLLDB do fabricante, as bibliotecas pelo `dnf`, 21 s); um projeto Rust compila (204 s) e
    abre no Wayland; as outras 7 linguagens ficam prontas pelo `aylis-tool setup`, e um app de
    cada uma publica e abre com o backend vivo. Achado e corrigido:
    - o `./instalar.sh` dava "Permissão negada": o `ide-package` gravava o script e os
      executáveis com 644. Agora 755 (Linux e o `.app` do macOS);
    - o Python do Fedora vem sem o `pip` (`No module named pip`): o debugpy e o servidor de
      linguagem falhavam. Agora há o item "pip", instalado pelo gerenciador da distribuição;
    - o .NET 8 não roda no OpenSSL 4 do Fedora 45 ("No usable version of libssl was found": sem
      NuGet, sem o csharp-ls). Agora há o item "OpenSSL 3", com o pacote de compatibilidade de
      cada distribuição (`openssl3-libs` no Fedora);
    - o servidor de linguagem do Pascal não tinha instalador fora do Windows: agora o pasls é
      compilado com o git e o lazbuild do sistema; e o Lazarus do Fedora (`/usr/lib64/lazarus`)
      passou a ser achado.
  - Depurar e o servidor de linguagem nas 8 linguagens, pela IDE da VM (pelo MCP), em 21/11:
    um botão com `on_click`, ponto de parada no handler, executar, clicar, a pilha, as variáveis,
    o "Avaliar" e o passo; um erro de propósito vira diagnóstico, a dica e as sugestões. Tudo
    funciona nas 8. Achado e corrigido:
    - Rust: a primeira parada levava minutos, as variáveis não vinham e o F8 não voltava: o
      resumo de struct do Rust 1.99 (`StructSummaryProvider`) junta o de todos os campos, sem
      limite, e o `ui: &mut Ui` tem centenas. O `lldb/rust_formatters.py` (era o
      `rust_hashmap.py`) limita o resumo a um nível e 6 campos; e o LLDB carrega os símbolos sob
      demanda. Agora a parada leva 1 a 8 s e o passo, 1 s;
    - Go, C++ e Free Pascal não paravam em nada: o Yama do Linux (`ptrace_scope = 1`, o padrão do
      Fedora e do Ubuntu) só deixa depurar os filhos, e o depurador se anexa ao backend. Agora o
      `aylis-host` faz o backend autorizar a IDE (`prctl(PR_SET_PTRACER)`, só esse processo e os
      filhos dele; `AYLIS_DEBUG_PID` leva o pid da IDE);
    - a IDE caía (estouro de pilha) ao realçar o `aylis.hpp` no passo do C++: um grupo de captura
      da gramática do C++ casava o mesmo trecho de novo, sem fim. Agora no máximo 16 grupos um
      dentro do outro (teste `the_clients_highlight_without_endless_recursion`);
    - Java: o JDT LS 1.38 não roda no Java 25 do Fedora; agora usa o JDK 17 do pacote. E todo
      arquivo mostrava "The import dev cannot be resolved": o `.classpath` gerado não tinha o
      cliente do Aylis; agora tem, por uma pasta ligada ao `.aylis/sdk` do projeto, e o antigo é
      refeito (teste `the_java_project_sees_the_aylis_client`);
    - Python: todo handler mostrava "undefined name 'Button'" (a anotação `sender: "Button"` sem
      import); agora o import do form traz as classes das anotações (gerador 27; teste
      `the_python_import_brings_the_sender_classes`);
    - o MCP dizia "o app abriu e fechou" ao depurar o Rust, com o app rodando.
  - Duas falhas de teste que apareceram na conferência do commit, corrigidas a pedido do autor
    ("corrija as duas falhas também"):
    - um form regravado no mesmo milissegundo não era lido de novo (o `uses` do DataModule
      sumia; teste `data_modules_come_first_and_forms_reach_their_components`): a anotação dos
      forms usava a hora em milissegundos; agora é o `FileStamp` (nanossegundos e tamanho). Um
      `.aylisproj` de antes ainda abre;
    - o servidor HTTP do MCP (o da IDE e o `McpServer` dos apps) soltava o soquete logo depois
      de responder, e no Windows uma resposta grande chegava cortada ("conexão cancelada pelo
      host remoto"; os testes `mcp` da IDE). Agora encerra a escrita e espera o cliente fechar.
  - Limitações vistas: o pasls (Pascal) não publica diagnósticos, só sugestões e dicas (é do
    servidor); o csharp-ls só manda os diagnósticos depois de o projeto carregar, e uma edição
    feita durante o carregamento só aparece na seguinte (a IDE poderia pedir de novo, o
    `textDocument/diagnostic`, quando ele termina de carregar).
  - Ubuntu 26.04 limpo (outra VM, 21/11): a IDE pelo `.deb` (`apt install`, 114 s, com as
    bibliotecas de compilação) e as 8 linguagens pelo `aylis-tool setup`. Achado e corrigido:
    - o debugpy não instalava: o Ubuntu recusa o `pip install` no Python do sistema (PEP 668,
      "externally-managed-environment"). No Linux, agora vem do pacote do sistema
      (`python3-debugpy`);
    - o pasls não compilava: o Ubuntu não traz o `git`; agora ele é instalado antes;
    - o Node das distribuições (o 22 do Ubuntu) vem sem o TypeScript ("ERR_NO_TYPESCRIPT") e
      vinha antes do Node do pacote no PATH: o `hostlang::node` agora testa cada um
      (`process.features.typescript`) e usa o que roda TypeScript;
    - o `dotnet` do Ubuntu (só o runtime 10) vinha antes do SDK do pacote: o publicar do C#
      falhava, e o csharp-ls fechava ao abrir (feito para o .NET 8). Agora o C# usa o primeiro
      `dotnet` com SDK, e o csharp-ls recebe o `DOTNET_ROOT` dele;
    - o pasls não dava dica nem sugestões: o Lazarus do Debian fica em `/usr/lib/lazarus/<versão>`
      e os fontes do Free Pascal em `/usr/share/fpcsrc/<versão>`; agora são achados;
    - a docstring que o gerador põe nos forms Python novos tinha linhas acima de 79 caracteres
      (o aviso E501 em todo projeto).
  - Depurar e o editor nas 8 linguagens também no Ubuntu, pela IDE (MCP), em 21/11: tudo
    funciona (o C# com a limitação do csharp-ls de antes).
  - Pedido do autor em 21/11: "quero poder gerar o .deb de um app no windows e no linux". Antes
    o `.deb` só saía pelo WSL e só em Rust. Agora:
    - no Linux, o Publicar com "Instalador" gera o `.deb` em Rust puro (`aylis_tools::app_deb`,
      com o `ar` e o `tar.gz` do pacote da IDE), nas 8 linguagens: o app em `/opt/<app>`, o
      atalho `/usr/bin/<app>`, o `.desktop` (com os `UrlScheme`), o ícone do projeto, as
      bibliotecas do sistema e o runtime da linguagem (o `python3` como dependência; Node, .NET
      e Java recomendados). Teste `the_app_deb_installs_in_opt_with_the_menu_entry`;
    - no Windows, o alvo Linux das outras linguagens usa o mesmo publicar do Linux pelo WSL: o
      `aylis-tool` de Linux é compilado no WSL a partir do código do Aylis da máquina (a primeira
      vez leva alguns minutos), prepara a linguagem e publica. Conferido: um app Python publicado
      no Windows virou `.deb`, instalou pelo `apt` no Ubuntu e abriu com o backend; os 8 apps
      publicados no Ubuntu viraram `.deb`, instalaram e abriram pelo atalho com o backend;
    - o app publicado em TypeScript leva `.js` (o publicar tira os tipos com o Node da máquina,
      `stripTypeScriptTypes`): o Node das distribuições não roda `.ts`, e o app não abria. Roda
      em qualquer Node 18+;
    - o backend em C# é compilado com `RollForward=Major`: o Ubuntu 26.04 só tem o .NET 10, e o
      app (do .NET 8) não abria;
    - a opção do Publicar virou "Instalador (Windows: o tipo do projeto; Linux: .deb)".
  - O `.rpm` da IDE (`aylis-tool ide-package --target linux --rpm`, pelo `rpmbuild`; a mesma
    árvore do `.deb`, com as dependências pelos nomes do `dnf`) saiu na release 0.9.1, conferido
    por dentro (os arquivos, as permissões, os scripts e as dependências). Falta: instalar
    num Fedora (o autor publicou sem esse teste, com a VM desligada) e o
    Executar no WSL das outras linguagens (só o publicar).
  - Pedido do autor em 20/11 ("devemos ter a opção de baixar ainda de lá tudo que for
    preciso"): a pasta do Drive é o espelho do Aylis (`settings::AYLIS_MIRROR`), junto da origem
    oficial, só para o que o Aylis monta (o Skia de 32 bits, o Free Pascal); posta como espelho
    nas Opções, serve para tudo. Teste `what_aylis_builds_comes_from_the_aylis_mirror`. A pasta
    precisa continuar com o `pacotes.json` e esses dois arquivos.

**Ajustes de 22/11/2026: o macOS (VM Intel) e o Linux Mint**

- [ ] Pedido do autor: "verifique oque precisa para rodar no mac" e "vamos testar também na vm do
  linux mint". O Aylis nunca tinha sido compilado num Mac: numa VM do macOS 15.8 Intel (Hyper-V,
  sem GPU), com as Command Line Tools e o rustup, o `aylis-tool` e a IDE compilaram de primeira.
  Achado e corrigido:
  - todo app do Aylis (a IDE inclusive) caía ao abrir fora de um `.app` (o `cargo run`): a
    `UNUserNotificationCenter` derruba o processo sem pacote. O push da Apple só liga com
    `bundleIdentifier`; fora dele, "o APNs só registra o app empacotado (.app)";
  - o "Verificando as linguagens…" ficava na barra para sempre (em todas as plataformas, na tela
    inicial): o fim de um trabalho do ambiente não limpava o status. E o status de um trabalho
    agora sai traduzido (o título com "…");
  - os rótulos em maiúsculas da IDE sem tradução (RECENTES, MENSAGENS, COMPONENTES, APARÊNCIA,
    APLICATIVO, ESCOLHIDO, ESCOLHIDA, FIXAS, PRONTAS). O `aylis-tool translations` não os acusa
    (falta conferir por quê);
  - os pacotes de ferramentas tinham só o Mac Apple Silicon: o Mac Intel caía no Homebrew, que o
    macOS não traz. Agora há `macos-x86_64` do rust, jdk, android-sdk, cargo-ndk, wasm-bindgen,
    node, dotnet e go (do fabricante, com SHA-256). O netcoredbg não publica para o Mac Intel:
    o C# roda, sem depurar. O que só sai pelo Homebrew (o Free Pascal) diz para instalá-lo
    (brew.sh) em vez do "brew: command not found";
  - o `/usr/bin/javac` e o `/usr/bin/jdb` do macOS são atalhos que, sem JDK, só avisam: o Java
    passava como pronto, o JDK do pacote não era instalado e o JDT LS fechava. Agora vale o
    `javac` que responde, e o `jdb` e o `java` saem da pasta dele;
  - o `setup` dizia "aylis-host não encontrado" depois de compilá-lo (o caminho era lido antes);
  - o LLDB e o Delve só param no app com o modo de desenvolvedor do macOS ligado: a verificação
    de ambiente avisa e o "Corrigir" liga (`DevToolsSecurity -enable`, com a senha numa janela);
  - o app em C++ terminava quando o depurador se anexava no macOS: o LLDB para o processo com
    SIGSTOP, a leitura do stdin volta com `EINTR` e o `std::getline` falhava como se a tela
    tivesse fechado. O cliente C++ (`clients/cpp/aylis.hpp`, `read_line`) repete a leitura
    interrompida. O Go e o Free Pascal já repetiam.
  Conferido no Mac: as 8 linguagens instaladas pelo `setup` (o Pascal com o Homebrew, que não
  foi instalado), o Rust, o TypeScript, o Python, o Go, o C++ e o Java rodam; depuram (param no
  ponto de parada, com a pilha, as variáveis e o passo) o Rust, o Go, o C++, o TypeScript e o
  Java (o `total` do Java veio "null": conferir; o LLDB não avalia expressão em Rust); o editor
  (diagnósticos, dica, sugestões) no Rust, no TypeScript, no C#, no Go, no C++ e no Python.
  Falta: o Pascal (precisa do Homebrew), o JDT LS no Mac, a câmera no macOS (`cameras()` vazio)
  e um Mac de verdade.
- [ ] **Os pacotes Linux da 0.9.1 só rodavam com a glibc 2.43** (compilados no WSL, Ubuntu
  26.04): no Mint 22 (Ubuntu 24.04, glibc 2.39) a IDE e o `aylis-tool` nem abriam. Recompilados
  num contêiner Ubuntu 22.04 (glibc 2.35, o commit 0c4b568e: a mesma 0.9.1) e conferidos no Mint
  22.3: instalou pelo `.deb`, a IDE abriu e as 8 linguagens ficaram prontas pelo `setup`; o
  Rust, o TypeScript, o C#, o Java, o Go, o C++ e o Pascal depuram e o editor responde. Espera o
  autor: trocar os três arquivos na release do GitHub.
  - Decidido pelo autor em 22/11 ("Ubuntu 22.04, glibc 2.35"; antes tinha escolhido o zigbuild
    com a 2.31, que não serve: o Skia pré-compilado do `skia-safe` é feito contra a libstdc++ do
    GCC e o zig liga com a libc++ dele, e a libstdc++ estática do GCC 15 pede a glibc 2.38,
    conferido no WSL): a glibc mínima dos executáveis do Linux é a 2.35 (Ubuntu 22.04, Mint 21,
    Debian 12, Fedora 36). Feito:
    - no Windows, o publicar e o Executar para Linux usam a distribuição `Ubuntu-22.04` do WSL
      quando ela existe (`wsl::distro`; o `AYLIS_WSL_DISTRO` continua mandando); a verificação de
      ambiente instala essa (`wsl --install -d Ubuntu-22.04`) e avisa, com o "Corrigir", quando a
      usada tem glibc mais nova que a mínima;
    - o executável é conferido pelas versões que pede no ELF (`aylis_tools::glibc`, sem o
      `objdump`; conferido contra o `objdump -T` no `ls` e no `g++` do WSL): no publicar (no
      Linux e pelo WSL), passar da 2.35, da libstdc++ do GCC 12 ou pedir a `GLIBC_PRIVATE` vira
      aviso com a versão pedida, e a tela e o backend compilado são conferidos;
    - o `ide-package --target linux` recusa a IDE, o `aylis-host`, o `aylis-tool` e o
      `aylis-migrate` que pedirem mais que a 2.35 (o que escapou na 0.9.1). A IDE da release
      sai da `Ubuntu-22.04` do WSL, como saiu do contêiner.
    - Conferido em 22/11: a `Ubuntu-22.04` instalada no WSL pelo mesmo comando do "Corrigir"
      (glibc 2.35), as bibliotecas e o Rust (1.100) pelos outros dois; o publicar de um projeto
      novo foi para ela sozinho (não para a `Ubuntu` 26.04); compilados lá, o exemplo `shapes`
      (com o Skia) e o `aylis-tool` pedem no máximo a `GLIBC_2.35`.
    - Achado e corrigido (já existia; a pedido do autor, "Sim, corrija"): o publicar e o
      Executar pelo WSL falhavam com o `sdk` do projeto numa pasta e nos exemplos do
      repositório, porque o gerador grava o caminho do Aylis relativo ao projeto (`../../...`) e
      a pasta-espelho (`~/.aylis-wsl/<pacote>`) só trocava os absolutos (`C:/...`). Agora os
      relativos passam a partir da pasta do código de verdade (`wsl::prepare`). Conferido: um
      projeto novo com o `sdk` no repositório publicou pelo WSL (o executável, "glibc pedida:
      2.35", e o `.deb`), o `.deb` instalou pelo `apt` na `Ubuntu-22.04` com todas as
      bibliotecas achadas (`ldd`) e saiu pelo `apt remove`.
    - Conferido na VM do Mint 22.3 (glibc 2.39) em 22/11: esse `.deb`, feito no Windows pela
      `Ubuntu-22.04`, instalou pelo `apt`, o `ldd` achou todas as bibliotecas e o app abriu a
      janela no X11 do Cinnamon (desenhando na CPU, sem placa de vídeo); depois, removido.
    - Falta: no Linux, publicar numa distribuição mais nova ainda sai preso à glibc dela (só o
      aviso; compilar num contêiner Ubuntu 22.04 seria o próximo passo); abrir um app publicado
      assim num Mint 21 ou num Ubuntu 22.04 com desktop (a glibc igual à mínima).

**Ajustes de 20/11/2026: o Windows limpo (uma VM com o Windows 11 Home, sem nada instalado)**

- [ ] Pedido do autor: "desinstalando todas as ferramentas que o aylis usa hoje e instalando por
  ele, pra garantir que funciona". Numa VM do Hyper-V, pelo instalador da IDE e pelo
  `aylis-tool setup`. Achado e corrigido:
  - a IDE e o `aylis-tool` nem abriam (`0xC0000135`): faltava o runtime do Visual C++
    (`VCRUNTIME140.dll`, `VCRUNTIME140_1.dll`, `MSVCP140.dll`), que vem com o Visual Studio e não
    com o Windows. Agora o instalador da IDE e o publicar para Windows (Rust e as outras
    linguagens) levam as três ao lado do executável (`aylis_tools::vcruntime`; a Microsoft
    permite o "app-local"). Testes `the_visual_cpp_runtime_goes_beside_the_executables` e
    `the_newest_redist_folder_wins`;
  - o servidor de linguagem do Rust, do Go, do C# e do C++ não era achado no PATH: a busca
    procurava `rust-analyzer.exe.exe` (o `.exe` duas vezes). E a correção do rust-analyzer
    chamava o `rustup`, que o Rust do pacote não tem: agora instala o pacote `rust`, que já o traz;
  - o Python do pacote do Aylis não era achado: o atalho da Microsoft Store
    (`WindowsApps\python.exe`, só abre a loja) vinha antes no PATH. Agora o `env::python` tenta
    cada um do PATH e devolve o caminho inteiro;
  - o servidor de linguagem do Pascal não compilava: o script do `cmd /C` ia com as aspas
    escapadas (`\"`), que o cmd não entende. Agora vai cru (`raw_arg`, `cmd /S /C`);
  - o app publicado em TypeScript, C# e Java abria a janela sem backend (o Node, o .NET e o Java
    estavam só nos pacotes do Aylis, fora do PATH do sistema), avisando só num toast que some; o
    de Python sumia calado (o atalho da Microsoft Store no lugar do Python). Agora o
    `spawn_backend` procura cada um do PATH (os atalhos da loja por último) e os pacotes do
    Aylis; sem o runtime, o `MessageDialog` diz qual instalar e fecha no OK; e o backend que
    termina ao abrir mostra o motivo (o fim do stderr dele) antes de fechar;
  - o app publicado em Python e em TypeScript terminava ao abrir em qualquer máquina: o
    `aylis_client.py` e o `aylis.ts` do `src` só apontam para `../../.aylis/sdk/clients/...`, que
    não vai na pasta publicada. Agora o publicar põe o cliente de verdade no lugar
    (`publish::bundle_client`).
  - Conferido na VM, do zero: o instalador silencioso põe a IDE, o Rust e o Visual Studio Build
    Tools; o `aylis-tool setup` deixa prontas as outras 7 linguagens (Python, Node, .NET, JDK e
    Go dos pacotes do fabricante; clangd; Lazarus pelo winget, com o pasls compilado); um
    projeto Rust compila (5 min 44 s) e abre (Direct2D, 136 fps); um projeto em cada uma das 7
    publica e abre com o backend vivo (`node`, `dotnet`, `java`, `python`, os `app.exe` de Go,
    C++ e Pascal).
  - A correção do `spawn_backend` chegou ao crates.io na 0.9.2 (na VM foi conferida com um
    `[patch.crates-io]` para a cópia instalada); falta conferir na VM com o `aylis` do crates.io.
  - Depurar e o editor pela IDE nas 8 linguagens, na mesma VM, em 22/11 (pedido do autor:
    "teste a depuração e o editor na VM do Windows"), com a IDE 0.9.1 do instalador:
    - a 0.9.0 que estava na VM caía (estouro de pilha) no passo do C++: é o realce do
      `aylis.hpp`, já corrigido na 0.9.1;
    - depuram (param, pilha, variáveis, "Avaliar" e passo) o TypeScript, o C#, o Java, o Go, o
      C++, o Python e o Free Pascal. O TypeScript só para se o clique vem depois de o js-debug
      se anexar (uns segundos depois de a janela abrir);
    - o editor (diagnósticos, dica, sugestões) responde no Rust, no TypeScript, no C#, no Java,
      no Go, no C++, no Python e no Pascal (o pasls sem diagnósticos, como no Linux);
    - o pylsp não abria ("Permission denied" no `__init__.py`): instalado por um processo
      elevado (o SSH; também um terminal de administrador), o `pip --target` monta tudo numa
      pasta do `mkdtemp`, que no Windows (Python 3.12.4+) só dá acesso ao dono, e o dono era o
      grupo Administradores; a IDE, sem elevação, não lia. Agora a correção do pylsp roda
      depois um `icacls <pasta> /reset /T` (`Fix::after`), e as permissões voltam às herdadas.
      Conferido na VM: o editor do Python responde;
    - **o Rust não depura no Windows** (na VM e nesta máquina, com o CodeLLDB 1.12.3): o app
      lançado pelo `launch` do CodeLLDB carrega as bibliotecas e trava sem abrir a janela; o
      LLDB acha que ele roda, e a thread nova espera o depurador no `LdrInitializeThunk`. Pelo
      `lldb` direto e num programa sem janela não trava; anexado depois de aberto (como o C++),
      funciona. E o "Parar" deixava o app travado vivo, prendendo a compilação seguinte.
      Decidido pelo autor ("Anexar e esperar"): no Windows, a IDE abre o app com
      `AYLIS_WAIT_DEBUGGER=30` e o `aylis-tool` avisa o processo assim que ele sobe
      (`AYLIS_BACKEND_PID=`, como o aylis-host); o app com `dev` espera o depurador no começo do
      `Application::run` (`aylis_platform_winit::wait_for_debugger`), e a IDE anexa o CodeLLDB
      (`Session::attach_rust`, com os formatadores do Rust). Um app com o `aylis` de antes (o
      do crates.io) não espera, mas é depurado do mesmo jeito a partir da anexação. Conferido
      nesta máquina: com o OlaNome por um script, o app espera, o CodeLLDB se anexa e para na
      primeira linha do `run_with_pump` (logo depois da espera); pela IDE (compilada à parte,
      um projeto com o `aylis` do repositório), o Ctrl+F5 com ponto de parada no
      `btnOi_click` para na linha, com a pilha e as variáveis, o passo leva 1 s e o Parar
      encerra o app. Achado no caminho: o CodeLLDB do Windows cai ao avaliar um nome que não
      existe no quadro, e o app morre junto (`STATUS_DEBUGGER_INACTIVE`). Agora o "Avaliar" de
      um nome só vai se ele estiver entre as variáveis ("total não existe neste ponto"), e o
      passo mede a memória do processo (o contador do aylis não é achado anexado). O `total`
      do teste não aparecia nas variáveis: era uma constante só descartada (`let _ = (ui,
      sender, total)`), e o compilador não a guarda; usada de verdade (`ui.push(total)`), o
      LLDB do Windows mostra `total = 42` e avalia a expressão (conferido num programa à parte).
      Falta: a VM (a espera está no `aylis` 0.9.2 do crates.io).
  - Conferido: o instalador silencioso instala a IDE e o Rust (do `static.rust-lang.org`) com o
    Visual Studio Build Tools; a IDE abre; um projeto Rust novo compila (5 min 44 s) e abre
    (Direct2D, 136 fps).

**Ajustes de 20/11/2026: uma janela por monitor, a que fica por cima e o arrasto entre elas**

- [ ] Relato do autor em 20/11 (um app dele, feito com uma LLM): abrir uma janela por monitor,
  umas em tela cheia e outras não; a do monitor principal levada para cima das outras e
  continuando visível; arrastar itens entre todas as janelas. No app, o arrasto que começava
  na janela em tela cheia não chegava à janela do outro monitor.
  - Testado com dois monitores 4K no exemplo novo `multi_monitor` (uma janela comum no
    principal, uma em tela cheia no outro e uma paleta `topmost`), com o mouse simulado
    (`SendInput`): as 9 combinações de arrasto (comum, tela cheia e paleta, entre si e na
    mesma janela) chegam ao destino, inclusive da tela cheia para a comum. A paleta, levada
    pela barra de título para cima da tela cheia no outro monitor, continua visível e recebe
    o soltar.
  - Achado e corrigido (`aylis-core`, `forms.rs`): o form principal era possuído pelo último
    form aberto. O `show` faz de cada form aberto o dono do próximo, e o `run` mostra o
    principal por último; num app que abre antes as janelas dos outros monitores e a paleta,
    o principal ficava possuído pela paleta, por cima dela e com o `topmost` dela. Assim a
    paleta sumia atrás da principal, e soltar nela (ou começar o arrasto nela) caía na janela
    de baixo. Agora o principal nunca tem dona (também quando é escolhido depois de aberto,
    no `set_main_form`). Teste `the_main_form_is_never_owned`.
  - Achado e corrigido (`aylis-core`, `input.rs`): o controle com `draggable` ou
    `accepts_drops` recebe o mouse mesmo com o `hit_test` desligado. O Label nasce com ele
    desligado, e ligar o `draggable` num Label não fazia nada, sem aviso. Teste
    `a_label_drags_without_turning_hit_test_on`.
  - Achado e corrigido (`aylis-platform-winit`, `resize_edge`): na tela cheia de um form com
    a barra própria (`border_style = custom`), a borda de 6 px continuava redimensionando (a
    seta aparecia e puxar tirava da tela cheia); só a maximizada deixava de redimensionar.
    Conferido com o `window_start full --border <estilo>` e o mouse nas bordas: nos quatro
    estilos a janela cobre o monitor inteiro (a barra de tarefas também) e o cursor fica a
    seta; sem a correção, o `custom` mostrava a seta de redimensionar.
  - Pedido do autor em 20/11: fixar o monitor em que o form nasce em tela cheia e, com o app
    rodando, escolher em que monitor ele ocupa a tela toda. Nascer: `monitor` e
    `window_state = full_screen` no form (já valia). Com o app rodando, feito
    (`aylis-controls-base`, `aylis-platform-winit`):
    - o `monitor` mudado com o form aberto leva a janela para lá (antes, só valia ao nascer);
    - em tela cheia, o `move_to_monitor` (e o `monitor`) passa a tela cheia para o outro
      monitor: sai dela, leva a janela comum para o centro do outro e entra em tela cheia lá
      (trocando direto, a volta ao normal ficava do tamanho do monitor);
    - o `move_to_monitor` atualiza o `monitor`, e a tela cheia ligada depois vai para o
      monitor certo.
    Conferido com o `window_start switch` (a cada 2 s: tela cheia no 0, tela cheia no 1, normal
    no 1 com o tamanho do form, normal no 0, tela cheia no 0), olhando o retângulo da janela
    por fora.
  - Revisão pedida pelo autor em 20/11 (os recursos do form com vários monitores). Achado e
    corrigido: o `remember_placement` em tela cheia gravava o retângulo da tela cheia como o
    lugar normal, e a janela reaberta vinha do tamanho do monitor, com moldura. Agora o
    `WindowPlacement` tem `full_screen`: em tela cheia, o lugar normal fica e a marca vai junto
    (com o monitor dela), e o form reabre em tela cheia no monitor em que estava (o guardado
    vale mais que o `monitor` do form, como a posição). Conferido com o `window_start
    --remember`: comum → tela cheia → reabre em tela cheia → volta ao normal → reabre comum,
    no tamanho do form.
  - Para decidir com o autor (vistos na revisão, não feitos):
    - o `monitor` é um índice na ordem do sistema, que pode mudar quando um monitor é
      desligado e ligado de novo (o `remember_placement` já guarda pelo nome do aparelho);
    - um monitor desligado com o app aberto: as janelas dele ficam onde o Windows as puser, e
      ao religar não voltam (o `Screen.on_change` avisa; o app decide). Não testado com uma
      janela em tela cheia nele;
    Decidido pelo autor em 20/11: o `monitor` continua o índice na ordem do sistema; para o
    monitor desligado, basta o evento (`Screen.on_change`).
  - Pedido do autor em 20/11 ("pode fazer"): os recursos do Windows que o form não expunha.
    Feito, cada um conferido na tela com o exemplo novo `window_features`:
    - `request_attention(critical)` (método): o botão na barra de tarefas pisca até a janela
      ser ativada (urgente: a janela também). Visto o botão destacado na barra de tarefas;
    - `keep_awake` (`off`, `system`, `display`): o maior pedido dos forms abertos, pelo
      `SetThreadExecutionState`. Conferido pelo estado de energia do sistema: `system` segura o
      PC, `display` segura a tela também, e fechar devolve o normal;
    - `bottommost`: sempre por baixo das outras (com `topmost`, vale o `topmost`). Conferido
      ativando a janela: ela continua atrás da comum;
    - `confine_cursor` (`none`, `window`, `monitor`): o mouse preso na janela ou no monitor
      dela enquanto ela está ativa (o `monitor` pelo `ClipCursor`, de novo a cada foco).
      Conferido o `window` (o mouse não sai da área da janela; fechar solta); o `monitor` só
      com mais de um monitor, que não estava ligado;
    - `window_opacity`: a janela inteira translúcida (`WS_EX_LAYERED`). Conferido a 50% no
      Direct2D, no Skia CPU e no Skia GPU (o mesmo tom);
    - `full_screen_mode = exclusive`, com `video_width`, `video_height` e `video_refresh`: o
      monitor passa para o modo de vídeo mais perto do pedido. Conferido: de 3840×2160 a 160 Hz
      para 1920×1080 a 60 Hz e de volta ao fechar.
    `request_attention`, `bottommost` e `confine_cursor = window` também valem no Linux e no
    macOS pelo winit (não conferidos lá); os outros, só no Windows por enquanto.
  - Pedido do autor em 20/11: o form que entra em outro e sai dele, "como as abas do
    navegador". Feito (`aylis-controls-base::docking`, `aylis-core`, `aylis-platform-winit`):
    - `host` do form (pelo código): entra no contêiner (numa faixa de abas, vira uma aba com o
      título dele); aberto numa janela, ela some sem fechar o form (o `Ui::release_window`, sem
      `on_close`); vazio, ele volta a ter janela, com o tamanho de antes. Eventos `on_dock` e
      `on_undock`. O principal e um modal não entram;
    - arrastando: a aba de um form com `dockable` puxada para fora da faixa vira a janela do
      próprio form, onde foi solta; a janela de um form com `dockable`, arrastada pela barra de
      título até uma faixa com `accept_forms`, entra nela como aba, no lugar marcado
      (`Ui::window_drag_over`/`window_drag_end`, pelos mesmos ganchos `drop_*` do arrasto).
    Conferido no exemplo novo `docking`, com o mouse simulado: os dois forms começam como abas;
    a aba do Clientes puxada para fora vira a janela dele (520×340, o mesmo conteúdo); a
    janela arrastada pela barra até a faixa volta como aba, depois de Pedidos; o botão (o
    `host`) tira e põe de volta. Testes `a_form_docks_into_a_tab_strip_and_back` e
    `a_dragged_window_docks_where_it_is_dropped`. O arrasto da janela até a faixa é só do
    Windows por enquanto (nas outras, pelo `host` e pela aba).
  - Pedido do autor em 20/11: o `app_command` simular o arrasto de um form para outro. Feito
    (`aylis-remote`): o `drag` com `to` noutro form arrasta de uma janela do app para outra (o
    `drag_over_form`, como a plataforma faz quando o mouse sai da janela); sem `to`, com
    `screen_x`/`screen_y`, solta fora de todas (a aba arrastada para longe vira janela); e o
    comando novo `drag_window` leva a janela inteira de um form até um componente de outro (a
    faixa com `accept_forms` recebe o form com `dockable`). No manual do `app_command`. Testes
    `drag_goes_from_one_form_to_another` e `drag_window_docks_a_form_and_drag_out_undocks_it`.
  - Relato do autor em 20/11: a pergunta "Confiar neste projeto?" às vezes ficava atrás da
    janela da IDE, travando tudo. Causa: o projeto aberto durante a tela de abertura (pelo MCP
    ou por um duplo clique) abre a pergunta com a abertura como dona; a abertura fecha antes de
    a janela da IDE existir, e a plataforma deixava a pergunta sem dona. Feito
    (`aylis-platform-winit`): a dona passa a ser o form principal mesmo sem janela ainda, e a
    janela dele, ao abrir, assume as que esperavam (`adopt_owned_windows`). Conferido abrindo
    um projeto de fora pelo MCP com a IDE nova: a pergunta fica com a janela da IDE como dona
    (antes, sem dona). E com o `window_features --splash --confirm`.
  - Visto nos testes de 20/11 (não é defeito): com um monitor só, a cópia de tela pela GDI não
    enxerga a janela do Skia GPU sem nada por cima (ela vai para o plano de overlay da placa);
    para fotos de teste, usar o Skia CPU ou pôr outra janela por cima de um canto.
  - Falta: o autor conferir no app dele, com três monitores. A falha da tela cheia para a
    janela comum não se repetiu aqui; se continuar lá, pode depender de monitores com escalas
    diferentes (aqui os dois estão na mesma) ou da ordem em que o app abre as janelas.

**Ajustes de 19/11/2026: a IDE para Linux, testada no WSL**

- Pedido: "gere a IDE para o linux e teste no meu wsl".
  - Feito: o `aylis-tool ide-package --target linux` leva, além da IDE, o `aylis-host`, o
    `aylis-tool` e o `aylis-migrate` compilados ao lado dela e a pasta `aylis/` com o código do
    Aylis (como o instalador do Windows); o `instalar.sh` copia para `~/.local/share/aylis-ide`
    e põe a IDE (um lançador) e o `aylis-tool` em `~/.local/bin`. Compilado no WSL (Ubuntu
    26.04, ~4,5 min em release), o pacote tem 338 MB (128 MB em `.tar.gz`).
  - Conferido no WSL (como root, WSLg): instalar a partir do `.tar.gz`, abrir a IDE, criar um
    projeto, compilar (2m53s na primeira vez) e Executar, com o app aberto ao lado da IDE e
    conectado a ela.
  - Achado: o compositor do WSLg (o weston) às vezes cai (segfault no `libpixman`) quando uma
    janela desenhada pelo EGL no Wayland convive com outra, e leva junto todos os apps do Linux
    ("Broken pipe"). É intermitente: de 1 em 1 a 0 em 8 nas mesmas condições. Não é do Aylis
    (diretriz 7), mas a plataforma agora, no WSL, vai pelo X11 (o Xwayland do WSLg) e põe o Mesa
    no d3d12 (a placa do Windows; sem ele, o desenho vai para a CPU no llvmpipe), como o Executar
    da IDE do Windows no WSL já fazia (`aylis_tools::wsl`). Pelo X11 nenhuma queda foi vista. O
    lançador da IDE faz o mesmo, para valer também para os apps compilados com a 0.8.0. Como a
    queda é intermitente, não dá para afirmar que sumiu: fica para o autor conferir no uso.
  - Também: quem roda o WSL como root não tem o soquete do Wayland no `XDG_RUNTIME_DIR`; fora do
    WSL, com o `WAYLAND_DISPLAY` apontando para um soquete que não existe, a plataforma vai pelo
    X11 em vez de não abrir.

**Ajustes de 19/11/2026: a janela sem o branco da abertura e a tela cheia**

- Pedido: "a janela exibida antes de ser pintada o primeiro quadro, fazendo o app piscar em
  branco na abertura" e "mandar uma janela ser fullscreen ou já nascer fullscreen tem que ser
  verificado se funciona corretamente".
  - Feito: a janela nasce escondida e só aparece com o primeiro quadro pintado (`open_window`
    pinta antes de mostrar). No Windows ela fica mostrada mas encoberta (`DWMWA_CLOAK`; a swap
    chain de uma janela escondida descarta o quadro) e é descoberta depois do quadro, com um
    `DwmFlush`; o form que nasce escondido e aparece depois (o `show`) também. E o fundo da
    janela (`WM_ERASEBKGND`) é apagado na cor do form: o que aparece por baixo da swap chain
    antes do quadro era branco.
  - Conferido no Windows (Direct2D e Skia na CPU), com o exemplo `window_start` gravado da tela
    a ~30 qps (`gdigrab`): nos modos normal, já em tela cheia, tela cheia pedida depois e de
    volta ao normal, a área da janela passa do fundo direto para a cor do form, sem quadro
    branco (antes, no Direct2D, a superfície da janela aparecia branca). A tela cheia, de
    nascença e pedida depois, cobre o monitor inteiro com a barra de tarefas
    (`GetWindowRect` = `(0,0)-(3840,2160)`, o form com 2560×1440 lógicos na escala 1,5), e a
    volta ao normal devolve o tamanho e o lugar de antes.
  - Falta: a tela cheia num segundo monitor (a máquina tem um só) e o Linux e o macOS (lá a
    janela nasce escondida e aparece com o primeiro quadro; não conferido): em
    [testes-em-dispositivos.md](testes-em-dispositivos.md).

**Ajustes de 18/11/2026: o acesso tipado o mais tipado possível**

- [ ] Pedido do autor em 18/11: "esses nomes não deveriam estar tipados?" (o `window_state` como
  texto) e "mapeie tudo que poderia estar tipado também e não está.. devemos manter o mais
  tipado possível"; do mapa, "Fazer todas" as etapas. Em andamento:
  - Feito (etapas 1 e 2): o catálogo leva o tipo Rust e os valores dos enums e dos conjuntos
    (`enum(WindowState:normal|...)`); o gerador das 8 linguagens dá um tipo a cada um (o Rust usa
    o do framework, pelo `aylis::prelude`; os 17 que faltavam lá foram exportados, e
    `crates/aylis/tests/catalog_enums.rs`, gerado com o catálogo, confere). A cor (`Color`:
    RGBA ou um nome do tema), as margens (`Bounds`) e a referência a outro componente (o objeto
    dele; o protocolo aceita o id e devolve id, nome e classe com `"ref": true`) ganharam tipo
    nos 7 clientes. `GENERATOR_VERSION` 14.
  - Feito (etapa 4, a parte simples): os tipos `Date`, `Time`, `DateTime`, `List`,
    `NumberList` e `ColorList` do `PropKind` (com `as` nos campos de texto, sem mudar o que o
    form grava); o `Date`/`Time`/`DateTime` passou do `aylis-data` para o `aylis-core`. No
    acesso tipado: a data de cada linguagem (`Option<Date>`, `datetime.date`, `DateOnly?`,
    `LocalDate`, `time.Time`, `TDateTime`...) e listas de texto, número e cor, em 60
    propriedades. As escolhas fixas que eram texto viraram enum: `Chart.legend_position`,
    `title_align`, `value_format` e `Form.present` (vazio continua valendo). `GENERATOR_VERSION` 15.
  - Conferido: os testes do gerador, o teste das apps (Calculadora, Paint, Player e IBGE dão o
    mesmo estado nas 8 linguagens, com o Paint passando a cor e a ferramenta tipadas) e um
    projeto com DateEdit, TimeEdit, ChipGroup e ActivityRings compilado e rodado nas 8.
  - Feito (etapa 3, decisão do autor em 19/11: "A: change the signature"): o handler recebe quem
    disparou na classe dele e o valor do evento no tipo. Cada componente declara o que o
    evento leva (`events: on_change: f64` no `properties!`; o trait `EventValue` tira o valor,
    o `DateRange` é o do Calendar com `range`), e o catálogo grava (`"on_change":
    "event:date"`): 143 eventos declarados pelo que os componentes mandam, 129 ainda sem tipo
    (ficam com a assinatura de sempre). O esqueleto novo nasce tipado nas 8 linguagens; o
    handler de antes (o evento inteiro) continua sendo chamado do jeito dele: no Rust, o
    `form_handler` aceita as duas assinaturas (o compilador escolhe); nas outras, a geração lê a
    assinatura no código do usuário e escreve a chamada certa no registro (no Pascal, um
    despacho por form em `aylisforms.pas`). Nas outras linguagens, o ponteiro, a roda, o gesto,
    o deslize, a tecla e o intervalo de datas ainda vão com a assinatura de sempre, e também o
    handler ligado a componentes de classes diferentes. `GENERATOR_VERSION` 16.
  - Conferido: um projeto com Button, DateEdit e TrackBar, os handlers tipados e um do jeito
    de antes, rodado nas 8 linguagens (o clique, a data que chega no tipo e o de antes); os
    testes do núcleo (`typed_handlers`) e do gerador (`handlers_get_the_sender_and_the_value_typed`).
  - Feito (etapa 3, 19/11): 122 eventos declarados (115 nos controles, a sombra e a curva dos
    editores com `EventValue` novos, os três `on_error` sem disparo achado como texto). Nas
    outras linguagens: o ponteiro, a roda, o gesto, o deslize e a tecla chegam como registros
    (`PointerEvent`, `KeyEvent`... com os campos que o protocolo já manda); o intervalo de datas
    como `DateRange` (início e fim nas datas de cada linguagem); o componente do
    `TabStrip.on_new_window` pelo id (menos no Pascal, onde o objeto não teria dono); o handler
    ligado a componentes de classes diferentes recebe quem disparou como o componente genérico
    (no C# e no C++, o evento ganhou o id de quem disparou; no C++, `Component::name()`). No
    caminho: o Java tomava `PointerEvent value` pela assinatura antiga (procurava `Event `).
    `GENERATOR_VERSION` 23.
  - Conferido (etapa 3): o projeto de teste nas 8 linguagens (o `on_pointer_down` de um Panel
    pelo arrasto do roteiro, `clicks:1`; um handler num Button e num CheckBox, com o nome de
    quem disparou; o `Calendar` com intervalo, `d2-5`).
  - Feito (etapa 3, 19/11, "Mini linguagem em estrutura"): os eventos com texto estruturado
    levam um registro (struct do framework no Rust, com `EventValue`; um registro gerado nas
    outras): `MasterChange`, `ServerQuery`, `GetTextRequest`, `SetTextRequest`, `ExceptionEvent`,
    `CropSelection`, `CalendarEventClick`, `PermissionResult`, `SharedContent`, `MapPoint`,
    `BluetoothDevice`, `BluetoothValue`, `TabMove`, `SwipeAction`, `ColumnLayout`. O
    `OptionGroup.on_change` leva sempre a lista dos marcados (no rádio, um só; o `selected`
    acompanha o `item_index`).
  - Falta (etapa 3): o `WmiQuery.on_event` (as propriedades vêm da classe WMI da consulta, sem
    campos fixos), o `Chart.on_click` (`série;índice`: é o `on_click` comum) e o `on_pick`/`on_cursor`
    do `PaintCanvas` (texto da barra de status) seguem texto. Os eventos-pergunta e os do
    `EventInterceptor` têm a assinatura deles.
  - Feito (19/11): os exemplos na assinatura tipada (Calculadora, Paint, Player e IBGE nas 8
    linguagens; Abas, OlaNome, Relatorios, TesteAparelho, GradeGigante, Galeria e Estresse3D em
    Rust; ProcessamentoAssincrono em Pascal). Ficam como estavam só os eventos-pergunta
    (`form_key_down`, `form_close_query`).
  - Feito (etapa 5, 19/11): a sombra (`Shadow`), a curva (`Easing`) e os pontos (`PointList`,
    o `Polygon.points`) tipados; os enums nos argumentos e nos resultados dos métodos
    (`Permissions.state(Permission) → PermissionState`, `Screen.set_orientation`,
    `Biometrics.availability`; o `MethodInfo` ganhou o tipo Rust deles); os registros
    (`RecordInfo`, `$records` no catálogo): o `Screen.monitors`/`monitor` devolve o `Monitor`
    com os `VideoMode`; os itens com campos num texto (`PropKind::Items`): o `ListView.items`,
    o `Timeline.items`, o `EventCalendar.events`, o `CommandPalette.items` e os `buttons`/`tasks`
    do `Taskbar` viram listas do registro; o `Form.gpu_adapters` virou lista. Nos 7 clientes,
    `Shadow`, `Easing` e `Point`. No caminho, o `FloatAnimation.property` quebrava o Python (o
    nome escondia o `@property`). `GENERATOR_VERSION` 21.
  - Conferido (etapa 5): o projeto de teste nas 8 linguagens (a sombra e os pontos gravados e
    lidos de volta, a lista do `Screen.monitors`, os itens do `ListView` com um item a mais) e a
    leitura do `Monitor` com dados de verdade em Python; o teste
    `structures_records_and_method_enums_are_typed`.
  - Feito (etapa 5, 19/11, "Sem compatibilidade com versão anterior, Mini linguagem em
    estrutura"): as mini-linguagens viraram listas de registros (`PropKind::RecordList`; no
    form, JSON de verdade, e no fio, o texto do JSON). Saiu o `PropKind::Items` (`a|b;c`). Os
    registros podem ter cor, data, listas, enum e registro dentro (o `TreeNode` tem os filhos;
    `RecordInfo` compara e mostra pelo nome). Convertidos: os itens do `ListView`, os nós do
    `TreeView`, o `Timeline`, o `EventCalendar`, o `CommandPalette`, o `Taskbar`, o menu do
    `TrayIcon`, as ações da `Notification`, as linhas do `MarkupTable`, as abas do `TabStrip`
    (a macro aceita `as` nas calculadas), o `VirtualList.item_bindings` (era `bindings`, o mesmo
    nome da ligação de dados do form), os marcadores do mapa, as séries do `Chart`, o trader e o
    BI (velas, indicadores, livro de ofertas, cotações, regiões), os botões do `PushButtons` e
    das notificações, os efeitos do `AnimatedText`, o `Permissions.reasons`, o NFC, as colunas e
    o rodapé do `DataGrid`, o `master_fields`/`indexes`/`params` do `ClientDataSet` (504 forms
    do ERP), as saídas e as fontes de áudio. Métodos: `Bluetooth.paired`, `Phone.history` e
    `PaintCanvas.size` devolvem registros. Nas 8 linguagens: ler e gravar a lista (`to_json`), e
    formas curtas para criar (`new ChartSeries { Name = "S" }` no C#, `newChartSeries({...})`
    no TypeScript, `ChartSeries.of().withName("S")` no Java). No caminho: o C++ lia a lista de
    registros vazia (o temporário do `Json::parse` morria antes do laço). `GENERATOR_VERSION` 24.
  - Conferido (etapa 5): o projeto de teste nas 8 linguagens (um TreeView com um nó filho e uma
    série de Chart com enum e números, gravados e lidos: `Filho:2:line:1`); os testes do
    gerador, do catálogo e dos controles mexidos.
  - Feito (19/11): no Object Inspector, o "…" de uma lista de registros abre o editor de
    coleção (`aylis_ide::records_editor`): os itens à esquerda (acrescentar, tirar, subir,
    descer) e os campos do escolhido à direita, cada um no editor do tipo (caixa para o
    booleano, lista para o enum, texto para número, data e cor, `a; b` para as listas); a lista
    dentro do item (os filhos de um nó) abre o mesmo editor por cima. O que fica vazio sai do
    JSON. Testes `records_editor` da IDE.
  - Segue texto, de propósito: o `PushTrigger.filter` (expressão lida também pelo Java) e o
    `PivotTable.data` (tabela sem campos fixos).
  - Conferido antes do commit (19/11): o `cargo check` para wasm de todos os crates mexidos e da
    IDE lite; o `cargo ndk` (x86_64) dos mesmos; o `giant_screen` em release; a suíte do
    workspace e os 4 apps nas 8 linguagens. A IDE de desktop não compila para Android por causa
    do `trash` (dependência do `aylis-tools` que já estava no commit anterior; a IDE de desktop
    não roda no Android).
  - Feito (etapa 6, 19/11): os nomes do projeto viram enums. O núcleo ganhou
    `PropKind::StyleName`, `ThemeName` e `FormName` (no fio, texto como antes): o `style` de
    cada classe é um enum dela (`ButtonStyle`: as variantes registradas e as dos temas do
    Aylis, que o catálogo guarda em `$styles`, mais as dos temas do projeto), o `theme` é o
    `ThemeName` (os temas do Aylis, `$themes`, e os de `themes/` do projeto) e o `form` do
    `Frame`, o `item_form` do `VirtualList`, o `form` da `Notification` e o do `WakeAppAction`
    são o `FormName` (os forms do projeto). O primeiro valor, vazio, é o `Default`. Mudar um
    tema ou a lista de forms refaz o arquivo gerado (a marca dos temas entra no índice); o
    camelCase separa as palavras (`frmOutro` → `FRM_OUTRO`). No Object Inspector, o seletor de
    forms do Frame vale para toda referência a form. `GENERATOR_VERSION` 22.
  - Conferido (etapa 6): o projeto de teste nas 8 linguagens (um tema `Empresa` com a variante
    `Button.perigo`, um segundo form num `Frame`: os três gravados pelo enum e lidos no
    roteiro); o teste `project_names_are_typed` (com a variante nova que entra ao mudar o tema).
  - Feito (etapa 6): os forms dos pacotes (`pkg:Termometro` → `FormName::PkgTermometro`; a
    lista de `packages/` entra na marca que refaz o gerado). Um estilo que o app registra só em
    código (`ui.register_variant`) não tem como ser visto ao gerar: declarado no tema do projeto
    (`"Button.meu": {}`), entra no enum.
  - Quebra o código que passava texto nessas propriedades (na 0.x pode; vai nas notas da versão).

**Ajustes de 18/11/2026: o iOS no iPhone pelo `aylis-tool`**

- [ ] Pedido do autor em 18/11: gerar o app para o iPhone (com a assinatura) direto pelo
  `aylis-tool`, sem passar pelo Xcode, mesmo sem Mac para testar. Feito em 18/11
  (`aylis_tools::ios_device`):
  - `aylis-tool publish app.aylisproj --target ios --device`: compila para `aarch64-apple-ios`
    (o iOS mínimo 14.0, no `IPHONEOS_DEPLOYMENT_TARGET` e no `MinimumOSVersion`), põe o perfil
    como `embedded.mobileprovision`, escreve os direitos a partir dele (a equipe, o app, o
    `get-task-allow`, o `aps-environment` com o `PushNotifications`), assina com o certificado
    e grava o `.ipa` (`Payload/<App>.app`). Sem `--device`, o simulador, como antes.
  - O perfil e o certificado são da máquina, nunca do projeto: o perfil é achado nas pastas
    onde o Xcode grava, pelo bundle id (o exato antes do curinga), pelo aparelho e pela
    validade; o certificado, pelo SHA-1 dos que o perfil aceita. As Opções podem fixar os
    dois (`ios_profile` e `ios_sign_identity` no `tools.json`, ou `AYLIS_IOS_PROFILE` e
    `AYLIS_IOS_IDENTITY`). Sem perfil que sirva, o erro diz por que cada um ficou de fora e
    como conseguir um (com a conta gratuita, só pelo Xcode).
  - `aylis-tool run app.aylisproj --target ios --device [--serial udid]`: o iPhone conectado
    (o único pareado, ou o pedido), instalado e aberto pelo `xcrun devicectl` (Xcode 15+), com
    o console do app na saída.
  - O ícone do projeto agora é compilado pelo `actool` para dentro do `.app`, no simulador e no
    iPhone (antes, o catálogo ficava ao lado, sem uso). O `Info.plist` ganhou a plataforma, a
    versão mínima e a família (iPhone e iPad).
  - O Verificar ambiente do iOS mostra o certificado de desenvolvedor do Keychain.
  - Pedido do autor em 18/11, depois: "pode fazer o --no-sign e o workflow do github" (compilar
    sem Mac e mandar o app para qualquer iPhone). Feito:
    - `publish --target ios --device --no-sign`: o `.ipa` só com a assinatura "ad hoc", sem
      certificado nem perfil, para o Sideloadly assinar no Windows com o Apple ID de quem
      instala (a conta gratuita serve; o app vale 7 dias).
    - `aylis-tool ci-ios app.aylisproj [--signed]` (`aylis_tools::github_ci`): grava
      `.github/workflows/ios-<app>.yml` no repositório do projeto. Na máquina macOS do GitHub:
      baixa o projeto e o Aylis na tag desta versão (`v<versão>`), compila o `aylis-tool`,
      roda o `generate` (que liga o `.aylis/sdk` ao Aylis baixado) e o `publish`, e deixa o
      `.ipa` nos Artifacts. Roda só pelo "Run workflow" (o minuto de macOS conta dez vezes em
      repositório privado). Com `--signed`, o `.p12`, a senha e o perfil vêm dos segredos do
      repositório para um Keychain temporário.
    - O workflow baixa a tag da versão do `aylis-tool` que o gerou (o `--device` existe desde a
      0.8.0).
  - Pedido do autor em 18/11, depois: "quero essa configuração de workflow nas configurações do
    projeto pelo Aylis IDE, não quero ter que ficar decorando comandos". Feito: Projeto →
    Opções do projeto → **iPhone (GitHub)** (`crates/aylis-ide/src/iosview.rs`, a interface do
    `aylis_tools::github_ci`): o passo a passo; o repositório (lido do `.git/config`, sem rodar
    o git) e o workflow (criado ou não, assinado ou não, de que versão do Aylis); a escolha da
    assinatura; Criar/Atualizar o workflow, Abrir no GitHub (a página do workflow, com o Run
    workflow) e Abrir a pasta. Sem assinatura: como instalar pelo Sideloadly e o link dele.
    Assinado: os três segredos, com "Copiar de um .p12…" e "Copiar de um .mobileprovision…"
    (leem o arquivo, convertem para base64 e copiam; o perfil é conferido contra o bundle id
    e a validade antes) e o link da página dos segredos. Fora da IDE lite (precisa de disco).
    Os textos nos 11 idiomas; as telas em `tests/visual/opcoes-iphone*-windows.png`.
  - Falta: nada rodou num Mac nem no GitHub Actions (veja [testes-em-dispositivos.md](testes-em-dispositivos.md)); a
    IDE ainda executa o iOS só no simulador (a escolha do iPhone no Executar vem depois da
    conferência no Mac); a publicação para a App Store (o perfil de distribuição e o envio pelo
    Transporter) não foi feita.

**Ajustes de 18/11/2026: Reveal Highlight nos estilos**

- [ ] Pedido do autor em 18/11: "suporte ao efeito Reveal Highlight, não precisa ser nativo, só
  que tenha nos componentes do estilo, permitindo habilitar". Feito em 18/11:
  - Tema: `reveal` (`border`, `background`, `both`, `none`), `reveal_color` e `reveal_radius`
    no estilo (`StyleProps`, lidos e gravados pelo `aylis-json`); resolvidos em
    `ResolvedStyle::reveal` (`theme::Reveal`). A cor padrão: o token `reveal`, ou branca
    translúcida no escuro e escura translúcida no claro. Nasce desligado; as variantes
    `Button.reveal` e `Panel.reveal` do tema padrão ligam num controle (`style = reveal`).
  - Núcleo: o `Ui` pinta a luz (um gradiente radial centrado no mouse) depois do controle: na
    borda, com o mouse a até o alcance; no fundo, com o mouse em cima (segue a forma, com
    `clip_shape`). O mouse do form fica no estado de entrada (`Ui::pointer_position`; o toque
    não acende). O mouse que anda repinta só a parte dos controles com Reveal que a luz
    alcança (`Ui::reveal_moved`, pela lista dos pintados com Reveal: o custo é o deles, não o
    do form).
  - Limitação: num controle guardado como camada (`cache_layer`), a luz fica na camada até
    ela ser refeita.
  - Conferido: os testes (`aylis-controls/tests/reveal_highlight.rs` no Skia e no Direct2D: a luz no
    fundo e na borda perto do mouse, nada além do alcance, apagando ao sair; a repintura só
    perto da luz; `aylis-json`: o tema lido e gravado) e a imagem de uma grade de botões no
    tema escuro com o Direct2D. Não conferido com o mouse de verdade na tela (exemplo
    `reveal`).

**Ajustes de 18/11/2026: form transparente, com forma e vidro fosco**

- [ ] Pedido do autor em 18/11: "formulário transparente com suporte a bordas arredondadas
  personalizada, com antialiasing para permitir até formulários redondos, deve permitir semi
  transparência no form também para permitir o efeito de vidro fosco". Feito em 18/11:
  - Form: `transparent` (a janela deixa ver o que está atrás onde o form não é opaco: o `fill`
    com alfa é a semitransparência), `window_shape` (`rectangle`, `rounded` com `shape_radius`,
    `ellipse`) e `shape_radius`. Fora do retângulo a janela já fica transparente, sem a moldura
    do sistema e sem redimensionar pela borda. Mudam com o app rodando (`UpdateCorners`).
  - Núcleo: `Component::clip_shape` (`ClipShape::RoundRect`/`Ellipse`): o fundo, a sombra, o
    vidro do tema e os filhos ficam dentro da forma, com a borda suave (`clip_round_rect`,
    `clip_path`); um controle com forma não conta como cobrindo o retângulo. O designer mostra a
    forma.
  - Windows com o Direct2D: o modo de apresentar `PresentMode::Transparent` (a composição com
    alfa pré-multiplicado) e a janela sem a superfície do GDI (`WS_EX_NOREDIRECTIONBITMAP`); o
    fundo começa transparente. A região da janela (`SetWindowRgn`, 1 pixel maior que a forma)
    faz os cliques fora da forma irem para o que está atrás. A pintura por parte com várias
    áreas limpa com cópia (antes, o fundo com alfa se acumulava).
  - Vidro fosco: o `blur` do `WindowSettings` passa a valer no Windows (o winit não faz nada lá):
    o desfoque do sistema (`CreateHostBackdropBrush`, `DWMWA_USE_HOSTBACKDROPBRUSH`) numa camada
    da composição do Windows por baixo do quadro, recortada pela forma com a borda suave. O
    `backdrop = acrylic` não serve para janela com forma (o DWM desenha a barra de título por
    cima e ignora a região), nem a política de destaque (`ACCENT_ENABLE_BLURBEHIND`, que fica só
    de reserva, desfoca o retângulo inteiro).
  - Limitações: no Windows com o Skia (OpenGL ou CPU), a janela fica opaca e retangular (o
    conteúdo sumia e a barra do sistema aparecia), e o form pinta a forma sobre a cor de fundo.
    macOS e Linux: `with_transparent` e a configuração do OpenGL com transparência, escritos e não
    conferidos (testes-em-dispositivos). Celular e navegador: sem efeito.
  - Forma personalizada (pedido em 18/11, depois da primeira entrega): `window_shape = path` com
    o contorno em `shape_path` (o `d` do SVG, esticado para o form: os limites do contorno viram
    os do form) e `shape_radii` (um raio por canto no `rounded`, na ordem do CSS). No núcleo,
    `ClipShape::Corners` e `ClipShape::Path` (`ClipShape::outline` dá o contorno de qualquer
    forma). No Windows, a região sai dos polígonos do contorno (dilatada em 1 pixel) e o
    desfoque é recortado pela geometria do Direct2D do contorno (`CompositionPath` com um
    `IGeometrySource2DInterop` próprio). Conferido na tela: uma estrela translúcida com o
    desfoque, as pontas sem serrilhado, o clique fora da estrela indo para trás, e o vidro com
    o canto de baixo à esquerda menor.
  - Conferido na tela (Windows 11, Direct2D, exemplo `shaped_forms`): o vidro com 27% de branco
    e o form redondo azul de trás desfocado, os cantos sem serrilhado na ampliação, e o clique
    no centro do círculo indo para ele e fora do círculo para a janela de trás. Testes:
    `aylis-controls/tests/window_shape.rs` (Skia e Direct2D: a elipse com o filho recortado e a
    borda suave, os cantos, o alfa do fundo e o pedido à janela).

**Ajustes de 17/11/2026: arrastar para uma janela do app que está por cima**

- [ ] Relato de um app do autor em 17/11 (uma janela pequena por cima de uma grande: soltar
  um item na de cima caía na grande, embaixo): o Aylis só
  procurava outra janela do app quando o mouse saía da janela onde o arrasto começou. Feito
  em 17/11 (`aylis-platform-winit`): durante um arrasto, a cada movimento a plataforma
  pergunta ao Windows qual janela está à vista naquele ponto (`win32::top_level_at`,
  `WindowFromPoint`); se é a de outro form do app, o destino é ela (acende ao passar, recebe o
  soltar) e o form de baixo não recebe o movimento. A imagem do arrasto é transparente ao
  mouse e não atrapalha. Fora do Windows continua como antes (o winit não diz a ordem das
  janelas): vai para testes-em-dispositivos. Conferir no app: arrastar da janela grande e
  da lista lateral para a de cima.

**Ajustes de 17/11/2026: ícones no TreeView e onde cada nó é pintado**

- [ ] Sugestão do autor em 17/11: "a lista em árvore que não permite ter ícones nos itens sem
  interferir no alinhamento do texto, e não informa onde um item é pintado a posição do item".
  Feito em 17/11 (`lists.rs`): `TreeNode.icon` (`with_icon`) e, no `items`, `texto|ícone`
  (como o `texto|detalhe` do ListView), com a coluna do ícone em todos os nós quando algum
  tem ícone. Depois de duas idas e vindas sobre as folhas ("só o último nível que empurrou o
  texto pra frente" e "rio de janeiro e minas ficaram desalinhados"), o autor pediu "o mais
  personalizável possível": `leaf_layout` (`aligned`, o padrão: todo texto do mesmo nível
  junto; `icon_in_arrow`: o ícone da folha no lugar da seta, o texto alinhado; `compact`:
  sem o espaço da seta) e `indent`, `icon_size` e `icon_gap`. Sem ícone nenhum, o desenho é
  o de antes. Onde cada nó
  é pintado: `TreeView::node_rects(caminho)` (a linha, a seta, o ícone e o texto) e, para as
  outras linguagens, `item_rect(path)` e `item_text_rect(path)` pelo caminho de textos do
  `on_change`, em `Bounds` nas coordenadas do controle (`Null` com o nó escondido). A
  amostra da galeria ganhou pastas. Teste `crates/aylis-controls/tests/tree_icons.rs`.
  Mudança de comportamento: um `|` no texto de um nó do `items` passa a separar o ícone.

**Ajustes de 17/11/2026: o piscar ao fechar uma janela**

- [ ] Sugestão vinda do NoOBS em 17/11 (o editor de print piscava preto ou branco ao fechar):
  "ao fechar uma janela, o Aylis solta a superfície de desenho antes de destruir a janela...
  A correção de verdade é no próprio Aylis: esconder a janela antes de soltar a superfície,
  em sync_windows". Feito em 17/11: o `sync_windows` esconde as janelas dos forms que
  fecharam antes de soltar o `FormWindow`, e o mesmo vale para os pedaços extras de um form
  em vários monitores (`span.rs`). Conferir no NoOBS, sem o contorno de esconder e fechar
  50 ms depois no `close()` do `print_editor.rs` (que pode sair de lá).

**Ajustes de 17/11/2026: o clique no item pai e o conteúdo próprio no item de menu**

- [ ] Pedido do autor em 17/11: "Em um popup com submenu não tem como capturar o clique desse
  menu pai, preciso que tenha.. um popup não esta aceitando mais botões dentro dele a frente
  do texto, botões de ação como ativar ou desativar, com ícone e cor de fundo.. imagino uma
  forma de colocar um frame dentro do item". Feito em 17/11 (`menus.rs`):
  - o item com submenu recebe o clique (`on_click`, `on_item_click`), no painel e na barra
    do `MainMenu`, e o menu fica aberto com o submenu; `close()` no `PopupMenu` e no
    `MainMenu` fecha pelo código;
  - `MenuItem.content`: um controle do form (Layout com botões, Frame) mostrado no item
    enquanto o menu está aberto, à direita do texto ou no item inteiro sem texto; a linha
    cresce até a altura dele; o menu o pega emprestado (filho da camada do menu) e devolve
    pai, posição, alinhamento e visibilidade ao fechar; o foco num botão dele não fecha o menu.
  Testes em `crates/aylis-controls/tests/menu_items.rs`. Conferir num app: um PopupMenu com um
  item de `content` apontando para um Layout invisível com dois botões coloridos com ícone.
  O que não é coberto: setas e Esc enquanto o foco está num botão do conteúdo vão para o botão.

**Ajustes de 17/11/2026: o publicar das outras linguagens no desktop com dois executáveis**

- [ ] Pedido do autor em 17/11, ao publicar um projeto em Free Pascal: "eu achava que ao
  compilar ia sair no máximo 2 .exe na pasta, um só com a tela e tudo que precisa só pra ela
  funcionar e outro do backend". Saíam o `aylis-host.exe` genérico (45 MB), os forms, temas,
  assets, traduções e o `.aylisproj` soltos (com os comandos de antes e depois da compilação e
  o caminho da chave do Android), o atalho `.cmd` e as units do Free Pascal (`.o`/`.ppu`).
  Feito em 17/11 (falta o autor conferir publicando pela IDE): `publish::desktop_shell` monta
  `<App>.exe` (a casca de `codegen::local_shell`, com o projeto embutido) e `backend/` (o
  programa compilado, sem o que só serviu para compilar: `hostlang::INTERMEDIATES`). Conferido
  com uma cópia do MeuProjeto (Pascal): `MeuProjeto.exe` (12,9 MB) e `backend\app.exe`; o
  `window_created` do Pascal pôs o "0", três cliques pela automação do Windows levaram a "3", e
  fechar a janela encerrou o backend. As outras seis linguagens não foram publicadas nesta
  máquina. Teste `crates/aylis-project/tests/desktop_shell.rs`.
- [ ] Falta: o instalador (`--installer`) dessas linguagens. Os três montadores (`.msi`, NSIS,
  Inno) só levam arquivos soltos ao lado do `.exe`, e o backend fica numa pasta; hoje o
  publicar avisa e entrega a pasta.

**Ajustes de 17/11/2026: a versão com beta e o "falta: Assinatura de código"**

- [ ] Relatado pelo autor em 17/11: "o numero de versão ao publicar um app não aceita
  x.x.x-beta, no NoOBS eu uso esse formato". A tela das Opções do projeto só aceitava números,
  e a publicação desmontava a versão pelos pontos (o `.msi` e o código de versão do Android
  viam "0-beta" como 0, e o `version4` punha o número da beta como a quarta parte: a beta
  `1.2.0.1` passava a final `1.2.0.0`). Feito em 17/11 (falta o autor conferir publicando o
  NoOBS): `aylis_project::is_valid_version` aceita a pré-release e o build do versionamento
  semântico (`1.2.0-beta.1`, `2.0.0-rc.2`, `1.0.0+42`); onde o sistema só aceita números (o
  `.msi`, o iOS, os dados de versão do `.exe`), vai a parte de antes do `-`
  (`aylis_build::version_core`); o nome do arquivo e o `versionName` do Android levam a versão
  inteira. O `.msi` ganhou `AllowSameVersionUpgrades` (a final instala por cima da beta de
  mesmo número). O código de versão do Android sai igual para a beta e a final: para mandar as
  betas à loja, o `version_code` das Opções do Android. Testes em
  `crates/aylis-project/tests/version.rs`.
- [ ] Relatado pelo autor em 17/11: no "Configurar linguagens", todas ficavam "instalada; falta:
  Assinatura de código", e o Rust não terminava sem erro, mesmo com tudo instalado. A
  assinatura (0.4.0) é um aviso de algo só para publicar, mas o `setup` (o do instalador e o da
  linha de comando) e a lista do assistente contavam os avisos do desktop como falta, com
  exceção só do WiX: em qualquer máquina sem certificado, nenhuma linguagem ficava pronta, e o
  instalador acusava erro no fim. Não tinha a ver com instalar por cima de outra versão. Feito
  em 17/11: `setup::publishing_only` (o WiX e a assinatura) fica fora do que conta para a
  linguagem ficar pronta, nos três lugares; conferido nesta máquina (`aylis-tool setup rust`:
  de "não ficou pronta" para "pronta"). Teste
  `publishing_only_items_do_not_keep_a_language_from_being_ready`. O `aylis-tool check` ganhou
  `--language <nome>`.
- [ ] Relato do autor em 17/11: "o 'fraGeneral' pesquisado tem no projeto e não aparece na tela
  do Ctrl+F12, e essa tela não permite eu estar focado na pesquisa e apertar seta pra baixo".
  O Abrir fonte filtrava só o caminho, e o fonte de um form leva o nome do arquivo dele
  (`forms/settings/general.json` → `settings/general.rs`), não o nome do form. Feito em 17/11:
  cada fonte de form mostra o form ao lado (`settings/general.rs — fraGeneral`), o filtro casa
  pelo caminho ou pelo form, sem diferença de maiúsculas nem de `_`, e ↑/↓/PageUp/PageDown no
  filtro andam na lista (Enter abre). Teste `the_filter_finds_the_source_by_the_form_name`.
  Conferir na IDE com o NoOBS.

**Ajustes de 16/11/2026: a roda do mouse no DataGrid dentro de uma ScrollBox**

- [ ] Pergunta do autor em 16/11 (uma grade dentro de uma `ScrollBox`: a página parava de rolar
  com o mouse sobre a grade) e pedido: "pode corrigir.. e garanta que a scroll horizontal
  funcione por padrão segurando o shift.. mais nada pode impedir de alguem programar para que
  funcione de formas diferentes". O `DataGrid` ficava com toda a roda, mesmo na última coluna,
  na última linha ou sem nada para rolar; as listas já deixavam seguir. Feito em 16/11 (falta o
  autor conferir num app): a grade só fica com a roda quando rola, e na ponta ela segue para o
  contêiner de fora. As colunas rolam com o Shift (ou o touchpad de lado); o app escolhe pela
  propriedade `horizontal_wheel` (`shift`, o padrão; `always`, a roda comum rola as colunas;
  `none`, só o código), rola pelo código com `scroll_left` e `top_row` (só em execução), recebe
  o `on_wheel` antes da grade e prende a roda com o `captures_wheel`. Testes em
  `crates/aylis-dbcontrols/tests/grid_wheel.rs` (o primeiro falha com o código de antes);
  rótulos nos 12 idiomas, catálogo e documentação refeitos.

**Ajustes de 16/11/2026: o componente WindowSettings (a janela sem animação e o resto)**

- [ ] Pedido do NoOBS em 16/11: o editor do print (`full_screen`, `topmost`, sem barra de
  tarefas, `span_monitors`) aparecia com a animação do Windows abrindo e maximizando, e o app não
  conseguia evitar (a janela ficava visível antes do `on_window_created`). E pedido do autor no
  mesmo dia: "fazer todas" as configurações de janela que só alguns sistemas têm, "um componente
  para controlar esse monte de coisas [...] ligar ele ao form e configurar nele.. ou configurar
  de forma global ou programaticamente". Feito em 16/11 (falta o autor conferir no NoOBS e numa
  janela de cada recurso):
  - o componente não visual `WindowSettings` (System; Windows, macOS e Linux): no form, vale para
    a janela dele; com `scope` = `app`, para as dos forms sem um próprio (num DataModule ou no
    form principal); cada propriedade troca em execução (a janela recebe de novo,
    `WindowCommand::UpdateOptions`), também pelo código e pelo protocolo. Sem o componente, a
    janela fica como o sistema a cria;
  - as propriedades: `animation` (o `DWMWA_TRANSITIONS_FORCEDISABLED`, posto com a janela ainda
    escondida, e nos pedaços do `span_monitors`), `capture_excluded`, `click_through`,
    `minimize_button`, `maximize_button`, `backdrop` (Mica, Acrylic, abas), `title_color`,
    `title_text_color`, `border_color`, `shadow` (na janela sem borda), `blur`, `no_activate`
    (`WS_EX_NOACTIVATE`) e `alt_tab` (`WS_EX_TOOLWINDOW`); onde o sistema não tem o recurso, não
    faz nada;
  - no núcleo, `Ui::window_settings_of` e `Ui::window_options` (`aylis_core::WindowOptions`), e a
    plataforma aplica (`apply_window_options`). A `window_animation` que eu tinha posto no Form
    antes saiu (não chegou a commit): foi para o componente, com o resto;
  - testes `crates/aylis-controls/tests/window_settings.rs` e
    `the_window_animation_is_turned_off_before_the_window_shows`; os rótulos nos 12 idiomas, o
    catálogo, a documentação e os registros da classe. Não desliga sozinho nos forms em tela
    cheia (o que muda a aparência de forms existentes nasce desligado). No NoOBS:
    um `WindowSettings` no form do print com `animation` = `false`.

**Ajustes de 16/11/2026: a escolha das linguagens e das plataformas na IDE**

- [ ] Pedido do autor em 16/11 ("tem a primeira abertura do aylis depois da instalação fazer a
  configuração de quais linguagens, LSP, compilador e ferramentas vai precisar? para não deixar
  a seleção por conta do instalador do NSIS e centralizar em um só local?" — "Sim, centralizar").
  Feito em 16/11 (falta o autor conferir numa máquina nova, roteiro 129):
  - o instalador traz só a IDE e o Rust (obrigatório) e o atalho: saíram os grupos "Outras
    linguagens" e "Plataformas de destino" e o `/LINGUAGENS=` e o `/PLATAFORMAS=` do modo
    silencioso; fica o `/ESPELHO=`;
  - o assistente "Configurar linguagens" ganhou as plataformas de destino (Android e navegador,
    `aylis_tools::setup::TARGETS` e `target_missing`), com a situação de cada uma, e instala as
    marcadas pelo mesmo `setup_targets` que o instalador usava;
  - na primeira abertura da IDE nesta máquina, o assistente abre sozinho uma vez (marcado em
    `tools.json`, `setup_wizard_shown`), estejam as linguagens instaladas ou não; depois, pelo
    menu Ferramentas → Configurar linguagens.
  - Achado no caminho e corrigido: na tabela de traduções, eu tinha posto as colunas na ordem
    árabe, bengali, híndi (a ferramenta espera híndi, árabe, bengali), e os textos desses três
    idiomas foram para os arquivos trocados (nos textos do assistente e no "Sempre composta
    (fora do overlay)" da 0.5.0). Os mesmos sinais aparecem em textos de antes (desde o
    primeiro commit público): 17 rodados entre os três e 140 do bengali que estão em híndi;
    ficam para uma tarefa à parte.

**Ajustes de 16/11/2026: o maximizar da barra do Windows e o instalador com o MCP em uso**

- [ ] Achado em 16/11: na 0.5.0, o `rust-version` do workspace subiu para 1.92 (o krilla), mas o
  `MIN_RUST` do `aylis-tools` (o mínimo do "Verificar ambiente") ficou em 1.88; o teste
  `the_minimum_rust_is_the_workspace_one` pegou. Corrigido para 1.92. Na 0.5.0 publicada, um Rust
  entre 1.88 e 1.91 passa na verificação e falha ao compilar um app com relatório (o pacote de
  ferramentas instala o 1.99, então só afeta quem usa um Rust próprio mais velho).

- [ ] Relatado em 16/11 (um app do autor): nas janelas com a barra de título do Windows (as de
  configuração), o botão maximizar não fazia nada. O `win32.rs` engolia o clique no
  `HTMAXBUTTON` de toda janela, achando que era o da barra própria do app. Feito em 16/11 (falta
  o autor conferir no app): o clique só é da barra própria na janela que registrou o botão
  (`set_maximize_button`). Teste `the_maximize_click_is_only_taken_where_the_own_bar_has_the_button`.
- [ ] Pedido do autor em 16/11: o instalador não instalava com o Claude usando o MCP (a ponte é
  o `aylis-tool.exe` da instalação, preso). Feito em 16/11 (falta o autor conferir numa
  instalação por cima com uma conversa ligada): antes de copiar cada executável (a IDE, o
  `aylis-tool`, o `aylis-host`, o `aylis-migrate`), o instalador renomeia o que está lá para
  `<nome>.antigo.exe` (o Windows deixa renomear o que está rodando) e o marca para sair no
  próximo reinício; nenhum processo é encerrado, para não derrubar o MCP das conversas (o Claude
  não religa o servidor sozinho). Teste `a_binary_in_use_is_renamed_before_the_copy`.

**Ajustes de 15/11/2026: o cursor parado sobre o form em tela cheia (MPO)**

- [ ] Relatado pelo NoOBS em 15/11 (o editor do print em tela cheia, placa AMD com MPO: o cursor
  congela; com o MPO desligado, não). Feito em 15 e 16/11 (falta o autor conferir o cursor no
  NoOBS com `"present": "composed"` no form do print):
  - o modo `composed` (`aylis_core::gpu::PresentMode::Composed`, `present_for`): no projeto
    (`"present"` do `.aylisproj`, e o terceiro item do combo "Apresentação (Windows)" das Opções
    do projeto, traduzido) e por form (a propriedade `present`, vazia: o do projeto), trocável
    com a janela aberta (recria o alvo, `HwndTarget::set_present`);
  - no Direct2D, a swap chain no modelo de cópia (`DXGI_SWAP_EFFECT_SEQUENTIAL`, um buffer): o
    MPO e o independent flip são só do modelo flip, e o DWM compõe sempre. Se a placa recusar,
    a swap chain da janela de sempre;
  - `HwndTarget::presentation()` diz como o quadro chegou, e o exemplo `present_modes` do
    `aylis-d2d` abre uma janela em tela cheia sem borda em cada modo. Medido nesta máquina (RX
    9070 XT) em 16/11, 150 quadros: `window`, overlay em todos (o MPO do relato);
    `composition`, composed em todos (o NoOBS relatou que nesse modo o cursor ainda travava);
    `composed`, o DXGI não informa (a estatística só existe no modelo flip).
  - Testes em `crates/aylis-d2d/tests/present.rs`; o catálogo, o rótulo e a documentação
    refeitos.

**Ajustes de 15/11/2026: o ícone da barra de título achatado**

- [ ] Relatado em 15/11: o ícone da barra de título (e da barra de tarefas) usava o PNG do ícone
  do app como está, e um de 1024 × 768 com margem transparente saía achatado (4:3 num quadrado)
  e minúsculo em 16 px; o `.ico` já recortava o quadrado do meio. Feito em 15/11 (falta o autor
  conferir na janela de um app): o ícone da janela e o da bandeja passam pelo mesmo recorte
  (`window_icon::square` no `aylis-platform-winit`: o quadrado do meio, reduzido a no máximo
  256 px pela média de cada área, pesada pelo alfa). Testes no próprio módulo.

**Ajustes de 15/11/2026: o PDF pelo krilla**

- [ ] Decidido pelo autor em 15/11: o núcleo deixa de escrever PDF (o escritor próprio só tinha
  a Helvetica e o Latin-1: a fonte do relatório não aparecia, nem chinês, árabe ou emoji), e o
  PDF passa a ser de uma biblioteca conhecida, pronta como provedor: "Sim, com o krilla" (o
  gerador de PDF do Typst; MIT/Apache, sem `unsafe`). Feito em 15/11 (falta o autor conferir o
  PDF de um relatório de verdade e a impressão no Linux):
  - **Núcleo:** saíram o `PageSet::to_pdf`, o `PdfWriter` e o `BuiltinPdf`. O `PdfProvider`
    monta o documento página por página (`begin` → `PdfDocument::add_page` → `finish`), e o
    `Services::pdf` guarda o padrão. O `PrintService` ganhou `wants_pdf`/`print_pdf`: fora do
    Windows a impressão é o PDF do provedor aberto no visualizador.
  - **`aylis-providers`:** o `KrillaPdfProvider` (no lugar do `AylisPdfProvider`), atrás da
    feature `pdf`. As fontes vão embutidas em subconjunto: as do projeto (`aylis_core::fonts`) e
    as do sistema (pelo `fontdb`, fora da web); o caractere que a fonte não tem sai da primeira
    fonte de reserva que o tenha. O texto é medido pelo `skrifa` (o que o krilla já usa) para
    quebrar, alinhar (inclusive justificado) e pôr as reticências. Na web, só as fontes do
    projeto.
  - **Tamanho:** a fachada ganhou a feature `pdf`; o `aylis-build` liga quando o form tem
    `Report`, `ReportPreview` ou o `KrillaPdfProvider`. A feature `providers` sozinha (código
    de barras, exportação) não traz o krilla. Medido no exemplo `report` em release: 31,97 →
    35,51 MB (+3,4 MB: o krilla, o `fontdb`, o `rustybuzz` e os leitores de PNG, JPEG, GIF e
    WebP que o módulo de imagens dele pede), só no app que tem relatório. O Rust mínimo do
    krilla é 1.92 (o do Aylis declara 1.88).
  - **Memória do relatório enorme:** o escritor próprio gravava cada página direto no arquivo;
    o krilla monta o documento inteiro na memória (já serializado e comprimido) e grava no fim.
    O teste de memória do `write_pdf` passou a medir só o lado do Aylis (com um provedor de
    teste que não guarda nada); a memória do krilla cresce com as páginas.
  - Testes: `the_text_is_drawn_with_the_font_embedded` (`aylis-providers/tests/pdf.rs`: acento,
    €, ✓ e chinês desenhados de volta pelo `aylis-docview`), os do `report` e dos provedores.

**Ajustes de 15/11/2026: a pinça que sobe para os pais**

- [ ] Relatado pelo RTSPlayer em 15/11 e aprovado pelo autor ("pode corrigir"): a pinça na linha
  do tempo (`pnlRegua`, com `on_gesture`) dentro do palco (`pnlPalco`, também com `on_gesture`)
  ampliava os dois, porque o gesto sobe pelos pais e o handler do app não tinha como parar a
  subida. Feito em 15/11 (falta o autor conferir no celular): a propriedade `captures_gesture`
  no `ControlBase`, como o `captures_wheel` da roda: o controle que a tem fica com a pinça e com
  o deslizar (`on_swipe`), e os pais não recebem. Sem ida e volta ao backend (o `handled` como
  pergunta faria a tela esperar o backend a cada passo da pinça). Teste
  `captures_gesture_keeps_the_pinch_and_the_swipe_from_the_parents` (`swipe.rs`); rótulo nos 12
  idiomas; catálogo refeito.

**Ajustes de 15/11/2026: o MCP enxuto**

- [ ] Pedido do autor em 15/11 ("validar a documentação do MCP [...] não podemos desperdiçar
  token"), aprovado "Tudo". Feito em 15/11, medido pela ponte com o IBGE aberto (falta o autor
  conferir numa conversa de verdade):
  - **A lista de ferramentas** (vai em toda conversa): 60,7 → 42,5 mil caracteres. O argumento
    `project` repetia 189 caracteres em 80 ferramentas (32% da lista); o `max_wait`, a espera de
    45 s e a regra de não gravar direto se repetiam em várias descrições; o manual do
    `app_command` (3,3 mil) passou a vir pelo `{"cmd":"help"}` (`APP_COMMAND_HELP`). O teste
    `the_tool_list_stays_small` põe teto de 42 mil e diz as maiores quando passar.
  - **As instruções** (3,5 → 3,0 mil) passaram a explicar a pasta do projeto (`.aylisproj`,
    `forms/` e variantes, `themes/`, `assets/`, `locales/`, `<linguagem>/src` e `generated/`,
    `code-sync.json`, `.aylis/`) e a citar toda ferramenta por grupo (teste
    `the_instructions_name_every_tool`).
  - **As respostas** saem em JSON compacto (o indentado gastava ~40%): o `get_layout` do IBGE,
    36,2 → 15,3 mil. O `list_components` virou uma linha por componente, recuada pelo nível
    (13,2 → 3,7 mil), e o pai de quem está debaixo de um componente sem nome deixou de sair
    vazio. O `class_info` traz cada propriedade numa entrada `nome: "tipo"` e as comuns a todo
    controle só pelo nome, em `common` (`inherited: true` traz com o tipo): o Button, 11,2 → 2,0
    mil. Teste `the_component_tree_is_one_indented_line_per_component` (`mcp_tokens.rs`).
  - **Corrigido:** as instruções mandavam chamar o `session_title` primeiro, com o `project`, e
    a ponte recusava porque o projeto ainda não estava aberto. Agora o título espera o projeto
    abrir e vai para a IDE dele (`Route::Hold`). A primeira versão da correção mandava o título
    à IDE fixada da ponte e, como o Claude Desktop divide uma ponte entre conversas, o título da
    conversa do RTSPlayer apareceu na IDE do Reflex (visto em 15/11); conferido depois com a
    ponte aberta: o título pedido antes de abrir o IBGE apareceu só na IDE dele. A ponte que já
    estava rodando com a versão errada (aberta às 17:30) só sai quando o Claude Desktop a
    reiniciar.
  - O `AGENTS.md` que o "Novo projeto" grava deixou de repetir as ferramentas do MCP (as
    instruções do servidor já vão na conversa); ficou a regra de não gravar direto.

**Ajustes de 14/11/2026: o lint dos não visuais e a porta do canal da IDE**

- O `aylis-tool lint` cobrava conteúdo de exemplo também dos componentes não visuais, contra a
  regra de 14/11 (não visual fica sem exemplo na galeria). Agora a regra `class-without-sample`
  pula os não visuais (a classe que não cria controle, como no designer); 45 linhas saíram da
  base (`resources/lint-baseline.tsv`), nenhuma entrou.
- A porta do canal da IDE, quando a padrão está ocupada, chegava ao app pela variável
  `AYLIS_DEV_PORT`, lida também em release. Decidido pelo autor em 14/11: argumento de linha de
  comando e a variável removida. A IDE passa `--aylis-dev-port N` (no app em Rust, no fim; no
  `aylis-host`, antes do `--`, que o aceita e ignora); o app só lê com a feature `dev`. No Android
  continua o `adb reverse`. Um app em Rust que interpreta os próprios argumentos precisa ignorar
  esse (só aparece quando a porta padrão está ocupada).

**Pedido do NoOBS (14/11/2026): o editor de print como form do Aylis**

O editor de print do NoOBS (Print Screen → área → anotar → copiar/salvar) é hoje uma janela Win32
própria (GDI/GDI+); a ideia é refazê-lo como form do Aylis, com o texto das anotações no `Memo`.
Conferido no código em 14/11 (a ordem sugerida: 1 e 2, depois 3 e 4, 5 medido, 6 se der):

- [ ] 1. Um form sem borda, sempre na frente e sem botão na barra de tarefas, cobrindo o
  retângulo virtual de todos os monitores (inclusive com origem negativa), visível em até
  ~100 ms do `show()`, criado e destruído a cada print, com 1 pixel do form = 1 pixel físico
  mesmo com escalas diferentes por monitor. Procede: o `span_monitors` abre as janelas dos
  outros monitores na volta seguinte do laço e todas na escala do principal; não há modo de
  pixel físico. Espera o fim da orientação da tela (mexe na mesma plataforma).
  Feito em 14/11 (falta o autor conferir, roteiro 159; o caso real com dois monitores de escalas
  diferentes, em [testes em dispositivos](testes-em-dispositivos.md)):
  - a propriedade nova `physical_pixels` do `Form` ("Pixels físicos", Layout, desligada): 1
    unidade = 1 pixel físico em todas as janelas do form (nos pedaços também, seja qual for a
    escala de cada monitor), sem a escala do monitor nem o zoom (`Ui::form_zoom` dá 1); o clique
    no pixel (x, y) da tela chega como (x, y) do form (com `full_screen` e `span_monitors`, do
    retângulo virtual, inclusive com origem negativa), e a imagem do tamanho do form em pixels
    bate pixel a pixel. Alterável em execução (`WindowCommand::UpdateScale`): a janela fica onde
    está e o form passa a ter o tamanho dela na escala nova. Desligada, nada muda.
    `Ui::physical_pixels` e `aylis_core::span::pixel_scale` no núcleo;
  - abrir na hora: o form que já nasce em vários monitores (`span_monitors` com `full_screen`,
    ou o lugar de abrir passando de um monitor) abre todas as janelas no primeiro `show()` — a
    principal nasce no pedaço dela, sem moldura e escondida, as outras logo depois, e todas
    aparecem juntas (antes a principal aparecia num monitor só, com moldura, e virava pedaço
    depois);
  - no Direct2D, as janelas da mesma placa dividem o dispositivo do Direct3D 11 e do Direct2D
    (`D2dRenderer::shared_device`; criar um custava ~30 ms por janela), e a janela que fecha
    solta na hora a swap chain e as imagens dela (`Flush` e `Trim`);
  - o resto já havia: `border_style = none`, `topmost`, `show_in_taskbar = false` (as janelas
    dos outros monitores nunca aparecem na barra), `force_foreground`, `close_action = free`; o
    `free` agora também esquece os pedaços do form;
  - medido (`print_overlay`, release, `AYLIS_SPAN_SPLIT=2` num 3840 × 2160 a 150%, 20 ciclos,
    `medicoes/print_overlay.json`), do `show()` até a pintura de todo o form: com a imagem da
    tela, 291 → 76 ms de mediana (o primeiro ciclo, frio, 291 → 139 ms; antes no modo lógico com
    RGBA, depois em pixels físicos com o print em BGRA; com RGBA, 118 ms, a conversão da imagem
    nova); sem imagem, 166 → 71 ms. Fechar e liberar 20 vezes deixa as mesmas janelas (4 → 4) e
    a memória estável ciclo a ciclo (~200–250 MB depois do primeiro, antes ~320 MB);
  - testes: `in_physical_pixels_a_screen_pixel_is_the_same_pixel_of_the_virtual_rect` e
    `the_virtual_rect_with_a_monitor_above_and_to_the_left` (`aylis-core`, `span.rs`);
    `in_physical_pixels_each_piece_shows_the_print_pixel_by_pixel` (Skia e Direct2D, 0 pixels
    diferentes) e `the_print_form_opened_and_freed_twenty_times_leaves_nothing_behind`
    (`crates/aylis-controls/tests/span_monitors.rs`); os exemplos `print_overlay` e
    `span_monitors --auto` (liga e desliga o `physical_pixels` com o app aberto).
  - A receita do NoOBS (o form do print):
    `{"class": "Form", "border_style": "none", "window_state": "full_screen", "span_monitors": true,
    "physical_pixels": true, "topmost": true, "show_in_taskbar": false, "force_foreground": true,
    "key_preview": true, "close_action": "free", "children": [{"class": "Image", "align": "client",
    "fit": "stretch"}]}`, com o print no `Image` por `set_picture(Image::bgra(...))` (o tamanho
    do retângulo virtual em pixels).
- [ ] 2. `Memo` e `Edit` que crescem com o texto (`auto_width`/`auto_height`, várias linhas, sem
  quebra nem rolagem) e um modo "só texto" (sem borda, padding, rolagem nem fundo). Procede: não
  há `auto_size` neles. Espera o fim do menu de texto do Edit e do Memo.
  Feito em 14/11 (falta o autor conferir, roteiro 157), com o mecanismo do Label e do Button (o
  `auto_size` do `ControlBase` e o `auto_width`/`auto_height` do controle, no layout do núcleo):
  - `Memo` com `auto_size`: a largura é a linha mais larga mais o lugar do cursor e o padding, a
    altura é o número de linhas vezes a altura da linha; cresce para a direita e para baixo a
    partir do `left`/`top` (Shift+Enter quebra a linha) e encolhe ao apagar; `min_*`/`max_*`
    limitam (passou do máximo, rola); com `wrap`, a largura fica e só a altura acompanha. A
    largura de cada linha é guardada e só as linhas editadas são medidas de novo (o `splice` as
    marca); a mais larga só é procurada de novo quando a que era a mais larga mudou. Sem o
    `auto_size`, nada muda. O `large_text` em release, antes → depois: 8085 linhas, digitar
    0,25 → 0,11 ms de evento e 7,75 → 3,77 ms de quadro, 100 alterações 0,30 → 0,12 ms e
    7,52 → 2,85 ms; 1 milhão de linhas, digitar 27,3 → 24,8 ms e 5,95 → 5,76 ms, 100 alterações
    33,5 → 25,6 ms e 4,83 → 4,85 ms, desfazer 37,7 → 31,4 ms (sem piora; a diferença é ruído
    da máquina, com outras sessões compilando);
  - `Edit` com `auto_size`: a largura do texto (ou do placeholder), do ícone, do "×", do
    prefixo e do sufixo, mais o lugar do cursor e o padding; a altura, uma linha;
  - a propriedade nova `plain` ("Só o texto") no `Memo` e no `Edit`: sem o fundo do tema (um
    `fill` próprio ainda vale), sem borda, sem padding e (no Memo) sem as barras de rolagem; o
    recorte do texto não passa da caixa. A receita da anotação: `auto_size: true, plain: true`
    no `left`/`top` do clique (o exemplo `annotation` do `aylis-platform-winit`);
  - testes `an_auto_sized_memo_follows_the_text` e `an_auto_sized_edit_follows_the_text`
    (`crates/aylis-controls/tests/plain_text.rs`).
- [ ] 3. O `Memo` transparente sobre uma imagem: o fundo aparece, a seleção translúcida, o cursor
  visível em qualquer fundo, o mesmo antialiasing na tela e na exportação, com um teste de
  imagem. A conferir junto do item 2.
  Feito em 14/11 (falta o autor conferir, roteiro 157). O teste
  `a_plain_memo_over_a_picture_in_skia`/`_in_direct2d` (`tests/plain_text.rs`) pinta um Memo
  `plain` com `auto_size`, com texto selecionado, sobre uma imagem com uma metade escura e uma
  clara, e compara com `crates/aylis-controls/tests/visual/plain_memo-{skia,d2d}-windows.png`
  (a mesma tolerância do teste visual da IDE; `AYLIS_UPDATE_VISUAL=1` regrava). Confere também:
  o print intacto dentro da caixa onde não há letra; a seleção igual à mistura da cor do tema
  com o print; a tela (o quadro da janela) e a exportação com os mesmos pixels. O que achou e
  foi corrigido:
  - o cursor tinha a cor do texto e sumia no fundo escuro (10 tons de contraste): sem fundo
    (`plain` ou `fill` com menos de metade de opacidade), ele ganha um contorno por baixo na cor
    do tema (`background` ou `text`, a que mais se afasta da do cursor), também no bloco e no
    sublinhado;
  - os temas de seleção opaca (Windows 7, Win32, Windows 10) cobriam a imagem e pintavam o texto
    selecionado de branco (`selection_text`): sem fundo, a seleção fica com 40% e o texto na cor
    dele;
  - o antialiasing já era o mesmo (tons de cinza no Direct2D na janela e fora dela; o Skia pinta
    a tela e o PNG pelo mesmo caminho): o teste agora garante.
- [ ] 4. `Ui::render_rgba` de um retângulo do form, sem PNG no meio, com os mesmos pixels da
  tela. Procede (há `render_png`, `snapshot` e o `render_rgba` do modo sem janela, da tela toda).
  Feito em 14/11 (falta o autor conferir, roteiro 158):
  - `Ui::render_rgba(id, rect, scale) -> Option<(largura, altura, pixels)>`: o retângulo `rect`
    (nas coordenadas do controle) em RGBA, `ceil(rect * scale)` pixels, pelo desenho fora da tela
    da plataforma (`OffscreenService::render_rgba`, novo, no Skia e no Direct2D), sem PNG. Num
    form, é o caminho da janela quando repinta uma área (`paint_regions`: só o que toca o
    retângulo, com o fundo do tema e o que fica por cima); num controle de dentro, como o
    `snapshot` (fundo transparente). Pinta 16 pixels a mais em volta e corta: o Skia refaz a
    curva cortada pela borda da imagem, e a beirada mudava (9 pixels numa faixa de 1 pixel que
    cortava uma elipse, antes da moldura). Mesmo com ela, uma curva cortada na vertical ainda
    muda até 5 tons na beirada no Skia (ele subdivide o resto da curva de outro jeito; é o mesmo
    efeito da pintura por parte da janela no Skia); no Direct2D, igual pixel por pixel;
  - pelo nome: o método `render_rgba(x, y, width, height, scale)` do Form (no catálogo e nos 7
    clientes), atendido pelo `Ui::invoke` (dentro do `invoke` do form, ele sairia sem o próprio
    fundo), devolve os pixels em base64 (`aylis_core::image::encode_base64`). Pelo protocolo, a
    resposta passa do limite de 1 MB do `aylis-host` acima de ~440 × 440: recorte grande vai em
    faixas (ou pelo Rust, direto);
  - testes `a_rectangle_of_the_form_comes_out_like_the_same_piece_of_the_whole_in_skia` e
    `_in_direct2d` (`crates/aylis-controls/tests/render_rgba.rs`): um form com imagem de fundo,
    anotações num `on_paint` (seleção, traço, elipse) e um Memo; sete recortes (a seleção, o Memo
    cortado, um canto, faixas de 1 a 7 pixels que cortam as curvas), em escala 1 e 2, saem
    iguais pixel por pixel ao mesmo pedaço do form inteiro no Direct2D e, no Skia, com até 8
    tons na beirada das curvas cortadas; o `invoke` devolve os mesmos bytes.
- [ ] 5. O arrasto num form 4K com imagem de fundo abaixo de ~5 ms por quadro, repintando só o
  que muda. Em parte: o limite de um quadro por atualização do monitor já existe (`frame_rate`
  ou a taxa do monitor); falta medir o `on_paint` sem cache.
  Medido e otimizado em 14/11 (falta o autor conferir, roteiro 158). A medida:
  `cargo run --release -p aylis-platform-winit --example drag_4k --features d2d` (form 3840 ×
  2160, imagem de fundo RGBA 4K, um `on_paint` com a seleção que anda e um traço que cresce, e o
  lápis do `PaintCanvas` numa imagem 4K; Skia na CPU e Direct2D com a swap chain; confere a
  imagem por partes com a inteira). Números em `medicoes/drag_4k.json` (ms por quadro, mediana):
  - o que estava lento: o Skia copiava os pixels de uma imagem RGBA (33 MB no print) a cada
    pintura, mesmo de 0,01% da tela (~6 ms fixos); o `PaintCanvas` repintava o controle todo a
    cada movimento do mouse;
  - o que mudou: o Skia guarda a imagem em pixels já copiada (`aylis-skia`, `ImageCache::pixels`,
    até o `pixels_mb` do `image_cache`; sai quando o app solta a imagem, e o quadro de vídeo, que
    aparece uma vez, custa a mesma cópia de antes); o `PaintCanvas` avisa só o pedaço novo do
    traço e onde a seleção arrastada estava e ficou (`ctx.repaint_part`);
  - Skia na CPU, antes → depois: seleção por parte 6,94 → 0,74; traço por parte 5,65 → 0,07;
    `PaintCanvas` 45,5 → 0,11 (área 100% → 0,9%). Avisando o controle todo (`ui.damage`), a tela
    4K inteira continua ~33 ms no Skia na CPU: o editor tem de avisar por parte
    (`ui.damage_part`, como no exemplo: as faixas das bordas da seleção, o último pedaço do
    traço). Direct2D (o padrão no Windows): tudo abaixo de 1 ms do processo, antes e depois;
  - teste `drawing_and_dragging_a_selection_in_a_paint_canvas_repaint_only_what_changed`
    (`partial_paint.rs`): o traço e a seleção arrastada repintam só um pedaço, igual à pintura
    inteira.
- [ ] 6. Trazer o form para a frente quando outro app está à frente (o Windows recusa o
  `SetForegroundWindow` depois do atalho global; o NoOBS usa `AttachThreadInput`). Procede: não
  existe.
  Feito em 14/11 (falta o autor conferir, roteiro 158): a propriedade `force_foreground` do Form
  (rótulo "Forçar a frente ao mostrar"; padrão desligada). Ao abrir, ao `Show` e ao trazer para a
  frente, no Windows, a janela vem pelo caminho do NoOBS (`win32::force_foreground`: liga por um
  instante a entrada desta thread à da janela em primeiro plano com `AttachThreadInput`, depois
  `BringWindowToTop` + `SetForegroundWindow` + `SetFocus`, e desliga); sem ela, e nas outras
  plataformas, o `focus_window` do winit (que tenta com um Alt sintético, o que um hook de
  teclado engole). A decisão é do núcleo (`Ui::forces_foreground`; nunca no designer), com o
  teste `only_a_form_with_force_foreground_takes_the_forced_path` (`tests/render_rgba.rs`). O
  exemplo `foreground` (`aylis-platform-winit`) abre um form por um timer, 5 s depois, e diz se
  ele ficou em primeiro plano; aqui, sem ninguém usando a máquina, o Windows deixou ir para a
  frente com e sem a propriedade (passado o `ForegroundLockTimeout` sem entrada, ele libera), e
  o caso de verdade (outro app recebendo cliques) fica com o autor.

**Pedidos de 14/11/2026 (de outra sessão): o ponteiro do clique, a câmera no Direct2D e os executáveis marcados pelo antivírus**

- [ ] O antivírus corporativo (Cortex XDR da Palo Alto) e o SmartScreen marcavam como suspeitos
  os executáveis gerados pelo Aylis: o `.exe` do app só tinha o ícone, sem dados de versão, e nada
  era assinado. Feito em 14/11 (falta o autor conferir com um certificado de verdade; guia em
  [publicar-windows.md](publicar-windows.md)):
  - o `aylis-build` grava o `RT_VERSION` (VS_VERSIONINFO, StringFileInfo `041604B0` ou
    `040904B0`, VarFileInfo) no mesmo `.res` do ícone, escrito à mão como ele
    (`aylis_build::VersionInfo`, `add_version`, `read_version_data`), com a empresa
    (`publish.publisher`), o produto, a versão e o nome do `.exe` publicado; a IDE também leva os
    dados dela. Teste que liga um `.exe` com o `.res` e lê pelo Windows
    (`windows_reads_the_version_of_a_linked_exe`);
  - a publicação do Windows assina o `.exe` (antes do instalador) e o instalador (`.msi`, NSIS,
    Inno) com o `signtool` e carimbo de tempo SHA-256 (`aylis_tools::codesign`); o certificado
    vem das Opções da IDE → Assinatura (`tools.json`: a impressão digital, um `.pfx` com a senha
    só em `AYLIS_SIGN_PASSWORD`, ou o Azure Trusted Signing por `/dlib` e `/dmd`); sem
    certificado, um aviso e segue. No projeto, só o `publish.windows.skip_signing` (Opções do
    projeto → Windows); numa publicação, `--no-sign` no `aylis-tool publish` e `sign: false` no
    MCP. A verificação de ambiente ganhou o item "Assinatura de código";
  - as páginas da Landing regravadas com o guia novo.

- [ ] O `on_click` chegava sem o ponteiro (onde, com que botão e teclas). Feito em 14/11 (falta o
  autor conferir num app e num backend de outra linguagem), sem mudar o argumento dos handlers:
  `Ui::click_pointer()` (`ClickPointer`: a posição no controle e no form, o botão, as teclas, os
  cliques e se veio do mouse, do toque ou da caneta) vale durante o handler do `on_click`; `None`
  no clique do teclado e no do código. No protocolo, o evento leva `pointer` (`x`, `y`, `form_x`,
  `form_y`, `button`, `modifiers`, `kind`, `clicks`), e os 7 clientes o expõem (`ev.pointer`,
  `ev.Pointer`, `AylisPointer` no Pascal...). Testes `the_mouse_click_brings_the_pointer`,
  `the_keyboard_and_code_clicks_have_no_pointer` e `the_mouse_click_event_brings_the_pointer`.
- O `on_change` "atrasado": conferido em 14/11, não há atraso no código (a fila é entregue logo
  depois de cada tecla ou clique, e o `on_change` do Edit sai na hora pelo protocolo); só os
  controles de arrastar (`TrackBar`, `Slider`...) juntam os valores enquanto o backend trata o
  anterior, de propósito. Espera a outra sessão dizer o controle e o caso.
- [ ] O Direct2D criava um bitmap novo na placa a cada quadro de câmera ou de vídeo. Feito em
  14/11: o bitmap de um quadro já solto, do mesmo tamanho, recebe os pixels do novo
  (`CopyFromMemory`), só para quadros de até 1 MB (cerca de 640 × 400): medido
  (`medicoes/d2d_frames.json`, `crates/aylis-d2d/examples/camera_frames.rs`), 36 câmeras de
  320 × 180 caem de 7,3 para 1,7 ms por quadro e 16 de 480 × 270 de 2,5 para 1,6 ms; acima de
  1 MB copiar custava mais que criar (uma de 1920 × 1080: 0,9 contra 1,4 ms), então segue criando.
  O "um post por quadro" não procedia: a câmera e o vídeo já deixam no máximo um aviso pendente
  (`notify_frame`).
  Configurável (pedido do autor em 14/11: "essas configurações de vídeo e pintura sejam
  acessíveis para o programador poder testar no seu próprio ambiente"): o `frame_reuse_kb` do
  `image_cache` no projeto (Opções do projeto → Memória → Reaproveitar quadros; `0` desliga), o
  `AYLIS_FRAME_REUSE_KB` no desenvolvimento, e o `aylis-tool bench-frames`, que mede na placa da
  máquina com e sem e sugere o valor. O guia `docs/video-e-pintura.md` junta o que se configura
  de pintura e vídeo (projeto, form, componentes e variáveis) e como medir. A regra, confirmada
  pelo autor em 14/11: o projeto configura o app todo, o form configura o form, e a variável de
  ambiente só vale no desenvolvimento.
  Feito em 14/11 também (falta o autor conferir, roteiro 153), pelo pedido "em tempo de execução
  as configurações devem estar disponíveis para serem alteradas":
  - o componente `RenderSettings` (não visual, System, `aylis-controls-base`), com `pixels_mb`,
    `files`, `layers_mb`, `auto_layers` e `frame_reuse_kb` computadas sobre os valores do
    processo: ler diz o que vale, trocar vale no app inteiro no quadro seguinte (o
    `LayerCache::get` solta o excedente quando o limite baixa; `auto_layers` repinta). Fora da
    faixa, ajustado (como o `ImageCacheLimits::clamped`). Não gravadas no form (são de
    execução): a configuração de uma vez é a do projeto. No designer, a troca fica só no
    componente, sem mexer na IDE. Sem exemplo na galeria, como os outros não visuais; a página
    tem o texto de `docs/componentes/RenderSettings.md`;
  - a propriedade `frame_reuse_kb` do Form (`-1`, o padrão: o do projeto; `0`: sempre cria;
    acima de 32768, ajustado), gravada no form. A plataforma passa o valor do form ao alvo do
    Direct2D da janela a cada quadro (`Ui::form_frame_reuse_kb`,
    `HwndTarget::set_frame_reuse_kb`), e o d2d decide em três níveis
    (`image_cache::frame_reuse_limit`: o `AYLIS_FRAME_REUSE_KB` só em debug, o form, o app). No
    Skia e no navegador, sem efeito (não reaproveitam);
  - testes em `crates/aylis-controls/tests/render_settings.rs` e em
    `crates/aylis-core/tests/image_cache.rs`.
  - conferido com o `camera_frames` (36 câmeras de 320 × 180, 300 quadros, release), que ganhou
    o 6º argumento, o limite do form: o do app em `0` cria 10800 bitmaps (7,0 ms por quadro), em
    `1024` cria 72 (1,8 ms); o do form ganha do app (app `1024` e form `0`: 10800, 7,6 ms; app `0`
    e form `1024`: 72, 2,0 ms).
- O teste `aylis-ide --test mcp` falhava cerca de uma vez em três com os quatro casos em paralelo
  (`ConnectionReset`): corrigido em 20/11 (o servidor do MCP encerra a escrita e espera o cliente
  fechar, `aylis-remote/src/mcp/http.rs`).
- [ ] O menu de texto dos campos no celular (relatado por outra sessão em 14/11: "No Motorola
  Edge 30 Neo (Android 14), ao cadastrar um servidor, o toque longo sobre o texto de um Edit não
  abre o menu de texto que todo campo do Android tem (Recortar, Copiar, Colar, Selecionar tudo),
  nem seleciona a palavra. Não há como colar a senha ou o endereço vindos de outro app"). Feito em
  14/11 (falta o autor conferir no celular, roteiro 156):
  - no núcleo, o gancho `Component::context_menu(ctx, at, touch) -> bool` (padrão: o do `inner`,
    senão `false`). O `open_popup_menu` (botão direito) e o toque longo procuram primeiro o
    `popup_menu` do controle e dos pais (o do app vence, como antes) e, sem nenhum, chamam o
    gancho do controle tocado e dos pais. O toque longo que abre um menu é dono do soltar do dedo:
    o controle não ganha o foco (o foco fechava o menu, que abria e sumia num `Edit` com
    `popup_menu`) nem o item embaixo do dedo é escolhido (`Press::menu`);
  - no `Edit` e no `Memo` (e em quem os usa por dentro pelo `inner`: `NumberBox`, `DateEdit`,
    `TimeEdit`, `DateTimeEdit`, `AutoComplete`, `CodeEditor`, o `CellEditor` dos dados), o menu de texto
    (`crates/aylis-controls-base/src/text_menu.rs`): Recortar, Copiar, Colar e Selecionar tudo
    (`aylis_core::messages::t`, com inglês e espanhol; os textos já estavam nas 11 línguas da IDE).
    Recortar e Copiar desligados sem seleção; com `read_only`, só Copiar e Selecionar tudo; na
    senha, sem Copiar e Recortar. Cada item manda ao campo a tecla do atalho (Ctrl+X/C/V/A): o
    `on_change`, o `on_paste`, o desfazer, o `max_length`, os caracteres e a máscara são os do
    teclado. Sobre uma palavra errada (`spellcheck`), as correções vêm em cima, um divisor e os
    itens de texto (antes o menu das correções abria no apertar do botão direito e, com um
    `popup_menu`, abriam os dois);
  - a propriedade `text_menu` (padrão ligada, Comportamento) no `Edit` e no `Memo` desliga o menu;
    o rótulo nas 12 línguas em `properties.tsv` e o catálogo regravado;
  - o toque longo seleciona a palavra sob o dedo e o menu abre embaixo da linha; com o menu aberto
    (ele tem o foco), a seleção continua à vista e, ao escolher ou fechar, o foco volta ao campo com
    a mesma seleção (o `auto_select` não seleciona tudo de novo);
  - as alças da seleção feita pelo toque: uma bolinha com um traço embaixo de cada ponta, na cor
    `primary` do tema, desenhadas por cima do form (`PaintCtx::overlay`: saem da caixa do campo).
    Arrastar uma alça muda aquela ponta (sem cruzar a outra); durante o arrasto o
    `pan_on_touch` do campo fica desligado (não rola o form). O campo recebe o dedo nas alças pelo
    `hit_slop`/`contains_point`. Com o mouse, não há alças; o teclado, o clique e o digitar as
    tiram (Ctrl+A e Ctrl+C não);
  - testes em `crates/aylis-controls/tests/text_menu.rs` (toque longo, botão direito, cada item
    com a área de transferência falsa, `on_change`/`on_paste`/desfazer, `read_only`, senha, o
    `popup_menu` do app vencendo, `text_menu` desligado, arrastar as alças, o Memo e o NumberBox);
    o teste da ortografia (`tests/text.rs`) passou a soltar o botão direito.
  Ficou de fora: Colar fica sempre ligado com a área de transferência da plataforma (saber se há
  texto exige ler o texto: no Android 12+ isso mostra o aviso "colado da área de transferência" e
  no navegador pede permissão; sem plataforma, vale a cópia do processo); a alça única do cursor
  depois de um toque (opcional no pedido); a barra flutuante do Android (o menu é o `PopupMenu`
  do Aylis); o `RichEditor` não ganhou o menu. Um controle irmão desenhado depois do campo, embaixo
  dele, fica com o toque na área das alças. Com as alças à vista, o campo é pintado com o form
  inteiro (o `overlay` não tem área própria), só enquanto a seleção do toque existe.

**Pedidos de 14/11/2026: a barra de status, a tecla Voltar, os gestos e o fim da rolagem no celular**

- [ ] Um componente para mudar a cor da barra de status do Android e do iOS (pedido do autor em
  14/11: "preciso de um componente que consiga manipular a cor da barra de status do Android ou
  iOS"). O que existia e o que foi levantado:
  - Antes, no Android, a plataforma pintava sozinha a barra de status e a de navegação com o fundo do
    form principal, com ícones escuros sobre fundo claro (`sync_system_bars` no
    `aylis-platform-winit`, `setSystemBars` na `AylisActivity.java`); o app não escolhe.
  - Proposta: um componente não visual (`SystemBars`, família Device) com `status_bar_color` e
    `navigation_bar_color` (vazio = o fundo do form, como hoje), `icons` (`auto`, `light`,
    `dark`) e, talvez, esconder as barras (tela cheia). No Android, a `setSystemBars` que já
    existe; no iOS, o winit tem `set_preferred_status_bar_style` e
    `set_prefers_status_bar_hidden` (a cor no iOS é o que o app desenha por baixo da barra, então
    seria uma faixa pintada pelo núcleo); no navegador, a `<meta name="theme-color">`; no desktop,
    `available` falso.
  - Atenção: com `targetSdk` 35 (Android 15), o Android ignora a cor da barra (o app desenha por
    baixo dela); o padrão do Aylis hoje é 34.
  - O iOS precisa de Mac para conferir (`docs/testes-em-dispositivos.md`).
  - Desenho aprovado pelo autor em 14/11: o `SystemBars` com `status_bar_color`,
    `navigation_bar_color`, `icons` (`auto`, `light`, `dark`), `status_bar_visible` e
    `navigation_bar_visible`; Android pela `setSystemBars` (e o esconder), iOS pelo winit e a
    faixa pintada pelo núcleo, navegador pela `theme-color`, desktop `available` falso.
  - Feito em 14/11 (falta o autor conferir): o `SystemBars` (família Device; `CLASS_PLATFORMS`
    Android, iOS e navegador) com as cinco propriedades e o `available`; o pedido vale para o app
    todo (`Ui::set_system_bars`, `aylis_core::system_bars`). Android pela `setSystemBars` com o
    esconder; iOS pelo estilo da barra e a faixa pintada pelo núcleo (não compilado aqui: precisa
    de Mac); navegador pela `theme-color`. Testes em `crates/aylis-controls/tests/system_bars.rs`.

- [ ] A tecla Voltar do Android não chega ao app (relatado por outra sessão em 14/11, do
  RTSPlayer). Hoje a plataforma trata o "voltar" sozinha (`aylis-platform-winit/src/lib.rs`,
  `NamedKey::GoBack`): cancela o modal, fecha o form aberto com `show` (esse passa pelo
  `on_close_query`) e, no form principal, manda o app para segundo plano sem perguntar. Um app
  que troca de tela dentro de um form só (frames) não volta à tela anterior: no player do
  RTSPlayer, a tecla tira o app da frente em vez de voltar à lista de câmeras. Proposta: um
  evento `on_back` no form do topo, uma pergunta como o `on_close_query` (o app responde que
  tratou, e a plataforma não faz mais nada); sem handler, ou sem tratar, o comportamento de
  hoje. No iOS não dispara (não há "voltar" do sistema). Decidido pelo autor em 14/11: fazer
  o `on_back`. Feito em 14/11 (falta o autor conferir no celular): o evento `on_back` do form
  (uma pergunta, `handled`), que o form do topo recebe primeiro (`Ui::back_pressed`); sem
  tratar, o comportamento de antes. Também no Pascal (`var Handled`) e nos outros clientes pelo
  nome; `GENERATOR_VERSION` 12. Teste `the_back_key_asks_the_top_form_first`.
  Defeito achado pelo RTSPlayer em 14/11 e corrigido: voltar do player trocava a tela e também
  mandava o app para segundo plano. O handler do `on_back` chamava o próprio form por dentro
  (`with_form`, ocupado com o handler); o Aylis adiava essa chamada levando a pergunta junto, e a
  resposta `handled` do handler se perdia. Agora só o despacho do handler da pergunta pode levá-la
  (`Ui::claim_question`); a chamada de dentro só espera. Teste
  `a_call_to_the_same_form_inside_on_back_does_not_take_the_question` (falha com o código antigo).
  O contorno do RTSPlayer (responder na hora e trocar a tela por `ui.poster().post`) pode sair.

- [ ] Os gestos do celular que faltam (pergunta do autor em 14/11: "se ja tem componente que
  suporte os gestos dos smartphones"). Já existem: em todo controle o `on_long_press`, o
  `on_dbl_click` (toque duplo), o `on_gesture` (dois dedos: pinça, giro e arrastar) e o toque
  cru com cada dedo (`on_pointer_*`); rolar com inércia nas caixas, listas e grades; o
  `pull_to_refresh` do `ScrollBox`; deslizar no `Carousel`; a alça do `BottomSheet`; a pinça
  no mapa, no grafo, no visualizador de documentos e no `Model3D`. Faltam:
  - um evento `on_swipe` (esquerda, direita, cima, baixo) em todo controle, para o app não
    calcular pelo `on_pointer_down`/`on_pointer_up` (como o `Carousel` faz por dentro);
  - o `NavigationDrawer` abrir deslizando a partir da borda da tela;
  - o `TabControl` trocar de página deslizando o dedo;
  - deslizar um item de lista para revelar ações (como o "excluir" do e-mail);
  - a pinça para zoom no `Image`.
  Decidido pelo autor em 14/11 ("Seguir todas"): fazer os cinco. O `on_swipe` no núcleo, junto
  do `on_long_press`, com a direção e a velocidade (o `Carousel` passa a usá-lo; a rolagem e o
  deslizar não podem atrapalhar um ao outro); a gaveta pela borda respeitando a faixa do gesto
  de voltar do Android; o `TabControl` com `swipe`, desligado por padrão; as ações ao deslizar o
  item na `VirtualList` e no `DataRepeater`, com os botões do app; o `Image` com `zoomable`,
  desligado por padrão. Feito em 14/11 (falta o autor conferir no celular): o `on_swipe` em todo
  controle (direção e velocidade; não dispara se a rolagem andou); a gaveta abre pela borda (32
  pontos) e fecha deslizando; o `Carousel` troca pelo `swipe`; o `TabControl` com `swipe`; o
  `ImageView` com `zoomable` (pinça, roda e arrastar); a `VirtualList` com `swipe_actions` e o
  `on_swipe_action` (`{index, action}`). O `DataRepeater` ganhou as mesmas ações em 22/11 (herda o `VirtualList`). O toque passou a
  achar o controle com o deslocamento (`translate`) somado. Testes em
  `crates/aylis-controls/tests/swipe.rs` (6).

- [ ] O efeito de fim da rolagem (pergunta do autor em 14/11: "tem o efeito no smartphone quando
  chega no final ou no inicio que aparece uma barra meia lua indicando o fim?"). Não tem: no
  `ScrollBox`, nas listas e nas grades, a rolagem e a inércia param na ponta sem sinal. Cada
  sistema faz um: o Android até o 11, o brilho em meia-lua na borda (na cor do tema, cresce com
  o puxão e some devagar); o Android 12 em diante, o conteúdo estica e volta; o iOS, passa do
  fim e volta com mola; o desktop, nada. Proposta: o efeito no núcleo (vale para todo controle
  com rolagem), pela plataforma por padrão, com uma propriedade `overscroll` (`glow`, `stretch`,
  `bounce`, `none`) para forçar, na cor do tema. Decidido pelo autor em 14/11 ("Pela
  plataforma"): fazer assim (`overscroll: auto` é o padrão). Feito em 14/11 (falta o autor
  conferir no celular): `aylis_core::overscroll`, com a propriedade `overscroll` no `ScrollBox`,
  `ListBox`, `TreeView`, `Memo`, `VirtualList`, `RichEditor`, `ChatView` e `DataGrid`; a
  plataforma escolhe o padrão (Android 12 ou mais: esticar; antes: brilho na cor `primary` do
  tema; iOS: mola; desktop: nada); a animação só roda enquanto volta. Teste
  `the_end_of_scroll_effect_paints_by_parts_and_goes_back` (`partial_paint`).
  Defeito relatado pelo RTSPlayer em 15/11 (Motorola Edge 30 Neo, `stretch`) e corrigido: na
  grade de câmeras (um `DataRepeater` que só rola na vertical), o arrasto de lado esticava o
  conteúdo na horizontal, porque o puxão ia ao eixo em que o dedo mais andou sem saber se o
  contêiner rola nele. Agora o puxão e o impulso da inércia só valem nos eixos do
  `Component::scroll_axes` do contêiner (como no Android: a lista vertical estica só em cima e
  embaixo); quem não diz os eixos puxa nos dois. Ganharam `scroll_axes`: `ListBox`, `TreeView`,
  `ChatView` e `RichEditor` (vertical), `Memo` (vertical e, com a barra horizontal, de lado) e
  `DataGrid` (os dois); o `ScrollBox` e a `VirtualList` já tinham. Testes em
  `crates/aylis-controls/tests/overscroll_axes.rs` (falham sem a correção). Falta conferir no
  celular (o mesmo item do roteiro do efeito).

- [ ] A orientação da tela no celular (pergunta do autor em 14/11: "preciso saber se tem alguma
  configuração facilitada para controlar a rotação de tela do projeto e formulários quando
  estão no smartphone"). Havia só o `publish.android.orientation` (o manifesto do Android, fixo);
  nada no iOS, no form nem em execução. Aprovado pelo autor em 14/11: os três níveis, no Android
  e no iOS. Feito em 14/11 (falta o autor conferir no celular e no iPhone, roteiro 155):
  - **projeto:** o `orientation` do `.aylisproj` (`any`/vazio, `portrait`, `landscape`,
    `sensor_portrait`, `sensor_landscape`; `aylis_core::ScreenOrientation`), em Opções do
    projeto → Geral → *Orientação da tela*. Vai para o `android:screenOrientation` e para o
    `UISupportedInterfaceOrientations` (e `~ipad`, com `UIRequiresFullScreen` quando não são
    todas) do `Info.plist`, somado às orientações próprias dos forms. Substituiu o
    `publish.android.orientation`: um `.aylisproj` com a chave antiga é lido sem erro, mas ela é
    ignorada (o app volta a girar) e some no próximo salvar; refaça a escolha nas Opções (na
    0.x não há conversão);
  - **form:** a propriedade `orientation` do Form (`aylis_core::FormOrientation`, padrão
    `project`); o form à frente manda, e fechá-lo devolve a do de baixo ou a do projeto;
  - **em execução:** a propriedade do form gira na hora; o app todo pelo `set_orientation` do
    `Screen` (e `orientation()`), em todas as linguagens; em Rust, `Ui::set_app_orientation` e
    `Ui::requested_orientation`. O núcleo calcula o pedido (`aylis_core::orientation`) e a
    plataforma aplica quando muda: Android pelo `Activity.setRequestedOrientation` (JNI, sem
    mexer na `AylisActivity.java`), iOS pelo `set_valid_orientations` do winit (não compilado
    aqui: precisa de Mac); desktop e navegador sem efeito (no navegador o `screen.orientation.lock`
    só vale em tela cheia: ficou de fora);
  - **desenvolvimento:** `AYLIS_ORIENTATION` (ou `debug.aylis.orientation` no Android), pelo
    `diagnostic_setting`: só em debug, por cima do projeto e dos forms;
  - `GENERATOR_VERSION` 13, catálogo regravado. Ficou de fora o designer mostrar o aparelho
    deitado pelo form (o aparelho é do projeto e o designer grava a escolha no `device`).
  Testes: `crates/aylis-controls/tests/orientation.rs` (4) e, no `aylis-tools`,
  `the_project_orientation_goes_to_the_android_manifest` e
  `the_project_and_form_orientations_go_to_the_ios_plist`. Atenção no iOS: uma orientação pedida
  só pelo código, que nem o projeto nem um form declaram, não está no `Info.plist`, e o iOS
  recusa (pode encerrar o app); por isso o publicar soma as dos forms.

**Defeitos relatados por outras sessões (13/11/2026)**

- [ ] **Hífen no nome do pacote** (o RTSPlayer, item 24 do `relatorio-mcp-aylis.md`): o pacote
  `rtsplayer-app` publicava para Android e parava em "a biblioteca ...librtsplayer-app.so não
  foi gerada", porque o cargo grava `librtsplayer_app.so` (troca o `-` por `_` na biblioteca).
  Feito em 13/11 (roteiro 135): `Project::lib_name` (o `[lib] name`, ou o pacote com
  `_`), usado no Android (o `.so` e o `lib_name` do manifesto), no navegador (o `.wasm` e o que o
  wasm-bindgen gera) e no `main.rs` gerado; o executável continua o do `[[bin]]`, com o hífen.
  Testes `a_package_with_a_hyphen_has_the_library_with_an_underscore` (aylis-project) e
  `the_android_manifest_loads_the_library_with_an_underscore` (publish). O RTSPlayer pode voltar
  a `rtsplayer-app` (falta publicar para Android de verdade).
- [ ] **O motivo exato quando um pacote não vem** (relatado pelo autor em 13/11: numa
  instalação em outra máquina, a mensagem dizia "sem internet, um pacote que não confere com o
  SHA-256 do Aylis ou uma instalação recusada", sem dizer qual). Feito em 13/11
  (roteiro 136): o erro do `toolpacks::fetch::obtain` começa pela causa — "Sem internet" (com o
  espelho e o motivo: DNS, conexão, prazo), "Os arquivos não estão ao lado do instalador" (não há
  a pasta `pacotes` ou falta o arquivo nela, com o caminho), "não confere com o SHA-256 do
  Aylis" (onde) — e depois o que cada origem respondeu. O instalador passa sempre a pasta
  `pacotes` ao lado dele (`AYLIS_PACKAGE_DIR`, `Sources::installer_folder`), exista ou não; o
  `aylis-tool setup` sai com 16 + os bits da causa (`toolpacks::causes`) e o resumo traz o
  "Motivo:"; a caixa do fim do instalador diz a causa de cada linguagem. Testes
  `the_error_says_exactly_what_went_wrong` e `the_script_sets_up_without_a_shell`.
- [ ] **Tirar o "(fechado)" da árvore do projeto** (pedido do autor em 13/11): feito em 13/11
  (roteiro 135): em Projeto › Forms, só o nome (com ★, ● e ⛁ como antes).
- [ ] **O relatório do RTSPlayer de 13/11** (`RTSPlayer`, migração do cliente pelo MCP, 9 itens
  conferidos no código da 0.2.1-beta.1). Feitos em 13/11 a pedido do autor ("Sim, corrija"; sem
  commit; roteiro 138):
  - **1.** A `VirtualList`/`DataRepeater` com `align: client` criava as linhas da altura do
    projeto (8 de 13 numa tela de 900): o `viewport()` usa o tamanho que o layout deu (`bounds`,
    menos o `padding`).
  - **2.** O modelo desenhado com `align: top` empilhava as cópias pelo align (sem `sizes` nem
    rolagem): as cópias nascem com `align: none` e quem as põe no lugar é a lista; o modelo fica
    como foi desenhado. A outra metade do item (`set_sizer`/`refresh` não acharem a lista no
    `DataRepeater`) não procedia: o `get_mut` desce pelo `inner`; um teste agora confere.
  - **4.** `topmost` mudado com a janela aberta envia `WindowCommand::SetTopmost`.
  - **5 (parte).** As arquiteturas do APK escolhidas: `publish.android.abis` do projeto
    (`set_project_setting`) ou o `abis` do `publish` do MCP (`arm64-v8a` só, por exemplo), com a
    lista das aceitas no erro; `armeabi-v7a` entrou em `env::ANDROID_ALL_ABIS`, fora do padrão.
  - **5. Android de 32 bits** (14/11, a pedido do autor: "Sim, compile o Skia"; sem commit;
    roteiro 139). O `skia-binaries` 0.153.3 não tem `armv7-linux-androideabi` com as features do
    Aylis (404), então o Skia é compilado pelo Aylis uma vez e vira o pacote de ferramenta
    `skia-android-armv7` (17,6 MB, SHA-256 no manifesto, `"built": "aylis-tool skia-android"`
    no lugar da URL oficial: vem só do cache, da pasta de pacotes e dos espelhos). O publicar com
    `armeabi-v7a` instala o pacote (e o `rust-android-armv7`, com o Rust do Aylis; com o rustup,
    o erro diz `rustup target add`) e passa o `SKIA_BINARIES_URL` (`file://` para a pasta dele)
    só nessa compilação. A caixa "Também 32 bits" voltou nas Opções do Android, e o
    `aylis-tool publish` ganhou `--abis`.
    - `aylis-tool skia-android [--libclang pasta] [--ninja ninja.exe] [--work pasta]` refaz o
      pacote (`skia_android.rs`): clona o fork do rust-skia (`m153-0.101.2`, sem links
      simbólicos: o Windows sem modo de desenvolvedor não deixa criar), roda o `git-sync-deps`,
      compila em modo offline (`SKIA_SOURCE_DIR`) e monta o `skia-binaries-<chave>.tar.gz` com os
      mesmos arquivos do oficial; grava o hash no manifesto. Pede Python, Ninja (o do Visual
      Studio Build Tools é achado) e libclang (o do pacote `libclang` do PyPI). Os contornos: um
      `python3.bat` (o `python3` do Windows é o atalho da Microsoft Store) e o `-resource-dir`
      do clang do NDK no bindgen (`BINDGEN_EXTRA_CLANG_ARGS_armv7-linux-androideabi`).
    - Só as features do `aylis-platform-winit` padrão (`cpu` + `gpu`: `ganesh-gl-jpegd-jpege-
      pdf-textlayout`); com outras, a chave muda e o erro do publicar manda refazer. Subir o
      `skia-bindings` exige refazer o pacote: o teste `the_constants_follow_the_skia_bindings_in_use`
      falha até lá.
    - O arquivo (`%USERPROFILE%\.aylis\downloads\skia-binaries-...-armv7-linux-androideabi-...
      .tar.gz`) já está no espelho do autor (o Drive; ele disse em 15/11). Falta rodar num
      aparelho de 32 bits ([testes-em-dispositivos.md](testes-em-dispositivos.md)).
  - **8.** A ponte do MCP procura a IDE ao lado de si, depois na instalação para a qual o
    `<projeto>/.aylis/sdk` aponta e em `%LOCALAPPDATA%\Programs\aylis`; o erro diz onde procurou.
  - **9.** `max_wait` fica em no máximo 50 s (`MAX_WAIT`; antes aceitava 30 min, e a resposta se
    perdia no prazo do cliente); as descrições do MCP dizem o limite.
  - Testes: `an_aligned_list_uses_the_real_height_and_ignores_the_template_align`,
    `the_list_functions_work_on_a_data_repeater`, `changing_topmost_asks_the_window`,
    `android_abis_come_from_the_publish_then_the_project`,
    `the_ide_is_looked_for_beside_the_bridge_then_in_the_project_installation`,
    `max_wait_stays_within_the_client_deadline`.
  - **6** (hífen no pacote) é o item acima, de outra sessão.

**Relatório do Tabernáculo (11/11/2026)**

O console do Tabernáculo (51 forms, projeções em tela cheia) foi feito no Aylis 0.1.0, e o
relatório dele traz 209 itens (R01 a R04 com reprodução, mais P, J, C, W, L, T, G, I, M, B, O e
F). Conferido contra a versão atual em 11/11: 21 resolvidos, 57 parciais, 129 abertos e 2 que
não se aplicam mais. A análise item a item ficou com o autor (`ANALISE-versao-atual.md`, na
pasta do relatório). Pedido do autor: registrar os defeitos abertos e começar pela R01 e pela
R02.

Feitos em 11/11 (roteiros 105 e 106), conferidos com as reproduções do relatório
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

Lote 1, os defeitos (feito em 11/11; testes `aylis-controls/tests/tabernaculo.rs`,
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
- J01: os atalhos de um PopupMenu valem só com ele ligado ao controle com o foco ou a um pai.
- T08: o BGRA é pré-multiplicado nos três renderizadores (o contrato do tipo).
- M04: a ponte `aylis-tool mcp` sai quando o processo que a abriu some. M22: o `add_component`
  com `parent` põe nele, não num filho sob o canto.
- I01: a IDE pega o `CARGO_HOME` e o `RUSTUP_HOME` do usuário (registro). I02: a verificação
  mostra o erro do cargo. I03: com o rustup já instalado, o "Corrigir" instala o toolchain por
  ele. I04: a porta do canal ocupada, a IDE abre na próxima livre e avisa o app
  (o argumento `--aylis-dev-port`; no Android, o `adb reverse`).
- W01: o WebView só pega o foco ao nascer se for o primeiro da ordem de tabulação do form.
- W05 e W06: o VideoPlayer e o MediaPlayer abrem em segundo plano (`on_ready`, `on_error`); o
  `play` antes de abrir toca quando ficar pronto.
- B02: o `render_png` refaz o layout até assentar (o que os `on_resize` mudam entra na foto).
- R03: o evento `on_window_created` do Form, com a janela (e o HWND) já existindo.
- M13: os forms e fontes abertos guardam a marca do arquivo; ao voltar o foco e antes de cada
  chamada do MCP, o que mudou por fora recarrega (sem alteração na IDE) ou vira conflito: a IDE
  pergunta; o MCP recusa com o diff até o `resolve_conflict` (`keep`: disk ou ide).
- Não se aplicam: L03 (o encaixe é por ordem, de propósito: `Left` depois de `Top` fica com o que
  sobrou; o `MostLeft` pega a altura inteira) e W03 (o wry só cria o WebView2 na hora, na thread
  da janela; como o W02, é do WebView).

Lote 2, janelas e plataforma (feito em 12/11; teste
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

Lote 3, controles e tema (feito em 12/11; teste
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

Lote 4, layout e texto (feito em 12/11; teste `aylis-controls/tests/tabernaculo_layout.rs`):
- `gap` no Layout e no Panel (L05); o FlowLayout com `v_align`, `baseline` (L12) e o `grow` dos
  filhos (o espaçador, L04); a GridLayout com `grid_row`, `grid_column` e `row_span` (G12); o
  ScrollBox com `gutter` e a barra só com o que rolar (L01); o form com `auto_size`, desligado por
  padrão (L07); `ui.preferred_size` (L11).
- Texto: o serviço de texto do renderizador do projeto já na montagem dos forms
  (`install_text_service`, T01); `line_height`, `font_weight`, `font_size` e `italic` no estilo
  (T04, T05); no Label, `line_height`, `text_shadow` e os métodos `min_content_width` e
  `max_content_width` (T02); o traço por dentro documentado no `Canvas` (T09).

Lote 5, mídia e WebView (feito em 12/11; testes em `aylis-media/tests/media.rs` e
`aylis-webview/tests/environment.rs`):
- No fim do vídeo a posição fica no fim (W04); vídeo por endereço `http(s)`/`rtsp` no Media
  Foundation, no GStreamer e no AVFoundation (W11; o macOS e o Linux por conferir); o VideoPlayer
  como fonte de quadros (`show_video`, `on_frame`, `current_frame`) com o relógio e a fila como API
  (`clock_state`, `queued_frames`, `frames_ahead`; W12, W13); o WebView com `background_color`,
  `data_dir` e `browser_args`, sem a depuração remota (W07); a foto do `snapshot` com o WebView
  pela captura da janela (`WindowCaptureService`, no Windows; W10).

Lote 6, gerador e projeto (feito em 12/11; testes em `aylis-project/tests/project.rs`):
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

Lote 7, IDE e MCP (em parte, 12/11):
- O argumento desconhecido é recusado com os que valem (M06); `ide_list` com o pid e `ide_select`
  por ele (M03); o `set_property` com `margins.left` em texto (M09); `set_properties` (M15) e
  `move_component` (M16); o `read_code` pelo caminho relativo ao projeto, com qualquer barra e
  dizendo onde procurou (M24, M26); a descrição do `generate_code` diz que salva (I09); o aviso da
  fonte do sistema uma vez por projeto (I11); a árvore do projeto fechada, lembrando o que o
  usuário abriu (I10); o Design desligado num fonte sem form (I05); a foto sem o cursor de texto e
  sem o hover (B03).

Lote 7, o resto (feito em 12/11; roteiros 112 a 117):
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
  Java, C++ e Pascal (decidido em 14/11: fazer); a pasta no "Novo DataModule"; o `set_theme`
  chegando ao app em execução. Os três feitos em 14/11 (falta o autor conferir): o executor
  mínimo de Java (`clients/java/dev/aylis/Test.java`, ou o JUnit quando o projeto o usa), de C++
  (`clients/cpp/aylis_test.hpp`) e de Pascal (FPCUnit, `clients/pascal/aylistest.pas`), no `test`
  do MCP, com a Calculadora de exemplo e o guia `docs/testes-do-projeto.md` (teste
  `java_cpp_and_pascal_run_the_project_tests`); o "Novo DataModule" com nome e pasta; o
  `set_theme` chama o `push_reload` (a resposta diz se o app recebeu).
- B04: `aylis::headless::Headless` monta o `Ui` sem janela nem `platform::run` (as classes, o texto
  e o desenho fora da tela do renderizador do projeto; `install_offscreen_service`), abre o projeto
  ou um form, simula `click`, `type_text`, `press`, `advance` e tira a foto (`render_png`).
  Documentado em `docs/funcionamento.md`; teste `crates/aylis/tests/headless.rs` (Skia e Direct2D).

Mídia (feito em 12/11; testes `aylis-media/tests/texture.rs` e `http.rs`):
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

Controles, layout e projeto (feito em 12/11; testes
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

Cantos da janela (pedido de 12/11; roteiro 118): a propriedade `window_corners` do Form
(`default`, `round`, `small`, `square`). No Windows 11 o DWM desenha os cantos
(`DWMWA_WINDOW_CORNER_PREFERENCE`), com a borda suave e a sombra dele; o `default` com
`border_style = custom` arredonda como uma janela comum, e com `none` fica como o sistema deixa
(reto). Muda com o app rodando (`WindowCommand::UpdateCorners`). No Windows 10 o atributo não
existe e os cantos ficam retos. No macOS, a barra própria passa a ser a barra do sistema
transparente com o conteúdo por baixo (mantém os cantos e a sombra); escrito, não compilado nem
conferido num Mac (testes-em-dispositivos). No Linux, no celular e no navegador, sem efeito.
Conferido na tela com o Direct2D e o Skia na CPU (a captura por GDI não mostra o Skia em OpenGL).

O menu de layouts do Windows 11 na barra própria (pedido de 12/11; roteiro 118): a
TitleBar diz ao `Ui` onde fica o botão maximizar (`Ui::set_maximize_button`,
`Ui::maximize_button`), e a subclass da janela responde `HTMAXBUTTON` no `WM_NCHITTEST` sobre
ele: o Windows passa a tratá-lo como o botão da janela e, com o mouse parado, mostra o próprio
menu de layouts (Snap). O mouse ali chega como mensagens da moldura (`WM_NCMOUSEMOVE`,
`WM_NCLBUTTONDOWN`/`UP`, `WM_NCMOUSELEAVE`), que voltam ao `Ui` como mouse na barra: o realce e o
clique (maximizar e restaurar) continuam os da barra. Conferido na tela (Direct2D, 150%): o menu
aparece e os dois cliques maximizam e restauram. Os ganchos do app (`on_message`) vêm antes e
podem mudar isso. Fora do Windows, sem efeito.

O instalador e o desinstalador mais rápidos (pedido de 12/11: "os projetos de exemplo
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

Pacotes de ferramentas e a verificação de ambiente rápida (pedido de 12/11; o
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
  por último a URL oficial do manifesto (desde 20/11; `package_official = off` desliga). Pacotes no Windows: rust 1.99.0
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
- **Feito em 14/11** (decidido pelo autor em 14/11, "Seguir todas"; falta o autor conferir):
  - o pacote `fpc` (o Free Pascal 3.2.2 do Windows: `aylis-tool fpc-package` confere o
    instalador oficial, tira os arquivos com o innoextract sem executá-lo e monta o zip, que vem
    só das pastas de pacotes e dos espelhos, como o Skia de 32 bits; conferido: instalado, compila
    um programa de 64 bits); o pacote `wix` (o `.nupkg` oficial, que roda com `dotnet wix.dll`; a
    verificação de ambiente instala o pacote em vez do `dotnet tool install`); o `android-sdk` e o
    `dotnet` de Linux e de macOS ARM, e o `android-ndk` de Linux (o de macOS só sai em `.dmg`).
    Falta conferir os pacotes de Linux e macOS numa máquina deles (o `wix` e os de Linux e macOS
    vêm do fabricante desde 20/11; só o `fpc` e o Skia de 32 bits dependem do espelho do Aylis).
- **Falta** (decidido pelo autor em 14/11, "Seguir todas"):
  - não fazer: empacotar o Build Tools da Microsoft (a licença não permite redistribuir; continua
    o instalador da Microsoft), pacotes para pylsp, tsls, gopls, delve, csharp-ls e pasls
    (continuam pelos gerenciadores, que já vêm nos pacotes das linguagens), Python e NSIS no
    Linux e no macOS (não fazem falta lá) e o proxy por PAC/WPAD (continua o proxy das
    variáveis e do Windows);
  - manter: o "Configurar linguagens" cair no winget/rustup só quando não há origem de pacotes;
  - conferir: o JDK do macOS (testes-em-dispositivos), as páginas do instalador vistas na tela
    e o desinstalador numa máquina de teste.

Quadros de câmera e a repintura (relatado em 13/11 por um app com várias câmeras e duas janelas;
feito; teste `aylis-media/tests/repaint.rs`):
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
- **Escala de um componente com os filhos (`scale`)** (13/11; roteiro,
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
  - Silencioso: `/ESPELHO=` (no lugar do `/PACOTES=`; o `/LINGUAGENS=` e o `/PLATAFORMAS=`
    saíram com a escolha, em 16/11: veja "a escolha das linguagens na IDE"). A pasta `pacotes` ao lado do instalador vale pela variável
    `AYLIS_PACKAGE_DIR` (sem ficar gravada nas Opções).
  - O espelho padrão (a pasta do Drive) saiu em 20/11: sem espelho, os arquivos vêm do
    fabricante, e o Drive ficou como `settings::AYLIS_MIRROR`, só para o `fpc` e o Skia de 32 bits.
  - Os exemplos não vão: o `Cargo.toml` instalado sai sem nenhum `examples/` nos membros. As
    páginas de referência dos componentes (o `component_docs` do MCP) vão para
    `aylis\docs\referencia`, onde o MCP procura primeiro.
  - Testes em `installer.rs` (a página agrupada, o Rust sempre, sem shell, sem exemplos) e em
    `settings.rs` (o espelho padrão). O instalador foi gerado; não foi rodado nesta máquina (ele
    trocaria a instalação do autor).
- **ComboBox: quantas opções a lista mostra, a barra e o caber na tela** (13/11; roteiro, item
  130). Pergunta: "O combobox tem scroll para os itens com possibilidade de definir quantos itens
  deve exibir?". Rolava (roda, dedo, setas), mas eram 10 linhas fixas no código e sem barra.
  Agora:
  - `drop_down_count` (padrão 10, mínimo 1), propriedade do
    registro (JSON, Object Inspector, protocolo, clientes), com o rótulo traduzido;
  - com mais opções que as à vista, uma barra fina à direita da lista (`text_muted` do tema):
    o polegar se arrasta, um clique na calha pula para lá, e soltar não escolhe nem fecha;
  - a lista abre embaixo se couber, senão em cima; sem lugar para todas as linhas pedidas em
    nenhum dos lados, abre no lado com mais lugar e mostra as que couberem (`drop_place`).
  Testes em `crates/aylis-controls/tests/combo_drop_down.rs` (as linhas, a roda, a barra
  arrastada, o encolher e a barra pintada só quando rola). O catálogo e a página do ComboBox
  na documentação foram refeitos.
- **`app_command`: arrastar e as teclas seguradas** (13/11; roteiro, item 131). Pedido: "O
  app_command falta ter opção de arrastar de um componente ao outro, falta teclas de
  modificadores Shift, Ctrl". No protocolo remoto (`aylis-remote`, o mesmo do canal de
  desenvolvimento):
  - `click` com `modifiers` (`"Ctrl+Shift"` ou `["ctrl", "shift"]`), `x`/`y` (dentro do
    componente), `button` (`left`, `right`, `middle`) ou `double`: o mouse de verdade naquele
    ponto. Sem eles, o clique do componente, como antes;
  - `drag` (`from`, `to`, `from_x`/`from_y`/`to_x`/`to_y`, `modifiers`, `button`, `steps`):
    aperta em um, anda em passos e solta no outro (o arrastar e soltar do Aylis; `from` igual
    a `to` arrasta dentro de um componente);
  - `key` aceita `modifiers` somado às da própria tecla (o `"Ctrl+A"` já valia).
  A descrição do `app_command` no MCP lista os três. Testes em
  `crates/aylis-remote/tests/protocol.rs` (Ctrl+clique e Shift+clique na lista de seleção
  múltipla, o arrastar entre dois painéis, o Ctrl+A pelo `modifiers`). O `ui_click` do
  `McpServer` do app ganhou o mesmo em 14/11 (falta o autor conferir): o `ui_click` com
  `modifiers`, ponto, botão e duplo, o `ui_drag` e o `ui_key` com `modifiers`, repassados ao
  `execute` (teste `the_agent_clicks_with_modifiers_and_drags`).
- **Modo de mistura (`blend_mode`)** (13/11; roteiro, item 132). Pedido: "o Canvas não tem modo
  de mistura para permitir colocar uma marca d'água em negativo por cima das imagens dos vídeo
  sendo reproduzidos". Agora:
  - `aylis_core::BlendMode` com os 16 modos que os três renderizadores desenham igual (normal,
    multiply, screen, overlay, darken, lighten, color_dodge, color_burn, hard_light,
    soft_light, difference, exclusion, hue, saturation, color, luminosity) e
    `Canvas::save_layer_blend(opacity, mode, bounds)`, com padrão (sem suporte, a camada
    normal);
  - a propriedade `blend_mode` em todo controle (Appearance, com os filhos, junto com a
    `opacity`); com mistura, o controle não vira camada guardada (o desenho depende do fundo);
  - Skia: a camada com o `BlendMode`; Direct2D: a camada num alvo fora da tela e, no
    `restore`, o fundo daquele pedaço copiado e juntado pelo efeito Blend (com o efeito Opacity
    antes, se houver); navegador: o canvas da camada composto com o `globalCompositeOperation`.
  - A marca d'água em negativo: um texto ou imagem branco por cima do player com `blend_mode:
    "difference"`. O quadro novo repinta a área do player e a marca vem junto.
  Testes em `crates/aylis/tests/blend.rs` (o negativo sobre vermelho e preto, a opacidade, os 16
  modos iguais no Skia e no Direct2D com branco e com uma cor do meio, e a pintura por parte
  com a marca por cima). `Headless::render_rgba` novo (os pixels sem o PNG). `giant_screen`
  passou (relógio 5,0 ms). Falta: ver sobre um vídeo tocando numa janela; o navegador só
  compilado; uma marca dentro de uma camada guardada (`cache_layer` num pai) mistura com o
  fundo dessa camada, não com o da tela.

Ficam para depois:
- T06 (o provedor de PDF que embute as fontes): feito em 15/11 (veja "O PDF pelo krilla").
- A pintura por parte no navegador (se o Canvas 2D tem o mesmo efeito do Skia na borda).

**Área de transferência central (12/11/2026)**

Pedido do autor: "pode implementar e ajustar os lugares onde tiveram que fazer diretamente em
cada projeto, vamos centralizar isso para usar o do Aylis atendendo todos os casos". Feito,
falta o autor conferir (roteiro 119):
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
chamado por qualquer app chega no evento dele. Feito; falta o autor conferir
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
- Linux feito em 14/11 (decidido pelo autor: "Só o Linux"; falta conferir num Linux com
  área de trabalho): em execução, um `.desktop` do usuário com `xdg-mime default`; na
  publicação, o `.deb` declara os prefixos (`MimeType`, `%u`); o link chega à cópia aberta por um
  socket em `XDG_RUNTIME_DIR` (`url_scheme_linux.rs`; teste `the_deb_declares_the_url_schemes`).
- Falta: macOS e iOS (`CFBundleURLTypes` no `Info.plist`). O navegador
  fica sem (decidido em 14/11: o `registerProtocolHandler` só aceita `web+` e abre a página de
  novo; um app web recebe pelo endereço da página).

**WMI pela API nativa (12/11/2026)**

Pedido do autor: um componente que fale com o WMI do Windows pela API nativa (o `OleObject`
chega só à API de script, sem laço sobre o resultado nem eventos). Feito; falta o
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
- Como no `OleObject`: o `aylis-project` não compilava então por uma mudança de outra sessão
  (`ImageCacheLimits.layers_mb`), então o `aylis-tool update-catalog` rodou pelo exemplo do
  `aylis-remote` (o mesmo que ele chama), o que também trouxe as mudanças de outras sessões no
  catálogo (`OleObject` renomeado, `UrlScheme`, `Clipboard`, `Form.layer_cache`); a página da
  documentação (`componentes/WmiQuery.md` e o `indice.json`) foi escrita à mão, no formato do
  gerador, com o texto de `docs/componentes/WmiQuery.md`. Não rodaram a suíte inteira nem os
  testes da IDE (o das traduções dos rótulos); rodaram os do componente, do serviço, das
  mensagens, do catálogo e de `crates/aylis/tests/classes.rs`, mais o wasm e o Android. Depois,
  a suíte inteira e o `aylis-tool docs` rodaram nas releases de 12 e 13/11.
- Sem exemplo na galeria, como os outros não visuais (decidido pelo autor em 14/11).

**Automação OLE (12/11/2026)**

Pedido do autor: um componente para falar com o Excel e outros programas por OLE/ActiveX; a
automação "pode fazer" (embutir controle ActiveX no form ficou de fora, o porquê no histórico).
Feito; falta o autor conferir (roteiro 123):
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
  roteiro 123 numa máquina com o Excel. A tela espera enquanto o programa responde (sem versão
  assíncrona, decidido pelo autor em 14/11). O backend remoto continua podendo comandar o Excel da máquina (decidido:
  basta a lista de programas).
- Na mesma rodada, o `aylis-tool update-catalog` também gravou no catálogo o `Clipboard` e o
  `UrlScheme` (de outras sessões), que faltavam nele.
- A página da documentação (`componentes/OleObject.md` e a entrada no `indice.json`) foi
  ajustada à mão, no formato do gerador, com o texto de `docs/componentes/OleObject.md`: o
  `aylis-tool docs` não rodava de novo porque o `aylis-project` estava sem compilar por uma
  mudança de outra sessão (`ImageCacheLimits.layers_mb`). Pelo mesmo motivo não rodaram a suíte
  inteira nem os testes da IDE (o das traduções); rodaram os do componente, do serviço, do
  catálogo e de `crates/aylis/tests/classes.rs`, mais o wasm e o Android dos três crates.
  Depois, a suíte inteira e o `aylis-tool docs` rodaram nas releases de 12 e 13/11.

**A placa de vídeo de cada janela (11/11/2026)**

Pedido do autor: uma janela arrastada de um monitor da placa dedicada para um da integrada
trava (o Windows copia cada quadro entre as placas). Veio num patch escrito sem compilar;
analisado, aplicado e corrigido em 11/11 (roteiro 104). Tudo em
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
- Um form ocupando vários monitores (decidido pelo autor em 14/11: fazer). Feito em 14/11 no
  Windows (falta o autor conferir na máquina dos seis monitores): a propriedade `span_monitors` do
  form; o núcleo divide o form em pedaços (`aylis_core::span`), e a plataforma abre uma janela sem
  moldura por monitor, cada uma na placa do seu monitor, com a pintura, a entrada e os menus pelo
  núcleo. `AYLIS_SPAN_SPLIT=N` simula N monitores num só; o exemplo `span_monitors` (`--auto`,
  `--full`). Linux e macOS ficam numa janela só. Detalhes e o que ficou de fora em
  `docs/gpu-por-monitor.md`; testes em `crates/aylis-core/tests/span.rs` e
  `crates/aylis-controls/tests/span_monitors.rs`.

**Pedido de 11/11/2026**
- [ ] **Uma pasta de compilação para todo o Rust da máquina:** "armazenamento está sendo o
  maior problema do rust, e o aylis deveria corrigir isso" — "por padrão é pra tudo ir pra um
  só lugar", e quem quiser desmarca. Feito em 11/11 (roteiro 165):
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
    também compila na pasta única (hoje `%USERPROFILE%\.aylis\target`, desde a 0.2.0-beta.5). O `default_target_dir` não tem mais o
    caso "rodando do repositório"; o instalador, o `bench` e o teste `apps` acham os
    executáveis por `aylis_tools::cargo_target_dir` (o `cargo metadata`) ou pelo
    `CARGO_TARGET_TMPDIR`. A ponte do MCP do Claude Desktop (`claude_desktop_config.json`)
    aponta para `%USERPROFILE%\.aylis\target\debug\aylis-tool.exe` (conferido em 14/11).
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
    `dev-secrets`) e o `trusted.json`, que uma IDE aberta fora do Claude não vê (as ferramentas
    baixadas já ficam em `%USERPROFILE%\.aylis\tools`).
- [ ] **Compilar com o app aberto em outro lugar:** "O compilador do Aylis não aceita compilar
  enquanto qualquer NoOBS.exe estiver aberto, inclusive o instalado." O autor: não é para ser
  assim. O Aylis deve conferir se o executável que a compilação vai gerar está em uso e não pode
  ser substituído, e não se há um processo com aquele nome rodando: pode haver mais de um na
  máquina, e o que roda pode não ser o da pasta de saída. Feito em 11/11 (falta o autor
  confirmar): antes de compilar, `aylis_tools::run` confere os executáveis que a
  compilação grava (o do `target` e o da pasta de saída) abrindo-os para escrita; um em uso
  recusa (ou, com "Encerrar o app aberto", encerra só os processos daquele arquivo, pelo
  caminho). Outra cópia do app, de outra pasta, não atrapalha mais. O Parar sem nada rodando na
  IDE encerra o app do projeto pelo executável (o L-20 abaixo). Teste:
  `only_the_written_exe_counts_as_busy` (uma cópia do ping.exe rodando).

**Relatório do MCP na migração do InsightDB (11/11/2026)**

O agente que refez a página inicial do InsightDB (monitoramento de banco, de web para o Aylis) só
pelo MCP anotou as lacunas (L-01 a L-28). Pedido do autor em 11/11: "pode colocar e já começar a
fazer". Os números são os do relatório.

Feitos em 11/11 (falta o autor confirmar; roteiro 102):
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
- Decididos pelo autor em 11/11 e feitos (falta confirmar):
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
- A cor no markup do Label (L-15, a segunda parte) foi feita depois: `<color=...>` com token do
  tema ou `#rrggbb` (`aylis-controls-base/src/basic.rs`).
- **A explicação das propriedades** (pedido de 11/11: "o mcp consegue explicar as propriedades
  como funcionam, o negativo do radius por exemplo"; decidido: a explicação fica na documentação,
  tirada do comentário `///` do campo no código, mais um `docs/componentes/<Classe>.md` opcional, e o
  MCP a devolve sob demanda, só o essencial). Feito em 11/11: a página de cada
  componente ganhou a coluna "O que faz" (`reference::source_field_docs`, pelas camadas da classe e
  o `ControlBase`), o `.md` escrito à mão entra no fim da página, e a ferramenta `component_docs`
  (com `property`, só a linha dela e a seção `### propriedade` do texto à mão). Comentários novos e
  curtos nos campos do `ControlBase` (`align`, `anchors`, `margins`, `padding`, `left`...) e no
  `radius`. Ainda sem texto: as propriedades óbvias (`text`, `items`, `value`, `min`, `max`...) e
  as classes cujo tipo no código tem outro nome (o `Image` é o `ImageView`).
- **Renomear embutido no MediaCard** (pedido de 11/11, do NoOBS: "o card ganha uma propriedade
  como editable_title e cuida sozinho do campo de edição, dispara um on_rename com o nome novo, e o
  app só grava"). Feito em 11/11 (roteiro 103): `editable_title` (o clique no nome abre
  o campo; a miniatura continua no `on_click`), `on_rename` com o nome novo (já gravado no `title`;
  vazio ou igual não dispara), Esc cancela, e o método `rename()` abre pelo código (o F2). O campo
  é uma camada sobre o form que segue o card (como o calendário do DateEdit). No núcleo: a camada
  que fecha com o clique fora perde o foco antes de sair, para o campo gravar. Teste
  `aylis-controls/tests/media_card_rename.rs`; texto em `docs/componentes/MediaCard.md`.

As caixas abaixo descrevem as lacunas como foram relatadas; todas foram feitas (o resumo acima)
e só esperam o autor conferir.

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

Feitos todos em 10/11 (pedido do autor: "faça todos"; falta o autor confirmar;
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

As caixas abaixo descrevem o defeito como foi relatado; todos foram feitos (o resumo acima) e
só esperam o autor conferir (o 7 continua suspeita).

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
- [ ] **Compilador separado da IDE:** um executável que gera o app passando pelas validações
  (gerar o código, verificar o projeto, compilar), usado pela linha de comando e pela IDE; a IDE
  deixa de gerar e verificar dentro do próprio processo e passa a chamar esse executável e mostrar
  o progresso e as mensagens dele. Antes, a geração e a verificação rodavam numa thread da IDE
  (`jobs.rs`, `aylis_project::check`) e só o `cargo`/as ferramentas das linguagens rodam fora.
  Decidido pelo autor em 09/11: um subcomando do `aylis-tool`. Feito em 09/11 (falta o autor
  confirmar; roteiro 97): `aylis-tool build <projeto> [--target t] [--release]
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

**Arrastar**
- [ ] Abas que saem e entram em janelas, como as do navegador (pedido em 06/11): arrastar a
  aba para fora cria outra janela, arrastar uma janela de fora para dentro também, com a
  imagem de prévia junto do mouse. Feito em 06/11 (falta o autor confirmar) no
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
  Decidido pelo autor em 14/11 não fazer: o arrastar no `TabControl`, a janela acompanhar o
  mouse durante o arrasto e juntar as abas pela faixa vazia.

**IDE**
- [ ] Recuo do Object Inspector (pedido em 09/11: "as propriedades deveriam estar identadas à
  frente do agrupador, e o agrupador parece um espaço à frente do alinhamento lateral"). Feito em
  09/11 (falta o autor confirmar): o nome do grupo na borda do painel, alinhado com a
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
    no Linux, onde houver um portal). Windows feito em 14/11 (falta o autor conferir): o `share`
    abre a folha "Compartilhar" do sistema com o texto e os arquivos (`share_windows.rs`). Falta
    o macOS e o Linux.
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
- [ ] `Nfc` (pedido em 01/11; feito depois, falta conferir com um leitor e tags reais): componente não visual (categoria "Device",
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
- [ ] `Bluetooth` (pedido em 01/11; feito depois, falta conferir com um aparelho real): suporte completo num componente não visual
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
  qualquer propriedade mexida na vista vale só nela. Feito em 06/11 (falta o autor
  confirmar): ao entrar na vista, o designer guarda o base de toda propriedade gravada de cada
  componente com nome e do próprio form (menos a posição e o tamanho do form, que são da
  prévia); o que ficar diferente vira mudança da vista. Mudança no próprio form vai com o nome
  vazio (`"": { "padding": 6 }`), que o núcleo e o `aylis-json` entendem.
- [ ] Vistas personalizáveis (pedido em 06/11): hoje a IDE só tem Celular (até 599) e Tablet
  (até 1023). Criar, renomear, apagar e mudar o limite de cada vista pelo designer. Feito em
  06/11 (falta o autor confirmar): o seletor da barra mostra as vistas do form
  (Celular e Tablet sempre) e "Editar vistas…", que abre o editor (`viewsdialog.rs`): nome, largura e
  altura de/até, orientação, plataformas e as escalas; com desfazer (`Document::set_views`).
- [ ] A vista escolhida por mais que a largura (pedido em 06/11): altura, orientação
  (retrato/paisagem) e plataforma/tipo de aparelho, sem código. Feito em 06/11
  (falta o autor confirmar): `FormView` com `min_width`, `max_width`, `min_height`,
  `max_height`, `orientation` e `platforms` (windows, linux, macos, android, ios, web); vale
  uma vista por vez, a primeira da mais estreita para a mais larga cujas condições todas valem.
  O tipo de aparelho fica pela largura e pela plataforma (não há como saber "tablet" de outro
  jeito sem o sistema).
- [ ] Escala de fonte e de densidade por vista (pedido em 06/11): mudar o tamanho das
  fontes e os espaços do form inteiro numa vista, sem mexer componente a componente. Feito em
  06/11 (falta o autor confirmar): `font_scale` e `spacing_scale` na vista (0,25 a
  4). O layout carimba as escalas nos controles do form; a fonte da pintura e a medida
  (`font_of`) saem escaladas, e `margins`/`padding` e o `gap` do `GridLayout`/`FlowLayout`
  também; as propriedades continuam com os valores do desenho. O espaçamento interno de cada
  controle (o do tema: a altura de um botão, por exemplo) não muda.
- [ ] `GridLayout` com um filho ocupando mais de uma coluna (o `colspan`; pedido em 06/11).
  Feito em 06/11 (falta o autor confirmar): `column_span` em todo controle
  (categoria Layout); o que não cabe no resto da linha vai para a próxima, e mais que a grade
  tem ocupa a linha inteira (numa grade responsiva que encolheu).
- [ ] Ver por que a vista não muda um componente de contêiner (pedido em 06/11): se o motivo
  for forte, fica como está e o motivo fica escrito; se não, a vista passa a mover. Visto em
  06/11: não havia motivo registrado; a vista só foi modelada como "propriedade → valor", e o
  núcleo já move um componente dentro do mesmo form sem problema de nomes (`set_parent`).
  Feito (falta o autor confirmar): as pseudo-propriedades `parent` (nome do
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
- [ ] Assistente de novo projeto (pedido em 01/11; feito depois, `crates/aylis-ide/src/wizard.rs`; falta o autor conferir): no lugar do diálogo único
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
    é o padrão, `%USERPROFILE%\.aylis\target`, desde 12/11);
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
  `private`, a variável global `frmX` (a primeira instância) e o
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
    - `Post` para rodar código na thread dos handlers, por
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
    thread) e o guia em `docs/threads.md`. Os projetos já criados guardavam a própria cópia
    do cliente (`src/aylis.pas`); feito depois: o cliente vem da instalação, e a geração apaga a
    cópia antiga (`aylis_project::hostlang`, `remove_client_copy`).
  - 3.3 O que a tela mostra (decidido): coberto pelo 3.1 (cursor de espera, a pintura
    continua e, depois de uns 3 s, o indicador discreto; com o `BusyWatcher`, o declarado).
  - 3.4 Diálogo dentro do handler (decidido em 02/11, opção C). Nos clientes remotos,
    `MessageDlg` e `ShowModal` esperam o diálogo fechar (`if FrmCliente.ShowModal = mrOk`,
    `CanClose := MessageDlg(...) = mrYes`), e a versão com callback continua:
    - o protocolo ganha `"wait": true` no `message_dlg` e no `show_modal`: a resposta
      `{"id": .., "result": "yes"}` só sai quando o diálogo fecha (resposta adiada na tela);
    - enquanto espera, o cliente trata os eventos que chegam (os do modal e os `Post`);
      num `Call` comum, os eventos continuam na fila;
    - o pedido fica em "esperando o usuário" (3.1), sem prazo;
    - em Rust, só callback (a thread da tela não pode esperar, e o winit e o navegador não
      têm laço aninhado), explicado no guia;
    - o ProcessamentoAssincrono passa a usar o `ShowModal` que espera.
  - 3.5 `before_post` com backend remoto (decidido em 02/11). `before_post` e
    `before_delete` viram eventos do form, por nome, em todas as linguagens (hoje só closure
    em Rust); `before_insert`, `before_edit` e `before_scroll` só se precisar:
    - recusar: exceção no handler (`raise EAylis.Create('Informe o CPF')` em
      Pascal, `throw`/`raise` nas outras, `error` em Go) recusa, e a mensagem vai para a
      tela; terminar normalmente aceita; em Rust, o handler devolve `Result<(), String>`
      (`Err` recusa, o `?` repassa) e a IDE gera o esqueleto já com esse tipo;
    - a tela espera naturalmente: enquanto o `post` roda, a entrada fica
      bloqueada com o cursor de espera (3.2) e a tela continua pintando; o registro só
      grava (e dispara o `after_post`) com a resposta; recusado, continua em edição, com a
      mensagem e o foco no campo; os controles DB* respeitam o `post` pendente;
    - o código do backend que chama `Post` só segue depois de o `post` terminar:
      um evento-pergunta roda mesmo durante um `Call` (senão o `Post` do próprio
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
    - `key_preview` no form (recebe o `on_key_down` antes do controle com
      foco) e um evento de teclado da aplicação inteira (antes de todos): aplicação → form (`key_preview`) → controle → os pais;
    - o handler engole ou troca a tecla (o `Key := 0` e o `Key := #0`/`Key := '.'` em
      Pascal): em Rust na hora; nas linguagens remotas também (casos legítimos existem), com
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
    valor na hora e o foco fica no campo; o `field_error`
    continua para a validação que termina mais tarde). Mais o `ShowModal` e o `MessageDlg`
    que esperam (3.4), com a mesma resposta adiada:
    - recusar com mensagem é sempre exceção (`raise`, `throw`; `Err` em Rust, `error` em Go);
    - responder um valor é do jeito de cada linguagem: em Pascal, `var`
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
  encerra a sessão); decidido pelo autor em 14/11 não fazer.

  Revisto em 02/11 (o que outra sessão fez, com a rolagem do editor de
  código): já existe parte da etapa 2 no núcleo: `Ui::damage(id)` (o `ctx.repaint()` passou a
  marcar só a área do controle, com folga fixa de 24), `take_damage` e `paint_region`; o
  Skia na GPU guarda a imagem do quadro anterior e repinta só a área (o Skia na CPU, o
  Direct2D e a web ainda repintavam inteiro, feito logo abaixo); o Skia guarda os parágrafos de uma linha já
  montados (`LINES`); o `Memo` desliza a roda pelo `frame()` do núcleo e marca a área pelo
  `ctx.repaint()`, sem lógica de repintura própria (os testes `damage` e `memo_scroll`
  passam). Para a repintura não se duplicar entre o núcleo e os componentes:
  - o componente só diz o que mudou (`ctx.repaint()`, `Ui::damage`); decidir a área, guardar
    imagem, recortar e copiar é só do núcleo e do renderizador; nenhum componente guarda
    pixels nem faz a própria cópia de rolagem;
  - a camada de rolagem (etapa 4) precisa servir também quem rola por dentro (o `Memo`, o
    `DataGrid`, o `VirtualList`, as listas): uma API do núcleo em que o componente diz a
    área visível e o deslocamento, e o núcleo guarda e copia, em vez de cada um fazer a sua;
  - o cache de parágrafos do Skia viraria o do motor de texto do núcleo quando os modos
    `layout`/`core` chegassem; decidido pelo autor em 14/11 manter os dois (o do Skia serve ao
    modo `native`, o do núcleo aos outros);
  - a folga fixa de 24 troca pela extensão real dos efeitos (sombra, desfoque, anel de
    foco), como decidido;
  - o teste de pintar por parte × por inteiro e comparar as imagens foi feito depois
    (`aylis-controls/tests/partial_paint.rs`).

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

  Pela regra decidida (parar na etapa 2 se a medição mostrar que o layout por parte e a pintura por retângulo bastam), as
  etapas 3 (desenho gravado) e 4 (camadas, com a API de rolagem interna) ficaram esperando (feitas
  depois, a partir de 05/11, veja abaixo):
  o que elas resolveriam hoje é o mouse indo de um controle para o fundo do form (repinta
  tudo, porque o form muda de estado) e a rolagem do Memo (20% da janela, 1,4 ms). Voltam
  se uma medição num app de verdade mostrar quadro lento nesses casos.

  Decidido pelo autor em 05/11 ("pode fazer todas as melhorias para o desempenho"): fazer as
  etapas 3 (desenho gravado por controle, regravado só quando ele muda) e 4 (camadas na rolagem
  e nos controles animando posição ou opacidade, e o `cache_layer`), nos três renderizadores.

  Feito em 05/11 (falta o autor confirmar na janela de verdade):
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
- [ ] 9. (Feito em 02/11, antes do primeiro commit público; o aviso do `panic = "abort"` foi com a verificação do
  1b, os outros 6 clientes mandando o `exception` com os forms com instância, e o evento de
  exceção da aplicação é o `on_exception` do `ApplicationEvents` (item 3.6); falta o autor
  confirmar.) Um erro no handler não derruba o app: a tela mostra o erro e continua.
  Hoje, em Rust, um `panic` derruba o app (só a C ABI usa `catch_unwind`); nas linguagens
  remotas, a exceção só vai para o log, e o usuário não sabe que falhou; na web, o wasm para.
  Decidido em 02/11. Pronto quando:
  - em Rust, handlers e métodos de componente rodam protegidos no laço da plataforma
    (desktop e Android), e a proteção devolve à árvore o componente que estava fora
    (item 4);
  - o erro aparece no `MessageDialog`; em modo dev, também o caminho do handler e a pilha,
    no log, no `activity` e na gravação (`.aylisrec`);
  - há um evento de exceção da aplicação, no
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
  seguinte dele é adiado e nunca roda (veja os ajustes de 08/11, grave). Corrigido depois: o
  `catch_unwind` devolve a instância antes de repassar o pânico (`aylis-core/src/instances.rs`);
  falta o autor conferir de novo.

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
  - o "depois" (os modos de texto `native`/`layout`/`core` por plataforma), feito logo abaixo.

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
  faz à mão, num SQLite). Falta o que um provedor de gravação dá: transformar o delta em
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
  16.1 e 16.2 conferidos e confirmados pelo autor em 07/11; falta a 16.3.
  - 16.3: os guias `docs/gravar-alteracoes.md` (o delta em cada linguagem) e
    `docs/interceptador.md`; a documentação na página: `aylis-tool docs` (o exemplo `docs` do
    `aylis-ide`, `aylis_ide::reference`) copia os guias de `docs/` e gera uma página por componente
    registrado (paleta, descrição, propriedades com o nome do Object Inspector e o tipo, eventos e
    quais perguntam, um trecho de código), as comuns a todo controle e o índice, em
    `examples/Landing/assets/docs` (235 páginas); o form `frmDocumentacao` da Landing (botão
    "Documentação"): seções, páginas, busca em todas e os links entre páginas (teste
    `documentacao` da Landing). Falta: exemplos de uso escritos para cada componente (hoje a página
    traz só o trecho gerado; adiados pelo autor em 05/11, "para outro momento", e de novo em
    14/11) e um teste que avise quando a documentação ficar velha (decidido em 14/11: fazer; ele
    regera as páginas e falha se o resultado mudou). Também decidido em 14/11: escrever o "O que
    faz" de todas as propriedades (1238 das 1810 linhas do site estavam vazias), com um teste que
    falha quando uma propriedade entra sem o comentário. Feitos em 14/11: o teste
    `crates/aylis-ide/tests/docs_fresh.rs` (as páginas iguais às do gerador, e toda propriedade
    com o "O que faz"), os ~760 comentários de campo que faltavam e as páginas regeradas.

  Aprovado pelo autor em 04/11, como sugerido: com dados parciais, ordenar, filtrar e
  localizar viram um evento ao backend, que recarrega o dataset; sem handler ligado, a tela
  avisa ("ordenando só os 500 carregados"); o total de registros pode vir do backend; limite
  de memória opcional.

  Feito em 07/11 (pedido do autor: "faça" os aprovados; falta o autor
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
  do `DataNavigator` (antes de cada ação, passando pelo interceptador); no protocolo, o
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
  renomeou certo (veja os ajustes de 08/11, grave). Corrigido e aprovado na noite de 08/11 (o
  cabeçalho do roteiro).

  Feito em 07/11 (falta o autor confirmar; `crate::rename`). No código, pelo
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

  Feito em 07/11 (falta o autor confirmar): "Onde é usado…" no menu de contexto do
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

  Feito em 07/11 (falta o autor confirmar):
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

  Feito em 07/11 (falta o autor confirmar; o subrelatório mestre-detalhe já
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

  Feito em 07/11 (falta o autor confirmar):
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
  Feito em 05/11 (falta o autor confirmar):
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
    (o estado de cada linha guardado) e desfazer pelas diferenças. Decidido pelo autor em 14/11
    ("Fazer o rope"): fazer, no `Memo` (o editor da IDE e o controle dos apps).
  Feito em 14/11 (falta o autor conferir), por etapas, com os testes a cada uma:
  - Um caminho só para mudar o texto (`Memo::splice`): digitar, apagar, colar, o `add_line`, o
    `apply_edits`, o desfazer e as operações de linha passam por ele, e ele acerta o resto só
    da linha editada em diante.
  - Desfazer pelas diferenças (`memo_code.rs`): cada passo guarda as trocas (onde, o que saiu,
    o que entrou) e o cursor de antes; digitações seguidas juntam numa troca só. Desfazer
    aplica as trocas ao contrário, e o cursor vai ao trecho desfeito como antes (o trecho sai
    das próprias trocas, sem comparar os textos). O texto trocado por fora (o campo direto)
    esquece o desfazer, como a propriedade `text` já fazia.
  - Tabela das linhas e linhas visuais: as quebras do trecho saem, as do novo entram e as de
    depois só andam (sem quebra automática nem bloco dobrado; com eles, refeitas como antes).
    A linha visual do cursor sai por busca binária. A linha mais larga mede só as editadas.
  - Realce retomado da linha editada (`memo_highlight.rs`, para a gramática TextMate e para o
    realce embutido): o estado do começo de cada linha fica guardado; uma edição troca só as
    linhas dela e o realce recomeça ali até o estado bater com o guardado. O `LineCache`
    comparava o texto inteiro a cada tecla e guardava uma cópia de cada linha; o realce
    embutido de um texto grande recomeçava do início a cada tecla (o resto ficava sem cor
    por vários quadros).
  - As posições pela tabela das linhas (antes andavam o texto desde o começo): a do servidor de
    linguagem (`index_of_position`, uma por erro), `position_of_index`, `go_to`,
    `go_to_line`, o caractere do lado do cursor (pares que fecham, `::`), Ctrl+seta e
    Ctrl+Backspace (copiavam o texto inteiro), o clique triplo e os marcadores (andam com a
    edição, sem comparar o texto).
  - Medido (`cargo run --release -p aylis-controls --example large_text <linhas>`, que agora
    mede uma tecla na linha do meio, 100 alterações separadas e o desfazer), 8 mil / 100 mil /
    1 milhão de linhas, por tecla (evento + quadro): antes 18 / 30 / 237 ms, depois 3,7 / 6,7
    / 27,6 ms. Memória a mais depois de 100 alterações: antes 80 MB / 981 MB / ~10 GB (1 GB em
    10), depois 44 KB nos três. Ctrl+Z: antes 1,4 / 18,9 / 203 ms por passo, depois 0,04 /
    2,5 / 23 ms. Primeiro quadro com 1 milhão: 136 → ~90 ms. O quadro ficou igual nos três
    tamanhos (3,6 a 4,8 ms).
  - O que ainda cresce com o tamanho (1 milhão de linhas, ~100 MB): o `on_change` leva o
    texto inteiro como valor (uma cópia por tecla, ~14 ms dos 23 do evento) e, na IDE, o
    handler copia de novo para o servidor de linguagem; mover o resto do `String` ao inserir
    custa ~3,5 ms e andar a tabela das linhas e as linhas visuais, ~1 a 3 ms.
  - O rope (texto em pedaços) não foi feito: o `text` é um campo público `String`, lido direto
    pela IDE (`codewindow.rs`, `tooldialogs.rs`) e pelos apps (`ui.get::<Memo>`), e vai para o
    JSON e o protocolo; com o texto em pedaços, ele teria de virar um método (`text()`), o que
    muda a API. E o ganho medido é pequeno perto do resto: os ~3,5 ms do `String` com 1
    milhão de linhas, contra ~14 ms da cópia do `on_change`. Sem dependência nova (o `ropey`
    não entrou).
  - Decidido pelo autor em 15/11: (a), o `on_change` sem a cópia quando ninguém escuta; o rope
    (trocar o campo `text` por um método) não se faz. Feito em 15/11: `Ui::listens(id, evento)`
    (o handler, um interceptador ou a ligação a dados) e o Memo só monta o texto do `on_change`
    quando alguém recebe. Teste `the_change_carries_the_text_only_when_someone_listens`
    (`memo_scroll`). Medido com 1 milhão de linhas (`large_text`, release), por tecla: 27,6 →
    19,5 ms (evento 11,5 + quadro 8,0); 100 alterações 16,5 ms cada; desfazer 23 → 9,8 ms por
    passo. Na IDE o editor de código tem handler (manda o texto ao servidor de linguagem), então
    lá a cópia continua.
- [ ] 27. Tabela da documentação com o texto da primeira coluna por cima da segunda (achado na
  conferência do roteiro em 05/11, na Landing publicada para a web): na página "Fontes e
  ícones offline", os caminhos `resources/fonts/...` e `resources/icons/...` passam da
  coluna "Recurso" e ficam por cima de "O que é". A coluna precisa quebrar o texto (ou ter a
  largura do conteúdo).
  Feito em 05/11 (falta o autor confirmar): no `aylis-markdown`, cada coluna tem
  de piso a palavra mais larga dela (medida com a fonte do trecho, a mono no código) e o que
  sobra é dividido pelo quanto cada uma ainda queria (antes todas encolhiam na mesma
  proporção); uma palavra mais larga que a linha quebra por caractere. Teste
  `table_cells_never_overlap_the_next_column`.
- [ ] 28. Sem comparações com outras ferramentas no site (pedido do autor em 05/11: "pode remover
  referencias ao Delphi"). Feito em 05/11: na página da Landing, o título
  ("montadas visualmente"), o cartão do Free Pascal ("Units, classes e eventos com var, no
  jeito do Pascal") e o "E os dados?"; nos guias que vão para a documentação do site
  (`docs/criar-componente.md`, `gravar-alteracoes.md`, `interceptador.md`, `threads.md`) e na
  cópia deles na Landing. Os comentários do código e os documentos internos continuavam com as
  comparações, e elas voltaram ao site pelos comentários dos campos (16 páginas em 14/11).
  Decidido pelo autor em 14/11: "Tirar qualquer referencia a Delphi/FMX/VCL de qualquer fonte
  ou documentação": o código (comentários e textos), os docs, o site e o CLAUDE.md (549
  ocorrências em 14/11), com um teste que barra a volta. Feito em 14/11 (falta o autor
  conferir): os comentários, os textos, os docs e o site reescritos sem as comparações, e o teste
  `crates/aylis/tests/no_delphi.rs` (com a lista do que pode citar o nome, como o histórico).
- [ ] 30. A documentação em tela cheia, no jeito do Docusaurus (pedido do autor em 05/11: "a
  documentação tem que abrir em tela cheia, seguindo um layout de navegação igual ao
  https://docusaurus.io/docs"). Feito em 05/11 (falta o autor confirmar):
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
  botão com contador; imagens geradas automaticamente. Feito em 05/11 (falta o
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
    (às 22:35) e a pasta perdeu os tutoriais e as imagens; regerei depois. Conferido em 14/11:
    a pasta tem a seção "Comece aqui" (`examples/Landing/assets/docs/indice.json`).
- [ ] 31. Redimensionar a janela pela borda (achado no NoOBS em 05/11: "os componentes são
  empurrados mesmo tendo espaço pra eles, só depois que eu solto o clique que eles se
  redimensionam"; depois, "deveria ter uma largura mínima onde parasse de tentar quebrar
  linha"). Feito em 05/11 (falta o autor confirmar):
  - Arrastando a borda no Windows, o laço do sistema não passa pelo `about_to_wait`, e os
    `set_timeout` do `on_resize` só rodavam ao soltar. Agora o quadro refaz o layout e roda
    os timers vencidos antes de pintar (`aylis-platform-winit`, `redraw`).
  - Propriedades novas em todo controle: `min_width`, `min_height` e `max_height` (com a
    `max_width` que já havia). O layout não deixa o controle
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
    pendente antes de responder (sem nada pendente,
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
  e vetores; sem os controles das outras famílias). Decidido em 06/11: um wasm por família (a categoria da paleta). Feito em
  06/11 (falta o autor confirmar): o crate `aylis-demos` (registra só as classes de
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
  cinza-claro do hover. Feito em 06/11 (falta o autor confirmar): `Color::lerp`
  mistura com alfa pré-multiplicado (entre cores opacas, igual a antes). Teste
  `mixing_from_transparent_does_not_go_through_black`.
- [ ] 37. Cada componente aparece direito na demonstração ao vivo (pedido do autor em 07/11:
  "alguns componentes não cabem dentro do container dele e outros não mostram nada"). Feito em
  07/11 (falta o autor confirmar): `aylis::samples` (conteúdo de exemplo por classe,
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
    categoria do Object Inspector mudaram, de outra sessão (falha igual sem a oclusão);
    resolvida depois (o `visual` 4/4, abaixo).
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
    the_handlers` (o tema `"dark"` × `"Aylis Dark"`); resolvida depois (o teste confere
    `"Aylis Dark"`).
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
  decisões para o autor.
  Decisões do autor em 08/11 (roteiro 69): (1) ordem: 2b primeiro (as ferramentas que faltam no
  MCP, com os buracos achados na conferência), depois 1a → 2a → 4b → 3a/3b → 4a → 1d → 1b/1c → o
  resto; (2) o agente na CI só comenta; (3) o guia do projeto é opção do Novo projeto marcada por
  padrão, com `AGENTS.md`, um `CLAUDE.md` de uma linha apontando para ele e o `.mcp.json`; (4) na
  CI a cobertura reprova (cair mais de 1 ponto) e o tempo só avisa; (5) as métricas de desempenho
  no `Analytics` do app publicado só quando o app pede. Passar para a seção de decisões do
  `docs/proposta-llm-ci.md`.
  Feito em 14/11 (falta o autor conferir; os workflows só se confirmam no GitHub):
  - 2b: `profile`, `recording` e `coverage` no MCP da IDE (`mcp_insight.rs`);
  - 1a: `aylis-tool lint` (o núcleo sem sistema e sem `unsafe`, `unsafe` sem `SAFETY`, `cmd`,
    cor fixa, classe fora das listas ou sem exemplo, texto sem tradução), com a base
    `resources/lint-baseline.tsv`, que só pode diminuir (185 classes sem exemplo, 49 cores fixas,
    187 `unsafe` sem `SAFETY`), e o comentário `lint-allow: <regra>` para o que é de propósito;
  - 2a: a opção "Guia para agentes de IA" no Novo projeto (marcada), com `AGENTS.md`, `CLAUDE.md`
    e `.mcp.json`, e o `agent_guide` no `new_project` do MCP;
  - 4b: a cobertura de handlers em qualquer linguagem (o app avisa o primeiro disparo de cada um
    pelo canal de desenvolvimento; `.aylis/coverage.json`; Projeto → Cobertura de handlers…);
  - 3a: o Profiler salva e abre o rastro (`.json` do Chrome/Perfetto e `.aylisprof`); 3d: o clique
    no handler mostra o arquivo e a linha;
  - 3b: o roteiro com `"profile"` mede os handlers e pode reprovar por limite, e o `aylis-tool
    handler-times` compara com `medicoes/handler_times.json` (só avisa);
  - 4a: `aylis-tool coverage-check` com `medicoes/cobertura.json` (reprova ao cair mais de 1
    ponto) e o `cargo llvm-cov` na CI do Windows e do Linux;
  - 1d: `aylis-ide --headless --mcp-stdio`; 2c: `aylis-tool ide-script`; 1e: `aylis-tool
    recording-to-test`;
  - 1b/1c: os workflows `agent-review.yml` (o agente só comenta) e `failure-triage.yml` (precisam
    do segredo `ANTHROPIC_API_KEY`);
  - 3c: a propriedade `performance` do `Analytics` (desligada por padrão; nada vai para a rede).
  Falta: as bases de cobertura e de tempos (a primeira CI), o tempo do backend nas outras 7
  linguagens no 3b, o 4c (cobertura por linha no editor, por linguagem), o visualizador de
  gravações levando ao código e a marca de tempo no editor (3d).
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
    menu: o form vem do nome que está na linha de dados (são 1009);
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
  grave); aprovado na noite de 08/11, depois da correção (o cabeçalho do roteiro). O Relatórios
  procurava componente pelo nome; não procura mais (o teste `examples_typed_access`).
- [ ] 68. O MCP do Aylis ligado à sessão do Claude (pedidos do autor em 08/11: "caso o Aylis não
  esteja aberto o MCP deve abri-lo para a sessão do Claude que pediu, e o MCP deve relacionar a
  Sessão com a instância da IDE aberta"; "no Aylis deve mostrar o título da sessão do Claude que
  está conectada nele"; "o Claude deve conseguir pelo MCP pedir para compilar, e com isso a
  sessão tem que ficar aguardando o Aylis compilar e o Aylis deve avisar quando terminar"). Feito
  em 08/11 (falta o autor confirmar):
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
  plugin (item 69). Feito em 08/11 (falta o autor confirmar): o `calamine` subiu de
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
  terminar, e gerar o instalador (`installer-ide`). Feito depois: a suíte inteira e o instalador
  nas releases de 09 a 13/11.
  Conferido em 08/11 (roteiro 96): o texto com o `harfrust` igual ao de antes (acentos, negrito,
  emoji, o código no editor) e `cargo test -p aylis-ide --test visual` 4/4. Os testes do
  `aylis-tools` (`mcp`, `apps` inteiro nas 8 linguagens) e o `--test mcp` do `aylis-ide` também
  passaram em 08/11.
- [ ] 71. Duas sessões do Claude no mesmo projeto (pergunta do autor em 08/11: "se tiver duas
  sessões abertas e o usuário pedir para fazer alteração no mesmo projeto, oque acontece?";
  escolha: "Na mesma IDE"). Antes, cada sessão ficava numa IDE, e as duas podiam abrir o mesmo
  projeto (uma gravava por cima da outra); numa IDE aberta pelo usuário, as duas caíam juntas e
  a barra mostrava só uma. Feito em 08/11 (falta o autor confirmar):
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
  aproximadamente 2mb cada"). Feito em 11/11 (falta o autor confirmar no celular;
  roteiro 164):
  - **Rolagem:** o `ScrollBox` refazia o layout do form inteiro a cada passo (na web, ~300
    `measureText` por quadro, cada texto umas 4 vezes; 12,6 ms de mediana e 38 ms no p90 por
    quadro num PC, em tamanho de celular). Agora `Ui::scroll_content` só anda os filhos diretos
    (com layout pendente, faz o layout como antes); no nativo, 0,074 → 0,003
    ms por passo (`scrolling_on_the_phone_does_not_lay_out_the_page_again`, no teste da
    Landing; `scrolling_moves_the_children_without_a_new_layout` em `scroll_box.rs`). E o
    Canvas 2D guarda as larguras medidas por fonte e texto (zeradas quando uma fonte termina
    de carregar). Medido no ZenFone em 22/11 e trocado pela rolagem por cópia (veja o bloco de 22/11).
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
  manter o cursor, a seleção e a rolagem.
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
  form lembra o que está recolhido; o escolhido dentro de um recolhido abre os de cima. O ←/→
  do teclado veio logo depois (abaixo).
  Depois, também em 08/11: com o foco no painel (clicar numa linha), ↑/↓ andam, → abre (aberto, vai
  ao primeiro filho) e ← recolhe (recolhido ou sem filhos, vai ao pai); teste
  `the_structure_panel_follows_the_arrows`.
- [ ] Object Inspector (roteiro 74, autor): a busca de propriedades abre os grupos recolhidos
  enquanto há texto (hoje o achado fica escondido).
  Feito em 08/11 (falta o autor confirmar): o código já abre os grupos enquanto há busca
  (`searching`); conferir de novo na IDE recompilada.
- [ ] Designer pelo teclado (autor): Shift+seta muda o tamanho (1 px), Ctrl+seta move
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
  mensagens para o agente, textos de teste), acumulados de outras sessões. Decidido pelo autor
  em 14/11 ("Separar e traduzir"): o que não é de tela vai para uma lista de "não traduzir" (um
  arquivo, como o `properties.tsv`), o que sobra é traduzido pelo `--apply`, e um teste cobra a
  lista vazia. Feito em 14/11: `crates/aylis-ide/resources/locales/untranslated.txt` (o que não é
  de tela; o `aylis-tool translations` o pula), os textos de tela traduzidos e o teste
  `no_screen_text_of_the_ide_lacks_a_translation`.
- [ ] Profiler (roteiros 54 e 87, autor): a roda invertida (o zoom da linha do tempo e a lista de
  Métodos); mostrar a área repintada de cada quadro (hoje só o tempo, então o "a pintura do mouse no
  fundo quase some" não dá para conferir); e conferir se mover o mouse no fundo, sem trocar de botão,
  gera quadro (918 quadros em ~15 s de mouse no TestePintura).
  Feito em 08/11 (falta o autor confirmar): a roda corrigida nos dois lugares (para cima aproxima; a
  lista de Métodos sobe). Conferido no núcleo, com rastro: mover o mouse no fundo de um form não
  marca nada para repintar (só ao entrar no form); os quadros do relato vêm de outro lugar (falta
  achar com o app). A área repintada de cada quadro passou a aparecer no Profiler (abaixo).
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
- [ ] Os forms com "criar no início" abrem todos visíveis ao executar (roteiro 70): o certo é
  criar sem mostrar (só o principal aparece).
  Feito em 08/11 (falta o autor confirmar): não abriam: o `ui_forms` do app dizia `visible: true`
  pela propriedade de forms criados e não mostrados. Agora diz `open` e `visible` só para o aberto.

*Execução e projeto (pedidos do autor, roteiro 88)*
- [ ] Propriedade do projeto para encerrar o processo do app se ele já estiver aberto, antes de
  executar (o NoOBS da IDE terminava na hora com o instalado aberto).
  Feito em 08/11 (falta o autor confirmar): `output.close_running` (Opções do projeto → Geral,
  "Encerrar o app aberto antes de executar"): com o app já aberto (o instalado também), encerra
  antes de compilar; sem ela, o Executar e o Compilar param logo com a mensagem
  (`run::close_or_refuse_running`). (Mudou em 11/11: só conta o executável que a compilação grava;
  veja "Compilar com o app aberto em outro lugar".)
- [ ] Antes de compilar, conferir se o `.exe` de saída pode ser sobrescrito, e avisar logo (ou
  encerrar pela propriedade acima).
  Feito em 08/11 (falta o autor confirmar): o app aberto com o mesmo nome de executável para a
  compilação antes de começar, com a mensagem (ou é encerrado, com a propriedade acima). (Mudou
  em 11/11: só conta o executável que a compilação grava.)
- [ ] Comandos antes e depois da compilação (cmd e PowerShell). Decidido
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
- [ ] Mudar o valor de um controle pelo código dispara o `on_change`, como a mudança do usuário (roteiro 53;
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
  `change_by_code.rs`, `change_events.rs` e `bindings.rs`. O `item_index` da
  ComboBox pelo código também dispara (a lista do autor inclui o `item_index`).

*Componentes*
- [ ] Botão com Enter seguro repete o clique (roteiro 42, autor), com uma propriedade
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
- [ ] A ponte sem a IDE (feito nesta conferência; conferir no roteiro 92): a lista das
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
  `an_agent_fills_the_dialogs_and_reads_the_menus`.
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
  títulos do índice, da trilha, da busca e dos botões Anterior/Próxima.
- [ ] Faltam exemplos e documentação: o `on_server_query` (só o nome do evento na página do
  `ClientDataSet`), o `Barcode` numa banda de relatório, o `on_close_query` no
  `ProcessamentoAssincrono`, e as variantes "Chart · Sankey" e "Chart · Waffle" na galeria.
  Feito em 08/11 (falta o autor confirmar): o `on_server_query` explicado no guia "Gravar as
  alterações" e na tabela de eventos das páginas de componente (coluna "Quando"); o `Barcode` numa
  banda no exemplo do `ReportPreview`; o `on_close_query` no `ProcessamentoAssincrono` (o X cancela
  a thread, e o fim dela fecha), com roteiro de teste; as variantes "Chart · Sankey" e "Chart ·
  Waffle" na galeria. O `aylis-tool docs` rodou depois (releases de 12 e 13/11).
- [ ] O roteiro: tirar "o menu leva a seção ao topo" (os itens saíram a pedido do autor) e o "aos
  pacotes" do GradeGigante. (Feito em 08/11 na reescrita do roteiro: os dois itens saíram com os
  aprovados.)
  Feito em 08/11 (falta o autor confirmar): feito na reescrita do roteiro.

*Achados e feitos na conferência de 08/11 à noite (falta o autor conferir o que não foi
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
- [ ] A diretriz dos eventos no `CLAUDE.md` começa só com "Eventos:", sem a comparação de antes.
  Feito em 08/11 (falta o autor confirmar): feito no `CLAUDE.md`.
- [ ] A exceção dos comandos antes e depois da compilação (cmd e PowerShell só em projeto confiado,
  mostrados no "Confiar neste projeto?") na diretriz "Texto não vira código nem comando".
  Feito em 08/11 (falta o autor confirmar): feito no `CLAUDE.md`.
- [ ] 73. "Abrir como administrador" no instalador do Windows (pedido do autor em 14/11, pelo
  NoOBS: "o Aylis tem que ter essa opção nos projetos dele, mais por padrão do Aylis isso não
  viria ativado"). Feito em 14/11 (falta o autor confirmar):
  - `publish.windows.run_as_admin` (`on`/`off`/vazio, como o `autostart`; vazio é o padrão e não
    oferece). NSIS e Inno mostram a caixa, mas o instalador continua sem pedir administrador: só
    grava a escolha em `HKCU\Software\<app>\RunAsAdmin` (DWORD 1 ou 0), e o app lê e decide
    como elevar. Atualizando, a caixa vem como está no registro (sem o valor, o padrão do
    projeto). IDE: Projeto → Opções do projeto → Windows.
  - Com `close_running`, o NSIS confere depois do `taskkill`: o app que continua aberto (um app
    de administrador não aceita ser fechado por quem não é) faz o instalador avisar e parar,
    em vez de seguir e falhar ao gravar o executável.
  - Teste `run_as_admin_is_offered_and_saved_for_the_app` (`win_installer.rs`).
