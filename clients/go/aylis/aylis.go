// Package aylis é o cliente do protocolo do aylis para backends em Go, via aylis-host.
//
// O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por
// linha.
//
//	app := aylis.New()
//	app.Send("load", aylis.M{"layout": layout})
//	name, _ := app.Call("get", aylis.M{"target": "edtName", "prop": "text"})
//	app.On("btn_salvar_click", func(ev aylis.Event) { ... })
//	app.Run("frmClientes")
//
// Os handlers rodam na goroutine do Run/Loop. De outras goroutines, use Send (seguro).
package aylis

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"math/big"
	"os"
	"strings"
	"sync"
)

// M é um objeto JSON.
type M = map[string]any

// Event é um evento da tela.
type Event struct {
	Event   string `json:"event"`
	Handler string `json:"handler"`
	Form    string `json:"form"`
	Sender  struct {
		ID    uint64 `json:"id"`
		Name  string `json:"name"`
		Path  string `json:"path"`
		Class string `json:"class"`
	} `json:"sender"`
	Args any `json:"args"`
	// Dados extras de alguns eventos (a URL de quem mandou, no on_message do WebView).
	Info    map[string]string `json:"info"`
	Message string            `json:"message"`
	// Num evento que pergunta (on_close_query, before_post, on_key_down...), o número da
	// pergunta: a tela espera a resposta, que o cliente manda quando o handler termina.
	Ask *int64 `json:"ask"`
	// A resposta de um evento que pergunta, gravável pelo handler: ev.Reply["can_close"] = false
	// (fechar), ev.Reply["key"] = "" (engolir a tecla) ou outro caractere (trocar),
	// "data"/"label" (o arrasto), "accept" (soltar aqui). Recusar com mensagem (before_post):
	// panic("Informe o CPF").
	Reply M `json:"-"`
}

// Component é um componente de um form, pelo id dele na tela (resolvido quando a instância
// do form é montada). Os tipos de cada componente vêm do components.go gerado.
type Component struct {
	App  *Aylis
	ID   int64
	Name string
}

// Ref é qualquer componente (os tipos gerados embutem o Component): o argumento dos métodos
// que recebem um componente, como o ScrollTo de um ScrollBox.
type Ref interface{ ComponentID() int64 }

// ComponentID é o id do componente na tela (o que vai no pedido).
func (c Component) ComponentID() int64 { return c.ID }

// Get lê uma propriedade (nil se falhar; a falha vai para o log).
func (c Component) Get(prop string) any {
	v, err := c.App.Call("get", M{"target": c.ID, "prop": prop})
	if err != nil {
		Log("ler", c.Name+"."+prop+":", err)
	}
	return v
}

// Set grava uma propriedade.
func (c Component) Set(prop string, value any) {
	c.App.Send("set", M{"target": c.ID, "props": M{prop: value}})
}

// Focus põe o foco no componente.
func (c Component) Focus() { c.App.Send("focus", M{"target": c.ID}) }

// Form é a base dos forms (como o TForm do Delphi): cada form aberto na tela tem a sua
// instância (duas janelas do mesmo form, duas instâncias), com os componentes resolvidos ao
// abrir. O tipo gerado de cada form embute Form e monta os componentes em Bind.
type Form struct {
	Component
}

// FormInstance é o que o tipo gerado de cada form sabe fazer.
type FormInstance interface {
	Bind(list []any) error
}

func findComponent(list []any, name string) map[string]any {
	for _, c := range list {
		if m, ok := c.(map[string]any); ok && m["name"] == name {
			return m
		}
	}
	return nil
}

// Resolve é o componente do form; o que falta ou mudou de classe: o form não bate com o código.
func (f *Form) Resolve(list []any, name, class string) (Component, error) {
	item := findComponent(list, name)
	if item == nil {
		return Component{}, fmt.Errorf("o form %s não bate com o código: falta o componente %s (%s)", f.Name, name, class)
	}
	if item["class"] != class {
		return Component{}, fmt.Errorf("o form %s não bate com o código: %s é %v no form e %s no código", f.Name, name, item["class"], class)
	}
	id, _ := item["id"].(float64)
	return Component{App: f.App, ID: int64(id), Name: name}, nil
}

