{ As contas da calculadora, sem tela: o mesmo comportamento da Calculadora do Windows no
  modo padrão (e o mesmo da versão em Rust, examples/Calculadora/rust/src/motor.rs).

  - `2 + 3 = = =` repete a última operação (8, 11...).
  - `%` depende da operação: em `+` e `-` é a porcentagem do primeiro número
    (`200 + 10 %` = 200 + 20); em `×` e `÷`, o número dividido por 100.
  - `1/x`, `x²` e `√x` agem no número do visor e aparecem na expressão (`sqr(3)`).
  - Memória (MC, MR, M+, M-, MS) e histórico das contas feitas com `=`.
  - Dividir por zero mostra a mensagem e trava até limpar ou digitar. }
unit motor;

{$mode objfpc}{$H+}

interface

uses
  SysUtils, Math;

type
  TOp = (opSomar, opSubtrair, opMultiplicar, opDividir);

  TConta = record
    Expressao: string;
    Resultado: Double;
  end;

  TCalculadora = class
  private
    FEntrada: string;
    FTemAcumulado: Boolean;
    FAcumulado: Double;
    FTemPendente: Boolean;
    FPendente: TOp;
    FTemRepetir: Boolean;
    FRepetirOp: TOp;
    FRepetirB: Double;
    FNovo: Boolean;
    FTemFuncao: Boolean;
    FFuncaoTexto: string;
    FErro: string;
    FTemErro: Boolean;
    procedure Zerar;
    procedure Mostrar(V: Double);
    procedure SairDoErro;
    function TextoDoNumero(V: Double): string;
    procedure ExpressaoFuncao;
    procedure Falhar(const E: string);
  public
    Memoria: array of Double;
    Historico: array of TConta;
    Expressao: string;
    function Visor: string;
    function TemErro: Boolean;
    function Valor: Double;
    procedure Digito(D: Integer);
    procedure Virgula;
    procedure Apagar;
    procedure InverterSinal;
    procedure LimparEntrada;
    procedure Limpar;
    procedure Operacao(Op: TOp);
    procedure Igual;
    procedure Porcentagem;
    procedure Funcao(const F: string);
    procedure Usar(AValor: Double);
    procedure MemoriaGuardar;
    procedure MemoriaLer;
    procedure MemoriaSomar(Sinal: Double);
    function Tecla(const T: string): Boolean;
  end;

function Formatar(V: Double): string;
function BotaoDoTeclado(const Tecla: string; Ctrl, Shift: Boolean): string;

implementation

const
  DIV_ZERO = 'Não é possível dividir por zero';
  INVALIDA = 'Entrada inválida';

function Simbolo(Op: TOp): string;
begin
  case Op of
    opSomar: Result := '+';
    opSubtrair: Result := '−';
    opMultiplicar: Result := '×';
  else
    Result := '÷';
  end;
end;

{ O resultado de `a op b`; Erro fica com a mensagem quando não dá. }
function Aplicar(Op: TOp; A, B: Double; out Erro: string): Double;
begin
  Erro := '';
  Result := 0;
  case Op of
    opSomar: Result := A + B;
    opSubtrair: Result := A - B;
    opMultiplicar: Result := A * B;
  else
    if B = 0 then
    begin
      if A = 0 then Erro := 'Resultado indefinido' else Erro := DIV_ZERO;
    end
    else
      Result := A / B;
  end;
end;

{ `1234567` → `1.234.567`. }
function Agrupar(const Digitos: string): string;
var
  I: Integer;
begin
  Result := '';
  for I := 1 to Length(Digitos) do
  begin
    if (I > 1) and ((Length(Digitos) - I + 1) mod 3 = 0) then
      Result := Result + '.';
    Result := Result + Digitos[I];
  end;
end;

function SemZerosNoFim(S: string): string;
begin
  while (Length(S) > 0) and (S[Length(S)] = '0') do
    Delete(S, Length(S), 1);
  if (Length(S) > 0) and (S[Length(S)] = '.') then
    Delete(S, Length(S), 1);
  Result := S;
end;

{ O número como a calculadora mostra: vírgula decimal, ponto nos milhares, até 16 dígitos
  significativos e notação científica nos muito grandes ou pequenos. }
