# Publicar no Windows: dados de versão e assinatura de código

Antivírus corporativos (o Cortex XDR da Palo Alto, o Defender for Endpoint) e o SmartScreen
do Windows decidem em parte pela **origem** do executável: quem fez, se dá para confirmar
que foi essa empresa e se o arquivo é conhecido. Um `.exe` novo, sem fabricante e sem
assinatura, é o perfil de um programa baixado de qualquer lugar, e cai nas regras mais
rígidas ("aplicativo não reconhecido", quarentena, bloqueio por "baixa prevalência").

O Aylis cuida de duas partes disso ao publicar para o Windows:

1. os **dados de versão** dentro do `.exe` (sempre, sem configurar nada);
2. a **assinatura de código** do `.exe` e do instalador (quando a máquina tem um
   certificado configurado).

## Dados de versão (VERSIONINFO)

Todo app compilado pelo Aylis no Windows leva os dados que o Explorer mostra em
Propriedades → Detalhes, tirados do `publish` do projeto (o nome do app, a versão, a descrição
e os direitos no diálogo Projeto → Publicar; o fabricante, `publish.publisher`, no `.aylisproj`
ou pelo `set_project_setting` do MCP):

| Campo | De onde vem |
|---|---|
| CompanyName | `publish.publisher` (vazio: o nome do app) |
| LegalCopyright | `publish.copyright` (vazio: `© ` e o fabricante) |
| FileDescription | `publish.description` (vazio: o nome do app) |
| ProductName | `publish.product_name` ("Produto" em Publicar; vazio: `publish.app_name`, e vazio esse, o nome do projeto). O `app_name` continua dando o nome do `.exe` |
| FileVersion, ProductVersion | `publish.version` (vazio: `1.0.0`) ou a última tag do git (abaixo); a parte numérica tem 4 números (`1.2.0-beta` → `1.2.0.0`) |
| OriginalFilename, InternalName | o nome do `.exe` publicado (`Vendas.exe`) |

### A versão da tag do git

Com `publish.version_from_git_tag: true` (a caixa "Versão da tag do git" em Publicar), a versão
publicada é a da última tag do git alcançável do commit atual (`git describe --tags --abbrev=0`,
sem o `v` do começo: `v1.4.0` → `1.4.0`), e não a `publish.version`. Vale em todas as
plataformas: o `.exe` e os instaladores do Windows, o Android (`versionName` e o `versionCode`
que sai dela), o iOS, o `.deb`, o `.rpm` e o AppImage. Não há variável de ambiente: o que manda
é o projeto.

- Sem o git, fora de um repositório, sem tag ou com uma tag que não é versão (`release-final`),
  a **publicação para** com o motivo e o que fazer. O build comum (Executar, `cargo build`)
  segue com a `publish.version` e avisa (`cargo:warning`).
- O `git` roda como processo, com os argumentos separados (nunca por shell). O build refaz
  quando nasce uma tag ou o ramo muda.
- O app lê em execução a versão gravada: `aylis::app_version()` em qualquer handler (ou
  `app.project.version()` no `Application`). Com a tag, o `.aylisproj` embutido no executável
  já sai com ela.
- O `Cargo.toml` gerado continua com a `publish.version` (a tag não muda arquivo do projeto).

**Mais de uma série de tags no mesmo repositório.** Num repositório com o cliente e o servidor
(`Client-v1.0.0.28` e `Server-v3.1.0`), a última tag de qualquer série seria a errada.
`publish.tag_prefix` ("Prefixo das tags" em Publicar) escolhe a série: a versão sai de
`git describe --tags --match "<prefixo>*"` e o prefixo é tirado (`Client-v1.0.0.28` →
`1.0.0.28`). Vazio, como antes: a última tag, sem o `v`.

**O build de desenvolvimento.** Com `publish.dev_version_after_tag` ("-dev fora da tag" em
Publicar), fora de uma tag limpa (commits depois dela, ou alteração pendente nos arquivos do
git) a versão sai com o último número + 1 e o sufixo `-dev` no texto: na tag
`Client-v1.0.0.28` com um commit a mais, `1.0.0.29-dev`, e nos números do `VS_FIXEDFILEINFO`
`1.0.0.29`, sem o sufixo. Assim o app que se atualiza sozinho e compara a versão instalada com a
publicada nunca troca o build de desenvolvimento pela 1.0.0.28. Uma tag com pré-release perde
a pré-release (`2.0.0-beta.1` → `2.0.1-dev`). Os commits contam no repositório inteiro (um
commit no servidor também tira o cliente da tag limpa). O build refaz a cada commit; uma
alteração pendente aparece na próxima compilação que o cargo refizer (a publicação sempre
confere).

O idioma da tabela segue o `locale` do projeto (português do Brasil, `0416`, ou inglês,
`0409`). O `aylis-build` grava esses dados no mesmo arquivo de recursos do ícone, sem precisar
do `rc.exe`. Os instaladores do NSIS e do Inno Setup já levavam os mesmos campos.

