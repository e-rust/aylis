# GroupBox

![GroupBox](img/GroupBox.png)

Moldura com título que agrupa controles.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caption` | Título | texto | O título; com `&` antes de uma letra, Alt+letra leva ao primeiro campo do grupo. |

### Teclado

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `allowed_chars` | Caracteres permitidos | texto | Os caracteres dos campos dentro do grupo (veja o `Form`). |
| `blocked_chars` | Caracteres bloqueados | texto | Os caracteres que os campos do grupo não aceitam (digitados ou colados); faixas como `a-z`. Vazio: nenhum. |
| `only_allowed` | Só os permitidos | lógico | Só os caracteres de `allowed_chars` entram nos campos do grupo. |
| `chars_override` | Força os caracteres | lógico | O filtro de caracteres do grupo manda nos campos de dentro (sem ele, o do campo decide primeiro). |
| `char_map` | Trocas de caracteres | texto | Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a mesma herança dos filtros. |

## Em código

No form, o `GroupBox1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "GroupBox", "name": "GroupBox1" }
```

E o handler, um método do form (`self.GroupBox1` já é um `GroupBox`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.GroupBox1.caption(ui);
        self.GroupBox1.set_caption(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
