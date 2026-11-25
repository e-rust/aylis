"""Cliente mínimo do protocolo do aylis para backends em Python (via aylis-host).

O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por linha.

    aylis = Aylis()
    aylis.send("load", layout={...})        # comando sem esperar resposta
    nome = aylis.call("get", target="edtNome", prop="text")   # espera a resposta

    @aylis.on("btn_salvar_click")          # handler pelo nome usado no layout
    def salvar(event): ...

    @aylis.on("frmClientes.btn_salvar_click")   # só para o btnSalvar do frmClientes
    def salvar_cliente(event): ...

    aylis.show("frmLista")                  # outra janela (como Show)
    aylis.show_modal("frmCliente", lambda result: ...)   # result: "ok", "cancel"...

    aylis.run("FrmClientes")                # abre a janela e trata os eventos até fechar

O evento é a mensagem JSON da tela, como dict. No `on_click` do mouse, do toque ou da caneta,
`event.get("pointer")` diz onde, com que botão e com que teclas (`None` no clique do teclado ou
do código): `{"x", "y"}` no componente, `{"form_x", "form_y"}` no form, `"button"` (`primary`,
`secondary`, `middle`...), `"modifiers"` (`["ctrl", "shift"]`), `"kind"` (`mouse`, `touch`,
`pen`) e `"clicks"`.

    pointer = event.get("pointer")
    if pointer and "ctrl" in pointer["modifiers"]: ...

Threads: os handlers rodam na thread principal (a do `run`). De outra thread (uma consulta
demorada ao banco, um timer), use `aylis.send(...)` (só envia, é seguro) ou
`aylis.post(funcao)`, que roda `funcao()` na thread principal. `aylis.call(...)` só na thread principal: ele lê as respostas, e duas threads lendo
roubariam as mensagens uma da outra.

Por WebSocket (o backend em outra máquina): o backend espera a tela conectar,
`Aylis(listen="0.0.0.0:9000")`, e a tela conecta nele (`aylis-host --connect ws://...`). Atende
uma tela por vez e sem TLS: serve para desenvolvimento. Em produção, use a ponte
(`aylis-host --bridge`), que inicia um backend por tela.
"""

import base64
import contextlib
import hashlib
import decimal
import itertools
import json
import queue
import threading
import socket
import struct
import sys


def _dialog_icon(icon, icon_color):
    """Os campos opcionais icon e icon_color do message_dlg e do prompt (só os que vieram)."""
    fields = {}
    if icon:
        fields["icon"] = icon
    if icon_color:
        fields["icon_color"] = icon_color
    return fields


class AylisError(Exception):
    pass


