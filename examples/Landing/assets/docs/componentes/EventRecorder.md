# EventRecorder

Grava tudo o que o usuário faz num .aylisrec (a IDE abre e mostra).

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `active` | Ativo | lógico | Grava: ligado, começa sozinho ao entrar na tela; desligar para a gravação. |
| `folder` | Pasta | texto | Pasta das gravações; vazio: `<pasta do app>/recordings` (`app_data_dir`). |
| `app_name` | Nome do aplicativo | texto | O nome do app (pasta e cabeçalho); vazio: o título do form principal. |
| `pointer_moves` | Movimentos do mouse | lógico | Grava o movimento do ponteiro (muitas linhas); os arrastos sempre entram. |
| `mask_text` | Esconde o texto digitado | lógico | Troca toda digitação por `*`. |
| `record_sensitive` | Grava os dados pessoais (LGPD) | lógico | Grava também o que se digita nos campos `sensitive` (dado pessoal). Desligado, eles gravam `*`. Liga e desliga em tempo de execução (a mudança fica na gravação). |
| `timers` | Temporizadores | lógico | Grava também os `on_timer` (um por tique). |
| `keep` | Quantas guardar | inteiro | Quantas gravações manter na pasta (as mais antigas saem). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `file_name` | Nome do arquivo | texto | O arquivo da gravação em andamento (só leitura). |

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
