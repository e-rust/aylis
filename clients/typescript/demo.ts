// Backend de demonstração em TypeScript: um campo, um botão e a resposta do backend.
//
//   aylis-host -- bun clients/typescript/demo.ts
//   aylis-host -- node --experimental-strip-types clients/typescript/demo.ts
//   aylis-host --png tela.png -- bun clients/typescript/demo.ts --selftest

import { readFileSync } from "node:fs";
import { Aylis, deltaSelftest, log } from "./aylis.ts";

const selftest = process.argv.includes("--selftest");
const runtime = typeof (globalThis as { Bun?: unknown }).Bun !== "undefined" ? "bun" : "node";
const aylis = new Aylis();

const layout = JSON.parse(readFileSync(new URL("../demo_layout.json", import.meta.url), "utf8"));
aylis.send("load", { layout });

aylis.on("hello", async () => {
  const name = (await aylis.call("get", { target: "edtName", prop: "text" })) as string;
  aylis.send("set", { target: "lblMsg", props: { text: `Olá, ${name || "mundo"}! (TypeScript/${runtime})` } });
  if (selftest) {
    log("SELFTEST typescript-" + runtime, await aylis.call("get", { target: "lblMsg", prop: "text" }));
    // Fechar pelo backend passa pelo on_close_query, que pergunta (e espera a resposta).
    aylis.send("close", { target: "frmDemo" });
  }
});

// O on_close_query pergunta: a tela só fecha com a resposta (o ev.reply).
aylis.on("pode_fechar", (ev) => {
  log("SELFTEST typescript-" + runtime + " pergunta");
  log("SELFTEST typescript-" + runtime + " " + deltaSelftest());
  ev.reply.can_close = true;
});

await aylis.call("run", { form: "frmDemo" });
if (selftest) {
  aylis.send("set", { target: "edtName", props: { text: "Ana" } });
  aylis.send("click", { target: "btnHello" });
}
await aylis.loop();
