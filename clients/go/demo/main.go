// Backend de demonstração em Go: um campo, um botão e a resposta do backend.
//
//	cd clients/go && go build -o demo.exe ./demo
//	aylis-host -- clients/go/demo.exe
//	aylis-host --png tela.png -- clients/go/demo.exe --selftest
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"aylis.dev/cliente/aylis"
)

func main() {
	selftest := len(os.Args) > 1 && os.Args[1] == "--selftest"
	app := aylis.New()

	// O layout fica em clients/demo_layout.json (ao lado da pasta go).
	exe, _ := os.Executable()
	text, err := os.ReadFile(filepath.Join(filepath.Dir(exe), "..", "demo_layout.json"))
	if err != nil {
		text, _ = os.ReadFile(filepath.Join("clients", "demo_layout.json"))
	}
	var layout any
	json.Unmarshal(text, &layout)
	app.Send("load", aylis.M{"layout": layout})

	app.On("hello", func(aylis.Event) {
		name, _ := app.Call("get", aylis.M{"target": "edtName", "prop": "text"})
		if name == "" {
			name = "mundo"
		}
		app.Send("set", aylis.M{"target": "lblMsg", "props": aylis.M{"text": fmt.Sprintf("Olá, %v! (Go)", name)}})
		if selftest {
			msg, _ := app.Call("get", aylis.M{"target": "lblMsg", "prop": "text"})
			aylis.Log("SELFTEST go", msg)
			// Fechar pelo backend passa pelo on_close_query, que pergunta (e espera a resposta).
			app.Send("close", aylis.M{"target": "frmDemo"})
		}
	})

	// O on_close_query pergunta: a tela só fecha com a resposta (o ev.Reply).
	app.On("pode_fechar", func(ev aylis.Event) {
		aylis.Log("SELFTEST go pergunta")
		aylis.Log("SELFTEST go " + aylis.DeltaSelftest())
		ev.Reply["can_close"] = true
	})

	if _, err := app.Call("run", aylis.M{"form": "frmDemo"}); err != nil {
		aylis.Log(err)
		return
	}
	if selftest {
		app.Send("set", aylis.M{"target": "edtName", "props": aylis.M{"text": "Ana"}})
		app.Send("click", aylis.M{"target": "btnHello"})
	}
	app.Loop()
}
