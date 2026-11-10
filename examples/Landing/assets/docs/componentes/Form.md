# Form

Janela. Pinta o fundo com o estilo `Form` do tema.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `caption` | Título | texto |
| `border_style` | Estilo da borda | um de: `sizeable`, `single`, `none`, `custom` |
| `zoom` | Zoom | número |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `transition` | Transição | um de: `none`, `fade`, `slide_left`, `slide_right`, `slide_up`, `slide_down` |
| `topmost` | Sempre na frente | lógico |
| `show_in_taskbar` | Na barra de tarefas | lógico |
| `active_control` | Controle ativo | componente |
| `tab_order` | Ordem de tabulação | texto |
| `remember_placement` | Lembrar a posição | lógico |

### Teclado

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `tab_keys` | Teclas de avançar campo | texto |
| `back_tab_keys` | Teclas de voltar campo | texto |
| `allowed_chars` | Caracteres permitidos | texto |
| `blocked_chars` | Caracteres bloqueados | texto |
| `only_allowed` | Só os permitidos | lógico |
| `chars_override` | Força os caracteres | lógico |
| `char_map` | Trocas de caracteres | texto |
| `key_preview` | Recebe as teclas antes | lógico |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `position` | Posição | um de: `center`, `screen_center`, `designed`, `default` |
| `window_state` | Estado da janela | um de: `normal`, `maximized`, `minimized`, `full_screen` |
| `monitor` | Retorno | inteiro |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_show` | o form abriu | — |  |
| `on_close` | o form fechou | — |  |
| `on_close_query` | o form vai fechar: `Err` recusa (o X não fecha) | — | sim: `can_close` |
| `on_activate` | — | — |  |
| `on_deactivate` | — | — |  |
| `on_navigate` | — | — |  |

## Em código

No form, o `Form1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Form", "name": "Form1", "on_show": "form1_show" }
```

E o handler, um método do form (`self.Form1` já é um `Form`):

```rust
impl FrmPrincipal {
    pub fn form1_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Form1.caption(ui);
        self.Form1.set_caption(ui, format!("{atual}!"));
    }
}
```
