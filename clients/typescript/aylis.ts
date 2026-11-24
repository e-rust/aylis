// Cliente do protocolo do aylis para backends em TypeScript (Node ou Bun), via aylis-host.
//
// O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por linha.
//
//     const aylis = new Aylis();
//     aylis.send("load", { layout });                         // sem esperar resposta
//     const name = await aylis.call("get", { target: "edtName", prop: "text" });
//     aylis.on("btn_salvar_click", async (ev) => { ... });   // handler pelo nome do layout
//     await aylis.run("frmClientes");                         // abre e trata os eventos até fechar
//
// aylis-host -- node --experimental-strip-types demo.ts      (Node 22.6+)
// aylis-host -- bun demo.ts

import * as readline from "node:readline";

export type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

/** O ponteiro de um `on_click` (o campo `pointer` do evento). */
export interface ClickPointer {
  x: number;
  y: number;
  form_x: number;
  form_y: number;
  button: "primary" | "secondary" | "middle" | "back" | "forward";
  /** As teclas apertadas: `ctrl`, `shift`, `alt`, `meta`. */
  modifiers: string[];
  kind: "mouse" | "touch" | "pen";
  clicks: number;
}

export interface AylisEvent {
  event: string;
  handler: string;
  form: string;
  sender: { id: number; name: string; path: string; class: string };
  args: Json;
  /** Dados extras de alguns eventos (a URL de quem mandou, no `on_message` do WebView). */
  info?: Record<string, string>;
  /**
   * No `on_click` do mouse, do toque ou da caneta: onde (`x`/`y` no componente, `form_x`/`form_y`
   * no form), com que botão e com que teclas. Ausente no clique do teclado ou do código.
   * `if (ev.pointer?.modifiers.includes("ctrl")) ...`
   */
  pointer?: ClickPointer;
  /**
   * Num evento que pergunta (`on_close_query`, `before_post`, `on_key_down`...), o número da
   * pergunta: a tela espera a resposta, que o cliente manda quando o handler termina.
   */
  ask?: number;
  /**
   * A resposta de um evento que pergunta, gravável pelo handler: `ev.reply.can_close = false`
   * (fechar), `ev.reply.key = ""` (engolir a tecla), `ev.reply.key = "."` (trocar o caractere),
   * `ev.reply.data`/`ev.reply.label` (o arrasto), `ev.reply.accept = false` (soltar aqui).
   * Recusar com mensagem (`before_post`): `throw new Error("Informe o CPF")`.
   */
  reply: { [key: string]: Json };
}

export type Handler = (event: AylisEvent) => void | Promise<void>;

export class AylisError extends Error {}

/** Os campos opcionais `icon` e `icon_color` do `message_dlg` e do `prompt` (só os que vieram). */
function dialogIcon(icon: string, iconColor: string): { [key: string]: Json } {
  const fields: { [key: string]: Json } = {};
  if (icon) fields.icon = icon;
  if (iconColor) fields.icon_color = iconColor;
  return fields;
}

/** Um componente da lista que a tela manda ao montar a instância de um form. */
export interface ComponentInfo {
  name: string;
  id: number;
  class: string;
}

/**
 * Um componente de um form, pelo id dele na tela (resolvido quando a instância do form é
 * montada). As classes de cada componente (Button, Label...) vêm do `components.ts` gerado.
 */
/** Uma cor do acesso tipado: RGBA (0 a 255) ou um nome do tema (`Color.ofToken("primary")`),
 *  que acompanha a troca de tema. */
export class Color {
  readonly r: number;
  readonly g: number;
  readonly b: number;
  readonly a: number;
  readonly token: string | undefined;

  constructor(r = 0, g = 0, b = 0, a = 255, token?: string) {
    this.r = r;
    this.g = g;
    this.b = b;
    this.a = a;
    this.token = token;
  }

  static rgb(r: number, g: number, b: number, a = 255): Color {
    return new Color(r, g, b, a);
  }

