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
	"math"
	"math/big"
	"os"
	"strconv"
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
	// No on_click do mouse, do toque ou da caneta: onde, com que botão e com que teclas
	// (if ev.Pointer != nil && ev.Pointer.Ctrl() { ... }); nil no clique do teclado ou do código.
	Pointer *Pointer `json:"pointer"`
}

// Pointer é o ponteiro de um on_click: X/Y no componente, FormX/FormY no form, Button
// ("primary", "secondary", "middle"...), as teclas apertadas ("ctrl", "shift", "alt", "meta")
// e Kind ("mouse", "touch", "pen").
type Pointer struct {
	X         float64  `json:"x"`
	Y         float64  `json:"y"`
	FormX     float64  `json:"form_x"`
	FormY     float64  `json:"form_y"`
	Button    string   `json:"button"`
	Modifiers []string `json:"modifiers"`
	Kind      string   `json:"kind"`
	Clicks    int      `json:"clicks"`
}

// Has diz se a tecla modificadora ("ctrl", "shift", "alt", "meta") estava apertada.
func (p *Pointer) Has(modifier string) bool {
	for _, m := range p.Modifiers {
		if m == modifier {
			return true
		}
	}
	return false
}

func (p *Pointer) Ctrl() bool  { return p.Has("ctrl") }
func (p *Pointer) Shift() bool { return p.Has("shift") }
func (p *Pointer) Alt() bool   { return p.Has("alt") }
func (p *Pointer) Meta() bool  { return p.Has("meta") }

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

// GetRef lê a referência a outro componente (PopupMenu, DataSource); nil se não há.
func (c Component) GetRef(prop string) *Component {
	v, err := c.App.Call("get", M{"target": c.ID, "prop": prop, "ref": true})
	if err != nil {
		Log("ler", c.Name+"."+prop+":", err)
	}
	return c.ComponentOf(v)
}

// SetRef grava a referência a outro componente (nil: nenhum).
func (c Component) SetRef(prop string, ref Ref) {
	if ref == nil {
		c.Set(prop, nil)
		return
	}
	c.Set(prop, ref.ComponentID())
}

// ComponentOf é o componente que a tela devolveu com "ref": true (o id e o nome); nil se não há.
func (c Component) ComponentOf(info any) *Component {
	m, ok := info.(map[string]any)
	if !ok {
		return nil
	}
	name, _ := m["name"].(string)
	return &Component{App: c.App, ID: int64(number(m["id"])), Name: name}
}

func number(v any) float64 {
	switch n := v.(type) {
	case float64:
		return n
	case int:
		return float64(n)
	case int64:
		return float64(n)
	case json.Number:
		f, _ := n.Float64()
		return f
	}
	return 0
}

// Color é uma cor do acesso tipado: RGBA (0 a 255) ou um nome do tema (ColorToken("primary")),
// que acompanha a troca de tema.
type Color struct {
	R, G, B, A uint8
	Token      string
}

// RGB é a cor opaca.
func RGB(r, g, b uint8) Color { return Color{R: r, G: g, B: b, A: 255} }

// RGBA é a cor com a opacidade.
func RGBA(r, g, b, a uint8) Color { return Color{R: r, G: g, B: b, A: a} }

// ColorToken é um nome de cor do tema ou do projeto.
func ColorToken(name string) Color { return Color{A: 255, Token: strings.TrimSpace(name)} }

