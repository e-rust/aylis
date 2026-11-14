# Nfc

Lê e grava tags NFC (NDEF): `active` lê enquanto o app está aberto; cada tag encostada chega no `on_tag` (o texto é o id) com `tag_id`, `tag_type` e `records` (um registro por linha: `text:Olá`, `uri:https://...`, `application/json:{...}`). `write(registros)` grava na próxima tag que encostar (o resultado no `on_written` ou no `on_error`).

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Lê as tags encostadas enquanto o app está aberto (cada uma chega no `on_tag`). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_tag` | — | — |  |
| `on_written` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `Nfc1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Nfc", "name": "Nfc1", "on_tag": "nfc1_tag" }
```

E o handler, um método do form (`self.Nfc1` já é um `Nfc`):

```rust
impl FrmPrincipal {
    pub fn nfc1_tag(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Nfc1.active(ui);
        self.Nfc1.set_active(ui, !atual);
    }
}
```
