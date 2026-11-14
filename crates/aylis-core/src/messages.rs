//! As mensagens do próprio aylis (botões e títulos dos diálogos, validação dos campos,
//! textos do grid...) nos idiomas que o aylis já traz: português (o original), inglês e
//! espanhol.
//!
//! O texto em português é a chave; partes variáveis vão como `{0}`, `{1}` ([`tf`]). O app
//! pode trocar qualquer uma no próprio `locales/<idioma>.json` (o [`Ui::tr`](crate::Ui::tr)
//! olha primeiro o dicionário do app e depois este).
//!
//! O idioma vale para o processo inteiro ([`set_language`]): mensagens nascem em lugares
//! sem o `Ui` à mão (a validação de um campo, por exemplo).

use std::sync::RwLock;

static LANGUAGE: RwLock<Lang> = RwLock::new(Lang::Pt);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lang {
    Pt,
    En,
    Es,
}

/// Escolhe o idioma das mensagens do aylis (`pt-BR`, `en-US`, `es`...; só a língua conta).
/// Outro idioma fica em português até o aylis trazer o dele (ou o app traduzir no
/// `locales/`).
pub fn set_language(language: &str) {
    let lang = match language.split(['-', '_']).next().unwrap_or_default().to_ascii_lowercase().as_str() {
        "en" => Lang::En,
        "es" => Lang::Es,
        _ => Lang::Pt,
    };
    if let Ok(mut l) = LANGUAGE.write() {
        *l = lang;
    }
}

/// A língua atual das mensagens do aylis (`pt`, `en` ou `es`).
pub fn language() -> &'static str {
    match LANGUAGE.read().map_or(Lang::Pt, |l| *l) {
        Lang::Pt => "pt",
        Lang::En => "en",
        Lang::Es => "es",
    }
}

/// A mensagem do aylis no idioma atual (sem tradução, o próprio texto).
pub fn t(text: &str) -> String {
    builtin(text).unwrap_or(text).to_string()
}

/// A tradução do catálogo do aylis, se houver.
pub(crate) fn builtin(text: &str) -> Option<&'static str> {
    let table: &[(&str, &str)] = match LANGUAGE.read().map_or(Lang::Pt, |l| *l) {
        Lang::Pt => return None,
        Lang::En => EN,
        Lang::Es => ES,
    };
    table.iter().find(|(k, _)| *k == text).map(|(_, v)| *v)
}

/// Como [`t`], trocando `{0}`, `{1}`... pelos argumentos.
pub fn tf(template: &str, args: &[&dyn std::fmt::Display]) -> String {
    let mut out = t(template);
    for (i, a) in args.iter().enumerate() {
        out = out.replace(&format!("{{{i}}}"), &a.to_string());
    }
    out
}

