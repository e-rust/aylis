# MenuItem

Item de um menu (texto, ícone, atalho, marca); dentro de outro vira submenu.

**Paleta:** Menus · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `text` | Texto | texto |  |
| `checked` | Marcado | lógico |  |
| `icon` | Ícone | texto | Ícone à esquerda do texto (`"save"`, `"pacote:nome"`); marcado, aparece o ✓. |
| `danger` | Perigoso | lógico | Ação destrutiva ("Excluir"): texto na cor `danger_text` (ou `danger`) do tema e, sob o mouse, fundo `danger` com texto branco. |
| `header` | Linha de títulos | lógico | Título de uma seção do menu (a cabeça de um menu de contexto: o nome do item clicado): texto fraco em negrito, sem clique nem realce. O relatório do Tabernáculo, C15. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `shortcut` | Atalho | texto | Atalho: `"Ctrl+S"`, `"Ctrl+Shift+N"`, `"F9"`, `"Delete"`... |
| `enabled` | Habilitado | lógico |  |
| `auto_check` | Marca sozinho | lógico | Clicar alterna `checked` sozinho (como o `AutoCheck` do Delphi). |
| `visible` | Visível | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |

## Em código

No form, o `MenuItem1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MenuItem", "name": "MenuItem1", "on_click": "menu_item1_click" }
```

E o handler, um método do form (`self.MenuItem1` já é um `MenuItem`):

```rust
impl FrmPrincipal {
    pub fn menu_item1_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MenuItem1.text(ui);
        self.MenuItem1.set_text(ui, format!("{atual}!"));
    }
}
```
