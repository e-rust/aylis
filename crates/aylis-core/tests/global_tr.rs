//! O `tr` livre: o mesmo dicionário do `ui.tr`, de qualquer thread (a bandeja, uma
//! notificação, uma thread de trabalho). Arquivo próprio: o dicionário vale para o processo.

use std::collections::HashMap;

use aylis_core::*;

#[test]
fn the_app_dictionary_translates_outside_the_ui_and_in_other_threads() {
    let mut ui = Ui::new();
    let map: HashMap<String, String> = [("Gravando", "Recording"), ("{0} de {1}", "{1} of {0}"), ("Pasta {} criada", "Folder {} created")]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    ui.set_translations(map);
    assert_eq!(ui.tr("Gravando"), "Recording");
    let other = std::thread::spawn(|| (tr("Gravando"), tr_with("{0} de {1}", &[&3, &10]), tr("Pasta Vídeos criada"), tr("Sem tradução"))).join().unwrap();
    assert_eq!(other.0, "Recording");
    assert_eq!(other.1, "10 of 3", "numerados: a tradução muda a ordem");
    assert_eq!(other.2, "Folder Vídeos created", "um texto já montado, pelo modelo");
    assert_eq!(other.3, "Sem tradução");
}
