# Chip

![Chip](img/Chip.png)

Etiqueta compacta (o `v-chip`): texto, ícone opcional, marcável (`checkable`) e com botão de fechar (`closable`, evento `on_close`). Variantes: `outline`, `success`...

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `text` | Texto | texto | O texto da etiqueta. |
| `icon` | Ícone | texto | Ícone pelo nome, antes do texto; vazio: sem ícone. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `closable` | Pode fechar | lógico | Mostra o "×" de fechar, que dispara o `on_close`. |
| `checkable` | Marcável | lógico | Cada clique alterna o `checked` (e dispara o `on_change`). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `checked` | Marcado | lógico | Marcado, com `checkable`. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_close` | o form fechou | — |  |
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `Chip1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Chip", "name": "Chip1", "on_close": "chip1_close" }
```

E o handler, um método do form (`self.Chip1` já é um `Chip`):

```rust
impl FrmPrincipal {
    pub fn chip1_close(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Chip1.text(ui);
        self.Chip1.set_text(ui, format!("{atual}!"));
    }
}
```
