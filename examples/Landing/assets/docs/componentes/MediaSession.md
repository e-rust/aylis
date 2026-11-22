# MediaSession

Os controles de mídia do sistema para um player. Não visual.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `player` | Player | componente | O `MediaPlayer` (ou o `VideoPlayer`) que o sistema controla. |
| `playlist` | Lista de faixas | componente | A `Playlist` do player: a próxima e a anterior do sistema vão para ela. |
| `info` | Tags da mídia | componente | O `MediaInfo` com o título, o artista, o álbum e a capa (sem ele, o nome do arquivo). |
| `active` | Ativo | lógico | Aparece nos controles do sistema (desligado: sai deles). |
| `forward` | Repassar ao player | lógico | Os comandos do sistema já vão para o player e a lista (desligado: só os eventos). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_command` | — | — |  |
| `on_seek` | — | — |  |

## Em código

No form, o `MediaSession1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MediaSession", "name": "MediaSession1", "on_command": "media_session1_command" }
```

E o handler, um método do form (`self.MediaSession1` já é um `MediaSession`):

```rust
impl FrmPrincipal {
    pub fn media_session1_command(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MediaSession1.active(ui);
        self.MediaSession1.set_active(ui, !atual);
    }
}
```

## Mais sobre o MediaSession

### Nos controles do sistema

Ligue o `player` num `MediaPlayer` (ou `VideoPlayer`), a `playlist` numa `Playlist` e o `info`
num `MediaInfo`: o que toca aparece no quadro do volume e na miniatura da barra de tarefas do
Windows, no painel do Linux (MPRIS) e no controle de mídia do navegador, com o título, o
artista e a capa, e as teclas de mídia e os botões do fone controlam o player.

Os comandos chegam no `on_command` (`play`, `pause`, `toggle`, `stop`, `next`, `previous`,
`seek`; o `seek` com a posição no `on_seek`). Com `forward` ligado (o padrão), já vão para o
player e a lista; desligue para decidir no código. `active` desligado tira o app dos controles.

No macOS, no iOS e no Android ainda não liga (o componente existe e fica parado).
