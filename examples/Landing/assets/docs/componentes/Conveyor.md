# Conveyor

![Conveyor](img/Conveyor.png)

Esteira: a correia com os rolos girando e as marcas andando quando ligada.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `speed` | Velocidade | número |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `running` | Ligado | lógico |
| `alarm` | Alarme | lógico |

## Em código

No form, o `Conveyor1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Conveyor", "name": "Conveyor1" }
```

E o handler, um método do form (`self.Conveyor1` já é um `Conveyor`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Conveyor1.running(ui);
        self.Conveyor1.set_running(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
