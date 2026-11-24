"""Backend de teste: o `on_create` do DataModule criado no início do projeto chega depois do
`run` (quando o backend já registrou os handlers). Sem ele em 15 s, fecha e avisa."""

import os
import sys
import threading

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "clients", "python"))
from aylis_client import Aylis  # noqa: E402

aylis = Aylis()


@aylis.on("dm_criado")
def dm_criado(event):
    print("SELFTEST on_create do DataModule", file=sys.stderr, flush=True)
    aylis.send("close")


def give_up():
    print("SELFTEST sem on_create", file=sys.stderr, flush=True)
    aylis.send("close")


aylis.call("run")
timer = threading.Timer(15, lambda: aylis.post(give_up))
timer.daemon = True
timer.start()
aylis.loop()
