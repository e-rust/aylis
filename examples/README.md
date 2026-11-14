# Apps de demonstração

Três aplicativos famosos do Windows e um painel de dados feitos com o aylis, cada um nas
8 linguagens que o aylis aceita, e uma grade com milhões de registros (só em Rust). Cada app é **um** projeto do aylis (abre na IDE, roda com F9): o visual (`forms/`) é
um só e o código dos eventos fica numa pasta por linguagem. A linguagem ativa é a do
`.aylisproj` (Projeto → Opções troca; as outras pastas ficam intactas).

| App | O que tem |
|---|---|
| **Calculadora** | O modo padrão da Calculadora do Windows 11: memória (MC, MR, M+, M-, MS), histórico, porcentagem, `1/x`, `x²`, `√x`, `±`, a operação repetida no `=`, os atalhos de teclado (números, `+ - * /`, Enter, Esc, Backspace, Delete, `%`, `R`, `Q`, `@`, Ctrl+M/R/P/Q/L, Ctrl+H) e o erro de divisão por zero. |
| **Paint** | O Paint do Windows 11: seleção (arrastar ou setas movem, Shift+seta anda 10, Delete apaga, Ctrl+A/C/X/V), lápis, pincel, borracha, balde, conta-gotas, linha, retângulo, elipse (com preenchimento), espessura, as duas cores e a paleta, desfazer/refazer, zoom, abrir e salvar PNG/JPEG/BMP. O desenho é do componente `PaintCanvas`. |
| **Player** | O Reprodutor Multimídia do Windows 11: fila de reprodução, tocar/pausar, parar, anterior/próxima, embaralhar, repetir (todos ou um), sem som, volume, barra de posição e as páginas "Reproduzindo agora" e "Fila de reprodução". Quem toca é o `VideoPlayer` (áudio e vídeo). |
| **GradeGigante** | Só em Rust. Milhões de registros (10 milhões por padrão) com 100 colunas, 50 de texto e 50 de números, num `ClientDataSet` mostrado por um `DataGrid`, para conferir que a grade não trava: a carga roda numa thread e entra em pacotes pela fila da interface (com o tempo e a memória na tela), e dá para rolar, ordenar pelo título (Ctrl+clique soma colunas), filtrar na linha de filtro e arrastar os títulos para mudar a ordem das colunas durante e depois da carga. Fica só em Rust porque inventar um bilhão de valores passando pelos bindings das outras linguagens levaria horas. |
| **IBGE** | Os dados abertos do IBGE com os componentes de dados: `ClientDataSet` + `DataSource` + `DataGrid` em mestre-detalhe (estados → municípios → distritos, pelo `master_source`), `Label` ligados aos campos ("Ligar a campo…") com os números (Censo 2022, estimativas, área, densidade, PIB 2021 e PIB per capita), as bandeiras do estado e do município (`Image`), os mapas do Brasil, do estado e do município com o escolhido em destaque (`Svg` com `highlight` e `zoom_to`) e três `DBChart` (a população do município ano a ano, os 10 maiores municípios do estado e a população por estado). |
| **TesteAparelho** | Só em Rust, para o celular de verdade (`docs/testes-em-dispositivos.md`): uma aba por item que o emulador não cobre — teclado sem sugestões, 3D, câmera traseira e frontal (troca e ciclo de aberturas), microfone (nível e gravação), vídeo, notificações (simples, botões, progresso, imagem de um form) e sistema (tema, limite de quadros, área segura). Roda com `aylis-tool run examples/TesteAparelho/TesteAparelho.aylisproj --target android`. |
| **Estresse3D** | Só em Rust. O lingote da abertura da IDE (`aylis_3d::ingot`, com o brilho) numa grade de 1, 4, 9, 16, 25 ou 36 lingotes girando juntos, com o medidor de quadros: mede quanto o 3D aguenta num aparelho. |
| **Landing** | Só em Rust. A página de apresentação do aylis feita com o aylis. Tem o hero com o lingote 3D, as 8 linguagens em cartões, os números contando ao aparecer, as plataformas, os recursos entrando ao rolar, as perguntas (`Expander`) e o rodapé. Responde à largura pelas vistas (tablet e celular), e as seções crescem com o conteúdo (`auto_size`). Tem também a documentação (o botão "Documentação"): os guias de `docs/` e a referência de cada componente, refeitos com `aylis-tool docs` em `assets/docs`. De código, o lingote, o menu rolando até a seção (`src/forms/principal.rs`) e a navegação da documentação (`src/forms/documentacao.rs`). Para o navegador: `aylis-tool publish examples/Landing/Landing.aylisproj --target web --out pasta` e `aylis-tool serve pasta`. |
| **Abas** | Só em Rust. Abas como as do navegador (`TabStrip` com páginas, `TabStrip::add_page`): arrastar a aba para fora abre outra janela, que é outra instância do `frmPrincipal` (os handlers falam com ela; o `on_show` vê a aba que chegou e não cria as iniciais); arrastar para a faixa de outra janela leva a aba e a página para lá, no lugar marcado, e a janela que fica vazia fecha. A janela de uma aba só vai inteira. No arrasto, a prévia (o título e a miniatura da página) segue o mouse, também fora da janela. O "+" abre uma aba em qualquer janela e o título da janela segue a aba escolhida. O teste `arrastar` faz a ida e a volta sem janela. |
| **Galeria** | Só em Rust. Um cartão por componente visual, com o componente de verdade dentro, e a busca pelo nome. Publicada para a web, é a galeria em wasm. |
| **Relatorios** | Só em Rust. Relatórios mestre-detalhe com textos e números: 36 pedidos (o mestre: cliente, cidade, vendedor, situação, peso, total em reais) com os itens (código, produto, unidade, quantidade com casas, preço, desconto, total) e as parcelas (vencimento, forma, valor), ligados pelo `master_source`. A aba "Dados" mostra as três grades; "Pedidos com itens" é o relatório de cada pedido com os itens dele e as somas (`sub_header`, `sub_detail`, `sub_footer`), e "Vendas por vendedor" agrupa por vendedor com dois sub-detalhes (itens e parcelas) e os totais do grupo e do relatório. Os relatórios estão inteiros no form (`forms/principal.json`); o código só inventa os dados. O teste `relatorios` confere que a soma dos itens bate com o total de cada pedido. |
| **ERP** | Só em Rust, para medir o aylis num sistema grande (item 21 dos ajustes): 1009 forms (RH, Financeiro, Contábil, Compras, Vendas e Fiscal; 252 entidades, cada uma com cadastro mestre-detalhe, lista, consulta e os itens de todas) sobre 504 tabelas num SQLite que o próprio exemplo cria e preenche. As telas usam o mesmo código comum (`rust/src/erp/`): carregar, incluir, gravar o mestre com os itens numa transação (o `Delta`), excluir, cancelar, buscar e abrir o cadastro a partir da lista. Fica fora do workspace (compila à parte: `cargo build --manifest-path examples/ERP/rust/Cargo.toml`); o teste `telas` abre as 1008 telas e grava um pedido com itens; as medições estão em `medicoes/erp_1000_forms.json`. |
| **ProcessamentoAssincrono** | Só em Free Pascal. Trabalho pesado numa `TThread` própria com um form de progresso modal: a thread processa as notas e grava o andamento direto na tela (o `Send` tem trava), o Cancelar do modal roda enquanto ela trabalha (a thread dos handlers está livre) e, no fim, o `Aylis.Post` volta à thread dos handlers e fecha o modal. A thread fica em `pascal/src/forms/fechamento.pas`. |
| **pacotes/Medidor** | Um pacote de código (tipo B) de exemplo: o crate com o `PressureDial` (mostrador com alarme) e o `register`. É o modelo de quem cria (`aylis-tool pack-code`; veja `docs/funcionamento.md`). |

