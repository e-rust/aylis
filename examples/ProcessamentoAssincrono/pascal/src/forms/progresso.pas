{ Eventos do form frmProgresso: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit progresso;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis, ayliscomponents;

type
  TFrmProgresso = class(TFrmProgressoBase)
  published
    procedure cancelar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure form_close_query(Sender: TAylisComponent; var CanClose: Boolean);
  end;

var
  frmProgresso: TFrmProgresso;

implementation

uses
  fechamento;

{ Roda enquanto a thread trabalha: a thread dos handlers está livre. }
procedure TFrmProgresso.cancelar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  btnCancelar.Enabled := False;
  lblEtapa.Text := 'Cancelando...';
  Controle.Cancelar;
end;

{ O X com o trabalho rodando (o OnCloseQuery do Delphi): fechar no meio deixaria a thread
  gravando num form fechado, então o form fica aberto e a thread é cancelada, como pelo
  Cancelar. Quando ela termina, o Post do fim fecha o modal (aí CanClose fica True). }
procedure TFrmProgresso.form_close_query(Sender: TAylisComponent; var CanClose: Boolean);
begin
  if Controle.Concluido then
    Exit;
  CanClose := False;
  btnCancelar.Enabled := False;
  lblEtapa.Text := 'Cancelando...';
  Controle.Cancelar;
end;
initialization
  RegisterForm(TFrmProgresso, 'frmProgresso', @frmProgresso);
end.
