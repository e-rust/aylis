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
  fpjson, ayliscomponents, player;

function DispatchFrmPlayer(Instance: TAylisForm; const Handler: string; Sender: TAylisComponent; Msg: TJSONObject): Boolean;
begin
  Result := True;
  if Handler = 'abrir_click' then
    TFrmPlayer(Instance).abrir_click(Sender)
  else
  if Handler = 'limpar_fila_click' then
    TFrmPlayer(Instance).limpar_fila_click(Sender)
  else
  if Handler = 'sair_click' then
    TFrmPlayer(Instance).sair_click(TMenuItem(Sender))
  else
  if Handler = 'reproduzir_click' then
    TFrmPlayer(Instance).reproduzir_click(Sender)
  else
  if Handler = 'parar_click' then
    TFrmPlayer(Instance).parar_click(TMenuItem(Sender))
  else
  if Handler = 'anterior_click' then
    TFrmPlayer(Instance).anterior_click(Sender)
  else
  if Handler = 'proxima_click' then
    TFrmPlayer(Instance).proxima_click(Sender)
  else
  if Handler = 'embaralhar_click' then
    TFrmPlayer(Instance).embaralhar_click(Sender)
  else
  if Handler = 'repetir_click' then
    TFrmPlayer(Instance).repetir_click(Sender)
  else
  if Handler = 'agora_click' then
    TFrmPlayer(Instance).agora_click(Sender)
  else
  if Handler = 'fila_click' then
    TFrmPlayer(Instance).fila_click(Sender)
  else
  if Handler = 'posicao_seek' then
    TFrmPlayer(Instance).posicao_seek(TSeekBar(Sender), Msg.Find('args').AsFloat)
  else
  if Handler = 'mudo_click' then
    TFrmPlayer(Instance).mudo_click(TButton(Sender))
  else
  if Handler = 'volume_change' then
    TFrmPlayer(Instance).volume_change(TTrackBar(Sender), Msg.Find('args').AsFloat)
  else
  if Handler = 'video_fim' then
    TFrmPlayer(Instance).video_fim(TVideoPlayer(Sender))
  else
  if Handler = 'video_erro' then
    TFrmPlayer(Instance).video_erro(TVideoPlayer(Sender), AsText(Msg.Find('args')))
  else
  if Handler = 'fila_item_click' then
    TFrmPlayer(Instance).fila_item_click(TListView(Sender), Msg.Find('args').AsInt64)
  else
  if Handler = 'abrir_select' then
    TFrmPlayer(Instance).abrir_select(TOpenDialog(Sender), AsList(Msg.Find('args')))
  else
  if Handler = 'posicao_timer' then
    TFrmPlayer(Instance).posicao_timer(TTimer(Sender))
  else
    Result := False;
end;

procedure RegisterForms(Aylis: TAylis);
begin
  RegisterTypedHandlers('frmPlayer', @DispatchFrmPlayer);
end;

end.
