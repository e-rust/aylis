# TabControl

![TabControl](img/TabControl.png)

Abas: os filhos são `TabItem`s, cada um uma página; só a de `tab_index` aparece. Clique numa aba (ou Ctrl+Tab com o foco) troca; `on_change` traz o índice. Sem `show_tabs`, só as páginas, sem barra nem moldura (como um `PageControl` com as abas escondidas): a tela tem a própria navegação e troca a página pelo `tab_index`; no designer aparece uma página por vez.

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `show_tabs` | Mostra as abas | lógico | Mostra a barra de abas e a moldura; desligado, só as páginas (a tela troca pelo `tab_index`). |
| `tab_position` | Posição das abas | um de: `top`, `bottom`, `left`, `right` | De que lado ficam as abas (`top`, `bottom`, `left`, `right`). |
| `tab_width` | Largura das abas | número | Largura da coluna de abas nas laterais (`left`/`right`); em cima e embaixo não conta. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tab_index` | Aba escolhida | inteiro | A página que aparece, a partir de 0 (a ordem dos `TabItem` filhos). |
| `transition` | Transição | um de: `none`, `fade`, `slide_left`, `slide_right`, `slide_up`, `slide_down` | Como a aba nova entra (`slide_left`/`slide_right` deslizam na direção da ordem das abas, `fade` esmaece). |
| `swipe` | Trocar de página deslizando | lógico | Deslizar o dedo para os lados troca de página (desligado: as páginas com mapa, grade ou controles que também arrastam para os lados não brigam com ele). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `TabControl1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "TabControl", "name": "TabControl1", "on_change": "tab_control1_change" }
```

E o handler, um método do form (`self.TabControl1` já é um `TabControl`):

```rust
impl FrmPrincipal {
    pub fn tab_control1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.TabControl1.tab_index(ui);
        self.TabControl1.set_tab_index(ui, atual + 1);
    }
}
```