// Ordem: diálogos, edição, dados, grid, arquivos e listas, mídia.
const EN: &[(&str, &str)] = &[
    ("Informação", "Information"),
    ("Aviso", "Warning"),
    ("Erro", "Error"),
    ("Confirmação", "Confirmation"),
    ("sem janela para compartilhar", "no window to share from"),
    ("arquivo não encontrado", "file not found"),
    ("OK", "OK"),
    ("Cancelar", "Cancel"),
    ("O aplicativo encontrou um erro e precisa ser recarregado.", "The app ran into an error and needs to be reloaded."),
    ("Recarregar", "Reload"),
    ("Há uma versão nova do aplicativo. Atualize para continuar.", "There is a new version of the app. Update it to continue."),
    ("Aguardando o programa…", "Waiting for the program…"),
    ("O programa não está respondendo. Encerrar o programa? As alterações não salvas serão perdidas.", "The program is not responding. Close the program? Unsaved changes will be lost."),
    ("O servidor do aplicativo é de uma versão anterior. Tente de novo mais tarde.", "The app server is from an older version. Try again later."),
    ("Carregando", "Loading"),
    ("Sim", "Yes"),
    ("(sem sugestões)", "(no suggestions)"),
    ("Adicionar ao dicionário", "Add to dictionary"),
    ("Não", "No"),
    ("Abortar", "Abort"),
    ("Repetir", "Retry"),
    ("Ignorar", "Ignore"),
    ("Fechar", "Close"),
    ("Desfazer", "Undo"),
    ("Obrigatório", "Required"),
    ("E-mail inválido", "Invalid e-mail"),
    ("Mínimo de {0} caracteres", "At least {0} characters"),
    ("Máximo de {0} caracteres", "At most {0} characters"),
    ("Digite um número", "Enter a number"),
    ("CPF inválido", "Invalid CPF"),
    ("Inválido", "Invalid"),
    ("Sim;Não", "Yes;No"),
    ("{0} é obrigatório", "{0} is required"),
    ("{0} é somente leitura", "{0} is read-only"),
    ("{0}: no máximo {1} caracteres", "{0}: at most {1} characters"),
    ("{0}: o mínimo é {1}", "{0}: the minimum is {1}"),
    ("{0}: o máximo é {1}", "{0}: the maximum is {1}"),
    ("'{0}' não é um número inteiro válido para {1}", "'{0}' is not a valid whole number for {1}"),
    ("'{0}' não é um número válido para {1}", "'{0}' is not a valid number for {1}"),
    ("'{0}' não é um valor válido para {1}", "'{0}' is not a valid value for {1}"),
    ("{0}: o valor {1} não cabe", "{0}: the value {1} does not fit"),
    ("'{0}' não é {1} nem {2}", "'{0}' is neither {1} nor {2}"),
    ("'{0}' não é uma data válida ({1})", "'{0}' is not a valid date ({1})"),
    ("'{0}' não é uma hora válida ({1})", "'{0}' is not a valid time ({1})"),
    ("'{0}' não é uma data/hora válida ({1})", "'{0}' is not a valid date/time ({1})"),
    ("{0} não pode ser digitado", "{0} cannot be typed"),
    ("{0} incompleto: '{1}'", "{0} incomplete: '{1}'"),
    ("o dataset é somente leitura", "the dataset is read-only"),
    ("não há registro para excluir", "there is no record to delete"),
    ("Sem registros", "No records"),
    ("filtrar…", "filter…"),
    ("Filtrar {0}", "Filter {0}"),
    ("Buscar: {0}", "Search: {0}"),
    ("Escolher arquivo…", "Choose a file…"),
    ("Arraste arquivos para cá ou clique para escolher", "Drag files here or click to choose"),
    ("Digite para buscar…", "Type to search…"),
    ("Digite um comando…", "Type a command…"),
    ("Nenhum resultado", "No results"),
    ("arquivo não encontrado: {0}", "file not found: {0}"),
    ("o arquivo não tem vídeo", "the file has no video"),
    ("formato de vídeo não suportado pelo sistema ({0})", "video format not supported by the system ({0})"),
    ("o arquivo não tem trilha de áudio", "the file has no audio track"),
    ("nenhuma saída de áudio", "no audio output"),
    ("nenhum microfone", "no microphone"),
    ("nenhuma câmera", "no camera"),    ("Resultados", "Results"),
    ("Recentes", "Recent"),
    ("Buscar emoji", "Search emoji"),
    ("Carinhas e emoções", "Smileys & emotion"),
    ("Pessoas", "People"),
    ("Animais e natureza", "Animals & nature"),
    ("Comidas e bebidas", "Food & drink"),
    ("Viagens e lugares", "Travel & places"),
    ("Atividades", "Activities"),
    ("Objetos", "Objects"),
    ("Símbolos", "Symbols"),
    ("Bandeiras", "Flags"),
    ("este aparelho não tem NFC", "this device has no NFC"),
    ("nada para gravar", "nothing to write"),
    ("informe o componente", "specify the component"),
    ("informe o índice do item", "specify the item index"),
    ("Espaço", "Space"),
    ("este aparelho não tem Bluetooth", "this device has no Bluetooth"),
    ("o Bluetooth clássico não está disponível aqui", "classic Bluetooth is not available here"),
    ("as notificações push não existem aqui", "push notifications are not available here"),
    ("Web Push: preencha vapid_key (a chave pública VAPID do servidor)", "Web Push: fill in vapid_key (the server's public VAPID key)"),
    ("vapid_key não é base64url válido", "vapid_key is not valid base64url"),
    ("a permissão de notificar foi negada", "the notification permission was denied"),
    ("a página não tem service worker (publique para a web e sirva em https)", "the page has no service worker (publish for the web and serve it over https)"),
    ("o WNS só registra o app empacotado (MSIX)", "WNS only registers packaged apps (MSIX)"),
    ("nenhum distribuidor do UnifiedPush instalado (ntfy, KUnifiedPush...)", "no UnifiedPush distributor installed (ntfy, KUnifiedPush...)"),
    ("as notificações não existem aqui", "notifications are not available here"),
    ("Avisos", "Alerts"),
    ("Copiar", "Copy"),
    ("Copiado", "Copied"),
    ("Ordenado só entre os {0} registros carregados: há mais no servidor", "Sorted only among the {0} loaded records: there are more on the server"),
    ("Filtrado só entre os {0} registros carregados: há mais no servidor", "Filtered only among the {0} loaded records: there are more on the server"),
    ("Procurado só entre os {0} registros carregados: há mais no servidor", "Searched only among the {0} loaded records: there are more on the server"),
    ("Mostrando os primeiros {0} registros (limite de memória do dataset)", "Showing the first {0} records (the dataset memory limit)"),
    ("informe o componente", "specify the component"),
    ("informe o formato", "specify the format"),
    ("a imagem só é colada num Image", "the image is only pasted into an Image"),
    ("prefixo inválido: {0} (letras, números, + - ., de 3 a 64)", "invalid prefix: {0} (letters, digits, + - ., 3 to 64)"),
    ("o prefixo {0} é do sistema", "the prefix {0} belongs to the system"),
    ("o prefixo {0} está registrado por outro programa", "the prefix {0} is registered by another program"),
    ("registrar um prefixo não existe aqui", "registering a prefix is not available here"),
    // O OleObject (automação OLE).
    ("a automação OLE só existe no Windows", "OLE automation is only available on Windows"),
    ("o OleObject precisa do prog_id", "the OleObject needs a prog_id"),
    ("a automação de '{0}' não foi permitida pelo app", "automating '{0}' was not allowed by the app"),
    ("caminho vazio", "empty path"),
    ("'{0}' não é um objeto", "'{0}' is not an object"),
    ("'{0}' não é um número", "'{0}' is not a number"),
    ("o valor não é um número", "the value is not a number"),
    ("matriz JSON inválida: {0}", "invalid JSON matrix: {0}"),
    ("put_data pede o data_source", "put_data needs the data_source"),
    ("a matriz tem de ser uma lista JSON", "the matrix must be a JSON list"),
    ("a matriz está vazia", "the matrix is empty"),
    ("caminho inválido '{0}': {1}", "invalid path '{0}': {1}"),
    ("falta um nome", "a name is missing"),
    ("falta fechar o parêntese", "a closing parenthesis is missing"),
    ("caractere inesperado '{0}'", "unexpected character '{0}'"),
    ("texto sem as aspas do fim", "text without the closing quote"),
    ("número inválido '{0}'", "invalid number '{0}'"),
    ("argumento: só números, \"textos\", true, false e null", "argument: only numbers, \"texts\", true, false and null"),
    ("falta o argumento", "the argument is missing"),
    ("o programa não respondeu a tempo (há um diálogo aberto nele?)", "the program did not answer in time (is a dialog open in it?)"),
    ("a automação OLE parou", "OLE automation stopped"),
    ("o objeto OLE já foi largado", "the OLE object was already released"),
    ("o programa '{0}' não está instalado", "the program '{0}' is not installed"),
    ("não consegui abrir '{0}': {1}", "could not open '{0}': {1}"),
    ("o objeto não tem '{0}'", "the object has no '{0}'"),
    ("'{0}' falhou: {1}", "'{0}' failed: {1}"),
    ("sem memória para a matriz", "out of memory for the matrix"),
    // O WmiQuery (o WMI do Windows).
    ("namespace do WMI inválido '{0}' (só desta máquina, como root\\cimv2)", "invalid WMI namespace '{0}' (only this machine, like root\\cimv2)"),
    ("o WMI parou", "WMI stopped"),
    ("não consegui conectar ao WMI ({0}): {1}", "could not connect to WMI ({0}): {1}"),
    ("a consulta do WMI falhou ({0}): {1}", "the WMI query failed ({0}): {1}"),
    ("consulta inválida", "invalid query"),
    ("classe inválida", "invalid class"),
    ("namespace inválido", "invalid namespace"),
    ("acesso negado", "access denied"),
    ("não encontrado", "not found"),
    ("o WMI só existe no Windows", "WMI is only available on Windows"),
    ("o WmiQuery precisa da consulta", "the WmiQuery needs a query"),
    ("o WMI não respondeu a tempo", "WMI did not answer in time"),
    ("o data_set do WmiQuery tem de ser um ClientDataSet", "the WmiQuery data_set must be a ClientDataSet"),
];

