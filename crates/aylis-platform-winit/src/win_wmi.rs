//! O WMI do Windows pela API nativa (o [`WmiService`] do `WmiQuery`): `IWbemLocator` →
//! `IWbemServices` → `ExecQuery`, sem o `IDispatch` da API de script.
//!
//! As consultas rodam numa thread só delas, com o COM multithread (o WMI não precisa de
//! apartamento) e as conexões guardadas por namespace; parada, a thread dorme no canal. Cada
//! assinatura de eventos tem a própria thread, esperando o próximo evento em fatias de 1 s
//! para ver se foi largada.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

use aylis_core::platform::{WmiColumn, WmiDone, WmiListener, WmiService, WmiTable, WmiType, WmiValue, WmiWatch, wmi_datetime, wmi_namespace, wmi_select_list};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoSetProxyBlanket, CoUninitialize, EOAC_NONE, RPC_C_AUTHN_LEVEL_CALL,
    RPC_C_IMP_LEVEL_IMPERSONATE, SAFEARRAY,
};
use windows::Win32::System::Ole::{SafeArrayGetDim, SafeArrayGetVartype, SafeArrayGetElement, SafeArrayGetLBound, SafeArrayGetUBound};
use windows::Win32::System::Variant::{
    VARIANT, VT_ARRAY, VT_BOOL, VT_BSTR, VT_BYREF, VT_DATE, VT_EMPTY, VT_I1, VT_I2, VT_I4, VT_I8, VT_INT, VT_NULL, VT_R4, VT_R8, VT_UI1, VT_UI2, VT_UI4, VT_UI8,
    VT_UINT, VT_UNKNOWN, VT_VARIANT, VariantClear,
};
use windows::Win32::System::Wmi::{
    CIM_BOOLEAN, CIM_CHAR16, CIM_DATETIME, CIM_FLAG_ARRAY, CIM_OBJECT, CIM_REAL32, CIM_REAL64, CIM_SINT8, CIM_SINT16, CIM_SINT32, CIM_SINT64, CIM_STRING, CIM_UINT8, CIM_UINT16,
    CIM_UINT32, CIM_UINT64, IEnumWbemClassObject, IWbemClassObject, IWbemLocator, IWbemServices, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_NONSYSTEM_ONLY,
    WBEM_FLAG_RETURN_IMMEDIATELY, WBEM_GENERIC_FLAG_TYPE, WbemLocator,
};
use windows::core::{BSTR, Interface, PCWSTR};

/// `RPC_C_AUTHN_WINNT` e `RPC_C_AUTHZ_NONE` (do `Win32_System_Rpc`, que não vale a feature).
const AUTHN_WINNT: u32 = 10;
const AUTHZ_NONE: u32 = 0;
/// `WBEM_S_FALSE`: o fim do resultado; `WBEM_S_TIMEDOUT`: nada chegou dentro do prazo.
const WBEM_S_FALSE: i32 = 1;
const WBEM_S_TIMEDOUT: i32 = 0x40004;

struct Query {
    namespace: String,
    wql: String,
    done: WmiDone,
}

/// O serviço: o canal para a thread das consultas (criada na primeira).
pub(crate) struct WinWmi {
    tx: Option<mpsc::Sender<Query>>,
}

impl WinWmi {
    pub(crate) fn new() -> Self {
        Self { tx: None }
    }
}

impl WmiService for WinWmi {
    fn query(&mut self, namespace: &str, wql: &str, done: WmiDone) {
        let namespace = match wmi_namespace(namespace) {
            Ok(ns) => ns,
            Err(e) => return done(Err(e)),
        };
        if self.tx.is_none() {
            let (tx, rx) = mpsc::channel();
            if let Err(e) = std::thread::Builder::new().name("aylis-wmi".into()).spawn(move || worker(rx)) {
                return done(Err(e.to_string()));
            }
            self.tx = Some(tx);
        }
        let request = Query { namespace, wql: wql.to_string(), done };
        if let Some(tx) = &self.tx
            && let Err(mpsc::SendError(q)) = tx.send(request)
        {
            (q.done)(Err(aylis_core::messages::t("o WMI parou")));
        }
    }

