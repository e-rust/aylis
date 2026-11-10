//! ERP: o ponto de entrada do desktop. Gerado pela IDE do Aylis (pode editar).
// Sem console no Windows quando publicado (em desenvolvimento, a saída vai para a IDE).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> Result<(), aylis::Error> {
    erp::app()?.run()
}
