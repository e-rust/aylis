{ Backend de teste das sessões em Free Pascal (aylis-host --bridge ... --sessions): um processo
  para todas as telas, um contador por tela; "boom" lança uma exceção que fica só naquela
  sessão, "lento" demora sem travar as outras e "entrar"/"comum" ligam as telas do mesmo
  usuário. }
program sessions_backend;

{$mode objfpc}{$H+}

uses
  {$IFDEF UNIX}cthreads,{$ENDIF}
  Classes, SysUtils, fpjson, aylis;

const
  LAYOUT = '{"class": "Form", "name": "frm", "width": 200, "height": 80, "children": [' +
    '{"class": "Label", "name": "lbl", "text": "-"},' +
    '{"class": "Button", "name": "btn", "text": "+", "on_click": "somar"},' +
    '{"class": "Button", "name": "boom", "text": "!", "on_click": "boom"},' +
    '{"class": "Button", "name": "lento", "text": "z", "on_click": "lento"},' +
    '{"class": "Label", "name": "lblu", "text": "-"},' +
    '{"class": "Button", "name": "entrar", "text": "e", "on_click": "entrar"},' +
    '{"class": "Button", "name": "comum", "text": "c", "on_click": "comum"}]}';

type
  TContador = class(TAylisSession)
  private
    FTotal: Integer;
    FShared: Integer;
    procedure SetText(const Target, Text: string);
    procedure Somar(Aylis: TAylis; Event: TJSONObject);
    procedure Boom(Aylis: TAylis; Event: TJSONObject);
    procedure Lento(Aylis: TAylis; Event: TJSONObject);
    procedure Entrar(Aylis: TAylis; Event: TJSONObject);
    procedure Comum(Aylis: TAylis; Event: TJSONObject);
    procedure ShowShared(Session: TAylisSession);
  public
    procedure Started; override;
    procedure Ended; override;
  end;

procedure TContador.SetText(const Target, Text: string);
begin
  Send('set', TJSONObject.Create(['target', Target, 'props', TJSONObject.Create(['text', Text])]));
end;

procedure TContador.Somar(Aylis: TAylis; Event: TJSONObject);
begin
  Inc(FTotal);
  SetText('frm.lbl', IntToStr(Number) + ':' + IntToStr(FTotal));
end;

procedure TContador.Boom(Aylis: TAylis; Event: TJSONObject);
begin
  raise Exception.Create('erro de propósito');
end;

procedure TContador.Lento(Aylis: TAylis; Event: TJSONObject);
begin
  Sleep(3000);
end;

procedure TContador.Entrar(Aylis: TAylis; Event: TJSONObject);
begin
  Login('ana');
  SetText('frm.lblu', 'ana');
end;

{ O contador do usuário: o mesmo nas telas dele; todas mostram. }
procedure TContador.Comum(Aylis: TAylis; Event: TJSONObject);
begin
  User.Lock.Enter;
  try
    User.Data.Integers['total'] := User.Data.Get('total', 0) + 1;
  finally
    User.Lock.Leave;
  end;
  User.Broadcast(@ShowShared);
end;

procedure TContador.ShowShared(Session: TAylisSession);
var
  Total: Integer;
begin
  Session.User.Lock.Enter;
  try
    Total := Session.User.Data.Get('total', 0);
  finally
    Session.User.Lock.Leave;
  end;
  TContador(Session).SetText('frm.lblu', 'u:' + IntToStr(Total));
end;

procedure TContador.Started;
begin
  AylisLog('SELFTEST sessão ' + IntToStr(Number) + ' abriu (pid ' + IntToStr(GetProcessID) + ')');
  OnEvent('somar', @Somar);
  OnEvent('boom', @Boom);
  OnEvent('lento', @Lento);
  OnEvent('entrar', @Entrar);
  OnEvent('comum', @Comum);
  Send('load', TJSONObject.Create(['layout', GetJSON(LAYOUT)]));
  Run('frm');
end;

procedure TContador.Ended;
begin
  AylisLog('SELFTEST sessão ' + IntToStr(Number) + ' saiu');
end;

begin
  TAylisServer.Serve(TContador);
end.
