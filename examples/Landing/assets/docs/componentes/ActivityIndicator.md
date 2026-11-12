# ActivityIndicator

![ActivityIndicator](img/ActivityIndicator.png)

Indicador de atividade girando. Só anima com `active`. `kind` escolhe o desenho (os spinners do Quasar: `dots`, `bars`, `audio`, `ios`...).

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `kind` | Tipo | um de: `ring`, `dots`, `bars`, `audio`, `ball`, `oval`, `puff`, `grid`, `ios`, `pie`, `tail`, `hourglass`, `rings`, `orbit` |  |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico |  |

## Em código

No form, o `ActivityIndicator1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "ActivityIndicator", "name": "ActivityIndicator1" }
```

E o handler, um método do form (`self.ActivityIndicator1` já é um `ActivityIndicator`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.ActivityIndicator1.active(ui);
        self.ActivityIndicator1.set_active(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
