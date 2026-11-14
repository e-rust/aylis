# RecordButton

![RecordButton](img/RecordButton.png)

Botão redondo de gravar: um círculo vermelho com o ponto branco no meio que vira um quadrado (parar) gravando, com o brilho pulsando. `paused` para o pulso; `finalizing` gira um arco verde em volta (o arquivo ainda está sendo fechado) e ignora o clique; `countdown` > 0 mostra os segundos no meio e um anel âmbar que esvazia (`countdown_fraction` de 1 a 0). Desabilitado (`enabled: false`) fica cinza. `on_click` no clique.

**Paleta:** Media · **controle visual** · também tem as [propriedades e eventos comuns a todo controle](controles.md)

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `glow` | Brilho | lógico | O brilho colorido em volta (desligue com uma camada translúcida por cima, onde ele vira pontos soltos em alguns renderizadores). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `recording` | Gravando | lógico | Gravando: o ponto vira o quadrado de parar e o brilho pulsa. |
| `paused` | Pausado | lógico | Gravação pausada: o brilho para de pulsar. |
| `finalizing` | Finalizando | lógico | Fechando o arquivo: um arco verde gira em volta e o clique é ignorado. |
| `countdown` | Contagem regressiva | inteiro | Os segundos da contagem antes de gravar, no meio do botão; 0 = sem contagem. |
| `countdown_fraction` | Fração da contagem | número | Quanto da contagem falta, de 1 (o anel âmbar cheio) a 0 (vazio). |

## Em código

No form, o `RecordButton1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "RecordButton", "name": "RecordButton1" }
```

E o handler, um método do form (`self.RecordButton1` já é um `RecordButton`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.RecordButton1.recording(ui);
        self.RecordButton1.set_recording(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
