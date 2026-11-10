# Carregamento e app enxuto

Como um projeto do Aylis vira executável, como ele é carregado e por que o app final só
leva os componentes que os forms usam.

## Do projeto ao executável

```
 Vendas/                         cargo build
 ├─ Vendas.aylisproj   ─┐
 ├─ forms/*.json       │  build.rs: aylis_build::project()
 ├─ themes/*.json      ├──►  OUT_DIR/project.aylispkg     (o projeto empacotado)
 ├─ assets/ locales/   │     OUT_DIR/aylis_classes.rs     (registro das classes usadas)
 │                    ─┘     OUT_DIR/icon.res            (ícone do .exe, no Windows)
 ├─ src/lib.rs   ── aylis::include_project!() embute os dois primeiros no executável
 ├─ src/main.rs  ── vendas::app()?.run()
 └─ Cargo.toml   ── aylis = { ..., default-features = false, features = [...] }
```

1. **`build.rs`** (gerado pela IDE) chama `aylis_build::project()`, que:
   - empacota o `.aylisproj`, `forms/`, `themes/`, `assets/` e `locales/` num arquivo só
     (`project.aylispkg`). O app publicado é um executável só, em qualquer plataforma;
   - **lê os forms e junta as classes usadas** (um subtipo, como `"Button.icon"`, conta
     como `Button`), mais as citadas pelo nome no código Rust do crate (`ui.create("MediaCard",
     ...)` num `src/**/*.rs`: todo `"Nome"` de uma classe conhecida), e gera
     `aylis_classes.rs`, uma função que registra só essas classes;
   - no Windows, põe o ícone no `.exe`.
2. **`aylis::include_project!()`** embute o pacote (`include_bytes!`) e a função de registro
   (`include!`) num `EmbeddedProject`.
3. **`Application::bundled(...)`** cria o `Ui`, chama a função de registro, aplica tema e
   idioma, liga os assets, cria os forms marcados como "criar no início" e abre o
   principal.

Para um form com `Label`, `Edit`, `CheckBox` e `Button`, o `aylis_classes.rs` gerado é:

```rust
// Gerado pelo aylis-build: registra só as classes usadas pelos forms do projeto.
{
fn aylis_register_used(ui: &mut ::aylis::core::Ui) {
    ::aylis::classes::Button(ui);
    ::aylis::classes::CheckBox(ui);
    ::aylis::classes::Edit(ui);
    ::aylis::classes::Form(ui);
    ::aylis::classes::Label(ui);
}
aylis_register_used
}
```

## Por que isso deixa o app menor

O registro de classes guarda, para cada nome, a função que cria o componente. Se o app
registrasse todas as classes, todas essas funções seriam citadas e o linker teria de
manter o código de todos os componentes: `Chart`, `DataGrid`, `NodeGraph`,
`VideoPlayer`...

Com o registro gerado, só os tipos usados são citados (`aylis::classes::Button` é uma
função que registra só o `Button`). O código dos outros componentes nunca é chamado
nem referenciado, e o linker o descarta. Um form com 4 tipos de componente leva só esses
4, mais os tipos que eles usam por dentro (o `ComboBox` usa uma lista no popup, o
`NumberBox` usa o `Edit`).

Um detalhe que faz diferença: o `EmbeddedProject` leva a função de registro como
`fn(&mut Ui)`, e não como `Option` com o `register_all` de reserva. **Só de citar o
`register_all` no caminho do app publicado, todos os componentes voltariam a entrar.**
As medições abaixo mostram esse efeito.

## Crates inteiros fora: as features

Além dos componentes, partes inteiras do Aylis são opcionais na fachada `aylis`:

