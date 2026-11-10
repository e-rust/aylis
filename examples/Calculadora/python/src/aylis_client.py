"""O cliente do protocolo vem do aylis (a instalação, ou o repositório): este arquivo só o
carrega. Gerado pela IDE do Aylis a cada "Gerar código": não edite."""

import importlib.util as _util
import os as _os
import sys as _sys

_path = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "../../../../clients/python/aylis_client.py")
_spec = _util.spec_from_file_location("aylis_client", _path)
_module = _util.module_from_spec(_spec)
_sys.modules["aylis_client"] = _module
_spec.loader.exec_module(_module)
