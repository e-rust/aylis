# Documentação do Aylis

O Aylis é um framework de interface em Rust: forms, componentes com propriedades e eventos,
alinhamento (`Align`, `Anchors`), temas com estilos por classe, camada de dados
(`ClientDataSet`, `Field`, `DataSource`) e uma IDE com
paleta, designer e Object Inspector. O mesmo app roda no Windows, Linux, macOS, Android,
iOS e no navegador.

| Documento | Para quem quer... |
|---|---|
| [Como o Aylis funciona](funcionamento.md) | entender as peças: `Ui`, componentes, layout, eventos, pintura, temas, dados, projeto |
| [Criar um componente](criar-componente.md) | escrever um componente novo (do zero ou a partir de outro), com propriedades, eventos, tema, subtipos e paleta |
| [Em tempo de execução](tempo-de-execucao.md) | achar componentes, ler e trocar propriedades, ligar eventos, trabalhar com dados, trocar tema |
| [Gravar as alterações no banco](gravar-alteracoes.md) | converter o delta do `apply_updates` em SQL, com mestre-detalhe e a chave gerada passando aos itens, em cada linguagem |
| [Interceptar eventos](interceptador.md) | tratar num lugar só os eventos de todos os componentes (permissão, auditoria, cancelar), com o `EventInterceptor` |
| [Notificações push](push.md) | o `PushNotifications`: registrar o aparelho (FCM, APNs, Web Push, WNS, UnifiedPush), receber as mensagens e o que o servidor manda |
| [O app como servidor MCP](mcp.md) | deixar um agente de IA usar o app pelos componentes, sem depender da tela (`McpServer`, `McpTool`) |
| [Carregamento e app enxuto](carregamento.md) | entender como o projeto vira executável e por que o app final só leva os componentes que usa |
| [Publicar no Windows](publicar-windows.md) | pôr empresa e versão no `.exe`, assinar o executável e o instalador (certificado OV/EV ou Trusted Signing) e reduzir os alertas do SmartScreen e dos antivírus |
| [Quanto cada componente custa](medidas.md) | saber quanto cada componente acrescenta ao executável e à memória RAM |
| [Temas de plataforma e aparência](temas.md) | usar o visual do Material, do iOS e do Windows; misturar temas; ver quem usa qual tema |
| [Vídeo e pintura](video-e-pintura.md) | tudo o que se configura na pintura e no vídeo (projeto, propriedades e variáveis de ambiente) e como medir na sua máquina (`AYLIS_BENCH`, `aylis-tool bench-frames`) |
| [Renderizadores no Windows](renderizadores.md) | comparar Skia (CPU e GPU) com Direct2D: tamanho do executável, velocidade e memória |
| [Material 3 e iOS 26 (Liquid Glass)](material3-ios.md) | usar os componentes dos catálogos do Material 3 e da Apple, e o vidro (desfoque do fundo) |
| [Componentes do Vuetify no Aylis](vuetify.md) | achar o equivalente de cada componente do Vuetify (e os 29 acrescentados para cobrir a lista) |
| [Editores visuais de propriedades](editores-visuais.md) | ajustar ângulo, sombra, curva, tempos e cor com o mouse no Object Inspector (como no DevTools) |
| [3D: modelos glTF/GLB](3d.md) | mostrar modelos 3D animados com controles por cima, e quanto isso custa |
| [Lottie, SVG, Markdown e WebView](lottie-svg-markdown-webview.md) | tocar animações (emojis Noto), desenhar SVG, mostrar Markdown e páginas da web no form |
| [Fontes e ícones offline](fontes-e-icones.md) | usar fontes do Google Fonts e ícones Material/Fluent sem internet, com só o que o app usa indo para o executável |
| [Testes em Mac e celular](testes-em-dispositivos.md) | saber o que já foi feito e ainda precisa ser conferido num Mac ou num celular de verdade |
| [Seu primeiro projeto](tutorial-primeiro-projeto.md) | criar, desenhar e executar o primeiro app passo a passo |
| [Seu primeiro componente](tutorial-componente.md) | o tutorial de um componente novo |
| [Depurar na IDE](depuracao.md) | pontos de parada, passo a passo, o Profiler e o editor de código |
| [Threads e a interface](threads.md) | trabalhar fora da thread da tela e voltar por `post`/`post_to` |
| [Segundo plano](segundo-plano.md) | o `BackgroundService` |
| [Forms em equipe](forms-em-equipe.md) | diferenças e conflitos de forms no git |
| [A placa de vídeo de cada janela](gpu-por-monitor.md) | a GPU que segue o monitor |
| [Testes do projeto](testes-do-projeto.md) | os testes de um projeto do Aylis |
| [Pacotes de ferramentas](pacotes.md) | de onde o Aylis baixa e instala o Rust, os SDKs e os depuradores |

O essencial em uma tabela:

| O quê | No Aylis |
|---|---|
| Componente e controle visual | trait `Component` + `ControlBase` |
| O aplicativo e a tela | `aylis::Application`, `Ui` |
| Achar um componente do form | o acesso tipado gerado (`self.btnSalvar`, `frmCliente.edtNome`); pelo nome (`ui.find_in(form, "Button1")`), só o criado em execução |
| Trocar uma propriedade | `frmCliente.edtNome.text = ...` (acesso tipado) ou, por nome, `ui.set_prop(id, "text", "Ok")` |
| Ligar um evento | `ui.on_click(id, ...)` ou handler pelo nome no form (gerado pela IDE) |
| Registrar uma classe na paleta | `ui.register_class::<T>("Paleta")` |
| O arquivo do form | `forms/*.json` |
| Estilos | `Theme` (JSON), `style` (a variante: `"Button.danger"`) |
| Texto dos controles | `h_align`, `v_align`, `wrap`, `trimming` |
| Dica | `hint` |
| Dados em memória | `ClientDataSet`, `Field`, `DataSource` (`aylis-data`) |
