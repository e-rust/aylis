"""Backend de teste: uma thread de trabalho manda comandos (send), tenta um call (que é
recusado fora da thread principal) e pede para rodar uma função na principal (post)."""

import os
import sys
import threading

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "clients", "python"))
from aylis_client import Aylis, AylisError  # noqa: E402

aylis = Aylis()
aylis.send("load", layout={"class": "Form", "name": "frm", "width": 200, "height": 80,
                          "children": [{"class": "Label", "name": "lbl", "text": "0"}]})
aylis.call("run", form="frm")


def worker():
    try:
        aylis.call("get", target="lbl", prop="text")
    except AylisError as exc:
        print("SELFTEST call recusado:", exc, file=sys.stderr, flush=True)
    # Muitos envios seguidos de outra thread, junto com o laço da principal.
    for i in range(200):
        aylis.send("set", target="lbl", props={"text": str(i)})

    def done():
        text = aylis.call("get", target="lbl", prop="text")
        main = threading.current_thread() is threading.main_thread()
        print("SELFTEST post na principal=%s texto=%s" % (main, text), file=sys.stderr, flush=True)
        aylis.send("close")

    aylis.post(done)


threading.Thread(target=worker).start()
aylis.loop()
