{ Calculadora: o ponto de entrada. Gerado pela IDE do aylis (pode editar). }
program app;

{$mode objfpc}{$H+}

uses
  aylis, aylisforms;

var
  Tela: TAylis;
begin
  Tela := TAylis.Create;
  RegisterForms(Tela);
  Tela.Run('');  { abre o form principal do projeto }
  Tela.Free;
end.
