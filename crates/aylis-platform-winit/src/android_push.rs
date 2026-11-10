//! Android: notificações push pelo FCM, pela `AylisActivity` (sem o SDK do Firebase: a
//! instalação no Firebase Installations e o token pelo Google Play Services). O Java guarda
//! os eventos numa fila e o aylis consulta (`pushPoll`) a cada timer do componente.

use aylis_core::platform::{ActionEvent, ActionService, PushConfig, PushEvent, PushService, parse_action_line, parse_push_line};
use jni::objects::{JString, JValue};

use super::android_jni::with_env;
use super::android_radio::{call, flag, text};

pub(crate) struct AndroidPush;

impl PushService for AndroidPush {
    fn available(&self) -> bool {
        flag("pushAvailable")
    }

    fn provider(&self) -> &'static str {
        "fcm"
    }

    fn register(&mut self, config: &PushConfig) {
        with_env(|env, a| {
            let s = env.new_string(&config.sender_id)?;
            let p = env.new_string(&config.project_id)?;
            let i = env.new_string(&config.app_id)?;
            let k = env.new_string(&config.api_key)?;
            env.call_method(
                a,
                "pushRegister",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V",
                &[(&s).into(), (&p).into(), (&i).into(), (&k).into(), JValue::Bool(config.show_in_foreground.into())],
            )
            .map(|_| ())
        });
    }

    fn unregister(&mut self) {
        with_env(|env, a| env.call_method(a, "pushUnregister", "()V", &[]).map(|_| ()));
    }

    fn poll(&mut self) -> Vec<PushEvent> {
        text("pushPoll").lines().filter_map(parse_push_line).collect()
    }
}

/// As ações do aparelho e as regras do `PushTrigger`, executadas pelo Java (também com o app
/// fechado: o receiver do push aplica as regras gravadas).
pub(crate) struct AndroidActions;

impl ActionService for AndroidActions {
    fn execute(&mut self, spec: &str, data: &str) -> Result<(), String> {
        let error = with_env(|env, a| {
            let s = env.new_string(spec)?;
            let d = env.new_string(data)?;
            let r: JString = env.call_method(a, "actionExecute", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", &[(&s).into(), (&d).into()])?.l()?.into();
            Ok(env.get_string(&r).map(String::from).unwrap_or_default())
        })
        .unwrap_or_else(|| "a AylisActivity não respondeu".into());
        if error.is_empty() { Ok(()) } else { Err(error) }
    }

    fn stop(&mut self, component: &str) {
        call("actionStop", &[component], None);
    }

    fn set_push_rules(&mut self, rules: &str) {
        call("actionsRules", &[rules], None);
    }

    fn poll(&mut self) -> Vec<ActionEvent> {
        text("actionsPoll").lines().filter_map(parse_action_line).collect()
    }
}
