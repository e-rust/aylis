# O app como servidor MCP

O `McpServer` deixa um agente de IA (Claude Code, Claude Desktop e outros clientes MCP) usar o
app **pelos componentes**, sem olhar a tela nem mandar cliques para a janela. Como o Aylis já
trata tudo por nome e por JSON (propriedades, eventos, o protocolo remoto), o agente trabalha
com a mesma precisão de um teste automatizado.

São quatro componentes não visuais da paleta **System**:

| Componente | O que faz |
|---|---|
| `McpServer` | Liga o servidor (HTTP em `127.0.0.1`, com token) e, se você quiser, as ferramentas genéricas de tela |
| `McpTool` | Uma ferramenta do domínio do app (filha do servidor): nome, título, descrição, parâmetros, anotações e o `on_execute` que responde |
| `McpResource` | Um recurso que o agente lê (filho do servidor), pelo endereço: o texto fixo ou o `on_read` |
| `McpPrompt` | Um prompt pronto que o cliente oferece ao usuário (filho do servidor), com argumentos |

A IDE tem um MCP próprio (para editar o projeto: veja Ferramentas → Opções da IDE → MCP). Este
aqui é diferente: vai **dentro do app publicado**, sem a IDE.

## Começando

Solte um `McpServer` num form (ou num DataModule) e ligue `active`:

```json
{ "class": "McpServer", "name": "mcpLoja", "active": true, "server_name": "loja",
  "instructions": "App de pedidos da loja. Use criar_pedido para lançar um pedido.",
  "children": [
    { "class": "McpTool", "name": "toolCriarPedido", "tool_name": "criar_pedido",
      "description": "Lança um pedido para o cliente, com os itens (código e quantidade).",
      "params": "cliente: string, itens: array, obs?: string",
      "on_execute": "tool_criar_pedido_execute" }
  ] }
```

No código, o handler lê os argumentos e responde. O `on_execute` é um evento que pergunta,
como o `before_post`: o esqueleto gerado devolve `Result`, e o `Err` vai ao agente como erro.

```rust
pub fn tool_criar_pedido_execute(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) -> Result<(), String> {
    let a = McpTool::arguments(args);
    let numero = pedidos::criar(ui, a["cliente"].as_str().unwrap_or_default(), &a["itens"])?;
    McpTool::reply(ui, json!({ "pedido": numero }));
    Ok(())
}
```

