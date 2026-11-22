# MediaInfo

As tags de um arquivo ou de um player, com provedor. Não visual.

**Paleta:** Media · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file` | Arquivo | texto | O arquivo: do projeto (`assets/audio/musica.mp3`) ou um caminho do disco. |
| `source` | Origem | componente | Um `MediaPlayer` a acompanhar: a cada arquivo que ele abre, as tags dele (no lugar de `file`). |
| `provider` | Provedor | componente | O provedor das tags (vazio: o Symphonia). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O título da música (ou do vídeo), das tags. |
| `artist` | Artista | texto | O artista (ou o artista do álbum, sem o da faixa). |
| `album` | Álbum | texto | O álbum. |
| `track` | Faixa | inteiro | O número da faixa no álbum (0: sem). |
| `year` | Ano | inteiro | O ano (0: sem). |
| `duration` | Duração | número | Em segundos. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_loaded` | — | — |  |
| `on_probe` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `MediaInfo1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "MediaInfo", "name": "MediaInfo1", "on_loaded": "media_info1_loaded" }
```

E o handler, um método do form (`self.MediaInfo1` já é um `MediaInfo`):

```rust
impl FrmPrincipal {
    pub fn media_info1_loaded(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.MediaInfo1.file(ui);
        self.MediaInfo1.set_file(ui, format!("{atual}!"));
    }
}
```

## Mais sobre o MediaInfo

### Seguir o que toca

Com `source` num `MediaPlayer`, o `MediaInfo` lê as tags de cada arquivo que o player abre e
avisa no `on_loaded`; a capa vem do `cover`:

```rust
pub fn infoMusica_loaded(&mut self, ui: &mut Ui, _sender: MediaInfo) {
    self.lblTitulo.set_text(ui, &self.infoMusica.title(ui));
    if let Some(capa) = aylis::prelude::MediaInfo::cover(ui, self.infoMusica.id) {
        self.imgCapa.set_image_bytes(ui, &capa);
    }
}
```

### A lista inteira

`probe(caminho)` lê outro arquivo em segundo plano, sem tocar; o resultado chega no
`on_probe`, com o `file` dele. Chame para cada arquivo de uma playlist.

O leitor padrão é o Symphonia; o `LoftyTagsProvider` (no `provider`) lê mais formatos de tag.
