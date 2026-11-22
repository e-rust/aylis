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