class WebSocket:
    """WebSocket mínimo (só a biblioteca padrão), do lado servidor: mensagens de texto."""

    @classmethod
    def accept(cls, address):
        """Espera uma conexão em `host:porta` (servidor) e faz o handshake."""
        host, port = address.rsplit(":", 1)
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        server.bind((host, int(port)))
        server.listen(1)
        log("esperando a tela em ws://%s" % address)
        conn, peer = server.accept()
        server.close()
        ws = cls.__new__(cls)
        ws.sock = conn
        request = b""
        while b"\r\n\r\n" not in request:
            chunk = conn.recv(1024)
            if not chunk:
                raise AylisError("a tela fechou a conexão no handshake")
            request += chunk
        head, ws.buffer = request.split(b"\r\n\r\n", 1)
        key = None
        for line in head.decode(errors="replace").split("\r\n"):
            if line.lower().startswith("sec-websocket-key:"):
                key = line.split(":", 1)[1].strip()
        if key is None:
            raise AylisError("pedido sem Sec-WebSocket-Key")
        accept = base64.b64encode(hashlib.sha1((key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode()).digest()).decode()
        conn.sendall(("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
                      "Sec-WebSocket-Accept: %s\r\n\r\n" % accept).encode())
        log("tela conectada (%s:%d)" % peer)
        return ws

    def _recv_exact(self, n):
        while len(self.buffer) < n:
            chunk = self.sock.recv(65536)
            if not chunk:
                raise EOFError()
            self.buffer += chunk
        data, self.buffer = self.buffer[:n], self.buffer[n:]
        return data

    def send(self, text):
        payload = text.encode("utf-8")
        # Do servidor para o cliente, sem máscara.
        header = bytes([0x81])
        n = len(payload)
        if n < 126:
            header += bytes([n])
        elif n < 65536:
            header += bytes([126]) + struct.pack(">H", n)
        else:
            header += bytes([127]) + struct.pack(">Q", n)
        self.sock.sendall(header + payload)

    def recv(self):
        """A próxima mensagem de texto (None quando a conexão fecha)."""
        parts = []
        try:
            while True:
                b0, b1 = self._recv_exact(2)
                opcode, n = b0 & 0x0F, b1 & 0x7F
                if n == 126:
                    n = struct.unpack(">H", self._recv_exact(2))[0]
                elif n == 127:
                    n = struct.unpack(">Q", self._recv_exact(8))[0]
                mask = self._recv_exact(4) if b1 & 0x80 else None
                data = self._recv_exact(n)
                if mask:
                    data = bytes(b ^ mask[i % 4] for i, b in enumerate(data))
                if opcode == 0x8:  # fechar
                    return None
                if opcode == 0x9:  # ping -> pong
                    self.sock.sendall(bytes([0x8A, 0x80 | len(data)]) + b"\0\0\0\0" + data)
                    continue
                if opcode in (0x1, 0x0):
                    parts.append(data)
                    if b0 & 0x80:
                        return b"".join(parts).decode("utf-8")
        except (EOFError, OSError):
            return None


class Delta:
    """As alterações de um apply_updates, para o backend gravar do jeito dele.

    Vêm na ordem de gravar (o mestre incluído antes dos itens; as exclusões do mestre por
    último). Cada alteração é um dict: ref, op ("insert", "update", "delete"), data_set,
    key, old, new e, no item incluído sob um mestre também incluído, master (ref do mestre e
    os campos de ligação). Ao aceitar o mestre com a chave gerada, os itens dele já recebem
    a chave antes de serem gravados::

        @aylis.on("gravar_pedidos")
        def gravar(event):
            delta = Delta(event)
            for change in delta:
                if change["op"] == "insert" and change["data_set"] == "cdsPedidos":
                    novo = db.execute("insert ... returning id", ...).fetchone()[0]
                    delta.accept(change, id=novo)
                ...
            event["reply"]["results"] = delta.results
    """

    def __init__(self, source):
        if isinstance(source, dict) and "args" in source:
            source = source["args"]
        if isinstance(source, str):
            source = json.loads(source)
        if isinstance(source, dict):
            self.data_set = source.get("data_set")
            changes = source.get("changes") or []
        else:
            self.data_set, changes = None, source
        self.changes = [dict(c, key=dict(c.get("key") or {}), new=dict(c.get("new") or {})) for c in changes]
        self.results = []

    def __iter__(self):
        return iter(self.changes)

    def __len__(self):
        return len(self.changes)

    def _of(self, op, data_set):
        return [c for c in self.changes if c["op"] == op and (data_set is None or c.get("data_set") == data_set)]

    def inserted(self, data_set=None):
        return self._of("insert", data_set)

    def updated(self, data_set=None):
        return self._of("update", data_set)

    def deleted(self, data_set=None):
        return self._of("delete", data_set)

    def of_data_set(self, data_set):
        return [c for c in self.changes if c.get("data_set") == data_set]

    def accept(self, change, **values):
        """Gravou; values são os que o banco definiu (a chave gerada). Os itens que dependem
        desta inclusão recebem a chave nos campos de ligação."""
        ref = change["ref"] if isinstance(change, dict) else change
        for other in self.changes:
            link = other.get("master")
            if not link or link.get("ref") != ref:
                continue
            for detail, master in (link.get("fields") or {}).items():
                if master in values:
                    for part in (other["new"], other["key"]):
                        if detail in part:
                            part[detail] = values[master]
        self.results.append({"ref": ref, "values": values} if values else {"ref": ref})

    def reject(self, change, message):
        """Não gravou: a mensagem vai para a tela e o registro continua pendente."""
        ref = change["ref"] if isinstance(change, dict) else change
        self.results.append({"ref": ref, "error": str(message)})


#: Um delta de exemplo (pedido novo com um item, um item alterado e um pedido excluído),
#: usado pelos selftests dos clientes para conferir o Delta sem banco.
SAMPLE_DELTA = (
    '{"data_set":"cdsPedidos","changes":['
    '{"ref":1,"op":"insert","data_set":"cdsPedidos","key":{"id":-1},"new":{"id":-1,"cliente":"Joao"}},'
    '{"ref":2,"op":"insert","data_set":"cdsItens","key":{"id":-2},"new":{"id":-2,"pedido_id":-1},'
    '"master":{"data_set":"cdsPedidos","ref":1,"fields":{"pedido_id":"id"}}},'
    '{"ref":3,"op":"update","data_set":"cdsItens","key":{"id":10},"old":{"produto":"A"},"new":{"produto":"B"}},'
    '{"ref":4,"op":"delete","data_set":"cdsPedidos","key":{"id":7},"old":{"id":7}}]}'
)


def delta_selftest():
    """O texto que todos os clientes escrevem ao conferir o Delta com o SAMPLE_DELTA."""
    delta = Delta(SAMPLE_DELTA)
    delta.accept(delta.inserted("cdsPedidos")[0], id=501)
    item = delta.inserted("cdsItens")[0]
    return "delta %d %d %d %s %d" % (len(delta.inserted()), len(delta.updated()), len(delta.deleted()), item["new"]["pedido_id"], len(delta.results))


class Color:
    """Uma cor do acesso tipado: RGBA (0 a 255) ou um nome do tema (`Color.of_token("primary")`),
    que acompanha a troca de tema."""

    def __init__(self, r=0, g=0, b=0, a=255, token=None):
        self.r, self.g, self.b, self.a, self.token = r, g, b, a, token

    @staticmethod
    def rgb(r, g, b, a=255):
        return Color(r, g, b, a)

    @staticmethod
    def hex(text):
        """`#RRGGBB`, `#RRGGBBAA` ou `#RGB`."""
        h = text.strip().lstrip("#")
        if len(h) == 3:
            h = "".join(c * 2 for c in h)
        try:
            r, g, b = int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16)
            a = int(h[6:8], 16) if len(h) >= 8 else 255
        except ValueError:
            raise AylisError("cor inválida: %r (use #RRGGBB ou #RRGGBBAA)" % text)
        return Color(r, g, b, a)

    @staticmethod
    def of_token(name):
        return Color(token=name.strip())

    @staticmethod
    def from_json(value):
        """O valor da tela (`#RRGGBB[AA]` ou um nome)."""
        if not isinstance(value, str) or not value:
            return Color()
        return Color.hex(value) if value.startswith("#") else Color.of_token(value)

    def to_json(self):
        if self.token:
            return self.token
        h = "#%02X%02X%02X" % (self.r, self.g, self.b)
        return h if self.a == 255 else h + "%02X" % self.a

    def __eq__(self, other):
        return isinstance(other, Color) and self.to_json() == other.to_json()

    def __hash__(self):
        return hash(self.to_json())

    def __repr__(self):
        return "Color(%r)" % self.to_json()


