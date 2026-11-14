//! A automação OLE do Windows (o [`OleService`] do `OleObject`): `IDispatch` pelo nome, como o
//! VBA.
//!
//! Os objetos vivem numa thread só deles, com o COM em apartamento de thread única (o que o
//! Office espera). A tela manda o pedido e espera a resposta pelo canal, sem processar
//! mensagens: o laço do COM que roda durante uma chamada a outro processo não entra no laço
//! de eventos da janela (entraria na thread da tela, com o handler no meio). Parada, a
//! thread dorme no canal e não gasta nada.

use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use aylis_core::platform::{OleReleaser, OleService, OleValue, ole_allowed};
use windows::Win32::Foundation::{DISP_E_EXCEPTION, VARIANT_BOOL};
use windows::Win32::Globalization::LOCALE_USER_DEFAULT;
use windows::Win32::System::Com::{
    CLSCTX_SERVER, CLSIDFromProgID, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize, DISPATCH_FLAGS, DISPATCH_METHOD,
    DISPATCH_PROPERTYGET, DISPATCH_PROPERTYPUT, DISPPARAMS, EXCEPINFO, IDispatch, SAFEARRAY, SAFEARRAYBOUND,
};
use windows::Win32::System::Ole::{DISPID_PROPERTYPUT, GetActiveObject, SafeArrayCreate, SafeArrayGetDim, SafeArrayGetElement, SafeArrayGetLBound, SafeArrayGetUBound, SafeArrayPutElement};
use windows::Win32::System::Variant::{
    VAR_CHANGE_FLAGS, VARENUM, VARIANT, VT_ARRAY, VT_BOOL, VT_BSTR, VT_BYREF, VT_CY, VT_DATE, VT_DECIMAL, VT_DISPATCH, VT_EMPTY, VT_ERROR, VT_I1, VT_I2, VT_I4, VT_I8,
    VT_INT, VT_NULL, VT_R4, VT_R8, VT_UI1, VT_UI2, VT_UI4, VT_UI8, VT_UINT, VT_UNKNOWN, VT_VARIANT, VariantChangeType, VariantClear,
};
use windows::core::{BSTR, GUID, Interface, IUnknown, PCWSTR};

enum Request {
    Create { prog_id: String, running: bool, reply: mpsc::Sender<Result<u64, String>> },
    Invoke { object: u64, name: String, args: Vec<OleValue>, put: Option<OleValue>, reply: mpsc::Sender<Result<OleValue, String>> },
    Release(u64),
}

/// O serviço: o canal para a thread dos objetos (criada no primeiro `create`).
pub(crate) struct WinOle {
    tx: Option<mpsc::Sender<Request>>,
    timeout: Duration,
}

impl WinOle {
    pub(crate) fn new() -> Self {
        Self { tx: None, timeout: Duration::from_secs(30) }
    }

    fn sender(&mut self) -> Result<&mpsc::Sender<Request>, String> {
        if self.tx.is_none() {
            let (tx, rx) = mpsc::channel();
            std::thread::Builder::new().name("aylis-ole".into()).spawn(move || worker(rx)).map_err(|e| e.to_string())?;
            self.tx = Some(tx);
        }
        Ok(self.tx.as_ref().expect("criado acima"))
    }

    fn wait<T>(&self, rx: mpsc::Receiver<Result<T, String>>) -> Result<T, String> {
        match rx.recv_timeout(self.timeout) {
            Ok(r) => r,
            Err(mpsc::RecvTimeoutError::Timeout) => Err(aylis_core::messages::t("o programa não respondeu a tempo (há um diálogo aberto nele?)")),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(aylis_core::messages::t("a automação OLE parou")),
        }
    }
}

impl OleService for WinOle {
    fn create(&mut self, prog_id: &str, running: bool) -> Result<u64, String> {
        // Conferido também aqui: o serviço não cria o que o app não permitiu, venha de onde vier.
        if !ole_allowed(prog_id) {
            return Err(aylis_core::messages::tf("a automação de '{0}' não foi permitida pelo app", &[&prog_id]));
        }
        let (reply, rx) = mpsc::channel();
        self.sender()?.send(Request::Create { prog_id: prog_id.to_string(), running, reply }).map_err(|e| e.to_string())?;
        self.wait(rx)
    }