function Formatar(V: Double): string;
var
  A, T: Double;
  E, Casas, P, Expo: Integer;
  S, Mantissa: string;
begin
  if IsNan(V) or IsInfinite(V) then
    Exit(INVALIDA);
  if V = 0 then
    Exit('0');
  A := Abs(V);
  if not ((A >= 1e-16) and (A < 1e16)) then
  begin
    { 16 algarismos: um antes do ponto e 15 depois (o %.15e do C). O Str dá 17
      algarismos; o último arredonda os outros. }
    Str(A, S);
    S := Trim(S);
    P := Pos('E', S);
    Expo := StrToInt(StringReplace(Copy(S, P + 1, 10), '+', '', []));
    Mantissa := StringReplace(Copy(S, 1, P - 1), '.', '', []);
    Casas := 16;
    if Mantissa[17] >= '5' then
    begin
      while (Casas > 0) and (Mantissa[Casas] = '9') do
      begin
        Mantissa[Casas] := '0';
        Dec(Casas);
      end;
      if Casas = 0 then
      begin
        Mantissa := '1' + Mantissa;
        Inc(Expo);
      end
      else
        Mantissa[Casas] := Chr(Ord(Mantissa[Casas]) + 1);
    end;
    Mantissa := SemZerosNoFim(Mantissa[1] + '.' + Copy(Mantissa, 2, 15));
    Mantissa := StringReplace(Mantissa, '.', ',', []);
    if V < 0 then
      Mantissa := '-' + Mantissa;
    if Expo >= 0 then
      Exit(Mantissa + 'e+' + IntToStr(Expo));
    Exit(Mantissa + 'e' + IntToStr(Expo));
  end;
  { A ordem de grandeza contando os dígitos (as mesmas contas em todas as linguagens). }
  E := 0;
  T := A;
  while T >= 10 do
  begin
    T := T / 10;
    Inc(E);
  end;
  while T < 1 do
  begin
    T := T * 10;
    Dec(E);
  end;
  Casas := Min(Max(15 - E, 0), 16);
  Str(A: 0: Casas, S);
  if Pos('.', S) > 0 then
    S := SemZerosNoFim(S);
  P := Pos('.', S);
  if P = 0 then
    Result := Agrupar(S)
  else
    Result := Agrupar(Copy(S, 1, P - 1)) + ',' + Copy(S, P + 1, Length(S));
  if V < 0 then
    Result := '-' + Result;
end;

function ParaNumero(const Texto: string): Double;
var
  Codigo: Integer;
begin
  Val(StringReplace(Texto, ',', '.', []), Result, Codigo);
  if (Texto = '') or (Codigo <> 0) then
    Result := 0;
end;

function ContarDigitos(const S: string): Integer;
var
  I: Integer;
begin
  Result := 0;
  for I := 1 to Length(S) do
    if S[I] in ['0'..'9'] then
      Inc(Result);
end;

{ TCalculadora }

procedure TCalculadora.Zerar;
begin
  FEntrada := '';
  FTemAcumulado := False;
  FTemPendente := False;
  FTemRepetir := False;
  FNovo := False;
  FTemFuncao := False;
  FFuncaoTexto := '';
  Expressao := '';
  FErro := '';
  FTemErro := False;
end;

procedure TCalculadora.Falhar(const E: string);
begin
  FErro := E;
  FTemErro := True;
end;

function TCalculadora.Visor: string;
var
  Resto, Sinal: string;
  P: Integer;
begin
  if FTemErro then
    Exit(FErro);
  if FEntrada = '' then
    Exit('0');
  if FNovo then
    Exit(Formatar(Valor));
  Sinal := '';
  Resto := FEntrada;
  if (Length(Resto) > 0) and (Resto[1] = '-') then
  begin
    Sinal := '-';
    Delete(Resto, 1, 1);
  end;
  P := Pos(',', Resto);
  if P = 0 then
    Result := Sinal + Agrupar(Resto)
  else
    Result := Sinal + Agrupar(Copy(Resto, 1, P - 1)) + ',' + Copy(Resto, P + 1, Length(Resto));
