# Permissions

Consulta e pede as permissões do aparelho (câmera, microfone, localização, notificações, fotos, contatos, agenda, bluetooth).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `permissions` | Permissões | texto |
| `reasons` | Motivos | texto |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_result` | — | — |  |

## Em código

No form, o `Permissions1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Permissions", "name": "Permissions1", "on_result": "permissions1_result" }
```

E o handler, um método do form (`self.Permissions1` já é um `Permissions`):

```rust
impl FrmPrincipal {
    pub fn permissions1_result(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Permissions1.permissions(ui);
        self.Permissions1.set_permissions(ui, format!("{atual}!"));
    }
}
```