    fn invoke(&mut self, object: u64, name: &str, args: Vec<OleValue>) -> Result<OleValue, String> {
        let (reply, rx) = mpsc::channel();
        self.sender()?.send(Request::Invoke { object, name: name.to_string(), args, put: None, reply }).map_err(|e| e.to_string())?;
        self.wait(rx)
    }

    fn put(&mut self, object: u64, name: &str, args: Vec<OleValue>, value: OleValue) -> Result<(), String> {
        let (reply, rx) = mpsc::channel();
        self.sender()?.send(Request::Invoke { object, name: name.to_string(), args, put: Some(value), reply }).map_err(|e| e.to_string())?;
        self.wait(rx).map(|_| ())
    }

    fn release(&mut self, object: u64) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(Request::Release(object));
        }
    }

    fn set_timeout(&mut self, seconds: f64) {
        if seconds.is_finite() && seconds > 0.0 {
            self.timeout = Duration::from_secs_f64(seconds);
        }
    }

    fn releaser(&self) -> Option<OleReleaser> {
        let tx = self.tx.clone()?;
        Some(Arc::new(move |o| {
            let _ = tx.send(Request::Release(o));
        }))
    }
}

/// A thread dos objetos: atende os pedidos até o serviço acabar.
fn worker(rx: mpsc::Receiver<Request>) {
    // SAFETY: inicializa o COM desta thread, que é só da automação; o par é o
    // `CoUninitialize` do fim, depois de largar todos os objetos.
    let init = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    let mut objects: HashMap<u64, IDispatch> = HashMap::new();
    let mut next = 1u64;
    for request in rx {
        match request {
            Request::Create { prog_id, running, reply } => {
                let r = create(&prog_id, running).map(|d| {
                    let id = next;
                    next += 1;
                    objects.insert(id, d);
                    id
                });
                let _ = reply.send(r);
            }
            Request::Invoke { object, name, args, put, reply } => {
                let r = match objects.get(&object).cloned() {
                    Some(d) => invoke(&d, &name, &args, put.as_ref(), &mut objects, &mut next),
                    None => Err(aylis_core::messages::t("o objeto OLE já foi largado")),
                };
                let _ = reply.send(r);
            }
            Request::Release(o) => {
                objects.remove(&o);
            }
        }
    }
    objects.clear();
    if init.is_ok() {
        // SAFETY: o par do `CoInitializeEx` que deu certo, sem objetos vivos.
        unsafe { CoUninitialize() };
    }
}

fn create(prog_id: &str, running: bool) -> Result<IDispatch, String> {
    let wide = wide(prog_id);
    // SAFETY: `wide` termina em zero e vive até o fim da chamada.
    let clsid = unsafe { CLSIDFromProgID(PCWSTR(wide.as_ptr())) }.map_err(|_| aylis_core::messages::tf("o programa '{0}' não está instalado", &[&prog_id]))?;
    if running {
        let mut unknown: Option<IUnknown> = None;
        // SAFETY: `clsid` e `unknown` são locais válidos; o COM preenche `unknown`.
        if unsafe { GetActiveObject(&clsid, None, &mut unknown) }.is_ok() {
            if let Some(d) = unknown.and_then(|u| u.cast::<IDispatch>().ok()) {
                return Ok(d);
            }
        }
    }
    // SAFETY: cria o objeto do CLSID registrado; o resultado é uma interface com contagem.
    unsafe { CoCreateInstance::<_, IDispatch>(&clsid, None, CLSCTX_SERVER) }.map_err(|e| aylis_core::messages::tf("não consegui abrir '{0}': {1}", &[&prog_id, &e.message()]))
}

