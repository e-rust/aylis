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
    procedure form_key_down(Sender: TForm; Value: TKeyEvent);
    procedure limpar_historico_click(Sender: TButton);
    procedure historico_item_click(Sender: TListView; Value: Int64);
    procedure memoria_item_click(Sender: TListView; Value: Int64);
    procedure historico_click(Sender: TButton);
    procedure tecla_click(Sender: TButton);
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
  Visor: string;
  Itens: TListViewItemArray;
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
  Itens := nil;
  for I := High(Calc.Historico) downto 0 do
  begin
    SetLength(Itens, Length(Itens) + 1);
    Itens[High(Itens)].Title := Formatar(Calc.Historico[I].Resultado);
    Itens[High(Itens)].Detail := Calc.Historico[I].Expressao;
  end;
  if Length(Itens) = 0 then
  begin
    SetLength(Itens, 1);
    Itens[0].Title := 'Ainda não há histórico';
  end;
  frmCalculadora.lvHistorico.Items := Itens;
  Itens := nil;
  for I := High(Calc.Memoria) downto 0 do
  begin
    SetLength(Itens, Length(Itens) + 1);
    Itens[High(Itens)].Title := Formatar(Calc.Memoria[I]);
  end;
  if Length(Itens) = 0 then
  begin
    SetLength(Itens, 1);
    Itens[0].Title := 'Não há nada salvo na memória';
  end;
  frmCalculadora.lvMemoria.Items := Itens;
end;

{ Aperta um botão pelo texto dele. }
procedure Apertar(Aylis: TAylis; const Texto: string);
begin
  if Calc.Tecla(Texto) then
    Atualizar(Aylis);
end;

{ Aylis.Reply('key', '') engoliria a tecla. }
procedure TFrmCalculadora.form_key_down(Sender: TForm; Value: TKeyEvent);
var
  Botao: string;
begin
  Botao := BotaoDoTeclado(Value.Key, Value.Ctrl, Value.Shift);
  if Botao = 'historico' then
    historico_click(btnHistorico)
  else if Botao <> '' then
    Apertar(Aylis, Botao);
end;

procedure TFrmCalculadora.limpar_historico_click(Sender: TButton);
begin
  SetLength(Calc.Historico, 0);
  Atualizar(Aylis);
end;

procedure TFrmCalculadora.historico_item_click(Sender: TListView; Value: Int64);
begin
  { A lista mostra a conta mais nova em cima. }
  if (Value >= 0) and (Value < Length(Calc.Historico)) then
  begin
    Calc.Usar(Calc.Historico[High(Calc.Historico) - Value].Resultado);
    Atualizar(Aylis);
  end;
end;

procedure TFrmCalculadora.memoria_item_click(Sender: TListView; Value: Int64);
begin
  if (Value >= 0) and (Value < Length(Calc.Memoria)) then
  begin
    Calc.Usar(Calc.Memoria[High(Calc.Memoria) - Value]);
    Atualizar(Aylis);
  end;
end;

procedure TFrmCalculadora.historico_click(Sender: TButton);
begin
  pnlLateral.Visible := not pnlLateral.Visible;
end;

procedure TFrmCalculadora.tecla_click(Sender: TButton);
var
  Texto: string;
begin
  Texto := Sender.Text;
  Apertar(Aylis, Texto);
end;

initialization
  RegisterForm(TFrmCalculadora, 'frmCalculadora', @frmCalculadora);
  Calc := TCalculadora.Create;
finalization
  Calc.Free;
end.
