# MediaCard

![MediaCard](img/MediaCard.png)

O cartão de um item de uma biblioteca de mídia (vídeos, fotos, gravações).

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `title` | Título | texto |
| `subtitle` | Subtítulo | texto |
| `duration` | Duração | texto |
| `badges` | Selos | texto |
| `icon` | Ícone | texto |
| `compact` | Compacto | lógico |
| `dimmed` | Apagado | lógico |
| `highlighted` | Destacado | lógico |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `selection_mode` | Modo de seleção | lógico |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `selected` | Escolhidos | lógico |
| `order` | Ordem | inteiro |
| `progress` | Progresso | número |
| `progress_text` | Texto do progresso | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_select` | — | — |  |
| `on_context` | — | — |  |

## Em código

No form, o `MediaCard1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MediaCard", "name": "MediaCard1", "on_select": "media_card1_select" }
```

E o handler, um método do form (`self.MediaCard1` já é um `MediaCard`):

```rust
impl FrmPrincipal {
    pub fn media_card1_select(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MediaCard1.title(ui);
        self.MediaCard1.set_title(ui, format!("{atual}!"));
    }
}
```
