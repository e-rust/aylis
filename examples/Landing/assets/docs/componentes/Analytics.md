# Analytics

Métricas de uso somadas entre as sessões (cliques, telas, erros).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico |  |
| `file` | Arquivo | texto | O arquivo; vazio: `<pasta do app>/analytics.json`. |
| `app_name` | Nome do aplicativo | texto |  |

## Em código

No form, o `Analytics1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Analytics", "name": "Analytics1" }
```

E o handler, um método do form (`self.Analytics1` já é um `Analytics`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Analytics1.active(ui);
        self.Analytics1.set_active(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
