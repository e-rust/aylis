## O código em Rust

- Gerados (não edite): `rust/generated/` inteiro (as structs dos forms, o acesso tipado e o
  registro dos handlers). A IDE também acerta as features e os caminhos do `Cargo.toml` e põe
  os `mod` que faltam no `src/lib.rs`; o resto desses arquivos é seu.
- Seus: `rust/src/lib.rs` (o `app()`), `rust/src/main.rs`, `rust/src/forms/<form>.rs` (os
  eventos de cada form), os seus módulos e `rust/tests/`.
- Um handler novo entra no fim do `impl` do form:

  ```rust
  pub fn btnSalvar_click(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
      let nome = self.edtNome.text(ui);
      self.lblResposta.set_text(ui, format!("Olá, {nome}!"));
  }
  ```

  Num evento-pergunta (`before_post`, `on_close_query`...) o handler devolve
  `Result<(), String>`: `Err(mensagem)` recusa (o `?` repassa).
- Outro form pelo tipo: `let form = FrmClientes::create(ui)?; ui.show(form)?;`.
- Mensagem: `message_dlg(ui, "Gravado.", DlgKind::Information, &[ModalResult::Ok], |_, _| {});`.
- Segundo plano: `ui.background(owner, move || trabalho(), move |ui, resultado| { ... })` roda o
  trabalho noutra thread e o fim na tela, só se o form `owner` ainda existir (no navegador, o
  trabalho roda na thread da tela). Com uma thread sua, volte à tela com
  `let poster = ui.poster();` e `poster.post(move |ui| ...)` (ou `post_to(owner, ...)`).
- Testes: `cargo test` na pasta `rust/` (`rust/tests/*.rs`): `let mut app = <crate>::app()?;`
  monta o app sem janela, e `app.ui.fire(id, "on_click", &EventArgs::None)` dispara um evento.
