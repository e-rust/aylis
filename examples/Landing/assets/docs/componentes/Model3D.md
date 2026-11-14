# Model3D

![Model3D](img/Model3D.png)

Modelo 3D (glTF 2.0 / GLB) com esqueleto, morph targets e animação.

**Paleta:** 3D · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Animação

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `animation` | Animação | texto | O nome do clipe de animação tocando; vazio: o primeiro do arquivo. |
| `playing` | Tocando | lógico | A animação anda; desligado, o modelo fica na pose atual. |
| `looping` | Repetindo | lógico | O clipe recomeça ao chegar ao fim; desligado, toca uma vez e fica na última pose. |
| `speed` | Velocidade | número | Velocidade da animação (1 = normal, 2 = o dobro). |
| `crossfade` | Transição suave | duração (ms) | Duração da transição ao trocar de clipe, em ms; 0: troca seca. |
| `auto_rotate` | Gira sozinho | número | Giro contínuo, em graus por segundo. |

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file_lights` | Luzes do arquivo | lógico | Usar as luzes do arquivo, quando houver. |
| `shadows` | Sombras | lógico | Sombra da primeira luz direcional. |
| `ray_traced` | Traçado de raios | lógico | Traçado de raios (onde a placa traça). |
| `background` | Fundo | cor | A cor do fundo; sem cor, transparente (o que está atrás aparece). |
| `resolution` | Resolução do 3D | número | Em quantos pixels por ponto o 3D é desenhado (a imagem é ampliada para o controle). 0 = automático: a densidade da tela, mas no celular no máximo `AUTO_RESOLUTION` (as telas de 2,5 a 3,5 px/ponto deixam a GPU do celular abaixo de 60 quadros/s, e num 3D de luz suave a diferença quase não se vê). Mais alto: mais nítido e mais pesado. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `orbit` | Girar com o mouse | lógico | Arrastar e a roda do mouse mexem na câmera. |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `source` | Origem | texto | O arquivo do modelo (`.glb`, glTF 2.0) no projeto. |

### Posição e tamanho

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `yaw` | Giro horizontal | ângulo (graus) | Giro da câmera em volta do modelo, na horizontal, em graus. |
| `pitch` | Inclinação | ângulo (graus) | Inclinação da câmera, olhando de cima ou de baixo, em graus. |
| `zoom` | Zoom | número | Aproximação da câmera (1 = o modelo inteiro na vista; maior aproxima), de 0,2 a 8. |
| `model_rotation` | Rotação do modelo | ângulo (graus) | Giro do modelo em graus. |
| `camera` | Câmera | texto | Câmera do arquivo pelo nome (vazio = a órbita). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_clip_end` | — | — |  |

## Em código

No form, o `Model3D1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Model3D", "name": "Model3D1", "on_clip_end": "model3_d1_clip_end" }
```

E o handler, um método do form (`self.Model3D1` já é um `Model3D`):

```rust
impl FrmPrincipal {
    pub fn model3_d1_clip_end(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Model3D1.source(ui);
        self.Model3D1.set_source(ui, format!("{atual}!"));
    }
}
```