class Bounds:
    """Margens (`margins`, `padding`): esquerda, cima, direita e baixo."""

    def __init__(self, left=0.0, top=0.0, right=0.0, bottom=0.0):
        self.left, self.top, self.right, self.bottom = left, top, right, bottom

    @staticmethod
    def all(value):
        return Bounds(value, value, value, value)

    @staticmethod
    def symmetric(horizontal, vertical):
        return Bounds(horizontal, vertical, horizontal, vertical)

    @staticmethod
    def from_json(value):
        """O valor da tela: um número, `[h, v]`, `[l, t, r, b]` ou `{"left": ...}`."""
        if isinstance(value, (int, float)):
            return Bounds.all(value)
        if isinstance(value, list) and len(value) == 2:
            return Bounds.symmetric(value[0], value[1])
        if isinstance(value, list) and len(value) == 4:
            return Bounds(*value)
        if isinstance(value, dict):
            return Bounds(value.get("left", 0), value.get("top", 0), value.get("right", 0), value.get("bottom", 0))
        return Bounds()

    def to_json(self):
        return [self.left, self.top, self.right, self.bottom]

    def __eq__(self, other):
        return isinstance(other, Bounds) and self.to_json() == other.to_json()

    def __repr__(self):
        return "Bounds(%r, %r, %r, %r)" % (self.left, self.top, self.right, self.bottom)


def _num(v):
    """O número como a tela grava (`0.5`, `12`)."""
    return "%g" % v


class Shadow:
    """Uma sombra: o deslocamento (`dx`, `dy`), o desfoque, o espalhamento e a cor (sem cor, preto a
    25%). `None` no acesso tipado é sem sombra."""

    def __init__(self, dx=0.0, dy=0.0, blur=0.0, spread=0.0, color=None):
        self.dx, self.dy, self.blur, self.spread = dx, dy, blur, spread
        self.color = color if color is not None else Color(0, 0, 0, 64)

    @staticmethod
    def from_json(value):
        """`"dx dy desfoque [espalhamento] [#cor]"` (vazio ou inválido: None)."""
        numbers, color = [], None
        for token in str(value or "").replace(",", " ").split():
            try:
                numbers.append(float(token.removesuffix("px")))
            except ValueError:
                if color is not None or not token.startswith("#"):
                    return None
                color = Color.hex(token)
        if not 2 <= len(numbers) <= 4:
            return None
        return Shadow(numbers[0], numbers[1], numbers[2] if len(numbers) > 2 else 0.0, numbers[3] if len(numbers) > 3 else 0.0, color)

    def to_json(self):
        text = "%s %s %s" % (_num(self.dx), _num(self.dy), _num(self.blur))
        if self.spread:
            text += " " + _num(self.spread)
        return text + " " + self.color.to_json()

    def __eq__(self, other):
        return isinstance(other, Shadow) and self.to_json() == other.to_json()

    def __repr__(self):
        return "Shadow(%r)" % self.to_json()


