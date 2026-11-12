# WakeAppAction

Abre o app por cima da tela bloqueada e acende a tela, tocando, como uma chamada chegando: `execute(dados)` mostra a notificação de tela cheia com `title` e `text` (modelos: `{caller}` vira o valor da chave nos dados) e os botões (`buttons`, `id:Texto;...`); os de `open_buttons` abrem o app (`*`: todos), os outros respondem sem abrir. Com a tela bloqueada ou apagada, o app abre direto no `form`.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `form` | Form | texto |  |
| `title` | Título | texto |  |
| `text` | Texto | texto |  |
| `buttons` | Botões | texto |  |
| `open_buttons` | Botões que abrem o app | texto |  |
| `ringtone` | Toque | um de: `default`, `notification`, `alarm`, `none` |  |
| `vibrate` | Vibrar | lógico |  |
| `timeout` | Tempo para sumir | número | Segundos até desistir (0: sem limite). |
| `call_style` | Estilo de chamada | lógico |  |
| `turn_screen_on` | Acender a tela | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_open` | — | — |  |
| `on_button` | — | — |  |
| `on_timeout` | — | — |  |

## Em código

No form, o `WakeAppAction1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "WakeAppAction", "name": "WakeAppAction1", "on_open": "wake_app_action1_open" }
```

E o handler, um método do form (`self.WakeAppAction1` já é um `WakeAppAction`):

```rust
impl FrmPrincipal {
    pub fn wake_app_action1_open(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.WakeAppAction1.form(ui);
        self.WakeAppAction1.set_form(ui, format!("{atual}!"));
    }
}
```
