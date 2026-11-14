# CameraView

![CameraView](img/CameraView.png)

A câmera ao vivo. `active` liga; `camera` escolhe (vazio: a primeira; `front` ou `back`: a da tela ou a de trás, no celular; ou o `id` de `cameras()`, que lista); `mirror` espelha (a câmera frontal costuma parecer mais natural assim).

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `mirror` | Espelhado | lógico | Espelha a imagem na horizontal (a frontal costuma parecer mais natural assim). |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` | Como a imagem ocupa o controle: `contain` (inteira), `cover` (cobre, cortando as bordas), `stretch`, `center` ou `original`. |
| `radius` | Raio dos cantos | número | O raio dos cantos arredondados, em pontos (0: retos). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `camera` | Câmera | texto | Qual câmera: vazio, a primeira; `front` ou `back`, a da tela ou a de trás (celular); ou o `id` de uma da lista de `cameras()`. |
| `active` | Ativo | lógico | Liga a câmera e mostra a imagem ao vivo; desligado, a câmera fecha. |
| `quality` | Qualidade | um de: `low`, `medium`, `high`, `max` | O tamanho pedido: `low` (até 640x480), `medium` (até 1280x960), `high` (até 1920x1440) ou `max` (o maior do aparelho). |
| `resolution` | Resolução do 3D | texto | Um tamanho exato (`1920x1080`), se a câmera tiver; vazio: vale a `quality`. |
| `torch` | Lanterna (flash aceso) | lógico | Acende o flash como lanterna (onde houver: veja `has_flash`). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_error` | — | — |  |
| `on_ready` | — | — |  |

## Em código

No form, o `CameraView1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "CameraView", "name": "CameraView1", "on_error": "camera_view1_error" }
```

E o handler, um método do form (`self.CameraView1` já é um `CameraView`):

```rust
impl FrmPrincipal {
    pub fn camera_view1_error(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.CameraView1.camera(ui);
        self.CameraView1.set_camera(ui, format!("{atual}!"));
    }
}
```
