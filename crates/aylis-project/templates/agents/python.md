## O código em Python

- Gerados (não edite): `python/generated/`, `python/src/forms/__init__.py` (o registro),
  `python/src/forms/components.py` (o acesso tipado) e `python/src/aylis_client.py` (o cliente).
- Seus: `python/src/main.py`, `python/src/forms/<form>.py` (os eventos de cada form) e os seus
  módulos.
- Um handler novo entra no fim do arquivo do form:

  ```python
  from .components import frmPrincipal

  def btnSalvar_click(aylis, event):
      nome = frmPrincipal.edtNome.text
      frmPrincipal.lblResposta.text = "Olá, %s!" % nome
  ```

- Outro form: `form = aylis.create_form("frmClientes")` e `aylis.show(form)` (ou
  `aylis.show_modal(form, ao_fechar)`).
- Mensagem: `aylis.message_dlg("Gravado.", kind="information")`.
- Segundo plano: uma `threading.Thread` sua; o resultado volta à tela com `aylis.post(funcao)`
  (ou `aylis.post_to(form, funcao)`, que desiste se o form fechou). Da outra thread, só
  `post`/`send`.
- Erro: `raise ValueError("Informe o CPF")` recusa e mostra a mensagem.
- Testes: `python -m unittest discover -v` na pasta `python/` (arquivos `test_*.py`).