  /** `#RRGGBB`, `#RRGGBBAA` ou `#RGB`. */
  static hex(text: string): Color {
    let h = text.trim().replace(/^#/, "");
    if (h.length === 3) h = h.split("").map((c) => c + c).join("");
    const n = (i: number) => parseInt(h.slice(i, i + 2), 16);
    if (!/^[0-9a-fA-F]{6}([0-9a-fA-F]{2})?$/.test(h)) throw new Error(`cor inválida: ${text} (use #RRGGBB ou #RRGGBBAA)`);
    return new Color(n(0), n(2), n(4), h.length >= 8 ? n(6) : 255);
  }

  static ofToken(name: string): Color {
    return new Color(0, 0, 0, 255, name.trim());
  }

  /** O valor da tela (`#RRGGBB[AA]` ou um nome). */
  static fromJson(value: unknown): Color {
    if (typeof value !== "string" || value === "") return new Color();
    return value.startsWith("#") ? Color.hex(value) : Color.ofToken(value);
  }

  toJson(): string {
    if (this.token) return this.token;
    const x = (v: number) => Math.max(0, Math.min(255, Math.round(v))).toString(16).padStart(2, "0").toUpperCase();
    return `#${x(this.r)}${x(this.g)}${x(this.b)}${this.a === 255 ? "" : x(this.a)}`;
  }
}

/** Margens (`margins`, `padding`): esquerda, cima, direita e baixo. */
export class Bounds {
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;

  constructor(left = 0, top = 0, right = 0, bottom = 0) {
    this.left = left;
    this.top = top;
    this.right = right;
    this.bottom = bottom;
  }

  static all(value: number): Bounds {
    return new Bounds(value, value, value, value);
  }

  static symmetric(horizontal: number, vertical: number): Bounds {
    return new Bounds(horizontal, vertical, horizontal, vertical);
  }

  /** O valor da tela: um número, `[h, v]`, `[l, t, r, b]` ou `{ left, top, right, bottom }`. */
  static fromJson(value: unknown): Bounds {
    if (typeof value === "number") return Bounds.all(value);
    if (Array.isArray(value) && value.length === 2) return Bounds.symmetric(value[0], value[1]);
    if (Array.isArray(value) && value.length === 4) return new Bounds(value[0], value[1], value[2], value[3]);
    if (value && typeof value === "object") {
      const o = value as Record<string, number>;
      return new Bounds(o.left ?? 0, o.top ?? 0, o.right ?? 0, o.bottom ?? 0);
    }
    return new Bounds();
  }

  toJson(): number[] {
    return [this.left, this.top, this.right, this.bottom];
  }
}

/** O número como a tela grava (`0.5`, `12`). */
function num(v: number): string {
  return String(Math.round(v * 1000) / 1000);
}

/** Uma sombra: o deslocamento (`dx`, `dy`), o desfoque, o espalhamento e a cor (sem cor, preto a
 *  25%). `null` no acesso tipado é sem sombra. */
export class Shadow {
  readonly dx: number;
  readonly dy: number;
  readonly blur: number;
  readonly spread: number;
  readonly color: Color;

  constructor(dx = 0, dy = 0, blur = 0, spread = 0, color: Color = new Color(0, 0, 0, 64)) {
    this.dx = dx;
    this.dy = dy;
    this.blur = blur;
    this.spread = spread;
    this.color = color;
  }

  /** `"dx dy desfoque [espalhamento] [#cor]"` (vazio ou inválido: `null`). */
  static fromJson(value: unknown): Shadow | null {
    const numbers: number[] = [];
    let color: Color | undefined;
    for (const token of String(value ?? "").replace(/,/g, " ").split(/\s+/).filter((t) => t !== "")) {
      const n = Number(token.replace(/px$/, ""));
      if (!Number.isNaN(n)) numbers.push(n);
      else if (!color && token.startsWith("#")) color = Color.hex(token);
      else return null;
    }
    if (numbers.length < 2 || numbers.length > 4) return null;
    return new Shadow(numbers[0], numbers[1], numbers[2] ?? 0, numbers[3] ?? 0, color);
  }

  toJson(): string {
    return `${num(this.dx)} ${num(this.dy)} ${num(this.blur)}${this.spread ? " " + num(this.spread) : ""} ${this.color.toJson()}`;
  }
}

/** Uma curva de suavização: uma das prontas (`Easing.outQuad`) ou a Bézier cúbica
 *  (`Easing.cubicBezier(0.2, 0, 0, 1)`), como o `cubic-bezier()` do CSS. */
export class Easing {
  static readonly NAMES = ["linear", "in_quad", "out_quad", "in_out_quad", "out_cubic", "in_out_cubic", "out_back"];
  static readonly linear = new Easing("linear");
  static readonly inQuad = new Easing("in_quad");
  static readonly outQuad = new Easing("out_quad");
  static readonly inOutQuad = new Easing("in_out_quad");
  static readonly outCubic = new Easing("out_cubic");
  static readonly inOutCubic = new Easing("in_out_cubic");
  static readonly outBack = new Easing("out_back");
  readonly name: string;
  readonly bezier: [number, number, number, number] | undefined;