end;

function TCalculadora.TemErro: Boolean;
begin
  Result := FTemErro;
end;

function TCalculadora.Valor: Double;
begin
  Result := ParaNumero(FEntrada);
end;

procedure TCalculadora.Mostrar(V: Double);
begin
  if V = 0 then
    FEntrada := ''
  else
    FEntrada := StringReplace(Formatar(V), '.', '', [rfReplaceAll]);
  if (Abs(V) >= 1e16) or ((V <> 0) and (Abs(V) < 1e-16)) then
  begin
    { Notação científica: guarda o valor exato (os 17 dígitos do Str voltam ao mesmo número). }
    Str(V, FEntrada);
    FEntrada := StringReplace(Trim(FEntrada), '.', ',', []);
  end;
end;

procedure TCalculadora.SairDoErro;
begin
  if FTemErro then
    Zerar;
end;

procedure TCalculadora.Digito(D: Integer);
begin
  SairDoErro;
  if FNovo then
  begin
    FEntrada := '';
    FNovo := False;
    FTemFuncao := False;
    if not FTemPendente then
      Expressao := '';
  end;
  if ContarDigitos(FEntrada) >= 16 then
    Exit;
  if (FEntrada = '0') or (FEntrada = '-0') then
    Delete(FEntrada, Length(FEntrada), 1);
  FEntrada := FEntrada + Chr(Ord('0') + D);
end;

procedure TCalculadora.Virgula;
begin
  SairDoErro;
  if FNovo then
  begin
    FEntrada := '';
    FNovo := False;
    FTemFuncao := False;
  end;
  if Pos(',', FEntrada) = 0 then
  begin
    if (FEntrada = '') or (FEntrada = '-') then
      FEntrada := FEntrada + '0';
    FEntrada := FEntrada + ',';
  end;
end;

procedure TCalculadora.Apagar;
begin
  if FTemErro then
  begin
    SairDoErro;
    Exit;
  end;
  if FNovo then
  begin
    if not FTemPendente then
      Expressao := '';
    Exit;
  end;
  if Length(FEntrada) > 0 then
    Delete(FEntrada, Length(FEntrada), 1);
  if (FEntrada = '-') or (FEntrada = '-0') then
    FEntrada := '';
end;

procedure TCalculadora.InverterSinal;
var
  V: Double;
  Dentro: string;
begin
  if FTemErro or (FEntrada = '') then
    Exit;
  if FEntrada[1] = '-' then
    Delete(FEntrada, 1, 1)
  else
    FEntrada := '-' + FEntrada;
  if FNovo then
  begin
    V := Valor;
    if FTemFuncao then Dentro := FFuncaoTexto else Dentro := Formatar(-V);
    FFuncaoTexto := 'negate(' + Dentro + ')';
    FTemFuncao := True;
    ExpressaoFuncao;
  end;
end;

procedure TCalculadora.LimparEntrada;
begin
  if FTemErro then
  begin
    SairDoErro;
    Exit;
  end;
  FEntrada := '';
  FTemFuncao := False;
  if not FTemPendente then
    Expressao := ''
  else if FTemAcumulado then
    Expressao := Formatar(FAcumulado) + ' ' + Simbolo(FPendente);
  FNovo := False;
end;

procedure TCalculadora.Limpar;
begin
  Zerar;
end;

function TCalculadora.TextoDoNumero(V: Double): string;
begin
  if FTemFuncao then Result := FFuncaoTexto else Result := Formatar(V);
end;

procedure TCalculadora.Operacao(Op: TOp);
var
  V, R: Double;
  E: string;
