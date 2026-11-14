## O código em Java

- Gerados (não edite): `java/generated/`, `java/src/forms/Forms.java` (o registro) e
  `java/src/forms/Components.java` (o acesso tipado).
- Seus: `java/src/Main.java`, `java/src/forms/Frm<Form>.java` (os eventos de cada form), as suas
  classes e `java/tests/`.
- Um handler novo entra antes do `}` final da classe do form:

  ```java
  import static forms.Components.frmPrincipal;

  public static void btnSalvar_click(Aylis aylis, Aylis.Event ev) {
      String nome = frmPrincipal.edtNome.getText();
      frmPrincipal.lblResposta.setText("Olá, " + nome + "!");
  }
  ```

- Outro form: `var form = aylis.createForm("frmClientes", false);` e `form.show()` (ou
  `aylis.showModal(form)`).
- Mensagem: `aylis.messageDlg("Gravado.", "information", "ok")`.
- Segundo plano: uma `Thread` sua; o resultado volta à tela com `aylis.post(() -> ...)` (ou
  `aylis.postTo(form, () -> ...)`, que desiste se o form fechou).
- Erro: `throw new AylisException("Informe o CPF")` recusa e mostra a mensagem.
- Testes: `java/tests/*.java`, com `@Test` e o `Check` de `dev.aylis` (ou o JUnit, se o jar
  estiver em `java/tests/lib/`).