// ResolveOptional é um componente que só existe em algumas plataformas (false se não está aqui).
func (f *Form) ResolveOptional(list []any, name string) (Component, bool) {
	item := findComponent(list, name)
	if item == nil {
		return Component{}, false
	}
	id, _ := item["id"].(float64)
	return Component{App: f.App, ID: int64(id), Name: name}, true
}

// Show abre o form numa janela.
func (f *Form) Show() { f.App.Send("show", M{"target": f.ID}) }

// Close fecha o form.
func (f *Form) Close() { f.App.Send("close", M{"target": f.ID}) }

type formClass struct {
	make     func(app *Aylis, id int64, name string) FormInstance
	activate func(FormInstance)
}

var formClasses = map[string]formClass{}

// RegisterFormClass registra o tipo de um form (o components.go gerado faz): cada form com
// esse nome aberto na tela vira uma instância dele; activate a torna a atual (o frmX gerado
// aponta para a janela cujo evento está rodando).
func RegisterFormClass(name string, make func(app *Aylis, id int64, name string) FormInstance, activate func(FormInstance)) {
	formClasses[strings.ToLower(name)] = formClass{make, activate}
}

// FormInstanceOf é a instância do form da tela (criada e montada na primeira vez), já como a
// atual; nil se o tipo dele não foi registrado. Um form que não bate com o código é erro.
func (i *Aylis) FormInstanceOf(form string, id int64) (FormInstance, error) {
	entry, ok := formClasses[strings.ToLower(form)]
	if !ok {
		return nil, nil
	}
	instance := i.forms[id]
	if instance == nil {
		instance = entry.make(i, id, form)
		list, err := i.Call("components", M{"target": id})
		if err != nil {
			return nil, err
		}
		items, _ := list.([]any)
		if err := instance.Bind(items); err != nil {
			return nil, err
		}
		if i.forms == nil {
			i.forms = map[int64]FormInstance{}
		}
		i.forms[id] = instance
	}
	entry.activate(instance)
	return instance, nil
}

// CreateForm cria o form na tela e devolve a instância dele (como o Application.CreateForm do
// Delphi); another: mais uma janela do mesmo form.
func (i *Aylis) CreateForm(form string, another bool) (FormInstance, error) {
	created, err := i.Call("create_form", M{"name": form, "new": another})
	if err != nil {
		return nil, err
	}
	m, _ := created.(map[string]any)
	id, _ := m["id"].(float64)
	return i.FormInstanceOf(form, int64(id))
}

// Aylis é a conexão com a tela.
type Aylis struct {
	in       *bufio.Scanner
	out      *bufio.Writer
	mu       sync.Mutex
	nextID   int
	handlers map[string]func(Event)
	// As instâncias dos forms, pelo id do form na tela.
	forms  map[int64]FormInstance
	queued []map[string]json.RawMessage
	// Numa sessão (Serve): as mensagens chegam por aqui e saem pelo servidor.
	inbox   chan item
	session *Session
	posted  []func()
	// Já mandou o hello (a primeira mensagem).
	greeted bool
	// Esperando um diálogo (MessageDlg, ShowModal): os eventos rodam já (os do modal); e os
	// handlers rodando (um evento que pergunta roda mesmo durante um Call).
	waiting int
	running int
	// Os Post do desktop (de qualquer goroutine).
	postMu sync.Mutex
	// Os forms liberados na tela (o aviso __freed).
	freed map[int64]bool
}

// Uma mensagem da tela ou um Post para rodar no Loop.
type item struct {
	msg map[string]json.RawMessage
	fn  func()
}

