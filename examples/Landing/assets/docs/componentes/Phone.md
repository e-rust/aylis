# Phone

O telefone: `dial` (abre o discador com o número), `call` (liga direto; com `direct_call`, a publicação declara `CALL_PHONE`) e o estado das chamadas (`state`: `idle`, `ringing`, `dialing`, `active`, `holding`; `number`), com `watch_state` (declara `READ_PHONE_STATE` e `READ_CALL_LOG`, para o número). `on_call_state` a cada mudança.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `watch_state` | Acompanhar o estado | lógico |
| `direct_call` | Ligar direto | lógico |
| `dialer_app` | App de telefone | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_call_state` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `Phone1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Phone", "name": "Phone1", "on_call_state": "phone1_call_state" }
```

E o handler, um método do form (`self.Phone1` já é um `Phone`):

```rust
impl FrmPrincipal {
    pub fn phone1_call_state(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Phone1.watch_state(ui);
        self.Phone1.set_watch_state(ui, !atual);
    }
}
```
