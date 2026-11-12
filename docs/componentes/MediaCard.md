### Duas áreas de clique

A caixa de seleção dispara `on_select`; o resto do card, `on_click`. Com `editable_title`,
o nome vira uma terceira área: o clique nele abre o campo de renomear em vez do `on_click`
(a miniatura continua abrindo o item).

### editable_title

O card cuida sozinho do campo: abre sobre o nome, com ele todo selecionado. Enter, Tab ou
o clique fora grava no `title` e dispara `on_rename` com o nome novo (sem espaços nas
pontas; vazio ou igual ao atual não dispara); Esc cancela. O app só grava o nome:

```rust
pub fn card_rename(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) -> Result<()> {
    let novo = args.text().unwrap_or_default();
    biblioteca::renomear(&self.item_atual, novo)?;
    Ok(())
}
```

Se a gravação falhar, devolva o erro e ponha o `title` antigo de volta.

### rename

O método `rename()` abre o campo pelo código, mesmo sem `editable_title`: é o F2 do app.
