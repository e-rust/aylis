# Form

Janela. Pinta o fundo com o estilo `Form` do tema.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caption` | Título | texto |  |
| `border_style` | Estilo da borda | um de: `sizeable`, `single`, `none`, `custom` |  |
| `window_corners` | Cantos da janela | um de: `default`, `round`, `small`, `square` | Os cantos da janela (Windows 11 e macOS; nas outras, sem efeito): útil com a barra de título própria (`border_style = custom`), que fica com os cantos de uma janela comum. |
| `zoom` | Zoom | número | O zoom deste form (1 = normal), vezes o do app (`Ui::set_zoom`): tudo nele fica maior ou menor e o layout se refaz no espaço da janela. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `transition` | Transição | um de: `none`, `fade`, `slide_left`, `slide_right`, `slide_up`, `slide_down` | Celular e navegador (os forms se empilham): como o form entra ao abrir e sai ao fechar (`slide_left` como uma tela nova do Android/iOS, `slide_up` como uma folha, `fade`). No desktop cada form é uma janela e isto não vale. |
| `topmost` | Sempre na frente | lógico | Sempre por cima das outras janelas (o `FormStyle := fsStayOnTop`). |
| `show_in_taskbar` | Na barra de tarefas | lógico | Aparece na barra de tarefas (desligue para menus e janelas de ferramenta). |
| `active_control` | Controle ativo | componente | O controle que recebe o foco quando o form abre (como o `ActiveControl`). |
| `tab_order` | Ordem de tabulação | texto | A ordem do Tab no form inteiro: nomes separados por `;` (`edtNome;edtEmail;btnSalvar`). Pula entre níveis (de um campo num painel para outro num grupo); os controles fora da lista vêm depois, na ordem natural. Vazio: a ordem natural (a da árvore). |
| `remember_placement` | Lembrar a posição | lógico | Lembra onde a janela estava (o monitor, a posição, o tamanho e se estava maximizada) e reabre igual, num arquivo do app; se aquele monitor sumiu, abre como se não houvesse. |
| `gpu` | Placa de vídeo | texto | A placa de vídeo da janela (vazio: a do projeto): `monitor` troca para a placa do monitor em que ela está (Windows e Linux com Wayland); `alto-desempenho`, `economia` ou parte do nome fixam uma. |
| `activate_on_show` | Ativar ao mostrar | lógico | Mostrar a janela dá o foco a ela (desligado: aparece sem tirar o foco de quem tem, como uma projeção que não deve roubar o teclado do console). |
| `close_action` | Ao fechar | um de: `hide`, `free` | O que fechar faz com o form (o `Action` do `OnClose` do Delphi): esconde (pode abrir de novo) ou libera (some da memória, depois de fechar). |
| `frame_rate` | Quadros por segundo | número | Quadros por segundo da janela (0 = a taxa do monitor dela). Um telão de 60 Hz ao lado de um monitor de 144 Hz não precisa acompanhar o mais rápido. |

### Teclado

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tab_keys` | Teclas de avançar campo | texto | As teclas que fazem o papel do Tab (`"Enter"`, `"Enter;Down"`), como o Enter que pula de campo nos sistemas em Delphi. O controle que usa a tecla (o Enter num Memo) fica com ela. |
| `back_tab_keys` | Teclas de voltar campo | texto | As que fazem o papel do Shift+Tab (`"Shift+Enter;Up"`). |
| `allowed_chars` | Caracteres permitidos | texto | Os caracteres liberados, bloqueados e só os liberados, para todos os campos do form (o campo decide primeiro; `chars_override` faz o form mandar). Faixas: `0-9`, `a-z`. |
| `blocked_chars` | Caracteres bloqueados | texto |  |
| `only_allowed` | Só os permitidos | lógico |  |
| `chars_override` | Força os caracteres | lógico |  |
| `char_map` | Trocas de caracteres | texto | Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a mesma herança dos filtros. |
| `key_preview` | Recebe as teclas antes | lógico | O form recebe as teclas antes do controle com foco (`on_key_down` e `on_key_press`), como o `KeyPreview` do Delphi. |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `position` | Posição | um de: `center`, `screen_center`, `designed`, `default`, `top_left`, `top_right`, `bottom_left`, `bottom_right` |  |
| `window_state` | Estado da janela | um de: `normal`, `maximized`, `minimized`, `full_screen` | Normal, maximizada, minimizada ou em tela cheia (`full_screen`). No celular e no navegador o form já ocupa a tela toda. |
| `monitor` | Retorno | inteiro | Em que monitor a janela abre (o índice do `Screen`; -1 = o do dono ou o principal). Com `position = designed`, o `left`/`top` contam do canto dele. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_show` | o form abriu | — |  |
| `on_window_created` | — | — |  |
| `on_close` | o form fechou | — |  |
| `on_close_query` | o form vai fechar: `Err` recusa (o X não fecha) | — | sim: `can_close` |
| `on_activate` | — | — |  |
| `on_deactivate` | — | — |  |
| `on_minimize` | — | — |  |
| `on_restore` | — | — |  |
| `on_navigate` | — | — |  |
| `on_gpu_change` | — | — |  |

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
