"""O aylis usado pela C ABI (aylis.dll) a partir de Python com ctypes — no mesmo processo.

O mesmo vale para Delphi, C#, Go ou C: carregar a biblioteca e chamar as funções.

    cargo build -p aylis-capi
    python clients/python/capi_demo.py              # abre a janela
    python clients/python/capi_demo.py --png t.png  # roda um clique e grava a tela
"""

import ctypes
import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
NAMES = {"win32": "aylis.dll", "darwin": "libaylis.dylib"}
LIBRARY = NAMES.get(sys.platform, "libaylis.so")


def load_library():
    for profile in ("release", "debug"):
        path = os.path.join(ROOT, "target", profile, LIBRARY)
        if os.path.exists(path):
            return ctypes.CDLL(path)
    sys.exit(f"{LIBRARY} não encontrada: rode `cargo build -p aylis-capi` antes")


lib = load_library()
EVENT_FN = ctypes.CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_char_p, ctypes.c_void_p)
lib.aylis_new.restype = ctypes.c_void_p
lib.aylis_free.argtypes = [ctypes.c_void_p]
lib.aylis_execute.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
lib.aylis_execute.restype = ctypes.c_void_p  # liberado com aylis_string_free
lib.aylis_ctx_execute.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
lib.aylis_ctx_execute.restype = ctypes.c_void_p
lib.aylis_string_free.argtypes = [ctypes.c_void_p]
lib.aylis_set_event_callback.argtypes = [ctypes.c_void_p, EVENT_FN, ctypes.c_void_p]
lib.aylis_run.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
lib.aylis_render_png.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p]


def call(function, handle, command):
    """Executa um comando e devolve o resultado (ou levanta o erro)."""
    raw = function(handle, json.dumps(command, ensure_ascii=False).encode("utf-8"))
    try:
        reply = json.loads(ctypes.string_at(raw).decode("utf-8"))
    finally:
        lib.aylis_string_free(raw)
    if "error" in reply:
        raise RuntimeError(reply["error"])
    return reply["result"]


app = lib.aylis_new()
call(lib.aylis_execute, app, {"cmd": "load", "layout": {
    "class": "Form", "name": "FrmOla", "caption": "aylis + Python (C ABI)", "width": 460, "height": 220,
    "padding": 16,
    "children": [
        {"class": "Label", "text": "Seu nome:", "align": "top", "height": 24},
        {"class": "Edit", "name": "edtNome", "text": "Maria", "align": "top", "height": 34,
         "margins": [0, 0, 0, 12]},
        {"class": "Button", "name": "btnOla", "text": "Dizer olá", "align": "top", "height": 34,
         "on_click": "ola_click"},
        {"class": "Label", "name": "lblResposta", "align": "client", "font_size": 20, "h_align": "center"},
    ],
}})


@EVENT_FN
def on_event(ctx, event_json, user):
    """Chamado de forma síncrona, na thread da tela, a cada evento com handler por nome."""
    event = json.loads(event_json.decode("utf-8"))
    if event["handler"] == "ola_click":
        nome = call(lib.aylis_ctx_execute, ctx, {"cmd": "get", "target": "edtNome", "prop": "text"})
        call(lib.aylis_ctx_execute, ctx, {"cmd": "set", "target": "lblResposta",
                                          "props": {"text": f"Olá, {nome}! (respondido pelo Python)"}})


lib.aylis_set_event_callback(app, on_event, None)

if "--png" in sys.argv:
    path = sys.argv[sys.argv.index("--png") + 1]
    call(lib.aylis_execute, app, {"cmd": "set", "target": "edtNome", "props": {"text": "Eduardo"}})
    call(lib.aylis_execute, app, {"cmd": "click", "target": "btnOla"})
    resposta = call(lib.aylis_execute, app, {"cmd": "get", "target": "lblResposta", "prop": "text"})
    print("resposta:", resposta)
    print("png:", lib.aylis_render_png(app, b"FrmOla", path.encode("utf-8")))
else:
    lib.aylis_run(app, b"FrmOla")
lib.aylis_free(app)
