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
  fpjson, ayliscomponents, calculadora;

function DispatchFrmCalculadora(Instance: TAylisForm; const Handler: string; Sender: TAylisComponent; Msg: TJSONObject): Boolean;
begin
  Result := True;
  if Handler = 'limpar_historico_click' then
    TFrmCalculadora(Instance).limpar_historico_click(TButton(Sender))
  else
  if Handler = 'historico_item_click' then
    TFrmCalculadora(Instance).historico_item_click(TListView(Sender), Msg.Find('args').AsInt64)
  else
  if Handler = 'memoria_item_click' then
    TFrmCalculadora(Instance).memoria_item_click(TListView(Sender), Msg.Find('args').AsInt64)
  else
  if Handler = 'historico_click' then
    TFrmCalculadora(Instance).historico_click(TButton(Sender))
  else
  if Handler = 'tecla_click' then
    TFrmCalculadora(Instance).tecla_click(TButton(Sender))
  else
    Result := False;
end;

procedure RegisterForms(Aylis: TAylis);
begin
  RegisterTypedHandlers('frmCalculadora', @DispatchFrmCalculadora);
end;

end.