    fn watch(&mut self, namespace: &str, wql: &str, listener: WmiListener) -> Result<WmiWatch, String> {
        let namespace = wmi_namespace(namespace)?;
        let wql = wql.to_string();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        // A assinatura começa na thread dela; o erro de começo (consulta errada) volta aqui.
        let (ready, started) = mpsc::channel();
        std::thread::Builder::new().name("aylis-wmi-watch".into()).spawn(move || watch_loop(&namespace, &wql, &listener, &flag, ready)).map_err(|e| e.to_string())?;
        match started.recv() {
            Ok(Ok(())) => Ok(WmiWatch::new(move || stop.store(true, Ordering::Relaxed))),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(aylis_core::messages::t("o WMI parou")),
        }
    }
}

/// Inicializa o COM da thread (multithread) e desfaz no fim.
struct ComThread(bool);

impl ComThread {
    fn new() -> Self {
        // SAFETY: a thread é só do WMI; o par é o `CoUninitialize` do `Drop`, depois de largar
        // as interfaces (elas são locais que caem antes).
        Self(unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok())
    }
}

impl Drop for ComThread {
    fn drop(&mut self) {
        if self.0 {
            // SAFETY: o par do `CoInitializeEx` que deu certo.
            unsafe { CoUninitialize() };
        }
    }
}

/// A thread das consultas: atende uma por vez, com as conexões guardadas por namespace.
fn worker(rx: mpsc::Receiver<Query>) {
    let _com = ComThread::new();
    let mut connections: HashMap<String, IWbemServices> = HashMap::new();
    for q in rx {
        let key = q.namespace.to_ascii_lowercase();
        let services = match connections.get(&key) {
            Some(s) => Ok(s.clone()),
            None => connect(&q.namespace).inspect(|s| {
                connections.insert(key.clone(), s.clone());
            }),
        };
        let result = services.and_then(|s| run_query(&s, &q.wql));
        (q.done)(result);
    }
}

/// Conecta ao namespace desta máquina, com a identidade de quem roda o app.
fn connect(namespace: &str) -> Result<IWbemServices, String> {
    let fail = |e: windows::core::Error| aylis_core::messages::tf("não consegui conectar ao WMI ({0}): {1}", &[&namespace, &e.message()]);
    // SAFETY: cria o localizador registrado do WMI; o resultado é uma interface com contagem.
    let locator: IWbemLocator = unsafe { CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER) }.map_err(fail)?;
    let empty = BSTR::new();
    // SAFETY: os BSTRs vivem até o fim da chamada; usuário e senha vazios são os do processo.
    let services = unsafe { locator.ConnectServer(&BSTR::from(namespace), &empty, &empty, &empty, 0, &empty, None) }.map_err(fail)?;
    // SAFETY: ajusta a segurança do proxy da conexão (personificar para o WMI ler em nome do
    // processo), como a documentação do WMI pede; nada global do processo muda.
    unsafe { CoSetProxyBlanket(&services, AUTHN_WINNT, AUTHZ_NONE, PCWSTR::null(), RPC_C_AUTHN_LEVEL_CALL, RPC_C_IMP_LEVEL_IMPERSONATE, None, EOAC_NONE) }.map_err(fail)?;
    Ok(services)
}

fn query_error(wql: &str, e: &windows::core::Error) -> String {
    aylis_core::messages::tf("a consulta do WMI falhou ({0}): {1}", &[&wql, &wmi_message(e)])
}

/// A mensagem do erro do WMI (os códigos `WBEM_E_*` não têm texto no sistema).
fn wmi_message(e: &windows::core::Error) -> String {
    match e.code().0 as u32 {
        0x8004_1017 => aylis_core::messages::t("consulta inválida"),
        0x8004_1010 => aylis_core::messages::t("classe inválida"),
        0x8004_100E => aylis_core::messages::t("namespace inválido"),
        0x8004_1003 => aylis_core::messages::t("acesso negado"),
        0x8004_1011 | 0x8004_1002 => aylis_core::messages::t("não encontrado"),
        _ => {
            let m = e.message();
            if m.trim().is_empty() { format!("0x{:08X}", e.code().0) } else { m }
        }
    }
}

