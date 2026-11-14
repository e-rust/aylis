## O código em TypeScript

- Gerados (não edite): `typescript/generated/`, `typescript/src/forms/index.ts` (o registro),
  `typescript/src/forms/components.ts` (o acesso tipado) e `typescript/src/aylis.ts` (o cliente).
- Seus: `typescript/src/main.ts`, `typescript/src/forms/<form>.ts` (os eventos de cada form) e os
  seus módulos.
- Um handler novo entra no fim do arquivo do form:

  ```ts
  import { frmPrincipal } from "./components.ts";

  export async function btnSalvar_click(aylis: Aylis, ev: AylisEvent) {
      const nome = await frmPrincipal.edtNome.text;
      frmPrincipal.lblResposta.text = `Olá, ${nome}!`;
  }
  ```

  Ler uma propriedade é assíncrono (`await`); escrever, não.
- Outro form: `const form = await aylis.createForm("frmClientes");` e `form.show()` (ou
  `await aylis.showModal(form)`).
- Mensagem: `await aylis.messageDlg("Gravado.", "information", ["ok"])`.
- Segundo plano: o trabalho assíncrono volta à tela com `aylis.post(job)` (ou
  `aylis.postTo(form, job)`, que desiste se o form fechou).
- Erro: `throw new Error("Informe o CPF")` recusa e mostra a mensagem.
- Testes: `bun test` (ou `node --test`) na pasta `typescript/` (arquivos `*.test.ts`).
