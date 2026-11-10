# DateEdit

![DateEdit](img/DateEdit.png)

Campo de data com calendário suspenso. Digite no formato do idioma ou escolha no calendário (clique no ícone, Alt+↓ ou F4). `date` é ISO; `on_change` traz a data ISO (vazia ao apagar).

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `placeholder` | Texto de exemplo | texto |
| `border` | Borda | lógico |
| `icon` | Ícone | texto |
| `label` | Rótulo | texto |
| `helper` | Texto de ajuda | texto |
| `counter` | Contador | lógico |
| `prefix` | Prefixo | texto |
| `suffix` | Sufixo | texto |

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `max_length` | Tamanho máximo | inteiro |
| `read_only` | Somente leitura | lógico |
| `password` | Senha (esconde o texto) | lógico |
| `sensitive` | Dado pessoal (LGPD: mascarado nas gravações) | lógico |
| `clear_button` | Botão de limpar | lógico |
| `mask` | Máscara | texto |
| `rules` | Regras de validação | texto |
| `spellcheck` | Corretor ortográfico | lógico |
| `autocomplete` | Completar sozinho | lógico |
| `language` | Idioma | texto |
| `keyboard_suggestions` | Sugestões do teclado | lógico |

### Cursor

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `caret_color` | Cor do cursor | cor |
| `caret_width` | Largura do cursor | número |
| `caret_style` | Forma do cursor | um de: `line`, `block`, `underline` |
| `caret_blink` | Piscar do cursor (ms) | duração (ms) |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `date` | Data | texto |
| `text` | Texto | texto |
| `error` | Erro | texto |

### Teclado

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `allowed_chars` | Caracteres permitidos | texto |
| `blocked_chars` | Caracteres bloqueados | texto |
| `only_allowed` | Só os permitidos | lógico |
| `char_map` | Trocas de caracteres | texto |
| `char_case` | Maiúsculas e minúsculas | um de: `normal`, `upper`, `lower` |

### Texto

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `h_align` | Alinhamento horizontal | um de: `leading`, `center`, `trailing`, `justify` |
| `v_align` | Alinhamento vertical | um de: `leading`, `center`, `trailing`, `justify` |
| `wrap` | Quebra de linha | lógico |
| `trimming` | Reticências | um de: `none`, `character`, `word` |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_caret_move` | — | — |  |

## Em código

No form, o `DateEdit1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DateEdit", "name": "DateEdit1", "on_change": "date_edit1_change" }
```

E o handler, um método do form (`self.DateEdit1` já é um `DateEdit`):

```rust
impl FrmPrincipal {
    pub fn date_edit1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DateEdit1.date(ui);
        self.DateEdit1.set_date(ui, format!("{atual}!"));
    }
}
```
