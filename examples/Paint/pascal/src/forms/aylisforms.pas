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
  fpjson, ayliscomponents, paint;

function DispatchFrmPaint(Instance: TAylisForm; const Handler: string; Sender: TAylisComponent; Msg: TJSONObject): Boolean;
begin
  Result := True;
  if Handler = 'novo_click' then
    TFrmPaint(Instance).novo_click(TMenuItem(Sender))
  else
  if Handler = 'abrir_click' then
    TFrmPaint(Instance).abrir_click(TMenuItem(Sender))
  else
  if Handler = 'salvar_click' then
    TFrmPaint(Instance).salvar_click(Sender)
  else
  if Handler = 'salvar_como_click' then
    TFrmPaint(Instance).salvar_como_click(TMenuItem(Sender))
  else
  if Handler = 'sair_click' then
    TFrmPaint(Instance).sair_click(TMenuItem(Sender))
  else
  if Handler = 'desfazer_click' then
    TFrmPaint(Instance).desfazer_click(Sender)
  else
  if Handler = 'refazer_click' then
    TFrmPaint(Instance).refazer_click(Sender)
  else
  if Handler = 'recortar_click' then
    TFrmPaint(Instance).recortar_click(TMenuItem(Sender))
  else
  if Handler = 'copiar_click' then
    TFrmPaint(Instance).copiar_click(TMenuItem(Sender))
  else
  if Handler = 'colar_click' then
    TFrmPaint(Instance).colar_click(TMenuItem(Sender))
  else
  if Handler = 'excluir_click' then
    TFrmPaint(Instance).excluir_click(TMenuItem(Sender))
  else
  if Handler = 'selecionar_tudo_click' then
    TFrmPaint(Instance).selecionar_tudo_click(TMenuItem(Sender))
  else
  if Handler = 'limpar_click' then
    TFrmPaint(Instance).limpar_click(TMenuItem(Sender))
  else
  if Handler = 'ampliar_click' then
    TFrmPaint(Instance).ampliar_click(TMenuItem(Sender))
  else
  if Handler = 'reduzir_click' then
    TFrmPaint(Instance).reduzir_click(TMenuItem(Sender))
  else
  if Handler = 'tamanho_real_click' then
    TFrmPaint(Instance).tamanho_real_click(TMenuItem(Sender))
  else
  if Handler = 'ferramenta_click' then
    TFrmPaint(Instance).ferramenta_click(TButton(Sender))
  else
  if Handler = 'preencher_change' then
    TFrmPaint(Instance).preencher_change(TCheckBox(Sender), Msg.Find('args').AsBoolean)
  else
  if Handler = 'espessura_change' then
    TFrmPaint(Instance).espessura_change(TComboBox(Sender), AsText(Msg.Find('args')))
  else
  if Handler = 'cor1_click' then
    TFrmPaint(Instance).cor1_click(TCircle(Sender))
  else
  if Handler = 'cor2_click' then
    TFrmPaint(Instance).cor2_click(TCircle(Sender))
  else
  if Handler = 'paleta_click' then
    TFrmPaint(Instance).paleta_click(TCircle(Sender))
  else
  if Handler = 'desenho_change' then
    TFrmPaint(Instance).desenho_change(TPaintCanvas(Sender))
  else
  if Handler = 'desenho_pick' then
    TFrmPaint(Instance).desenho_pick(TPaintCanvas(Sender), AsText(Msg.Find('args')))
  else
  if Handler = 'desenho_cursor' then
    TFrmPaint(Instance).desenho_cursor(TPaintCanvas(Sender), AsText(Msg.Find('args')))
  else
  if Handler = 'abrir_select' then
    TFrmPaint(Instance).abrir_select(TOpenDialog(Sender), AsList(Msg.Find('args')))
  else
  if Handler = 'salvar_select' then
    TFrmPaint(Instance).salvar_select(TSaveDialog(Sender), AsList(Msg.Find('args')))
  else
    Result := False;
end;

procedure RegisterForms(Aylis: TAylis);
begin
  RegisterTypedHandlers('frmPaint', @DispatchFrmPaint);
end;

end.
