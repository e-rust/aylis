# Screen

Os monitores, para todas as linguagens: `count()`, `primary()` (o índice do principal), `monitors()` (a lista em JSON: nome do aparelho, nome do sistema, principal, posição e tamanho, área útil sem a barra de tarefas, escala, taxa de atualização e modos de vídeo), `monitor(i)`, `refresh_rate(i)` e `form_monitor(form)` (em qual está a janela do form); `on_change` quando um monitor entra, sai ou muda de resolução. Em Rust, também direto: `ui.monitors()` e `ui.form_monitor(form)`. Para abrir num monitor, o `monitor` do form; para mudar, o `move_to_monitor` dele.

**Paleta:** Device · **componente não visual**

## Propriedades

Só as comuns.

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `Screen1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Screen", "name": "Screen1", "on_change": "screen1_change" }
```

E o handler, um método do form (`self.Screen1` já é um `Screen`):

```rust
impl FrmPrincipal {
    pub fn screen1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.Screen1;
    }
}
```