begin
  if FTemErro then
    Exit;
  V := Valor;
  if FTemAcumulado and FTemPendente then
  begin
    if FNovo and not FTemFuncao then
    begin
      { Trocou a operação antes de digitar o segundo número. }
      FPendente := Op;
      Expressao := Formatar(FAcumulado) + ' ' + Simbolo(Op);
      Exit;
    end;
    { Encadeado: 2 + 3 + → mostra 5. }
    R := Aplicar(FPendente, FAcumulado, V, E);
    if E <> '' then
    begin
      Expressao := Formatar(FAcumulado) + ' ' + Simbolo(FPendente) + ' ' + TextoDoNumero(V) + ' ' + Simbolo(Op);
      Falhar(E);
      Exit;
    end;
    FAcumulado := R;
    Mostrar(R);
  end
  else
  begin
    FAcumulado := V;
    FTemAcumulado := True;
  end;
  FPendente := Op;
  FTemPendente := True;
  FTemRepetir := False;
  FNovo := True;
  FTemFuncao := False;
  Expressao := Formatar(FAcumulado) + ' ' + Simbolo(Op);
end;

procedure TCalculadora.Igual;
var
  V, A, B, R: Double;
  Op: TOp;
  Segundo, E: string;
  Conta: TConta;
begin
  if FTemErro then
  begin
    SairDoErro;
    Exit;
  end;
  V := Valor;
  if FTemAcumulado and FTemPendente then
  begin
    A := FAcumulado;
    Op := FPendente;
    B := V;
  end
  else if FTemRepetir then
  begin
    A := V;
    Op := FRepetirOp;
    B := FRepetirB;
  end
  else
  begin
    { Só um número: `5 =`. }
    Expressao := TextoDoNumero(V) + ' =';
    Conta.Expressao := Expressao;
    Conta.Resultado := V;
    SetLength(Historico, Length(Historico) + 1);
    Historico[High(Historico)] := Conta;
    FNovo := True;
    FTemFuncao := False;
    Exit;
  end;
  if FTemPendente then Segundo := TextoDoNumero(B) else Segundo := Formatar(B);
  Expressao := Formatar(A) + ' ' + Simbolo(Op) + ' ' + Segundo + ' =';
  R := Aplicar(Op, A, B, E);
  if E = '' then
  begin
    Conta.Expressao := Expressao;
    Conta.Resultado := R;
    SetLength(Historico, Length(Historico) + 1);
    Historico[High(Historico)] := Conta;
    Mostrar(R);
    FTemRepetir := True;
    FRepetirOp := Op;
    FRepetirB := B;
  end
  else
    Falhar(E);
  FTemAcumulado := False;
  FTemPendente := False;
  FNovo := True;
  FTemFuncao := False;
end;

procedure TCalculadora.Porcentagem;
var
  V, R: Double;
begin
  if FTemErro then
    Exit;
  V := Valor;
  if FTemAcumulado and FTemPendente and (FPendente in [opSomar, opSubtrair]) then
    R := FAcumulado * V / 100
  else if FTemAcumulado and FTemPendente then
    R := V / 100
  else
    R := 0; { sem conta, o Windows zera }
  Mostrar(R);
  FFuncaoTexto := Formatar(R);
  FTemFuncao := True;
  FNovo := True;
  ExpressaoFuncao;
end;

procedure TCalculadora.Funcao(const F: string);
var
  V, R: Double;
  Dentro, Texto, E: string;
begin
  if FTemErro then
    Exit;
  V := Valor;
  Dentro := TextoDoNumero(V);
  R := 0;
  E := '';
  if F = 'inverso' then
  begin
    Texto := '1/(' + Dentro + ')';
    if V = 0 then E := DIV_ZERO else R := 1 / V;
  end
  else if F = 'quadrado' then
  begin
    Texto := 'sqr(' + Dentro + ')';
    R := V * V;
  end
  else
  begin
    Texto := '√(' + Dentro + ')';
    if V < 0 then E := INVALIDA else R := Sqrt(V);
  end;
  FFuncaoTexto := Texto;
  FTemFuncao := True;
  ExpressaoFuncao;
  if E = '' then Mostrar(R) else Falhar(E);
  FNovo := True;
end;

procedure TCalculadora.ExpressaoFuncao;
var
  Texto: string;
begin
  if FTemFuncao then Texto := FFuncaoTexto else Texto := '';
  if FTemAcumulado and FTemPendente then
    Expressao := Formatar(FAcumulado) + ' ' + Simbolo(FPendente) + ' ' + Texto
  else
    Expressao := Texto;
end;

