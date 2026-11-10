// Backend de teste das sessões em Go (aylis-host --bridge ... --sessions), usado pelo teste
// `sessions_in_go` do aylis-host: um processo para todas as telas, um contador por tela; "boom"
// entra em pânico só naquela sessão, "lento" demora sem travar as outras e "entrar"/"comum"
// ligam as telas do mesmo usuário.
package main

import (
	"fmt"
	"os"
	"time"

	"aylis.dev/cliente/aylis"
)

var layout = aylis.M{"class": "Form", "name": "frm", "width": 200, "height": 80, "children": []aylis.M{
	{"class": "Label", "name": "lbl", "text": "-"},
	{"class": "Button", "name": "btn", "text": "+", "on_click": "somar"},
	{"class": "Button", "name": "boom", "text": "!", "on_click": "boom"},
	{"class": "Button", "name": "lento", "text": "z", "on_click": "lento"},
	{"class": "Label", "name": "lblu", "text": "-"},
	{"class": "Button", "name": "entrar", "text": "e", "on_click": "entrar"},
	{"class": "Button", "name": "comum", "text": "c", "on_click": "comum"},
}}

func main() {
	aylis.Serve(func(s *aylis.Session) {
		total := 0
		aylis.Log(fmt.Sprintf("SELFTEST sessão %d abriu (pid %d)", s.Number, os.Getpid()))
		s.On("somar", func(aylis.Event) {
			total++
			s.Send("set", aylis.M{"target": "frm.lbl", "props": aylis.M{"text": fmt.Sprintf("%d:%d", s.Number, total)}})
		})
		s.On("boom", func(aylis.Event) { panic("erro de propósito") })
		s.On("lento", func(aylis.Event) { time.Sleep(3 * time.Second) })
		s.On("entrar", func(aylis.Event) {
			s.Login("ana")
			s.Send("set", aylis.M{"target": "frm.lblu", "props": aylis.M{"text": "ana"}})
		})
		s.On("comum", func(aylis.Event) {
			u := s.User
			u.Lock()
			n, _ := u.Data["total"].(int)
			n++
			u.Data["total"] = n
			u.Unlock()
			u.Broadcast(func(t *aylis.Session) {
				t.Send("set", aylis.M{"target": "frm.lblu", "props": aylis.M{"text": fmt.Sprintf("u:%d", n)}})
			})
		})
		s.Send("load", aylis.M{"layout": layout})
		s.Run("frm")
	}, func(s *aylis.Session) {
		aylis.Log(fmt.Sprintf("SELFTEST sessão %d saiu", s.Number))
	})
}
