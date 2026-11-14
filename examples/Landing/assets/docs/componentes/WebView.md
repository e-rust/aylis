# WebView

![WebView](img/WebView.png)

Navegador embutido. `url` abre um endereço; sem ele, `html` mostra o texto dado.

**Paleta:** Additional · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `skeleton` | Skeleton ao carregar | lógico | Com o app rodando, enquanto a página carrega, um skeleton no lugar (blocos no tom do fundo): a vista nativa fica por cima dele quando a página aparece. Sem ele, a reserva do designer (a barra de endereço e o aviso). |
| `background_color` | Cor de fundo | cor | A cor da vista antes da primeira pintura da página (sem ela, branca: o clarão num app escuro). O relatório do Tabernáculo, W07. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `allowed_origins` | Origens permitidas | texto | Origens que podem mandar mensagens (`https://pagamento.exemplo.com`), separadas por vírgula ou espaço; vazia, a origem do `url` (com `html`, só o próprio conteúdo). |
| `allowed_navigation` | Navegação permitida | texto | Para onde a página pode navegar (origens ou prefixos, separados por vírgula ou espaço); vazia, qualquer lugar. Um endereço fora vira `on_blocked`. |
| `data_dir` | Pasta de dados | texto | A pasta de dados do navegador (cookies, cache); vazia, a do sistema. Um app instalado em `Program Files` precisa de uma pasta gravável (a do usuário). |
| `browser_args` | Argumentos do navegador | texto | Argumentos do navegador do sistema (o WebView2 no Windows), como `--autoplay-policy=no-user-gesture-required`. Valem ao criar a vista; o WebView2 recusa argumentos diferentes na mesma pasta de dados. A depuração remota não é aceita (abriria uma porta). |
| `sandbox` | Restrições do iframe (sandbox) | texto | O atributo `sandbox` do `<iframe>` no navegador (`allow-scripts allow-forms`...). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `url` | Endereço (URL) | texto | O endereço aberto (`https://...`); vazio: mostra o `html`. |
| `html` | Conteúdo (HTML) | texto | O HTML mostrado quando não há `url`. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_load` | — | — |  |
| `on_navigate` | — | — |  |
| `on_blocked` | — | — |  |
| `on_message` | — | — |  |

## Em código

No form, o `WebView1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "WebView", "name": "WebView1", "on_load": "web_view1_load" }
```

E o handler, um método do form (`self.WebView1` já é um `WebView`):

```rust
impl FrmPrincipal {
    pub fn web_view1_load(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.WebView1.url(ui);
        self.WebView1.set_url(ui, format!("{atual}!"));
    }
}
```
