//! As contas da calculadora, como a do Windows.

use calculadora::motor::{Calculadora, botao_do_teclado, formatar};

fn conta(teclas: &[&str]) -> Calculadora {
    let mut c = Calculadora::new();
    for t in teclas {
        assert!(c.tecla(t), "tecla {t}");
    }
    c
}

#[test]
fn contas_basicas_e_encadeadas() {
    assert_eq!(conta(&["1", "2", "+", "3", "0", "="]).visor(), "42");
    assert_eq!(conta(&["1", "2", "+", "3", "0", "="]).expressao(), "12 + 30 =");
    // 2 + 3 × 4: a do Windows (padrão) faz na ordem: (2 + 3) × 4.
    let c = conta(&["2", "+", "3", "×"]);
    assert_eq!(c.visor(), "5");
    assert_eq!(c.expressao(), "5 ×");
    assert_eq!(conta(&["2", "+", "3", "×", "4", "="]).visor(), "20");
    // Trocar a operação antes do segundo número.
    assert_eq!(conta(&["9", "+", "−", "4", "="]).visor(), "5");
    assert_eq!(conta(&["7", "÷", "2", "="]).visor(), "3,5");
    assert_eq!(conta(&["0", ",", "1", "+", "0", ",", "2", "="]).visor(), "0,3");
}

#[test]
fn igual_repetido_porcentagem_e_funcoes() {
    assert_eq!(conta(&["2", "+", "3", "=", "=", "="]).visor(), "11");
    // Em + e −, a porcentagem do primeiro número.
    assert_eq!(conta(&["2", "0", "0", "+", "1", "0", "%", "="]).visor(), "220");
    // Em × e ÷, o número dividido por 100.
    assert_eq!(conta(&["5", "0", "×", "1", "0", "%", "="]).visor(), "5");
    let c = conta(&["9", "²√x"]);
    assert_eq!((c.visor().as_str(), c.expressao()), ("3", "√(9)"));
    let c = conta(&["3", "x²", "x²"]);
    assert_eq!((c.visor().as_str(), c.expressao()), ("81", "sqr(sqr(3))"));
    assert_eq!(conta(&["4", "¹⁄ₓ"]).visor(), "0,25");
    let c = conta(&["1", "0", "+", "4", "²√x", "="]);
    assert_eq!((c.visor().as_str(), c.expressao()), ("12", "10 + √(4) ="));
    assert_eq!(conta(&["5", "±"]).visor(), "-5");
    assert_eq!(conta(&["1", "2", "3", "⌫"]).visor(), "12");
}

#[test]
fn erros_limpar_e_memoria() {
    let mut c = conta(&["5", "÷", "0", "="]);
    assert!(c.tem_erro());
    assert_eq!(c.visor(), "Não é possível dividir por zero");
    c.tecla("7");
    assert!(!c.tem_erro());
    assert_eq!(c.visor(), "7");
    assert_eq!(conta(&["1", "²√x", "±", "²√x"]).visor(), "Entrada inválida");

    let mut c = conta(&["8", "+", "2", "CE", "5", "="]);
    assert_eq!(c.visor(), "13");
    c.tecla("C");
    assert_eq!((c.visor().as_str(), c.expressao()), ("0", ""));
    assert_eq!(c.historico.len(), 1, "o C não apaga o histórico");

    let mut c = conta(&["1", "0", "MS", "5", "M+", "2", "M-"]);
    assert_eq!(c.memoria, vec![13.0]);
    c.tecla("C");
    c.tecla("MR");
    assert_eq!(c.visor(), "13");
    c.tecla("MC");
    assert!(c.memoria.is_empty());
}

#[test]
fn numeros_formatados() {
    assert_eq!(formatar(1234567.0), "1.234.567");
    assert_eq!(formatar(-1234.5), "-1.234,5");
    assert_eq!(formatar(1.0 / 3.0), "0,3333333333333333");
    assert_eq!(formatar(1e20), "1e+20");
    assert_eq!(conta(&["1", "2", "3", "4", ",", "5"]).visor(), "1.234,5");
    // Um bilhão ao quadrado vai para a notação científica e continua fazendo conta.
    let bilhao = ["1", "0", "0", "0", "0", "0", "0", "0", "0", "0"];
    let mut c = Calculadora::new();
    for t in bilhao.iter().chain(&["×"]).chain(bilhao.iter()).chain(&["="]) {
        c.tecla(t);
    }
    assert_eq!(c.visor(), "1e+18");
    for t in ["÷"].iter().chain(bilhao.iter()).chain(&["="]) {
        c.tecla(t);
    }
    assert_eq!(c.visor(), "1.000.000.000");
}


#[test]
fn teclado() {
    assert_eq!(botao_do_teclado("7", false, false), Some("7"));
    assert_eq!(botao_do_teclado("=", false, true), Some("+"));
    assert_eq!(botao_do_teclado("8", false, true), Some("×"));
    assert_eq!(botao_do_teclado("enter", false, false), Some("="));
    assert_eq!(botao_do_teclado("h", true, false), Some("historico"));
    assert_eq!(botao_do_teclado("m", true, false), Some("MS"));
    assert_eq!(botao_do_teclado("x", false, false), None);
}