// ColorHex lê "#RRGGBB", "#RRGGBBAA" ou "#RGB".
func ColorHex(text string) (Color, error) {
	h := strings.TrimPrefix(strings.TrimSpace(text), "#")
	if len(h) == 3 {
		h = string([]byte{h[0], h[0], h[1], h[1], h[2], h[2]})
	}
	var c Color
	c.A = 255
	switch len(h) {
	case 6:
		if _, err := fmt.Sscanf(h, "%02x%02x%02x", &c.R, &c.G, &c.B); err != nil {
			return Color{}, fmt.Errorf("cor inválida: %s (use #RRGGBB ou #RRGGBBAA)", text)
		}
	case 8:
		if _, err := fmt.Sscanf(h, "%02x%02x%02x%02x", &c.R, &c.G, &c.B, &c.A); err != nil {
			return Color{}, fmt.Errorf("cor inválida: %s (use #RRGGBB ou #RRGGBBAA)", text)
		}
	default:
		return Color{}, fmt.Errorf("cor inválida: %s (use #RRGGBB ou #RRGGBBAA)", text)
	}
	return c, nil
}

// ColorFromJSON é a cor que a tela devolveu ("#RRGGBB[AA]" ou um nome).
func ColorFromJSON(v any) Color {
	s, _ := v.(string)
	if s == "" {
		return Color{}
	}
	if !strings.HasPrefix(s, "#") {
		return ColorToken(s)
	}
	c, _ := ColorHex(s)
	return c
}

// JSON é a cor como a tela entende.
func (c Color) JSON() string {
	if c.Token != "" {
		return c.Token
	}
	if c.A == 255 {
		return fmt.Sprintf("#%02X%02X%02X", c.R, c.G, c.B)
	}
	return fmt.Sprintf("#%02X%02X%02X%02X", c.R, c.G, c.B, c.A)
}

// Bounds são margens (margins, padding): esquerda, cima, direita e baixo.
type Bounds struct {
	Left, Top, Right, Bottom float64
}

// BoundsAll é a mesma medida nos quatro lados.
func BoundsAll(v float64) Bounds { return Bounds{v, v, v, v} }

// BoundsSymmetric é a medida na horizontal e na vertical.
func BoundsSymmetric(horizontal, vertical float64) Bounds {
	return Bounds{horizontal, vertical, horizontal, vertical}
}

// BoundsFromJSON lê o valor da tela: um número, [h, v], [l, t, r, b] ou um objeto.
func BoundsFromJSON(v any) Bounds {
	switch x := v.(type) {
	case []any:
		if len(x) == 2 {
			return BoundsSymmetric(number(x[0]), number(x[1]))
		}
		if len(x) == 4 {
			return Bounds{number(x[0]), number(x[1]), number(x[2]), number(x[3])}
		}
	case map[string]any:
		return Bounds{number(x["left"]), number(x["top"]), number(x["right"]), number(x["bottom"])}
	case nil:
		return Bounds{}
	default:
		return BoundsAll(number(x))
	}
	return Bounds{}
}

// JSON são as margens como a tela entende.
func (b Bounds) JSON() []float64 { return []float64{b.Left, b.Top, b.Right, b.Bottom} }

func num(v float64) string { return strconv.FormatFloat(math.Round(v*1000)/1000, 'g', -1, 64) }

// Shadow é uma sombra: o deslocamento (Dx, Dy), o desfoque, o espalhamento e a cor. nil no acesso
// tipado é sem sombra.
type Shadow struct {
	Dx, Dy, Blur, Spread float64
	Color                Color
}

// ShadowFromJSON lê "dx dy desfoque [espalhamento] [#cor]" (vazio ou inválido: nil; sem cor,
// preto a 25%).
func ShadowFromJSON(v any) *Shadow {
	s, _ := v.(string)
	var numbers []float64
	color := RGBA(0, 0, 0, 64)
	hasColor := false
	for _, token := range strings.Fields(strings.ReplaceAll(s, ",", " ")) {
		if f, err := strconv.ParseFloat(strings.TrimSuffix(token, "px"), 64); err == nil {
			numbers = append(numbers, f)
		} else if c, err := ColorHex(token); err == nil && !hasColor {
			color, hasColor = c, true
		} else {
			return nil
		}
	}
	if len(numbers) < 2 || len(numbers) > 4 {
		return nil
	}
	sh := &Shadow{Dx: numbers[0], Dy: numbers[1], Color: color}
	if len(numbers) > 2 {
		sh.Blur = numbers[2]
	}
	if len(numbers) > 3 {
		sh.Spread = numbers[3]
	}
	return sh
}