/// Chama `name` no objeto (lê ou chama; com `put`, grava).
fn invoke(d: &IDispatch, name: &str, args: &[OleValue], put: Option<&OleValue>, objects: &mut HashMap<u64, IDispatch>, next: &mut u64) -> Result<OleValue, String> {
    let w = wide(name);
    let names = [PCWSTR(w.as_ptr())];
    let mut id = 0i32;
    // SAFETY: um nome terminado em zero e um id de saída, ambos locais.
    unsafe { d.GetIDsOfNames(&GUID::zeroed(), names.as_ptr(), 1, LOCALE_USER_DEFAULT, &mut id) }.map_err(|_| aylis_core::messages::tf("o objeto não tem '{0}'", &[&name]))?;

    // O IDispatch recebe os argumentos do último para o primeiro; o valor gravado vem antes.
    let mut vars: Vec<VARIANT> = Vec::with_capacity(args.len() + 1);
    let mut built = Ok(());
    for v in put.into_iter().chain(args.iter().rev()) {
        match to_variant(v, objects) {
            Ok(var) => vars.push(var),
            Err(e) => {
                built = Err(e);
                break;
            }
        }
    }
    let result = built.and_then(|()| {
        let mut named = DISPID_PROPERTYPUT;
        let params = DISPPARAMS {
            rgvarg: vars.as_mut_ptr(),
            rgdispidNamedArgs: if put.is_some() { &mut named } else { std::ptr::null_mut() },
            cArgs: vars.len() as u32,
            cNamedArgs: u32::from(put.is_some()),
        };
        let flags = if put.is_some() { DISPATCH_PROPERTYPUT } else { DISPATCH_FLAGS(DISPATCH_METHOD.0 | DISPATCH_PROPERTYGET.0) };
        let mut out = VARIANT::default();
        let mut info = EXCEPINFO::default();
        let mut arg_err = 0u32;
        // SAFETY: `params` aponta para `vars` (vivos até o fim), e `out`/`info`/`arg_err` são
        // saídas locais; os dois são limpos abaixo.
        let r = unsafe { d.Invoke(id, &GUID::zeroed(), LOCALE_USER_DEFAULT, flags, &params, Some(&mut out), Some(&mut info), Some(&mut arg_err)) };
        let value = match r {
            Ok(()) => Ok(from_variant(&out, objects, next)),
            Err(e) if e.code() == DISP_E_EXCEPTION => {
                let text = exception_text(&mut info);
                Err(if text.is_empty() { e.message() } else { text })
            }
            Err(e) => Err(aylis_core::messages::tf("'{0}' falhou: {1}", &[&name, &e.message()])),
        };
        // SAFETY: `out` foi preenchido pelo Invoke (ou é VT_EMPTY): limpar é o par dele.
        unsafe {
            let _ = VariantClear(&mut out);
        }
        drop_exception(&mut info);
        value
    });
    for v in &mut vars {
        // SAFETY: cada um foi montado por `to_variant` e é dono do que guarda.
        unsafe {
            let _ = VariantClear(v);
        }
    }
    result
}

fn exception_text(info: &mut EXCEPINFO) -> String {
    if let Some(fill) = info.pfnDeferredFillIn {
        // SAFETY: a função que o próprio servidor deu para completar o `info`.
        unsafe {
            let _ = fill(info);
        }
    }
    let source = info.bstrSource.to_string();
    let text = info.bstrDescription.to_string();
    if source.is_empty() { text } else if text.is_empty() { source } else { format!("{source}: {text}") }
}

