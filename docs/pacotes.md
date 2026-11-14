# Pacotes de ferramentas

Decidido com o autor em 12/11/2026: instala-se a IDE e, depois, os pacotes que vão ser usados,
baixados conforme a necessidade ou tirados da pasta ao lado do instalador. Este documento é o
desenho que as partes seguem.

## O que é um pacote

Uma ferramenta que um projeto precisa e que o Aylis instala sozinho, sem administrador e sem ir
a site nenhum: o Rust, o JDK, o SDK e o NDK do Android, o Python, o Node, o .NET, o Go, o Free
Pascal, os depuradores, os servidores de linguagem, o `wasm-bindgen`, o `cargo-ndk`, o NSIS, o
WiX. Cada pacote tem:

- um nome estável (`rust`, `rust-android`, `jdk`, `android-sdk`, `android-ndk`, `python`,
  `node`, `dotnet`, `go`, `fpc`, `codelldb`, `clangd`...), um título e uma descrição;
- a versão fixada e, por plataforma (`windows-x86_64`, `linux-x86_64`, `macos-aarch64`...), os
  arquivos: nome, tamanho e **SHA-256** — o manifesto embutido no Aylis
  (`resources/packages.lock.json`, sucessor do `tools.lock.json`). O hash vem só dele: a origem
  do arquivo nunca é confiável por si;
- o que instalar faz: extrair (zip, tar.gz, tar.xz) numa pasta própria e, se preciso, um passo
  fixo do Aylis (ligar o toolchain do Rust, apontar o `JAVA_HOME`, instalar as wheels do pip
  sem rede...). Nada de script vindo do pacote;
- de quais outros depende (`rust-android` precisa de `rust`, `android-ndk` de `android-sdk`).

Instalado vai para `%USERPROFILE%\.aylis\tools\<nome>\<versão>` (fora do `AppData`: o Claude
Desktop, um pacote MSIX, desvia o `AppData` de quem ele abre; veja o CLAUDE.md), e o registro
do que está instalado fica em `%USERPROFILE%\.aylis\tools\installed.json`. Um pacote do Aylis
não mexe no que a máquina já tem: a verificação de ambiente continua aceitando a ferramenta já
instalada no sistema, e o pacote só entra onde ela falta.

## De onde vêm os arquivos

Na ordem, e o primeiro que tiver o arquivo com o hash certo vence:

1. **O cache** (`%USERPROFILE%\.aylis\downloads`).
2. **A pasta de pacotes**: a pasta `pacotes` ao lado do instalador ou da IDE, e as pastas
   configuradas nas Opções (uma pasta de rede, a pasta do Google Drive para desktop). Os
   arquivos estão lá pelo nome.
3. **Os espelhos** configurados nas Opções (sem nada gravado, o espelho padrão: a pasta do
   Google Drive de quem distribui o Aylis, `settings::DEFAULT_PACKAGE_MIRROR`; apagado nas
   Opções, fica apagado). Cada espelho é o
   endereço HTTPS de um índice (`pacotes.json`) que diz onde está cada arquivo — um endereço
   HTTPS, ou o id de um arquivo do Google Drive (`"drive": "<id>"`, baixado pelo endereço de
   download direto do Drive, com a confirmação que ele pede nos arquivos grandes). O índice
   só diz o lugar; o hash é o do manifesto embutido. O espelho também pode ser **o link de
   uma pasta compartilhada do Drive** (`https://drive.google.com/drive/folders/<id>`): a
   lista da pasta (a vista embutida dela, sem chave de API) vira o índice, pelo nome de cada
   arquivo. Basta subir a pasta do `pack` e compartilhar como "qualquer pessoa com o link".

Não há endereço oficial dos fabricantes como último recurso (decisão do autor): quem distribui o Aylis monta a
pasta de pacotes uma vez, numa máquina com internet, com `aylis-tool packages pack --out
<pasta>` (baixa das origens oficiais as versões fixadas, confere os hashes e grava a pasta e o
`pacotes.json` com os nomes), e a sobe no Drive, num servidor ou ao lado do instalador.

O download respeita o proxy do sistema e tem prazo; um erro diz qual arquivo, de onde e por quê.

**Pacote feito pelo Aylis.** O que não tem origem oficial leva, no lugar da `url`, o comando que
o gera (`"built": "aylis-tool skia-android"`): o `pack` o leva do cache, e o `packages update`
o pula (o comando grava o hash). São dois:

- o `skia-android-armv7`: o Skia do Android de 32 bits, que o projeto do Skia não publica pronto
  (o publicar com `armeabi-v7a` o instala);
- o `fpc`: o Free Pascal 3.2.2 do Windows, que só tem instalador (Inno Setup). `aylis-tool
  fpc-package --installer fpc-3.2.2.win32.and.win64.exe --innoextract innoextract.exe` confere o
  SHA-256 do instalador oficial (SourceForge), tira os arquivos com o
  [innoextract](https://constexpr.org/innoextract/) sem executar o instalador e monta o zip com o
  compilador, as mensagens e as units de 64 bits.

Quem distribui roda o comando, depois o `pack`, e sobe o arquivo com os outros.

**O WiX** é o pacote `wix`: o `.nupkg` da ferramenta do .NET, extraído, roda com `dotnet wix.dll`
(o .NET do pacote `dotnet` ou o instalado). A verificação de ambiente instala o pacote, sem o
`dotnet tool install`, que baixaria do nuget.org.

**Linux e macOS:** o `android-sdk` e o `dotnet` têm os arquivos das duas (macOS só em ARM), e o
`android-ndk` só o de Linux: o do macOS sai só em `.dmg`, que o Aylis não extrai (instale pelo
Android Studio).

## Onde aparecem

- **`aylis-tool packages`**: `list`, `install <nome>...`, `remove <nome>`, `sources`, `pack`.
- **A IDE**: Ferramentas → Pacotes (a lista, instalar e remover, com o progresso; as origens nas
  Opções). O "Corrigir…" da verificação de ambiente instala o pacote que resolve o item.
- **O instalador da IDE**: uma página só, agrupada. A IDE com o Rust é obrigatória (sem ele o
  Aylis não funciona); as outras linguagens e as plataformas de destino (Android, navegador) são
  opcionais, e cada uma marcada vem com tudo o que precisa: no fim, o `aylis-tool setup` dela
  instala o compilador ou o interpretador, o depurador e o servidor de linguagem (o mesmo do
  "Configurar linguagens" da IDE), dos pacotes da pasta `pacotes` ao lado do instalador ou do
  espelho padrão. Os projetos de exemplo não vão no instalador.

## A verificação de ambiente

Abre na hora com o último resultado guardado (`%USERPROFILE%\.aylis\environment.json`) e
reconfere em segundo plano. Cada conferência anota de que arquivos dependeu (o executável, a
pasta do SDK, o `PATH`), com a marca de cada um (`FileStamp`); sem mudança, não roda de novo.
As conferências rodam em paralelo. As plataformas que o projeto usa vêm primeiro, e o WSL só é
consultado quando o Linux é usado ou pedido (acordar o WSL custava 4,5 s a cada abertura).
