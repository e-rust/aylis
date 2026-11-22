# Image

![Image](img/Image.png)

Mostra uma imagem do aplicativo (`image: "assets/images/logo.png"`, lida pelo serviço de recursos do projeto) ou uma `Image` definida por código (`set_picture`). Um arquivo do disco ou bytes na memória (a capa de uma música) vêm pelo código: `ImageView::set_image_file` e `ImageView::set_image_bytes` (decidido em 21/11: o form não aponta para uma pasta da máquina de quem desenhou).

**Paleta:** Standard · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `image` | Imagem | texto | Caminho do arquivo, relativo à pasta do projeto (só os do projeto; um do disco, pelo `ImageView::set_image_file`). |
| `fit` | Ajuste | um de: `contain`, `cover`, `stretch`, `center`, `original` | Como a imagem ocupa o controle: `contain` (inteira, mantendo a proporção), `cover` (cobre, cortando as bordas), `stretch` (estica), `center` ou `original` (tamanho real). |
| `radius` | Raio dos cantos | número | Arredonda os cantos (recorta a imagem). |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `zoomable` | Ampliar com pinça | lógico | A pinça amplia e o dedo arrasta a imagem ampliada. |

## Em código

No form, o `Image1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Image", "name": "Image1" }
```

E o handler, um método do form (`self.Image1` já é um `Image`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Image1.image(ui);
        self.Image1.set_image(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)

## Mais sobre o Image

### Arquivo do disco e bytes

A propriedade `image` é um arquivo do projeto (`assets/images/logo.png`): o form não aponta
para uma pasta da máquina de quem desenhou. Um arquivo do disco ou uma imagem na memória (a
capa embutida numa música, uma foto baixada) vêm pelo código:

```rust
self.imgCapa.set_image_file(ui, "/home/ana/Música/capa.jpg")?;
self.imgCapa.set_image_bytes(ui, &bytes);
let fundo = self.imgCapa.dominant_color(ui)?; // a cor mais comum, para o degradê
```

Nas outras linguagens, o `set_image_bytes` recebe os bytes em base64. Um `image` que não está
no projeto avisa na saída da IDE (no app compilado para depurar).