// ErrClosed: a tela foi fechada.
var ErrClosed = errors.New("a tela foi fechada")

// New conecta pelo stdin/stdout.
func New() *Aylis {
	in := bufio.NewScanner(os.Stdin)
	in.Buffer(make([]byte, 1<<20), 64<<20)
	return &Aylis{in: in, out: bufio.NewWriter(os.Stdout), handlers: map[string]func(Event){}}
}

// Protocol é a versão do protocolo do aylis que este cliente fala (o hello).
const Protocol = 1

func (i *Aylis) write(message M) {
	// A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes).
	if !i.greeted {
		i.greeted = true
		i.write(M{"cmd": "hello", "protocol": Protocol, "client": "go", "features": []string{"done"}})
	}
	if i.session != nil {
		message["session"] = i.session.Number
		i.session.Server.write(message)
		return
	}
	data, err := json.Marshal(message)
	if err != nil {
		Log("mensagem inválida:", err)
		return
	}
	i.mu.Lock()
	defer i.mu.Unlock()
	i.out.Write(data)
	i.out.WriteByte('\n')
	i.out.Flush()
}

func (i *Aylis) read() (map[string]json.RawMessage, error) {
	if i.inbox != nil {
		it, ok := <-i.inbox
		if !ok {
			return nil, ErrClosed
		}
		if it.fn != nil {
			i.posted = append(i.posted, it.fn)
			return map[string]json.RawMessage{"event": json.RawMessage(`"__post"`)}, nil
		}
		return it.msg, nil
	}
	for i.in.Scan() {
		line := i.in.Bytes()
		if len(line) == 0 {
			continue
		}
		var message map[string]json.RawMessage
		if err := json.Unmarshal(line, &message); err != nil {
			Log("linha inválida:", err)
			continue
		}
		return message, nil
	}
	return nil, ErrClosed
}

// Send envia um comando sem esperar resposta (erros chegam como evento "error").
func (i *Aylis) Send(cmd string, fields M) {
	message := M{"cmd": cmd}
	for k, v := range fields {
		message[k] = v
	}
	i.write(message)
}

// Profile marca um trecho no profiler da IDE (a faixa "Marcas"): defer app.Profile("consulta")().
func (i *Aylis) Profile(name string) func() {
	i.Send("profile_begin", M{"name": name})
	return func() { i.Send("profile_end", nil) }
}

// Call envia um comando e espera o resultado (só na goroutine do Loop).
func (i *Aylis) Call(cmd string, fields M) (any, error) {
	i.nextID++
	id := i.nextID
	message := M{"cmd": cmd, "id": id}
	for k, v := range fields {
		message[k] = v
	}
	i.write(message)
	for {
		reply, err := i.read()
		if err != nil {
			return nil, err
		}
		var got int
		if raw, ok := reply["id"]; ok && json.Unmarshal(raw, &got) == nil && got == id {
			if raw, ok := reply["error"]; ok {
				var text string
				json.Unmarshal(raw, &text)
				return nil, errors.New(text)
			}
			var result any
			if raw, ok := reply["result"]; ok {
				json.Unmarshal(raw, &result)
			}
			return result, nil
		}
		// Esperando um diálogo, os eventos (os do modal) rodam já; um evento que pergunta também
		// (o post do próprio backend dispara o before_post).
		if raw, ok := reply["event"]; ok {
			var kind string
			json.Unmarshal(raw, &kind)
			_, ask := reply["ask"]
			if kind == "__post" && i.waiting > 0 {
				i.runPosted()
				continue
			}
			if kind == "ping" && i.waiting > 0 {
				i.Send("pong", nil) // a sincronia dos roteiros: o que veio antes já rodou
				continue
			}
			if kind != "closed" && kind != "ping" && kind != "__post" && (i.waiting > 0 || (ask && i.running < 8)) {
				i.handleEvent(reply)
				continue
			}
		}
		i.queued = append(i.queued, reply)
	}
}

