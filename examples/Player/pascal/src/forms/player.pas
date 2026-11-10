{ Eventos do form frmPlayer: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit player;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis, ayliscomponents;

type
  TFrmPlayer = class(TFrmPlayerBase)
  published
    procedure abrir_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure limpar_fila_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure sair_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure reproduzir_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure parar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure anterior_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure proxima_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure embaralhar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure repetir_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure agora_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure fila_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure posicao_seek(Sender: TAylisComponent; Event: TJSONObject);
    procedure mudo_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure volume_change(Sender: TAylisComponent; Event: TJSONObject);
    procedure video_fim(Sender: TAylisComponent; Event: TJSONObject);
    procedure video_erro(Sender: TAylisComponent; Event: TJSONObject);
    procedure fila_item_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure abrir_select(Sender: TAylisComponent; Event: TJSONObject);
    procedure posicao_timer(Sender: TAylisComponent; Event: TJSONObject);
  end;

var
  frmPlayer: TFrmPlayer;

implementation

uses
  SysUtils, Math, StrUtils;

const
  { As extensões de áudio (o resto é vídeo). }
  AUDIO: array[0..6] of string = ('mp3', 'wav', 'm4a', 'flac', 'ogg', 'wma', 'aac');
  DICAS_REPETIR: array[0..2] of string = ('Repetir: desativado', 'Repetir: todos', 'Repetir: um');

var
  { O que o form guarda. Repetir: 0 não repete, 1 repete a fila, 2 repete a mídia. }
  Fila: array of string;
  Atual: Integer = -1;
  Tocando: Boolean = False;
  Embaralhar: Boolean = False;
  Repetir: Integer = 0;
  Mudo: Boolean = False;
  Semente: Int64 = 12345;

{ O Reprodutor Multimídia do Windows: a fila de reprodução, os botões de tocar, pausar,
  anterior e próxima, embaralhar, repetir (todos ou um), volume e a barra de posição. Quem
  toca é o componente VideoPlayer (vpVideo), áudio e vídeo. }

function Separador(const Caminho: string): Integer;
begin
  Result := Max(RPos('\', Caminho), RPos('/', Caminho));
end;

{ O nome do arquivo sem a pasta. }
function NomeDoArquivo(const Caminho: string): string;
begin
  Result := Copy(Caminho, Separador(Caminho) + 1, Length(Caminho));
end;

{ A pasta do arquivo (vazio se não tiver). }
function Pasta(const Caminho: string): string;
begin
  if Separador(Caminho) > 0 then
    Result := Copy(Caminho, 1, Separador(Caminho) - 1)
  else
    Result := '';
end;

function SemExtensao(const Nome: string): string;
var
  I: Integer;
begin
  I := RPos('.', Nome);
  if I > 1 then
    Result := Copy(Nome, 1, I - 1)
  else
    Result := Nome;
end;

function Tipo(const Caminho: string): string;
var
  Nome, Ext: string;
  I: Integer;
begin
  Nome := NomeDoArquivo(Caminho);
  I := RPos('.', Nome);
  if I > 0 then Ext := LowerCase(Copy(Nome, I + 1, Length(Nome))) else Ext := '';
  Result := 'Vídeo';
  for I := Low(AUDIO) to High(AUDIO) do
    if Ext = AUDIO[I] then
      Exit('Música');
end;

{ "3:07" }
function Tempo(Segundos: Double): string;
var
  S: Int64;
begin
  S := Trunc(Floor(Max(Segundos, 0.0)));
  Result := IntToStr(S div 60) + ':' + Format('%.2d', [S mod 60]);
end;

{ A fila na lista, com a mídia atual selecionada. }
procedure AtualizarFila(Aylis: TAylis);
var
  Itens: string;
  I: Integer;
begin
  Itens := '';
  for I := 0 to High(Fila) do
  begin
    if I > 0 then
      Itens := Itens + ';';
    Itens := Itens + NomeDoArquivo(Fila[I]) + '|' + Pasta(Fila[I]);
  end;
  frmPlayer.lvFila.Items := Itens;
  frmPlayer.lvFila.ItemIndex := Atual;
end;

{ O título, o detalhe e o título da janela. }
procedure MostrarAtual(Aylis: TAylis);
var
  Caminho, Titulo: string;
begin
  if Atual < 0 then
  begin
    frmPlayer.lblTitulo.Text := 'Nenhuma mídia';
    frmPlayer.lblDetalhe.Text := '';
    frmPlayer.Caption := 'Reprodutor Multimídia';
    Exit;
  end;
  Caminho := Fila[Atual];
  Titulo := SemExtensao(NomeDoArquivo(Caminho));
  frmPlayer.lblTitulo.Text := Titulo;
  frmPlayer.lblDetalhe.Text := Tipo(Caminho) + ' · ' + IntToStr(Atual + 1) + ' de ' + IntToStr(Length(Fila));
  frmPlayer.Caption := Titulo + ' - Reprodutor Multimídia';
end;

{ O botão do meio mostra o que ele vai fazer: tocar ou pausar. }
procedure BotaoReproduzir(Aylis: TAylis);
begin
  if Tocando then
  begin
    frmPlayer.btnReproduzir.Icon := 'pause';
    frmPlayer.btnReproduzir.Hint := 'Pausar (Ctrl+P)';
  end
  else
  begin
    frmPlayer.btnReproduzir.Icon := 'play';
    frmPlayer.btnReproduzir.Hint := 'Reproduzir (Ctrl+P)';
  end;
end;

{ Toca a mídia I da fila. Tocar é a última coisa: um erro chega no video_erro. }
procedure Tocar(Aylis: TAylis; I: Integer);
begin
  Atual := I;
  Tocando := True;
  frmPlayer.vpVideo.&File := Fila[I];
  MostrarAtual(Aylis);
  AtualizarFila(Aylis);
  BotaoReproduzir(Aylis);
  frmPlayer.sbPosicao.Position := 0;
  frmPlayer.vpVideo.Play;
end;

{ A próxima da fila: na ordem ou embaralhada; no fim, volta ao começo se repetir a fila.
  -1: acabou. }
function ProximoIndice: Integer;
var
  N: Integer;
begin
  N := Length(Fila);
  if N = 0 then
    Exit(-1);
  if Embaralhar then
  begin
    if N = 1 then
      Exit(0);
    repeat
      Semente := (Semente * 1103515245 + 12345) mod 2147483648;
      Result := Semente mod N;
    until Result <> Atual;
    Exit;
  end;
  if Atual + 1 < N then
    Result := Atual + 1
  else if Repetir = 1 then
    Result := 0
  else
    Result := -1;
end;

procedure TFrmPlayer.abrir_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  dlgAbrir.Execute;
end;

procedure TFrmPlayer.abrir_select(Sender: TAylisComponent; Event: TJSONObject);
var
  Texto, Parte: string;
  D: TJSONData;
  Primeiro, I, Inicio: Integer;
  Novos: array of string;
begin
  { Os arquivos escolhidos vêm separados por ";". }
  D := Event.Find('args');
  if (D <> nil) and (D.JSONType = jtString) then Texto := D.AsString else Texto := '';
  Novos := nil;
  Inicio := 1;
  for I := 1 to Length(Texto) + 1 do
    if (I > Length(Texto)) or (Texto[I] = ';') then
    begin
      Parte := Copy(Texto, Inicio, I - Inicio);
      if Parte <> '' then
      begin
        SetLength(Novos, Length(Novos) + 1);
        Novos[High(Novos)] := Parte;
      end;
      Inicio := I + 1;
    end;
  if Length(Novos) = 0 then
    Exit;
  Primeiro := Length(Fila);
  for I := 0 to High(Novos) do
  begin
    SetLength(Fila, Length(Fila) + 1);
    Fila[High(Fila)] := Novos[I];
  end;
  AtualizarFila(Aylis);
  if Atual < 0 then
    Tocar(Aylis, Primeiro)
  else
    MostrarAtual(Aylis);
end;

procedure TFrmPlayer.limpar_fila_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  vpVideo.Stop;
  SetLength(Fila, 0);
  Atual := -1;
  Tocando := False;
  vpVideo.&File := '';
  AtualizarFila(Aylis);
  MostrarAtual(Aylis);
  BotaoReproduzir(Aylis);
end;

procedure TFrmPlayer.sair_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Close;
end;

procedure TFrmPlayer.reproduzir_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  if Atual < 0 then
  begin
    if Length(Fila) > 0 then
      Tocar(Aylis, 0)
    else
      abrir_click(Self, Event);
  end
  else if Tocando then
  begin
    vpVideo.Pause;
    Tocando := False;
    BotaoReproduzir(Aylis);
  end
  else
  begin
    Tocando := True;
    BotaoReproduzir(Aylis);
    vpVideo.Play;
  end;
end;

procedure TFrmPlayer.parar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  if Atual < 0 then
    Exit;
  vpVideo.Stop;
  Tocando := False;
  BotaoReproduzir(Aylis);
  sbPosicao.Position := 0;
end;

procedure TFrmPlayer.anterior_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  if Atual > 0 then
    Tocar(Aylis, Atual - 1)
  else if (Atual = 0) and (Repetir = 1) then
    Tocar(Aylis, High(Fila))
  else if Atual = 0 then
    vpVideo.Seek(0);
end;

procedure TFrmPlayer.proxima_click(Sender: TAylisComponent; Event: TJSONObject);
var
  I: Integer;
begin
  I := ProximoIndice;
  if I >= 0 then
    Tocar(Aylis, I);
end;

procedure TFrmPlayer.embaralhar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Embaralhar := not Embaralhar;
  btnEmbaralhar.Checked := Embaralhar;
  if Embaralhar then
    btnEmbaralhar.Hint := 'Embaralhar: ativado'
  else
    btnEmbaralhar.Hint := 'Embaralhar: desativado';
  mnuEmbaralhar.Checked := Embaralhar;
end;

procedure TFrmPlayer.repetir_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Repetir := (Repetir + 1) mod 3;
  btnRepetir.Checked := Repetir > 0;
  if Repetir = 2 then
    btnRepetir.Icon := 'repeat_one'
  else
    btnRepetir.Icon := 'repeat';
  btnRepetir.Hint := DICAS_REPETIR[Repetir];
  mnuRepetir.Checked := Repetir > 0;
end;

{ Mostra a página "Reproduzindo agora" ou a "Fila de reprodução". }
procedure Pagina(Aylis: TAylis; Agora: Boolean);
begin
  frmPlayer.pnlAgora.Visible := Agora;
  frmPlayer.pnlFila.Visible := not Agora;
  frmPlayer.btnAgora.Checked := Agora;
  frmPlayer.btnFila.Checked := not Agora;
end;

procedure TFrmPlayer.agora_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Pagina(Aylis, True);
end;

procedure TFrmPlayer.fila_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Pagina(Aylis, False);
end;

procedure TFrmPlayer.posicao_seek(Sender: TAylisComponent; Event: TJSONObject);
var
  D: TJSONData;
begin
  D := Event.Find('args');
  if (D <> nil) and (D.JSONType = jtNumber) then
    vpVideo.Seek(D.AsFloat);
end;

procedure TFrmPlayer.mudo_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Mudo := not Mudo;
  vpVideo.Muted := Mudo;
  if Mudo then
  begin
    btnMudo.Icon := 'mute';
    btnMudo.Hint := 'Com som';
  end
  else
  begin
    btnMudo.Icon := 'volume';
    btnMudo.Hint := 'Sem som';
  end;
end;

procedure TFrmPlayer.volume_change(Sender: TAylisComponent; Event: TJSONObject);
var
  D: TJSONData;
begin
  D := Event.Find('args');
  if (D <> nil) and (D.JSONType = jtNumber) then
    vpVideo.Volume := D.AsFloat / 100;
end;

procedure TFrmPlayer.video_fim(Sender: TAylisComponent; Event: TJSONObject);
var
  I: Integer;
begin
  if (Repetir = 2) and (Atual >= 0) then
  begin
    Tocar(Aylis, Atual);
    Exit;
  end;
  I := ProximoIndice;
  if I >= 0 then
    Tocar(Aylis, I)
  else
  begin
    Tocando := False;
    BotaoReproduzir(Aylis);
  end;
end;

procedure TFrmPlayer.video_erro(Sender: TAylisComponent; Event: TJSONObject);
begin
  Tocando := False;
  BotaoReproduzir(Aylis);
  lblDetalhe.Text := 'Não foi possível reproduzir este arquivo';
end;

procedure TFrmPlayer.fila_item_click(Sender: TAylisComponent; Event: TJSONObject);
var
  D: TJSONData;
  I: Integer;
begin
  D := Event.Find('args');
  if (D = nil) or (D.JSONType <> jtNumber) then
    Exit;
  I := D.AsInteger;
  if (I >= 0) and (I < Length(Fila)) then
    Tocar(Aylis, I);
end;

procedure TFrmPlayer.posicao_timer(Sender: TAylisComponent; Event: TJSONObject);
var
  Posicao, Duracao: Double;
begin
  if Atual < 0 then
    Exit;
  Posicao := vpVideo.Position;
  Duracao := vpVideo.Duration;
  sbPosicao.Duration := Duracao;
  sbPosicao.Position := Posicao;
  lblTempo.Text := Tempo(Posicao);
  lblDuracao.Text := Tempo(Duracao);
end;

initialization
  RegisterForm(TFrmPlayer, 'frmPlayer', @frmPlayer);
finalization
end.