| Feature | Traz | Quando a IDE liga |
|---|---|---|
| `data` | `aylis-data` (ClientDataSet, Field, DataSource) | algum form usa ClientDataSet, Field ou DataSource |
| `dbcontrols` | `aylis-dbcontrols` (DataGrid, DataNavigator, DBChart...) | algum form usa um controle de dados |
| `media` | `aylis-media` (MediaPlayer, VideoPlayer; decodificadores de áudio e vídeo) | algum form usa MediaPlayer ou VideoPlayer |
| `remote` | `aylis-remote` (inspetor do app em execução, app web remoto) | app web remoto; ao executar pela IDE vem com a `dev` |
| `dev` | o canal cifrado com a IDE (`aylis-devlink`), os forms da pasta do projeto, os diagnósticos | só ao executar pela IDE (feature `dev` do projeto); a publicação nunca |

O `Cargo.toml` gerado usa `default-features = false` e a lista de features que os forms
pedem. A cada "Gerar código", a IDE confere os forms e atualiza a lista. Um projeto sem
dados nem mídia nem compila esses crates (nem as dependências deles: symphonia, cpal...).

```toml
[dependencies]
aylis = { path = "...", default-features = false, features = ["data"] }

[features]
# Ligada pela IDE ao executar: o "Inspetor do app". Fica fora do app publicado.
inspect = ["aylis/remote"]
```

## Medições

Um projeto com um form de 4 tipos de componente (`Label`, `Edit`, `CheckBox`, `Button`),
compilado em release no Windows (x86_64, MSVC), em 25/10/2026:

| Variante | Executável | Diferença |
|---|---:|---:|
| Tudo: todas as features e todas as classes registradas | 22,09 MB | — |
| Só o registro enxuto (todas as features) | 21,42 MB | −0,67 MB |
| Só sem features (todas as classes registradas) | 19,51 MB | −2,58 MB |
| **Enxuto: as duas coisas, com Skia** | **19,02 MB** | **−3,07 MB (−13,9%)** |
| **Enxuto com Direct2D (o padrão dos projetos novos no Windows)** | **1,97 MB** | **−20,12 MB (−91%)** |

Com o Skia, o que sobra (~19 MB) é a base que todo app tem: o Skia (desenho na CPU e na
GPU), os dados do ICU para quebra de texto e o sistema de janelas. Com o Direct2D (que
já vem no Windows) essa base some: o mesmo app tem 1,97 MB. Veja
[Renderizadores no Windows](renderizadores.md).

(MB aqui é 1024 × 1024 bytes, como no Explorer do Windows.)

Com todas as features, o registro enxuto economiza menos porque a feature `remote` (o
inspetor) cita todos os componentes de propósito: ela precisa poder criar qualquer um.
Por isso o inspetor fica numa feature que só a IDE liga ao executar.

## Desenvolvimento e publicação

| | Executar pela IDE (F9, debug) | App publicado (release) |
|---|---|---|
| De onde vêm forms, temas e assets | da pasta do projeto (hot reload: salvou, aparece) | do pacote dentro do executável |
| Classes registradas | todas (`register_all`), porque um form pode ganhar componente novo sem recompilar | só as usadas (`aylis_classes.rs`) |
| Canal com a IDE (inspetor, hot reload, rastro) | ligado (feature `dev`) | fora |
| Console | saída na IDE | sem console no Windows |

### O canal de desenvolvimento

O app que a IDE compila para executar (desktop, Android, iOS, navegador; debug ou release)
leva a feature `dev`: ele **conecta na IDE**, no endereço embutido na compilação (Projeto →
Opções, "Endereço da IDE"; o padrão é `127.0.0.1:7342`). O app não abre porta nenhuma e não
lê variável de ambiente, parâmetro ou URL para isso. Sem a IDE, segue tentando em silêncio.

- **Cifrado e autenticado:** Noise `NKpsk0` (X25519, ChaCha20-Poly1305, BLAKE2s, crate
  `aylis-devlink`) por cima de um WebSocket comum. O app conhece a chave pública da IDE desta
  máquina (`%LOCALAPPDATA%\aylis\dev-key.json`) e o segredo do projeto
  (`dev-secrets.json`), os dois fora do projeto e do git; o `build.rs` embute o alvo que a
  IDE grava antes de compilar. Outro processo na porta não se passa pela IDE, e a IDE
  recusa quem não tem o segredo do projeto.
