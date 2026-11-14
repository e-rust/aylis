# SystemBars

Escolhe as barras do sistema no celular: a cor da barra de status (`status_bar_color`) e a da de navegação (`navigation_bar_color`; vazia, a mesma da de status), a cor dos ícones (`icons`: `auto` pela claridade da barra, `light` ou `dark`) e se cada uma aparece (`status_bar_visible`, `navigation_bar_visible`: escondidas, a tela fica cheia, e a área segura muda). Sem cor, as barras ficam com o fundo do form principal, como sem o componente. O pedido vale para o app todo (com mais de um, o último que mudou).

**Paleta:** Device · **componente não visual**

## Propriedades

### Aparência

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `status_bar_color` | Cor da barra de status | cor | A cor da barra de status (em cima); vazia: o fundo do form principal. |
| `navigation_bar_color` | Cor da barra de navegação | cor | A cor da barra de navegação (embaixo); vazia: a mesma da de status. |
| `icons` | Ícones | um de: `auto`, `light`, `dark` | A cor dos ícones das barras: `auto` (pela claridade da barra), `light` ou `dark`. |

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `status_bar_visible` | Mostrar a barra de status | lógico | A barra de status aparece; escondida, a tela fica cheia e a área segura muda. |
| `navigation_bar_visible` | Mostrar a barra de navegação | lógico | A barra de navegação aparece; escondida, a tela fica cheia e a área segura muda. |

## Em código

No form, o `SystemBars1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "SystemBars", "name": "SystemBars1" }
```

E o handler, um método do form (`self.SystemBars1` já é um `SystemBars`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.SystemBars1.status_bar_visible(ui);
        self.SystemBars1.set_status_bar_visible(ui, !atual);
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
