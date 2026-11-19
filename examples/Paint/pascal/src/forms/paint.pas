{ Eventos do form frmPaint: os handlers são métodos published da classe do form
  (achados pelo nome do layout); a IDE só acrescenta os novos. }
unit paint;

{$mode objfpc}{$H+}

interface

uses
  SysUtils, fpjson, aylis, ayliscomponents;

type
  TFrmPaint = class(TFrmPaintBase)
  published
    procedure novo_click(Sender: TMenuItem);
    procedure abrir_click(Sender: TMenuItem);
    procedure salvar_click(Sender: TAylisComponent);
    procedure salvar_como_click(Sender: TMenuItem);
    procedure sair_click(Sender: TMenuItem);
    procedure desfazer_click(Sender: TAylisComponent);
    procedure refazer_click(Sender: TAylisComponent);
    procedure recortar_click(Sender: TMenuItem);
    procedure copiar_click(Sender: TMenuItem);
    procedure colar_click(Sender: TMenuItem);
    procedure excluir_click(Sender: TMenuItem);
    procedure selecionar_tudo_click(Sender: TMenuItem);
    procedure limpar_click(Sender: TMenuItem);
    procedure ampliar_click(Sender: TMenuItem);
    procedure reduzir_click(Sender: TMenuItem);
    procedure tamanho_real_click(Sender: TMenuItem);
    procedure ferramenta_click(Sender: TButton);
    procedure preencher_change(Sender: TCheckBox; Value: Boolean);
    procedure espessura_change(Sender: TComboBox; Value: string);
    procedure cor1_click(Sender: TCircle);
    procedure cor2_click(Sender: TCircle);
    procedure paleta_click(Sender: TCircle);
    procedure desenho_change(Sender: TPaintCanvas);
    procedure desenho_pick(Sender: TPaintCanvas; Value: string);
    procedure desenho_cursor(Sender: TPaintCanvas; Value: string);
    procedure abrir_select(Sender: TOpenDialog; Value: TStringArray);
    procedure salvar_select(Sender: TSaveDialog; Value: TStringArray);
  end;

var
  frmPaint: TFrmPaint;

implementation

uses
  Math;

type
  TBotoesFerramenta = array[0..8] of TButton;

const
  { A ferramenta de cada botão de BotoesFerramenta, na mesma ordem. }
  NOMES_FERRAMENTA: array[0..8] of TPaintTool = (TPaintTool.Select, TPaintTool.Pencil, TPaintTool.Brush, TPaintTool.Eraser,
    TPaintTool.Fill, TPaintTool.Picker, TPaintTool.Line, TPaintTool.Rectangle, TPaintTool.Ellipse);

var
  { O que o form guarda: o arquivo aberto, se mudou, qual cor a paleta troca e o zoom (%). }
  Arquivo: string = '';
  Modificado: Boolean = False;
  CorAtiva: Integer = 1;
  ZoomAtual: Integer = 100;

{ O desenho em si é do componente PaintCanvas (pcDesenho); aqui ficam o menu, as
  ferramentas, as cores, o zoom, o arquivo e a barra de status. }

{ O primeiro caminho da lista do diálogo (um arquivo só). }
function Primeiro(const Caminhos: TStringArray): string;
begin
  if Length(Caminhos) > 0 then
    Result := Caminhos[0]
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
  Tamanho: TImageSize;
begin
  Desfazer := frmPaint.pcDesenho.CanUndo;
  Refazer := frmPaint.pcDesenho.CanRedo;
  frmPaint.mnuDesfazer.Enabled := Desfazer;
  frmPaint.btnDesfazer.Enabled := Desfazer;
  frmPaint.mnuRefazer.Enabled := Refazer;
  frmPaint.btnRefazer.Enabled := Refazer;
  Tamanho := frmPaint.pcDesenho.Size;
  frmPaint.lblTamanho.Text := IntToStr(Tamanho.Width) + ' × ' + IntToStr(Tamanho.Height) + 'px';
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
    frmPaint.btnCor1.Stroke := AylisRGB($00, $67, $C0);
    frmPaint.btnCor1.StrokeWidth := 3;
    frmPaint.btnCor2.Stroke := AylisRGB($9A, $9A, $9A);
    frmPaint.btnCor2.StrokeWidth := 1;
  end
  else
  begin
    frmPaint.btnCor1.Stroke := AylisRGB($9A, $9A, $9A);
    frmPaint.btnCor1.StrokeWidth := 1;
    frmPaint.btnCor2.Stroke := AylisRGB($00, $67, $C0);
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

procedure TFrmPaint.novo_click(Sender: TMenuItem);
begin
  pcDesenho.New(640, 400);
  Arquivo := '';
  Modificado := False;
  Titulo(Aylis);
  AtualizarEdicao(Aylis);
