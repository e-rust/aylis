{ Eventos do form frmPlayer: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit player;

{$mode objfpc}{$H+}

interface

uses
  SysUtils, fpjson, aylis, ayliscomponents;

type
  TFrmPlayer = class(TFrmPlayerBase)
  published
    procedure abrir_click(Sender: TAylisComponent);
    procedure limpar_fila_click(Sender: TAylisComponent);
    procedure sair_click(Sender: TMenuItem);
    procedure reproduzir_click(Sender: TAylisComponent);
    procedure parar_click(Sender: TMenuItem);
    procedure anterior_click(Sender: TAylisComponent);
    procedure proxima_click(Sender: TAylisComponent);
    procedure embaralhar_click(Sender: TAylisComponent);
    procedure repetir_click(Sender: TAylisComponent);
    procedure agora_click(Sender: TAylisComponent);
    procedure fila_click(Sender: TAylisComponent);
    procedure posicao_seek(Sender: TSeekBar; Value: Double);
    procedure mudo_click(Sender: TButton);
    procedure volume_change(Sender: TTrackBar; Value: Double);
    procedure video_fim(Sender: TVideoPlayer);
    procedure video_erro(Sender: TVideoPlayer; Value: string);
    procedure fila_item_click(Sender: TListView; Value: Int64);
    procedure abrir_select(Sender: TOpenDialog; Value: TStringArray);
    procedure posicao_timer(Sender: TTimer);
  end;

var
  frmPlayer: TFrmPlayer;

implementation

uses
  Math, StrUtils;

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
  Itens: TListViewItemArray;
  I: Integer;
begin
  SetLength(Itens, Length(Fila));
  for I := 0 to High(Fila) do
  begin
    Itens[I].Title := NomeDoArquivo(Fila[I]);
    Itens[I].Detail := Pasta(Fila[I]);
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

procedure TFrmPlayer.abrir_click(Sender: TAylisComponent);
begin
  dlgAbrir.Execute;
end;

procedure TFrmPlayer.abrir_select(Sender: TOpenDialog; Value: TStringArray);
var
  Primeiro, I: Integer;
  Novos: array of string;
begin
  { Os arquivos escolhidos (o evento já separa a lista). }
  Novos := nil;
  for I := 0 to High(Value) do
    if Value[I] <> '' then
    begin
      SetLength(Novos, Length(Novos) + 1);
      Novos[High(Novos)] := Value[I];
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

procedure TFrmPlayer.limpar_fila_click(Sender: TAylisComponent);
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

procedure TFrmPlayer.sair_click(Sender: TMenuItem);
begin
  Close;
end;

procedure TFrmPlayer.reproduzir_click(Sender: TAylisComponent);
begin
  if Atual < 0 then
  begin
    if Length(Fila) > 0 then
      Tocar(Aylis, 0)
    else
      abrir_click(Sender);
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

procedure TFrmPlayer.parar_click(Sender: TMenuItem);
begin
  if Atual < 0 then
    Exit;
  vpVideo.Stop;
  Tocando := False;
  BotaoReproduzir(Aylis);
  sbPosicao.Position := 0;
end;

procedure TFrmPlayer.anterior_click(Sender: TAylisComponent);
begin
  if Atual > 0 then
    Tocar(Aylis, Atual - 1)
  else if (Atual = 0) and (Repetir = 1) then
    Tocar(Aylis, High(Fila))
  else if Atual = 0 then
    vpVideo.Seek(0);
end;

procedure TFrmPlayer.proxima_click(Sender: TAylisComponent);
var
  I: Integer;
begin
  I := ProximoIndice;
  if I >= 0 then
    Tocar(Aylis, I);
end;

procedure TFrmPlayer.embaralhar_click(Sender: TAylisComponent);
begin
  Embaralhar := not Embaralhar;
  btnEmbaralhar.Checked := Embaralhar;
  if Embaralhar then
    btnEmbaralhar.Hint := 'Embaralhar: ativado'
  else
    btnEmbaralhar.Hint := 'Embaralhar: desativado';
  mnuEmbaralhar.Checked := Embaralhar;
end;

procedure TFrmPlayer.repetir_click(Sender: TAylisComponent);
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

procedure TFrmPlayer.agora_click(Sender: TAylisComponent);
begin
  Pagina(Aylis, True);
end;

procedure TFrmPlayer.fila_click(Sender: TAylisComponent);
begin
  Pagina(Aylis, False);
end;

procedure TFrmPlayer.posicao_seek(Sender: TSeekBar; Value: Double);
begin
  vpVideo.Seek(Value);
end;

procedure TFrmPlayer.mudo_click(Sender: TButton);
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

procedure TFrmPlayer.volume_change(Sender: TTrackBar; Value: Double);
begin
  vpVideo.Volume := Value / 100;
end;

procedure TFrmPlayer.video_fim(Sender: TVideoPlayer);
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

procedure TFrmPlayer.video_erro(Sender: TVideoPlayer; Value: string);
begin
  Tocando := False;
  BotaoReproduzir(Aylis);
  lblDetalhe.Text := 'Não foi possível reproduzir este arquivo';
end;

procedure TFrmPlayer.fila_item_click(Sender: TListView; Value: Int64);
var
  I: Integer;
begin
  I := Value;
  if (I >= 0) and (I < Length(Fila)) then
    Tocar(Aylis, I);
end;

procedure TFrmPlayer.posicao_timer(Sender: TTimer);
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
