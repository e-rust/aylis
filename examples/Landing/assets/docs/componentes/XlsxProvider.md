# XlsxProvider

Planilha do Excel (`.xlsx`), pela biblioteca `rust_xlsxwriter` (trocar de biblioteca é escrever outro provedor): os títulos em negrito e congelados, números, datas e os formatos dos campos (`#,##0.00`), e a largura das colunas pelo conteúdo.

**Paleta:** Providers · **componente não visual**

## Propriedades

Só as comuns.

## Em código

No form, o `XlsxProvider1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "XlsxProvider", "name": "XlsxProvider1" }
```

E o handler, um método do form (`self.XlsxProvider1` já é um `XlsxProvider`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = &self.XlsxProvider1;
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)
