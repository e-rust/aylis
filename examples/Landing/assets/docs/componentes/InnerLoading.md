# InnerLoading

![InnerLoading](img/InnerLoading.png)

O carregando por cima (`QInnerLoading`): cobre o contêiner onde está (ocupa a área toda) com um véu e um spinner enquanto `showing`; bloqueia os cliques embaixo. Escondido, não aparece nem atrapalha.

**Paleta:** Quasar · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `kind` | Tipo | um de: `ring`, `dots`, `bars`, `audio`, `ball`, `oval`, `puff`, `grid`, `ios`, `pie`, `tail`, `hourglass`, `rings`, `orbit` |
| `label` | Rótulo | texto |
| `size` | Tamanho | número |
| `dim` | Escurecer | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `showing` | Mostrando | lógico |

## Em código

No form, o `InnerLoading1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "InnerLoading", "name": "InnerLoading1" }
```

E o handler, um método do form (`self.InnerLoading1` já é um `InnerLoading`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.InnerLoading1.showing(ui);
        self.InnerLoading1.set_showing(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