- **Android:** num endereço local, a IDE faz `adb reverse tcp:<porta> tcp:<porta>` (vale
  para o emulador e para o celular no cabo). Um celular na rede usa o IP da máquina no
  endereço do projeto.
- **Outras linguagens:** o `aylis-host --dev` lê o alvo numa linha JSON do stdin (a IDE
  escreve ao iniciar).
- Vários apps ao mesmo tempo: os comandos vão para o último que conectou; o hot reload, para
  todos.

Pelo canal passam:
- o **Inspetor do app** (árvore, propriedades ao vivo, destacar) e os comandos do MCP;
- o **hot reload** no celular e no navegador: ao salvar um form na IDE, ela manda o texto
  novo e o app aplica no form aberto (só o que mudou; o que foi digitado fica). Trocar o
  tema em Projeto → Opções do projeto → Aparência também vai. Um form com erro não derruba o app: a barra de
  status da IDE mostra o erro e o form continua como estava. No desktop o app lê os forms
  da pasta do projeto;
- o **rastro de eventos** (Executar → Rastro de eventos), que liga e desliga na hora.

O app publicado não tem nada disso (a publicação compila sem `dev`).

### Android: teclado, área de transferência e "voltar"

A publicação põe no APK uma Activity pequena em Java, a `dev.aylis.AylisActivity` (uma
`NativeActivity`; o fonte fica em `crates/aylis-tools/android/AylisActivity.java`, compilado
com o `javac` do JDK e o `d8` das build tools). Ela dá ao Aylis o teclado completo do
sistema: sugestões, correção, emoji, ditado. O teclado manda cada mudança como "apaga N
antes do cursor, M depois, insere este texto"; o Aylis aplica no controle com foco
(`crates/aylis-platform-winit/src/android_jni.rs`). O tipo do teclado segue o controle: senha
(`password`) sem sugestões, número (`rules` com `number`, `NumberBox`), e-mail (`rules` com
`email`).

- **Área de transferência:** o `ClipboardManager` do Android (`ui.services_mut().clipboard`),
  a mesma do resto do sistema.
- **"Voltar":** fecha o form de cima (um modal sai com `Cancel`, uma tela aberta com `show`
  fecha); no form principal o app vai para segundo plano e volta como estava, como os apps
  do Android 12 em diante.

- **Barras do sistema:** as barras de status e de navegação ficam com a cor de fundo do
  form principal e mudam junto com o tema; com fundo claro os ícones ficam escuros.
- **iOS:** o teclado virtual reduz a área segura (a faixa de baixo passa a ser a altura
  dele), como o `adjustResize` do Android: o form encolhe e o campo fica visível.
  Compilado aqui; o teste no simulador precisa de um Mac.

Sem a `AylisActivity` (um APK montado à mão com a `NativeActivity`), o Aylis usa o teclado
simples (só teclas).

Num projeto, o nome e o ícone vêm do `.aylisproj` (Projeto → Opções na IDE).

### Android: Activity recriada com o processo vivo

O app Android é uma `NativeActivity`: a cada Activity nova o sistema chama o
`android_main` de novo, às vezes no mesmo processo (a Activity foi destruída e o processo
ficou: "não manter atividades", uma abertura com `CLEAR_TASK`, o sistema liberando
memória). O winit só cria o laço de eventos uma vez por processo, então nesse caso o Aylis
pede ao Android a mesma Activity (com a mesma intenção) numa tarefa nova e encerra o
processo velho; o sistema sobe um processo novo e o app abre do zero, como numa abertura
normal. O que precisa sobreviver a isso fica salvo (arquivo, banco), como em qualquer app
Android cujo processo é morto.

