# Label

![Label](img/Label.png)

Texto fixo na tela: um título, uma legenda, a explicação de um campo. Quebra linhas (`wrap`), cresce com o texto (`auto_size`) e segue a fonte e a cor do tema.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `text` | Texto | texto |
| `h_align` | Alinhamento horizontal | um de: `leading`, `center`, `trailing`, `justify` |
| `v_align` | Alinhamento vertical | um de: `leading`, `center`, `trailing`, `justify` |
| `wrap` | Quebra de linha | lógico |
| `trimming` | Reticências | um de: `none`, `character`, `word` |
| `bold` | Negrito | lógico |
| `color` | Cor | cor |
| `highlight` | Destacar | texto |
| `letter_spacing` | Espaço entre letras | número |
| `text_case` | Maiúsculas/minúsculas | um de: `none`, `upper`, `lower` |
| `markup` | Formatação no texto | lógico |
| `outline` | Contorno | cor |
| `outline_width` | Largura do contorno | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `focus_control` | Controle do foco | componente |

## Em código

No form, o `Label1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Label", "name": "Label1" }
```

E o handler, um método do form (`self.Label1` já é um `Label`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Label1.text(ui);
        self.Label1.set_text(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
