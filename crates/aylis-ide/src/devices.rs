//! Os aparelhos Android para o Executar: uma thread olha o `adb devices -l` a cada 2 s
//! (enquanto a saída Android foi escolhida alguma vez) e a lista ao lado da saída mostra os
//! aparelhos — os sem autorização ou offline aparecem marcados. Plugar o cabo faz o aparelho
//! aparecer sozinho.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use aylis_controls::ComboBox;
use aylis_core::Ui;
use aylis_tools::env::AndroidDevice;

use crate::app::Ids;
use crate::state::Shared;

/// Quanto a thread espera entre uma olhada e outra.
const EVERY: Duration = Duration::from_secs(2);

/// O que a thread achou e a interface ainda não leu.
#[derive(Default)]
pub struct Watch {
    fresh: Mutex<Option<Vec<AndroidDevice>>>,
}

/// Começa a olhar os aparelhos (uma vez; sem o Android SDK, não faz nada).
pub fn start(state: &Shared) {
    if state.borrow().device_watch.is_some() {
        return;
    }
    let Some(sdk) = aylis_tools::env::android_sdk() else { return };
    let watch = Arc::new(Watch::default());
    let waker = state.borrow().waker.clone();
    state.borrow_mut().device_watch = Some(watch.clone());
    let _ = std::thread::Builder::new().name("aylis-ide-adb".into()).spawn(move || {
        let mut last: Option<Vec<AndroidDevice>> = None;
        // A IDE soltou o vigia (fechou): a thread acaba.
        while Arc::strong_count(&watch) > 1 {
            let list = aylis_tools::env::android_device_list(&sdk);
            if last.as_ref() != Some(&list) {
                *watch.fresh.lock().unwrap_or_else(|e| e.into_inner()) = Some(list.clone());
                last = Some(list);
                if let Some(w) = &waker {
                    w.wake_by_ref();
                }
            }
            std::thread::sleep(EVERY);
        }
    });
}

/// Pega a lista nova, se houver (na bomba de mensagens). `true` se mudou.
pub fn poll(state: &Shared) -> bool {
    let Some(watch) = state.borrow().device_watch.clone() else { return false };
    let Some(list) = watch.fresh.lock().unwrap_or_else(|e| e.into_inner()).take() else { return false };
    let mut st = state.borrow_mut();
    st.run_device = choose(st.run_device.take(), &list);
    st.devices = list;
    true
}

/// O aparelho do Executar: o escolhido enquanto o adb ainda o vê (mesmo offline: o cabo pode
/// voltar); se sumiu, o primeiro pronto.
fn choose(current: Option<String>, list: &[AndroidDevice]) -> Option<String> {
    match current {
        Some(s) if list.iter().any(|d| d.serial == s) => Some(s),
        current => list.iter().find(|d| d.is_ready()).map(|d| d.serial.clone()).or(current),
    }
}

/// Mostra a lista só com a saída Android, com os aparelhos de agora.
pub fn sync(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let android = state.borrow().run_target == aylis_tools::Target::Android;
    if android {
        start(state);
    }
    let visible = ui.control(ids.device_combo).is_some_and(|c| c.visible());
    if visible != android {
        ui.control_mut(ids.device_combo).unwrap().set_visible(android);
    }
    if !android {
        return;
    }
    let (items, index) = {
        let st = state.borrow();
        if st.devices.is_empty() {
            ("nenhum aparelho conectado".to_string(), 0)
        } else {
            let labels: Vec<String> = st.devices.iter().map(|d| d.label().replace(';', ",")).collect();
            let index = st.run_device.as_ref().and_then(|s| st.devices.iter().position(|d| &d.serial == s)).unwrap_or(0);
            (labels.join(";"), index as i64)
        }
    };
    if ui.get::<ComboBox>(ids.device_combo).is_some_and(|c| c.items != items) {
        let _ = ui.set_prop(ids.device_combo, "items", items);
    }
    if ui.get::<ComboBox>(ids.device_combo).is_some_and(|c| c.item_index != index) {
        let _ = ui.set_prop(ids.device_combo, "item_index", index);
    }
}

/// Escolheu na lista.
pub fn chosen(state: &Shared, index: usize) {
    let mut st = state.borrow_mut();
    if let Some(d) = st.devices.get(index) {
        st.run_device = Some(d.serial.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(serial: &str, state: &str) -> AndroidDevice {
        AndroidDevice { serial: serial.into(), model: String::new(), state: state.into() }
    }

    #[test]
    fn the_chosen_device_stays_while_adb_sees_it() {
        let list = vec![device("A", "offline"), device("B", "device")];
        assert_eq!(choose(None, &list).as_deref(), Some("B"), "o primeiro pronto");
        assert_eq!(choose(Some("A".into()), &list).as_deref(), Some("A"), "offline continua escolhido");
        assert_eq!(choose(Some("C".into()), &list).as_deref(), Some("B"), "sumiu: o primeiro pronto");
        assert_eq!(choose(Some("C".into()), &[]).as_deref(), Some("C"), "sem nenhum, fica o de antes");
    }
}