/// Roda a consulta e lê todas as linhas.
fn run_query(services: &IWbemServices, wql: &str) -> Result<WmiTable, String> {
    let flags = WBEM_GENERIC_FLAG_TYPE(WBEM_FLAG_FORWARD_ONLY.0 | WBEM_FLAG_RETURN_IMMEDIATELY.0);
    // SAFETY: os BSTRs vivem até o fim da chamada; o resultado é uma interface com contagem.
    let list = unsafe { services.ExecQuery(&BSTR::from("WQL"), &BSTR::from(wql), flags, None) }.map_err(|e| query_error(wql, &e))?;
    let mut table = WmiTable::default();
    let mut index: HashMap<String, usize> = HashMap::new();
    // As colunas que a consulta pediu, na ordem dela.
    for name in wmi_select_list(wql) {
        index.insert(name.to_ascii_lowercase(), table.columns.len());
        table.columns.push(WmiColumn { name, kind: WmiType::String });
    }
    let mut typed = vec![false; table.columns.len()];
    while let Some(object) = next(&list, -1).map_err(|e| query_error(wql, &e))? {
        let mut row = vec![WmiValue::Null; table.columns.len()];
        for (name, kind, value) in properties(&object, false) {
            let at = match index.get(&name.to_ascii_lowercase()) {
                Some(&i) => i,
                None => {
                    index.insert(name.to_ascii_lowercase(), table.columns.len());
                    table.columns.push(WmiColumn { name, kind: WmiType::String });
                    typed.push(false);
                    for r in &mut table.rows {
                        r.push(WmiValue::Null);
                    }
                    row.push(WmiValue::Null);
                    table.columns.len() - 1
                }
            };
            if !typed[at] {
                table.columns[at].kind = kind;
                typed[at] = true;
            }
            row[at] = value;
        }
        table.rows.push(row);
    }
    Ok(table)
}

/// O próximo objeto do resultado (`None` no fim; com prazo, também quando nada chegou).
fn next(list: &IEnumWbemClassObject, timeout_ms: i32) -> windows::core::Result<Option<IWbemClassObject>> {
    let mut objects = [None];
    let mut returned = 0u32;
    // SAFETY: um lugar para um objeto e o contador, ambos locais.
    let hr = unsafe { list.Next(timeout_ms, &mut objects, &mut returned) };
    if hr.is_err() {
        return Err(hr.into());
    }
    // Nenhum objeto: o fim (`WBEM_S_FALSE`) ou o prazo (`WBEM_S_TIMEDOUT`).
    debug_assert!(returned > 0 || hr.0 == WBEM_S_FALSE || hr.0 == WBEM_S_TIMEDOUT || hr.0 == 0);
    Ok(if returned == 0 { None } else { objects[0].take() })
}

/// As propriedades do objeto (sem as do sistema, `__PATH` e companhia): nome, tipo e valor.
/// Com `class`, o `__CLASS` vem primeiro (o evento e o objeto embutido dizem o que são).
fn properties(object: &IWbemClassObject, class: bool) -> Vec<(String, WmiType, WmiValue)> {
    let mut out = Vec::new();
    if class {
        let mut v = VARIANT::default();
        let name: Vec<u16> = "__CLASS\0".encode_utf16().collect();
        // SAFETY: nome terminado em zero e um VARIANT local, limpo logo depois.
        if unsafe { object.Get(PCWSTR(name.as_ptr()), 0, &mut v, None, None) }.is_ok() {
            out.push(("__CLASS".to_string(), WmiType::String, from_variant(&v, CIM_STRING.0)));
        }
        // SAFETY: o VARIANT preenchido pelo `Get` (ou vazio).
        unsafe {
            let _ = VariantClear(&mut v);
        }
    }
    // SAFETY: a enumeração é do próprio objeto, aberta e fechada aqui; cada nome e VARIANT
    // devolvido é nosso e é largado/limpo depois de lido.
    unsafe {
        if object.BeginEnumeration(WBEM_FLAG_NONSYSTEM_ONLY.0).is_err() {
            return out;
        }
        loop {
            let mut name = BSTR::new();
            let mut value = VARIANT::default();
            let mut cim = 0i32;
            let mut flavor = 0i32;
            if object.Next(0, &mut name, &mut value, &mut cim, &mut flavor).is_err() || name.is_empty() {
                let _ = VariantClear(&mut value);
                break;
            }
            out.push((name.to_string(), column_type(cim), from_variant(&value, cim)));
            let _ = VariantClear(&mut value);
        }
        let _ = object.EndEnumeration();
    }
    out
}

