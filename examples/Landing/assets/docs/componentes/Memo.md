# Memo

![Memo](img/Memo.png)

Campo de texto de várias linhas: observações, descrições, código. Rola, quebra linhas e pode mostrar os números das linhas e realçar a sintaxe.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `placeholder` | Texto de exemplo | texto |  |
| `border` | Borda | lógico | A borda da caixa (desligada: só o texto, sem o contorno nem o traço do foco; dentro de um painel ou numa tabela). Ligada por padrão. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `read_only` | Somente leitura | lógico |  |
| `sensitive` | Dado pessoal (LGPD: mascarado nas gravações) | lógico | Dado pessoal (LGPD): as gravações do `EventRecorder` mascaram o que é digitado aqui, a não ser com o `record_sensitive` dele ligado. Liga e desliga em tempo de execução. |
| `max_length` | Tamanho máximo | inteiro | Limite de caracteres; 0 = sem limite. |
| `want_tabs` | Aceita Tab | lógico |  |
| `wheel_lines` | Linhas por rolagem | inteiro | Quantas linhas cada dente da roda do mouse rola (o touchpad acompanha o dedo). Com o Ctrl, cada dente rola uma página, como no Delphi. |
| `spellcheck` | Corretor ortográfico | lógico | Sublinha as palavras erradas; o clique direito numa delas mostra as correções. |
| `autocomplete` | Completar sozinho | lógico | Completa a palavra enquanto digita (Tab ou seta para a direita aceita). |
| `language` | Idioma | texto | Idioma da correção e das sugestões; vazio: o do sistema. |
| `keyboard_suggestions` | Sugestões do teclado | lógico | Celular: a faixa de sugestões e a correção automática do teclado do sistema (desligue em códigos, logins, identificadores). |

### Cursor

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `caret_color` | Cor do cursor | cor | Cor do cursor; vazio: a cor do texto. |
| `caret_width` | Largura do cursor | número |  |
| `caret_style` | Forma do cursor | um de: `line`, `block`, `underline` |  |
| `caret_blink` | Piscar do cursor (ms) | duração (ms) | Intervalo de piscar (ms); 0: não pisca. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `text` | Texto | texto |  |

### Teclado

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `allowed_chars` | Caracteres permitidos | texto | Os caracteres liberados (vencem um bloqueio do form ou de um painel), os bloqueados e se só os liberados valem (`allowed_chars = "0-9"` com `only_allowed`: só números). Faixas como `a-z`; `-` é o hífen. Vale para o digitado e o colado. |
| `blocked_chars` | Caracteres bloqueados | texto |  |
| `only_allowed` | Só os permitidos | lógico |  |
| `char_map` | Trocas de caracteres | texto | Trocas do que o teclado produz: pares "digitado, entregue" (`",."` troca a vírgula por ponto), ao digitar e ao colar; vale também o de um contêiner (veja o `Form`). |
| `char_case` | Maiúsculas e minúsculas | um de: `normal`, `upper`, `lower` | Maiúsculas ou minúsculas no valor (o `CharCase` do Delphi): ao digitar, ao colar e quando o código grava o `text`. Ligado a um campo, manda o `char_case` do campo. |

### Texto

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `h_align` | Alinhamento horizontal | um de: `leading`, `center`, `trailing`, `justify` |  |
| `wrap` | Quebra de linha | lógico | Quebra automática de linha. |
| `trimming` | Reticências | um de: `none`, `character`, `word` | Reticências nas linhas que não cabem (sem `wrap`, fora da edição). |
| `syntax` | Linguagem do realce | texto | Linguagem do realce (vazio = texto comum). |
| `line_numbers` | Números das linhas | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código, como na VCL) | — |  |
| `on_caret_move` | — | — |  |
| `on_gutter_click` | — | — |  |
| `on_bookmarks_change` | — | — |  |
| `on_paste` | — | — |  |

## Em código

No form, o `Memo1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Memo", "name": "Memo1", "on_change": "memo1_change" }
```

E o handler, um método do form (`self.Memo1` já é um `Memo`):

```rust
impl FrmPrincipal {
    pub fn memo1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Memo1.text(ui);
        self.Memo1.set_text(ui, format!("{atual}!"));
    }
}
```
