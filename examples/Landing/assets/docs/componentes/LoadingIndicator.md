# LoadingIndicator

![LoadingIndicator](img/LoadingIndicator.png)

O indicador de carregamento do Material 3 Expressive: uma forma que gira e se transforma (flor, biscoito, pentágono, pílula, sol...). `contained` põe um círculo de fundo.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `contained` | Com fundo | lógico | Põe um círculo de fundo atrás da forma. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Animando; desligado, a forma fica parada. |

## Em código

No form, o `LoadingIndicator1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "LoadingIndicator", "name": "LoadingIndicator1" }
```

E o handler, um método do form (`self.LoadingIndicator1` já é um `LoadingIndicator`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.LoadingIndicator1.contained(ui);
        self.LoadingIndicator1.set_contained(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
