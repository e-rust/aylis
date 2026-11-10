{ Eventos do form frmCalculadora: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit calculadora;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis, ayliscomponents;

type
  TFrmCalculadora = class(TFrmCalculadoraBase)
  published
    procedure form_key_down(Sender: TAylisComponent; var Key: string; Event: TJSONObject);
    procedure limpar_historico_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure historico_item_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure memoria_item_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure historico_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure tecla_click(Sender: TAylisComponent; Event: TJSONObject);
  end;

var
  frmCalculadora: TFrmCalculadora;

implementation

uses
  SysUtils, Math, motor;

var
  Calc: TCalculadora;

type
  TBotoesSemErro = array[0..12] of TButton;

{ Os botões que só funcionam sem erro no visor (os do form do evento). }
function SemErro: TBotoesSemErro;
begin
  Result[0] := frmCalculadora.btnMMais;
  Result[1] := frmCalculadora.btnMMenos;
  Result[2] := frmCalculadora.btnMS;
  Result[3] := frmCalculadora.btnPorcento;
  Result[4] := frmCalculadora.btnInverso;
  Result[5] := frmCalculadora.btnQuadrado;
  Result[6] := frmCalculadora.btnRaiz;
  Result[7] := frmCalculadora.btnDividir;
  Result[8] := frmCalculadora.btnMultiplicar;
  Result[9] := frmCalculadora.btnSubtrair;
  Result[10] := frmCalculadora.btnSomar;
  Result[11] := frmCalculadora.btnSinal;
  Result[12] := frmCalculadora.btnVirgula;
end;

{ Quantos caracteres (não bytes) o texto UTF-8 tem. }
function Caracteres(const S: string): Integer;
var
  I: Integer;
begin
  Result := 0;
  for I := 1 to Length(S) do
    if (Ord(S[I]) and $C0) <> $80 then
      Inc(Result);
end;

{ Põe o estado da calculadora na tela. }
procedure Atualizar(Aylis: TAylis);
var
  Visor, Itens: string;
  N, Tamanho, I: Integer;
  Erro: Boolean;
  Botao: TButton;
begin
  Visor := Calc.Visor;
  { O número encolhe para caber, como no Windows. }
  N := Caracteres(Visor);
  if Calc.TemErro then
    Tamanho := 26
  else if N <= 11 then
    Tamanho := 46
  else
    Tamanho := Max(46 * 11 div N, 22);
  frmCalculadora.lblVisor.Text := Visor;
  frmCalculadora.lblVisor.FontSize := Tamanho;
  frmCalculadora.lblExpressao.Text := Calc.Expressao;
  Erro := Calc.TemErro;
  for Botao in SemErro do
    Botao.Enabled := not Erro;
  frmCalculadora.btnMC.Enabled := (not Erro) and (Length(Calc.Memoria) > 0);
  frmCalculadora.btnMR.Enabled := (not Erro) and (Length(Calc.Memoria) > 0);
  Itens := '';
  for I := High(Calc.Historico) downto 0 do
  begin
    if Itens <> '' then
      Itens := Itens + ';';
    Itens := Itens + Formatar(Calc.Historico[I].Resultado) + '|' + Calc.Historico[I].Expressao;
  end;
  if Itens = '' then
    Itens := 'Ainda não há histórico|';
  frmCalculadora.lvHistorico.Items := Itens;
  Itens := '';
  for I := High(Calc.Memoria) downto 0 do
  begin
    if Itens <> '' then
      Itens := Itens + ';';
    Itens := Itens + Formatar(Calc.Memoria[I]) + '|';
  end;
  if Itens = '' then
    Itens := 'Não há nada salvo na memória|';
  frmCalculadora.lvMemoria.Items := Itens;
end;

{ Aperta um botão pelo texto dele. }
procedure Apertar(Aylis: TAylis; const Texto: string);
begin
  if Calc.Tecla(Texto) then
    Atualizar(Aylis);
end;

function Indice(Event: TJSONObject): Integer;
var
  Args: TJSONData;
begin
  Args := Event.Find('args');
  if (Args <> nil) and (Args.JSONType = jtNumber) then
    Result := Args.AsInteger
  else
    Result := -1;
end;

procedure TFrmCalculadora.form_key_down(Sender: TAylisComponent; var Key: string; Event: TJSONObject);
var
  Args: TJSONObject;
  Botao: string;
begin
  if not (Event.Find('args') is TJSONObject) then
    Exit;
  Args := TJSONObject(Event.Find('args'));
  Botao := BotaoDoTeclado(Args.Get('key', ''), Args.Get('ctrl', False), Args.Get('shift', False));
  if Botao = 'historico' then
    historico_click(Self, Event)
  else if Botao <> '' then
    Apertar(Aylis, Botao);
end;

procedure TFrmCalculadora.limpar_historico_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  SetLength(Calc.Historico, 0);
  Atualizar(Aylis);
end;

procedure TFrmCalculadora.historico_item_click(Sender: TAylisComponent; Event: TJSONObject);
var
  I: Integer;
begin
  { A lista mostra a conta mais nova em cima. }
  I := Indice(Event);
  if (I >= 0) and (I < Length(Calc.Historico)) then
  begin
    Calc.Usar(Calc.Historico[High(Calc.Historico) - I].Resultado);
    Atualizar(Aylis);
  end;
end;

procedure TFrmCalculadora.memoria_item_click(Sender: TAylisComponent; Event: TJSONObject);
var
  I: Integer;
begin
  I := Indice(Event);
  if (I >= 0) and (I < Length(Calc.Memoria)) then
  begin
    Calc.Usar(Calc.Memoria[High(Calc.Memoria) - I]);
    Atualizar(Aylis);
  end;
end;

procedure TFrmCalculadora.historico_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pnlLateral.Visible := not pnlLateral.Visible;
end;

procedure TFrmCalculadora.tecla_click(Sender: TAylisComponent; Event: TJSONObject);
var
  Texto: string;
begin
  Texto := SenderAsButton(Event).Text;
  Apertar(Aylis, Texto);
end;

initialization
  RegisterForm(TFrmCalculadora, 'frmCalculadora', @frmCalculadora);
  Calc := TCalculadora.Create;
finalization
  Calc.Free;
end.
