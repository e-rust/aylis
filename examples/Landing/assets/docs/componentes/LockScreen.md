# LockScreen

O form sobre a tela de bloqueio (`show_when_locked`), acendendo a tela ao abrir (`turn_screen_on`), e `dismiss` para pedir o desbloqueio. Com uma `Notification` de `full_screen`, o app abre por cima do bloqueio (uma ligação, um alarme). Hoje, o Android.

**Paleta:** Device · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `show_when_locked` | Mostrar no bloqueio | lógico |  |
| `turn_screen_on` | Acender a tela | lógico |  |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_error` | — | — |  |

## Em código

No form, o `LockScreen1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "LockScreen", "name": "LockScreen1", "on_error": "lock_screen1_error" }
```

E o handler, um método do form (`self.LockScreen1` já é um `LockScreen`):

```rust
impl FrmPrincipal {
    pub fn lock_screen1_error(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.LockScreen1.show_when_locked(ui);
        self.LockScreen1.set_show_when_locked(ui, !atual);
    }
}
```
