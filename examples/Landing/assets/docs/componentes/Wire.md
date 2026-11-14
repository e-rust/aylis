# Wire

![Wire](img/Wire.png)

Fio elétrico preso a dois componentes (como o `Pipe`, fino); `energized` o pinta na cor de energizado.

**Paleta:** Electrical · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `color` | Cor | cor | A cor do fio desenergizado. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `from` | De | componente | O componente onde o fio começa (vazio: um fio reto no próprio retângulo). |
| `from_port` | Lado de saída | um de: `left`, `right`, `top`, `bottom` | O lado do componente de origem onde o fio se prende. |
| `to` | Para | componente | O componente onde o fio termina (vazio: um fio reto no próprio retângulo). |
| `to_port` | Lado de chegada | um de: `left`, `right`, `top`, `bottom` | O lado do componente de destino onde o fio se prende. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `energized` | Energizado | lógico | Energizado: o fio fica laranja, no lugar da `color`. |

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
