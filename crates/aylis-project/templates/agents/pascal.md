## O código em Free Pascal

- Gerados (não edite): `pascal/generated/`, `pascal/src/forms/aylisforms.pas` (o registro),
  `pascal/src/forms/ayliscomponents.pas` (os componentes e a classe base `TFrm<Form>Base`) e
  `pascal/src/aylis.pas` (o cliente).
- Seus: `pascal/src/app.lpr`, `pascal/src/forms/<form>.pas` (a classe de cada form, herdeira da
  base gerada), as suas units e `pascal/tests/`.
- Um handler novo ganha a declaração na seção `published` e o corpo antes do `initialization`:

  ```pascal
  procedure TFrmPrincipal.btnSalvar_click(Sender: TAylisComponent; Event: TJSONObject);
  begin
    lblResposta.Text := 'Olá, ' + edtNome.Text + '!';
  end;
  ```

  Dentro do form, os componentes vêm direto (`edtNome`); de fora, pelo form
  (`frmPrincipal.edtNome`).
- Outro form pelo tipo: `Aylis.CreateForm(TFrmClientes);` e `Aylis.ShowModal(frmClientes);`.
- Mensagem: `Aylis.MessageDlg('Gravado.', 'information', ['ok']);`.
- Segundo plano: uma `TThread` sua; o resultado volta à tela com `Aylis.Post(@Terminou)` (ou
  `PostTo(Form, @Terminou)`, que desiste se o form fechou). Da thread, gravar propriedades pode;
  ler, não.
- Erro: `raise Exception.Create('Informe o CPF');` recusa e mostra a mensagem.
- Testes: `pascal/tests/*.pas` com o FPCUnit (`TTestCase` e `RegisterTest` no `initialization`).
