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
| `recording` | Gravando | lógico |  |
| `paused` | Pausado | lógico |  |
| `finalizing` | Finalizando | lógico |  |
| `countdown` | Contagem regressiva | inteiro |  |
| `countdown_fraction` | Fração da contagem | número |  |

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
