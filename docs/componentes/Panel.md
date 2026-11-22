### A ordem do `align`

Os encaixados `top` e `bottom` ocupam a largura toda antes dos `left` e `right`,
qualquer que seja a ordem em que foram declarados: um `bottom` declarado depois de um `left`
ainda pega a largura inteira. Para uma barra lateral que vai até embaixo, ponha o rodapé dentro
do painel do conteúdo (o `client`), ou use `most_left`/`most_bottom`, que encaixam antes.

### Reagir ao mouse

Uma linha de lista ou um "botão" feito de painel clareia com o mouse em cima com `interactive`
(as cores de estado do tema) ou com `hover_fill` e `pressed_fill` (cores próprias). O mouse
num filho (o ícone, o texto) conta como em cima do painel. Foco pelo teclado e Enter continuam
sendo do `Button`.
