# Fan

![Fan](img/Fan.png)

Ventilador de supervisório: gira quando ligado e muda de cor no alarme.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `label` | Rótulo | texto | O texto do equipamento: no corpo do motor (vazio: "M") e embaixo da bomba. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `speed` | Velocidade | número | Voltas por segundo do desenho, ligado. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `running` | Ligado | lógico | Ligado: gira e fica verde; desligado, parado e cinza. |
| `alarm` | Alarme | lógico | Em alarme: fica vermelho e o contorno pisca. |

## Em código

No form, o `Fan1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Fan", "name": "Fan1" }
```

E o handler, um método do form (`self.Fan1` já é um `Fan`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Fan1.running(ui);
        self.Fan1.set_running(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