  constructor(name = "linear", bezier?: [number, number, number, number]) {
    this.name = name;
    this.bezier = bezier;
  }

  static cubicBezier(x1: number, y1: number, x2: number, y2: number): Easing {
    return new Easing("", [x1, y1, x2, y2]);
  }

  static fromJson(value: unknown): Easing {
    const text = String(value ?? "").trim();
    const m = /^cubic-bezier\(([^)]*)\)$/.exec(text);
    if (m) {
      const v = m[1].split(",").map(Number);
      return v.length === 4 && v.every((n) => !Number.isNaN(n)) ? Easing.cubicBezier(v[0], v[1], v[2], v[3]) : new Easing();
    }
    const css: Record<string, [number, number, number, number]> = { ease: [0.25, 0.1, 0.25, 1], "ease-in": [0.42, 0, 1, 1], "ease-out": [0, 0, 0.58, 1], "ease-in-out": [0.42, 0, 0.58, 1] };
    if (css[text.toLowerCase()]) return new Easing("", css[text.toLowerCase()]);
    const plain = text.replace(/-/g, "_").toLowerCase();
    return Easing.NAMES.includes(plain) ? new Easing(plain) : new Easing();
  }

  toJson(): string {
    return this.bezier ? `cubic-bezier(${this.bezier.map(num).join(", ")})` : this.name;
  }
}

/** Um ponto (`x`, `y`). */
export class Point {
  readonly x: number;
  readonly y: number;

  constructor(x = 0, y = 0) {
    this.x = x;
    this.y = y;
  }

  /** `"0.5,0 1,1 0,1"` (os que não são pontos ficam de fora). */
  static listFromJson(value: unknown): Point[] {
    return String(value ?? "")
      .split(/\s+/)
      .map((p) => p.split(",").map(Number))
      .filter((p) => p.length === 2 && !Number.isNaN(p[0]) && !Number.isNaN(p[1]))
      .map((p) => new Point(p[0], p[1]));
  }

  static listToJson(points: Point[]): string {
    return points.map((p) => `${num(p.x)},${num(p.y)}`).join(" ");
  }
}

export class AylisComponent {
  readonly aylis: Aylis;
  readonly id: number;
  readonly name: string;

  // Sem propriedades no parâmetro: o Node roda só tirando os tipos (strip types).
  constructor(aylis: Aylis, id: number, name: string) {
    this.aylis = aylis;
    this.id = id;
    this.name = name;
  }

  /** Lê uma propriedade. */
  protected getProp(prop: string): Promise<any> {
    return this.aylis.call("get", { target: this.id, prop });
  }

  /** Grava uma propriedade. */
  protected setProp(prop: string, value: Json) {
    this.aylis.send("set", { target: this.id, props: { [prop]: value } });
  }

  /** A referência a outro componente (`popupMenu`, `dataSource`), ou `null`. */
  protected async getRef(prop: string): Promise<AylisComponent | null> {
    const info: any = await this.aylis.call("get", { target: this.id, prop, ref: true });
    return info ? new AylisComponent(this.aylis, info.id, info.name) : null;
  }

  protected setRef(prop: string, component: AylisComponent | null) {
    this.setProp(prop, component ? component.id : null);
  }

  /** O componente que um método devolveu (com `ref: true`: o id e o nome), ou `null`. */
  protected componentOf(info: any): AylisComponent | null {
    return info ? new AylisComponent(this.aylis, info.id, info.name) : null;
  }

  setFocus() {
    this.aylis.send("focus", { target: this.id });
  }
}

/**
 * A base dos forms: cada form aberto na tela tem a sua instância
 * (duas janelas do mesmo form, duas instâncias), com os componentes resolvidos ao abrir. A
 * classe gerada de cada form monta os componentes em `bind`.
 */
export class AylisForm extends AylisComponent {
  protected parts: AylisComponent[] = [];

  /** Monta os componentes a partir da lista da tela (a classe gerada sobrescreve). */
  bind(_list: ComponentInfo[]) {}

