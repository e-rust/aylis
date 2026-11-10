# Interceptar eventos num lugar só

O `EventInterceptor` (paleta System) recebe os eventos de **qualquer** componente antes do
handler: para conferir permissão, registrar auditoria, padronizar uma confirmação ou
cancelar, sem repetir código em cada form. Ele faz o papel que em outros frameworks se faz
com herança de forms ou com um evento global da aplicação.

## Propriedades

| Propriedade | O que faz |
|---|---|
| `scope` | `form` (só os componentes do form onde ele está) ou `application` (todos os forms) |
| `events` | os eventos que ele recebe, separados por `;` (`on_click;before_delete`); vazio = todos, menos os contínuos |
| `classes` | as classes que ele recebe (`Button;DataGrid`); vazio = todas |
| `enabled` | desligado, não intercepta |

Os contínuos (mover o mouse, rolar, redimensionar, o `on_timer` e os outros que se juntam)
ficam de fora do "todos": um handler num backend em outra linguagem receberia uma ida e volta
a cada movimento do mouse. Para recebê-los, ponha o nome em `events`.

## Eventos

- `on_intercept` (pergunta): antes do handler, com o form, o componente que disparou, o nome
  do evento, os argumentos dele e o nome do handler ligado. Cancelar faz o handler não rodar.
- `after_intercept`: depois do handler (auditoria).

A ordem: os interceptadores da aplicação, depois os do form, e por fim o handler. Vários
rodam na ordem de criação; o primeiro que cancela encerra. Passa também pelos eventos sem
handler ligado (um clique num botão sem `on_click`).

Num evento que já é pergunta, cancelar é **recusar**: o `before_post` cancelado não grava, o
`on_close_query` cancelado não fecha o form, o `before_delete` não exclui. O botão com
`modal_result` só fecha o form se o clique não foi cancelado, e o navegador de dados
(`DataNavigator`) pergunta `before_action` antes de cada botão (que também passa pelo
interceptador).

## Em cada linguagem

### Rust

```rust
pub fn interceptar(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
    let EventArgs::Intercept(e) = args else { return };
    let botao = e.sender.and_then(|s| ui.name(s)).unwrap_or_default();
    if e.event == "on_click" && botao.starts_with("btnExcluir") && !usuario_pode_excluir() {
        ui.refuse("Você não tem permissão para excluir");  // cancela, com a mensagem
    }
}
```

`ui.cancel_event()` cancela sem mensagem.

### Free Pascal

```pascal
procedure TFrmPrincipal.Interceptar(Sender: TAylisComponent; Info: TJSONObject; var Cancel: Boolean);
begin
  if (Info.Get('event', '') = 'on_click') and (Pos('btnExcluir', TJSONObject(Info.Find('sender')).Get('name', '')) = 1) then
    Cancel := not UsuarioPodeExcluir;
end;
```

### Python, TypeScript e as outras

O evento chega com `args` = `{ "form", "form_id", "sender": { "id", "name", "path", "class" },
"event", "handler", "args" }`; para cancelar, responda `cancel`:

```python
@aylis.on("interceptar")
def interceptar(event):
    info = event["args"]
    if info["event"] == "on_click" and info["sender"]["name"].startswith("btnExcluir"):
        event["reply"]["cancel"] = not usuario_pode_excluir()
```

Com o handler num backend em outra linguagem, o evento interceptado espera a resposta (e, se
ele próprio é pergunta, a pergunta dele também): um clique não é perdido nem roda antes da
hora.

## Depuração

Em modo de desenvolvimento, cada evento cancelado vai para o log com o caminho do componente e
do interceptador que cancelou:

```
aylis: frmPedidos.btnExcluir.on_click cancelado por frmPrincipal.EventInterceptor1
```
