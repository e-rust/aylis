# DataModule

Um contêiner só de componentes não visuais e da regra de negócio, fora da tela: conexões, ClientDataSets, Timers, diálogos... É uma raiz como um form, mas nunca abre janela. Os forms que usam um DataModule o declaram em `"uses"` no layout e falam com os componentes dele pelo caminho (`dmDados.cdsClientes`); o aplicativo cria os DataModules antes dos forms. `on_create` dispara depois de carregado.

**Paleta:** Standard · **componente não visual**

## Propriedades

### Outros

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `tag` | Marca (número) | inteiro | Um número livre para o código (como o `Tag` do Delphi). |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_create` | o componente foi criado (o DataModule, ao abrir o app) | — |  |

## Em código

No form, o `DataModule1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "DataModule", "name": "DataModule1", "on_create": "data_module1_create" }
```

E o handler, um método do form (`self.DataModule1` já é um `DataModule`):

```rust
impl FrmPrincipal {
    pub fn data_module1_create(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.DataModule1.tag(ui);
        self.DataModule1.set_tag(ui, atual + 1);
    }
}
```
