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
