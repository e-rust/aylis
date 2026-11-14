# BackgroundService

O serviço em segundo plano: a lógica sem tela do projeto (`src/background.rs`, com `pub fn run(service: aylis::background::Service)`) roda numa thread que continua com a tela fechada; no Android, num serviço em primeiro plano com a notificação fixa (`title`, `text`) do tipo `kind` (o que o Android 14+ exige: `dataSync`, `location`, `mediaPlayback`, `connectedDevice`...). Com `keep_alive`, o sistema reinicia o serviço (e no boot) e o app pede para sair da otimização de bateria; alguns fabricantes têm listas próprias (veja `docs/segundo-plano.md`). `send` manda texto à lógica; o que ela manda chega em `on_message` (guardado enquanto a tela estava fechada). No iOS não há serviço assim: a lógica roda com o app aberto.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Liga o serviço e a lógica do projeto; desligado, para os dois. |
| `title` | Título | texto | O título da notificação fixa do serviço; vazio: "Rodando em segundo plano". |
| `text` | Texto | texto | O texto da notificação fixa do serviço. |
| `kind` | Tipo | texto | O tipo do serviço no Android 14+ (`dataSync`, `location`, `mediaPlayback`, `connectedDevice`...). |
| `keep_alive` | Manter vivo | lógico | O sistema reinicia o serviço (e no boot) e o app pede para sair da otimização de bateria. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_message` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `BackgroundService1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "BackgroundService", "name": "BackgroundService1", "on_message": "background_service1_message" }
```

E o handler, um método do form (`self.BackgroundService1` já é um `BackgroundService`):

```rust
impl FrmPrincipal {
    pub fn background_service1_message(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.BackgroundService1.active(ui);
        self.BackgroundService1.set_active(ui, !atual);
    }
}
```
