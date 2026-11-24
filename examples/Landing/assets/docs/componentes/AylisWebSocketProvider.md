# AylisWebSocketProvider

O provedor WebSocket do Aylis: o `tungstenite` (rustls) fora do navegador, o `WebSocket` do navegador dentro. É o padrão do `WebSocketClient` sem provedor ligado.

**Paleta:** Providers · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `AylisWebSocketProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "AylisWebSocketProvider", "name": "AylisWebSocketProvider1" }
```

E o handler, um método do form (`self.AylisWebSocketProvider1` já é um `AylisWebSocketProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.AylisWebSocketProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