fn drop_exception(info: &mut EXCEPINFO) {
    // SAFETY: os BSTRs do `EXCEPINFO` são nossos depois do Invoke; cada um é largado uma vez.
    unsafe {
        ManuallyDrop::drop(&mut info.bstrSource);
        ManuallyDrop::drop(&mut info.bstrDescription);
        ManuallyDrop::drop(&mut info.bstrHelpFile);
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn variant(vt: VARENUM, fill: impl FnOnce(&mut windows::Win32::System::Variant::VARIANT_0_0_0)) -> VARIANT {
    let mut v = VARIANT::default();
    // SAFETY: o VARIANT zerado é VT_EMPTY; aqui se põe o tipo e o campo que ele usa.
    unsafe {
        let inner = &mut v.Anonymous.Anonymous;
        inner.vt = vt;
        fill(&mut inner.Anonymous);
    }
    v
}

fn to_variant(v: &OleValue, objects: &HashMap<u64, IDispatch>) -> Result<VARIANT, String> {
    Ok(match v {
        OleValue::Null => VARIANT::default(),
        OleValue::Bool(b) => variant(VT_BOOL, |u| u.boolVal = VARIANT_BOOL(if *b { -1 } else { 0 })),
        OleValue::Int(i) => match i32::try_from(*i) {
            Ok(n) => variant(VT_I4, |u| u.lVal = n),
            // O Office não aceita VT_I8 em todo lugar: o número grande vai como double.
            Err(_) => variant(VT_R8, |u| u.dblVal = *i as f64),
        },
        OleValue::Float(f) => variant(VT_R8, |u| u.dblVal = *f),
        OleValue::Date(d) => variant(VT_DATE, |u| u.date = *d),
        OleValue::String(s) => variant(VT_BSTR, |u| u.bstrVal = ManuallyDrop::new(BSTR::from(s.as_str()))),
        OleValue::Object(o) => {
            let d = objects.get(o).cloned().ok_or_else(|| aylis_core::messages::t("o objeto OLE já foi largado"))?;
            variant(VT_DISPATCH, |u| u.pdispVal = ManuallyDrop::new(Some(d)))
        }
        OleValue::Array(rows) => {
            let height = rows.len() as u32;
            let width = rows.iter().map(Vec::len).max().unwrap_or(0) as u32;
            let bounds = [SAFEARRAYBOUND { cElements: height, lLbound: 1 }, SAFEARRAYBOUND { cElements: width, lLbound: 1 }];
            // SAFETY: duas dimensões com os limites acima; o array passa a ser do VARIANT.
            let array = unsafe { SafeArrayCreate(VT_VARIANT, 2, bounds.as_ptr()) };
            if array.is_null() {
                return Err(aylis_core::messages::t("sem memória para a matriz"));
            }
            let out = variant(VARENUM(VT_ARRAY.0 | VT_VARIANT.0), |u| u.parray = array);
            for (r, row) in rows.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    let mut item = to_variant(cell, objects)?;
                    let index = [r as i32 + 1, c as i32 + 1];
                    // SAFETY: o índice está dentro dos limites; o PutElement copia o VARIANT, e
                    // a cópia local é limpa logo depois.
                    unsafe {
                        let _ = SafeArrayPutElement(array, index.as_ptr(), (&raw const item).cast());
                        let _ = VariantClear(&mut item);
                    }
                }
            }
            out
        }
    })
}

/// Os códigos de erro do Excel numa célula (`CVErr`).
fn excel_error(scode: i32) -> Option<&'static str> {
    Some(match scode as u32 {
        0x800A_07D0 => "#NULL!",
        0x800A_07D7 => "#DIV/0!",
        0x800A_07DF => "#VALUE!",
        0x800A_07E7 => "#REF!",
        0x800A_07ED => "#NAME?",
        0x800A_07F4 => "#NUM!",
        0x800A_07FA => "#N/A",
        _ => return None,
    })
}

fn from_variant(v: &VARIANT, objects: &mut HashMap<u64, IDispatch>, next: &mut u64) -> OleValue {
    // SAFETY: lê o campo que o `vt` diz estar preenchido.
    unsafe {
        let inner = &v.Anonymous.Anonymous;
        let vt = inner.vt;
        let u = &inner.Anonymous;
        if vt.0 & VT_BYREF.0 != 0 {
            return if vt.0 == (VT_BYREF.0 | VT_VARIANT.0) && !u.pvarVal.is_null() { from_variant(&*u.pvarVal, objects, next) } else { OleValue::Null };
        }
        if vt.0 & VT_ARRAY.0 != 0 {
            return from_array(u.parray, objects, next);
        }
        match vt {
            VT_EMPTY | VT_NULL => OleValue::Null,
            VT_BOOL => OleValue::Bool(u.boolVal.0 != 0),
            VT_I1 => OleValue::Int(i64::from(u.cVal)),
            VT_UI1 => OleValue::Int(i64::from(u.bVal)),
            VT_I2 => OleValue::Int(i64::from(u.iVal)),
            VT_UI2 => OleValue::Int(i64::from(u.uiVal)),
            VT_I4 | VT_INT => OleValue::Int(i64::from(u.lVal)),
            VT_UI4 | VT_UINT => OleValue::Int(i64::from(u.ulVal)),
            VT_I8 => OleValue::Int(u.llVal),
            VT_UI8 => OleValue::Int(u.ullVal as i64),
            VT_R4 => OleValue::Float(f64::from(u.fltVal)),
            VT_R8 => OleValue::Float(u.dblVal),
            VT_DATE => OleValue::Date(u.date),
            VT_BSTR => OleValue::String(u.bstrVal.to_string()),
            VT_ERROR => excel_error(u.scode).map_or(OleValue::Null, |e| OleValue::String(e.into())),
            VT_CY | VT_DECIMAL => {
                let mut d = VARIANT::default();
                if VariantChangeType(&mut d, v, VAR_CHANGE_FLAGS(0), VT_R8).is_ok() { OleValue::Float(d.Anonymous.Anonymous.Anonymous.dblVal) } else { OleValue::Null }
            }
            VT_DISPATCH | VT_UNKNOWN => {
                let disp = if vt == VT_DISPATCH { (*u.pdispVal).clone() } else { (*u.punkVal).as_ref().and_then(|k| k.cast::<IDispatch>().ok()) };
                match disp {
                    Some(d) => {
                        let id = *next;
                        *next += 1;
                        objects.insert(id, d);
                        OleValue::Object(id)
                    }
                    None => OleValue::Null,
                }
            }
            _ => OleValue::Null,
        }
    }
}

