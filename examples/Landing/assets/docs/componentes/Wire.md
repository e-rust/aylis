# Wire

![Wire](img/Wire.png)

Fio elétrico preso a dois componentes (como o `Pipe`, fino); `energized` o pinta na cor de energizado.

**Paleta:** Electrical · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `color` | Cor | cor |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `from` | De | componente |
| `from_port` | Lado de saída | um de: `left`, `right`, `top`, `bottom` |
| `to` | Para | componente |
| `to_port` | Lado de chegada | um de: `left`, `right`, `top`, `bottom` |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `energized` | Energizado | lógico |

## Em código

No form, o `Wire1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Wire", "name": "Wire1" }
```

E o handler, um método do form (`self.Wire1` já é um `Wire`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Wire1.energized(ui);
        self.Wire1.set_energized(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