// Post roda fn na goroutine do Loop, de qualquer goroutine (o TThread.Queue do Delphi): no
// desktop, a tela devolve um eco (wake) que acorda o Loop.
func (i *Aylis) Post(fn func()) {
	if i.session != nil {
		i.session.Post(fn)
		return
	}
	i.postMu.Lock()
	i.posted = append(i.posted, fn)
	i.postMu.Unlock()
	i.Send("wake", nil)
}

// PostTo é o Post ligado a um form: se ele foi liberado até lá (a janela fechou e saiu), fn não
// roda.
func (i *Aylis) PostTo(owner *Form, fn func()) {
	i.Post(func() {
		i.postMu.Lock()
		gone := i.freed[owner.ID]
		i.postMu.Unlock()
		if !gone {
			fn()
		}
	})
}

// FieldDecimal é o valor de um campo decimal (Currency, BCD) como *big.Rat, exato: o protocolo
// manda o texto, e ele nunca passa por float64. nil se o campo é nulo.
func (i *Aylis) FieldDecimal(dataSet, field string) (*big.Rat, error) {
	value, err := i.Call("data", M{"target": dataSet, "op": "value", "field": field})
	if err != nil || value == nil {
		return nil, err
	}
	r, ok := new(big.Rat).SetString(fmt.Sprint(value))
	if !ok {
		return nil, fmt.Errorf("%s.%s: não é um decimal: %v", dataSet, field, value)
	}
	return r, nil
}

// MessageDlg é a caixa de mensagem que espera, como o MessageDlg do Delphi: devolve o botão
// ("yes", "no", "ok"...). Enquanto espera, os eventos continuam rodando.
func (i *Aylis) MessageDlg(text, kind string, buttons ...string) string {
	if len(buttons) == 0 {
		buttons = []string{"ok"}
	}
	return i.waitFor("message_dlg", M{"text": text, "kind": kind, "buttons": buttons})
}

// ShowModal abre o form como modal e espera ele fechar, como o ShowModal do Delphi: devolve o
// resultado ("ok", "cancel"...).
func (i *Aylis) ShowModal(form *Form) string {
	return i.waitFor("show_modal", M{"target": form.ID})
}

func (i *Aylis) waitFor(cmd string, fields M) string {
	fields["wait"] = true
	i.waiting++
	defer func() { i.waiting-- }()
	// Como o laço do ShowModal do Delphi: o que chegou antes (enquanto outro Call esperava) roda já.
	queued := i.queued
	i.queued = nil
	for _, m := range queued {
		var kind string
		if raw, ok := m["event"]; ok {
			json.Unmarshal(raw, &kind)
		}
		switch {
		case kind == "__post":
			i.runPosted()
		case kind == "ping":
			i.Send("pong", nil)
		case kind != "" && kind != "closed" && kind != "error":
			i.handleEvent(m)
		default:
			i.queued = append(i.queued, m)
		}
	}
	result, err := i.Call(cmd, fields)
	if err != nil {
		Log(cmd+":", err)
		return ""
	}
	return fmt.Sprint(result)
}

// On registra o handler pelo nome usado no layout ("frmClientes.nome": só daquele form).
func (i *Aylis) On(name string, handler func(Event)) {
	i.handlers[name] = handler
}

// Run abre o form e trata os eventos até a tela fechar ("": o principal do projeto).
func (i *Aylis) Run(form string) error {
	var name any = form
	if form == "" {
		name = nil
	}
	if _, err := i.Call("run", M{"form": name}); err != nil {
		return err
	}
	// A instância do form principal antes do primeiro evento (o código pode usar frmX).
	if form != "" {
		if found, err := i.Call("find", M{"name": form}); err == nil {
			if id, ok := found.(float64); ok {
				if _, err := i.FormInstanceOf(form, int64(id)); err != nil {
					Log(err)
				}
			}
		}
	}
	i.Loop()
	return nil
}

