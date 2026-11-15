# RenderSettings

Lê e troca em execução as configurações de pintura do app (cache de imagens, camadas, quadros de vídeo).

**Paleta:** System · **componente não visual**

## Propriedades

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro | Um número livre para o código (não muda nada). |

## Em código

No form, o `RenderSettings1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "RenderSettings", "name": "RenderSettings1" }
```

E o handler, um método do form (`self.RenderSettings1` já é um `RenderSettings`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.RenderSettings1.tag(ui);
        self.RenderSettings1.set_tag(ui, atual + 1);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)

## Mais sobre o RenderSettings

### As propriedades (só em execução)

As mesmas do `image_cache` do projeto (Opções do projeto → Memória). São lidas e trocadas
enquanto o app roda e não ficam gravadas no form: a configuração de uma vez é a do projeto.

| Propriedade | O que é | Faixa |
|---|---|---|
| `pixels_mb` | megabytes de imagens em pixels (quadros de vídeo, prévias, câmera) guardadas na placa | 8 a 4096 |
| `files` | quantas imagens de arquivo (PNG, JPEG, SVG) ficam guardadas | 16 a 65536 |
| `layers_mb` | megabytes das camadas guardadas (transições, `cache_layer`); `0`: nenhuma | 0 a 4096 |
| `auto_layers` | as camadas automáticas nos forms com `layer_cache = project` | |
| `frame_reuse_kb` | até quantos KB um quadro de vídeo reaproveita o bitmap do anterior (Direct2D) nos forms com `frame_reuse_kb = -1`; `0`: nunca | 0 a 32768 |

Fora da faixa, o valor é ajustado a ela (como o projeto faz ao abrir), e ler de volta diz o
que ficou. Vale no app inteiro, já no próximo quadro: baixar um limite solta o excedente no
próximo uso, e trocar `auto_layers` repinta.

### Os três níveis

1. O projeto (`.aylisproj`, `image_cache`) configura o app inteiro, ao abrir.
2. A propriedade do form configura aquele form: `layer_cache` (as camadas automáticas) e
   `frame_reuse_kb` (`-1`: o do projeto).
3. O `RenderSettings` troca o valor do app em execução (o que o form não tiver próprio).

Variável de ambiente (`AYLIS_FRAME_REUSE_KB`) só no desenvolvimento, em depuração.

```rust
// Um modo econômico: menos memória na placa e sem camadas automáticas.
self.RenderSettings1.set_pixels_mb(ui, 32);
self.RenderSettings1.set_layers_mb(ui, 0);
self.RenderSettings1.set_auto_layers(ui, false);
```

No designer, trocar não mexe na IDE: o valor fica só no componente.
