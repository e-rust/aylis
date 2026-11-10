"""Backend de teste das sessões (aylis-host --bridge ... --sessions): um processo para todas
as telas, um contador por tela; "boom" lança uma exceção que fica só naquela sessão,
"lento" demora sem travar as outras e "entrar"/"comum" ligam as telas do mesmo usuário."""

import os
import sys
import time

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "clients", "python"))
import aylis_client  # noqa: E402

LAYOUT = {"class": "Form", "name": "frm", "width": 200, "height": 80, "children": [
    {"class": "Label", "name": "lbl", "text": "-"},
    {"class": "Button", "name": "btn", "text": "+", "on_click": "somar"},
    {"class": "Button", "name": "boom", "text": "!", "on_click": "boom"},
    {"class": "Button", "name": "lento", "text": "z", "on_click": "lento"},
    {"class": "Label", "name": "lblu", "text": "-"},
    {"class": "Button", "name": "entrar", "text": "e", "on_click": "entrar"},
    {"class": "Button", "name": "comum", "text": "c", "on_click": "comum"},
]}


class Contador(aylis_client.Session):
    form = "frm"

    def started(self):
        self.total = 0
        print("SELFTEST sessão %s abriu (pid %s)" % (self.number, os.getpid()), file=sys.stderr, flush=True)
        self.send("load", layout=LAYOUT)
        self.run()

    @aylis_client.handler("somar")
    def somar(self, event):
        self.total += 1
        self.send("set", target="frm.lbl", props={"text": "%s:%s" % (self.number, self.total)})

    @aylis_client.handler("boom")
    def boom(self, event):
        raise RuntimeError("erro de propósito")

    @aylis_client.handler("lento")
    def lento(self, event):
        time.sleep(3)

    @aylis_client.handler("entrar")
    def entrar(self, event):
        self.login("ana")
        self.send("set", target="frm.lblu", props={"text": "ana"})

    @aylis_client.handler("comum")
    def comum(self, event):
        # O contador do usuário: o mesmo nas telas dele; todas mostram.
        with self.user.lock:
            self.user.total = getattr(self.user, "total", 0) + 1
            total = self.user.total
        self.user.broadcast(lambda s: s.send("set", target="frm.lblu", props={"text": "u:%s" % total}))

    def closed(self):
        print("SELFTEST sessão %s saiu" % self.number, file=sys.stderr, flush=True)


aylis_client.serve(Contador)
