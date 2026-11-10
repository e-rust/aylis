{ Eventos do form frmPaint: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit paint;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis, ayliscomponents;

type
  TFrmPaint = class(TFrmPaintBase)
  published
    procedure novo_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure abrir_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure salvar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure salvar_como_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure sair_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure desfazer_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure refazer_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure recortar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure copiar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure colar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure excluir_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure selecionar_tudo_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure limpar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure ampliar_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure reduzir_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure tamanho_real_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure ferramenta_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure preencher_change(Sender: TAylisComponent; Event: TJSONObject);
    procedure espessura_change(Sender: TAylisComponent; Event: TJSONObject);
    procedure cor1_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure cor2_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure paleta_click(Sender: TAylisComponent; Event: TJSONObject);
    procedure desenho_change(Sender: TAylisComponent; Event: TJSONObject);
    procedure desenho_pick(Sender: TAylisComponent; Event: TJSONObject);
    procedure desenho_cursor(Sender: TAylisComponent; Event: TJSONObject);
    procedure abrir_select(Sender: TAylisComponent; Event: TJSONObject);
    procedure salvar_select(Sender: TAylisComponent; Event: TJSONObject);
  end;

var
  frmPaint: TFrmPaint;

implementation

uses
  SysUtils, Math;

type
  TBotoesFerramenta = array[0..8] of TButton;

const
  { A ferramenta de cada botão de BotoesFerramenta, na mesma ordem. }
  NOMES_FERRAMENTA: array[0..8] of string = ('select', 'pencil', 'brush', 'eraser', 'fill', 'picker', 'line',
    'rectangle', 'ellipse');

var
  { O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%). }
  Arquivo: string = '';
  Modificado: Boolean = False;
  CorAtiva: Integer = 1;
  ZoomAtual: Integer = 100;

{ O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
  ferramentas, as cores, o zoom, o arquivo e a barra de status. }

function Texto(Event: TJSONObject): string;
var
  D: TJSONData;
begin
  D := Event.Find('args');
  if (D <> nil) and (D.JSONType = jtString) then
    Result := D.AsString
  else
    Result := '';
end;

{ Os botões de ferramenta (os do form do evento). }
function BotoesFerramenta: TBotoesFerramenta;
begin
  Result[0] := frmPaint.btnSelecionar;
  Result[1] := frmPaint.btnLapis;
  Result[2] := frmPaint.btnPincel;
  Result[3] := frmPaint.btnBorracha;
  Result[4] := frmPaint.btnBalde;
  Result[5] := frmPaint.btnContaGotas;
  Result[6] := frmPaint.btnLinha;
  Result[7] := frmPaint.btnRetangulo;
  Result[8] := frmPaint.btnElipse;
end;

{ O nome do arquivo sem a pasta. }
function NomeDoArquivo(const Caminho: string): string;
var
  I: Integer;
