# AvatarGroup

![AvatarGroup](img/AvatarGroup.png)

Avatares empilhados (o `v-avatar-group`): as iniciais de `names` (separados por `;`) sobrepostas; passando de `max`, o último mostra "+N".

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `max` | Máximo | inteiro | Quantos avatares aparecem; os que passarem viram o "+N" no último. |
| `size` | Tamanho | número | O diâmetro de cada avatar, em pontos. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `names` | Nomes | lista de textos (separados por `;`) | Os nomes das pessoas, separados por `;`: cada avatar mostra as iniciais. |

## Em código

No form, o `AvatarGroup1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AvatarGroup", "name": "AvatarGroup1" }
```

E o handler, um método do form (`self.AvatarGroup1` já é um `AvatarGroup`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.AvatarGroup1.max(ui);
        self.AvatarGroup1.set_max(ui, atual + 1);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