Preencha ao menos o **fabricante** (`publish.publisher`) com o nome da empresa: é ele que
aparece no SmartScreen e no aviso do Windows ao instalar, e deve ser o mesmo nome do
certificado.

## O app que se atualiza trocando só o `.exe`

Um app que se atualiza baixando só o executável (sem instalador) não pode depender de nada
ao lado dele. Duas opções em Projeto → Opções → Windows cuidam disso.

### O runtime do Visual C++ dentro do executável

Os executáveis do Rust no Windows (e o Skia, em C++) carregam o runtime do Visual C++
(`VCRUNTIME140.dll`, `VCRUNTIME140_1.dll`, `MSVCP140.dll`), que não vem com o Windows; por
padrão, a publicação copia essas DLLs ao lado do `.exe`. Com `publish.windows.static_crt`
("Runtime do Visual C++ dentro do executável"), o runtime vai ligado dentro do `.exe`:

- o Aylis grava no `.cargo/config.toml` **do projeto** (o arquivo que ele já mantém, desta
  máquina e fora do git) `[target.'cfg(all(windows, target_env = "msvc"))'] rustflags =
  ["-C", "target-feature=+crt-static"]`, e o cargo de fora (terminal, editor) compila igual.
  Nunca no `config.toml` global, que vale para todos os projetos. Onde o arquivo não é do Aylis
  (escrito pelo autor do projeto), as flags vão na linha de comando do cargo;
- a publicação deixa de copiar as DLLs; o `dumpbin /dependents` do `.exe` não mostra mais o
  `VCRUNTIME140.dll` nem os `api-ms-win-crt-*` (conferido com um app com Direct2D e WebView);
- a primeira compilação com a opção refaz as dependências (as flags mudam); as das duas formas
  convivem na pasta de compilação, e trocar a opção refaz só o executável;
- **com o Skia** no Windows, o rust-skia não publica o Skia já compilado com o runtime estático,
  e o `skia-bindings` compila o Skia do código-fonte, o que pede o LLVM (`clang-cl`) e demora
  muito na primeira vez. Sem o LLVM, a publicação para antes de compilar e diz isso. Com o
  Direct2D (o padrão no Windows), nada disso.

### O WebView2

O **carregador** do WebView2 já vai dentro do `.exe`: o `webview2-com` liga o
`WebView2LoaderStatic.lib` no MSVC, e o `.exe` não pede o `WebView2Loader.dll`. O **runtime**
(o navegador) é do sistema e não cabe no executável. `publish.windows.webview2` escolhe qual:

- `evergreen` (vazio, o padrão): o do Windows (o Windows 11 já traz; no 10, o instalador da
  Microsoft), atualizado pela própria Microsoft;
- `fixed`: o runtime de **versão fixa** da Microsoft, na pasta `WebView2` ao lado do executável
  (a publicação copia; os instaladores do NSIS, do Inno e o `.msi` levam a pasta inteira). Ao
  criar o primeiro `WebView`, o app aponta para ela pela `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`
  (quem já definiu a variável vale). Sem a pasta, o do Windows.

O de versão fixa tem cerca de 420 MB (x64, 158.0.4370.33), **não se atualiza sozinho** (as
correções de segurança chegam só quando o app publica de novo com a versão nova) e a Microsoft o
documenta para ser levado junto do app ("Fixed Version distribution mode"), sob os termos de
licença do runtime aceitos ao baixá-lo. Os cuidados da Microsoft: extrair com o `expand
<arquivo.cab> -F:* <pasta>` (o Explorer pode trocar a estrutura), não rodar de pasta de rede e,
no **Windows 10**, a partir da versão 120, dar leitura às contas das apps em contêiner
(`icacls <pasta> /grant *S-1-15-2-2:(OI)(CI)(RX)` e `*S-1-15-2-1`), o que o instalador do app tem
de fazer. O pacote de ferramenta `webview2-fixed` (versão fixa e SHA-256 no
`resources/packages.lock.json`) ainda espera o hash; até lá, a publicação diz como pôr a pasta à
mão.

## Assinatura de código

Assinado, o executável tem um fabricante **verificável**: o Windows confere que o arquivo não
mudou desde a assinatura e que o certificado foi emitido para aquela empresa por uma
autoridade reconhecida. É o que mais pesa para os antivírus e para o SmartScreen. A
publicação assina, nesta ordem:

- o `.exe` do app (antes de entrar no instalador, então o instalado também vai assinado);
- o instalador: o `.msi` (WiX) ou o `.exe` do NSIS ou do Inno Setup.

Cada assinatura leva **carimbo de tempo** (`/tr`, SHA-256): com ele, a assinatura continua
válida depois que o certificado vence. O padrão é o servidor público da DigiCert
(`http://timestamp.digicert.com`); outro, como o da Sectigo, vai nas Opções.

### O que precisa na máquina

