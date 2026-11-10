{ Eventos do form frmIBGE. Gerado pela IDE do aylis: o corpo dos métodos é seu;
  a IDE só refaz os blocos marcados e acrescenta métodos novos.

  Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa
  e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem
  o mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou
  município escolhido: as bandeiras, os mapas e os dados dos gráficos. }
unit ibge;

{$mode objfpc}{$H+}

interface

uses
  fpjson, aylis, ayliscomponents;

type
  TFrmIBGE = class(TFrmIBGEBase)
  published
    procedure form_show(Sender: TAylisComponent; Event: TJSONObject);
    procedure estado_mudou(Sender: TAylisComponent; Event: TJSONObject);
    procedure municipio_mudou(Sender: TAylisComponent; Event: TJSONObject);
  end;

var
  frmIBGE: TFrmIBGE;

implementation

uses
  Classes, SysUtils, jsonparser;

const
  { A pasta dos dados no projeto (para os componentes). }
  Dados = 'assets/dados/';

var
  { Os municípios (para os 10 maiores de cada estado). }
  Municipios: TJSONArray;
  { Código do estado ou do município → [[ano, habitantes], ...]. }
  Populacao: TJSONObject;

{ A mesma pasta dos dados no disco (o backend roda na pasta da linguagem). }
function Pasta: string;
begin
  if DirectoryExists('../assets/dados') then
    Result := '../assets/dados/'
  else if DirectoryExists('assets/dados') then
    Result := 'assets/dados/'
  else
    Result := '../assets/dados/';
end;

{ Um arquivo JSON da pasta dos dados (nil se faltar). Quem chama libera. }
function Ler(const Arquivo: string): TJSONData;
var
  Texto: TStringList;
begin
  Result := nil;
  if not FileExists(Pasta + Arquivo) then
    Exit;
  Texto := TStringList.Create;
  try
    Texto.LoadFromFile(Pasta + Arquivo);
    Result := GetJSON(Texto.Text);
  finally
    Texto.Free;
  end;
end;

{ Põe os registros (uma lista de objetos) num ClientDataSet do form. `Linhas` passa a ser
  do comando (é liberado depois de enviado). }
procedure Carregar(Aylis: TAylis; Dataset: TAylisComponent; Linhas: TJSONData);
begin
  if Linhas = nil then
    Linhas := TJSONArray.Create;
  Aylis.Call('data', TJSONObject.Create(['target', Dataset.Id, 'op', 'load', 'rows', Linhas]));
end;

{ O registro atual de um ClientDataSet (campo → valor). Quem chama libera. }
function Registro(Aylis: TAylis; Dataset: TAylisComponent): TJSONObject;
var
  R: TJSONData;
begin
  R := Aylis.Call('data', TJSONObject.Create(['target', Dataset.Id, 'op', 'record']));
  if R is TJSONObject then
    Result := TJSONObject(R.Clone)
  else
    Result := TJSONObject.Create;
end;

{ O valor de um campo como texto (vazio se for nulo). }
function Texto(Obj: TJSONObject; const Campo: string): string;
var
  V: TJSONData;
begin
  V := Obj.Find(Campo);
  if (V = nil) or (V.JSONType = jtNull) then
    Result := ''
  else
    Result := V.AsString;
end;

{ Um número de um campo (0 se for nulo). }
function Numero(Obj: TJSONObject; const Campo: string): Int64;
var
  V: TJSONData;
begin
  V := Obj.Find(Campo);
  if (V = nil) or (V.JSONType <> jtNumber) then
    Result := 0
  else
    Result := V.AsInt64;
end;

{ O caminho de uma imagem da pasta dos dados (vazio: não tem). }
function Imagem(const Caminho: string): string;
begin
  if Caminho = '' then
    Result := ''
  else
    Result := Dados + Caminho;
end;

procedure TFrmIBGE.form_show(Sender: TAylisComponent; Event: TJSONObject);
var
  Estados, Lido: TJSONData;
begin
  Estados := Ler('estados.json');
  if Estados = nil then
  begin
    lblFonte.Text := 'Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados';
    Exit;
  end;
  FreeAndNil(Municipios);
  FreeAndNil(Populacao);
  Lido := Ler('municipios.json');
  if Lido is TJSONArray then Municipios := TJSONArray(Lido) else begin Lido.Free; Municipios := TJSONArray.Create; end;
  Lido := Ler('populacao.json');
  if Lido is TJSONObject then Populacao := TJSONObject(Lido) else begin Lido.Free; Populacao := TJSONObject.Create; end;
  svgBrasil.Source := Dados + 'mapas/BR.svg';
  // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
  Carregar(Aylis, frmIBGE.cdsDistritos, Ler('distritos.json'));
  Carregar(Aylis, frmIBGE.cdsMunicipios, Municipios.Clone);
  Carregar(Aylis, frmIBGE.cdsEstados, Estados);
  estado_mudou(Self, Event);
