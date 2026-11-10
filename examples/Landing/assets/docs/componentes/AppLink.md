# AppLink

Fala com outros apps: abre um endereço (`https:`, `mailto:`, `tel:`, `geo:`) ou outro app, compartilha texto e arquivos pela folha do sistema e pede algo a outro app (`request`, com a resposta em `on_result`). Também recebe o que chega de fora: `on_link` (o link que abriu o app, dos `deep_links` do projeto) e `on_shared` (o que outro app compartilhou com este: o texto, ou os arquivos separados por `;`). Com o `on_shared` num form, a publicação do Android põe o app na lista de compartilhar.

**Paleta:** System · **componente não visual**

## Propriedades

Só as comuns.

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_link` | — | — |  |
| `on_shared` | — | — |  |
| `on_result` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `AppLink1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AppLink", "name": "AppLink1", "on_link": "app_link1_link" }
```

E o handler, um método do form (`self.AppLink1` já é um `AppLink`):

```rust
impl FrmPrincipal {
    pub fn app_link1_link(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.AppLink1;
    }
}
```