// JSON é a sombra como a tela entende ("" sem sombra).
func (s *Shadow) JSON() string {
	if s == nil {
		return ""
	}
	text := num(s.Dx) + " " + num(s.Dy) + " " + num(s.Blur)
	if s.Spread != 0 {
		text += " " + num(s.Spread)
	}
	return text + " " + s.Color.JSON()
}

// Easing é uma curva de suavização: uma das prontas (EasingOutQuad) ou a Bézier cúbica
// (EasingCubicBezier(0.2, 0, 0, 1)), como o cubic-bezier() do CSS.
type Easing struct {
	Name   string
	Bezier *[4]float64
}

// As curvas prontas.
var (
	EasingLinear     = Easing{Name: "linear"}
	EasingInQuad     = Easing{Name: "in_quad"}
	EasingOutQuad    = Easing{Name: "out_quad"}
	EasingInOutQuad  = Easing{Name: "in_out_quad"}
	EasingOutCubic   = Easing{Name: "out_cubic"}
	EasingInOutCubic = Easing{Name: "in_out_cubic"}
	EasingOutBack    = Easing{Name: "out_back"}
)

// EasingCubicBezier é a Bézier cúbica com os dois pontos de controle.
func EasingCubicBezier(x1, y1, x2, y2 float64) Easing { return Easing{Bezier: &[4]float64{x1, y1, x2, y2}} }

// EasingFromJSON lê o nome, um nome do CSS ou "cubic-bezier(x1, y1, x2, y2)".
func EasingFromJSON(v any) Easing {
	s, _ := v.(string)
	text := strings.TrimSpace(s)
	if strings.HasPrefix(text, "cubic-bezier(") && strings.HasSuffix(text, ")") {
		parts := strings.Split(text[13:len(text)-1], ",")
		if len(parts) == 4 {
			var b [4]float64
			for i, p := range parts {
				f, err := strconv.ParseFloat(strings.TrimSpace(p), 64)
				if err != nil {
					return EasingLinear
				}
				b[i] = f
			}
			return Easing{Bezier: &b}
		}
		return EasingLinear
	}
	switch strings.ToLower(text) {
	case "ease":
		return EasingCubicBezier(0.25, 0.1, 0.25, 1)
	case "ease-in":
		return EasingCubicBezier(0.42, 0, 1, 1)
	case "ease-out":
		return EasingCubicBezier(0, 0, 0.58, 1)
	case "ease-in-out":
		return EasingCubicBezier(0.42, 0, 0.58, 1)
	}
	plain := strings.ToLower(strings.ReplaceAll(text, "-", "_"))
	for _, n := range []string{"linear", "in_quad", "out_quad", "in_out_quad", "out_cubic", "in_out_cubic", "out_back"} {
		if n == plain {
			return Easing{Name: n}
		}
	}
	return EasingLinear
}

// JSON é a curva como a tela entende.
func (e Easing) JSON() string {
	if e.Bezier == nil {
		return e.Name
	}
	b := e.Bezier
	return "cubic-bezier(" + num(b[0]) + ", " + num(b[1]) + ", " + num(b[2]) + ", " + num(b[3]) + ")"
}

// Point é um ponto (X, Y).
type Point struct{ X, Y float64 }

// PointsFromJSON lê "0.5,0 1,1 0,1" (os que não são pontos ficam de fora).
func PointsFromJSON(v any) []Point {
	s, _ := v.(string)
	var out []Point
	for _, pair := range strings.Fields(s) {
		xy := strings.Split(pair, ",")
		if len(xy) != 2 {
			continue
		}
		x, e1 := strconv.ParseFloat(xy[0], 64)
		y, e2 := strconv.ParseFloat(xy[1], 64)
		if e1 == nil && e2 == nil {
			out = append(out, Point{x, y})
		}
	}
	return out
}

