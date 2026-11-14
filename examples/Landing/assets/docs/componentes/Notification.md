# Notification

Notificação do sistema (Windows, Android), com ações e imagem.

**Paleta:** System · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `image` | Imagem | texto | Imagem (PNG) ao lado do texto, opcional: arquivo ou asset do projeto. |
| `form` | Form | texto | Um form inteiro desenhado como a imagem grande da notificação (o nome dele; criado se preciso), no tamanho `form_width` x `form_height`. |
| `form_width` | Largura do form | número | A largura com que o `form` é desenhado na imagem, em pontos. |
| `form_height` | Altura do form | número | A altura com que o `form` é desenhado na imagem, em pontos. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `title` | Título | texto | O título da notificação. |
| `text` | Texto | texto | O texto da notificação, embaixo do título. |
| `app_name` | Nome do aplicativo | texto | O nome do app na notificação (vazio: o título do form principal). |
| `app_id` | ID do aplicativo | texto | O identificador do app no sistema (`empresa.app`; vazio: derivado do nome). |
| `notification_id` | ID da notificação | inteiro | Mostrar de novo com o mesmo número atualiza a notificação em vez de criar outra. |
| `channel` | Canal | texto | Canal do Android (o usuário configura som e destaque por canal). |
| `channel_name` | Nome do canal | texto | O nome do canal que o usuário vê nas configurações do Android. |
| `priority` | Prioridade | um de: `low`, `default`, `high` | `low` (silenciosa, só na lista), `default` ou `high` (aparece por cima do app, com som). |
| `actions` | Ações | texto | Botões: `"responder:Responder;arquivar:Arquivar"` (identificador:texto); o toque chega em `on_action` com o identificador. |
| `ongoing` | Em andamento | lógico | Fixa (o usuário não descarta), como um download em andamento. |
| `progress` | Progresso | número | Barra de progresso de 0 a 1; negativo: sem barra. |
| `full_screen` | Tela cheia | lógico | Android: abre o app por cima de tudo, até da tela de bloqueio (ligações, alarmes). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_click` | o clique (ou Enter e Espaço com o foco) | — |  |
| `on_action` | — | — |  |

## Em código

No form, o `Notification1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Notification", "name": "Notification1", "on_click": "notification1_click" }
```

E o handler, um método do form (`self.Notification1` já é um `Notification`):

```rust
impl FrmPrincipal {
    pub fn notification1_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Notification1.title(ui);
        self.Notification1.set_title(ui, format!("{atual}!"));
    }
}
```