```text
examples/
  <App>/
    <App>.aylisproj        o projeto (linguagem ativa: rust)
    forms/                o visual, o mesmo para todas as linguagens
    code-sync.json        o que cada linguagem já tem (Projeto → Pendências entre linguagens)
    rust/ python/ typescript/ csharp/ java/ go/ cpp/ pascal/   o código de cada uma
  scripts/<app>.json      o roteiro de teste de cada app
```

## Os dados do IBGE

Ficam numa pasta só, a mesma para as 8 linguagens: `IBGE/assets/dados` (`estados.json`,
`municipios.json`, `distritos.json`, `populacao.json`, `mapas/*.svg` e `bandeiras/`). Quem
monta é o `aylis-tool`, uma vez: baixa da API do IBGE (localidades, Censo 2022, PIB dos
municípios, estimativas de população e malhas) e as bandeiras do Wikimedia Commons (achadas
pelo Wikidata), guarda as respostas em `IBGE/ibge-cache` e gera os arquivos. Rodar de novo só
baixa o que falta.

```text
aylis-tool ibge examples/IBGE/assets/dados
aylis-tool ibge examples/IBGE/assets/dados --offline   só refaz os arquivos, com o cache
```

As bandeiras vêm devagar (uma por segundo: o Wikimedia bloqueia quem pede rápido); município
sem bandeira no Commons mostra o aviso no lugar.

## Rodar

Na IDE, abra o `.aylisproj` e aperte F9. Pela linha de comando:

```text
aylis-tool run examples/Calculadora/Calculadora.aylisproj --language python
```

Os projetos em Rust são membros do workspace: `cargo run -p calculadora`, `-p paint`,
`-p player`, `-p ibge`, `-p relatorios`.

## O mesmo resultado em todas as linguagens

Cada app tem um roteiro (`scripts/<app>.json`): cliques, teclas, arrastos e o estado final
de alguns componentes. O roteiro roda sem janela (`AYLIS_SCRIPT`): nos apps em Rust no
próprio processo e nas outras linguagens pelo aylis-host, que espera a lógica responder a
cada passo. O estado tem de sair igual nas 8 linguagens:

```text
aylis-tool script examples/Paint/Paint.aylisproj examples/scripts/paint.json --language go
cargo test -p aylis-tools --test apps -- --ignored --nocapture
```

O teste também confere que nenhuma linguagem tem pendências (handlers que faltam ou lógica
atrasada). Mudou o form? `aylis-tool generate` (ou "Gerar código" na IDE) acrescenta os
handlers novos em todas as linguagens e renomeia os renomeados.

Detalhes que deixam as contas iguais em toda linguagem (veja os `motor` da Calculadora):
- a ordem de grandeza do número sai contando dígitos, não pelo `log10`, que varia um tico
  entre as bibliotecas;
- o tamanho da fonte do visor é inteiro (um número quebrado pode virar texto diferente);
- números enormes ficam guardados com 17 dígitos (voltam ao mesmo `double`);
- o embaralhar do Player usa a mesma conta de sorteio (com 64 bits; no TypeScript, `BigInt`).