// Loop trata os eventos até a tela fechar.
func (i *Aylis) Loop() {
	for {
		var message map[string]json.RawMessage
		if len(i.queued) > 0 {
			message, i.queued = i.queued[0], i.queued[1:]
		} else {
			m, err := i.read()
			if err != nil {
				return
			}
			message = m
		}
		raw, ok := message["event"]
		if !ok {
			continue
		}
		data, _ := json.Marshal(message)
		var ev Event
		json.Unmarshal(data, &ev)
		_ = raw
		switch ev.Event {
		case "closed":
			return
		case "__post":
			i.runPosted()
			continue
		case "ping":
			// Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes.
			i.Send("pong", nil)
			continue
		case "error":
			Log("erro na tela:", ev.Message)
			continue
		}
		i.handleEvent(message)
	}
}

// runPosted roda os Post pendentes.
func (i *Aylis) runPosted() {
	for {
		i.postMu.Lock()
		if len(i.posted) == 0 {
			i.postMu.Unlock()
			return
		}
		fn := i.posted[0]
		i.posted = i.posted[1:]
		i.postMu.Unlock()
		func() {
			defer func() {
				if r := recover(); r != nil {
					Log("erro no post", r)
				}
			}()
			fn()
		}()
	}
}

// handleEvent roda o handler do evento. Um evento que pergunta (com ask) recebe a resposta
// quando o handler termina: o ev.Reply ou, com panic, a recusa com a mensagem (a tela mostra),
// como no BeforePost do Delphi.
func (i *Aylis) handleEvent(message map[string]json.RawMessage) {
	// Forms liberados na tela: as instâncias saem e os PostTo deles não rodam.
	if raw, ok := message["event"]; ok && string(raw) == `"__freed"` {
		var ids []int64
		json.Unmarshal(message["forms"], &ids)
		i.postMu.Lock()
		if i.freed == nil {
			i.freed = map[int64]bool{}
		}
		for _, id := range ids {
			i.freed[id] = true
			delete(i.forms, id)
		}
		i.postMu.Unlock()
		return
	}
	data, _ := json.Marshal(message)
	var ev Event
	json.Unmarshal(data, &ev)
	ev.Reply = M{}
	handler := i.handlers[ev.Form+"."+ev.Handler]
	if handler == nil {
		handler = i.handlers[ev.Handler]
	}
	i.running++
	defer func() {
		i.running--
		// O handler terminou: a tela deixa de mostrar o backend ocupado.
		if raw, ok := message["seq"]; ok {
			var seq int64
			if json.Unmarshal(raw, &seq) == nil {
				defer i.Send("done", M{"seq": seq})
			}
		}
		if r := recover(); r != nil {
			if ev.Ask != nil {
				i.Send("answer", M{"ask": *ev.Ask, "error": fmt.Sprint(r)})
				return
			}
			Log("erro no handler", ev.Handler, r)
			// A tela mostra o erro ao usuário (como o Application.HandleException do Delphi).
			i.Send("exception", M{"message": fmt.Sprint(r), "path": ev.Form, "handler": ev.Handler})
		}
	}()
	if handler == nil {
		Log("evento sem handler:", ev.Handler)
	} else {
		// A janela do evento vira a atual (frmX aponta para ela durante o handler).
		if raw, ok := message["form_id"]; ok {
			var id float64
			if json.Unmarshal(raw, &id) == nil {
				if _, err := i.FormInstanceOf(ev.Form, int64(id)); err != nil {
					panic(err)
				}
			}
		}
		handler(ev)
	}
	if ev.Ask != nil {
		i.Send("answer", M{"ask": *ev.Ask, "values": ev.Reply})
	}
}

// Log escreve no console do aylis-host (o stdout é do protocolo).
func Log(args ...any) {
	fmt.Fprintln(os.Stderr, append([]any{"[backend]"}, args...)...)
}