  /** O componente do form; o que falta ou mudou de classe: o form não bate com o código. */
  protected resolve<T extends AylisComponent>(list: ComponentInfo[], name: string, cls: string, ctor: new (aylis: Aylis, id: number, name: string) => T): T {
    const item = list.find((c) => c.name === name);
    if (!item) throw new AylisError(`o form ${this.name} não bate com o código: falta o componente ${name} (${cls})`);
    if (item.class !== cls) throw new AylisError(`o form ${this.name} não bate com o código: ${name} é ${item.class} no form e ${cls} no código`);
    const part = new ctor(this.aylis, item.id, name);
    this.parts.push(part);
    return part;
  }

  /** Um componente que só existe em algumas plataformas (undefined se não está aqui). */
  protected resolveOptional<T extends AylisComponent>(list: ComponentInfo[], name: string, ctor: new (aylis: Aylis, id: number, name: string) => T): T | undefined {
    const item = list.find((c) => c.name === name);
    if (!item) return undefined;
    const part = new ctor(this.aylis, item.id, name);
    this.parts.push(part);
    return part;
  }

  /** O componente deste form com o id (o do evento). */
  componentById(id: number): AylisComponent | undefined {
    return id === this.id ? this : this.parts.find((p) => p.id === id);
  }

  show() {
    this.aylis.send("show", { target: this.id });
  }

  close() {
    this.aylis.send("close", { target: this.id });
  }
}

/** Uma classe de form registrada: como criar a instância e como torná-la a atual. */
interface FormClass {
  ctor: new (aylis: Aylis, id: number, name: string) => AylisForm;
  activate: (form: AylisForm) => void;
}

const formClasses = new Map<string, FormClass>();

/**
 * Registra a classe de um form (o `components.ts` gerado faz): cada form com esse nome aberto
 * na tela vira uma instância dela. `activate` torna a instância a atual (a variável `frmX`
 * gerada aponta para a janela cujo evento está rodando).
 */
export function registerFormClass(name: string, ctor: new (aylis: Aylis, id: number, name: string) => AylisForm, activate: (form: AylisForm) => void) {
  formClasses.set(name.toLowerCase(), { ctor, activate });
}

export class Aylis {
  private handlers = new Map<string, Handler>();
  /** As instâncias dos forms, pelo id do form na tela. */
  private forms = new Map<number, AylisForm>();
  /** Os forms liberados na tela (o aviso `__freed`). */
  private freed = new Set<number>();
  private pending = new Map<number, { resolve: (v: Json) => void; reject: (e: Error) => void }>();
  protected events: AylisEvent[] = [];
  protected wake: (() => void) | null = null;
  protected closed = false;
  private posted: (() => void | Promise<void>)[] = [];
  private nextId = 1;
  /** Esperando um diálogo (`messageDlg`, `showModal`): os eventos rodam já (os do modal). */
  private waiting = 0;
  /** Handlers rodando agora: um evento que pergunta roda já, mesmo com um deles esperando um `call`. */
  private running = 0;

  /** `attach`: lê o stdin (falso nas sessões: quem lê é o servidor). */
  constructor(attach = true) {
    if (!attach) return;
    const rl = readline.createInterface({ input: process.stdin, crlfDelay: Infinity });
    rl.on("line", (line) => this.receive(line));
    rl.on("close", () => {
      this.closed = true;
      for (const p of this.pending.values()) p.reject(new AylisError("a tela foi fechada"));
      this.pending.clear();
      this.wake?.();
    });
  }

  protected receive(line: string) {
    if (!line.trim()) return;
    this.deliver(JSON.parse(line));
  }

  /** Uma mensagem que chegou (as respostas e os eventos). */
  deliver(message: any) {
    if (message.id !== undefined && this.pending.has(message.id)) {
      const p = this.pending.get(message.id)!;
      this.pending.delete(message.id);
      if (message.error !== undefined) p.reject(new AylisError(message.error));
      else p.resolve(message.result ?? null);
      return;
    }
    if (message.event === "closed") this.closed = true;
    // Esperando um diálogo, o ping (a sincronia dos roteiros) responde já: o que veio antes já rodou.
    if (message.event === "ping" && this.waiting > 0) {
      this.send("pong");
      return;
    }
    // Esperando um diálogo, ou um evento que pergunta com um handler esperando um `call` (o
    // `post` do próprio backend dispara o `before_post`): roda já, sem esperar a fila.
    const now = message.event !== undefined && message.event !== "closed" && message.event !== "ping" && (this.waiting > 0 || (message.ask !== undefined && this.running > 0 && this.running < 8));
    if (now) {
      void this.handle(message as AylisEvent);
      return;
    }
    if (message.event !== undefined) this.events.push(message as AylisEvent);
    this.wake?.();
  }

