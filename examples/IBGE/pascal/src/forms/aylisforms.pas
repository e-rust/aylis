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
  fpjson, ayliscomponents, ibge;

function DispatchFrmIBGE(Instance: TAylisForm; const Handler: string; Sender: TAylisComponent; Msg: TJSONObject): Boolean;
begin
  Result := True;
  if Handler = 'form_show' then
    TFrmIBGE(Instance).form_show(TForm(Sender))
  else
  if Handler = 'estado_mudou' then
    TFrmIBGE(Instance).estado_mudou(TClientDataSet(Sender))
  else
  if Handler = 'municipio_mudou' then
    TFrmIBGE(Instance).municipio_mudou(TClientDataSet(Sender))
  else
    Result := False;
end;

procedure RegisterForms(Aylis: TAylis);
begin
  RegisterTypedHandlers('frmIBGE', @DispatchFrmIBGE);
end;

end.
