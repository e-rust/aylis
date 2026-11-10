{ Backend de demonstração em Free Pascal: um campo, um botão e a resposta do backend.

    fpc -Mobjfpc -FEclients/pascal/out clients/pascal/demo.lpr
    aylis-host -- clients/pascal/out/demo.exe
    aylis-host --png tela.png -- clients/pascal/out/demo.exe --selftest }
program demo;

{$mode objfpc}{$H+}

uses
  Classes, SysUtils, fpjson, aylis;

type
  TDemo = class
    Aylis: TAylis;
    SelfTest: Boolean;
    procedure Hello(Sender: TAylis; Event: TJSONObject);
    procedure PodeFechar(Sender: TAylis; Event: TJSONObject);
  end;

procedure TDemo.Hello(Sender: TAylis; Event: TJSONObject);
var
  Name: string;
begin
  Name := Aylis.Call('get', TJSONObject.Create(['target', 'edtName', 'prop', 'text'])).AsString;
  if Name = '' then
    Name := 'mundo';
  Aylis.Send('set', TJSONObject.Create(['target', 'lblMsg', 'props', TJSONObject.Create(['text', 'Olá, ' + Name + '! (Free Pascal)'])]));
  if SelfTest then
  begin
    AylisLog('SELFTEST pascal ' + Aylis.Call('get', TJSONObject.Create(['target', 'lblMsg', 'prop', 'text'])).AsString);
    { Fechar pelo backend passa pelo on_close_query, que pergunta (e espera a resposta). }
    Aylis.Send('close', TJSONObject.Create(['target', 'frmDemo']));
  end;
end;

{ O on_close_query pergunta: a tela só fecha com a resposta (o Reply). }
procedure TDemo.PodeFechar(Sender: TAylis; Event: TJSONObject);
begin
  AylisLog('SELFTEST pascal pergunta');
  AylisLog('SELFTEST pascal ' + AylisDeltaSelftest);
  Aylis.Reply('can_close', True);
end;

var
  App: TDemo;
  Layout: TStringList;
begin
  App := TDemo.Create;
  App.Aylis := TAylis.Create;
  App.SelfTest := (ParamCount > 0) and (ParamStr(1) = '--selftest');
  Layout := TStringList.Create;
  try
    Layout.LoadFromFile('clients/demo_layout.json');
    App.Aylis.Send('load', TJSONObject.Create(['layout', GetJSON(Layout.Text)]));
  finally
    Layout.Free;
  end;
  App.Aylis.OnEvent('hello', @App.Hello);
  App.Aylis.OnEvent('pode_fechar', @App.PodeFechar);
  App.Aylis.Call('run', TJSONObject.Create(['form', 'frmDemo']));
  if App.SelfTest then
  begin
    App.Aylis.Send('set', TJSONObject.Create(['target', 'edtName', 'props', TJSONObject.Create(['text', 'Ana'])]));
    App.Aylis.Send('click', TJSONObject.Create(['target', 'btnHello']));
  end;
  App.Aylis.Loop;
  App.Aylis.Free;
  App.Free;
end.
