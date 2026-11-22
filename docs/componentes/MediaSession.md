### Nos controles do sistema

Ligue o `player` num `MediaPlayer` (ou `VideoPlayer`), a `playlist` numa `Playlist` e o `info`
num `MediaInfo`: o que toca aparece no quadro do volume e na miniatura da barra de tarefas do
Windows, no painel do Linux (MPRIS) e no controle de mídia do navegador, com o título, o
artista e a capa, e as teclas de mídia e os botões do fone controlam o player.

Os comandos chegam no `on_command` (`play`, `pause`, `toggle`, `stop`, `next`, `previous`,
`seek`; o `seek` com a posição no `on_seek`). Com `forward` ligado (o padrão), já vão para o
player e a lista; desligue para decidir no código. `active` desligado tira o app dos controles.

No macOS, no iOS e no Android ainda não liga (o componente existe e fica parado).
