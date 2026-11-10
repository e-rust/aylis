# Threads e a interface

Resumo: **a interface tem uma thread só**, e o compilador impede outra thread de mexer
nela. Trabalho pesado vai para outras threads, e o resultado volta pela fila da interface
(`ui.poster()`). Não existe corrida de dados com os
componentes. O que ainda pode dar errado é lógica (esperar demais, ordem dos eventos), e
está listado abaixo.

## Por que não há corrida de dados

Em muitos frameworks a interface também não é *thread-safe*, mas nada impede mexer num
componente de dentro de uma thread de trabalho: o erro aparece em produção, de vez em quando.
No Aylis, o `Ui` (que guarda todos os componentes) não é `Send` nem `Sync`: o Rust **não
compila** um código que leve o `Ui`, ou uma referência a um componente, para outra thread.

O único caminho de uma thread de trabalho para a interface é o [`Poster`](../crates/aylis-core/src/post.rs):

```rust
let poster = ui.poster();               // pode ir para qualquer thread
std::thread::spawn(move || {
    let total = consulta_demorada();    // aqui, nada da tela
    poster.post(move |ui| {             // isto roda na thread da interface
        let _ = ui.set_prop(lbl_total, "text", total.to_string());
    });
});
```

`post` guarda a função numa fila e acorda a interface, que roda as funções na ordem em que
chegaram, entre um quadro e outro.

## O que já roda em outras threads no próprio Aylis

| Quem | Faz na thread dele | Volta pela fila |
|---|---|---|
| Áudio (`MediaPlayer`) | decodifica e toca | `on_end` |
| Vídeo (`VideoPlayer`) | decodifica até 6 quadros à frente | o quadro novo (troca de imagem) |
| Hot reload (desktop) | lê os arquivos do projeto a cada 400 ms | aplica o form, o tema, as traduções |
| Inspetor do app (TCP) e canal de desenvolvimento (WebSocket) | lê os comandos da IDE | executa e devolve a resposta |
| `aylis-host` (backend em outro processo) | lê as mensagens do backend | executa cada comando |
| IDE: Executar e Publicar | compila, roda o `cargo` e o `adb` | as linhas da saída |

Os decodificadores seguram a trava compartilhada só para pôr ou tirar um quadro, nunca
enquanto decodificam.

## Riscos que continuam (e como evitar)

1. **Esperar outra thread dentro da interface congela a tela.** Um `recv()` sem prazo, um
   `join()`, um `Mutex` disputado com uma thread lenta: a tela para de responder até a
   outra terminar. Se a outra thread estiver esperando a interface (por exemplo, postou
   uma função e espera a resposta), as duas param para sempre (*deadlock*). Regra: a
   interface nunca espera; ela só recebe o que for postado.
   - Corrigido nesta revisão: fechar um vídeo ou um áudio fazia `join()` na thread de
     decodificação. Com uma fonte de rede presa (RTSP), a tela congelava. Agora a interface
     só avisa a thread, que sai sozinha.
2. **O mundo muda entre o `post` e a execução.** Quando a função roda, o componente pode
   ter sido liberado, o form fechado, o registro atual do dataset pode ser outro. Os ids
   do Aylis são "geracionais": um id de um componente liberado não aponta para outro (as
   consultas devolvem `None`). Trate o `None` e, nos dados, guarde a chave ou o id do
   registro (`DataSet::goto_id`), não a posição.
3. **Não há ordem entre threads diferentes.** O que uma thread posta chega na ordem em que
   ela postou; entre duas threads, não há garantia. Se a ordem importa, faça o trabalho
   numa thread só, ou numere as respostas e ignore as velhas (uma busca digitada rápido:
   só a resposta da última tecla vale).
4. **Travas suas.** Dados que você divide entre threads (`Arc<Mutex<...>>`) seguem as
   regras de sempre: segure a trava pouco tempo e nunca chame algo que espere a interface
   enquanto ela está presa.
5. **Pânico.** Um pânico numa thread de trabalho termina só aquela thread; numa função
   postada, acontece na thread da interface (como qualquer evento).

## Fora do Rust

- **C ABI (`aylis.dll`):** o C não tem o compilador para proteger. `aylis_execute` só vale
  na thread que criou o app (de outra, devolve um erro); dentro de um evento, use o `ctx`
  do callback; de qualquer outra thread, `aylis_post`, que manda o comando para a fila da
  interface.
- **Backend em Python (ou outra linguagem) pelo `aylis-host`:** o backend é outro processo,
  e as mensagens entram numa fila: ele não disputa nada com a tela. Dentro do backend, o
  cliente `aylis_client.py`:
  - grava uma mensagem por vez (uma trava), então várias threads podem usar `send`;
  - recusa `call` fora da thread principal (duas threads lendo roubariam as respostas uma
    da outra);
  - tem `post(funcao)`, que roda a função na thread principal do backend.

  Testado em `crates/aylis-host/tests/python_backend.rs`: 200 envios de outra thread
  chegam inteiros e em ordem.
- **Navegador:** o WebAssembly roda numa thread só; os eventos do WebSocket e os
  temporizadores entram na mesma fila.

### Backends em outras linguagens: o padrão

Os handlers rodam um de cada vez, na thread do laço do cliente (a thread principal da
interface). Trabalho pesado vai para uma thread da própria linguagem, com um form em
`ShowModal` mostrando o progresso e um Cancelar comum (o handler dele roda, porque a thread
dos handlers está livre). Em todos os clientes (Pascal, C#, Java, Go, C++, Python e
TypeScript, no desktop e nas sessões da ponte):

