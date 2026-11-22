### Tocar a lista

Ligue o `player` num `MediaPlayer` e ponha os arquivos em `items`. `play_index(i)` começa,
`next` e `previous` andam (o `previous` volta ao começo da faixa depois de 3 s), e o
`on_track_change` avisa a faixa nova (-1 no fim). Com `gapless` (ligado), a próxima faixa é
aberta antes do fim da atual e emendada sem silêncio.

`repeat` = `one` repete a faixa; `all` volta ao começo da lista. O `shuffle` sorteia uma ordem
que passa por todas antes de repetir.
