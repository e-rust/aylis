# OleObject

Comanda um programa instalado pela automação OLE (Excel, Word, Outlook), por caminhos do VBA: `ActiveSheet.Range("A1").Value`.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `prog_id` | Programa (ProgID) | texto | O programa (`Excel.Application`, `Word.Application`); fora do Office, só o que o app permitir (`allow_ole`). |
| `use_running` | Usar o já aberto | lógico | Usa o programa já aberto (só cria se não houver). |
| `call_timeout` | Tempo limite da chamada | inteiro | Milissegundos para esperar cada resposta do programa (um diálogo aberto nele o prende). |

## Em código

No form, o `OleObject1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "OleObject", "name": "OleObject1" }
```

E o handler, um método do form (`self.OleObject1` já é um `OleObject`):

```rust
impl FrmPrincipal {
    pub fn form_show(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.OleObject1.prog_id(ui);
        self.OleObject1.set_prog_id(ui, format!("{atual}!"));
    }
}
```

(Sem eventos próprios: o exemplo usa o `on_show` do form.)

## Mais sobre o OleObject

### Métodos

| Método | O que faz |
|---|---|
| `open()` | Abre o programa (ou pega o aberto, com `use_running`). Os outros métodos abrem sozinhos. |
| `release()` | Larga o programa. Não fecha: para fechar, `call("Quit")` antes. |
| `get_value(caminho)` | O valor no fim do caminho, como texto (data em ISO; matriz em JSON). |
| `get_number(caminho)` | O valor como número. |
| `set_value(caminho, valor)` | Grava. `true` e `false` vão como booleano; o resto, como texto (o Excel converte números e datas como se fossem digitados). |
| `call(caminho)` | Chama um método do programa (`Workbooks.Add`, `ActiveWorkbook.Save`). |
| `get_values(caminho)` | Uma matriz inteira em JSON, numa chamada (`Range("A1:C10").Value`). |
| `set_values(caminho, json)` | Grava uma matriz JSON inteira numa chamada (`[[1, "a"], [2, "b"]]`). |
| `put_data(célula, data_source)` | Os títulos e os registros de um dataset a partir da célula, numa chamada. |

### Caminhos

Os mesmos nomes do VBA, separados por ponto, com argumentos literais entre parênteses:
números, `"textos"` (aspas dobradas dentro), `true`, `false` e `null`. São só endereço: não há
variáveis, contas nem laços.

```text
Workbooks.Add
ActiveSheet.Range("A1").Value
Worksheets("Vendas").Cells(2, 3).Value
ActiveWorkbook.SaveAs("C:\Relatorios\vendas.xlsx")
```

### Exemplo: o dataset no Excel

```rust
pub fn btn_excel_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) -> Result<()> {
    let excel = self.oleExcel;
    excel.set_value(ui, "Visible", "true")?;
    excel.call(ui, "Workbooks.Add")?;
    let linhas = excel.put_data(ui, "ActiveSheet.Range(\"A1\")", self.dsClientes)?;
    excel.call(ui, "ActiveSheet.Columns.AutoFit")?;
    show_message(ui, &format!("{linhas} clientes enviados ao Excel"));
    Ok(())
}
```

### Uma chamada por lote

Cada chamada atravessa para o processo do programa e leva milissegundos. Para muitas células,
use `set_values` ou `put_data` (uma chamada para a planilha inteira), nunca um `set_value` por
célula. Enquanto o programa responde, a tela espera; o `call_timeout` limita a espera (um
diálogo aberto no Excel prende a resposta).

### Programas permitidos

Só abrem o Excel, o Word, o Outlook e o PowerPoint. Outro programa entra pelo código do app,
uma vez, no início:

```rust
aylis::core::platform::allow_ole("AutoCAD.Application");
```

Não há propriedade nem comando do protocolo para isso de propósito: quem comanda a tela de fora
(um backend remoto) não escolhe o que roda na máquina. Um `WScript.Shell`, por exemplo,
executaria comandos.

### Plataformas

Só no Windows (a automação OLE é do Windows). Nas outras plataformas o componente existe e o
`open` responde que não há automação; para só gerar planilhas, em qualquer plataforma e sem o
Excel instalado, use o `Export` com o `XlsxProvider`.