const ES: &[(&str, &str)] = &[
    ("Informação", "Información"),
    ("Aviso", "Aviso"),
    ("Erro", "Error"),
    ("Confirmação", "Confirmación"),
    ("sem janela para compartilhar", "no hay ventana para compartir"),
    ("arquivo não encontrado", "archivo no encontrado"),
    ("OK", "Aceptar"),
    ("Cancelar", "Cancelar"),
    ("O aplicativo encontrou um erro e precisa ser recarregado.", "La aplicación encontró un error y necesita recargarse."),
    ("Recarregar", "Recargar"),
    ("Há uma versão nova do aplicativo. Atualize para continuar.", "Hay una nueva versión de la aplicación. Actualícela para continuar."),
    ("Aguardando o programa…", "Esperando al programa…"),
    ("O programa não está respondendo. Encerrar o programa? As alterações não salvas serão perdidas.", "El programa no responde. ¿Cerrar el programa? Se perderán los cambios no guardados."),
    ("O servidor do aplicativo é de uma versão anterior. Tente de novo mais tarde.", "El servidor de la aplicación es de una versión anterior. Inténtelo de nuevo más tarde."),
    ("Carregando", "Cargando"),
    ("Sim", "Sí"),
    ("(sem sugestões)", "(sin sugerencias)"),
    ("Adicionar ao dicionário", "Agregar al diccionario"),
    ("Não", "No"),
    ("Abortar", "Abortar"),
    ("Repetir", "Reintentar"),
    ("Ignorar", "Omitir"),
    ("Fechar", "Cerrar"),
    ("Desfazer", "Deshacer"),
    ("Obrigatório", "Obligatorio"),
    ("E-mail inválido", "Correo no válido"),
    ("Mínimo de {0} caracteres", "Mínimo {0} caracteres"),
    ("Máximo de {0} caracteres", "Máximo {0} caracteres"),
    ("Digite um número", "Escriba un número"),
    ("CPF inválido", "CPF no válido"),
    ("Inválido", "No válido"),
    ("Sim;Não", "Sí;No"),
    ("{0} é obrigatório", "{0} es obligatorio"),
    ("{0} é somente leitura", "{0} es de solo lectura"),
    ("{0}: no máximo {1} caracteres", "{0}: como máximo {1} caracteres"),
    ("{0}: o mínimo é {1}", "{0}: el mínimo es {1}"),
    ("{0}: o máximo é {1}", "{0}: el máximo es {1}"),
    ("'{0}' não é um número inteiro válido para {1}", "'{0}' no es un número entero válido para {1}"),
    ("'{0}' não é um número válido para {1}", "'{0}' no es un número válido para {1}"),
    ("'{0}' não é um valor válido para {1}", "'{0}' no es un valor válido para {1}"),
    ("{0}: o valor {1} não cabe", "{0}: el valor {1} no cabe"),
    ("'{0}' não é {1} nem {2}", "'{0}' no es {1} ni {2}"),
    ("'{0}' não é uma data válida ({1})", "'{0}' no es una fecha válida ({1})"),
    ("'{0}' não é uma hora válida ({1})", "'{0}' no es una hora válida ({1})"),
    ("'{0}' não é uma data/hora válida ({1})", "'{0}' no es una fecha/hora válida ({1})"),
    ("{0} não pode ser digitado", "{0} no se puede escribir"),
    ("{0} incompleto: '{1}'", "{0} incompleto: '{1}'"),
    ("o dataset é somente leitura", "el dataset es de solo lectura"),
    ("não há registro para excluir", "no hay registro para eliminar"),
    ("Sem registros", "Sin registros"),
    ("filtrar…", "filtrar…"),
    ("Filtrar {0}", "Filtrar {0}"),
    ("Buscar: {0}", "Buscar: {0}"),
    ("Escolher arquivo…", "Elegir archivo…"),
    ("Arraste arquivos para cá ou clique para escolher", "Arrastre archivos aquí o haga clic para elegir"),
    ("Digite para buscar…", "Escriba para buscar…"),
    ("Digite um comando…", "Escriba un comando…"),
    ("Nenhum resultado", "Sin resultados"),
    ("arquivo não encontrado: {0}", "archivo no encontrado: {0}"),
    ("o arquivo não tem vídeo", "el archivo no tiene vídeo"),
    ("formato de vídeo não suportado pelo sistema ({0})", "formato de vídeo no compatible con el sistema ({0})"),
    ("o arquivo não tem trilha de áudio", "el archivo no tiene pista de audio"),
    ("nenhuma saída de áudio", "ninguna salida de audio"),
    ("nenhum microfone", "ningún micrófono"),
    ("nenhuma câmera", "ninguna cámara"),    ("Resultados", "Resultados"),
    ("Recentes", "Recientes"),
    ("Buscar emoji", "Buscar emoji"),
    ("Carinhas e emoções", "Caras y emociones"),
    ("Pessoas", "Personas"),
    ("Animais e natureza", "Animales y naturaleza"),
    ("Comidas e bebidas", "Comida y bebida"),
    ("Viagens e lugares", "Viajes y lugares"),
    ("Atividades", "Actividades"),
    ("Objetos", "Objetos"),
    ("Símbolos", "Símbolos"),
    ("Bandeiras", "Banderas"),
    ("este aparelho não tem NFC", "este dispositivo no tiene NFC"),
    ("nada para gravar", "nada para grabar"),
    ("informe o componente", "indique el componente"),
    ("informe o índice do item", "indique el índice del elemento"),
    ("Espaço", "Espacio"),
    ("este aparelho não tem Bluetooth", "este dispositivo no tiene Bluetooth"),
    ("o Bluetooth clássico não está disponível aqui", "el Bluetooth clásico no está disponible aquí"),
    ("as notificações push não existem aqui", "las notificaciones push no están disponibles aquí"),
    ("Web Push: preencha vapid_key (a chave pública VAPID do servidor)", "Web Push: complete vapid_key (la clave pública VAPID del servidor)"),
    ("vapid_key não é base64url válido", "vapid_key no es base64url válido"),
    ("a permissão de notificar foi negada", "se denegó el permiso de notificaciones"),
    ("a página não tem service worker (publique para a web e sirva em https)", "la página no tiene service worker (publíquela para la web y sírvala por https)"),
    ("o WNS só registra o app empacotado (MSIX)", "WNS solo registra la app empaquetada (MSIX)"),
    ("nenhum distribuidor do UnifiedPush instalado (ntfy, KUnifiedPush...)", "ningún distribuidor de UnifiedPush instalado (ntfy, KUnifiedPush...)"),
    ("as notificações não existem aqui", "las notificaciones no están disponibles aquí"),
    ("Avisos", "Avisos"),
    ("Copiar", "Copiar"),
    ("Copiado", "Copiado"),
    ("Ordenado só entre os {0} registros carregados: há mais no servidor", "Ordenado solo entre los {0} registros cargados: hay más en el servidor"),
    ("Filtrado só entre os {0} registros carregados: há mais no servidor", "Filtrado solo entre los {0} registros cargados: hay más en el servidor"),
    ("Procurado só entre os {0} registros carregados: há mais no servidor", "Buscado solo entre los {0} registros cargados: hay más en el servidor"),
    ("Mostrando os primeiros {0} registros (limite de memória do dataset)", "Mostrando los primeros {0} registros (el límite de memoria del dataset)"),
    ("informe o componente", "indique el componente"),
    ("informe o formato", "indique el formato"),
    ("a imagem só é colada num Image", "la imagen solo se pega en un Image"),
    ("prefixo inválido: {0} (letras, números, + - ., de 3 a 64)", "prefijo inválido: {0} (letras, números, + - ., de 3 a 64)"),
    ("o prefixo {0} é do sistema", "el prefijo {0} es del sistema"),
    ("o prefixo {0} está registrado por outro programa", "el prefijo {0} está registrado por otro programa"),
    ("registrar um prefixo não existe aqui", "registrar un prefijo no existe aquí"),
    // O OleObject (automação OLE).
    ("a automação OLE só existe no Windows", "la automatización OLE solo existe en Windows"),
    ("o OleObject precisa do prog_id", "el OleObject necesita el prog_id"),
    ("a automação de '{0}' não foi permitida pelo app", "la automatización de '{0}' no fue permitida por la app"),
    ("caminho vazio", "ruta vacía"),
    ("'{0}' não é um objeto", "'{0}' no es un objeto"),
    ("'{0}' não é um número", "'{0}' no es un número"),
    ("o valor não é um número", "el valor no es un número"),
    ("matriz JSON inválida: {0}", "matriz JSON inválida: {0}"),
    ("put_data pede o data_source", "put_data necesita el data_source"),
    ("a matriz tem de ser uma lista JSON", "la matriz debe ser una lista JSON"),
    ("a matriz está vazia", "la matriz está vacía"),
    ("caminho inválido '{0}': {1}", "ruta inválida '{0}': {1}"),
    ("falta um nome", "falta un nombre"),
    ("falta fechar o parêntese", "falta cerrar el paréntesis"),
    ("caractere inesperado '{0}'", "carácter inesperado '{0}'"),
    ("texto sem as aspas do fim", "texto sin las comillas de cierre"),
    ("número inválido '{0}'", "número inválido '{0}'"),
    ("argumento: só números, \"textos\", true, false e null", "argumento: solo números, \"textos\", true, false y null"),
    ("falta o argumento", "falta el argumento"),
    ("o programa não respondeu a tempo (há um diálogo aberto nele?)", "el programa no respondió a tiempo (¿hay un diálogo abierto en él?)"),
    ("a automação OLE parou", "la automatización OLE se detuvo"),
    ("o objeto OLE já foi largado", "el objeto OLE ya fue liberado"),
    ("o programa '{0}' não está instalado", "el programa '{0}' no está instalado"),
    ("não consegui abrir '{0}': {1}", "no pude abrir '{0}': {1}"),
    ("o objeto não tem '{0}'", "el objeto no tiene '{0}'"),
    ("'{0}' falhou: {1}", "'{0}' falló: {1}"),
    ("sem memória para a matriz", "sin memoria para la matriz"),
    // O WmiQuery (o WMI do Windows).
    ("namespace do WMI inválido '{0}' (só desta máquina, como root\\cimv2)", "namespace de WMI inválido '{0}' (solo de esta máquina, como root\\cimv2)"),
    ("o WMI parou", "WMI se detuvo"),
    ("não consegui conectar ao WMI ({0}): {1}", "no pude conectar a WMI ({0}): {1}"),
    ("a consulta do WMI falhou ({0}): {1}", "la consulta de WMI falló ({0}): {1}"),
    ("consulta inválida", "consulta inválida"),
    ("classe inválida", "clase inválida"),
    ("namespace inválido", "namespace inválido"),
    ("acesso negado", "acceso denegado"),
    ("não encontrado", "no encontrado"),
    ("o WMI só existe no Windows", "WMI solo existe en Windows"),
    ("o WmiQuery precisa da consulta", "el WmiQuery necesita la consulta"),
    ("o WMI não respondeu a tempo", "WMI no respondió a tiempo"),
    ("o data_set do WmiQuery tem de ser um ClientDataSet", "el data_set del WmiQuery debe ser un ClientDataSet"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_has_the_same_keys() {
        fn keys(t: &[(&'static str, &'static str)]) -> Vec<&'static str> {
            t.iter().map(|(k, _)| *k).collect()
        }
        assert_eq!(keys(EN), keys(ES));
        // Os marcadores {0}, {1}... da chave estão na tradução.
        for (k, v) in EN.iter().chain(ES) {
            for i in 0..3 {
                let m = format!("{{{i}}}");
                assert_eq!(k.contains(&m), v.contains(&m), "{k} → {v}");
            }
        }
    }
}
