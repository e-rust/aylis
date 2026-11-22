### Vistas: o mesmo form em outras larguras

Um form muda com o tamanho da janela pelas vistas (`views`): cada vista diz quando vale (até
uma largura, uma altura, uma orientação ou uma plataforma) e só o que muda nela. Saindo da
faixa, tudo volta ao desenho base. No designer, escolha a vista na barra de cima e mexa nos
componentes: a mudança fica na vista.

```json
"views": [
  { "name": "compact", "max_width": 699, "font_scale": 0.9,
    "changes": {
      "pnlLateral": { "visible": false },
      "pnlVolume": { "parent": "pnlMusica", "align": "bottom" },
      "lblTitulo": { "font_size": 28 }
    } }
]
```

Uma vista troca propriedades, muda um componente de contêiner (`parent`) e de posição
(`index`) e escala as fontes e os espaços. O que não é propriedade vai no `on_view_change`, que
chega com o nome da vista que entrou (vazio: o base) só quando ela muda, e não a cada pixel do
arrasto da janela:

```rust
pub fn frmPlayer_view_change(&mut self, ui: &mut Ui, _sender: Form, view: String) {
    self.data.compacto = view == "compact";
    self.lstMusicas.set_item_height(ui, if self.data.compacto { 48.0 } else { 56.0 });
}
```
