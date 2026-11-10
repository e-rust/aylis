// Player: o ponto de entrada. Gerado pela IDE do aylis (pode editar).
package main

import (
	"app/src/forms"
	"aylis.dev/cliente/aylis"
)

func main() {
	app := aylis.New()
	forms.Register(app)
	app.Run("") // abre o form principal do projeto
}
