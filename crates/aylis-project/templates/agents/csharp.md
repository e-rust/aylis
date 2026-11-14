## O código em C#

- Gerados (não edite): `csharp/generated/`, `csharp/src/Forms/AllForms.cs` (o registro),
  `csharp/src/Forms/Components.cs` (o acesso tipado) e o bloco do cliente do Aylis no
  `App.csproj` (o resto do `App.csproj` é seu).
- Seus: `csharp/src/Program.cs`, `csharp/src/Forms/Frm<Form>.cs` (os eventos de cada form) e as
  suas classes.
- Um handler novo entra antes do `}` final da classe do form:

  ```cs
  using static App.Forms.Components;

  public static void btnSalvar_click(Aylis aylis, AylisEvent ev)
  {
      var nome = frmPrincipal.edtNome.Text;
      frmPrincipal.lblResposta.Text = $"Olá, {nome}!";
  }
  ```

- Outro form: `var form = aylis.CreateForm("frmClientes");` e `form?.Show()` (ou
  `aylis.ShowModal(form)`).
- Mensagem: `aylis.MessageDlg("Gravado.", "information", "ok")`.
- Segundo plano: uma `Task`/`Thread` sua; o resultado volta à tela com `aylis.Post(() => ...)`
  (ou `aylis.PostTo(form, () => ...)`, que desiste se o form fechou).
- Erro: `throw new Exception("Informe o CPF")` recusa e mostra a mensagem.
- Testes: `dotnet test` (com um projeto de testes ao lado).