class Easing:
    """Uma curva de suavização: uma das prontas (`Easing.OUT_QUAD`) ou a Bézier cúbica
    (`Easing.cubic_bezier(0.2, 0, 0, 1)`), como o `cubic-bezier()` do CSS."""

    NAMES = ("linear", "in_quad", "out_quad", "in_out_quad", "out_cubic", "in_out_cubic", "out_back")

    def __init__(self, name="linear", bezier=None):
        self.name, self.bezier = name, bezier

    @staticmethod
    def cubic_bezier(x1, y1, x2, y2):
        return Easing(None, (x1, y1, x2, y2))

    @staticmethod
    def from_json(value):
        text = str(value or "").strip()
        if text.startswith("cubic-bezier(") and text.endswith(")"):
            try:
                x1, y1, x2, y2 = (float(n) for n in text[13:-1].split(","))
                return Easing.cubic_bezier(x1, y1, x2, y2)
            except ValueError:
                return Easing()
        css = {"ease": (0.25, 0.1, 0.25, 1.0), "ease-in": (0.42, 0.0, 1.0, 1.0), "ease-out": (0.0, 0.0, 0.58, 1.0), "ease-in-out": (0.42, 0.0, 0.58, 1.0)}
        if text.lower() in css:
            return Easing.cubic_bezier(*css[text.lower()])
        plain = text.replace("-", "_").lower()
        return Easing(plain) if plain in Easing.NAMES else Easing()

    def to_json(self):
        if self.bezier is not None:
            return "cubic-bezier(%s)" % ", ".join(_num(v) for v in self.bezier)
        return self.name

    def __eq__(self, other):
        return isinstance(other, Easing) and self.to_json() == other.to_json()

    def __repr__(self):
        return "Easing(%r)" % self.to_json()


for _name in Easing.NAMES:
    setattr(Easing, _name.upper(), Easing(_name))


class Point:
    """Um ponto (`x`, `y`)."""

    def __init__(self, x=0.0, y=0.0):
        self.x, self.y = x, y

    def __eq__(self, other):
        return isinstance(other, Point) and (self.x, self.y) == (other.x, other.y)

    def __repr__(self):
        return "Point(%r, %r)" % (self.x, self.y)

    @staticmethod
    def list_from_json(value):
        """`"0.5,0 1,1 0,1"` (os que não são pontos ficam de fora)."""
        out = []
        for pair in str(value or "").split():
            try:
                x, y = pair.split(",")
                out.append(Point(float(x), float(y)))
            except ValueError:
                pass
        return out

    @staticmethod
    def list_to_json(points):
        return " ".join("%s,%s" % (_num(p.x), _num(p.y)) for p in points)


class AylisComponent:
    """Um componente de um form, pelo id dele na tela (resolvido quando a instância do form é
    montada). As classes de cada componente (Button, Label...) vêm do components.py gerado."""

    def __init__(self, aylis, id, name):
        self.aylis = aylis
        self.id = id
        self.name = name

    def _get(self, prop):
        return self.aylis.call("get", target=self.id, prop=prop)

    def _set(self, prop, value):
        self.aylis.send("set", target=self.id, props={prop: value})

    def _get_ref(self, prop):
        """A referência a outro componente (`popup_menu`, `data_source`), ou None."""
        info = self.aylis.call("get", target=self.id, prop=prop, ref=True)
        return AylisComponent(self.aylis, info["id"], info["name"]) if info else None

    def _set_ref(self, prop, component):
        self._set(prop, component.id if component is not None else None)

    def _component(self, info):
        """O componente que um método devolveu (com `ref=True`: o id e o nome), ou None."""
        return AylisComponent(self.aylis, info["id"], info["name"]) if info else None


    def set_focus(self):
        self.aylis.send("focus", target=self.id)


class AylisForm(AylisComponent):
    """A base dos forms: cada form aberto na tela tem a sua instância
    (duas janelas do mesmo form, duas instâncias), com os componentes resolvidos ao abrir. A
    classe gerada de cada form monta os componentes em bind."""

    def __init__(self, aylis, id, name):
        super().__init__(aylis, id, name)
        self._parts = []

    def bind(self, components):
        """Monta os componentes a partir da lista da tela (a classe gerada sobrescreve)."""

    def _resolve(self, components, name, cls_name, cls):
        item = next((c for c in components if c["name"] == name), None)
        if item is None:
            raise AylisError("o form %s não bate com o código: falta o componente %s (%s)" % (self.name, name, cls_name))
        if item["class"] != cls_name:
            raise AylisError("o form %s não bate com o código: %s é %s no form e %s no código" % (self.name, name, item["class"], cls_name))
        part = cls(self.aylis, item["id"], name)
        self._parts.append(part)
        return part

    def _resolve_optional(self, components, name, cls):
        item = next((c for c in components if c["name"] == name), None)
        if item is None:
            return None
        part = cls(self.aylis, item["id"], name)
        self._parts.append(part)
        return part

    def component_by_id(self, id):
        """O componente deste form com o id (o do evento)."""
        if id == self.id:
            return self
        return next((p for p in self._parts if p.id == id), None)

    def show(self):
        self.aylis.send("show", target=self.id)

    def close(self):
        self.aylis.send("close", target=self.id)


class CurrentForm:
    """O frmX do components.py gerado: repassa tudo para a janela do form cujo evento está
    rodando (a primeira, fora dos handlers). O `from .components import frmX` dos arquivos dos
    forms guarda este objeto, que não muda; a janela por trás dele muda a cada evento."""

    def __init__(self, name):
        object.__setattr__(self, "_name", name)
        object.__setattr__(self, "_form", None)

    def _activate(self, form):
        object.__setattr__(self, "_form", form)

    def __getattr__(self, attr):
        form = object.__getattribute__(self, "_form")
        if form is None:
            raise AylisError("o form %s ainda não foi aberto" % object.__getattribute__(self, "_name"))
        return getattr(form, attr)

    def __setattr__(self, attr, value):
        form = object.__getattribute__(self, "_form")
        if form is None:
            raise AylisError("o form %s ainda não foi aberto" % object.__getattribute__(self, "_name"))
        setattr(form, attr, value)


