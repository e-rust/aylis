# AppBar

![AppBar](img/AppBar.png)

Barra do topo (a "top app bar" do Material 3, a "navigation bar" do iOS): título, botão de navegação à esquerda (`nav_icon`: `arrow-left`, `menu`; no iOS com `back_text`) e ações à direita (`actions`: ícones separados por `;`). `on_nav` e `on_action` (índice da ação).

**Paleta:** Layouts · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O título da barra. |
| `subtitle` | Subtítulo | texto | A linha menor embaixo do título; vazio: sem subtítulo. |
| `size` | Tamanho | um de: `small`, `center`, `medium`, `large` | O tamanho: `small` e `center` numa linha, `medium` e `large` com o título grande embaixo. |
| `nav_icon` | Ícone de navegação | texto | O ícone do botão de navegação à esquerda (`arrow-left`, `menu`); vazio: sem botão. |
| `back_text` | Texto do voltar | texto | No iOS, o texto ao lado da seta de voltar (`Voltar`); vazio: só a seta. |
| `actions` | Ações | texto | Os ícones das ações à direita, separados por `;` (`search;menu`); o clique dispara `on_action` com o índice. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_nav` | — | — |  |
| `on_action` | — | — |  |

## Em código

No form, o `AppBar1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AppBar", "name": "AppBar1", "on_nav": "app_bar1_nav" }
```

E o handler, um método do form (`self.AppBar1` já é um `AppBar`):

```rust
impl FrmPrincipal {
    pub fn app_bar1_nav(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.AppBar1.title(ui);
        self.AppBar1.set_title(ui, format!("{atual}!"));
    }
}
```
