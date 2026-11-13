# WmiQuery

Consulta o WMI do Windows pela API nativa (o hardware, os discos, os processos, os serviços): `query` em WQL (`SELECT Name, Size, FreeSpace FROM Win32_LogicalDisk`) no `namespace` (`root\cimv2`); as linhas vão para um `ClientDataSet`, e com `watch` os eventos do sistema chegam no `on_event`.

**Paleta:** System · **componente não visual**

## Propriedades

### Comportamento

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `namespace` | Namespace do WMI | texto | Onde estão as classes (`root\cimv2`, o padrão; só desta máquina). |
| `query` | Consulta (WQL) | texto | A consulta em WQL (com `watch`, uma consulta de eventos). |
| `timeout` | Espera da consulta (s) | real | Segundos que `execute`, `get_value` e `get_json` esperam. |
| `watch` | Assinar eventos | lógico | Assina a consulta de eventos enquanto ligado (só com o app rodando). |

### Dados

| Propriedade | No Object Inspector | Tipo | O que faz |
|---|---|---|---|
| `data_set` | Conjunto de dados | componente | O `ClientDataSet` que recebe as linhas. |

## Eventos

| Evento | Quando | No Object Inspector | Pergunta |
|---|---|---|---|
| `on_done` | — | — |  |
| `on_event` | — | — |  |
| `on_error` | — | — |  |

## Em código

No form, o `WmiQuery1` com o evento ligado (no Object Inspector, na aba Eventos):

```json
{ "class": "WmiQuery", "name": "WmiQuery1", "on_done": "wmi_query1_done" }
```

E o handler, um método do form (`self.WmiQuery1` já é um `WmiQuery`):

```rust
impl FrmPrincipal {
    pub fn wmi_query1_done(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let atual = self.WmiQuery1.watch(ui);
        self.WmiQuery1.set_watch(ui, !atual);
    }
}
```

## Mais sobre o WmiQuery

### Métodos

| Método | O que faz |
|---|---|
| `execute()` | Roda a `query` e espera (até `timeout`): as linhas vão para o `data_set` e para `result`; devolve quantas vieram. |
| `execute_async()` | O mesmo sem esperar: `busy` fica ligado, e o fim chega no `on_done` (o número de linhas) ou no `on_error`. |
| `get_value(consulta, propriedade)` | O valor da propriedade na primeira linha, como texto (data em ISO; lista em JSON). |
| `get_json(consulta)` | Todas as linhas como uma lista JSON de objetos. |

### Consultas

WQL, a linguagem de consulta do WMI. Os nomes das classes estão na documentação da Microsoft
(`Win32_*`):

```text
SELECT Caption, Version, LastBootUpTime FROM Win32_OperatingSystem
SELECT Name, Size, FreeSpace FROM Win32_LogicalDisk WHERE DriveType = 3
SELECT Name, ProcessId, WorkingSetSize FROM Win32_Process
SELECT Name, State, StartMode FROM Win32_Service WHERE StartMode = 'Auto'
SELECT SerialNumber FROM Win32_BIOS
```

O `namespace` padrão é o `root\cimv2`, onde está quase tudo; outros, como o
`root\StandardCimv2` (rede) e o `root\WMI` (sensores), servem do mesmo jeito.

### O resultado no ClientDataSet

Com um `ClientDataSet` sem campos no `data_set`, a primeira consulta cria um `Field` por
coluna, no tipo do WMI: texto, inteiro (os de 64 bits também), real, lógico, data e hora (a do
WMI, `20261112083000.000000-180`, vira a hora local) e memo para as listas, que vão em JSON. As
colunas seguem a ordem do `SELECT`; com `*`, vêm todas. Com campos já definidos, cada um recebe
a coluna do mesmo nome, sem diferenciar maiúsculas, e o que não vier fica vazio. Ligue um
`DataSource` e um `DBGrid` para ver.

### Exemplo: os discos numa grade

```rust
pub fn btn_discos_click(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) -> Result<()> {
    self.wmiDiscos.set_query(ui, "SELECT Name, Size, FreeSpace FROM Win32_LogicalDisk WHERE DriveType = 3");
    let discos = self.wmiDiscos.execute(ui)?;
    let serie = self.wmiDiscos.get_value(ui, "SELECT SerialNumber FROM Win32_BIOS", "SerialNumber")?;
    show_message(ui, &format!("{discos} discos; número de série {serie}"));
    Ok(())
}
```

### Eventos do sistema

Com `watch` ligado, a `query` é uma consulta de eventos, e cada um chega no `on_event` como um
objeto JSON (também em `last_event`), com o objeto que mudou no `TargetInstance`:

```text
SELECT * FROM __InstanceCreationEvent WITHIN 2 WHERE TargetInstance ISA 'Win32_Process'
SELECT * FROM __InstanceDeletionEvent WITHIN 2 WHERE TargetInstance ISA 'Win32_LogicalDisk'
SELECT * FROM __InstanceModificationEvent WITHIN 5 WHERE TargetInstance ISA 'Win32_Service'
```

```json
{ "__CLASS": "__InstanceCreationEvent", "TIME_CREATED": 134076312000000000,
  "TargetInstance": { "__CLASS": "Win32_Process", "Name": "notepad.exe", "ProcessId": 4242 } }
```

O `WITHIN` diz de quantos em quantos segundos o WMI confere: um processo que abre e fecha
antes disso não aparece. A assinatura só liga com o app rodando (no designer, não), e
desligar o `watch` ou liberar o componente a encerra. Parado, o componente não usa timer.

### Só consulta, só nesta máquina

O componente não chama métodos do WMI (o `Win32_Process.Create` abriria programas, o
`Win32_OperatingSystem.Shutdown` desligaria a máquina), e o `namespace` é desta máquina: um
computador no caminho (`\\servidor\root\cimv2`) é recusado. Algumas classes pedem o app
rodando como administrador (o `Win32_ProcessStartTrace`, por exemplo); sem isso, a consulta
volta com "acesso negado".

### Em segundo plano

A consulta roda numa thread do WMI, fora da tela. O `execute` espera a resposta (a maioria
leva milissegundos; a lista de programas instalados, `Win32_Product`, leva vários segundos);
para não travar a tela numa consulta lenta, use o `execute_async`.

### Plataformas

Só no Windows (o WMI é do Windows). Nas outras plataformas o componente existe e as consultas
respondem que não há WMI.
