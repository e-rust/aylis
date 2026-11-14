# Report

![Report](img/Report.png)

A página do relatório. Tamanho em pontos (1/72"): A4 = 595 × 842; `landscape` deita a página. As bandas ficam dentro dela, de cima para baixo, na largura da área útil (a página menos as margens).

**Paleta:** Reports · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O nome do documento (no PDF e na fila da impressora). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data_source` | Fonte de dados | componente | O `DataSource` dos registros: a banda de detalhe se repete uma vez por registro. |
| `provider` | Provedor | componente | Quem gera o PDF (um provedor de PDF, item 25); sem ele, o PDF do próprio Aylis. |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `page_width` | Largura da página | número | Largura da página em pontos (1/72"; A4 = 595). |
| `page_height` | Altura da página | número | Altura da página em pontos (1/72"; A4 = 842). |
| `landscape` | Paisagem | lógico | Deita a página (troca a largura pela altura). |
| `margin` | Margem | número | A margem dos quatro lados da página, em pontos. |

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
