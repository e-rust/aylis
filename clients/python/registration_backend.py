"""Backend em Python para uma tela de clientes do aylis.

A tela (frontend) não sabe nada de banco: recebe registros, mostra, edita e manda de
volta o delta. Este backend guarda os dados num SQLite em memória e decide o que gravar.

    aylis-host -- python clients/python/registration_backend.py
    aylis-host --png tela.png -- python clients/python/registration_backend.py --selftest
"""

import os
import sqlite3
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from aylis_client import Aylis, delta_selftest, log  # noqa: E402

SELFTEST = "--selftest" in sys.argv

# ----- "banco de dados" do backend -----

db = sqlite3.connect(":memory:")
db.execute("""create table clientes (
    id integer primary key autoincrement,
    nome text not null,
    email text,
    limite text,
    ativo integer not null default 1)""")
NOMES = ["Maria Silva", "João Pereira", "Ana Costa", "Carlos Souza", "Paula Nogueira", "Lucas Lima",
         "Fernanda Rocha", "Rafael Alves", "Juliana Melo", "Bruno Dias"]
for i, nome in enumerate(NOMES * 3):
    db.execute("insert into clientes (nome, email, limite, ativo) values (?, ?, ?, ?)",
               (f"{nome} {i + 1}", f"cliente{i + 1}@exemplo.com.br", f"{(i + 1) * 350:.2f}", i % 4 != 0))
db.commit()


def ler_clientes():
    rows = db.execute("select id, nome, email, limite, ativo from clientes order by id").fetchall()
    return [{"id": r[0], "nome": r[1], "email": r[2], "limite": r[3], "ativo": bool(r[4])} for r in rows]


# ----- a tela -----

aylis = Aylis()
aylis.send("locale", locale="pt-BR")
aylis.send("load", layout={
    "class": "Form", "name": "FrmClientes", "caption": "aylis + Python (SQLite no backend)",
    "width": 920, "height": 560, "padding": 10,
    "children": [
        {"class": "ClientDataSet", "name": "cdsClientes", "key_fields": "id", "active": True,
         "on_apply_updates": "gravar_clientes",
         "before_post": "antes_de_gravar",
         "children": [
             {"class": "Field", "name": "id", "type": "auto_inc", "display_label": "Código"},
             {"class": "Field", "name": "nome", "type": "string", "size": 60, "display_label": "Nome", "required": True},
             {"class": "Field", "name": "email", "type": "string", "size": 80, "display_label": "E-mail", "char_case": "lower"},
             {"class": "Field", "name": "limite", "type": "currency", "display_label": "Limite",
              "display_format": "R$ #,##0.00", "min": 0},
             {"class": "Field", "name": "ativo", "type": "boolean", "display_label": "Ativo", "default_value": "true"},
         ]},
        {"class": "DataSource", "name": "dsClientes", "data_set": "cdsClientes"},
        {"class": "Layout", "align": "top", "height": 36, "margins": [0, 0, 0, 10], "children": [
            {"class": "DataNavigator", "data_source": "dsClientes", "align": "left", "width": 324,
             "margins": [0, 2, 12, 2]},
            {"class": "Button", "name": "btnGravar", "text": "Gravar no banco", "align": "left", "width": 150,
             "margins": [0, 0, 6, 0], "on_click": "gravar_click"},
            {"class": "Button", "name": "btnRecarregar", "text": "Recarregar", "align": "left", "width": 110,
             "style": "flat", "on_click": "recarregar_click"},
        ]},
        {"class": "Label", "name": "lblStatus", "align": "bottom", "height": 24, "margins": [0, 6, 0, 0],
         "text": "Dados vindos do SQLite do backend Python."},
        {"class": "DataGrid", "name": "grdClientes", "data_source": "dsClientes", "align": "client"},
    ],
})
aylis.data("cdsClientes", "load", rows=ler_clientes())


def status(texto):
    aylis.send("set", target="lblStatus", props={"text": texto})


# ----- regras de negócio do backend -----

