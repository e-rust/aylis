// Backend de teste das sessões em TypeScript (aylis-host --bridge ... --sessions): um processo
// para todas as telas, um contador por tela; "boom" lança uma exceção que fica só naquela
// sessão, "lento" demora sem travar as outras e "entrar"/"comum" ligam as telas do mesmo
// usuário.

import { Session, log, serve } from "../../../../clients/typescript/aylis.ts";

const LAYOUT = {
  class: "Form", name: "frm", width: 200, height: 80, children: [
    { class: "Label", name: "lbl", text: "-" },
    { class: "Button", name: "btn", text: "+", on_click: "somar" },
    { class: "Button", name: "boom", text: "!", on_click: "boom" },
    { class: "Button", name: "lento", text: "z", on_click: "lento" },
    { class: "Label", name: "lblu", text: "-" },
    { class: "Button", name: "entrar", text: "e", on_click: "entrar" },
    { class: "Button", name: "comum", text: "c", on_click: "comum" },
  ],
};

class Contador extends Session {
  total = 0;

  async started() {
    log(`SELFTEST sessão ${this.number} abriu (pid ${process.pid})`);
    this.on("somar", () => {
      this.total++;
      this.send("set", { target: "frm.lbl", props: { text: `${this.number}:${this.total}` } });
    });
    this.on("boom", () => {
      throw new Error("erro de propósito");
    });
    this.on("lento", () => new Promise((resolve) => setTimeout(resolve, 3000)));
    this.on("entrar", () => {
      this.login("ana");
      this.send("set", { target: "frm.lblu", props: { text: "ana" } });
    });
    this.on("comum", () => {
      const user = this.user!;
      const total = ((user.data.total as number) ?? 0) + 1;
      user.data.total = total;
      user.broadcast((s) => s.send("set", { target: "frm.lblu", props: { text: `u:${total}` } }));
    });
    this.send("load", { layout: LAYOUT });
    await this.run("frm");
  }

  ended() {
    log(`SELFTEST sessão ${this.number} saiu`);
  }
}

serve(Contador);
