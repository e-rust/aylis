## O código em C++

- Gerados (não edite): `cpp/generated/`, `cpp/src/forms/forms.hpp` (as declarações e o
  registro), `cpp/src/forms/components.hpp` (o acesso tipado) e `cpp/src/aylis.hpp` (o cliente).
- Seus: `cpp/src/main.cpp`, `cpp/src/forms/<form>.cpp` (os eventos de cada form), os seus
  arquivos e `cpp/tests/`.
- Um handler novo entra antes do fim do `namespace` do form:

  ```cpp
  void btnSalvar_click(aylis::Aylis& app, const aylis::Event& ev) {
      auto nome = frmPrincipal.edtNome.text();
      frmPrincipal.lblResposta.set_text("Olá, " + nome + "!");
  }
  ```

- Outro form: `auto form = app.create_form("frmClientes");` e `app.show_modal(form)`.
- Mensagem: `app.message_dlg("Gravado.", "information", {"ok"})`.
- Segundo plano: uma `std::thread` sua; o resultado volta à tela com `app.post([&] { ... })` (ou
  `app.post_to(form, ...)`, que desiste se o form fechou).
- Erro: `throw std::runtime_error("Informe o CPF")` recusa e mostra a mensagem.
- Testes: `cpp/tests/*.cpp`, com `#include "aylis_test.hpp"`, `AYLIS_TEST(nome) { ... }` e
  `AYLIS_CHECK(...)`.