  /** A versão do protocolo do aylis que este cliente fala (o hello, a primeira mensagem). */
  static readonly PROTOCOL = 1;
  private greeted = false;

  /** A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes). */
  protected greet() {
    if (this.greeted) return;
    this.greeted = true;
    this.write({ cmd: "hello", protocol: Aylis.PROTOCOL, client: "typescript", features: ["done"] });
  }

  protected write(message: { [key: string]: Json }) {
    process.stdout.write(JSON.stringify(message) + "\n");
  }

  /** Envia um comando sem esperar resposta (erros chegam como evento "error"). */
  send(cmd: string, fields: { [key: string]: Json } = {}) {
    this.greet();
    this.write({ cmd, ...fields });
  }

  /** Marca um trecho no profiler da IDE (a faixa "Marcas"): `aylis.profile("consulta", () => ...)`. */
  profile<T>(name: string, body: () => T): T {
    this.send("profile_begin", { name });
    try {
      return body();
    } finally {
      this.send("profile_end");
    }
  }

  /** Envia um comando e espera o resultado. */
  call(cmd: string, fields: { [key: string]: Json } = {}): Promise<Json> {
    this.greet();
    const id = this.nextId++;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.write({ id, cmd, ...fields });
    });
  }

  /**
   * O valor de um campo decimal (Currency, BCD) como o texto exato (`"1234.50"`): o JavaScript
   * não tem decimal, e o `number` perderia centavos; use uma biblioteca de decimal com ele.
   * `null` se o campo é nulo.
   */
  async fieldDecimal(dataSet: string, field: string): Promise<string | null> {
    const value = await this.call("data", { target: dataSet, op: "value", field });
    return value === null ? null : String(value);
  }

  /**
   * Caixa de mensagem que espera: devolve o botão (`"yes"`, `"no"`, `"ok"`...). Enquanto espera, os eventos continuam rodando.
   * `icon` (nome de ícone) e `iconColor` (token do tema, como `"danger"`, ou `"#rrggbb"`) trocam o ícone do tipo.
   */
  async messageDlg(text: string, kind: "information" | "warning" | "error" | "confirmation" = "information", buttons: string[] = ["ok"], icon = "", iconColor = ""): Promise<string> {
    return String(await this.waitFor("message_dlg", { text, kind, buttons, ...dialogIcon(icon, iconColor) }));
  }

  /**
   * A pergunta com um campo de texto (OK e Cancelar), que espera: devolve o texto digitado, ou
   * `null` no Cancelar. `required` não deixa confirmar vazio. Enquanto espera, os eventos continuam rodando.
   */
  async prompt(title: string, label: string, value = "", required = false, icon = "", iconColor = ""): Promise<string | null> {
    const fields: { [key: string]: Json } = { title, label, value, ...dialogIcon(icon, iconColor) };
    if (required) fields.required = true;
    const result = await this.waitFor("prompt", fields);
    return result === null || result === undefined ? null : String(result);
  }

  /**
   * Abre o form como modal e espera ele fechar: devolve o resultado (`"ok"`, `"cancel"`...).
   * Com `container` (um componente de outro form, como a página de uma aba), o modal fica só
   * nele e o resto da tela continua respondendo.
   */
  async showModal(form: string | number | AylisForm, container?: string | number | AylisComponent): Promise<string> {
    const target = form instanceof AylisForm ? form.id : form;
    const fields: { [key: string]: Json } = { target };
    if (container !== undefined && container !== "") fields.container = container instanceof AylisComponent ? container.id : container;
    return String(await this.waitFor("show_modal", fields));
  }

  private async waitFor(cmd: string, fields: { [key: string]: Json }): Promise<Json> {
    this.waiting++;
    // O que chegou antes (enquanto outro call esperava) roda já.
    for (const event of this.events.splice(0)) {
      if (event.event === "ping") this.send("pong");
      else if (event.event === "closed") this.events.push(event);
      else if (event.event !== "error") void this.handle(event);
    }
    try {
      return await this.call(cmd, { ...fields, wait: true });
    } finally {
      this.waiting--;
    }
  }

  /** Handler pelo nome usado no layout (ou "frmClientes.nome", só daquele form). */
  on(name: string, handler: Handler) {
    this.handlers.set(name, handler);
  }

  /** Como o `post`, ligado a um form: se ele foi liberado até lá (a janela fechou e saiu), `job` não roda. */
  postTo(owner: AylisForm, job: () => void | Promise<void>) {
    this.post(() => (this.freed.has(owner.id) ? undefined : job()));
  }

  /** Roda `job` no laço desta tela, depois do evento em andamento. */
  post(job: () => void | Promise<void>) {
    this.posted.push(job);
    this.wake?.();
  }

  /**
   * A instância do form da tela (criada e montada na primeira vez), já como a atual; undefined
   * se a classe dele não foi registrada. Um form que não bate com o código lança AylisError.
   */
  async formInstance(form: string, id: number): Promise<AylisForm | undefined> {
    const entry = formClasses.get(form.toLowerCase());
    if (!entry) return undefined;
    let instance = this.forms.get(id);
    if (!instance) {
      instance = new entry.ctor(this, id, form);
      instance.bind((await this.call("components", { target: id })) as unknown as ComponentInfo[]);
      this.forms.set(id, instance);
    }
    entry.activate(instance);
    return instance;
  }

  /** Cria o form na tela e devolve a instância dele; `another`: mais uma janela do mesmo form. */
  async createForm(form: string, another = false): Promise<AylisForm | undefined> {
    const created = (await this.call("create_form", { name: form, new: another })) as { id: number };
    return this.formInstance(form, created.id);
  }

  /** Abre o form e trata os eventos até a tela fechar. */
  async run(form: string | null = null) {
    await this.call("run", { form });
    // A instância do form principal antes do primeiro evento (o código pode usar frmX).
    if (form) {
      const id = await this.call("find", { name: form });
      if (typeof id === "number") await this.formInstance(form, id);
    }
    await this.loop();
  }

  /** Trata os eventos até a tela fechar. */
  async loop() {
    for (;;) {
      const job = this.posted.shift();
      if (job) {
        try {
          await job();
        } catch (e) {
          log("erro no post", String(e));
        }
        continue;
      }
      const event = this.events.shift();
      if (!event) {
        if (this.closed) return;
        await new Promise<void>((resolve) => (this.wake = resolve));
        this.wake = null;
        continue;
      }
      if (event.event === "closed") return;
      // Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes.
      if (event.event === "ping") {
        this.send("pong");
        continue;
      }
      if (event.event === "error") {
        log("erro na tela:", (event as unknown as { message: string }).message);
        continue;
      }
      await this.handle(event);
    }
  }

  /**
   * Roda o handler do evento. Um evento que pergunta (com `ask`) recebe a resposta quando o
   * handler termina: o `ev.reply` ou, com exceção, a recusa com a mensagem (a tela mostra).
   */
  private async handle(event: AylisEvent) {
    // Forms liberados na tela: as instâncias saem e os `postTo` deles não rodam.
    if (event.event === "__freed") {
      for (const id of (event as unknown as { forms: number[] }).forms ?? []) {
        this.freed.add(id);
        this.forms.delete(id);
      }
      return;
    }
    event.reply = {};
    const handler = this.handlers.get(`${event.form}.${event.handler}`) ?? this.handlers.get(event.handler);
    this.running++;
    try {
      if (!handler) {
        log("evento sem handler:", event.handler);
      } else {
        // A janela do evento vira a atual (frmX aponta para ela durante o handler).
        const formId = (event as unknown as { form_id?: number }).form_id;
        if (formId !== undefined) await this.formInstance(event.form, formId);
        await handler(event);
      }
      if (event.ask !== undefined) this.send("answer", { ask: event.ask, values: event.reply });
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      if (event.ask !== undefined) {
        this.send("answer", { ask: event.ask, error: message });
      } else {
        log("erro no handler", event.handler, String(e));
        // A tela mostra o erro ao usuário.
        this.send("exception", { message, path: event.form, handler: event.handler });
      }
    } finally {
      this.running--;
      // O handler terminou: a tela deixa de mostrar o backend ocupado.
      const seq = (event as unknown as { seq?: number }).seq;
      if (seq !== undefined) this.send("done", { seq });
    }
  }
}

