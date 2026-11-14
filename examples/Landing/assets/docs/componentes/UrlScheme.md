# UrlScheme

Um prefixo de URL próprio (meuapp://) que abre o app e entrega o endereço no on_link.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `scheme` | Prefixo de URL | texto | O prefixo, sem `://` (`meuapp`): letras, números, `+`, `-` e `.`. |
| `description` | Descrição | texto | O nome que o sistema mostra ao perguntar com que app abrir. |
| `active` | Ativo | lógico | Registra o `scheme` no sistema e passa a receber os links dele no `on_link`. |
| `bring_to_front` | Trazer a janela para a frente | lógico | Trazer a janela do form para a frente quando um link chega com o app aberto. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_link` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `UrlScheme1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "UrlScheme", "name": "UrlScheme1", "on_link": "url_scheme1_link" }
```

E o handler, um método do form (`self.UrlScheme1` já é um `UrlScheme`):

```rust
impl FrmPrincipal {
    pub fn url_scheme1_link(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.UrlScheme1.scheme(ui);
        self.UrlScheme1.set_scheme(ui, format!("{atual}!"));
    }
}
```