_form_classes = {}


def register_form_class(name, cls, activate):
    """Registra a classe de um form (o components.py gerado faz): cada form com esse nome
    aberto na tela vira uma instância dela; activate a torna a atual."""
    _form_classes[name.lower()] = (cls, activate)


class Aylis:
    # A versão do protocolo do aylis que este cliente fala (o hello, a primeira mensagem).
    PROTOCOL = 1
    _greeted = False

    def __init__(self, listen=None):
        """Sem `listen`, conversa por stdin/stdout (o aylis-host inicia este processo). Com
        `listen="host:porta"`, espera a tela conectar (aylis-host --connect)."""
        self._ids = itertools.count(1)
        self._handlers = {}
        # As instâncias dos forms, pelo id do form na tela.
        self._forms = {}
        # Os forms liberados na tela (o aviso __freed).
        self._freed = set()
        # Envio de qualquer thread: uma mensagem por vez (senão as linhas se misturam).
        self._write_lock = threading.Lock()
        self._main = threading.current_thread()
        self._posted = queue.Queue()
        self._queued = []
        self._lines = []
        # Esperando um diálogo (message_dlg, show_modal sem on_close): os eventos rodam já.
        self._waiting = 0
        # Handlers rodando: um evento que pergunta roda já, mesmo com um deles num call.
        self._running = 0
        # As respostas dos eventos que perguntam em andamento (a última é a do handler que roda),
        # para o reply() do handler tipado, que não recebe o evento.
        self._replies = []
        self._ws = WebSocket.accept(listen) if listen else None
        if self._ws is None:
            # O protocolo é UTF-8 nos dois sentidos (o padrão do Windows não é).
            sys.stdin.reconfigure(encoding="utf-8")
            sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")

    # ----- envio -----

    def _greet(self):
        """A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes)."""
        if not self._greeted:
            self._greeted = True
            self._write({"cmd": "hello", "protocol": self.PROTOCOL, "client": "python", "features": ["done"]})

    def _write(self, message):
        text = json.dumps(message, ensure_ascii=False)
        with self._write_lock:
            if self._ws is not None:
                self._ws.send(text)
                return
            sys.stdout.write(text + "\n")
            sys.stdout.flush()

    def post(self, function):
        """Roda function() na thread principal do backend (pode chamar de qualquer thread).
        A tela responde a um pedido vazio, o que acorda o laço de eventos para rodar a função."""
        self._posted.put(function)
        self._greet()
        self._write({"id": "__post", "cmd": "tr", "text": ""})

    def post_to(self, owner, function):
        """Como o post, ligado a um form: se ele foi liberado até lá (a janela fechou e saiu),
        function não roda."""
        self.post(lambda: None if owner.id in self._freed else function())

    def _run_posted(self):
        while True:
            try:
                function = self._posted.get_nowait()
            except queue.Empty:
                return
            try:
                function()
            except Exception as exc:  # uma função com erro não derruba o backend
                log("erro numa função de post:", repr(exc))

    def reply(self, name, value):
        """Dentro de um evento que pergunta: grava um campo da resposta, como o
        event["reply"][name] do handler que recebe o evento inteiro (o handler tipado não o
        recebe). reply("key", "") no on_key_down engole a tecla; uma exceção recusa.
        Fora de um evento que pergunta, não faz nada."""
        if self._replies and self._replies[-1] is not None:
            self._replies[-1][name] = value

    def send(self, cmd, **fields):
        """Envia um comando sem esperar resposta (erros chegam como evento "error")."""
        self._greet()
        self._write({"cmd": cmd, **fields})

    def call(self, cmd, **fields):
        """Envia um comando e espera o resultado. Eventos que chegarem antes ficam na fila.
        Só na thread principal (de outra, use send ou post)."""
        if threading.current_thread() is not self._main:
            raise AylisError("call() só na thread principal; de outra thread use send() ou post()")
        self._greet()
        request_id = next(self._ids)
        self._write({"id": request_id, "cmd": cmd, **fields})
        while True:
            message = self._read()
            if message is None:
                raise AylisError("a tela foi fechada")
            if message.get("id") == request_id:
                if "error" in message:
                    raise AylisError(message["error"])
                return message.get("result")
            if message.get("id") == "__post" and self._waiting > 0:
                self._run_posted()  # esperando um diálogo, os post também rodam
                continue
            if message.get("event") == "ping" and self._waiting > 0:
                self.send("pong")  # a sincronia dos roteiros: o que veio antes já rodou
                continue
            # Esperando um diálogo, os eventos (os do modal) rodam já; um evento que pergunta
            # também (o data post do próprio backend dispara o before_post).
            event = message.get("event")
            if event and event not in ("closed", "ping") and (self._waiting > 0 or ("ask" in message and self._running < 8)):
                self._handle(message)
                continue
            self._queued.append(message)

    def _wait_call(self, cmd, **fields):
        self._waiting += 1
        # O que chegou antes (enquanto outro call esperava) roda já.
        queued, self._queued = self._queued, []
        for message in queued:
            event = message.get("event")
            if message.get("id") == "__post":
                self._run_posted()
            elif event == "ping":
                self.send("pong")
            elif event and event not in ("closed", "error"):
                self._handle(message)
            else:
                self._queued.append(message)
        try:
            return self.call(cmd, wait=True, **fields)
        finally:
            self._waiting -= 1

    def create_form(self, name):
        """Cria um form do projeto; se já existe, devolve o mesmo."""
        return self.call("create_form", name=name)

    def show(self, form):
        """Abre o form numa janela, sem bloquear os outros (como Show)."""
        self.send("show", target=form)

    def show_modal(self, form, on_close=None, container=None):
        """Abre o form como modal. Sem on_close, espera ele fechar e devolve o resultado
        ("ok", "cancel"...), e os eventos continuam rodando. Com
        on_close, não bloqueia: on_close(result) recebe o resultado. Com container (um
        componente de outro form, como a página de uma aba), o modal fica só nele e o resto
        da tela continua respondendo."""
        extra = {"container": container} if container else {}
        if on_close is None:
            return self._wait_call("show_modal", target=form, **extra)
        handler = None
        if on_close is not None:
            handler = "__modal_%d" % next(self._ids)

            def done(event):
                del self._handlers[handler]
                on_close(event.get("args"))

            self._handlers[handler] = done
        self.send("show_modal", target=form, handler=handler, **extra)

    def close(self, form):
        """Fecha o form (o on_close_query pode impedir). Fechar o principal encerra."""
        return self.call("close", target=form)

    def tr(self, text):
        """O texto no idioma do aplicativo (locales/<idioma>.json do projeto)."""
        return self.call("tr", text=text)

    def set_language(self, language):
        """Troca o idioma com o app aberto (formatos e textos da tela)."""
        self.send("set_language", language=language)

    def toast(self, text):
        """Aviso rápido no rodapé da tela, que some sozinho."""
        self.send("toast", text=text)

    @contextlib.contextmanager
    def profile(self, name):
        """Marca um trecho no profiler da IDE (a faixa "Marcas"):
        with aylis.profile("consulta"): ..."""
        self.send("profile_begin", name=name)
        try:
            yield
        finally:
            self.send("profile_end")

    def message_dlg(self, text, kind="information", buttons=("ok",), on_close=None, icon=None, icon_color=None):
        """Caixa de mensagem (como o MessageDlg). kind: information, warning, error,
        confirmation; buttons: "ok", "cancel", "yes", "no"... Sem on_close, espera e devolve o
        botão ("yes"), e os eventos continuam rodando. Com
        on_close, não bloqueia: on_close(result) recebe o botão escolhido. icon (nome de
        ícone) e icon_color (token do tema, como "danger", ou "#rrggbb") trocam o ícone do tipo."""
        extra = _dialog_icon(icon, icon_color)
        if on_close is None:
            return self._wait_call("message_dlg", text=text, kind=kind, buttons=list(buttons), **extra)
        handler = None
        if on_close is not None:
            handler = "__msg_%d" % next(self._ids)

            def done(event):
                del self._handlers[handler]
                on_close(event.get("args"))

            self._handlers[handler] = done
        return self.call("message_dlg", text=text, kind=kind, buttons=list(buttons), handler=handler, **extra)

    def prompt(self, title, label, value="", required=False, on_close=None, icon=None, icon_color=None):
        """A pergunta com um campo de texto (OK e Cancelar). Sem on_close, espera e devolve o
        texto digitado, ou None no Cancelar, e os eventos continuam rodando. Com on_close, não
        bloqueia: on_close(texto ou None). required não deixa confirmar vazio."""
        fields = {"title": title, "label": label, "value": value, **_dialog_icon(icon, icon_color)}
        if required:
            fields["required"] = True
        if on_close is None:
            return self._wait_call("prompt", **fields)
        handler = "__prompt_%d" % next(self._ids)

        def done(event):
            del self._handlers[handler]
            on_close(event.get("args"))

        self._handlers[handler] = done
        return self.call("prompt", handler=handler, **fields)

    def field_decimal(self, dataset, field):
        """O valor de um campo decimal (Currency, BCD) como Decimal, exato: o protocolo manda o
        texto, e ele nunca passa por float (0.1 + 0.2 continua 0.3). None se o campo é nulo."""
        value = self.data(dataset, "value", field=field)
        return None if value is None else decimal.Decimal(str(value))

    def execute(self, dialog):
        """Abre um OpenDialog/SaveDialog; o caminho chega no on_select do componente."""
        self.send("execute", target=dialog)

    def data(self, target, op, **fields):
        """Atalho para operações de dados: aylis.data("cdsClientes", "next")."""
        return self.call("data", target=target, op=op, **fields)

    # ----- eventos -----

    def on(self, handler_name):
        def register(function):
            self._handlers[handler_name] = function
            return function
        return register

    def _read(self):
        if self._ws is not None:
            while not self._lines:
                text = self._ws.recv()
                if text is None:
                    return None
                self._lines.extend(l for l in text.split("\n") if l.strip())
            return json.loads(self._lines.pop(0))
        line = sys.stdin.readline()
        if not line:
            return None
        return json.loads(line)

    def form_instance(self, form, id):
        """A instância do form da tela (criada e montada na primeira vez), já como a atual;
        None se a classe dele não foi registrada. Um form que não bate com o código levanta
        AylisError."""
        entry = _form_classes.get((form or "").lower())
        if entry is None:
            return None
        cls, activate = entry
        instance = self._forms.get(id)
        if instance is None:
            instance = cls(self, id, form)
            instance.bind(self.call("components", target=id))
            self._forms[id] = instance
        activate(instance)
        return instance

    def create_form(self, form, another=False):
        """Cria o form na tela e devolve a instância dele; another: mais uma janela do mesmo form."""
        created = self.call("create_form", name=form, new=another)
        return self.form_instance(form, created["id"])

    def run(self, form=None):
        """Abre a janela e trata os eventos até ela fechar. Sem form, abre o principal
        do projeto (aylis-host --project)."""
        self.call("run", form=form)
        # A instância do form principal antes do primeiro evento (o código pode usar frmX).
        if form:
            found = self.call("find", name=form)
            if isinstance(found, int):
                self.form_instance(form, found)
        self.loop()

    def loop(self):
        while True:
            self._run_posted()
            message = self._queued.pop(0) if self._queued else self._read()
            if message is None or message.get("event") == "closed":
                return
            if message.get("id") == "__post":
                continue  # só acordou o laço (as funções de post rodam no começo da volta)
            if message.get("event") == "ping":
                # Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes.
                self.send("pong")
                continue
            if message.get("event") == "error":
                log("erro na tela:", message.get("message"), "(comando:", message.get("cmd"), ")")
                continue
            self._handle(message)

    def _handle(self, message):
        """Roda o handler do evento. Um evento que pergunta (com "ask") recebe a resposta
        quando o handler termina: o message["reply"] (can_close, key, data, label, accept)
        ou, com exceção, a recusa com a mensagem (a tela mostra)."""
        if message.get("event") == "__freed":
            # Forms liberados na tela: as instâncias saem e os post_to deles não rodam.
            for form_id in message.get("forms") or []:
                self._freed.add(form_id)
                self._forms.pop(form_id, None)
            return
        message["reply"] = {}
        ask = message.get("ask")
        # Primeiro o handler do form ("frmClientes.btn_salvar_click"), depois o geral.
        name = message.get("handler")
        form = message.get("form")
        handler = self._handlers.get("%s.%s" % (form, name)) if form else None
        if handler is None:
            handler = self._handlers.get(name)
        self._running += 1
        self._replies.append(message["reply"] if ask is not None else None)
        try:
            if handler is None:
                log("evento sem handler:", message.get("handler"))
            else:
                # A janela do evento vira a atual (frmX aponta para ela durante o handler).
                if message.get("form_id") is not None:
                    self.form_instance(form, message["form_id"])
                handler(message)
            if ask is not None:
                self.send("answer", ask=ask, values=message["reply"])
        except Exception as exc:  # um handler com erro não derruba o backend
            if ask is not None:
                self.send("answer", ask=ask, error=str(exc))
            else:
                log("erro no handler", message.get("handler"), "->", repr(exc))
                # A tela mostra o erro ao usuário.
                self.send("exception", message=str(exc), path=form or "", handler=name or "")
        finally:
            self._running -= 1
            self._replies.pop()
            # O handler terminou: a tela deixa de mostrar o backend ocupado.
            if message.get("seq") is not None:
                self.send("done", seq=message["seq"])


