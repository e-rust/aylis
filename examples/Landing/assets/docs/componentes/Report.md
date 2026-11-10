# Report

![Report](img/Report.png)

A página do relatório. Tamanho em pontos (1/72"): A4 = 595 × 842; `landscape` deita a página. As bandas ficam dentro dela, de cima para baixo, na largura da área útil (a página menos as margens).

**Paleta:** Reports · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `title` | Título | texto |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `data_source` | Fonte de dados | componente |
| `provider` | Provedor | componente |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `page_width` | Largura da página | número |
| `page_height` | Altura da página | número |
| `landscape` | Paisagem | lógico |
| `margin` | Margem | número |

## Em código

No form, o `Report1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Report", "name": "Report1" }
```

E o handler, um método do form (`self.Report1` já é um `Report`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Report1.title(ui);
        self.Report1.set_title(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
