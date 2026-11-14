# Analytics

Métricas de uso somadas entre as sessões (cliques, telas, erros).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Mede: ligado (o padrão), começa sozinho ao entrar na tela; desligar para de medir. |
| `file` | Arquivo | texto | O arquivo; vazio: `<pasta do app>/analytics.json`. |
| `app_name` | Nome do aplicativo | texto | O nome do app (a pasta do arquivo); vazio: o título do form principal. |
| `performance` | Mede o desempenho (handlers e quadros) | lógico | Mede também o tempo dos handlers e os quadros lentos (desligado por padrão). |

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