fn column_type(cim: i32) -> WmiType {
    if cim & CIM_FLAG_ARRAY.0 != 0 {
        return WmiType::Array;
    }
    match cim {
        c if c == CIM_BOOLEAN.0 => WmiType::Boolean,
        c if [CIM_SINT8, CIM_UINT8, CIM_SINT16, CIM_UINT16, CIM_SINT32, CIM_UINT32, CIM_SINT64, CIM_UINT64].iter().any(|t| t.0 == c) => WmiType::Integer,
        c if c == CIM_REAL32.0 || c == CIM_REAL64.0 => WmiType::Float,
        c if c == CIM_DATETIME.0 => WmiType::DateTime,
        c if c == CIM_OBJECT.0 => WmiType::Object,
        _ => WmiType::String,
    }
}

/// O valor do VARIANT no tipo do CIM: os inteiros de 64 bits chegam como texto, e a data é
/// texto no formato do CIM.
fn from_variant(v: &VARIANT, cim: i32) -> WmiValue {
    let base = cim & !CIM_FLAG_ARRAY.0;
    // SAFETY: lê o campo que o `vt` diz estar preenchido.
    unsafe {
        let inner = &v.Anonymous.Anonymous;
        let vt = inner.vt;
        let u = &inner.Anonymous;
        if vt.0 & VT_BYREF.0 != 0 {
            return if vt.0 == (VT_BYREF.0 | VT_VARIANT.0) && !u.pvarVal.is_null() { from_variant(&*u.pvarVal, cim) } else { WmiValue::Null };
        }
        if vt.0 & VT_ARRAY.0 != 0 {
            return from_array(u.parray, base);
        }
        match vt {
            VT_EMPTY | VT_NULL => WmiValue::Null,
            VT_BOOL => WmiValue::Bool(u.boolVal.0 != 0),
            VT_I1 => WmiValue::Int(i64::from(u.cVal)),
            VT_UI1 => WmiValue::Int(i64::from(u.bVal)),
            // O `CIM_CHAR16` chega como inteiro de 16 bits: é um caractere.
            VT_I2 if base == CIM_CHAR16.0 => WmiValue::String(char::from_u32(u.iVal as u16 as u32).map(String::from).unwrap_or_default()),
            VT_I2 => WmiValue::Int(i64::from(u.iVal)),
            VT_UI2 => WmiValue::Int(i64::from(u.uiVal)),
            VT_I4 | VT_INT => WmiValue::Int(i64::from(u.lVal)),
            VT_UI4 | VT_UINT => WmiValue::Int(i64::from(u.ulVal)),
            VT_I8 => WmiValue::Int(u.llVal),
            VT_UI8 => i64::try_from(u.ullVal).map_or(WmiValue::Float(u.ullVal as f64), WmiValue::Int),
            VT_R4 => WmiValue::Float(f64::from(u.fltVal)),
            VT_R8 | VT_DATE => WmiValue::Float(u.dblVal),
            VT_BSTR => {
                let s = u.bstrVal.to_string();
                if base == CIM_SINT64.0 || base == CIM_UINT64.0 {
                    match s.trim().parse::<i64>() {
                        Ok(n) => WmiValue::Int(n),
                        Err(_) => s.trim().parse::<f64>().map_or(WmiValue::String(s), WmiValue::Float),
                    }
                } else if base == CIM_DATETIME.0 {
                    wmi_datetime(&s).map_or(WmiValue::String(s), WmiValue::DateTime)
                } else {
                    WmiValue::String(s)
                }
            }
            VT_UNKNOWN => match (*u.punkVal).as_ref().and_then(|k| k.cast::<IWbemClassObject>().ok()) {
                Some(o) => WmiValue::Object(properties(&o, true).into_iter().map(|(n, _, v)| (n, v)).collect()),
                None => WmiValue::Null,
            },
            _ => WmiValue::Null,
        }
    }
}

