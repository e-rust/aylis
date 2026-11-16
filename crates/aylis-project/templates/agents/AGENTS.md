# {name} — guia para agentes de IA

Aplicativo feito com o Aylis: a tela (forms, temas, imagens) é descrita em
JSON e o código dos eventos fica em {language}, na pasta `{folder}/`. Este arquivo foi criado
pelo "Novo projeto" da IDE do Aylis; pode editar à vontade (a IDE não o refaz).

## Onde está cada coisa

| Caminho | O que é |
|---|---|
| `{project_file}` | O projeto: forms, tema, idioma, alvos e publicação |
| `forms/*.json` | Os forms (a tela), no formato do Aylis (`{"aylis": 1, "root": {...}}`) |
| `themes/`, `assets/`, `locales/` | O tema, as imagens, fontes e sons, e as traduções |
| `{folder}/generated/` | Gerado pela IDE a cada "Gerar código": **somente leitura** |
| `{folder}/` (o resto) | O código do app: **é seu** |

{language_section}

## Regras do código

- **Gerado × seu.** O que a IDE gera fica em `generated/` (e nos arquivos marcados como gerados
  acima) e é refeito a cada geração: não edite. O arquivo de cada form é seu; a IDE só acrescenta
  no fim dele os handlers novos que você ligar a um evento.
- **Acesso tipado.** Use os componentes pelo nome tipado gerado (exemplos acima), nunca
  procurando pelo nome em texto: renomear um componente vira erro de compilação, e não um erro
  em tempo de execução.
- **Nada de banco na tela.** Sem conexão, SQL ou driver no código da tela: o `ClientDataSet`
  guarda os dados e as alterações (o delta), e quem aplica no banco é o backend.
- **Mensagens ao usuário** pelo diálogo do Aylis (o `MessageDialog`/`message_dlg`), nunca a
  caixa de mensagem do sistema: ela sai igual em todas as plataformas e com o tema do app.
- **Eventos um por vez.** Os handlers rodam na thread da tela, um de cada vez. Trabalho demorado
  vai para outra thread, e o resultado volta à tela pelo `post`/`post_to` (veja acima); no
  navegador não há threads.
- **Erro num handler** recusa a ação (um evento-pergunta, como `before_post`, mostra a mensagem
  e não deixa gravar) e aparece na caixa de erro do Aylis: o app não cai.

## Forms e a IDE

Com o projeto aberto na IDE do Aylis, **não grave os arquivos do projeto direto** (forms, temas,
`.aylisproj`, código): a IDE guarda o que está aberto na memória e perderia a mudança ao salvar.
O `.mcp.json` desta pasta liga o agente à IDE (`aylis-tool mcp`), e as instruções do servidor
dizem as ferramentas para mudar os forms e o código, compilar, testar e comandar o app.
Sem a IDE: `aylis-tool run {project_file}` executa.
