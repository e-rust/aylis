# AylisHttpProvider

O provedor HTTP do Aylis: o `ureq` (rustls) fora do navegador, o `fetch` dentro. É o padrão do `HttpClient` sem provedor ligado; ligue um para mudar o `user_agent`.

**Paleta:** Providers · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `user_agent` | Identificação do cliente (User-Agent) | texto | O `User-Agent` dos pedidos (vazio: `aylis/<versão>`); no navegador quem manda é ele. |

## Em código

No form, o `AylisHttpProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AylisHttpProvider", "name": "AylisHttpProvider1" }
```

E o handler, um método do form (`self.AylisHttpProvider1` já é um `AylisHttpProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.AylisHttpProvider1.user_agent(ui);
        self.AylisHttpProvider1.set_user_agent(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