- O `signtool.exe`, que vem com o **Windows SDK** (o Visual Studio com "Desenvolvimento para
  desktop com C++" já instala). O Aylis procura nas Opções, no PATH e em
  `C:\Program Files (x86)\Windows Kits\10\bin\<versão>\x64`.
- Um certificado de assinatura de código, configurado em **Ferramentas → Opções da IDE → Assinatura**.
  Fica em `%USERPROFILE%\.aylis\tools.json`, da máquina: **nunca no projeto** (quem clona o
  projeto não leva o certificado).

### De onde vem o certificado

Vale a primeira origem preenchida, nesta ordem:

1. **Repositório do Windows, pela impressão digital** (o recomendado). Instale o certificado
   (ou ligue o token USB, no caso dos EV) e copie a impressão digital em `certmgr.msc` →
   Pessoal → Certificados → o certificado → Detalhes → Impressão digital. Espaços e o
   caractere invisível que o Windows põe no começo são ignorados. A chave privada não sai do
   repositório (nem do token), e não há senha em lugar nenhum.
2. **Arquivo `.pfx`**: o caminho nas Opções e a senha **só** na variável de ambiente
   `AYLIS_SIGN_PASSWORD` (nunca gravada; nas mensagens da publicação aparece `*`). Sem a
   variável, a publicação para com o erro "falta a senha do .pfx".
3. **Na nuvem (Azure Trusted Signing)**: a DLL do cliente (`Azure.CodeSigning.Dlib.dll`, do
   pacote `Microsoft.Trusted.Signing.Client`) e o `metadata.json` com o endereço, a conta e o
   perfil. O `signtool` recebe `/dlib` e `/dmd`; o login é o do Azure na máquina (`az login` ou
   as variáveis do `DefaultAzureCredential`).

A linha que a publicação roda (sem a senha, que aparece como `*`):

```text
signtool sign /fd sha256 /tr http://timestamp.digicert.com /td sha256 /sha1 <impressão> App.exe
```

### Sem certificado

A publicação não falha: avisa uma vez que o executável sai **sem assinatura** e segue. A
verificação de ambiente (Ferramentas → Verificar ambiente) mostra o item **Assinatura de
código** como aviso quando falta o `signtool`, o certificado ou a senha do `.pfx`, ou quando a
impressão digital não está no repositório.

Para não assinar um projeto mesmo com certificado na máquina: desmarque "Assinar o executável e
o instalador ao publicar" em Projeto → Opções do projeto → Windows (`publish.windows.skip_signing`
no `.aylisproj`). Numa publicação só: `aylis-tool publish app.aylisproj --installer --no-sign`,
ou `sign: false` no `publish` do MCP. O Executar (compilação de desenvolvimento) não assina.

## Comprar o certificado

| Tipo | O que é | SmartScreen |
|---|---|---|
| **OV** (validação da organização) | A autoridade confere que a empresa existe. Desde 2023 a chave vem num token USB ou num HSM na nuvem (não há mais `.pfx` para baixar). | A reputação começa do zero e cresce com os downloads: os primeiros usuários ainda podem ver o aviso. |
| **EV** (validação estendida) | Validação mais rigorosa da empresa, chave em token ou HSM. | Desde 2024 a Microsoft trata EV como OV para a reputação: ajuda, mas não tira o aviso de imediato. |
| **Azure Trusted Signing** | Serviço da Microsoft por assinatura mensal; a Microsoft valida a empresa (ou a pessoa, em alguns países) e guarda a chave. | Reputação ligada à identidade validada; costuma ser o caminho mais barato e simples para quem já usa Azure. |

Autoridades conhecidas: DigiCert, Sectigo, GlobalSign, SSL.com, Certum. O nome no certificado
deve ser o mesmo do `publish.publisher`. O certificado vale de 1 a 3 anos; com o carimbo de
tempo, o que foi assinado antes do vencimento continua válido.

## Por que isso reduz os alertas

- **SmartScreen**: decide pela reputação do arquivo e de quem assinou. Sem assinatura, cada
  versão nova recomeça do zero ("O Windows protegeu o computador"). Assinado, a reputação se
  acumula no certificado e passa para as versões seguintes.
- **Antivírus corporativos** (Cortex XDR, Defender for Endpoint, CrowdStrike): usam a
  assinatura e o fabricante nas regras de confiança. Executável sem assinatura e sem dados de
  versão, recém-compilado e raro, é exatamente o que o aprendizado de máquina deles marca como
  suspeito. Assinado, a equipe de TI pode liberar **pelo fabricante** (uma regra só, para
  todas as versões), em vez de liberar arquivo por arquivo, pelo hash, a cada atualização.
- **Instalador**: o aviso do Controle de Conta de Usuário mostra "Fabricante: <empresa>" no
  lugar de "Fabricante: desconhecido".

Assinar não é garantia: um antivírus ainda pode marcar um arquivo assinado (falso positivo).
Nesse caso, envie o arquivo ao fabricante do antivírus como falso positivo (a Palo Alto e a
Microsoft têm formulário para isso) e peça à TI a liberação pelo certificado.
