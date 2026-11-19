# SkeletonLoader

![SkeletonLoader](img/SkeletonLoader.png)

Esqueleto do conteúdo enquanto carrega (o `v-skeleton-loader`), com brilho passando. `layout` é a lista das peças, de cima para baixo: `avatar`, `heading`, `text`, `image`, `button`, `chip`, `list-item` (avatar e duas linhas), `card` (imagem, título e texto), `table-row`. Repita a peça para ter mais de uma (`text;text;text`: três linhas).

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `layout` | Disposição | lista de textos (separados por `;`) | As peças do esqueleto, de cima para baixo (`avatar;heading;text;text`, `card`, `list-item`, `table-row`...). |
| `animated` | Animado | lógico | Uma faixa de brilho passa pelas peças, mostrando que está carregando. |

## Em código

No form, o `SkeletonLoader1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "SkeletonLoader", "name": "SkeletonLoader1" }
```

E o handler, um método do form (`self.SkeletonLoader1` já é um `SkeletonLoader`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.SkeletonLoader1.animated(ui);
        self.SkeletonLoader1.set_animated(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
