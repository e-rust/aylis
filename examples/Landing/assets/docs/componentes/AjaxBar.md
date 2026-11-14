# AjaxBar

![AjaxBar](img/AjaxBar.png)

A barra de carregamento do topo (`QAjaxBar`): fina, na borda de cima (ou de baixo) da janela. Ligando `active`, ela anda devagar até perto do fim (sem saber quanto falta); desligando, completa e some.

**Paleta:** Quasar · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Ligado: a barra aparece e anda devagar; desligado: completa e some. |

## Em código

No form, o `AjaxBar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AjaxBar", "name": "AjaxBar1" }
```

E o handler, um método do form (`self.AjaxBar1` já é um `AjaxBar`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.AjaxBar1.active(ui);
        self.AjaxBar1.set_active(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
