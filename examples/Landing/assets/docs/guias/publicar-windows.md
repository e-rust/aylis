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
Propriedades → Detalhes, tirados do projeto (Projeto → Opções → Publicação):

| Campo | De onde vem |
|---|---|
| CompanyName, LegalCopyright | `publish.publisher` (vazio: o nome do app) |
| FileDescription, ProductName | `publish.app_name` (vazio: o nome do projeto) |
| FileVersion, ProductVersion | `publish.version` (vazio: `1.0.0`); a parte numérica tem 4 números (`1.2.0-beta` → `1.2.0.0`) |
| OriginalFilename, InternalName | o nome do `.exe` publicado (`Vendas.exe`) |

O idioma da tabela segue o `locale` do projeto (português do Brasil, `0416`, ou inglês,
`0409`). O `aylis-build` grava esses dados no mesmo arquivo de recursos do ícone, sem precisar
do `rc.exe`. Os instaladores do NSIS e do Inno Setup já levavam os mesmos campos.

Preencha ao menos o **fabricante** (`publish.publisher`) com o nome da empresa: é ele que
aparece no SmartScreen e no aviso do Windows ao instalar, e deve ser o mesmo nome do
certificado.

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
- Um certificado de assinatura de código, configurado em **Ferramentas → Opções → Assinatura**.
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
