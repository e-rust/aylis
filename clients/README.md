# Backends em outras linguagens

A tela do aylis pode ser comandada por um programa em qualquer linguagem: o `aylis-host`
abre a tela e inicia o backend, e os dois conversam por stdin/stdout, uma mensagem JSON
por linha (o protocolo do `aylis-remote`, o mesmo do cliente Python em `clients/python`).
Por WebSocket, o backend pode estar até em outra máquina.

Cada pasta tem um cliente pequeno, sem dependências além da biblioteca padrão, e o mesmo
backend de demonstração: um campo, um botão e a resposta do backend.

| Linguagem | Cliente | Demonstração |
|---|---|---|
| TypeScript (Node 22.6+ e Bun) | `typescript/aylis.ts` | `aylis-host -- bun clients/typescript/demo.ts` |
| C# (.NET 6+) | `csharp/Aylis.cs` | `dotnet build clients/csharp` e `aylis-host -- dotnet clients/csharp/bin/.../AylisDemo.dll` |
| Java (17+) | `java/dev/aylis/Aylis.java` | `javac -d out ...` e `aylis-host -- java -cp out Demo` |
| Go (1.21+) | `go/aylis/aylis.go` | `go build -o demo.exe ./demo` (em `clients/go`) e `aylis-host -- clients/go/demo.exe` |
| C++17 | `cpp/aylis.hpp` (um cabeçalho) | `cl /std:c++17 /EHsc /utf-8 clients\cpp\demo.cpp` ou `g++ -std=c++17` |
| Free Pascal / Lazarus | `pascal/aylis.pas` (usa `fpjson`) | `fpc -Mobjfpc clients/pascal/demo.lpr` |

Rode da pasta raiz do repositório (as demonstrações leem `clients/demo_layout.json`).

A API é a mesma em todas, com os nomes de cada linguagem:

- `send(cmd, campos)`: um comando sem esperar resposta (`load`, `set`, `click`, `show`...);
- `call(cmd, campos)`: um comando que devolve o resultado (`get`, `run`, `data`...);
- `on(nome, handler)`: o handler pelo nome usado no layout (`"on_click": "salvar"`), ou
  `"frmClientes.salvar"` só para aquele form;
- `run(form)` / `loop()`: abre a tela e trata os eventos até ela fechar.

Os comandos estão documentados no `aylis-remote` (`crates/aylis-remote/src/lib.rs`).

## Segurança do backend

Publicado para o celular ou o navegador, o app conecta na ponte (`aylis-host --bridge`), que
inicia um backend por tela. A ponte fica na internet e **não tem senha própria**: um token
embutido no app seria público (qualquer um extrai do APK ou do wasm). Então:

- **Todo evento pode ser forjado.** O backend recebe `{"event": "on_click", "handler":
  "btn_excluir_click", ...}` e chama a função, mas nada garante que o botão estava visível ou
  habilitado, nem que o usuário tinha aquele form aberto: qualquer um escreve um cliente
  WebSocket e chama `btn_excluir_click` direto. Confira a permissão **no handler**, no
  servidor, como numa API web.
- **O que vem da tela é entrada do usuário**: o texto de um campo (`get`), os argumentos de
  um evento. Valide, e nunca concatene em SQL (use parâmetros).
- **O login é do app**: o backend pede usuário e senha (num form do próprio app), guarda quem
  entrou no estado da sessão e confere em cada handler.
- **A ponte se protege do resto**: limite de conexões (`--max-connections`, padrão 200, e
  `--max-per-ip`, padrão 50), prazo de 10 s para o handshake, mensagens até 1 MB, conexão
  parada por 30 min fechada, e `--allow-origin https://app.empresa.com` para recusar outros
  sites no navegador (sem `Origin`, o app nativo, entra). Na rede, use TLS (`--tls-cert` e
  `--tls-key`).

## Sessões: um processo para todas as telas

Por padrão a ponte inicia um backend por tela, e 200 usuários são 200 processos. Com
`aylis-host --bridge 0.0.0.0:9000 --sessions -- python src/main.py`, um processo só atende
todas as telas:

- cada mensagem que chega ao backend leva o número da tela (`"session": 17`), e cada uma que
  ele manda diz para qual tela vai;
- a tela que conecta chega como `{"session": n, "event": "session_open"}`, e a que sai como
  `session_close`;
- se o backend cair, as telas abertas fecham e a próxima tela inicia outro.

No Python, cada tela é uma instância da sua classe, numa thread própria. Um handler demorado
não trava as outras telas, e uma exceção fica naquela sessão. Guarde o estado do usuário nos
atributos, **nunca em variáveis globais**: elas são de todos os usuários.

```python
import aylis_client

class Vendas(aylis_client.Session):
    form = "frmVendas"

    def started(self):          # a tela conectou
        self.usuario = None
        self.run()

    @aylis_client.handler("btn_entrar_click")
    def entrar(self, event):
        self.usuario = self.call("get", target="frmVendas.edtUsuario", prop="text")

    def closed(self):           # a tela saiu
        ...

aylis_client.serve(Vendas)
```

As outras linguagens ainda usam um processo por tela (o padrão).

## Testes

`cargo test -p aylis-host --test languages` compila e roda cada demonstração com
`--selftest` (o backend preenche o campo, clica e confere a resposta, sem janela). A
linguagem sem compilador na máquina é pulada. Fora do PATH, o caminho de cada ferramenta vai
na aba da linguagem nas Opções da IDE (o Go, o .NET, o Free Pascal e as units dele, o
`vcvars64.bat` do Visual C++...).

## Esforço por linguagem (avaliação de 26/10/2026)

| | Cliente | Gerar handlers na IDE | Depurar |
|---|---|---|---|
| TypeScript | pronto (JSON nativo, `await`) | fácil: um arquivo por form | Node `--inspect` / Bun: o VS Code anexa |
| C# | pronto (`System.Text.Json`) | fácil: classe parcial por form | `dotnet` + o depurador do VS Code (netcoredbg) |
| Java | pronto (JSON próprio, ~150 linhas) | fácil | JDWP (`-agentlib:jdwp`), o VS Code anexa |
| Go | pronto (`encoding/json`) | fácil | Delve (`dlv`), por DAP: o mesmo caminho do depurador da IDE |
| C++ | pronto (JSON próprio, um cabeçalho) | médio: compilador por plataforma | `cppvsdbg`/`gdb`/`lldb` pelo VS Code |
| Free Pascal | pronto (`fpjson`) | médio: unit por form, `lpr` | `gdb` (o Lazarus também depura o backend) |

O que falta para cada uma virar linguagem de projeto na IDE (como Rust e Python): o
modelo do projeto, o gerador de handlers por form e o comando de compilar/executar no
F9. O cliente e o protocolo, a parte difícil, já estão prontos e testados.
