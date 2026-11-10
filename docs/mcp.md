# O app como servidor MCP

O `McpServer` deixa um agente de IA (Claude Code, Claude Desktop e outros clientes MCP) usar o
app **pelos componentes**, sem olhar a tela nem mandar cliques para a janela. Como o Aylis já
trata tudo por nome e por JSON (propriedades, eventos, o protocolo remoto), o agente trabalha
com a mesma precisão de um teste automatizado.

São dois componentes não visuais da paleta **System**:

| Componente | O que faz |
|---|---|
| `McpServer` | Liga o servidor (HTTP em `127.0.0.1`, com token) e, se você quiser, as ferramentas genéricas de tela |
| `McpTool` | Uma ferramenta do domínio do app (filha do servidor): nome, descrição, parâmetros e o `on_execute` que responde |

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

## Os parâmetros (`params`)

Há três formas:

- **Forma curta:** `cliente: string, quantidade: integer, obs?: string`. O `?` marca o
  parâmetro opcional; os outros são obrigatórios. Os tipos aceitos são `string` (`texto`),
  `integer` (`inteiro`), `number`, `boolean`, `object` e `array` (`lista`).
- **Só as propriedades**, em JSON Schema: `{"cliente": {"type": "string", "description": "CPF"}}`.
- **O JSON Schema inteiro:** `{"type": "object", "properties": {...}, "required": [...]}`.

`read_only` avisa o agente que a ferramenta só lê. `enabled` falso tira a ferramenta da lista.

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
| `ui_type`, `ui_key` | Digita no controle, ou aperta uma tecla, como o teclado (passa pelas máscaras e validações) |
| `ui_invoke` | Chama um método do componente |

Para limitar:

- `read_only`: só as que leem (`ui_forms`, `ui_tree`, `ui_get`, `ui_items`, `ui_rows` e o
  `ui_data` de leitura).
- `forms`: os forms que o agente alcança, separados por vírgula (vazio: todos).
- O próprio `McpServer` e as `McpTool` nunca são alcançáveis por essas ferramentas, então o
  agente não consegue desligar o `read_only` sozinho.

Prefira as `McpTool` para o que importa. Uma ferramenta `criar_pedido` passa pela regra de
negócio e é uma chamada só. Com as de tela, o agente preenche campo por campo e pode errar no
caminho. Elas servem bem para consultar, testar e automatizar telas que ainda não têm
ferramenta.

## Segurança

- Só aceita conexões deste computador (`127.0.0.1`), com o token. Uma página de outro site
  (o `Origin` de fora) é recusada, mesmo daqui.
- `on_call` dispara antes de cada chamada, com o texto `{"tool": .., "arguments": ..}`. Use
  para registrar o que o agente fez, ou para recusar (`ui.refuse("...")`): o agente recebe a
  mensagem como erro.
- No designer o servidor nunca liga.
- Com `expose_ui`, o agente lê o que a tela mostra (inclusive um `Edit` de senha). Restrinja
  com `forms` ou deixe a tela de fora e exponha só `McpTool`.

## Onde funciona

Windows, Linux e macOS. No celular e no navegador o componente existe mas não liga: o
navegador não abre portas, e o agente não roda no celular. `call_timeout` (30 s por padrão) é
quanto uma chamada pode esperar o app responder.
