# EventRecorder

Grava tudo o que o usuário faz num .aylisrec (a IDE abre e mostra).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `active` | Ativo | lógico |
| `folder` | Pasta | texto |
| `app_name` | Nome do aplicativo | texto |
| `pointer_moves` | Movimentos do mouse | lógico |
| `mask_text` | Esconde o texto digitado | lógico |
| `record_sensitive` | Grava os dados pessoais (LGPD) | lógico |
| `timers` | Temporizadores | lógico |
| `keep` | Quantas guardar | inteiro |

### Dados

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `file_name` | Nome do arquivo | texto |

## Em código

No form, o `EventRecorder1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "EventRecorder", "name": "EventRecorder1" }
```

E o handler, um método do form (`self.EventRecorder1` já é um `EventRecorder`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.EventRecorder1.active(ui);
        self.EventRecorder1.set_active(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