{ Usa um resultado do histórico ou da memória (clicar na lista). }
procedure TCalculadora.Usar(AValor: Double);
begin
  SairDoErro;
  Mostrar(AValor);
  FNovo := True;
  FTemFuncao := False;
end;

procedure TCalculadora.MemoriaGuardar;
begin
  if not FTemErro then
  begin
    SetLength(Memoria, Length(Memoria) + 1);
    Memoria[High(Memoria)] := Valor;
    FNovo := True;
  end;
end;

procedure TCalculadora.MemoriaLer;
begin
  if Length(Memoria) > 0 then
    Usar(Memoria[High(Memoria)]);
end;

procedure TCalculadora.MemoriaSomar(Sinal: Double);
var
  V: Double;
begin
  if FTemErro then
    Exit;
  V := Valor * Sinal;
  if Length(Memoria) > 0 then
    Memoria[High(Memoria)] := Memoria[High(Memoria)] + V
  else
  begin
    SetLength(Memoria, 1);
    Memoria[0] := V;
  end;
  FNovo := True;
end;

{ Aperta um botão pelo texto dele (o mesmo da tela e do teclado). }
function TCalculadora.Tecla(const T: string): Boolean;
begin
  Result := True;
  if (Length(T) = 1) and (T[1] in ['0'..'9']) then Digito(Ord(T[1]) - Ord('0'))
  else if (T = ',') or (T = '.') then Virgula
  else if T = '+' then Operacao(opSomar)
  else if (T = '−') or (T = '-') then Operacao(opSubtrair)
  else if (T = '×') or (T = '*') then Operacao(opMultiplicar)
  else if (T = '÷') or (T = '/') then Operacao(opDividir)
  else if T = '=' then Igual
  else if T = '%' then Porcentagem
  else if T = '⌫' then Apagar
  else if T = 'CE' then LimparEntrada
  else if T = 'C' then Limpar
  else if T = '±' then InverterSinal
  else if T = '¹⁄ₓ' then Funcao('inverso')
  else if T = 'x²' then Funcao('quadrado')
  else if T = '²√x' then Funcao('raiz')
  else if T = 'MC' then SetLength(Memoria, 0)
  else if T = 'MR' then MemoriaLer
  else if T = 'M+' then MemoriaSomar(1)
  else if T = 'M-' then MemoriaSomar(-1)
  else if T = 'MS' then MemoriaGuardar
  else Result := False;
end;

{ O botão de uma tecla, como os atalhos da calculadora do Windows; 'historico' mostra ou
  esconde o histórico. Vazio: a tecla não faz nada. }
function BotaoDoTeclado(const Tecla: string; Ctrl, Shift: Boolean): string;
begin
  Result := '';
  if Ctrl then
  begin
    if Tecla = 'm' then Result := 'MS'
    else if Tecla = 'r' then Result := 'MR'
    else if Tecla = 'p' then Result := 'M+'
    else if Tecla = 'q' then Result := 'M-'
    else if Tecla = 'l' then Result := 'MC'
    else if Tecla = 'h' then Result := 'historico';
    Exit;
  end;
  if Shift then
  begin
    if Tecla = '=' then Exit('+');
    if Tecla = '8' then Exit('×');
    if Tecla = '5' then Exit('%');
    if Tecla = '2' then Exit('²√x');
  end;
  if (Length(Tecla) = 1) and (Tecla[1] in ['0'..'9']) then Exit(Tecla);
  if Tecla = '+' then Result := '+'
  else if Tecla = '-' then Result := '−'
  else if Tecla = '*' then Result := '×'
  else if Tecla = '/' then Result := '÷'
  else if (Tecla = ',') or (Tecla = '.') then Result := ','
  else if Tecla = '%' then Result := '%'
  else if Tecla = 'r' then Result := '¹⁄ₓ'
  else if Tecla = 'q' then Result := 'x²'
  else if Tecla = '@' then Result := '²√x'
  else if (Tecla = '=') or (Tecla = 'enter') then Result := '='
  else if Tecla = 'backspace' then Result := '⌫'
  else if Tecla = 'delete' then Result := 'CE'
  else if Tecla = 'escape' then Result := 'C'
  else if Tecla = 'f9' then Result := '±';
end;

end.