Para reproduzir: `adb shell am start --activity-clear-task -n <pacote>/dev.aylis.AylisActivity`
com o app aberto.

## Classes do próprio app

Componentes que o app cria (veja [Criar um componente](criar-componente.md)) não são do
aylis: o `aylis_classes.rs` só os cita num comentário, e o app registra cada um no `app()`
(`ui.register_class::<StarRating>("Meus componentes")`), antes de carregar os forms.

## Projetos antigos

Projetos criados antes desta versão continuam funcionando, registrando só as classes
usadas. Para tirar também os crates não usados, troque no `Cargo.toml` a linha do Aylis por
`aylis = { path = "...", default-features = false, features = [] }` e acrescente
`[features] inspect = ["aylis/remote"]`. A partir daí a IDE mantém a lista atualizada.

## Para quem mexe no Aylis

- a lista de classes que podem aparecer num form é `aylis_build::CLASSES` (nome e
  feature);
- cada uma tem uma função de mesmo nome em `aylis::classes` (`crates/aylis/src/lib.rs`);
- o teste `crates/aylis/tests/classes.rs` confere que as duas listas batem com o
  `register_all` e que cada função registra a classe certa.

## Publicar para as lojas e instaladores

**Android, chave do desenvolvedor.** Sem configurar nada, o APK sai assinado com a chave
de depuração (instala, mas a Play Store recusa). Em Publicar → Chave (Android), "Criar…"
gera a chave (`.jks`, RSA 2048, 25 anos; o mesmo que `aylis-tool keystore chave.jks --alias
app --name "Empresa"` com a senha em `AYLIS_KEYSTORE_PASSWORD`). O projeto guarda o
arquivo e o apelido, nunca a senha: a IDE pergunta a cada publicação, e a linha de comando
lê `AYLIS_KEYSTORE_PASSWORD` (e `AYLIS_KEY_PASSWORD`, se a da chave for outra). Guarde o
arquivo e a senha: a Play Store exige a mesma chave em todas as atualizações.

**AAB (Play Store).** Marque "Android: AAB" (ou `aylis-tool publish ... --target android
--aab`): os recursos vão em protobuf (`aapt2 --proto-format`), o módulo `base` é montado
com o manifesto, o `classes.dex` e as bibliotecas de cada arquitetura, o `bundletool` do
Google gera o `.aab` e o `jarsigner` assina. O Verificar ambiente baixa o `bundletool`.
Conferido aqui: o `bundletool validate` aceita, e o APK universal gerado do AAB instala e
abre no emulador.

**Instalador do Windows (`.msi`).** Marque "Windows: instalador .msi" (ou `--msi`): o WiX
5 monta um MSI que instala em Arquivos de Programas, põe o atalho no menu Iniciar e
aparece em Aplicativos para desinstalar. O código de atualização é derivado do
identificador do projeto, então a versão nova substitui a anterior. O Verificar ambiente
instala o WiX (`dotnet tool`, precisa do .NET SDK). Fabricante: `publish.publisher` no
projeto.

**Outras linguagens no celular e no navegador.** O código de um projeto em Python,
TypeScript, C#, Java, Go, C++ ou Free Pascal não roda no celular nem no navegador; ele roda
num servidor. A publicação para Android ou web gera um app casca em Rust
(`.aylis/shell`) que leva os forms, o tema e os assets do projeto e liga a tela ao
endereço em Publicar → Servidor (`wss://servidor:9443`). No servidor:

```bash
aylis-host --bridge 0.0.0.0:9443 --project Loja.aylisproj --tls-cert cert.pem --tls-key chave.pem -- python src/main.py
```

A ponte aceita cada app que conecta e inicia um backend só dele, ligado pelo mesmo
stdin/stdout de sempre: o código do projeto não muda. No código Rust, o mesmo é
`aylis::connect_backend(&mut app.ui, url)`.