/// Uma lista do CIM (uma dimensão).
fn from_array(array: *mut SAFEARRAY, base: i32) -> WmiValue {
    if array.is_null() {
        return WmiValue::Null;
    }
    // SAFETY: o array veio do VARIANT da propriedade e vive até ele ser limpo; os índices ficam
    // dentro dos limites que o próprio array informa. Cada elemento sai como VARIANT pela
    // cópia (`SafeArrayGetElement` de um array de VARIANT) ou é lido no tipo do array.
    unsafe {
        if SafeArrayGetDim(array) != 1 {
            return WmiValue::Null;
        }
        let lo = SafeArrayGetLBound(array, 1).unwrap_or(0);
        let hi = SafeArrayGetUBound(array, 1).unwrap_or(-1);
        let vt = SafeArrayGetVartype(array).unwrap_or(VT_VARIANT);
        let mut items = Vec::new();
        for i in lo..=hi {
            let index = [i];
            let mut item = VARIANT::default();
            if vt == VT_VARIANT {
                if SafeArrayGetElement(array, index.as_ptr(), (&raw mut item).cast()).is_err() {
                    items.push(WmiValue::Null);
                    continue;
                }
            } else {
                // O elemento no tipo do array, posto num VARIANT desse tipo para ler igual.
                let inner = &mut item.Anonymous.Anonymous;
                if SafeArrayGetElement(array, index.as_ptr(), (&raw mut inner.Anonymous).cast()).is_err() {
                    items.push(WmiValue::Null);
                    continue;
                }
                inner.vt = vt;
            }
            items.push(from_variant(&item, base));
            let _ = VariantClear(&mut item);
        }
        WmiValue::Array(items)
    }
}

