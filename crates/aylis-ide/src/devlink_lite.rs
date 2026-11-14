//! O canal de desenvolvimento na IDE lite do navegador: não há app em execução para
//! conectar (ela não compila nem executa). A mesma API do `devlink.rs` do desktop, sempre
//! "nenhum app conectado".

use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

const NO_APP: &str = "o app não está conectado (a IDE no navegador não executa o projeto)";

/// O servidor que nunca abre.
pub struct DevServer {
    address: String,
}

pub fn for_project(_existing: Option<Arc<DevServer>>, _project: &aylis_project::Project) -> Result<Arc<DevServer>, String> {
    Err(NO_APP.into())
}

impl DevServer {
    pub fn start(address: &str, _private: [u8; 32], _secret: [u8; 32]) -> Result<Arc<DevServer>, String> {
        let _ = DevServer { address: address.to_string() };
        Err(NO_APP.into())
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn set_trace(&self, _on: bool) {}

    pub fn port(&self) -> u16 {
        0
    }

    pub fn url(&self) -> String {
        String::new()
    }

    pub fn set_secret(&self, _secret: [u8; 32]) {}

    pub fn connected(&self) -> bool {
        false
    }

    pub fn apps(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn wait(&self, _timeout: Duration) -> bool {
        false
    }

    pub fn take_traces(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn take_handlers(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn call(&self, _command: Value) -> Result<Value, String> {
        Err(NO_APP.into())
    }

    pub fn call_app(&self, _app: Option<&Value>, _command: Value) -> Result<Value, String> {
        Err(NO_APP.into())
    }

    pub fn apps_json(&self) -> Value {
        Value::Array(Vec::new())
    }

    pub fn reload(&self, _kind: &str, _name: &str, _text: &str) -> Result<String, String> {
        Err(NO_APP.into())
    }
}
