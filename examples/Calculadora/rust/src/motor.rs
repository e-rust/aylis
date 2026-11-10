//! As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
//! modo padrão.
//!
//! - `2 + 3 = = =` repete a última operação (8, 11...).
//! - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
//!   (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
//! - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
//! - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
//! - Dividir por zero mostra a mensagem e trava até limpar ou digitar.

/// Uma operação de dois números.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Somar,
    Subtrair,
    Multiplicar,
    Dividir,
}

impl Op {
    pub fn simbolo(self) -> &'static str {
        match self {
            Op::Somar => "+",
            Op::Subtrair => "−",
            Op::Multiplicar => "×",
            Op::Dividir => "÷",
        }
    }

    fn aplicar(self, a: f64, b: f64) -> Result<f64, &'static str> {
        match self {
            Op::Somar => Ok(a + b),
            Op::Subtrair => Ok(a - b),
            Op::Multiplicar => Ok(a * b),
            Op::Dividir if b == 0.0 => Err(if a == 0.0 { "Resultado indefinido" } else { "Não é possível dividir por zero" }),
            Op::Dividir => Ok(a / b),
        }
    }
}

/// Uma operação de um número só.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Funcao {
    Inverso,
    Quadrado,
    Raiz,
}

/// Uma conta do histórico: a expressão (`"12 + 30 ="`) e o resultado.
#[derive(Clone, Debug, PartialEq)]
pub struct Conta {
    pub expressao: String,
    pub resultado: f64,
}

/// O estado da calculadora.
#[derive(Clone, Debug, Default)]
pub struct Calculadora {
    /// O número sendo digitado, como texto (`"12,"`, `"-0,05"`).
    entrada: String,
    /// O primeiro número da operação pendente.
    acumulado: Option<f64>,
    pendente: Option<Op>,
    /// A última operação e o segundo número, para o `=` repetir.
    repetir: Option<(Op, f64)>,
    /// O próximo dígito começa um número novo.
    novo: bool,
    /// O número do visor veio de uma função (`sqr(3)`): o texto dela na expressão.
    funcao_texto: Option<String>,
    expressao: String,
    erro: Option<&'static str>,
    pub memoria: Vec<f64>,
    pub historico: Vec<Conta>,
}

/// O número como a calculadora mostra: vírgula decimal, ponto nos milhares, até 16
/// dígitos significativos e notação científica nos muito grandes ou pequenos.
pub fn formatar(v: f64) -> String {
    if v.is_nan() || v.is_infinite() {
        return "Entrada inválida".into();
    }
    if v == 0.0 {
        return "0".into();
    }
    let abs = v.abs();
    if !(1e-16..1e16).contains(&abs) {
        let s = format!("{v:.15e}");
        let (mantissa, exp) = s.split_once('e').unwrap_or((&s, "0"));
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.').replace('.', ",");
        let exp: i32 = exp.parse().unwrap_or(0);
        return format!("{mantissa}e{}{}", if exp >= 0 { "+" } else { "" }, exp);
    }
    // 16 dígitos significativos, sem zeros à direita. A ordem de grandeza sai contando os
    // dígitos (só contas do IEEE, iguais em qualquer linguagem; o log10 varia um tico).
    let (mut e, mut t) = (0i32, abs);
    while t >= 10.0 {
        t /= 10.0;
        e += 1;
    }
    while t < 1.0 {
        t *= 10.0;
        e -= 1;
    }
    let casas = (15 - e).clamp(0, 16) as usize;
    let s = format!("{abs:.casas$}");
    let s = if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s };
    let (inteira, decimal) = s.split_once('.').map_or((s.as_str(), None), |(i, d)| (i, Some(d)));
    let mut out = agrupar(inteira);
    if let Some(d) = decimal {
        out.push(',');
        out.push_str(d);
    }
    if v < 0.0 {
        out.insert(0, '-');
    }
    out
}

