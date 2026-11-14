# Rating

![Rating](img/Rating.png)

Avaliação em estrelas (o `v-rating`): `value` de 0 a `max`, com meia estrela (`half`). Mostra a prévia ao passar o mouse; `read_only` só mostra.

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `half` | Meia estrela | lógico | Aceita meia estrela: o clique na metade esquerda e as setas andam de 0,5 em 0,5. |
| `read_only` | Somente leitura | lógico | Só mostra a nota: o clique e o teclado não mudam. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `value` | Valor | número | A nota, de 0 (nenhuma estrela) a `max`. |
| `max` | Máximo | inteiro | Quantas estrelas aparecem. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_change` | o valor mudou (pelo usuário ou pelo código) | — |  |

## Em código

No form, o `Rating1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Rating", "name": "Rating1", "on_change": "rating1_change" }
```

E o handler, um método do form (`self.Rating1` já é um `Rating`):

```rust
impl FrmPrincipal {
    pub fn rating1_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Rating1.value(ui);
        self.Rating1.set_value(ui, atual + 10.0);
    }
}
```
