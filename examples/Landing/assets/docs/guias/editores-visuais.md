# Editores visuais de propriedades

O DevTools dos navegadores tem ferramentas para ajustar valores com o mouse em vez de digitar:
- o relógio de ângulo;
- o editor de sombra;
- o editor da curva de suavização (`cubic-bezier`);
- a linha do tempo das animações;
- o seletor de cores.

O Aylis tem os mesmos editores, e o Object Inspector da IDE escolhe um deles pelo **tipo da
propriedade**. O editor vem do tipo da propriedade, sem registro à parte.

| Tipo (`PropKind`) | Valor | No Object Inspector |
|---|---|---|
| `Angle` | número, em graus | relógio pequeno na própria linha (arrastar grava ao soltar) e `…` abre o relógio grande |
| `Shadow` | texto `"dx dy desfoque [espalhamento] #cor"` | amostra com a sombra e `…` abre o editor de sombra |
| `Easing` | nome (`out_quad`, `ease-in`) ou `cubic-bezier(x1, y1, x2, y2)` | miniatura da curva; ela e o `…` abrem o editor da curva |
| `Duration` | número, em milissegundos | `…` abre a linha do tempo |
| `Color` | `#RRGGBB[AA]` ou token do tema | amostra e `…` abre o seletor (quadro de cores + tokens do tema + cores fixas) |

Em todos, o campo de texto continua lá para quem prefere digitar.

## Os editores

**Relógio de ângulo.** 0° aponta para cima e o ângulo cresce no sentido horário, como no
`rotate()` e nos gradientes do CSS. No relógio, dá para:
- arrastar o ponteiro (com Shift, ele anda de 15 em 15°);
- usar as setas ou a roda do mouse (1° por vez);
- clicar num atalho (0°, 45°... 315°).

**Sombra.** O editor tem quatro partes:
- o quadro, onde se arrasta o deslocamento (com Shift, trava num eixo);
- as faixas de desfoque e de espalhamento;
- a cor, que abre o seletor de cores;
- as sombras prontas: Nenhuma, Suave, Média, Elevada, Contorno e Brilho.

A amostra mostra o resultado na hora.

**Curva de suavização.** Os dois pontos de controle são arrastáveis. O `y` pode passar de
0 a 1, para curvas que passam do ponto e voltam. Embaixo, uma bolinha anda com a curva.

À direita ficam as curvas prontas, cada uma com a miniatura:
- as do Aylis;
- as do CSS (`ease`, `ease-in`, `ease-out`, `ease-in-out`);
- as do Material 3 (padrão e enfáticas);
- a do Fluent;
- uma de mola.

O valor gravado é o nome, quando a curva escolhida tem nome, ou `cubic-bezier(...)`.

**Linha do tempo.** O tracejado é o atraso, a barra é a duração, a parte mais clara é a volta
(ida e volta) e as repetições aparecem apagadas. Arrastar muda os tempos:
- o começo da barra muda o atraso (o fim fica onde está);
- o fim da barra muda a duração;
- o meio da barra move tudo.

Os tempos andam de 10 em 10 ms; com Shift, de 1 em 1 ms.

Embaixo, a curva mostra o valor ao longo do tempo, com uma agulha percorrendo a animação.
Numa `FloatAnimation` ou `ColorAnimation`, o mesmo diálogo edita juntos:
- duração e atraso;
- curva;
- ida e volta e repetir.

Num `Timer`, edita só o intervalo.

**Cor.** À esquerda fica o quadro de cores, com matiz, saturação, brilho e transparência. À
direita ficam o valor, os tokens do tema (que acompanham o tema claro e o escuro) e as cores
fixas. Escolher um token grava o nome dele; mexer no quadro grava um `#hex`.

## Propriedades novas

Para os editores terem o que editar:

| Onde | Propriedade | Tipo |
|---|---|---|
| Todo controle | `rotation` (gira em torno do centro) | `Angle` |
| Todo controle | `shadow` (vale no lugar da sombra do estilo) | `Shadow` |
| `Rectangle` | `fill2` (com ela, o fundo vira gradiente de `fill` a `fill2`) e `gradient_angle` | `Color`, `Angle` |
| `FloatAnimation`, `ColorAnimation` | `delay` (espera antes de começar; só na ida) | `Duration` |
| `FloatAnimation`, `ColorAnimation` | `duration` | `Duration` |
| `FloatAnimation`, `ColorAnimation` | `easing` (agora aceita `cubic-bezier`) | `Easing` |
| `Timer`, `Carousel` | `interval` | `Duration` |
| `Snackbar` | `timeout` | `Duration` |

O clique também respeita o giro: cai no controle onde ele aparece desenhado, e o ponto chega a ele nas coordenadas dele (`Ui::local_point`).

`Easing` ganhou `CubicBezier(x1, y1, x2, y2)`, `Easing::parse`, `to_spec` e `to_bezier`.
`AnimationOptions` ganhou `delay_ms`. `Shadow` ganhou `parse` e `to_spec`.

## Nos seus componentes

O `properties!` usa o tipo do campo. Para um `f32` abrir o relógio ou a linha do tempo, diga
o tipo com `as`:

```rust
properties! {
    giro: f32 as Angle => "Appearance",
    tempo: u32 as Duration => "Behavior",
    sombra: Option<Shadow> => "Appearance",   // já é Shadow pelo tipo
    curva: Easing => "Animation",             // já é Easing pelo tipo
}
```

Os editores também são componentes comuns, na paleta (Additional), para usar em qualquer tela:
- `AngleDial`;
- `ShadowEditor`;
- `EasingEditor` (com `compact` para miniaturas);
- `TimingEditor`.

Todos disparam `on_change` enquanto o valor muda e `on_commit` ao soltar o mouse.
