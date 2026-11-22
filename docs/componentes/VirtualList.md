### Preencher a linha no `on_bind`

O `on_bind` chega com o índice do item que entrou na tela. A linha é uma cópia do modelo (os
componentes de dentro da lista), e o acesso tipado gera uma struct com as peças dela:

```rust
pub fn lstMusicas_bind(&mut self, ui: &mut Ui, _sender: VirtualList, index: i64) {
    let musica = &self.data.musicas[index as usize];
    if let Some(row) = self.lstMusicas_row(ui, index as usize) {
        row.lblTitulo.set_text(ui, &musica.titulo);
        row.lblDuracao.set_text(ui, &musica.duracao);
    }
}
```

Sem a struct, peça a peça: `VirtualList::part(ui, self.lstMusicas.id, i, self.lblTitulo.id)`
devolve a cópia do `lblTitulo` na linha `i`.

### Achar o item no clique

Um clique numa peça da linha chega com ela em `sender`: `self.lstMusicas_row_of(ui, sender.id)`
devolve a linha (o `index` dela), ou `VirtualList::index_of(ui, self.lstMusicas.id, sender.id)`
só o índice.

### Escolher, teclado e barra

Com `selection` = `single` (ou `multiple`), o clique escolhe a linha, as setas, Page Up/Down e
Home/End andam e o Enter dispara o `on_item_activate` (que o duplo clique também dispara: o
"tocar esta música"). Os eventos chegam com o índice: `on_item_click`, `on_item_dbl_click`,
`on_item_activate` e `on_change` (a escolha mudou). A linha escolhida fica no estado `checked`
do tema: um `Panel` com `interactive` (ou com `selected_fill`) mostra a cor. `scroll_bar` =
`auto` mostra a barra quando os itens passam da área.

O que é do app (a música que toca) o app desenha no `on_bind`; quando muda, `refresh_item(i)`
refaz só aquela linha.

### Vários modelos de linha

Cada filho direto da lista é um modelo. `kinds` diz, por item, o nome do modelo
(`pnlSecao;pnlMusica;pnlMusica`; o que falta usa o primeiro), `kind_field` lê do campo do
registro (no `DataRepeater`) e, no Rust, `VirtualList::set_kind(ui, lista, |i| ...)`. Cada
item tem a altura do modelo dele (sem `sizes`). No acesso tipado, um método por modelo:
`self.lstMusicas_pnlMusica(ui, i)` devolve `None` no item de outro modelo.

### Animações nas linhas

O modelo pode ter `FloatAnimation` e `ColorAnimation`; cada linha leva uma cópia delas. Como as
linhas são reaproveitadas ao rolar, a lista cuida de que a animação acompanhe o item, e não a
linha:

- a linha que vai para outro item para as animações e volta cada propriedade ao valor de antes
  delas (a animação pela metade não passa para a música seguinte);
- o estado do item chega pronto, sem animar: o item escolhido que volta à tela já aparece no fim
  da animação `checked`;
- o `appear` roda uma vez por item: o que entra rolando anima, e o que já apareceu volta pronto.
  O `refresh` e uma lista com menos itens (filtro, recarga) fazem todos valerem de novo;
- pelo código, a animação de um item é a da linha dele: `self.lstMusicas_pnlMusica(ui, i)` e o
  `start` (o `stop` deixa o valor onde está). Fora da tela, o item não tem linha e não há o que
  animar.

Para o mouse em qualquer lugar da linha (no título, no ícone), use o gatilho `hover_within`; o
`hover` é só o controle debaixo do mouse. Não anime uma propriedade que o `on_bind` grava (a
cor que vem do item, por exemplo): um sobrescreve o outro.

### Cabeçalho

O `ListHeader` é opcional e fica à parte: ele alinha cada título a um componente do modelo pelo
nome (`part`), sem obrigar as linhas a terem as mesmas colunas.
