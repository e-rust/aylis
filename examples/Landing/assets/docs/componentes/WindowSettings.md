# WindowSettings

Configura a janela do form em que está (ou, com `scope` = `app`, as de todos os forms): animação, captura, clique, botões da barra, fundo e cores do sistema, sombra, desfoque, foco e Alt+Tab. A plataforma aplica ao abrir a janela e a cada troca em execução (a janela recebe as opções de novo); no designer, nada muda na IDE. Lido pela plataforma em `aylis_core::Ui::window_options`.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `backdrop` | Fundo do sistema | um de: `system`, `none`, `mica`, `acrylic`, `tabbed` | O fundo que o Windows 11 desenha por trás do conteúdo: `system`, `none`, `mica`, `acrylic` ou `tabbed` (aparece onde o form não pinta fundo). |
| `title_color` | Cor da barra de título | cor | A cor da barra de título do sistema (Windows 11; vazio: a do sistema). |
| `title_text_color` | Cor do texto do título | cor | A cor do texto da barra de título do sistema (Windows 11; vazio: a do sistema). |
| `border_color` | Cor da borda | cor | A cor da borda da janela (Windows 11; vazio: a do sistema). |
| `shadow` | Sombra | lógico | A sombra do Windows na janela sem borda (`border_style` = `none`). |
| `blur` | Desfoque | lógico | O que está atrás da janela sai desfocado (macOS e Linux com KDE). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `scope` | Alcance | um de: `form`, `app` | A quem vale: `form` (a janela do form em que está) ou `app` (todas as janelas; o de um form ganha deste). |
| `animation` | Animação | lógico | A animação do Windows ao abrir, minimizar e restaurar (`false`: aparece e some na hora, como o editor de um print sobre a tela congelada). |
| `capture_excluded` | Fora das capturas | lógico | Fora das capturas de tela e das gravações (Windows e macOS): a janela sai preta ou some no que outro programa grava. |
| `click_through` | Clique atravessa | lógico | O mouse atravessa a janela: os cliques vão para o que está atrás (um HUD por cima). |
| `minimize_button` | Botão minimizar | lógico | O botão minimizar da barra do sistema (Windows e macOS). |
| `maximize_button` | Botão maximizar | lógico | O botão maximizar da barra do sistema (Windows e macOS). |
| `no_activate` | Não pega o foco | lógico | Clicar na janela não tira o foco de onde ele está (Windows): um teclado na tela, uma paleta flutuante. |
| `alt_tab` | Aparece no Alt+Tab | lógico | A janela aparece no Alt+Tab (Windows; desligado, também sai da barra de tarefas). |

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro | Um número livre para o código (não muda nada). |

## Em código

No form, o `WindowSettings1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "WindowSettings", "name": "WindowSettings1" }
```

E o handler, um método do form (`self.WindowSettings1` já é um `WindowSettings`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.WindowSettings1.animation(ui);
        self.WindowSettings1.set_animation(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
