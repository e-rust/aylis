{ O fechamento do mês: o trabalho pesado numa TThread própria e o ciclo dele (começar,
  cancelar, terminar). A thread grava o andamento direto na tela (Send tem trava) e volta à
  thread dos handlers no fim pelo Post, como o TThread.Queue do Delphi. }
unit fechamento;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils, aylis;

const
  { Quantas notas o fechamento processa (30 ms cada: uns 3 s). }
  Notas = 100;

type
  TControle = class;

  { A thread: processa as notas e mostra o andamento no frmProgresso. }
  TFechamento = class(TThread)
  private
    FControle: TControle;
    FFeitas: Integer;
    FCancelado: Boolean;
  protected
    procedure Execute; override;
  public
    constructor Create(AControle: TControle);
    property Feitas: Integer read FFeitas;
    property Cancelado: Boolean read FCancelado;
  end;

  { O fechamento em andamento. Vive o app inteiro: o Post do fim nunca chama um objeto já
    liberado. }
  TControle = class
  private
    FAylis: TAylis;
    FTrabalho: TFechamento;
    FConcluido: Boolean;
    { Pelo Post, na thread dos handlers: a thread terminou, fecha o modal. }
    procedure Terminou;
  public
    { Começa a thread e abre o frmProgresso como modal: o ShowModal espera ele fechar (pelo
      fim do trabalho ou pelo X), como no Delphi; enquanto isso, o Cancelar e o Post da thread
      rodam. }
    procedure Iniciar(Aylis: TAylis);
    { Pede para a thread parar (ela para na próxima nota). }
    procedure Cancelar;
    { Espera a thread acabar, libera e devolve o resumo para a tela. }
    function Encerrar: string;
    function Ativo: Boolean;
    { A thread terminou (até o fim ou cancelada): o modal já pode fechar. Antes disso, o X
      do frmProgresso só cancela (o on_close_query dele). }
    property Concluido: Boolean read FConcluido;
  end;

var
  Controle: TControle;

implementation

uses
  fpjson, ayliscomponents, progresso;

constructor TFechamento.Create(AControle: TControle);
begin
  inherited Create(True); // suspensa: começa no Start
  FControle := AControle;
end;

procedure TFechamento.Execute;
var
  I: Integer;
begin
  try
    for I := 1 to Notas do
    begin
      if Terminated then
      begin
        FCancelado := True;
        Exit;
      end;
      Sleep(30); // uma nota
      FFeitas := I;
      if I mod 5 = 0 then
      begin
        // Gravar direto daqui pode: é só uma mensagem, e a escrita tem trava. Ler (o get)
        // não pode: a resposta chega pela leitura da thread dos handlers.
        frmProgresso.pbProgresso.Value := I * 100 div Notas;
        frmProgresso.lblEtapa.Text := Format('Nota %d de %d', [I, Notas]);
      end;
    end;
  finally
    FControle.FAylis.Post(@FControle.Terminou);
  end;
end;

procedure TControle.Iniciar(Aylis: TAylis);
begin
  FAylis := Aylis;
  FConcluido := False;
  // Como o Application.CreateForm: cria o form do projeto (o mesmo, se já existe) e a
  // instância dele (a variável frmProgresso passa a apontar para ela).
  Aylis.CreateForm(TFrmProgresso);
  frmProgresso.pbProgresso.Value := 0;
  frmProgresso.lblEtapa.Text := 'Começando...';
  frmProgresso.btnCancelar.Enabled := True;
  FTrabalho := TFechamento.Create(Self);
  FTrabalho.Start;
  // O modal bloqueia os outros forms na tela; o ShowModal espera ele fechar tratando os
  // eventos que chegam (o Cancelar do modal, o Post do fim da thread).
  Aylis.ShowModal(frmProgresso);
end;

procedure TControle.Cancelar;
begin
  if FTrabalho <> nil then
    FTrabalho.Terminate;
end;

procedure TControle.Terminou;
begin
  // Fecha o modal (o ShowModal do Iniciar volta): agora o on_close_query deixa.
  FConcluido := True;
  if FTrabalho <> nil then
    FAylis.Send('modal_result', TJSONObject.Create(['target', frmProgresso.Id, 'result', 'ok']));
end;

function TControle.Encerrar: string;
begin
  // O modal só fecha com a thread no fim (o on_close_query do frmProgresso), então aqui
  // ela já terminou ou está terminando.
  FTrabalho.Terminate;
  FTrabalho.WaitFor;
  if FTrabalho.Cancelado then
    Result := Format('Cancelado na nota %d de %d', [FTrabalho.Feitas, Notas])
  else
    Result := Format('Mês fechado: %d notas', [FTrabalho.Feitas]);
  FreeAndNil(FTrabalho);
end;

function TControle.Ativo: Boolean;
begin
  Result := FTrabalho <> nil;
end;

initialization
  Controle := TControle.Create;

finalization
  Controle.Free;
end.