end;

{ Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios. }
procedure TFrmIBGE.estado_mudou(Sender: TAylisComponent; Event: TJSONObject);
var
  E, M, T: TJSONObject;
  UF: Int64;
  Mapa: string;
  DoEstado: array of TJSONObject;
  I, J, N: Integer;
  Maiores: TJSONArray;

  { A vem antes de B: mais habitantes; empatados, o nome. }
  function Antes(A, B: TJSONObject): Boolean;
  begin
    if Numero(A, 'populacao') <> Numero(B, 'populacao') then
      Result := Numero(A, 'populacao') > Numero(B, 'populacao')
    else
      Result := Texto(A, 'nome') < Texto(B, 'nome');
  end;

begin
  E := Registro(Aylis, frmIBGE.cdsEstados);
  try
    if Texto(E, 'id') = '' then
      Exit;
    UF := Numero(E, 'id');
    frmIBGE.lblEstado.Text := Texto(E, 'nome');
    frmIBGE.lblRegiao.Text := Texto(E, 'sigla') + ' · Região ' + Texto(E, 'regiao');
    frmIBGE.imgBandeiraEstado.Image := Imagem(Texto(E, 'bandeira'));
    frmIBGE.lblEstadoEstimativa.Text := 'Estimativa de ' + Texto(E, 'estimativa_ano');
  finally
    E.Free;
  end;
  frmIBGE.svgBrasil.Highlight := 'e' + IntToStr(UF);
  Mapa := Dados + 'mapas/' + IntToStr(UF) + '.svg';
  frmIBGE.svgEstado.Source := Mapa;
  frmIBGE.svgMunicipio.Source := Mapa;

  // Os municípios do estado, ordenados (inserção: são no máximo uns 850).
  N := 0;
  SetLength(DoEstado, Municipios.Count);
  for I := 0 to Municipios.Count - 1 do
  begin
    M := Municipios.Objects[I];
    if Numero(M, 'uf_id') <> UF then
      Continue;
    J := N;
    while (J > 0) and Antes(M, DoEstado[J - 1]) do
    begin
      DoEstado[J] := DoEstado[J - 1];
      Dec(J);
    end;
    DoEstado[J] := M;
    Inc(N);
  end;
  Maiores := TJSONArray.Create;
  for I := 0 to N - 1 do
  begin
    if I = 10 then
      Break;
    T := TJSONObject.Create;
    T.Add('nome', Texto(DoEstado[I], 'nome'));
    T.Add('habitantes', DoEstado[I].Find('populacao').Clone);
    Maiores.Add(T);
  end;
  Carregar(Aylis, frmIBGE.cdsMaiores, Maiores);
  municipio_mudou(Self, Event);
end;

{ Outro município: a bandeira, o destaque nos mapas e a população ano a ano. }
procedure TFrmIBGE.municipio_mudou(Sender: TAylisComponent; Event: TJSONObject);
var
  M, Linha: TJSONObject;
  Codigo: Int64;
  Bandeira, Alvo: string;
  Serie: TJSONData;
  Linhas: TJSONArray;
  I: Integer;
begin
  M := Registro(Aylis, frmIBGE.cdsMunicipios);
  try
    if Texto(M, 'id') = '' then
      Exit;
    Codigo := Numero(M, 'id');
    lblMunicipio.Text := Texto(M, 'nome');
    lblCodigo.Text := 'Código IBGE ' + IntToStr(Codigo);
    Bandeira := Texto(M, 'bandeira');
    imgBandeiraMunicipio.Image := Imagem(Bandeira);
    lblSemBandeira.Visible := Bandeira = '';
    lblEstimativa.Text := 'Estimativa de ' + Texto(M, 'estimativa_ano');
  finally
    M.Free;
  end;
  Alvo := 'm' + IntToStr(Codigo);
  svgEstado.Highlight := Alvo;
  svgMunicipio.Highlight := Alvo;
  svgMunicipio.ZoomTo := Alvo;

  Linhas := TJSONArray.Create;
  Serie := Populacao.Find(IntToStr(Codigo));
  if Serie is TJSONArray then
    for I := 0 to TJSONArray(Serie).Count - 1 do
    begin
      Linha := TJSONObject.Create;
      Linha.Add('ano', TJSONArray(Serie).Arrays[I].Items[0].AsString);
      Linha.Add('habitantes', TJSONArray(Serie).Arrays[I].Items[1].AsInt64);
      Linhas.Add(Linha);
    end;
  Carregar(Aylis, frmIBGE.cdsPopulacao, Linhas);
end;

initialization
  RegisterForm(TFrmIBGE, 'frmIBGE', @frmIBGE);
finalization
  Municipios.Free;
  Populacao.Free;
end.
