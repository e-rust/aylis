# Button

![Button](img/Button.png)

Botão que dispara uma ação: texto, ícone opcional (à esquerda, à direita ou em cima) e os estilos do tema (contorno, plano, flutuante...). Pode ficar marcado (`checkable`) e, num diálogo, fechar o form com um `modal_result`.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `text` | Texto | texto |
| `icon` | Ícone | texto |
| `checked_icon` | Ícone marcado | texto |
| `icon_position` | Posição do ícone | um de: `left`, `right`, `top` |
| `icon_size` | Tamanho do ícone | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `checkable` | Marcável | lógico |
| `modal_result` | Resultado do modal | um de: `none`, `ok`, `cancel`, `abort`, `retry`, `ignore`, `yes`, `no`, `close` |
| `repeat_enter` | Repetir com Enter | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `checked` | Marcado | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |

## Em código

No form, o `Button1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Button", "name": "Button1", "on_change": "button1_change" }
```

E o handler, um método do form (`self.Button1` já é um `Button`):

```rust
impl FrmPrincipal {
    pub fn button1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Button1.text(ui);
        self.Button1.set_text(ui, format!("{atual}!"));
    }
}
```
