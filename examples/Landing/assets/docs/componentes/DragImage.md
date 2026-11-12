# DragImage

A imagem que acompanha o mouse durante um arrastar e soltar.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `image` | Imagem | texto | Arquivo do projeto (`assets/images/arquivo.png`). |
| `control` | Controle desenhado | componente | O controle cujo desenho vai junto (vazio: o próprio controle arrastado, quando não há imagem). |
| `opacity` | Opacidade | número | 0..1. |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `width` | Largura | número | Tamanho na tela, em pontos; 0 = automático (160 × 90 numa imagem, o tamanho do controle num desenho). |
| `height` | Altura | número |  |
| `hot_x` | Ponto no cursor (X) | número | O ponto da imagem (em pontos) que fica no cursor; negativo = o padrão (no meio, um pouco acima: como o Explorer). |
| `hot_y` | Ponto no cursor (Y) | número |  |

## Em código

No form, o `DragImage1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DragImage", "name": "DragImage1" }
```

E o handler, um método do form (`self.DragImage1` já é um `DragImage`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DragImage1.image(ui);
        self.DragImage1.set_image(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