- **gravar** (`Send`, mudar uma propriedade) vale de qualquer thread: a escrita tem trava;
- **ler** (`Call`, que espera a resposta) só na thread dos handlers: a resposta chega pela
  mesma leitura, e o cliente recusa um `Call` de outra thread com um erro claro;
- **`Post(função)`** roda a função na thread dos handlers (o `TThread.Queue`): no desktop o
  cliente pede à tela um eco (`wake`), que acorda o laço parado lendo a entrada. É o jeito de
  fechar o modal de progresso no fim do trabalho.

O exemplo é o `examples/ProcessamentoAssincrono` (em Pascal).

**Diálogos que esperam**: `MessageDlg(texto, tipo, botões)` devolve o botão
(`"yes"`) e `ShowModal(form)` o resultado (`"ok"`), só quando a caixa fecha; enquanto isso,
o cliente trata os eventos que chegam (os do modal e os `Post`). Em Rust não há: a thread da
tela não pode esperar (o winit e o navegador não têm laço aninhado), então lá é o callback
(`message_dlg(ui, ..., |ui, resultado| ...)`).

**Eventos que perguntam** (`on_close_query`, `before_post`, `before_delete`, o
`on_validate` do Field, `on_key_down` e `on_key_press`, `on_drag_start` e `on_drag_over`):
a tela espera a resposta, que o cliente manda sozinho quando o handler termina. Recusar com
mensagem é sempre exceção; responder um valor é do jeito de cada linguagem:

| Linguagem | Recusar | Responder |
|---|---|---|
| Free Pascal | `raise EAylis.Create('Informe o CPF')` | parâmetros `var`: `(Sender; var CanClose: Boolean)`, `(Sender; var Key: string; Event)`, `(Sender; var Key: string)`, `(Sender; var Data, Caption: string)`, `(Sender; const Data: string; var Accept: Boolean)` |
| Rust | `Err("Informe o CPF".into())` (o handler devolve `Result<(), String>`) | `ui.reply("can_close", false)`, `ui.suppress_key()`, `ui.replace_key_char('.')`, `ui.begin_drag(..)`, `ui.reject_drop()` |
| C# | `throw new Exception(...)` | `ev.Reply["can_close"] = false` |
| Java | `throw new AylisException(...)` | `ev.reply().put("can_close", false)` |
| Go | `panic("...")` | `ev.Reply["can_close"] = false` |
| C++ | `throw std::runtime_error(...)` | `ev.reply.set("can_close", false)` |
| Python | `raise ValueError(...)` | `event["reply"]["can_close"] = False` |
| TypeScript | `throw new Error(...)` | `ev.reply.can_close = false` |

Os campos: `can_close` (fechar), `key` (`""` engole a tecla; outro caractere troca, no
`on_key_press`), `data` e `label` (o que vai no arrasto), `accept` (o destino aceita). A IDE
gera o esqueleto com a assinatura certa (o catálogo marca cada evento-pergunta). Um evento
que pergunta roda mesmo enquanto o backend espera um `Call`: o `Post` do próprio backend
dispara o `before_post`, e o `data post` só responde quando a gravação termina (recusada,
volta o erro). Com `on_key_down`/`on_key_press` ligados a um handler remoto, cada tecla
daquele controle espera a resposta (as que chegam nesse meio-tempo ficam numa fila da tela,
em ordem): pela `--bridge` na internet, cada tecla custa uma ida e volta; prefira o
`char_map`, o `char_case` e os filtros (`allowed_chars`), que a tela resolve sozinha, e o
`drag_data`/`drop_types`, que arrastam sem perguntar.

## Trabalho em segundo plano ligado ao form

Em Rust, o jeito documentado é o `ui.background(dono, trabalho, fim)`: o trabalho roda numa
thread e o fim, na tela, com o resultado:

```rust
ui.background(form, move || buscar_no_servidor(), |ui, dados| carregar(ui, dados));
```

- Se o dono (o form, um componente) foi liberado até lá, o fim **não roda**: o resultado é
  descartado, em vez de chegar a um form que já não existe.
  Com o `poster` direto, o mesmo é `poster.post_to(dono, ...)`.
- **Na web** o wasm não cria threads (`std::thread::spawn` compila e falha ao rodar): lá o
  `background` roda o trabalho na própria thread da tela. Funciona, mas a tela espera num
  cálculo pesado. Por isso, num projeto que publica para a web, use o `background`, nunca o
  `std::thread::spawn` direto.
- **Para esperar** (rede, arquivos, uma biblioteca `async` como o `sqlx`), o
  `ui.spawn_local(dono, async { ... }, fim)`: roda na thread da tela sem travar, é cancelado
  quando o dono sai, e os serviços têm versão para `await` (`ui.http_get(url)`,
  `ui.choose_files(...)`, `ui.request_permissions(...)`, `ui.print_pages(...)`). É para
  esperar, não para calcular: um cálculo pesado dentro do `async` trava a tela.

| Quero | Desktop e celular | Web |
|---|---|---|
| Calcular algo pesado | `background` (thread) | `background` (a tela espera) |
| Esperar a rede ou um arquivo | `spawn_local` ou `background` | `spawn_local` |
| Usar uma biblioteca `async` | `spawn_local` | `spawn_local` |
