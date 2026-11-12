# MediaCard

![MediaCard](img/MediaCard.png)

O cartão de um item de uma biblioteca de mídia (vídeos, fotos, gravações).

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto |  |
| `subtitle` | Subtítulo | texto |  |
| `duration` | Duração | texto |  |
| `badges` | Selos | texto |  |
| `icon` | Ícone | texto |  |
| `compact` | Compacto | lógico |  |
| `dimmed` | Apagado | lógico |  |
| `highlighted` | Destacado | lógico | Um alvo de soltar marcado (a pasta sob o item arrastado). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `selection_mode` | Modo de seleção | lógico |  |
| `editable_title` | Nome editável | lógico | O clique no nome abre o campo de renomear (em vez do `on_click`); o nome novo vem no `on_rename`. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `selected` | Escolhidos | lógico |  |
| `order` | Ordem | inteiro |  |
| `progress` | Progresso | número |  |
| `progress_text` | Texto do progresso | texto |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_select` | — | — |  |
| `on_context` | — | — |  |
| `on_rename` | — | — |  |

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

## Mais sobre o MediaCard

### Duas áreas de clique

A caixa de seleção dispara `on_select`; o resto do card, `on_click`. Com `editable_title`,
o nome vira uma terceira área: o clique nele abre o campo de renomear em vez do `on_click`
(a miniatura continua abrindo o item).

### editable_title

O card cuida sozinho do campo: abre sobre o nome, com ele todo selecionado. Enter, Tab ou
o clique fora grava no `title` e dispara `on_rename` com o nome novo (sem espaços nas
pontas; vazio ou igual ao atual não dispara); Esc cancela. O app só grava o nome:

```rust
pub fn card_rename(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) -> Result<()> {
    let novo = args.text().unwrap_or_default();
    biblioteca::renomear(&self.item_atual, novo)?;
    Ok(())
}
```

Se a gravação falhar, devolva o erro e ponha o `title` antigo de volta.

### rename

O método `rename()` abre o campo pelo código, mesmo sem `editable_title`: é o F2 do app.