/** Mensagem no console do aylis-host (o stdout é do protocolo). */
export function log(...args: unknown[]) {
  process.stderr.write("[backend] " + args.map(String).join(" ") + "\n");
}

/** Uma alteração do delta (veja {@link Delta}). */
export interface Change {
  ref: number;
  op: "insert" | "update" | "delete";
  data_set: string;
  key: { [field: string]: Json };
  old?: { [field: string]: Json };
  new?: { [field: string]: Json };
  /** No item incluído sob um mestre também incluído: o mestre e os campos de ligação. */
  master?: { data_set: string; ref: number; fields: { [detail: string]: string } };
}

/**
 * As alterações de um `apply_updates`, para o backend gravar do jeito dele, na ordem de gravar
 * (o mestre incluído antes dos itens; as exclusões do mestre por último). Ao aceitar o mestre
 * com a chave gerada, os itens dele já recebem a chave antes de serem gravados:
 *
 * ```ts
 * aylis.on("gravar_pedidos", (ev) => {
 *   const delta = new Delta(ev);
 *   for (const c of delta.changes) {
 *     if (c.op === "insert" && c.data_set === "cdsPedidos") delta.accept(c, { id: inserirPedido(c.new!) });
 *     // ...
 *   }
 *   ev.reply.results = delta.results;
 * });
 * ```
 */
