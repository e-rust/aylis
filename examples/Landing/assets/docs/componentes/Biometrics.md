# Biometrics

A biometria do aparelho: a digital ou o rosto no celular, o Windows Hello, o autenticador do aparelho no navegador. `Biometrics::authenticate` mostra a verificação do sistema com `reason`; o resultado chega em `on_success`, `on_fail` (com o motivo) ou `on_cancel`. Com `allow_device_credential`, o PIN ou o padrão do aparelho também valem.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo |
|---|---|---|
| `reason` | Motivo | texto |
| `allow_device_credential` | Aceita a senha do aparelho | lógico |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_success` | — | — |  |
| `on_fail` | — | — |  |
| `on_cancel` | — | — |  |

## Em código

No form, o `Biometrics1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "Biometrics", "name": "Biometrics1", "on_success": "biometrics1_success" }
```

E o handler, um método do form (`self.Biometrics1` já é um `Biometrics`):

```rust
impl FrmPrincipal {
    pub fn biometrics1_success(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.Biometrics1.reason(ui);
        self.Biometrics1.set_reason(ui, format!("{atual}!"));
    }
}
```