// PointsJSON são os pontos como a tela entende.
func PointsJSON(points []Point) string {
	out := make([]string, len(points))
	for i, p := range points {
		out[i] = num(p.X) + "," + num(p.Y)
	}
	return strings.Join(out, " ")
}

// Form é a base dos forms: cada form aberto na tela tem a sua
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

// CreateForm cria o form na tela e devolve a instância dele; another: mais uma janela do mesmo form.
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

// Post roda fn na goroutine do Loop, de qualquer goroutine: no
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

// MessageDlg é a caixa de mensagem que espera: devolve o botão
// ("yes", "no", "ok"...). Enquanto espera, os eventos continuam rodando.
func (i *Aylis) MessageDlg(text, kind string, buttons ...string) string {
	return i.MessageDlgIcon(text, kind, "", "", buttons...)
}

// MessageDlgIcon é a MessageDlg com outro ícone: icon (nome de ícone) e iconColor (token do
// tema, como "danger", ou "#rrggbb"); vazios, fica o do tipo.
func (i *Aylis) MessageDlgIcon(text, kind, icon, iconColor string, buttons ...string) string {
	if len(buttons) == 0 {
		buttons = []string{"ok"}
	}
	return i.waitFor("message_dlg", dialogIcon(M{"text": text, "kind": kind, "buttons": buttons}, icon, iconColor))
}

// Prompt é a pergunta com um campo de texto (OK e Cancelar), que espera: devolve o texto
// digitado e true, ou "" e false no Cancelar. required não deixa confirmar vazio. Enquanto
// espera, os eventos continuam rodando.
func (i *Aylis) Prompt(title, label, value string, required bool) (string, bool) {
	return i.PromptIcon(title, label, value, required, "", "")
}

// PromptIcon é o Prompt com outro ícone (icon e iconColor, como na MessageDlgIcon).
func (i *Aylis) PromptIcon(title, label, value string, required bool, icon, iconColor string) (string, bool) {
	fields := dialogIcon(M{"title": title, "label": label, "value": value}, icon, iconColor)
	if required {
		fields["required"] = true
	}
	result, ok := i.waitForValue("prompt", fields)
	if !ok || result == nil {
		return "", false
	}
	return fmt.Sprint(result), true
}

// ShowModal abre o form como modal e espera ele fechar: devolve o
// resultado ("ok", "cancel"...).
func (i *Aylis) ShowModal(form *Form) string {
	return i.ShowModalIn(form, nil)
}

// ShowModalIn é o ShowModal só no container (um componente de outro form, como a página de
// uma aba): o resto da tela continua respondendo. Com nil, a tela toda.
func (i *Aylis) ShowModalIn(form *Form, container Ref) string {
	fields := M{"target": form.ID}
	if container != nil {
		fields["container"] = container.ComponentID()
	}
	return i.waitFor("show_modal", fields)
}

// dialogIcon põe os campos opcionais icon e icon_color do message_dlg e do prompt.
func dialogIcon(fields M, icon, iconColor string) M {
	if icon != "" {
		fields["icon"] = icon
	}
	if iconColor != "" {
		fields["icon_color"] = iconColor
	}
	return fields
}

func (i *Aylis) waitFor(cmd string, fields M) string {
	result, ok := i.waitForValue(cmd, fields)
	if !ok {
		return ""
	}
	return fmt.Sprint(result)
}

// waitForValue espera a resposta adiada (false se o pedido falhou; a falha vai para o log).
func (i *Aylis) waitForValue(cmd string, fields M) (any, bool) {
	fields["wait"] = true
	i.waiting++
	defer func() { i.waiting-- }()
	// O que chegou antes (enquanto outro Call esperava) roda já.
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
		return nil, false
	}
	return result, true
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
// quando o handler termina: o ev.Reply ou, com panic, a recusa com a mensagem (a tela mostra).
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
			// A tela mostra o erro ao usuário.
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