export class Delta {
  readonly dataSet: string;
  readonly changes: Change[];
  readonly results: { [key: string]: Json }[] = [];

  constructor(source: AylisEvent | Json | string) {
    let value: unknown = typeof source === "string" ? JSON.parse(source) : source;
    if (value && typeof value === "object" && "args" in (value as object)) value = (value as AylisEvent).args;
    if (typeof value === "string") value = JSON.parse(value);
    const list = Array.isArray(value) ? value : ((value as { changes?: Change[] })?.changes ?? []);
    this.dataSet = Array.isArray(value) ? "" : String((value as { data_set?: string })?.data_set ?? "");
    this.changes = (list as Change[]).map((c) => ({ ...c, key: { ...c.key }, new: { ...(c.new ?? {}) } }));
  }

  private of(op: Change["op"], dataSet?: string) {
    return this.changes.filter((c) => c.op === op && (dataSet === undefined || c.data_set === dataSet));
  }
  inserted(dataSet?: string) { return this.of("insert", dataSet); }
  updated(dataSet?: string) { return this.of("update", dataSet); }
  deleted(dataSet?: string) { return this.of("delete", dataSet); }
  ofDataSet(dataSet: string) { return this.changes.filter((c) => c.data_set === dataSet); }

  /** Gravou; `values` são os que o banco definiu (a chave gerada). */
  accept(change: Change | number, values: { [field: string]: Json } = {}) {
    const ref = typeof change === "number" ? change : change.ref;
    for (const other of this.changes) {
      if (other.master?.ref !== ref) continue;
      for (const [detail, master] of Object.entries(other.master.fields)) {
        if (!(master in values)) continue;
        for (const part of [other.new!, other.key]) if (detail in part) part[detail] = values[master];
      }
    }
    this.results.push(Object.keys(values).length ? { ref, values } : { ref });
  }

  /** Não gravou: a mensagem vai para a tela e o registro continua pendente. */
  reject(change: Change | number, message: string) {
    this.results.push({ ref: typeof change === "number" ? change : change.ref, error: message });
  }
}

/** O delta de exemplo dos selftests dos clientes (o mesmo em todas as linguagens). */
export const SAMPLE_DELTA =
  '{"data_set":"cdsPedidos","changes":[' +
  '{"ref":1,"op":"insert","data_set":"cdsPedidos","key":{"id":-1},"new":{"id":-1,"cliente":"Joao"}},' +
  '{"ref":2,"op":"insert","data_set":"cdsItens","key":{"id":-2},"new":{"id":-2,"pedido_id":-1},' +
  '"master":{"data_set":"cdsPedidos","ref":1,"fields":{"pedido_id":"id"}}},' +
  '{"ref":3,"op":"update","data_set":"cdsItens","key":{"id":10},"old":{"produto":"A"},"new":{"produto":"B"}},' +
  '{"ref":4,"op":"delete","data_set":"cdsPedidos","key":{"id":7},"old":{"id":7}}]}';