/// `1234567` → `1.234.567`.
fn agrupar(digitos: &str) -> String {
    let mut out = String::new();
    for (i, c) in digitos.chars().enumerate() {
        if i > 0 && (digitos.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(c);
    }
    out
}

impl Calculadora {
    pub fn new() -> Self {
        Self::default()
    }

    /// O texto do visor grande.
    pub fn visor(&self) -> String {
        if let Some(e) = self.erro {
            return e.to_string();
        }
        if self.entrada.is_empty() {
            return "0".into();
        }
        if self.novo {
            // Um resultado: formatado inteiro (notação científica nos enormes).
            return formatar(self.valor());
        }
        // O que está sendo digitado, com os milhares agrupados e a vírgula como veio.
        let (sinal, resto) = self.entrada.strip_prefix('-').map_or(("", self.entrada.as_str()), |r| ("-", r));
        let (inteira, decimal) = resto.split_once(',').map_or((resto, None), |(i, d)| (i, Some(d)));
        let mut out = format!("{sinal}{}", agrupar(inteira));
        if let Some(d) = decimal {
            out.push(',');
            out.push_str(d);
        }
        out
    }

    /// A linha de cima (a conta em andamento).
    pub fn expressao(&self) -> &str {
        &self.expressao
    }

    pub fn tem_erro(&self) -> bool {
        self.erro.is_some()
    }

    /// O número do visor.
    pub fn valor(&self) -> f64 {
        self.entrada.replace(',', ".").parse().unwrap_or(0.0)
    }

    fn mostrar(&mut self, v: f64) {
        // Guarda o número inteiro (sem o agrupamento), para continuar a conta.
        self.entrada = if v == 0.0 { String::new() } else { formatar(v).replace('.', "") };
        if v.abs() >= 1e16 || (v != 0.0 && v.abs() < 1e-16) {
            // Notação científica: guarda o valor exato (17 dígitos voltam ao mesmo número).
            self.entrada = format!("{v:.16e}").replace('.', ",");
        }
    }

    fn sair_do_erro(&mut self) {
        if self.erro.is_some() {
            *self = Calculadora { memoria: std::mem::take(&mut self.memoria), historico: std::mem::take(&mut self.historico), ..Default::default() };
        }
    }

    /// Um dígito de 0 a 9.
    pub fn digito(&mut self, d: u8) {
        self.sair_do_erro();
        if self.novo {
            self.entrada.clear();
            self.novo = false;
            self.funcao_texto = None;
            if self.pendente.is_none() {
                self.expressao.clear();
            }
        }
        let digitos = self.entrada.chars().filter(char::is_ascii_digit).count();
        if digitos >= 16 {
            return;
        }
        if self.entrada == "0" || self.entrada == "-0" {
            self.entrada.pop();
        }
        self.entrada.push((b'0' + d.min(9)) as char);
    }

    /// A vírgula decimal.
    pub fn virgula(&mut self) {
        self.sair_do_erro();
        if self.novo {
            self.entrada.clear();
            self.novo = false;
            self.funcao_texto = None;
        }
        if !self.entrada.contains(',') {
            if self.entrada.is_empty() || self.entrada == "-" {
                self.entrada.push('0');
            }
            self.entrada.push(',');
        }
    }

    /// ⌫: apaga o último dígito do número sendo digitado.
    pub fn apagar(&mut self) {
        if self.erro.is_some() {
            self.sair_do_erro();
            return;
        }
        if self.novo {
            // Resultado: o Windows não apaga, só limpa a expressão terminada.
            if self.pendente.is_none() {
                self.expressao.clear();
            }
            return;
        }
        self.entrada.pop();
        if self.entrada == "-" || self.entrada == "-0" {
            self.entrada.clear();
        }
    }

    /// ±
    pub fn inverter_sinal(&mut self) {
        if self.erro.is_some() {
            return;
        }
        if self.entrada.is_empty() {
            return;
        }
        if let Some(r) = self.entrada.strip_prefix('-') {
            self.entrada = r.to_string();
        } else {
            self.entrada.insert(0, '-');
        }
        if self.novo {
            // Inverter um resultado vira `negate(x)` na expressão.
            let v = self.valor();
            let texto = format!("negate({})", self.funcao_texto.take().unwrap_or_else(|| formatar(-v)));
            self.funcao_texto = Some(texto);
            self.atualizar_expressao_funcao();
        }
    }

    /// CE: limpa o número do visor (a conta continua).
    pub fn limpar_entrada(&mut self) {
        if self.erro.is_some() {
            self.sair_do_erro();
            return;
        }
        self.entrada.clear();
        self.funcao_texto = None;
        if self.pendente.is_none() {
            self.expressao.clear();
        } else if let (Some(a), Some(op)) = (self.acumulado, self.pendente) {
            self.expressao = format!("{} {}", formatar(a), op.simbolo());
        }
        self.novo = false;
    }

    /// C: limpa tudo (menos a memória e o histórico).
    pub fn limpar(&mut self) {
        let memoria = std::mem::take(&mut self.memoria);
        let historico = std::mem::take(&mut self.historico);
        *self = Calculadora { memoria, historico, ..Default::default() };
    }

    /// `+`, `−`, `×` ou `÷`.
    pub fn operacao(&mut self, op: Op) {
        if self.erro.is_some() {
            return;
        }
        let v = self.valor();
        match (self.acumulado, self.pendente) {
            // Trocou a operação antes de digitar o segundo número.
            (Some(a), Some(_)) if self.novo && self.funcao_texto.is_none() => {
                self.pendente = Some(op);
                self.expressao = format!("{} {}", formatar(a), op.simbolo());
                return;
            }
            // Encadeado: 2 + 3 + → mostra 5.
            (Some(a), Some(anterior)) => match anterior.aplicar(a, v) {
                Ok(r) => {
                    self.acumulado = Some(r);
                    self.mostrar(r);
                }
                Err(e) => {
                    self.expressao = format!("{} {} {} {}", formatar(a), anterior.simbolo(), self.texto_do_numero(v), op.simbolo());
                    self.erro = Some(e);
                    return;
                }
            },
            _ => self.acumulado = Some(v),
        }
        self.pendente = Some(op);
        self.repetir = None;
        self.novo = true;
        self.funcao_texto = None;
        self.expressao = format!("{} {}", formatar(self.acumulado.unwrap_or(0.0)), op.simbolo());
    }

    fn texto_do_numero(&self, v: f64) -> String {
        self.funcao_texto.clone().unwrap_or_else(|| formatar(v))
    }

    /// `=`: termina a conta (e repete a última, se apertado de novo).
    pub fn igual(&mut self) {
        if self.erro.is_some() {
            self.sair_do_erro();
            return;
        }
        let v = self.valor();
        let (a, op, b) = match (self.acumulado, self.pendente) {
            (Some(a), Some(op)) => (a, op, v),
            _ => match self.repetir {
                Some((op, b)) => (v, op, b),
                None => {
                    // Só um número: `5 =`.
                    let texto = self.texto_do_numero(v);
                    self.expressao = format!("{texto} =");
                    self.historico.push(Conta { expressao: self.expressao.clone(), resultado: v });
                    self.novo = true;
                    self.funcao_texto = None;
                    return;
                }
            },
        };
        let segundo = if self.pendente.is_some() { self.texto_do_numero(b) } else { formatar(b) };
        self.expressao = format!("{} {} {} =", formatar(a), op.simbolo(), segundo);
        match op.aplicar(a, b) {
            Ok(r) => {
                self.historico.push(Conta { expressao: self.expressao.clone(), resultado: r });
                self.mostrar(r);
                self.repetir = Some((op, b));
            }
            Err(e) => self.erro = Some(e),
        }
        self.acumulado = None;
        self.pendente = None;
        self.novo = true;
        self.funcao_texto = None;
    }

    /// `%`.
    pub fn porcentagem(&mut self) {
        if self.erro.is_some() {
            return;
        }
        let v = self.valor();
        let r = match (self.acumulado, self.pendente) {
            (Some(a), Some(Op::Somar | Op::Subtrair)) => a * v / 100.0,
            (Some(_), Some(_)) => v / 100.0,
            // Sem conta, o Windows zera.
            _ => 0.0,
        };
        self.mostrar(r);
        self.funcao_texto = Some(formatar(r));
        self.novo = true;
        self.atualizar_expressao_funcao();
    }

    /// `1/x`, `x²` ou `√x`.
    pub fn funcao(&mut self, f: Funcao) {
        if self.erro.is_some() {
            return;
        }
        let v = self.valor();
        let dentro = self.texto_do_numero(v);
        let (r, texto) = match f {
            Funcao::Inverso => (if v == 0.0 { Err("Não é possível dividir por zero") } else { Ok(1.0 / v) }, format!("1/({dentro})")),
            Funcao::Quadrado => (Ok(v * v), format!("sqr({dentro})")),
            Funcao::Raiz => (if v < 0.0 { Err("Entrada inválida") } else { Ok(v.sqrt()) }, format!("√({dentro})")),
        };
        self.funcao_texto = Some(texto);
        self.atualizar_expressao_funcao();
        match r {
            Ok(r) => self.mostrar(r),
            Err(e) => self.erro = Some(e),
        }
        self.novo = true;
    }

    fn atualizar_expressao_funcao(&mut self) {
        let texto = self.funcao_texto.clone().unwrap_or_default();
        self.expressao = match (self.acumulado, self.pendente) {
            (Some(a), Some(op)) => format!("{} {} {}", formatar(a), op.simbolo(), texto),
            _ => texto,
        };
    }

    /// Usa um resultado do histórico (clicar na conta).
    pub fn usar(&mut self, valor: f64) {
        self.sair_do_erro();
        self.mostrar(valor);
        self.novo = true;
        self.funcao_texto = None;
    }

    // ----- memória -----

    /// MS: guarda o número do visor numa memória nova.
    pub fn memoria_guardar(&mut self) {
        if self.erro.is_none() {
            self.memoria.push(self.valor());
            self.novo = true;
        }
    }

    /// MR: traz a última memória.
    pub fn memoria_ler(&mut self) {
        if let Some(&m) = self.memoria.last() {
            self.usar(m);
        }
    }

    /// M+ (ou M- com `sinal` = -1): soma o visor na última memória (ou cria uma).
    pub fn memoria_somar(&mut self, sinal: f64) {
        if self.erro.is_some() {
            return;
        }
        let v = self.valor() * sinal;
        match self.memoria.last_mut() {
            Some(m) => *m += v,
            None => self.memoria.push(v),
        }
        self.novo = true;
    }

    /// MC: apaga todas as memórias.
    pub fn memoria_limpar(&mut self) {
        self.memoria.clear();
    }

    /// Aperta um botão pelo texto dele (o mesmo da tela e do teclado).
    pub fn tecla(&mut self, t: &str) -> bool {
        match t {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.digito(t.as_bytes()[0] - b'0'),
            "," | "." => self.virgula(),
            "+" => self.operacao(Op::Somar),
            "−" | "-" => self.operacao(Op::Subtrair),
            "×" | "*" => self.operacao(Op::Multiplicar),
            "÷" | "/" => self.operacao(Op::Dividir),
            "=" => self.igual(),
            "%" => self.porcentagem(),
            "⌫" => self.apagar(),
            "CE" => self.limpar_entrada(),
            "C" => self.limpar(),
            "±" => self.inverter_sinal(),
            "¹⁄ₓ" => self.funcao(Funcao::Inverso),
            "x²" => self.funcao(Funcao::Quadrado),
            "²√x" => self.funcao(Funcao::Raiz),
            "MC" => self.memoria_limpar(),
            "MR" => self.memoria_ler(),
            "M+" => self.memoria_somar(1.0),
            "M-" => self.memoria_somar(-1.0),
            "MS" => self.memoria_guardar(),
            _ => return false,
        }
        true
    }
}

/// O botão de uma tecla, como os atalhos da calculadora do Windows. `tecla` é o nome que o
/// aylis dá à tecla (`"7"`, `"+"`, `"enter"`, `"escape"`, `"f9"`...); `"historico"` mostra ou
/// esconde o histórico.
pub fn botao_do_teclado(tecla: &str, ctrl: bool, shift: bool) -> Option<&'static str> {
    if ctrl {
        return match tecla {
            "m" => Some("MS"),
            "r" => Some("MR"),
            "p" => Some("M+"),
            "q" => Some("M-"),
            "l" => Some("MC"),
            "h" => Some("historico"),
            _ => None,
        };
    }
    Some(match tecla {
        "=" if shift => "+",
        "8" if shift => "×",
        "5" if shift => "%",
        "2" if shift => "²√x",
        "0" => "0",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
        "+" => "+",
        "-" => "−",
        "*" => "×",
        "/" => "÷",
        "," | "." => ",",
        "%" => "%",
        "r" => "¹⁄ₓ",
        "q" => "x²",
        "@" => "²√x",
        "=" | "enter" => "=",
        "backspace" => "⌫",
        "delete" => "CE",
        "escape" => "C",
        "f9" => "±",
        _ => return None,
    })
}
