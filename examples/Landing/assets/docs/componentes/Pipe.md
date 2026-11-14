# Pipe

![Pipe](img/Pipe.png)

Cano preso a dois componentes (`from`/`to` e o lado de cada um): segue os dois e faz os cotovelos; `running` mostra o fluxo andando na cor do produto.

**Paleta:** Industrial · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `flow_color` | Cor do fluxo | cor | A cor do produto que corre no cano. |
| `thickness` | Espessura | número | A grossura do cano, em pontos (mínimo 2). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `from` | De | componente | O componente onde o cano começa (vazio: um cano reto no próprio retângulo). |
| `from_port` | Lado de saída | um de: `left`, `right`, `top`, `bottom` | O lado do componente de origem onde o cano se prende. |
| `to` | Para | componente | O componente onde o cano termina (vazio: um cano reto no próprio retângulo). |
| `to_port` | Lado de chegada | um de: `left`, `right`, `top`, `bottom` | O lado do componente de destino onde o cano se prende. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `running` | Ligado | lógico | Mostra o fluxo andando dentro do cano. |
| `alarm` | Alarme | lógico | Em alarme: o fluxo fica na cor de perigo. |

## Em código

No form, o `Pipe1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Pipe", "name": "Pipe1" }
```

E o handler, um método do form (`self.Pipe1` já é um `Pipe`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Pipe1.running(ui);
        self.Pipe1.set_running(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
