//! Galeria: o ponto de entrada do desktop. Gerado pela IDE do aylis (pode editar).
// Sem console no Windows quando publicado (em desenvolvimento, a saída vai para a IDE).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> Result<(), aylis::Error> {
    galeria::app()?.run()
}
