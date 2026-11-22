# Playlist

A lista de faixas de um player. Não visual.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `player` | Player | componente | O `MediaPlayer` que toca a lista. |
| `shuffle` | Aleatório | lógico | A ordem aleatória (sem repetir até passar por todas). |
| `repeat` | Repetir | um de: `off`, `all`, `one` | No fim: `off` para, `all` volta ao começo da lista, `one` repete a faixa. |
| `gapless` | Sem silêncio entre faixas | lógico | Emenda a próxima faixa sem silêncio (desligado: o player fecha uma e abre a outra). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `items` | Itens | lista de textos (separados por `;`) | Os arquivos (do projeto ou caminhos do disco), na ordem da lista. |
| `index` | Índice | inteiro | A faixa de agora (-1: nenhuma). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_track_change` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `Playlist1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Playlist", "name": "Playlist1", "on_track_change": "playlist1_track_change" }
```

E o handler, um método do form (`self.Playlist1` já é um `Playlist`):

```rust
impl FrmPrincipal {
    pub fn playlist1_track_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Playlist1.index(ui);
        self.Playlist1.set_index(ui, atual + 1);
    }
}
```

## Mais sobre o Playlist

### Tocar a lista

Ligue o `player` num `MediaPlayer` e ponha os arquivos em `items`. `play_index(i)` começa,
`next` e `previous` andam (o `previous` volta ao começo da faixa depois de 3 s), e o
`on_track_change` avisa a faixa nova (-1 no fim). Com `gapless` (ligado), a próxima faixa é
aberta antes do fim da atual e emendada sem silêncio.

`repeat` = `one` repete a faixa; `all` volta ao começo da lista. O `shuffle` sorteia uma ordem
que passa por todas antes de repetir.