end;

procedure TFrmPaint.abrir_click(Sender: TMenuItem);
begin
  dlgAbrir.Execute;
end;

procedure TFrmPaint.salvar_click(Sender: TAylisComponent);
begin
  if Arquivo <> '' then
    SalvarEm(Aylis, Arquivo)
  else
    { Sem arquivo ainda: o mesmo do "Salvar como". }
    dlgSalvar.Execute;
end;

procedure TFrmPaint.salvar_como_click(Sender: TMenuItem);
begin
  dlgSalvar.Execute;
end;

procedure TFrmPaint.sair_click(Sender: TMenuItem);
begin
  Close;
end;

procedure TFrmPaint.desfazer_click(Sender: TAylisComponent);
begin
  pcDesenho.Undo;
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.refazer_click(Sender: TAylisComponent);
begin
  pcDesenho.Redo;
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.recortar_click(Sender: TMenuItem);
begin
  pcDesenho.Cut;
end;

procedure TFrmPaint.copiar_click(Sender: TMenuItem);
begin
  pcDesenho.Copy;
end;

procedure TFrmPaint.colar_click(Sender: TMenuItem);
begin
  { O que foi colado entra como seleção solta: a ferramenta passa a ser a seleção. }
  if pcDesenho.Paste then
    MarcarFerramenta(Aylis, frmPaint.btnSelecionar);
end;

procedure TFrmPaint.excluir_click(Sender: TMenuItem);
begin
  pcDesenho.Delete;
end;

procedure TFrmPaint.selecionar_tudo_click(Sender: TMenuItem);
begin
  pcDesenho.SelectAll;
  MarcarFerramenta(Aylis, frmPaint.btnSelecionar);
end;

procedure TFrmPaint.limpar_click(Sender: TMenuItem);
begin
  pcDesenho.Clear;
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.ampliar_click(Sender: TMenuItem);
begin
  AplicarZoom(Aylis, Min(ZoomAtual * 2, 800));
end;

procedure TFrmPaint.reduzir_click(Sender: TMenuItem);
begin
  AplicarZoom(Aylis, Max(ZoomAtual div 2, 25));
end;

procedure TFrmPaint.tamanho_real_click(Sender: TMenuItem);
begin
  AplicarZoom(Aylis, 100);
end;

procedure TFrmPaint.ferramenta_click(Sender: TButton);
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

procedure TFrmPaint.preencher_change(Sender: TCheckBox; Value: Boolean);
begin
  pcDesenho.Filled := Value;
end;

procedure TFrmPaint.espessura_change(Sender: TComboBox; Value: string);
var
  T: string;
  N, Codigo: Integer;
begin
  { "5 px" → 5 }
  T := Value;
  if Pos(' ', T) > 0 then
    T := Copy(T, 1, Pos(' ', T) - 1);
  Val(T, N, Codigo);
  if Codigo = 0 then
    pcDesenho.BrushSize := N;
end;

procedure TFrmPaint.cor1_click(Sender: TCircle);
begin
  MarcarCor(Aylis, 1);
end;

procedure TFrmPaint.cor2_click(Sender: TCircle);
begin
  MarcarCor(Aylis, 2);
end;

procedure TFrmPaint.paleta_click(Sender: TCircle);
var
  Cor: TAylisColor;
begin
  Cor := Sender.Fill;
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

procedure TFrmPaint.desenho_change(Sender: TPaintCanvas);
begin
  MarcarModificado(Aylis);
end;

procedure TFrmPaint.desenho_pick(Sender: TPaintCanvas; Value: string);
begin
  { "#RRGGBB" (cor 1) ou "#RRGGBB2" (cor 2, o botão direito) }
  if (Length(Value) > 0) and (Value[Length(Value)] = '2') then
    btnCor2.Fill := AylisColorHex(Copy(Value, 1, Length(Value) - 1))
  else
    btnCor1.Fill := AylisColorHex(Value);
end;

procedure TFrmPaint.desenho_cursor(Sender: TPaintCanvas; Value: string);
begin
  if Value <> '' then
    lblPosicao.Text := Value + 'px'
  else
    lblPosicao.Text := '';
end;

procedure TFrmPaint.abrir_select(Sender: TOpenDialog; Value: TStringArray);
var
  Caminho: string;
begin
  Caminho := Primeiro(Value);
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

procedure TFrmPaint.salvar_select(Sender: TSaveDialog; Value: TStringArray);
begin
  SalvarEm(Aylis, Primeiro(Value));
end;

initialization
  RegisterForm(TFrmPaint, 'frmPaint', @frmPaint);
finalization
end.
