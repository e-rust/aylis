{ Forms do projeto. Gerado pela IDE do Aylis a cada "Gerar código": não edite. }
unit aylisforms;

{$mode objfpc}{$H+}

interface

uses
  aylis;

{ Os forms do projeto (cada unit registra a classe do form dela ao carregar). }
procedure RegisterForms(Aylis: TAylis);

implementation

uses
  fpjson, ayliscomponents, principal, progresso;

function DispatchFrmPrincipal(Instance: TAylisForm; const Handler: string; Sender: TAylisComponent; Msg: TJSONObject): Boolean;
begin
  Result := True;
  if Handler = 'fechar_mes_click' then
    TFrmPrincipal(Instance).fechar_mes_click(TButton(Sender))
  else
    Result := False;
end;

function DispatchFrmProgresso(Instance: TAylisForm; const Handler: string; Sender: TAylisComponent; Msg: TJSONObject): Boolean;
begin
  Result := True;
  if Handler = 'cancelar_click' then
    TFrmProgresso(Instance).cancelar_click(TButton(Sender))
  else
    Result := False;
end;

procedure RegisterForms(Aylis: TAylis);
begin
  RegisterTypedHandlers('frmPrincipal', @DispatchFrmPrincipal);
  RegisterTypedHandlers('frmProgresso', @DispatchFrmProgresso);
end;

end.