# Pergunta antes de gravar o registro: a tela espera a
# resposta, e o data post do backend só volta depois dela. raise recusa, com a mensagem.
@aylis.on("antes_de_gravar")
def antes_de_gravar(event):
    nome = aylis.data("cdsClientes", "value", field="nome")
    if not nome:
        raise ValueError("Informe o nome")
    if SELFTEST:
        log(f"SELFTEST before_post nome={nome}")


@aylis.on("gravar_click")
def gravar_click(event):
    aylis.send("data", target="cdsClientes", op="apply")


@aylis.on("recarregar_click")
def recarregar_click(event):
    aylis.data("cdsClientes", "load", rows=ler_clientes())
    status("Recarregado do banco.")


@aylis.on("gravar_clientes")
def gravar_clientes(event):
    """Recebe o delta e grava no SQLite, uma alteração por vez."""
    results, gravadas, recusadas = [], 0, []
    for change in event["args"]["changes"]:
        ref, op = change["ref"], change["op"]
        try:
            if op == "delete":
                if change["key"]["id"] == 1:
                    raise ValueError("o cliente 1 tem pedidos e não pode ser excluído")
                db.execute("delete from clientes where id = ?", (change["key"]["id"],))
                results.append({"ref": ref})
            elif op == "update":
                novo = change["new"]
                if "limite" in novo and novo["limite"] is not None and float(novo["limite"]) > 50000:
                    raise ValueError("limite acima de R$ 50.000,00 precisa de aprovação")
                sets = ", ".join(f"{campo} = ?" for campo in novo)
                db.execute(f"update clientes set {sets} where id = ?", (*novo.values(), change["key"]["id"]))
                results.append({"ref": ref})
            elif op == "insert":
                novo = {k: v for k, v in change["new"].items() if k != "id"}
                campos = ", ".join(novo)
                marcas = ", ".join("?" for _ in novo)
                cursor = db.execute(f"insert into clientes ({campos}) values ({marcas})", tuple(novo.values()))
                results.append({"ref": ref, "values": {"id": cursor.lastrowid}})
            gravadas += 1
        except Exception as exc:
            results.append({"ref": ref, "error": str(exc)})
            recusadas.append(str(exc))
    db.commit()
    aylis.send("data", target="cdsClientes", op="reconcile", results=results)
    texto = f"Banco: {gravadas} alteração(ões) gravada(s)"
    if recusadas:
        texto += f", {len(recusadas)} recusada(s): {recusadas[0]}"
    status(texto)
    log(texto)


# ----- execução -----

if SELFTEST:
    # Roteiro automático: altera, inclui, exclui, grava e confere no banco.
    aylis.call("run", form="FrmClientes")
    aylis.data("cdsClientes", "move_to", index=1)
    aylis.data("cdsClientes", "edit")
    aylis.data("cdsClientes", "set_text", field="limite", text="1.234,50")
    # O decimal exato (o texto do protocolo vira Decimal, sem passar por float).
    log(f"SELFTEST decimal={aylis.field_decimal('cdsClientes', 'limite')!r}")
    aylis.data("cdsClientes", "append_record")
    aylis.data("cdsClientes", "set_value", field="nome", value="Novo Cliente Python")
    aylis.data("cdsClientes", "post")
    aylis.data("cdsClientes", "first")
    aylis.data("cdsClientes", "delete")  # o backend vai recusar (cliente 1)
    aylis.send("data", target="cdsClientes", op="apply")
    # o evento com o delta chega; o handler grava e responde
    while True:
        message = aylis._queued.pop(0) if aylis._queued else aylis._read()
        if message.get("handler") == "gravar_clientes":
            gravar_clientes(message)
            break
    limite = db.execute("select limite from clientes where id = 2").fetchone()[0]
    novo = db.execute("select id from clientes where nome = 'Novo Cliente Python'").fetchone()
    pendentes = aylis.data("cdsClientes", "info")["changes"]
    log("SELFTEST python " + delta_selftest())
    log(f"SELFTEST limite={limite} novo_id={novo[0] if novo else None} pendentes={pendentes}")
    aylis.data("cdsClientes", "last")
    aylis.send("close")
    aylis.loop()
else:
    aylis.run("FrmClientes")