def handler(name):
    """Marca um método de uma Session como handler ("btn_ok_click" ou
    "frmVendas.btn_ok_click")."""
    def mark(function):
        function._aylis_handler = name
        return function
    return mark


class Session(Aylis):
    """Uma tela ligada pela ponte com --sessions (aylis-host --bridge ... --sessions): um
    processo atende todas as telas, e cada uma ganha uma instância desta classe, numa thread
    própria (um handler demorado não trava as outras telas; uma exceção fica nela).

    Guarde o estado do usuário nos atributos (self.total), nunca em variáveis globais: elas
    são de todos. A API é a do Aylis (call, send, toast, message_dlg, data...).

        class Vendas(aylis_client.Session):
            form = "frmVendas"

            def started(self):
                self.total = 0
                self.run()

            @aylis_client.handler("btn_somar_click")
            def somar(self, event):
                self.total += 1

        aylis_client.serve(Vendas)
    """

    # O form que run() abre (None: o principal do projeto).
    form = None

    def __init__(self, server, number):
        # Sem o __init__ do Aylis: o stdin e o stdout são do servidor.
        self._server = server
        self.number = number
        self._ids = server._ids
        self._write_lock = server._write_lock
        self._main = None
        self._posted = queue.Queue()
        self._queued = []
        self._lines = []
        self._ws = None
        self._inbox = queue.Queue()
        self._forms = {}
        self._freed = set()
        self._waiting = 0
        self._running = 0
        # As respostas dos eventos que perguntam (o reply() do handler tipado), como no Aylis.
        self._replies = []
        self._handlers = {}
        for attribute in dir(type(self)):
            function = getattr(self, attribute, None)
            name = getattr(function, "_aylis_handler", None)
            if name:
                self._handlers[name] = function

    def _write(self, message):
        self._server._write({**message, "session": self.number})

    def _read(self):
        return self._inbox.get()

    def post(self, function):
        """Roda function() na thread desta sessão (pode chamar de qualquer thread)."""
        self._posted.put(function)
        self._inbox.put({"id": "__post"})

    def run(self, form=None):
        """Abre a tela desta sessão (os eventos começam depois de started)."""
        self.call("run", form=form or self.form)

    def started(self):
        """A tela conectou: monte o estado e abra o form (o padrão: run())."""
        self.run()

    def closed(self):
        """A tela saiu (fechou o app ou caiu a rede)."""

    # O usuário logado (o mesmo objeto em todas as telas dele).
    user = None

    def login(self, user_id):
        """Liga esta tela ao usuário user_id: as telas do mesmo usuário (duas janelas, duas
        abas) dividem o mesmo User (self.user), com os atributos dele e o broadcast."""
        self.logout()
        self.user = self._server._user(user_id)
        with self.user._lock:
            self.user.sessions.append(self)

    def logout(self):
        """Solta o usuário desta tela (sozinho, ao fechar)."""
        if self.user is not None:
            self._server._leave(self.user, self)
            self.user = None

    def _work(self):
        self._main = threading.current_thread()
        try:
            self.started()
            self.loop()
        except Exception as exc:  # uma sessão com erro não derruba as outras
            log("erro na sessão", self.number, "->", repr(exc))
        finally:
            try:
                self.closed()
            except Exception as exc:
                log("erro em closed da sessão", self.number, "->", repr(exc))
            self.logout()
            self._server._ended(self.number)


