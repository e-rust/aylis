package aylis

import (
	"bufio"
	"encoding/json"
	"os"
	"sync"
)

// Session é uma tela da ponte com --sessions (aylis-host --bridge ... --sessions): um processo
// atende todas as telas, e cada uma roda numa goroutine própria (um handler demorado não trava
// as outras; um panic fica nela). Guarde o estado da tela em variáveis da função passada ao
// Serve (nunca em globais, que são de todas). A API é a do Aylis (Call, Send, On, Run...).
type Session struct {
	*Aylis
	// Number é o número da tela na ponte.
	Number int
	// User é o usuário logado (o mesmo nas telas dele); nil antes do Login.
	User   *User
	Server *Server
}

// Post roda fn na goroutine desta tela (pode chamar de qualquer goroutine).
func (s *Session) Post(fn func()) {
	defer func() { recover() }() // a tela já fechou
	s.inbox <- item{fn: fn}
}

// Login liga esta tela ao usuário id: as telas do mesmo usuário (duas janelas, duas abas)
// dividem o mesmo User.
func (s *Session) Login(id string) {
	s.Logout()
	s.User = s.Server.user(id)
	s.User.mu.Lock()
	s.User.sessions = append(s.User.sessions, s)
	s.User.mu.Unlock()
}

// Logout solta o usuário desta tela (sozinho, ao fechar).
func (s *Session) Logout() {
	if s.User != nil {
		s.Server.leave(s.User, s)
		s.User = nil
	}
}

// User é o usuário logado, dividido pelas telas dele. Data guarda o que vale para todas (o
// carrinho, as preferências); use Lock/Unlock quando duas telas podem mexer ao mesmo tempo.
type User struct {
	ID       string
	Data     map[string]any
	mu       sync.Mutex
	sessions []*Session
}

// Lock e Unlock protegem Data.
func (u *User) Lock()   { u.mu.Lock() }
func (u *User) Unlock() { u.mu.Unlock() }

// Broadcast roda fn(tela) na goroutine de cada tela do usuário (atualizar todas).
func (u *User) Broadcast(fn func(*Session)) {
	u.mu.Lock()
	sessions := append([]*Session(nil), u.sessions...)
	u.mu.Unlock()
	for _, s := range sessions {
		s := s
		s.Post(func() { fn(s) })
	}
}

// Server é o processo das sessões: lê o stdin e entrega cada mensagem à tela dela.
type Server struct {
	out      *bufio.Writer
	mu       sync.Mutex
	sessions map[int]*Session
	users    map[string]*User
}

func (sv *Server) write(message M) {
	data, err := json.Marshal(message)
	if err != nil {
		Log("mensagem inválida:", err)
		return
	}
	sv.mu.Lock()
	defer sv.mu.Unlock()
	sv.out.Write(data)
	sv.out.WriteByte('\n')
	sv.out.Flush()
}

func (sv *Server) user(id string) *User {
	sv.mu.Lock()
	defer sv.mu.Unlock()
	u := sv.users[id]
	if u == nil {
		u = &User{ID: id, Data: M{}}
		sv.users[id] = u
	}
	return u
}

func (sv *Server) leave(u *User, s *Session) {
	u.mu.Lock()
	for i, x := range u.sessions {
		if x == s {
			u.sessions = append(u.sessions[:i], u.sessions[i+1:]...)
			break
		}
	}
	empty := len(u.sessions) == 0
	u.mu.Unlock()
	if empty {
		sv.mu.Lock()
		delete(sv.users, u.ID)
		sv.mu.Unlock()
	}
}

// Serve atende as telas da ponte com --sessions: started roda numa goroutine para cada tela
// que conecta (monte o estado e chame s.Run); ended (pode ser nil) quando ela sai.
func Serve(started func(*Session), ended func(*Session)) {
	sv := &Server{out: bufio.NewWriter(os.Stdout), sessions: map[int]*Session{}, users: map[string]*User{}}
	in := bufio.NewScanner(os.Stdin)
	in.Buffer(make([]byte, 1<<20), 64<<20)
	for in.Scan() {
		var message map[string]json.RawMessage
		if json.Unmarshal(in.Bytes(), &message) != nil {
			continue
		}
		var number int
		if json.Unmarshal(message["session"], &number) != nil {
			continue
		}
		delete(message, "session")
		var event string
		json.Unmarshal(message["event"], &event)
		if event == "session_open" {
			s := &Session{Aylis: &Aylis{handlers: map[string]func(Event){}, inbox: make(chan item, 256)}, Number: number, Server: sv}
			s.Aylis.session = s
			sv.mu.Lock()
			sv.sessions[number] = s
			sv.mu.Unlock()
			go func() {
				defer func() {
					if r := recover(); r != nil {
						Log("erro na sessão", number, r)
					}
					if ended != nil {
						func() {
							defer func() { recover() }()
							ended(s)
						}()
					}
					s.Logout()
					sv.mu.Lock()
					delete(sv.sessions, number)
					sv.mu.Unlock()
				}()
				started(s)
			}()
			continue
		}
		sv.mu.Lock()
		s := sv.sessions[number]
		sv.mu.Unlock()
		if s == nil {
			continue
		}
		if event == "session_close" {
			s.inbox <- item{msg: map[string]json.RawMessage{"event": json.RawMessage(`"closed"`)}}
		} else {
			s.inbox <- item{msg: message}
		}
	}
}
