## O código em Go

- Gerados (não edite): `go/generated/`, `go/src/forms/forms.go` (o registro),
  `go/src/forms/components.go` (o acesso tipado) e `go/go.mod`.
- Seus: `go/main.go`, `go/src/forms/<form>.go` (os eventos de cada form) e os seus arquivos.
- Um handler novo entra no fim do arquivo do form:

  ```go
  func (f FrmPrincipal) btnSalvar_click(app *aylis.Aylis, ev aylis.Event) {
      nome := frmPrincipal.edtNome.Text()
      frmPrincipal.lblResposta.SetText("Olá, " + nome + "!")
  }
  ```

- Outro form: `form, err := app.CreateForm("frmClientes", false)` e `form.Show()` (ou
  `app.ShowModal(form)`).
- Mensagem: `app.MessageDlg("Gravado.", "information", "ok")`.
- Segundo plano: uma goroutine; o resultado volta à tela com `app.Post(func() { ... })` (ou
  `app.PostTo(form, func() { ... })`, que desiste se o form fechou).
- Erro: `panic("Informe o CPF")` recusa e mostra a mensagem.
- Testes: `go test -v ./...` na pasta `go/` (arquivos `*_test.go`).