class User:
    """O usuário logado, dividido pelas telas dele (Session.login). Guarde nos atributos o
    que vale para todas (o carrinho, as preferências); mexa com `with user.lock:` quando
    duas telas podem mexer ao mesmo tempo."""

    def __init__(self, user_id):
        self.id = user_id
        self.sessions = []
        self._lock = threading.Lock()

    @property
    def lock(self):
        return self._lock

    def broadcast(self, function):
        """Roda function(session) na thread de cada tela do usuário (atualizar todas)."""
        with self._lock:
            sessions = list(self.sessions)
        for session in sessions:
            session.post(lambda s=session: function(s))


class _Server:
    """O processo das sessões: lê o stdin e entrega cada mensagem à sessão dela."""

    def __init__(self, session_class):
        self._class = session_class
        self._ids = itertools.count(1)
        self._write_lock = threading.Lock()
        self._sessions = {}
        self._lock = threading.Lock()
        sys.stdin.reconfigure(encoding="utf-8")
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stderr.reconfigure(encoding="utf-8")

    def _write(self, message):
        text = json.dumps(message, ensure_ascii=False)
        with self._write_lock:
            sys.stdout.write(text + "\n")
            sys.stdout.flush()

    def _ended(self, number):
        with self._lock:
            self._sessions.pop(number, None)

    def _user(self, user_id):
        """O User de user_id (criado na primeira tela que loga)."""
        with self._lock:
            users = self.__dict__.setdefault("_users", {})
            if user_id not in users:
                users[user_id] = User(user_id)
            return users[user_id]

    def _leave(self, user, session):
        """A tela saiu do usuário; sem telas, ele é esquecido."""
        with user._lock:
            if session in user.sessions:
                user.sessions.remove(session)
            empty = not user.sessions
        if empty:
            with self._lock:
                self.__dict__.get("_users", {}).pop(user.id, None)

    def serve(self):
        for line in sys.stdin:
            if not line.strip():
                continue
            message = json.loads(line)
            number = message.pop("session", None)
            if number is None:
                continue
            if message.get("event") == "session_open":
                session = self._class(self, number)
                with self._lock:
                    self._sessions[number] = session
                threading.Thread(target=session._work, name="sessao-%s" % number, daemon=True).start()
                continue
            with self._lock:
                session = self._sessions.get(number)
            if session is None:
                continue
            if message.get("event") == "session_close":
                session._inbox.put({"event": "closed"})
            else:
                session._inbox.put(message)


def serve(session_class):
    """Atende as telas da ponte com --sessions: uma instância de session_class por tela."""
    _Server(session_class).serve()


def log(*args):
    print("[backend]", *args, file=sys.stderr, flush=True)