/// A thread de uma assinatura: avisa se começou e repassa cada evento até ser largada.
fn watch_loop(namespace: &str, wql: &str, listener: &WmiListener, stop: &AtomicBool, ready: mpsc::Sender<Result<(), String>>) {
    let _com = ComThread::new();
    let flags = WBEM_GENERIC_FLAG_TYPE(WBEM_FLAG_FORWARD_ONLY.0 | WBEM_FLAG_RETURN_IMMEDIATELY.0);
    let events = connect(namespace).and_then(|s| {
        // SAFETY: os BSTRs vivem até o fim da chamada; o resultado é uma interface com contagem.
        unsafe { s.ExecNotificationQuery(&BSTR::from("WQL"), &BSTR::from(wql), flags, None) }.map_err(|e| query_error(wql, &e))
    });
    let events = match events {
        Ok(e) => {
            let _ = ready.send(Ok(()));
            e
        }
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    while !stop.load(Ordering::Relaxed) {
        match next(&events, 1000) {
            Ok(Some(event)) => {
                if !stop.load(Ordering::Relaxed) {
                    listener(Ok(properties(&event, true).into_iter().map(|(n, _, v)| (n, v)).collect()));
                }
            }
            Ok(None) => {}
            Err(e) => {
                if !stop.load(Ordering::Relaxed) {
                    listener(Err(query_error(wql, &e)));
                }
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(namespace: &str, wql: &str) -> Result<WmiTable, String> {
        let (tx, rx) = mpsc::channel();
        WinWmi::new().query(namespace, wql, Box::new(move |r| tx.send(r).unwrap()));
        rx.recv_timeout(std::time::Duration::from_secs(60)).unwrap()
    }

    /// O WMI de verdade: o sistema, com as colunas na ordem do `SELECT`, os tipos do CIM (o
    /// número de 64 bits que chega como texto, a data) e uma lista.
    #[test]
    fn reads_the_operating_system() {
        let t = run("root\\cimv2", "SELECT Caption, TotalVisibleMemorySize, LastBootUpTime, MUILanguages FROM Win32_OperatingSystem").unwrap();
        let names: Vec<&str> = t.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["Caption", "TotalVisibleMemorySize", "LastBootUpTime", "MUILanguages"]);
        assert_eq!(t.rows.len(), 1);
        let row = &t.rows[0];
        assert!(matches!(&row[0], WmiValue::String(s) if s.contains("Windows")), "{row:?}");
        assert!(matches!(row[1], WmiValue::Int(n) if n > 0), "{row:?}");
        assert_eq!(t.columns[1].kind, WmiType::Integer);
        assert!(matches!(&row[2], WmiValue::DateTime(d) if d.len() == 19 && d.as_bytes()[10] == b'T'), "{row:?}");
        assert_eq!(t.columns[2].kind, WmiType::DateTime);
        assert!(matches!(&row[3], WmiValue::Array(a) if !a.is_empty()), "{row:?}");
    }

    /// Várias linhas, `*` (as colunas que vierem) e a conexão guardada entre consultas.
    #[test]
    fn many_rows_and_the_star() {
        let mut wmi = WinWmi::new();
        for _ in 0..2 {
            let (tx, rx) = mpsc::channel();
            wmi.query("root/cimv2", "SELECT * FROM Win32_Process", Box::new(move |r| tx.send(r).unwrap()));
            let t = rx.recv_timeout(std::time::Duration::from_secs(60)).unwrap().unwrap();
            assert!(t.rows.len() > 3);
            let at = t.columns.iter().position(|c| c.name == "ProcessId").expect("ProcessId");
            assert!(t.rows.iter().all(|r| r.len() == t.columns.len()));
            assert!(t.rows.iter().any(|r| matches!(r[at], WmiValue::Int(_))));
        }
    }

    #[test]
    fn errors_come_back_as_text() {
        let e = run("root\\cimv2", "SELECT * FROM Win32_NaoExiste").unwrap_err();
        assert!(e.contains("Win32_NaoExiste"), "{e}");
        let e = run("\\\\servidor\\root\\cimv2", "SELECT * FROM Win32_Bios").unwrap_err();
        assert!(e.contains("servidor"), "{e}");
    }

    /// Uma assinatura começa, a consulta errada é recusada na hora e largar para a thread.
    #[test]
    fn watch_starts_and_stops() {
        let mut wmi = WinWmi::new();
        let listener: WmiListener = Arc::new(|_| {});
        assert!(wmi.watch("root\\cimv2", "SELECT * FROM __InstanceNaoExiste", listener.clone()).is_err());
        let w = wmi.watch("root\\cimv2", "SELECT * FROM __InstanceCreationEvent WITHIN 1 WHERE TargetInstance ISA 'Win32_Process'", listener).unwrap();
        drop(w);
    }

    /// Um evento de verdade: um processo novo chega com o `TargetInstance` como objeto. O
    /// `ping` fica uns segundos aberto: o WMI confere a cada `WITHIN` segundos, e um processo
    /// que fecha antes disso não aparece.
    #[test]
    fn a_new_process_arrives_as_an_event() {
        let mut wmi = WinWmi::new();
        let (tx, rx) = mpsc::channel();
        let tx = std::sync::Mutex::new(tx);
        let listener: WmiListener = Arc::new(move |e| {
            let _ = tx.lock().unwrap().send(e);
        });
        let _w = wmi.watch("root\\cimv2", "SELECT * FROM __InstanceCreationEvent WITHIN 1 WHERE TargetInstance ISA 'Win32_Process' AND TargetInstance.Name = 'PING.EXE'", listener).unwrap();
        let mut ping = std::process::Command::new("ping").args(["-n", "4", "127.0.0.1"]).stdout(std::process::Stdio::null()).spawn().unwrap();
        let event = rx.recv_timeout(std::time::Duration::from_secs(15));
        let _ = ping.kill();
        let _ = ping.wait();
        let event = event.expect("o evento não chegou").unwrap();
        assert!(event.iter().any(|(n, v)| n == "__CLASS" && *v == WmiValue::String("__InstanceCreationEvent".into())), "{event:?}");
        let target = event.iter().find(|(n, _)| n == "TargetInstance").map(|(_, v)| v.clone());
        let Some(WmiValue::Object(props)) = target else { panic!("sem TargetInstance: {event:?}") };
        assert!(props.iter().any(|(n, v)| n == "Name" && matches!(v, WmiValue::String(s) if s.eq_ignore_ascii_case("ping.exe"))), "{props:?}");
    }
}
