# KeyboardCapture

Com `active` ligado, as teclas que o sistema guarda para si vão para a janela do form deste componente enquanto ela está em primeiro plano: a tecla Windows/Super (sozinha e com outra, como Win+R), o Alt+Tab, o Alt+Esc, o Ctrl+Esc, o Ctrl+Shift+Esc, o Alt+Espaço, o PrintScreen e, com `alt_f4`, o Alt+F4 (sem ele, o Alt+F4 fecha a janela como sempre). Elas chegam pelos eventos de tecla de sempre (`on_key_down`/`on_key_up` do form e do controle com foco). Ao perder o foco, ao desligar `active` ou ao liberar o componente, solta tudo.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Prende as teclas do sistema enquanto a janela do form está em foco. |
| `alt_f4` | Capturar o Alt+F4 | lógico | O Alt+F4 também vai para o app (sem isso, fecha a janela). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_capture_change` | o teclado foi preso (`true`: a janela ganhou o foco) ou solto (`false`) | — |  |

## Em código

No form, o `KeyboardCapture1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "KeyboardCapture", "name": "KeyboardCapture1", "on_capture_change": "keyboard_capture1_capture_change" }
```

E o handler, um método do form (`self.KeyboardCapture1` já é um `KeyboardCapture`):

```rust
impl FrmPrincipal {
    pub fn keyboard_capture1_capture_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.KeyboardCapture1.active(ui);
        self.KeyboardCapture1.set_active(ui, !atual);
    }
}
```
