{ Eventos do form frmPrincipal: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit principal;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis, ayliscomponents;

type
  TFrmPrincipal = class(TFrmPrincipalBase)
  published
    procedure fechar_mes_click(Sender: TAylisComponent; Event: TJSONObject);
  end;

var
  frmPrincipal: TFrmPrincipal;

implementation

uses
  fechamento;

procedure TFrmPrincipal.fechar_mes_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  if Controle.Ativo then
    Exit;
  lblStatus.Text := 'Fechando...';
  { O ShowModal dentro do Iniciar espera o modal fechar: aqui, o trabalho
    já acabou ou foi cancelado. }
  Controle.Iniciar(Aylis);
  lblStatus.Text := Controle.Encerrar;
end;
initialization
  RegisterForm(TFrmPrincipal, 'frmPrincipal', @frmPrincipal);
end.