/** O texto que todos os clientes escrevem ao conferir o {@link Delta} com o {@link SAMPLE_DELTA}. */
export function deltaSelftest(): string {
  const delta = new Delta(SAMPLE_DELTA);
  delta.accept(delta.inserted("cdsPedidos")[0], { id: 501 });
  const item = delta.inserted("cdsItens")[0];
  return `delta ${delta.inserted().length} ${delta.updated().length} ${delta.deleted().length} ${item.new!.pedido_id} ${delta.results.length}`;
}

/**
 * Uma tela da ponte com `--sessions` (`aylis-host --bridge ... --sessions`): um processo atende
 * todas as telas e cada uma ganha uma instância desta classe, com o estado dela nos campos
 * (nunca em variáveis do módulo, que são de todas). A API é a do Aylis.
 *
 *     class Vendas extends Session {
 *       form = "frmVendas";
 *       total = 0;
 *       async started() {
 *         this.on("btn_somar_click", () => { this.total++; });
 *         await this.run();
 *       }
 *     }
 *     serve(Vendas);
 */
export class Session extends Aylis {
  /** O form que `started` abre (null: o principal do projeto). */
  form: string | null = null;
  /** O usuário logado (o mesmo objeto nas telas dele). */
  user: User | null = null;

  readonly server: Server;
  readonly number: number;

  constructor(server: Server, number: number) {
    super(false);
    this.server = server;
    this.number = number;
  }

  protected override write(message: { [key: string]: Json }) {
    this.server.write({ ...message, session: this.number });
  }

  /** A tela conectou: monte o estado e abra o form (o padrão: `run()`). */
  async started() {
    await this.run(this.form);
  }

  /** A tela saiu (fechou o app ou caiu a rede). */
  ended() {}

  /** Liga esta tela ao usuário `id`: as telas dele dividem o mesmo `User`. */
  login(id: string) {
    this.logout();
    this.user = this.server.user(id);
    this.user.sessions.push(this);
  }

  /** Solta o usuário desta tela (sozinho, ao fechar). */
  logout() {
    if (this.user) this.server.leave(this.user, this);
    this.user = null;
  }

  /** A tela fechou: o laço termina. */
  close() {
    this.closed = true;
    this.events.push({ event: "closed" } as AylisEvent);
    this.wake?.();
  }
}

/** O usuário logado, dividido pelas telas dele (`Session.login`). */
export class User {
  readonly sessions: Session[] = [];
  /** O que vale para todas as telas do usuário (o carrinho, as preferências). */
  data: Record<string, unknown> = {};

  readonly id: string;

  constructor(id: string) {
    this.id = id;
  }

  /** Roda `job(tela)` no laço de cada tela do usuário (atualizar todas). */
  broadcast(job: (session: Session) => void | Promise<void>) {
    for (const s of [...this.sessions]) s.post(() => job(s));
  }
}

/** O processo das sessões: lê o stdin e entrega cada mensagem à tela dela. */
export class Server {
  private sessions = new Map<number, Session>();
  private users = new Map<string, User>();

  write(message: { [key: string]: Json }) {
    process.stdout.write(JSON.stringify(message) + "\n");
  }

  user(id: string): User {
    let u = this.users.get(id);
    if (!u) this.users.set(id, (u = new User(id)));
    return u;
  }

  leave(user: User, session: Session) {
    const i = user.sessions.indexOf(session);
    if (i >= 0) user.sessions.splice(i, 1);
    if (user.sessions.length === 0) this.users.delete(user.id);
  }

  serve(make: (server: Server, number: number) => Session) {
    const rl = readline.createInterface({ input: process.stdin, crlfDelay: Infinity });
    rl.on("line", (line) => {
      if (!line.trim()) return;
      const message = JSON.parse(line);
      const number = message.session;
      delete message.session;
      if (number === undefined) return;
      if (message.event === "session_open") {
        const session = make(this, number);
        this.sessions.set(number, session);
        (async () => {
          try {
            await session.started();
          } catch (e) {
            log("erro na sessão", number, String(e));
          } finally {
            try {
              session.ended();
            } catch (e) {
              log("erro em ended da sessão", number, String(e));
            }
            session.logout();
            this.sessions.delete(number);
          }
        })();
        return;
      }
      const session = this.sessions.get(number);
      if (!session) return;
      if (message.event === "session_close") session.close();
      else session.deliver(message);
    });
  }
}

/** Atende as telas da ponte com `--sessions`: uma instância de `cls` por tela. */
export function serve(cls: new (server: Server, number: number) => Session) {
  new Server().serve((server, number) => new cls(server, number));
}