Num backend em outra linguagem (Python, C#...), os argumentos chegam como o texto JSON do
evento, e a resposta é o campo `result` (em Python, `event["reply"]["result"] = ...`). O
cliente do Aylis responde quando o handler termina, e uma exceção vira o erro.

## Conectando o agente

Com o app aberto, o método `client_command` devolve a linha pronta para o Claude Code:

```bash
claude mcp add --transport http loja http://127.0.0.1:7350/mcp --header "Authorization: Bearer <token>"
```

- `port` 0 (o padrão) escolhe uma porta livre a cada execução. Para configurar o cliente uma
  vez só, fixe a porta.
- `token` vazio gera um token aleatório ao ligar (fica na propriedade `token`). Para não
  reconfigurar, grave um token seu, com 16 caracteres ou mais, fora do código (variável de
  ambiente, arquivo do usuário).
- `url`, `running` e `last_error` (só leitura) dizem onde ele está e se ligou. Uma falha (porta
  ocupada, por exemplo) também chega no `on_error`.

### O Claude Desktop e a ponte stdio

O Claude Desktop só inicia servidores por stdio. Para ele, ligue a opção **`mcp_stdio`** do
projeto (`publish.mcp_stdio` no `.aylisproj`; desligada por padrão): o executável publicado
passa a aceitar `--mcp-stdio`, e o Claude Desktop o inicia assim. A ponte não abre janela nem
roda o app: repassa cada mensagem ao `McpServer` do app **que já está aberto** (com o login de
quem usa), pelo mesmo HTTP, com as mesmas regras (`127.0.0.1`, token, sem `Origin` de fora). Com
o app fechado, a ponte responde que ele não está aberto.

O método `stdio_config` devolve a entrada pronta do `claude_desktop_config.json`:

```json
{ "mcpServers": { "estoque": { "command": "C:\\Program Files\\Estoque\\estoque.exe", "args": ["--mcp-stdio", "estoque"] } } }
```

Como a ponte acha a porta e o token: o servidor ligado grava um arquivo pequeno (a porta, o
token, o processo) na pasta de dados **deste usuário** (`%LOCALAPPDATA%\aylis\mcp-apps` no
Windows; `~/.local/share/aylis/mcp-apps`, com permissão só do dono, no Linux;
`~/Library/Application Support/aylis/mcp-apps` no macOS), e apaga ao parar. O token nunca vai
para a configuração do cliente, e outro usuário da máquina não lê o arquivo. O nome do arquivo
leva um resumo do caminho do executável, então duas instalações do mesmo app não se confundem.
Com mais de um `McpServer` no app, o nome do servidor vai depois do `--mcp-stdio`.

## Os parâmetros (`params`)

Há três formas:

- **Forma curta:** `cliente: string, quantidade: integer, obs?: string`. O `?` marca o
  parâmetro opcional; os outros são obrigatórios. Os tipos aceitos são `string` (`texto`),
  `integer` (`inteiro`), `number`, `boolean`, `object` e `array` (`lista`).
- **Só as propriedades**, em JSON Schema: `{"cliente": {"type": "string", "description": "CPF"}}`.
- **O JSON Schema inteiro:** `{"type": "object", "properties": {...}, "required": [...]}`.

`enabled` falso tira a ferramenta da lista.

## Anotações, título e a resposta em JSON

As anotações avisam o agente, e o cliente pode pedir confirmação ao usuário antes de uma
ferramenta que muda dados:

| Propriedade | O que avisa |
|---|---|
| `read_only` | Só lê, não muda nada |
| `destructive` | Pode apagar ou estragar dados. Ligada por padrão, como no MCP; só vale sem `read_only` |
| `idempotent` | Repetir a chamada com os mesmos argumentos não muda mais nada |
| `title` | O nome para pessoas (o cliente mostra no lugar do `tool_name`) |

Uma resposta que é um objeto JSON (`McpTool::reply(ui, json!({..}))`) vai ao agente como texto
e também como `structuredContent`. Com `output_schema` (as mesmas três formas do `params`), o
agente sabe o formato de antemão (`outputSchema` no `tools/list`), e a resposta precisa ser um
objeto com os campos obrigatórios; senão, a chamada volta como erro, com o motivo.

```json
{ "class": "McpTool", "name": "toolTabela", "tool_name": "tabela", "title": "Consultar tabela",
  "params": "nome: string", "output_schema": "nome: string, colunas: array",
  "read_only": true, "idempotent": true, "on_execute": "tool_tabela_execute" }
```

## Recursos e prompts

Um `McpResource` é um conteúdo que o agente lê, pelo endereço (`uri`). Com `{nome}` no
endereço, é um modelo (`resources/templates/list`) e vale para todo endereço que bate com ele;
o `on_read` recebe `{"uri": .., "params": {"tabela": ..}}` e responde com
`McpTool::reply(ui, texto)` (ou `ui.reply("result", ..)`). Sem `on_read`, vale o `text`.
`mime_type` diz o tipo (`text/plain`, `application/json`, `text/markdown`).

```json
{ "class": "McpResource", "name": "resColunas", "uri": "app://tabelas/{tabela}",
  "title": "Colunas de uma tabela", "mime_type": "application/json", "on_read": "res_colunas_read" }
```

Um `McpPrompt` é um texto pronto que o cliente oferece ao usuário (no Claude Desktop, no menu
de prompts). `arguments` são os argumentos, separados por `;`, com a descrição depois do `:` e o
`?` no opcional; o `text` troca cada `{argumento}` pelo valor, ou o `on_get` responde (recebe os
argumentos em JSON).

```json
{ "class": "McpPrompt", "name": "prmResumo", "prompt_name": "resumo_tabela",
  "arguments": "tabela: o nome da tabela; foco?: o que destacar",
  "text": "Leia app://tabelas/{tabela} e resuma para que serve a tabela, destacando {foco}." }
```

## A versão, o `on_result` e o aviso de mudança

- `server_version` é a versão que o agente vê (`serverInfo.version`); vazia, a do app (a da
  publicação: `publish.version`, ou a da tag do git).
- `on_result` dispara depois de cada chamada de ferramenta, com um `McpResult`: `tool`,
  `arguments` (JSON), `result` (o texto, ou o JSON), `is_error` e `elapsed_ms`. Serve para
  registrar o que o agente fez e quanto levou.
- Quando uma lista muda (uma ferramenta, um recurso ou um prompt ligado, desligado ou
  renomeado), o servidor avisa quem está ouvindo (`notifications/tools/list_changed` e as de
  recursos e prompts, pelo `GET /mcp`). Criar ou liberar um filho em execução não passa por
  propriedade: chame o método `notify_lists_changed` do servidor depois.

## As ferramentas de tela (`expose_ui`)

Com `expose_ui`, o agente também ganha as ferramentas genéricas, que são os comandos do
protocolo remoto:

| Ferramenta | O que faz |
|---|---|
| `ui_forms` | Os forms abertos: nome, classe, título, se está visível |
| `ui_tree` | A árvore de componentes de um form |
| `ui_get` | As propriedades de um componente (todas, ou só uma) |
| `ui_items` | Os itens que o controle desenha sozinho: abas, linhas, nós, itens de menu |
| `ui_rows` | Os registros de um ClientDataSet, em páginas |
| `ui_data` | Uma operação num ClientDataSet (navegar, `locate`, `edit`, `set_values`, `post`...) |
| `ui_set` | Muda uma propriedade |
| `ui_click`, `ui_click_item` | Clica num controle, ou num item dele pelo texto |
| `ui_drag` | Arrasta de um componente a outro (ou entre pontos dele), em passos, com botão e modificadores |
| `ui_type`, `ui_key` | Digita no controle, ou aperta uma tecla, como o teclado (passa pelas máscaras e validações) |
| `ui_invoke` | Chama um método do componente |

Para limitar:

- `read_only`: só as que leem (`ui_forms`, `ui_tree`, `ui_get`, `ui_items`, `ui_rows` e o
  `ui_data` de leitura).
- `forms`: os forms que o agente alcança, separados por vírgula (vazio: todos).
- O próprio `McpServer` e os filhos dele (`McpTool`, `McpResource`, `McpPrompt`) nunca são
  alcançáveis por essas ferramentas, então o
  agente não consegue desligar o `read_only` sozinho.

Prefira as `McpTool` para o que importa. Uma ferramenta `criar_pedido` passa pela regra de
negócio e é uma chamada só. Com as de tela, o agente preenche campo por campo e pode errar no
caminho. Elas servem bem para consultar, testar e automatizar telas que ainda não têm
ferramenta.

## Segurança

- Só aceita conexões deste computador (`127.0.0.1`), com o token. Uma página de outro site
  (o `Origin` de fora) é recusada, mesmo daqui.
- `on_call` dispara antes de cada chamada, com o texto `{"tool": .., "arguments": ..}` (nas
  leituras, `{"resource": uri}` e `{"prompt": .., "arguments": ..}`). Use para registrar o que
  o agente fez, ou para recusar (`ui.refuse("...")`): o agente recebe a mensagem como erro.
- O fluxo de avisos (`GET /mcp`) também pede o token, e aceita até 8 clientes ouvindo.
- A ponte stdio só existe com a opção `mcp_stdio` do projeto; sem ela, o `--mcp-stdio` é
  ignorado e o servidor não grava o arquivo da porta e do token.
- No designer o servidor nunca liga.
- Com `expose_ui`, o agente lê o que a tela mostra (inclusive um `Edit` de senha). Restrinja
  com `forms` ou deixe a tela de fora e exponha só `McpTool`.

## Onde funciona

Windows, Linux e macOS. No celular e no navegador o componente existe mas não liga: o
navegador não abre portas, e o agente não roda no celular. `call_timeout` (30 s por padrão) é
quanto uma chamada pode esperar o app responder.
