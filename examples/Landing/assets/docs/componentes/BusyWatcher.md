# BusyWatcher

A espera pelo backend em outro processo (não visual, no form principal): o que a tela faz quando um handler do backend passa do `delay` sem terminar. Sem ele, o padrão é o cursor de espera e, depois de uns 3 s, um indicador discreto. Conveniência para quem não pôs o trabalho pesado numa thread (veja `docs/threads.md`). `on_busy_start` roda antes de mostrar (recusar, `ui.refuse("")`, pula o `busy_form`) e `on_busy_end` depois de esconder, do lado da tela (em Rust ou no próprio `busy_form`).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `delay` | Atraso | inteiro | Quanto esperar antes de mostrar, em milissegundos. |
| `action` | Ação | um de: `default`, `form`, `none` |  |
| `busy_form` | Form de espera | componente | O form mostrado com `action = form`. |
| `block_input` | Bloquear a entrada | lógico | Recusa o clique (e guarda as teclas) enquanto o backend está ocupado. |
| `min_visible` | Tempo mínimo à mostra | inteiro | O `busy_form` fica pelo menos isto à mostra (sem piscar), em milissegundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_busy_start` | — | — |  |
| `on_busy_end` | — | — |  |

## Em código

No form, o `BusyWatcher1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "BusyWatcher", "name": "BusyWatcher1", "on_busy_start": "busy_watcher1_busy_start" }
```

E o handler, um método do form (`self.BusyWatcher1` já é um `BusyWatcher`):

```rust
impl FrmPrincipal {
    pub fn busy_watcher1_busy_start(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.BusyWatcher1.delay(ui);
        self.BusyWatcher1.set_delay(ui, atual + 1);
    }
}
```
