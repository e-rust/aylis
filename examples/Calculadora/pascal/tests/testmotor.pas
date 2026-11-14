{ As contas da calculadora, como a do Windows (os mesmos casos do teste em Rust,
  examples/Calculadora/rust/tests/motor.rs). FPCUnit; rodam pelo `test` do Aylis. }
unit testmotor;

{$mode objfpc}{$H+}

interface

uses
  fpcunit, testregistry, motor;

type
  TTestMotor = class(TTestCase)
  published
    procedure ContasBasicasEEncadeadas;
    procedure IgualRepetido;
  end;

implementation

function Conta(const Teclas: array of string): TCalculadora;
var
  T: string;
begin
  Result := TCalculadora.Create;
  for T in Teclas do
    if not Result.Tecla(T) then
      raise EAssertionFailedError.Create('tecla ' + T);
end;

procedure Confere(const Esperado: string; Teclas: array of string; Expressao: Boolean = False);
var
  C: TCalculadora;
  Veio: string;
begin
  C := Conta(Teclas);
  try
    if Expressao then
      Veio := C.Expressao
    else
      Veio := C.Visor;
  finally
    C.Free;
  end;
  TAssert.AssertEquals(Esperado, Veio);
end;

procedure TTestMotor.ContasBasicasEEncadeadas;
begin
  Confere('42', ['1', '2', '+', '3', '0', '=']);
  Confere('12 + 30 =', ['1', '2', '+', '3', '0', '='], True);
  { 2 + 3 × 4: a do Windows (padrão) faz na ordem: (2 + 3) × 4. }
  Confere('20', ['2', '+', '3', '×', '4', '=']);
  Confere('3,5', ['7', '÷', '2', '=']);
  Confere('0,3', ['0', ',', '1', '+', '0', ',', '2', '=']);
end;

procedure TTestMotor.IgualRepetido;
begin
  Confere('11', ['2', '+', '3', '=', '=', '=']);
end;

initialization
  RegisterTest(TTestMotor);
end.
