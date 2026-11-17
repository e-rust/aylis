# MenuItem

Item de um menu (texto, ícone, atalho, marca); dentro de outro vira submenu.

**Paleta:** Menus · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `text` | Texto | texto | O texto do item; `"-"` faz dele um separador. |
| `checked` | Marcado | lógico | Marcado: mostra o ✓ à esquerda. |
| `icon` | Ícone | texto | Ícone à esquerda do texto (`"save"`, `"pacote:nome"`); marcado, aparece o ✓. |
| `danger` | Perigoso | lógico | Ação destrutiva ("Excluir"): texto na cor `danger_text` (ou `danger`) do tema e, sob o mouse, fundo `danger` com texto branco. |
| `header` | Linha de títulos | lógico | Título de uma seção do menu (a cabeça de um menu de contexto: o nome do item clicado): texto fraco em negrito, sem clique nem realce. O relatório do Tabernáculo, C15. |
| `content` | Conteúdo | componente | Um controle do form mostrado no item enquanto o menu está aberto (botões de ativar e desativar, um `Frame` desenhado à vontade): à frente do texto, no lado direito, ou no item inteiro quando o texto está vazio. A linha cresce até a altura dele. Deixe-o invisível no form: o menu o mostra ao abrir e o devolve como estava ao fechar. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `shortcut` | Atalho | texto | Atalho: `"Ctrl+S"`, `"Ctrl+Shift+N"`, `"F9"`, `"Delete"`... |
| `enabled` | Habilitado | lógico | Aceita clique e atalho; desligado, aparece apagado. |
| `auto_check` | Marca sozinho | lógico | Clicar alterna `checked` sozinho. |
| `visible` | Visível | lógico | Aparece no menu; escondido, some (e o atalho também não vale). |

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
