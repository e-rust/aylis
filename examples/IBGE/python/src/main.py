"""IBGE: o ponto de entrada. Gerado pela IDE do aylis (pode editar).

Executado pelo aylis-host: aylis-host --project ../IBGE.aylisproj -- python src/main.py
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from aylis_client import Aylis  # noqa: E402
import forms  # noqa: E402

aylis = Aylis()
forms.register(aylis)
aylis.run()  # abre o form principal do projeto
