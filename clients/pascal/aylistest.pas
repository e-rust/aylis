{ O executor de testes do Aylis para Free Pascal (o `test` da IDE e do MCP): os testes da pasta
  `tests/` do projeto são do FPCUnit (TTestCase, RegisterTest na initialization), e esta unit
  os roda ouvindo o resultado, com uma linha por teste que o Aylis lê:

      AYLIS_TEST ok TTestMotor.TestSoma
      AYLIS_TEST FAIL TTestMotor.TestDivisao esperado 2, veio 3

  O Aylis gera o programa que chama RunAll; sai com 1 se algum falhou. }
unit aylistest;

{$mode objfpc}{$H+}

interface

{ Roda todos os testes registrados e termina o programa com o código de saída. }
procedure RunAll;

implementation

uses
  SysUtils, Classes, fpcunit, testregistry;

type
  TAylisListener = class(TInterfacedObject, ITestListener)
  private
    FFailed: Boolean;
    FMessage: string;
  public
    Passed, Failed: Integer;
    procedure AddFailure(ATest: TTest; AFailure: TTestFailure);
    procedure AddError(ATest: TTest; AError: TTestFailure);
    procedure StartTest(ATest: TTest);
    procedure EndTest(ATest: TTest);
    procedure StartTestSuite(ATestSuite: TTestSuite);
    procedure EndTestSuite(ATestSuite: TTestSuite);
  end;

{ A mensagem numa linha só (a linha do teste tem de ficar inteira). }
function OneLine(const S: string): string;
begin
  Result := StringReplace(StringReplace(S, #13, ' ', [rfReplaceAll]), #10, ' ', [rfReplaceAll]);
end;

procedure TAylisListener.AddFailure(ATest: TTest; AFailure: TTestFailure);
begin
  FFailed := True;
  FMessage := AFailure.ExceptionMessage;
end;

procedure TAylisListener.AddError(ATest: TTest; AError: TTestFailure);
begin
  FFailed := True;
  FMessage := AError.ExceptionClassName + ': ' + AError.ExceptionMessage;
end;

procedure TAylisListener.StartTest(ATest: TTest);
begin
  FFailed := False;
  FMessage := '';
end;

procedure TAylisListener.EndTest(ATest: TTest);
var
  Name: string;
begin
  Name := ATest.TestSuiteName + '.' + ATest.TestName;
  if FFailed then
  begin
    WriteLn('AYLIS_TEST FAIL ', Name, ' ', OneLine(FMessage));
    Inc(Failed);
  end
  else
  begin
    WriteLn('AYLIS_TEST ok ', Name);
    Inc(Passed);
  end;
end;

procedure TAylisListener.StartTestSuite(ATestSuite: TTestSuite);
begin
end;

procedure TAylisListener.EndTestSuite(ATestSuite: TTestSuite);
begin
end;

procedure RunAll;
var
  Results: TTestResult;
  Listener: TAylisListener;
  Keep: ITestListener;
begin
  Results := TTestResult.Create;
  Listener := TAylisListener.Create;
  { A interface guarda o ouvinte vivo (contado por referência) até o fim. }
  Keep := Listener;
  try
    Results.AddListener(Keep);
    GetTestRegistry.Run(Results);
    WriteLn('AYLIS_TEST_DONE ', Listener.Passed, ' ok, ', Listener.Failed, ' falharam');
    if Listener.Failed > 0 then
      ExitCode := 1;
  finally
    Results.Free;
  end;
end;

end.
