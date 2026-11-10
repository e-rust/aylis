// Player: o ponto de entrada. Gerado pela IDE do aylis (pode editar).

#include "aylis.hpp"
#include "forms/forms.hpp"

int main() {
    aylis::Aylis app;
    register_forms(app);
    app.run();  // abre o form principal do projeto
    return 0;
}