begin
  I := Length(Caminho);
  while (I > 0) and not (Caminho[I] in ['\', '/']) do
    Dec(I);
  Result := Copy(Caminho, I + 1, Length(Caminho));
end;

{ O título como o do Paint do Windows: "*Sem título - Paint". }
procedure Titulo(Aylis: TAylis);
var
  Nome, Marca: string;
begin
  if Arquivo = '' then Nome := 'Sem título' else Nome := NomeDoArquivo(Arquivo);
  if Modificado then Marca := '*' else Marca := '';
  frmPaint.Caption := Marca + Nome + ' - Paint';
end;

{ Desfazer, refazer e o tamanho da imagem acompanham o desenho. }
procedure AtualizarEdicao(Aylis: TAylis);
var
  Desfazer, Refazer: Boolean;
begin
  Desfazer := frmPaint.pcDesenho.CanUndo;
  Refazer := frmPaint.pcDesenho.CanRedo;
  frmPaint.mnuDesfazer.Enabled := Desfazer;
  frmPaint.btnDesfazer.Enabled := Desfazer;
  frmPaint.mnuRefazer.Enabled := Refazer;
  frmPaint.btnRefazer.Enabled := Refazer;
  frmPaint.lblTamanho.Text := frmPaint.pcDesenho.Size + 'px';
end;

procedure MarcarModificado(Aylis: TAylis);
begin
  Modificado := True;
  Titulo(Aylis);
  AtualizarEdicao(Aylis);
end;

procedure AplicarZoom(Aylis: TAylis; Z: Integer);
begin
  ZoomAtual := Z;
  frmPaint.pcDesenho.Zoom := Z / 100;
  frmPaint.lblZoom.Text := IntToStr(Z) + '%';
end;

{ Só o botão da ferramenta escolhida fica marcado. }
procedure MarcarFerramenta(Aylis: TAylis; Escolhido: TButton);
var
  B: TButton;
begin
  for B in BotoesFerramenta do
    B.Checked := B.Id = Escolhido.Id;
end;

{ A cor que a paleta troca fica com o anel azul, como no Paint. }
procedure MarcarCor(Aylis: TAylis; Cor: Integer);
begin
  CorAtiva := Cor;
  if Cor = 1 then
  begin
    frmPaint.btnCor1.Stroke := '#0067C0';
    frmPaint.btnCor1.StrokeWidth := 3;
    frmPaint.btnCor2.Stroke := '#9A9A9A';
    frmPaint.btnCor2.StrokeWidth := 1;
  end
  else
  begin
    frmPaint.btnCor1.Stroke := '#9A9A9A';
    frmPaint.btnCor1.StrokeWidth := 1;
    frmPaint.btnCor2.Stroke := '#0067C0';
    frmPaint.btnCor2.StrokeWidth := 3;
  end;
end;

procedure Erro(Aylis: TAylis; const Mensagem: string);
begin
  Aylis.Send('message_dlg', TJSONObject.Create(['text', Mensagem, 'kind', 'error', 'buttons', TJSONArray.Create(['ok'])]));
end;

procedure SalvarEm(Aylis: TAylis; const Caminho: string);
begin
  try
    frmPaint.pcDesenho.Save(Caminho);
  except
    on E: EAylis do
    begin
      Erro(Aylis, 'Não foi possível salvar a imagem.' + LineEnding + E.Message);
      Exit;
    end;
  end;
  Arquivo := Caminho;
  Modificado := False;
  Titulo(Aylis);
end;

procedure TFrmPaint.novo_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.New(640, 400);
  Arquivo := '';
  Modificado := False;
  Titulo(Aylis);
  AtualizarEdicao(Aylis);
end;

procedure TFrmPaint.abrir_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  dlgAbrir.Execute;
end;

procedure TFrmPaint.salvar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  if Arquivo <> '' then
    SalvarEm(Aylis, Arquivo)
  else
    salvar_como_click(Self, Event);
end;

procedure TFrmPaint.salvar_como_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  dlgSalvar.Execute;
end;

procedure TFrmPaint.sair_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  Close;
end;

procedure TFrmPaint.desfazer_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.Undo;
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.refazer_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.Redo;
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.recortar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.Cut;
end;

procedure TFrmPaint.copiar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.Copy;
end;

procedure TFrmPaint.colar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  { O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção. }
  if pcDesenho.Paste then
    MarcarFerramenta(Aylis, frmPaint.btnSelecionar);
end;

procedure TFrmPaint.excluir_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.Delete;
end;

procedure TFrmPaint.selecionar_tudo_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.SelectAll;
  MarcarFerramenta(Aylis, frmPaint.btnSelecionar);
end;

procedure TFrmPaint.limpar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  pcDesenho.Clear;
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.ampliar_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  AplicarZoom(Aylis, Min(ZoomAtual * 2, 800));
end;

procedure TFrmPaint.reduzir_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  AplicarZoom(Aylis, Max(ZoomAtual div 2, 25));
end;

procedure TFrmPaint.tamanho_real_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  AplicarZoom(Aylis, 100);
end;

procedure TFrmPaint.ferramenta_click(Sender: TAylisComponent; Event: TJSONObject);
var
  I: Integer;
  Botoes: TBotoesFerramenta;
begin
  Botoes := BotoesFerramenta;
  for I := Low(Botoes) to High(Botoes) do
    if Botoes[I].Id = Sender.Id then
    begin
      pcDesenho.Tool := NOMES_FERRAMENTA[I];
      MarcarFerramenta(Aylis, Botoes[I]);
      Exit;
    end;
end;

procedure TFrmPaint.preencher_change(Sender: TAylisComponent; Event: TJSONObject);
var
  D: TJSONData;
begin
  D := Event.Find('args');
  if (D <> nil) and (D.JSONType = jtBoolean) then
    pcDesenho.Filled := D.AsBoolean;
end;

procedure TFrmPaint.espessura_change(Sender: TAylisComponent; Event: TJSONObject);
var
  T: string;
  N, Codigo: Integer;
begin
  { "5 px" → 5 }
  T := Texto(Event);
  if Pos(' ', T) > 0 then
    T := Copy(T, 1, Pos(' ', T) - 1);
  Val(T, N, Codigo);
  if Codigo = 0 then
    pcDesenho.BrushSize := N;
end;

procedure TFrmPaint.cor1_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  MarcarCor(Aylis, 1);
end;

procedure TFrmPaint.cor2_click(Sender: TAylisComponent; Event: TJSONObject);
begin
  MarcarCor(Aylis, 2);
end;

procedure TFrmPaint.paleta_click(Sender: TAylisComponent; Event: TJSONObject);
var
  Cor: string;
begin
  Cor := SenderAsCircle(Event).Fill;
  if CorAtiva = 1 then
  begin
    pcDesenho.Color := Cor;
    btnCor1.Fill := Cor;
  end
  else
  begin
    pcDesenho.BackColor := Cor;
    btnCor2.Fill := Cor;
  end;
end;

procedure TFrmPaint.desenho_change(Sender: TAylisComponent; Event: TJSONObject);
begin
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.desenho_pick(Sender: TAylisComponent; Event: TJSONObject);
var
  T: string;
begin
  { "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito) }
  T := Texto(Event);
  if (Length(T) > 0) and (T[Length(T)] = '2') then
    btnCor2.Fill := Copy(T, 1, Length(T) - 1)
  else
    btnCor1.Fill := T;
end;

procedure TFrmPaint.desenho_cursor(Sender: TAylisComponent; Event: TJSONObject);
var
  T: string;
begin
  T := Texto(Event);
  if T <> '' then
    T := T + 'px';
  lblPosicao.Text := T;
end;

procedure TFrmPaint.abrir_select(Sender: TAylisComponent; Event: TJSONObject);
var
  Caminho: string;
begin
  Caminho := Texto(Event);
  try
    pcDesenho.Open(Caminho);
  except
    on E: EAylis do
    begin
      Erro(Aylis, 'Não foi possível abrir a imagem.' + LineEnding + E.Message);
      Exit;
    end;
  end;
  Arquivo := Caminho;
  Modificado := False;
  Titulo(Aylis);
  AtualizarEdicao(Aylis);
end;

procedure TFrmPaint.salvar_select(Sender: TAylisComponent; Event: TJSONObject);
begin
  SalvarEm(Aylis, Texto(Event));
end;

initialization
  RegisterForm(TFrmPaint, 'frmPaint', @frmPaint);
finalization
end.