/// Uma matriz de uma ou duas dimensões (uma dimensão vira uma linha).
fn from_array(array: *mut SAFEARRAY, objects: &mut HashMap<u64, IDispatch>, next: &mut u64) -> OleValue {
    if array.is_null() {
        return OleValue::Null;
    }
    // SAFETY: o array veio do VARIANT do Invoke e vive até ele ser limpo; os índices ficam
    // dentro dos limites que o próprio array informa.
    unsafe {
        let dims = SafeArrayGetDim(array);
        let bound = |d: u32| -> (i32, i32) { (SafeArrayGetLBound(array, d).unwrap_or(0), SafeArrayGetUBound(array, d).unwrap_or(-1)) };
        let read = |index: &[i32], objects: &mut HashMap<u64, IDispatch>, next: &mut u64| {
            let mut item = VARIANT::default();
            if SafeArrayGetElement(array, index.as_ptr(), (&raw mut item).cast()).is_err() {
                return OleValue::Null;
            }
            let value = from_variant(&item, objects, next);
            let _ = VariantClear(&mut item);
            value
        };
        match dims {
            1 => {
                let (lo, hi) = bound(1);
                OleValue::Array(vec![(lo..=hi).map(|i| read(&[i], objects, next)).collect()])
            }
            2 => {
                let (r0, r1) = bound(1);
                let (c0, c1) = bound(2);
                OleValue::Array((r0..=r1).map(|r| (c0..=c1).map(|c| read(&[r, c], objects, next)).collect()).collect())
            }
            _ => OleValue::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Com o Excel instalado: abre escondido, grava uma matriz, lê de volta e fecha
    /// (`--ignored`; sem o Excel, só avisa).
    #[test]
    #[ignore]
    fn excel_round_trip() {
        let mut ole = WinOle::new();
        let app = match ole.create("Excel.Application", false) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("sem o Excel: {e}");
                return;
            }
        };
        let books = ole.invoke(app, "Workbooks", vec![]).unwrap();
        let OleValue::Object(books) = books else { panic!("Workbooks não é objeto") };
        let OleValue::Object(book) = ole.invoke(books, "Add", vec![]).unwrap() else { panic!() };
        let OleValue::Object(sheet) = ole.invoke(book, "ActiveSheet", vec![]).unwrap() else { panic!() };
        let OleValue::Object(range) = ole.invoke(sheet, "Range", vec![OleValue::String("A1:C2".into())]).unwrap() else { panic!() };
        let data = vec![
            vec![OleValue::String("Nome".into()), OleValue::String("Idade".into()), OleValue::String("Data".into())],
            vec![OleValue::String("Ana".into()), OleValue::Int(31), OleValue::Date(aylis_core::platform::ole_date(2026, 11, 12, 0, 0, 0))],
        ];
        ole.put(range, "Value", vec![], OleValue::Array(data.clone())).unwrap();
        let back = ole.invoke(range, "Value", vec![]).unwrap();
        // O Excel devolve os números como double.
        let OleValue::Array(rows) = back else { panic!("não veio matriz: {back:?}") };
        assert_eq!(rows[0], data[0]);
        assert_eq!(rows[1][0], OleValue::String("Ana".into()));
        assert_eq!(rows[1][1], OleValue::Float(31.0));
        assert_eq!(rows[1][2], data[1][2]);
        // A célula B1 isolada confirma a ordem linha × coluna.
        let OleValue::Object(b1) = ole.invoke(sheet, "Range", vec![OleValue::String("B1".into())]).unwrap() else { panic!() };
        assert_eq!(ole.invoke(b1, "Value", vec![]).unwrap(), OleValue::String("Idade".into()));
        ole.put(app, "DisplayAlerts", vec![], OleValue::Bool(false)).unwrap();
        ole.invoke(book, "Close", vec![OleValue::Bool(false)]).unwrap();
        for o in [b1, range, sheet, book, books] {
            ole.release(o);
        }
        ole.invoke(app, "Quit", vec![]).unwrap();
        ole.release(app);
    }

    /// O COM de verdade sem o Office: o `Scripting.Dictionary`, que vem com o Windows. Passa
    /// textos, números, datas e booleanos de ida e volta, recebe uma matriz (`Keys`) e o erro
    /// do próprio objeto.
    #[test]
    fn real_com_round_trip_with_the_dictionary() {
        aylis_core::platform::allow_ole("Scripting.Dictionary");
        let mut ole = WinOle::new();
        let d = ole.create("Scripting.Dictionary", false).unwrap();
        let date = aylis_core::platform::ole_date(2026, 11, 12, 8, 30, 0);
        let items = [
            ("texto", OleValue::String("Olá, ação".into())),
            ("inteiro", OleValue::Int(42)),
            ("real", OleValue::Float(2.5)),
            ("data", OleValue::Date(date)),
            ("sim", OleValue::Bool(true)),
            ("nada", OleValue::Null),
        ];
        for (k, v) in &items {
            ole.invoke(d, "Add", vec![OleValue::String((*k).into()), v.clone()]).unwrap();
        }
        assert_eq!(ole.invoke(d, "Count", vec![]).unwrap(), OleValue::Int(6));
        for (k, v) in &items {
            assert_eq!(&ole.invoke(d, "Item", vec![OleValue::String((*k).into())]).unwrap(), v, "{k}");
        }
        // Gravar uma propriedade com argumento (`Item("inteiro") = 7`).
        ole.put(d, "Item", vec![OleValue::String("inteiro".into())], OleValue::Int(7)).unwrap();
        assert_eq!(ole.invoke(d, "Item", vec![OleValue::String("inteiro".into())]).unwrap(), OleValue::Int(7));
        // Uma matriz que volta numa chamada só (uma dimensão vira uma linha).
        let OleValue::Array(keys) = ole.invoke(d, "Keys", vec![]).unwrap() else { panic!("Keys não veio matriz") };
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0][0], OleValue::String("texto".into()));
        assert_eq!(keys[0].len(), 6);
        // Uma matriz de duas dimensões como argumento, guardada e devolvida igual.
        let m = vec![vec![OleValue::Int(1), OleValue::String("a".into())], vec![OleValue::Date(date), OleValue::Null]];
        ole.invoke(d, "Add", vec![OleValue::String("matriz".into()), OleValue::Array(m.clone())]).unwrap();
        assert_eq!(ole.invoke(d, "Item", vec![OleValue::String("matriz".into())]).unwrap(), OleValue::Array(m));
        // O erro do objeto (chave repetida) volta como texto, e o nome errado também.
        assert!(ole.invoke(d, "Add", vec![OleValue::String("texto".into()), OleValue::Int(1)]).is_err());
        let e = ole.invoke(d, "NaoExiste", vec![]).unwrap_err();
        assert!(e.contains("NaoExiste"), "{e}");
        ole.release(d);
        assert!(ole.invoke(d, "Count", vec![]).is_err(), "largado");
    }

    #[test]
    fn refuses_what_the_app_did_not_allow() {
        let mut ole = WinOle::new();
        let e = ole.create("WScript.Shell", false).unwrap_err();
        assert!(e.contains("WScript.Shell"), "{e}");
    }
}
