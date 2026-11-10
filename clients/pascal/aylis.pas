{ Cliente do protocolo do aylis para backends em Free Pascal / Lazarus, via aylis-host.

  O aylis-host inicia este processo e conversa por stdin/stdout: uma mensagem JSON por
  linha. Usa a unit fpjson (fcl-json), que vem com o Free Pascal.

    Aylis := TAylis.Create;
    Aylis.Send('load', TJSONObject.Create(['layout', GetJSON(Texto)]));
    Name := Aylis.Call('get', TJSONObject.Create(['target', 'edtName', 'prop', 'text'])).AsString;
    Aylis.OnEvent('btn_salvar_click', @Self.BtnSalvarClick);
    Aylis.Run('frmClientes');

  Os objetos passados a Send/Call passam a ser do cliente (ele libera). O resultado de
  Call é do cliente também: copie o que precisar (Clone) antes da próxima chamada.

  Trabalho pesado vai para uma thread própria (TThread). De lá, Send pode (gravar
  propriedades, mostrar progresso: a escrita tem trava); Call não (a resposta chega pela
  mesma leitura da thread dos handlers). Para voltar à thread dos handlers no fim (fechar o
  form de progresso, ler a tela), use Post, como o TThread.Queue do Delphi:

    Aylis.Post(@Self.Terminou); }
unit aylis;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils, SyncObjs, fpjson, jsonparser, Contnrs, FmtBCD;

const
  { A versão do protocolo do aylis que este cliente fala (o hello). }
  AylisProtocol = 1;

type
  EAylis = class(Exception);

  TAylis = class;

  { Handler de evento: Sender é o cliente; Event traz event, handler, form, sender, args. }
  TAylisHandler = procedure(Aylis: TAylis; Event: TJSONObject) of object;

  { Um trabalho para a thread dos handlers (Post). }
  TAylisProc = procedure of object;

  TAylisForm = class;

  { Um componente de um form, pelo id dele na tela (resolvido quando a instância do form é
    montada). As classes de cada componente (TButton, TLabel...) vêm da unit gerada
    (ayliscomponents), com as propriedades tipadas. }
  TAylisComponent = class(TPersistent)
  private
    FAylis: TAylis;
    FId: Int64;
    FName: string;
  protected
    { Lê uma propriedade (o resultado é do cliente: use antes da próxima chamada). }
    function GetProp(const Prop: string): TJSONData;
    procedure SetProp(const Prop: string; Value: TJSONData);
    function Invoke(const Method: string; Args: TJSONArray): TJSONData;
  public
    constructor Create(AAylis: TAylis; AId: Int64; const AName: string); virtual;
    procedure SetFocus;
    { O cliente (Aylis.Call, Aylis.Send, Aylis.Post) e o id e o nome na tela. }
    property Aylis: TAylis read FAylis;
    property Id: Int64 read FId;
    property Name: string read FName;
  end;
  TAylisComponentClass = class of TAylisComponent;

  { Um evento de um form com instância: o componente que disparou (Sender) e a mensagem
    (Event: event, handler, form, sender, args). }
  TAylisFormEvent = procedure(Sender: TAylisComponent; Event: TJSONObject) of object;

  { Os eventos que perguntam (a tela espera a resposta), com var como no Delphi; os outros
    (before_post, before_delete, on_validate) são TAylisFormEvent e recusam com raise. }
  TAylisCloseQueryEvent = procedure(Sender: TAylisComponent; var CanClose: Boolean) of object;
  { Key é o nome da tecla ('a', 'enter', 'f2'); Key := '' engole (o Key := 0). }
  TAylisKeyEvent = procedure(Sender: TAylisComponent; var Key: string; Event: TJSONObject) of object;
  { Key é o caractere digitado; Key := '' engole, Key := '.' troca. }
  TAylisKeyPressEvent = procedure(Sender: TAylisComponent; var Key: string) of object;
  TAylisDragStartEvent = procedure(Sender: TAylisComponent; var Data, Caption: string) of object;
  TAylisDragOverEvent = procedure(Sender: TAylisComponent; const Data: string; var Accept: Boolean) of object;
  { O on_intercept de um EventInterceptor: Info traz o form, o sender, o event, os args e o
    handler do evento interceptado; Cancel := True faz o handler dele não rodar. }
  TAylisInterceptEvent = procedure(Sender: TAylisComponent; Info: TJSONObject; var Cancel: Boolean) of object;

  { A base dos forms, como o TForm do Delphi: cada form aberto na tela tem a sua instância
    (duas janelas do mesmo form, duas instâncias), com os componentes como campos published
    (a classe base gerada de cada form declara e monta) e o estado do usuário em campos
    private. Os handlers são métodos published, achados pelo nome do layout. }
  TAylisForm = class(TAylisComponent)
  private
    FComponents: TList;
  protected
    { Monta os componentes a partir da lista da tela ([{name, id, class}]); a classe base
      gerada de cada form sobrescreve. Um componente que o código espera e o form não tem
      (ou de outra classe) é um erro: o form não bate com o código. }
    procedure BindComponents(List: TJSONArray); virtual;
    function Resolve(List: TJSONArray; const AName, AClass: string; AType: TAylisComponentClass): TAylisComponent;
    function ResolveOptional(List: TJSONArray; const AName: string; AType: TAylisComponentClass): TAylisComponent;
  public
    constructor Create(AAylis: TAylis; AId: Int64; const AName: string); override;
    destructor Destroy; override;
    { O componente deste form com o id (nil se não é dele). }
    function ComponentById(AId: Int64): TAylisComponent;
    procedure Show;
    procedure Close;
  end;
  TAylisFormClass = class of TAylisForm;

  TAylis = class
  private
    FHandlers: TStringList;
    { As instâncias dos forms (TAylisForm), pelo id do form na tela. }
    FForms: TList;
    FQueued: TObjectQueue;
    FLastResult: TJSONData;
    { A resposta do evento-pergunta que roda agora (nil fora de um), e quantos rodam um dentro
      do outro (um evento-pergunta roda mesmo durante um Call). }
    FReply: TJSONObject;
    FAskDepth: Integer;
    { Os forms liberados na tela (o aviso __freed): os PostTo deles não rodam. }
    FFreed: TList;
    { Esperando um diálogo (MessageDlg, ShowModal): os eventos rodam já (os do modal). }
    FWaiting: Integer;
    FNextId: Integer;
    { A escrita vem de qualquer thread (Send numa TThread); a leitura, só da dos handlers. }
    FWriteLock: TCriticalSection;
    FPostLock: TCriticalSection;
    FPosted: TList;
    FOwner: TThreadID;
    { Já mandou o hello (a primeira mensagem, com a versão do protocolo). }
    FGreeted: Boolean;
    procedure Greet;
    function MakeMessage(const Cmd: string; Fields: TJSONObject): TJSONObject;
    procedure RunPosted;
    function FormInstance(const Form: string; FormId: Int64): TAylisForm;
    function DispatchToForm(Msg: TJSONObject): Boolean;
    procedure CallFormMethod(Method: TMethod; Sender: TAylisComponent; Msg: TJSONObject);
    procedure HandleEvent(Msg: TJSONObject);
    procedure HandleNested(Got: TJSONObject);
    function FieldText(const DataSet, Field: string): string;
    procedure FormsFreed(Msg: TJSONObject);
    function WaitFor(const Cmd: string; Fields: TJSONObject): string;
  protected
    { Escreve e lê uma mensagem (no stdin e no stdout; numa sessão, pelo servidor). }
    procedure WriteMessage(Msg: TJSONObject); virtual;
    function ReadMessage: TJSONObject; virtual;
    { Uma mensagem que não é resposta nem evento conhecido (as sessões tratam o "__post"). }
    procedure Special(Msg: TJSONObject); virtual;
  public
    constructor Create;
    destructor Destroy; override;
    { Envia um comando sem esperar resposta (erros chegam como evento "error"). }
    procedure Send(const Cmd: string; Fields: TJSONObject = nil);
    { Marca um trecho no profiler da IDE (a faixa "Marcas"): ProfileBegin('consulta'); ...
      ProfileEnd; }
    procedure ProfileBegin(const Name: string);
    procedure ProfileEnd;
    { Envia um comando e espera o resultado. Só da thread dos handlers. }
    function Call(const Cmd: string; Fields: TJSONObject = nil): TJSONData;
    { Roda Job na thread dos handlers, de qualquer thread (o TThread.Queue do Delphi): a
      tela devolve um eco que acorda o Loop, e Job roda na vez dele, entre os eventos. }
    procedure Post(Job: TAylisProc); overload;
    { Como o Post, ligado a um form: se o form foi liberado até lá (a janela fechou e saiu),
      o Job não roda (no Delphi, o TThread.Queue com o form já liberado dá Access Violation). }
    procedure PostTo(Owner: TAylisForm; Job: TAylisProc);
    { Dentro de um evento-pergunta (o evento veio com "ask"): um campo da resposta (can_close,
      key, data, label, accept), que o cliente manda quando o handler termina. Os handlers
      com var (TAylisCloseQueryEvent...) já respondem sozinhos. }
    procedure Reply(const Name: string; Value: Boolean); overload;
    procedure Reply(const Name, Value: string); overload;
    { Handler pelo nome usado no layout ('frmClientes.nome': só daquele form). }
    procedure OnEvent(const Name: string; Handler: TAylisHandler);
    { Cria o form na tela e devolve a instância dele (como o Application.CreateForm do
      Delphi); Another: mais uma janela do mesmo form, mesmo que ele já exista. }
    function CreateForm(const Form: string; Another: Boolean = False): TAylisForm; overload;
    { O mesmo pela classe do form (`Aylis.CreateForm(TFrmProgresso)`), como o
      Application.CreateForm do Delphi: sem o nome em texto no código. }
    function CreateForm(FormClass: TAylisFormClass; Another: Boolean = False): TAylisForm; overload;
    { O valor de um campo decimal (Currency, BCD) no tipo exato, como o AsCurrency e o AsBCD do
      Delphi: o protocolo manda o texto, e ele nunca passa por Double. Nulo: 0. }
    function FieldAsCurrency(const DataSet, Field: string): Currency;
    function FieldAsBCD(const DataSet, Field: string): TBCD;
    { Caixa de mensagem que espera, como o MessageDlg do Delphi: devolve o botão ('yes', 'no',
      'ok'...). Kind: information, warning, error, confirmation. Enquanto espera, os eventos
      continuam rodando (os Post também). }
    function MessageDlg(const Text: string; const Kind: string; const Buttons: array of string): string;
    { Abre o form como modal e espera ele fechar, como o ShowModal do Delphi: devolve o
      resultado ('ok', 'cancel'...). }
    function ShowModal(Form: TAylisForm): string;
    { Abre o form e trata os eventos até a tela fechar. }
    procedure Run(const Form: string);
    { Trata os eventos até a tela fechar. }
    procedure Loop;
  end;

  TAylisServer = class;
  TAylisUser = class;
  TAylisSession = class;

  { Um trabalho para rodar na thread de uma tela (Post, Broadcast). }
  TAylisJob = procedure(Session: TAylisSession) of object;

  { Uma tela da ponte com --sessions (aylis-host --bridge ... --sessions): um processo atende
    todas as telas e cada uma ganha uma instância da sua subclasse, numa thread própria (um
    handler demorado não trava as outras; uma exceção fica nela). Guarde o estado da tela nos
    campos da subclasse (nunca em variáveis globais, que são de todas). A API é a do TAylis.

      TContador = class(TAylisSession)
        Total: Integer;
        procedure Started; override;   // OnEvent(...); Run('frmVendas');
      end;
      TAylisServer.Serve(TContador); }
  TAylisSession = class(TAylis)
  private
    FServer: TAylisServer;
    FNumber: Integer;
    FUser: TAylisUser;
    FLock: TCriticalSection;
    FSignal: TEventObject;
    FInbox: TStringList;
    FJobs: TList;
  protected
    procedure WriteMessage(Msg: TJSONObject); override;
    function ReadMessage: TJSONObject; override;
    procedure Special(Msg: TJSONObject); override;
  public
    constructor Create(AServer: TAylisServer; ANumber: Integer); virtual;
    destructor Destroy; override;
    { Entrega uma mensagem a esta tela (o servidor chama). }
    procedure Deliver(const Line: string);
    { Roda Job(Self) na thread desta tela (de qualquer thread). }
    procedure Post(Job: TAylisJob); overload;
    { A tela conectou: monte o estado e abra o form. }
    procedure Started; virtual; abstract;
    { A tela saiu (fechou o app ou caiu a rede). }
    procedure Ended; virtual;
    { Liga esta tela ao usuário Id: as telas do mesmo usuário dividem o mesmo TAylisUser. }
    procedure Login(const Id: string);
    { Solta o usuário desta tela (sozinho, ao fechar). }
    procedure Logout;
    property Number: Integer read FNumber;
    property User: TAylisUser read FUser;
    property Server: TAylisServer read FServer;
  end;

  TAylisSessionClass = class of TAylisSession;

  { O usuário logado, dividido pelas telas dele. Data guarda o que vale para todas (o
    carrinho, as preferências); use Lock.Enter/Lock.Leave quando duas telas podem mexer ao
    mesmo tempo. }
  TAylisUser = class
  private
    FId: string;
    FSessions: TList;
  public
    Data: TJSONObject;
    Lock: TCriticalSection;
    constructor Create(const AId: string);
    destructor Destroy; override;
    { Roda Job(tela) na thread de cada tela do usuário (atualizar todas). }
    procedure Broadcast(Job: TAylisJob);
    property Id: string read FId;
  end;

  { O processo das sessões: lê o stdin e entrega cada mensagem à tela dela. }
  TAylisServer = class
  private
    FLock: TCriticalSection;
    FWriteLock: TCriticalSection;
    FSessions: TList;
    FUsers: TStringList;
    function UserOf(const Id: string): TAylisUser;
    procedure Leave(AUser: TAylisUser; Session: TAylisSession);
    procedure Forget(Session: TAylisSession);
  public
    constructor Create;
    destructor Destroy; override;
    procedure WriteLine(const Text: string);
    { Atende as telas da ponte com --sessions: uma instância de SessionClass por tela. }
    class procedure Serve(SessionClass: TAylisSessionClass);
  end;

{ Escreve no console do aylis-host (o stdout é do protocolo). }
procedure AylisLog(const Text: string);

type
  TAylisChanges = array of TJSONObject;

  { As alterações de um apply_updates, para o backend gravar do jeito dele, na ordem de
    gravar (o mestre incluído antes dos itens; as exclusões do mestre por último). Cada
    alteração é um TJSONObject: ref, op ('insert', 'update', 'delete'), data_set, key, old,
    new e, no item incluído sob um mestre também incluído, master. Ao aceitar o mestre com a
    chave gerada, os itens dele já recebem a chave antes de serem gravados:

      Delta := TAylisDelta.Create(Event);
      try
        for C in Delta.Inserted('cdsPedidos') do
          Delta.Accept(C, TJSONObject.Create(['id', InserirPedido(C)]));
        ...
        Aylis.Reply('results', Delta.ResultsJSON);
      finally
        Delta.Free;
      end;

    As alterações são cópias (o evento pode ser liberado); os Values passados a Accept passam
    a ser do delta. }
  TAylisDelta = class
  private
    FDataSet: string;
    FChanges: TJSONArray;
    FResults: TJSONArray;
    function GetCount: Integer;
    function GetChange(Index: Integer): TJSONObject;
    function Filter(const Op, ADataSet: string): TAylisChanges;
  public
    { Source: o evento (com args), os args (o objeto com changes) ou o texto JSON. }
    constructor Create(Source: TJSONData); overload;
    constructor Create(const JSON: string); overload;
    destructor Destroy; override;
    function Inserted(const ADataSet: string = ''): TAylisChanges;
    function Updated(const ADataSet: string = ''): TAylisChanges;
    function Deleted(const ADataSet: string = ''): TAylisChanges;
    function OfDataSet(const ADataSet: string): TAylisChanges;
    { Gravou; Values são os que o banco definiu (a chave gerada). }
    procedure Accept(Change: TJSONObject; Values: TJSONObject = nil);
    { Não gravou: a mensagem vai para a tela e o registro continua pendente. }
    procedure Reject(Change: TJSONObject; const Message: string);
    { Os resultados, para o Reply('results', ...) do on_apply_updates. }
    function ResultsJSON: string;
    property DataSet: string read FDataSet;
    property Count: Integer read GetCount;
    property Changes[Index: Integer]: TJSONObject read GetChange; default;
    property Results: TJSONArray read FResults;
  end;

const
  { O delta de exemplo dos selftests dos clientes (o mesmo em todas as linguagens). }
  AylisSampleDelta =
    '{"data_set":"cdsPedidos","changes":[' +
    '{"ref":1,"op":"insert","data_set":"cdsPedidos","key":{"id":-1},"new":{"id":-1,"cliente":"Joao"}},' +
    '{"ref":2,"op":"insert","data_set":"cdsItens","key":{"id":-2},"new":{"id":-2,"pedido_id":-1},' +
    '"master":{"data_set":"cdsPedidos","ref":1,"fields":{"pedido_id":"id"}}},' +
    '{"ref":3,"op":"update","data_set":"cdsItens","key":{"id":10},"old":{"produto":"A"},"new":{"produto":"B"}},' +
    '{"ref":4,"op":"delete","data_set":"cdsPedidos","key":{"id":7},"old":{"id":7}}]}';

{ O texto que todos os clientes escrevem ao conferir o TAylisDelta com o AylisSampleDelta. }
function AylisDeltaSelftest: string;

{ Registra a classe de um form (na initialization da unit dele): cada form com esse nome
  aberto na tela vira uma instância dela. Global, se dado, aponta para a primeira (o
  frmClientes do Delphi). }
procedure RegisterForm(AClass: TAylisFormClass; const Name: string; Global: PPointer = nil);

implementation

type
  { A thread de uma tela. }
  TSessionThread = class(TThread)
  private
    FSession: TAylisSession;
  protected
    procedure Execute; override;
  public
    constructor Create(Session: TAylisSession);
  end;

  { Um Post pendente. }
  TJobBox = class
    Job: TAylisJob;
  end;

  { Um Post pendente do TAylis (desktop). }
  TProcBox = class
    Job: TAylisProc;
    { O form dono (PostTo): liberado até lá, o Job não roda. 0: sem dono. }
    Owner: Int64;
  end;

threadvar
  { O Output desta thread já está em UTF-8. }
  OutputReady: Boolean;

constructor TSessionThread.Create(Session: TAylisSession);
begin
  FSession := Session;
  FreeOnTerminate := True;
  inherited Create(False);
end;

procedure TSessionThread.Execute;
begin
  // Output e StdErr são de cada thread no Free Pascal: UTF-8 aqui também.
  SetTextCodePage(Output, CP_UTF8);
  SetTextCodePage(StdErr, CP_UTF8);
  SetTextLineEnding(Output, #10);
  try
    FSession.Started;
  except
    on E: Exception do
      AylisLog('erro na sessão ' + IntToStr(FSession.Number) + ': ' + E.Message);
  end;
  try
    FSession.Ended;
  except
    on E: Exception do
      AylisLog('erro em Ended da sessão ' + IntToStr(FSession.Number) + ': ' + E.Message);
  end;
  FSession.Logout;
  FSession.Server.Forget(FSession);
end;

constructor TAylisSession.Create(AServer: TAylisServer; ANumber: Integer);
begin
  inherited Create;
  FServer := AServer;
  FNumber := ANumber;
  FLock := TCriticalSection.Create;
  FSignal := TEventObject.Create(nil, False, False, '');
  FInbox := TStringList.Create;
  FJobs := TList.Create;
end;

destructor TAylisSession.Destroy;
begin
  FJobs.Free;
  FInbox.Free;
  FSignal.Free;
  FLock.Free;
  inherited Destroy;
end;

procedure TAylisSession.WriteMessage(Msg: TJSONObject);
begin
  try
    Msg.Add('session', FNumber);
    FServer.WriteLine(Msg.AsJSON);
  finally
    Msg.Free;
  end;
end;

function TAylisSession.ReadMessage: TJSONObject;
var
  Line: string;
  Data: TJSONData;
begin
  repeat
    FLock.Enter;
    try
      if FInbox.Count > 0 then
      begin
        Line := FInbox[0];
        FInbox.Delete(0);
      end
      else
        Line := '';
    finally
      FLock.Leave;
    end;
    if Line = '' then
    begin
      FSignal.WaitFor(INFINITE);
      Continue;
    end;
    Data := GetJSON(Line);
    if Data is TJSONObject then
      Exit(TJSONObject(Data));
    Data.Free;
  until False;
end;

procedure TAylisSession.Deliver(const Line: string);
begin
  FLock.Enter;
  try
    FInbox.Add(Line);
  finally
    FLock.Leave;
  end;
  FSignal.SetEvent;
end;

procedure TAylisSession.Post(Job: TAylisJob);
var
  Box: TJobBox;
begin
  Box := TJobBox.Create;
  Box.Job := Job;
  FLock.Enter;
  try
    FJobs.Add(Box);
  finally
    FLock.Leave;
  end;
  Deliver('{"event": "__post"}');
end;

procedure TAylisSession.Special(Msg: TJSONObject);
var
  Box: TJobBox;
begin
  repeat
    FLock.Enter;
    try
      if FJobs.Count = 0 then
        Exit;
      Box := TJobBox(FJobs[0]);
      FJobs.Delete(0);
    finally
      FLock.Leave;
    end;
    try
      try
        Box.Job(Self);
      except
        on E: Exception do
          AylisLog('erro no post: ' + E.Message);
      end;
    finally
      Box.Free;
    end;
  until False;
end;

procedure TAylisSession.Ended;
begin
end;

procedure TAylisSession.Login(const Id: string);
begin
  Logout;
  FUser := FServer.UserOf(Id);
  FUser.Lock.Enter;
  try
    FUser.FSessions.Add(Self);
  finally
    FUser.Lock.Leave;
  end;
end;

procedure TAylisSession.Logout;
begin
  if FUser <> nil then
    FServer.Leave(FUser, Self);
  FUser := nil;
end;

constructor TAylisUser.Create(const AId: string);
begin
  inherited Create;
  FId := AId;
  FSessions := TList.Create;
  Data := TJSONObject.Create;
  Lock := TCriticalSection.Create;
end;

destructor TAylisUser.Destroy;
begin
  Lock.Free;
  Data.Free;
  FSessions.Free;
  inherited Destroy;
end;

procedure TAylisUser.Broadcast(Job: TAylisJob);
var
  I: Integer;
  List: array of TAylisSession;
begin
  Lock.Enter;
  try
    SetLength(List, FSessions.Count);
    for I := 0 to FSessions.Count - 1 do
      List[I] := TAylisSession(FSessions[I]);
  finally
    Lock.Leave;
  end;
  for I := 0 to High(List) do
    List[I].Post(Job);
end;

constructor TAylisServer.Create;
begin
  inherited Create;
  FLock := TCriticalSection.Create;
  FWriteLock := TCriticalSection.Create;
  FSessions := TList.Create;
  FUsers := TStringList.Create;
  FUsers.Sorted := True;
end;

destructor TAylisServer.Destroy;
begin
  FUsers.Free;
  FSessions.Free;
  FWriteLock.Free;
  FLock.Free;
  inherited Destroy;
end;

procedure TAylisServer.WriteLine(const Text: string);
begin
  FWriteLock.Enter;
  try
    Write(Output, Text, #10);
    Flush(Output);
  finally
    FWriteLock.Leave;
  end;
end;

function TAylisServer.UserOf(const Id: string): TAylisUser;
var
  I: Integer;
begin
  FLock.Enter;
  try
    I := FUsers.IndexOf(Id);
    if I < 0 then
      I := FUsers.AddObject(Id, TAylisUser.Create(Id));
    Result := TAylisUser(FUsers.Objects[I]);
  finally
    FLock.Leave;
  end;
end;

procedure TAylisServer.Leave(AUser: TAylisUser; Session: TAylisSession);
var
  Empty: Boolean;
  I: Integer;
begin
  AUser.Lock.Enter;
  try
    AUser.FSessions.Remove(Session);
    Empty := AUser.FSessions.Count = 0;
  finally
    AUser.Lock.Leave;
  end;
  if Empty then
  begin
    FLock.Enter;
    try
      I := FUsers.IndexOf(AUser.Id);
      if I >= 0 then
        FUsers.Delete(I);
    finally
      FLock.Leave;
    end;
  end;
end;

procedure TAylisServer.Forget(Session: TAylisSession);
begin
  FLock.Enter;
  try
    FSessions.Remove(Session);
  finally
    FLock.Leave;
  end;
end;

class procedure TAylisServer.Serve(SessionClass: TAylisSessionClass);
var
  Server: TAylisServer;
  Line, Kind: string;
  Data: TJSONData;
  Msg: TJSONObject;
  Number, I: Integer;
  Session, Found: TAylisSession;
begin
  DefaultSystemCodePage := CP_UTF8;
  SetTextCodePage(Input, CP_UTF8);
  SetTextCodePage(Output, CP_UTF8);
  SetTextCodePage(StdErr, CP_UTF8);
  SetTextLineEnding(Output, #10);
  Server := TAylisServer.Create;
  while not EOF(Input) do
  begin
    ReadLn(Input, Line);
    if Trim(Line) = '' then
      Continue;
    Data := GetJSON(Line);
    try
      if not (Data is TJSONObject) then
        Continue;
      Msg := TJSONObject(Data);
      if (Msg.Find('session') = nil) or (Msg.Find('session').JSONType <> jtNumber) then
        Continue;
      Number := Msg.Integers['session'];
      Msg.Delete('session');
      Kind := Msg.Get('event', '');
      if Kind = 'session_open' then
      begin
        Session := SessionClass.Create(Server, Number);
        Server.FLock.Enter;
        try
          Server.FSessions.Add(Session);
        finally
          Server.FLock.Leave;
        end;
        TSessionThread.Create(Session);
        Continue;
      end;
      Found := nil;
      Server.FLock.Enter;
      try
        for I := 0 to Server.FSessions.Count - 1 do
          if TAylisSession(Server.FSessions[I]).Number = Number then
            Found := TAylisSession(Server.FSessions[I]);
      finally
        Server.FLock.Leave;
      end;
      if Found = nil then
        Continue;
      if Kind = 'session_close' then
        Found.Deliver('{"event": "closed"}')
      else
        Found.Deliver(Msg.AsJSON);
    finally
      Data.Free;
    end;
  end;
end;

type
  THandlerBox = class
    Handler: TAylisHandler;
  end;

procedure AylisLog(const Text: string);
begin
  WriteLn(StdErr, '[backend] ', Text);
  Flush(StdErr);
end;

{ ---------------------------------------------------------------------------
  TAylisDelta }

constructor TAylisDelta.Create(Source: TJSONData);
var
  Value, Parsed, Item: TJSONData;
  List: TJSONArray;
  Change: TJSONObject;
  I: Integer;
begin
  inherited Create;
  FChanges := TJSONArray.Create;
  FResults := TJSONArray.Create;
  Parsed := nil;
  Value := Source;
  if (Value is TJSONObject) and (TJSONObject(Value).Find('args') <> nil) then
    Value := TJSONObject(Value).Find('args');
  if Value is TJSONString then
  begin
    Parsed := GetJSON(Value.AsString);
    Value := Parsed;
  end;
  try
    List := nil;
    if Value is TJSONObject then
    begin
      FDataSet := TJSONObject(Value).Get('data_set', '');
      if TJSONObject(Value).Find('changes') is TJSONArray then
        List := TJSONArray(TJSONObject(Value).Find('changes'));
    end
    else if Value is TJSONArray then
      List := TJSONArray(Value);
    if List <> nil then
      for I := 0 to List.Count - 1 do
      begin
        Item := List[I];
        if not (Item is TJSONObject) then
          Continue;
        Change := TJSONObject(Item.Clone);
        if not (Change.Find('key') is TJSONObject) then
          Change.Add('key', TJSONObject.Create);
        if not (Change.Find('new') is TJSONObject) then
          Change.Add('new', TJSONObject.Create);
        FChanges.Add(Change);
      end;
  finally
    Parsed.Free;
  end;
end;

constructor TAylisDelta.Create(const JSON: string);
var
  Data: TJSONData;
begin
  Data := GetJSON(JSON);
  try
    Create(Data);
  finally
    Data.Free;
  end;
end;

destructor TAylisDelta.Destroy;
begin
  FChanges.Free;
  FResults.Free;
  inherited Destroy;
end;

function TAylisDelta.GetCount: Integer;
begin
  Result := FChanges.Count;
end;

function TAylisDelta.GetChange(Index: Integer): TJSONObject;
begin
  Result := FChanges.Objects[Index];
end;

function TAylisDelta.Filter(const Op, ADataSet: string): TAylisChanges;
var
  I, N: Integer;
  C: TJSONObject;
begin
  Result := nil;
  N := 0;
  for I := 0 to FChanges.Count - 1 do
  begin
    C := FChanges.Objects[I];
    if ((Op = '') or (C.Get('op', '') = Op)) and ((ADataSet = '') or (C.Get('data_set', '') = ADataSet)) then
    begin
      SetLength(Result, N + 1);
      Result[N] := C;
      Inc(N);
    end;
  end;
end;

function TAylisDelta.Inserted(const ADataSet: string): TAylisChanges;
begin
  Result := Filter('insert', ADataSet);
end;

function TAylisDelta.Updated(const ADataSet: string): TAylisChanges;
begin
  Result := Filter('update', ADataSet);
end;

function TAylisDelta.Deleted(const ADataSet: string): TAylisChanges;
begin
  Result := Filter('delete', ADataSet);
end;

function TAylisDelta.OfDataSet(const ADataSet: string): TAylisChanges;
begin
  Result := Filter('', ADataSet);
end;

procedure TAylisDelta.Accept(Change: TJSONObject; Values: TJSONObject);
var
  Ref: Int64;
  I, J: Integer;
  Other, Link, Fields, Part: TJSONObject;
  Detail, Master, PartName: string;
begin
  Ref := Change.Get('ref', Int64(0));
  if Values = nil then
    Values := TJSONObject.Create;
  for I := 0 to FChanges.Count - 1 do
  begin
    Other := FChanges.Objects[I];
    if not (Other.Find('master') is TJSONObject) then
      Continue;
    Link := TJSONObject(Other.Find('master'));
    if Link.Get('ref', Int64(-1)) <> Ref then
      Continue;
    if not (Link.Find('fields') is TJSONObject) then
      Continue;
    Fields := TJSONObject(Link.Find('fields'));
    for J := 0 to Fields.Count - 1 do
    begin
      Detail := Fields.Names[J];
      Master := Fields.Items[J].AsString;
      if Values.Find(Master) = nil then
        Continue;
      for PartName in ['new', 'key'] do
      begin
        Part := TJSONObject(Other.Find(PartName));
        if (Part <> nil) and (Part.Find(Detail) <> nil) then
          Part.Elements[Detail] := Values.Find(Master).Clone;
      end;
    end;
  end;
  if Values.Count = 0 then
  begin
    Values.Free;
    FResults.Add(TJSONObject.Create(['ref', Ref]));
  end
  else
    FResults.Add(TJSONObject.Create(['ref', Ref, 'values', Values]));
end;

procedure TAylisDelta.Reject(Change: TJSONObject; const Message: string);
begin
  FResults.Add(TJSONObject.Create(['ref', Change.Get('ref', Int64(0)), 'error', Message]));
end;

function TAylisDelta.ResultsJSON: string;
begin
  Result := FResults.AsJSON;
end;

function AylisDeltaSelftest: string;
var
  Delta: TAylisDelta;
  Item: TJSONObject;
begin
  Delta := TAylisDelta.Create(AylisSampleDelta);
  try
    Delta.Accept(Delta.Inserted('cdsPedidos')[0], TJSONObject.Create(['id', 501]));
    Item := TJSONObject(Delta.Inserted('cdsItens')[0].Find('new'));
    Result := Format('delta %d %d %d %s %d', [Length(Delta.Inserted), Length(Delta.Updated), Length(Delta.Deleted),
      Item.Find('pedido_id').AsString, Delta.Results.Count]);
  finally
    Delta.Free;
  end;
end;

{ ---------------------------------------------------------------------------
  Forms com instância }

type
  { Uma classe de form registrada: a classe e a variável global da primeira instância. }
  TFormClassBox = class
    FormClass: TAylisFormClass;
    Global: PPointer;
  end;

var
  FormClasses: TStringList;

procedure RegisterForm(AClass: TAylisFormClass; const Name: string; Global: PPointer);
var
  Box: TFormClassBox;
begin
  if FormClasses = nil then
  begin
    FormClasses := TStringList.Create;
    FormClasses.OwnsObjects := True;
    FormClasses.CaseSensitive := False;
  end;
  Box := TFormClassBox.Create;
  Box.FormClass := AClass;
  Box.Global := Global;
  FormClasses.AddObject(Name, Box);
end;

function FormClassBox(const Name: string): TFormClassBox;
var
  I: Integer;
begin
  Result := nil;
  if FormClasses = nil then
    Exit;
  I := FormClasses.IndexOf(Name);
  if I >= 0 then
    Result := TFormClassBox(FormClasses.Objects[I]);
end;

constructor TAylisComponent.Create(AAylis: TAylis; AId: Int64; const AName: string);
begin
  inherited Create;
  FAylis := AAylis;
  FId := AId;
  FName := AName;
end;

function TAylisComponent.GetProp(const Prop: string): TJSONData;
begin
  Result := FAylis.Call('get', TJSONObject.Create(['target', FId, 'prop', Prop]));
end;

procedure TAylisComponent.SetProp(const Prop: string; Value: TJSONData);
begin
  FAylis.Send('set', TJSONObject.Create(['target', FId, 'props', TJSONObject.Create([Prop, Value])]));
end;

function TAylisComponent.Invoke(const Method: string; Args: TJSONArray): TJSONData;
begin
  Result := FAylis.Call('invoke', TJSONObject.Create(['target', FId, 'method', Method, 'args', Args]));
end;

procedure TAylisComponent.SetFocus;
begin
  FAylis.Send('focus', TJSONObject.Create(['target', FId]));
end;

constructor TAylisForm.Create(AAylis: TAylis; AId: Int64; const AName: string);
begin
  inherited Create(AAylis, AId, AName);
  FComponents := TList.Create;
end;

destructor TAylisForm.Destroy;
var
  I: Integer;
begin
  for I := 0 to FComponents.Count - 1 do
    TObject(FComponents[I]).Free;
  FComponents.Free;
  inherited Destroy;
end;

procedure TAylisForm.BindComponents(List: TJSONArray);
begin
end;

function FindIn(List: TJSONArray; const AName: string): TJSONObject;
var
  I: Integer;
begin
  Result := nil;
  for I := 0 to List.Count - 1 do
    if TJSONObject(List[I]).Get('name', '') = AName then
      Exit(TJSONObject(List[I]));
end;

function TAylisForm.Resolve(List: TJSONArray; const AName, AClass: string; AType: TAylisComponentClass): TAylisComponent;
var
  Item: TJSONObject;
begin
  Item := FindIn(List, AName);
  if Item = nil then
    raise EAylis.CreateFmt('o form %s não bate com o código: falta o componente %s (%s)', [Name, AName, AClass]);
  if Item.Get('class', '') <> AClass then
    raise EAylis.CreateFmt('o form %s não bate com o código: %s é %s no form e %s no código', [Name, AName, Item.Get('class', ''), AClass]);
  Result := AType.Create(Aylis, Item.Get('id', Int64(0)), AName);
  FComponents.Add(Result);
end;

function TAylisForm.ResolveOptional(List: TJSONArray; const AName: string; AType: TAylisComponentClass): TAylisComponent;
var
  Item: TJSONObject;
begin
  Result := nil;
  Item := FindIn(List, AName);
  if Item = nil then
    Exit;
  Result := AType.Create(Aylis, Item.Get('id', Int64(0)), AName);
  FComponents.Add(Result);
end;

function TAylisForm.ComponentById(AId: Int64): TAylisComponent;
var
  I: Integer;
begin
  if AId = Id then
    Exit(Self);
  for I := 0 to FComponents.Count - 1 do
    if TAylisComponent(FComponents[I]).Id = AId then
      Exit(TAylisComponent(FComponents[I]));
  Result := nil;
end;

procedure TAylisForm.Show;
begin
  Aylis.Send('show', TJSONObject.Create(['target', Id]));
end;

procedure TAylisForm.Close;
begin
  Aylis.Send('close', TJSONObject.Create(['target', Id]));
end;

{ A instância do form da tela (cria e monta na primeira vez); nil se a classe dele não foi
  registrada. Um form que não bate com o código levanta EAylis. }
function TAylis.FormInstance(const Form: string; FormId: Int64): TAylisForm;
var
  I: Integer;
  Box: TFormClassBox;
  List: TJSONData;
begin
  for I := 0 to FForms.Count - 1 do
    if TAylisForm(FForms[I]).Id = FormId then
      Exit(TAylisForm(FForms[I]));
  Result := nil;
  Box := FormClassBox(Form);
  if (Box = nil) or (FormId < 0) then
    Exit;
  Result := Box.FormClass.Create(Self, FormId, Form);
  try
    List := Call('components', TJSONObject.Create(['target', FormId]));
    Result.BindComponents(TJSONArray(List));
  except
    Result.Free;
    raise;
  end;
  FForms.Add(Result);
  if (Box.Global <> nil) and (Box.Global^ = nil) then
    Box.Global^ := Result;
end;

{ O evento vai para o método published da instância do form, se a classe dele está
  registrada. Devolve se tratou. }
function TAylis.DispatchToForm(Msg: TJSONObject): Boolean;
var
  Form, Handler: string;
  Instance: TAylisForm;
  Method: TMethod;
  Sender: TAylisComponent;
  SenderData: TJSONObject;
begin
  Form := Msg.Get('form', '');
  Instance := FormInstance(Form, Msg.Get('form_id', Int64(-1)));
  if Instance = nil then
    Exit(False);
  Result := True;
  Handler := Msg.Get('handler', '');
  Method.Code := Instance.MethodAddress(Handler);
  if Method.Code = nil then
  begin
    AylisLog('o form ' + Form + ' não tem o handler ' + Handler + ' (um método published de ' + Instance.ClassName + ')');
    Exit;
  end;
  Method.Data := Instance;
  Sender := nil;
  SenderData := TJSONObject(Msg.Find('sender'));
  if SenderData <> nil then
    Sender := Instance.ComponentById(SenderData.Get('id', Int64(-1)));
  if Sender = nil then
    Sender := Instance;
  CallFormMethod(Method, Sender, Msg);
end;

{ Chama o método do form com a assinatura do evento: os que perguntam com var, como no
  Delphi (o que o handler mudar vira a resposta); os outros com (Sender, Event). }
procedure TAylis.CallFormMethod(Method: TMethod; Sender: TAylisComponent; Msg: TJSONObject);
var
  Kind, Key, Original, Data, Caption: string;
  CanClose, Accept, Cancel: Boolean;
  Args: TJSONData;
begin
  Kind := Msg.Get('event', '');
  Args := Msg.Find('args');
  if (Kind = 'on_intercept') and (Args is TJSONObject) then
  begin
    Cancel := False;
    TAylisInterceptEvent(Method)(Sender, TJSONObject(Args), Cancel);
    if Cancel then
      Reply('cancel', True);
  end
  else if Kind = 'on_close_query' then
  begin
    CanClose := True;
    TAylisCloseQueryEvent(Method)(Sender, CanClose);
    if not CanClose then
      Reply('can_close', False);
  end
  else if Kind = 'on_key_down' then
  begin
    Key := '';
    if Args is TJSONObject then
      Key := TJSONObject(Args).Get('key', '');
    Original := Key;
    TAylisKeyEvent(Method)(Sender, Key, Msg);
    if Key <> Original then
      Reply('key', Key);
  end
  else if Kind = 'on_key_press' then
  begin
    Key := '';
    if (Args <> nil) and (Args.JSONType = jtString) then
      Key := Args.AsString;
    Original := Key;
    TAylisKeyPressEvent(Method)(Sender, Key);
    if Key <> Original then
      Reply('key', Key);
  end
  else if Kind = 'on_drag_start' then
  begin
    Data := '';
    Caption := '';
    TAylisDragStartEvent(Method)(Sender, Data, Caption);
    { Sem dados, vale o drag_data do componente (para cancelar, raise). }
    if Data <> '' then
    begin
      Reply('data', Data);
      Reply('label', Caption);
    end;
  end
  else if Kind = 'on_drag_over' then
  begin
    Data := '';
    if (Args <> nil) and (Args.JSONType = jtString) then
      Data := Args.AsString;
    Accept := True;
    TAylisDragOverEvent(Method)(Sender, Data, Accept);
    if not Accept then
      Reply('accept', False);
  end
  else
    TAylisFormEvent(Method)(Sender, Msg);
end;

procedure TAylis.Reply(const Name: string; Value: Boolean);
begin
  if FReply = nil then
    Exit;
  FReply.Delete(Name);
  FReply.Add(Name, Value);
end;

procedure TAylis.Reply(const Name, Value: string);
begin
  if FReply = nil then
    Exit;
  FReply.Delete(Name);
  FReply.Add(Name, Value);
end;

function TAylis.FieldText(const DataSet, Field: string): string;
var
  Got: TJSONData;
begin
  Got := Call('data', TJSONObject.Create(['target', DataSet, 'op', 'value', 'field', Field]));
  if (Got = nil) or (Got.JSONType = jtNull) then
    Result := '0'
  else
    Result := Got.AsString;
end;

function TAylis.FieldAsCurrency(const DataSet, Field: string): Currency;
var
  Fmt: TFormatSettings;
begin
  Fmt := DefaultFormatSettings;
  Fmt.DecimalSeparator := '.';
  Fmt.ThousandSeparator := #0;
  Result := StrToCurr(FieldText(DataSet, Field), Fmt);
end;

function TAylis.FieldAsBCD(const DataSet, Field: string): TBCD;
var
  Fmt: TFormatSettings;
begin
  Fmt := DefaultFormatSettings;
  Fmt.DecimalSeparator := '.';
  Fmt.ThousandSeparator := #0;
  Result := StrToBCD(FieldText(DataSet, Field), Fmt);
end;

function TAylis.WaitFor(const Cmd: string; Fields: TJSONObject): string;
var
  Got: TJSONData;
  I: Integer;
begin
  Fields.Add('wait', True);
  Inc(FWaiting);
  try
    { Como o laço do ShowModal do Delphi: o que chegou antes (enquanto outro Call esperava)
      roda já, sem esperar o diálogo fechar. }
    for I := 1 to FQueued.Count do
      HandleNested(TJSONObject(FQueued.Pop));
    Got := Call(Cmd, Fields);
    if (Got <> nil) and (Got.JSONType = jtString) then
      Result := Got.AsString
    else
      Result := '';
  finally
    Dec(FWaiting);
  end;
end;

function TAylis.MessageDlg(const Text: string; const Kind: string; const Buttons: array of string): string;
var
  List: TJSONArray;
  I: Integer;
begin
  List := TJSONArray.Create;
  for I := 0 to High(Buttons) do
    List.Add(Buttons[I]);
  if List.Count = 0 then
    List.Add('ok');
  Result := WaitFor('message_dlg', TJSONObject.Create(['text', Text, 'kind', Kind, 'buttons', List]));
end;

function TAylis.ShowModal(Form: TAylisForm): string;
begin
  Result := WaitFor('show_modal', TJSONObject.Create(['target', Form.Id]));
end;

function TAylis.CreateForm(const Form: string; Another: Boolean): TAylisForm;
var
  Created: TJSONData;
begin
  Created := Call('create_form', TJSONObject.Create(['name', Form, 'new', Another]));
  Result := FormInstance(Form, TJSONObject(Created).Get('id', Int64(-1)));
end;

function TAylis.CreateForm(FormClass: TAylisFormClass; Another: Boolean): TAylisForm;
var
  I: Integer;
begin
  if FormClasses <> nil then
    for I := 0 to FormClasses.Count - 1 do
      if TFormClassBox(FormClasses.Objects[I]).FormClass = FormClass then
        Exit(CreateForm(FormClasses[I], Another));
  raise EAylis.Create('a classe ' + FormClass.ClassName + ' não foi registrada (RegisterForm)');
end;

constructor TAylis.Create;
begin
  inherited Create;
  // Tudo em UTF-8 (o JSON do protocolo), sem conversão para a página de código.
  DefaultSystemCodePage := CP_UTF8;
  SetTextCodePage(Input, CP_UTF8);
  SetTextCodePage(Output, CP_UTF8);
  SetTextCodePage(StdErr, CP_UTF8);
  SetTextLineEnding(Output, #10);
  FHandlers := TStringList.Create;
  FHandlers.OwnsObjects := True;
  FHandlers.Sorted := True;
  FQueued := TObjectQueue.Create;
  FFreed := TList.Create;
  FWriteLock := TCriticalSection.Create;
  FPostLock := TCriticalSection.Create;
  FPosted := TList.Create;
  FForms := TList.Create;
end;

destructor TAylis.Destroy;
var
  I: Integer;
begin
  while FQueued.Count > 0 do
    FQueued.Pop.Free;
  FQueued.Free;
  FFreed.Free;
  for I := 0 to FPosted.Count - 1 do
    TObject(FPosted[I]).Free;
  FPosted.Free;
  FPostLock.Free;
  FWriteLock.Free;
  for I := 0 to FForms.Count - 1 do
    TObject(FForms[I]).Free;
  FForms.Free;
  FHandlers.Free;
  FLastResult.Free;
  inherited Destroy;
end;

procedure TAylis.WriteMessage(Msg: TJSONObject);
var
  Line: string;
begin
  try
    Line := Msg.AsJSON;
  finally
    Msg.Free;
  end;
  // Uma linha inteira por vez: um Send de outra thread não se mistura com o desta.
  FWriteLock.Enter;
  try
    // O Output é de cada thread no Free Pascal: o de uma TThread nova também em UTF-8.
    if not OutputReady then
    begin
      SetTextCodePage(Output, CP_UTF8);
      OutputReady := True;
    end;
    Write(Output, Line, #10);
    Flush(Output);
  finally
    FWriteLock.Leave;
  end;
end;

procedure TAylis.PostTo(Owner: TAylisForm; Job: TAylisProc);
var
  Box: TProcBox;
begin
  Box := TProcBox.Create;
  Box.Job := Job;
  Box.Owner := Owner.Id;
  FPostLock.Enter;
  try
    FPosted.Add(Box);
  finally
    FPostLock.Leave;
  end;
  Send('wake');
end;

procedure TAylis.Post(Job: TAylisProc);
var
  Box: TProcBox;
begin
  Box := TProcBox.Create;
  Box.Job := Job;
  FPostLock.Enter;
  try
    FPosted.Add(Box);
  finally
    FPostLock.Leave;
  end;
  // O Loop está parado lendo a entrada: a tela devolve {"event": "__post"} e ele acorda.
  Send('wake');
end;

procedure TAylis.RunPosted;
var
  Box: TProcBox;
begin
  repeat
    FPostLock.Enter;
    try
      if FPosted.Count = 0 then
        Exit;
      Box := TProcBox(FPosted[0]);
      FPosted.Delete(0);
    finally
      FPostLock.Leave;
    end;
    try
      try
        { O dono saiu: o trabalho é descartado. }
        if (Box.Owner = 0) or (FFreed.IndexOf(Pointer(PtrInt(Box.Owner))) < 0) then
          Box.Job();
      except
        on E: Exception do
          AylisLog('erro no post: ' + E.Message);
      end;
    finally
      Box.Free;
    end;
  until False;
end;

function TAylis.ReadMessage: TJSONObject;
var
  Line: string;
  Data: TJSONData;
begin
  Result := nil;
  while not EOF(Input) do
  begin
    ReadLn(Input, Line);
    if Trim(Line) = '' then
      Continue;
    Data := GetJSON(Line);
    if Data is TJSONObject then
      Exit(TJSONObject(Data));
    Data.Free;
  end;
end;

function TAylis.MakeMessage(const Cmd: string; Fields: TJSONObject): TJSONObject;
var
  I: Integer;
begin
  Result := TJSONObject.Create(['cmd', Cmd]);
  if Fields <> nil then
  begin
    for I := 0 to Fields.Count - 1 do
      Result.Add(Fields.Names[I], Fields.Items[I].Clone);
    Fields.Free;
  end;
end;

procedure TAylis.Special(Msg: TJSONObject);
begin
end;

procedure TAylis.Greet;
begin
  // A primeira mensagem para a tela: a versão do protocolo (a tela recusa tudo antes).
  if FGreeted then
    Exit;
  FGreeted := True;
  WriteMessage(MakeMessage('hello', TJSONObject.Create(['protocol', AylisProtocol, 'client', 'pascal', 'features', TJSONArray.Create(['done'])])));
end;

procedure TAylis.Send(const Cmd: string; Fields: TJSONObject);
begin
  Greet;
  WriteMessage(MakeMessage(Cmd, Fields));
end;

procedure TAylis.ProfileBegin(const Name: string);
begin
  Send('profile_begin', TJSONObject.Create(['name', Name]));
end;

procedure TAylis.ProfileEnd;
begin
  Send('profile_end');
end;

function TAylis.Call(const Cmd: string; Fields: TJSONObject): TJSONData;
var
  Msg, Got: TJSONObject;
  Id: Integer;
  IdData: TJSONData;
begin
  // A resposta chega pela leitura da thread dos handlers: de outra thread, as duas leriam a
  // mesma entrada. A primeira thread que lê é a dona (numa sessão, a thread da tela).
  if FOwner = TThreadID(0) then
    FOwner := GetCurrentThreadId
  else if FOwner <> GetCurrentThreadId then
  begin
    Fields.Free;
    raise EAylis.Create('Call(''' + Cmd + ''') de outra thread: leia na thread dos handlers (Post) ou antes de iniciar a thread');
  end;
  Greet;
  Inc(FNextId);
  Id := FNextId;
  Msg := MakeMessage(Cmd, Fields);
  Msg.Add('id', Id);
  WriteMessage(Msg);
  repeat
    Got := ReadMessage;
    if Got = nil then
      raise EAylis.Create('a tela foi fechada');
    IdData := Got.Find('id');
    if (IdData <> nil) and (IdData.JSONType = jtNumber) and (IdData.AsInteger = Id) then
    begin
      try
        if Got.Find('error') <> nil then
          raise EAylis.Create(Got.Get('error', ''));
        FreeAndNil(FLastResult);
        if Got.Find('result') <> nil then
          FLastResult := Got.Find('result').Clone
        else
          FLastResult := TJSONNull.Create;
        Exit(FLastResult);
      finally
        Got.Free;
      end;
    end;
    { Um evento-pergunta roda já, mesmo esperando: o Post do próprio backend dispara o
      before_post, e a tela espera a resposta dele antes de responder ao Post. Esperando um
      diálogo (MessageDlg, ShowModal), todos os eventos rodam já (os do modal e os Post). }
    if (Got.Find('event') <> nil) and (FAskDepth < 8) and ((FWaiting > 0) or (Got.Find('ask') <> nil))
      and (Got.Get('event', '') <> 'closed') and ((Got.Get('event', '') <> 'ping') or (FWaiting > 0)) then
    begin
      HandleNested(Got);
      Continue;
    end;
    FQueued.Push(Got);
  until False;
end;

procedure TAylis.OnEvent(const Name: string; Handler: TAylisHandler);
var
  Box: THandlerBox;
  I: Integer;
begin
  Box := THandlerBox.Create;
  Box.Handler := Handler;
  I := FHandlers.IndexOf(Name);
  if I >= 0 then
    FHandlers.Delete(I);
  FHandlers.AddObject(Name, Box);
end;

procedure TAylis.Run(const Form: string);
var
  Fields: TJSONObject;
  Found: TJSONData;
begin
  // Sem form: o principal do projeto (aylis-host --project).
  Fields := TJSONObject.Create;
  if Form = '' then
    Fields.Add('form', TJSONNull.Create)
  else
    Fields.Add('form', Form);
  Call('run', Fields);
  { A instância do form principal já existe antes do primeiro evento (o código pode usar a
    variável global dele, como no Delphi). }
  if Form <> '' then
  begin
    Found := Call('find', TJSONObject.Create(['name', Form]));
    if (Found <> nil) and (Found.JSONType = jtNumber) then
      FormInstance(Form, Found.AsInt64);
  end;
  Loop;
end;

procedure TAylis.Loop;
var
  Msg: TJSONObject;
  Kind: string;
begin
  repeat
    if FQueued.Count > 0 then
      Msg := TJSONObject(FQueued.Pop)
    else
      Msg := ReadMessage;
    if Msg = nil then
      Exit;
    try
      Kind := Msg.Get('event', '');
      if Kind = '' then
        Continue;
      if Kind = 'closed' then
        Exit;
      if Kind = '__post' then
      begin
        RunPosted;
        Special(Msg);
        Continue;
      end;
      { Sincronia (roteiros de teste): responde depois de tratar tudo que veio antes. }
      if Kind = '__freed' then
      begin
        FormsFreed(Msg);
        Continue;
      end;
      if Kind = 'ping' then
      begin
        Send('pong');
        Continue;
      end;
      if Kind = 'error' then
      begin
        AylisLog('erro na tela: ' + Msg.Get('message', ''));
        Continue;
      end;
      HandleEvent(Msg);
    finally
      Msg.Free;
    end;
  until False;
end;

{ Roda o handler do evento. Um evento-pergunta (com "ask") recebe a resposta quando o
  handler termina: os campos gravados (Reply, os var) ou, com exceção, a recusa com a
  mensagem (a tela mostra), como no BeforePost do Delphi. }
{ Os forms liberados na tela: as instâncias saem (a variável global que apontava para uma
  passa a nil) e os PostTo com eles como dono não rodam mais. }
procedure TAylis.FormsFreed(Msg: TJSONObject);
var
  Ids: TJSONArray;
  I, J, K: Integer;
  Id: Int64;
  Box: TFormClassBox;
begin
  if not (Msg.Find('forms') is TJSONArray) then
    Exit;
  Ids := TJSONArray(Msg.Find('forms'));
  for I := 0 to Ids.Count - 1 do
  begin
    Id := Ids.Items[I].AsInt64;
    FFreed.Add(Pointer(PtrInt(Id)));
    for J := FForms.Count - 1 downto 0 do
      if TAylisForm(FForms[J]).Id = Id then
      begin
        if FormClasses <> nil then
          for K := 0 to FormClasses.Count - 1 do
          begin
            Box := TFormClassBox(FormClasses.Objects[K]);
            if (Box.Global <> nil) and (Box.Global^ = Pointer(FForms[J])) then
              Box.Global^ := nil;
          end;
        TAylisForm(FForms[J]).Free;
        FForms.Delete(J);
      end;
  end;
end;

{ Uma mensagem que roda já, no meio de um Call (um evento-pergunta, ou qualquer uma esperando
  um diálogo): o Post, a sincronia dos roteiros e os eventos. Libera a mensagem. }
procedure TAylis.HandleNested(Got: TJSONObject);
begin
  Inc(FAskDepth);
  try
    if Got.Get('event', '') = '__post' then
    begin
      RunPosted;
      Special(Got);
    end
    else if Got.Get('event', '') = 'ping' then
      { A sincronia dos roteiros: o que veio antes já rodou. }
      Send('pong')
    else if Got.Get('event', '') = '__freed' then
      FormsFreed(Got)
    else if Got.Get('event', '') = 'error' then
      AylisLog('erro na tela: ' + Got.Get('message', ''))
    else if Got.Get('event', '') = 'closed' then
      FQueued.Push(Got.Clone)
    else if Got.Find('event') <> nil then
      HandleEvent(Got);
  finally
    Dec(FAskDepth);
    Got.Free;
  end;
end;

procedure TAylis.HandleEvent(Msg: TJSONObject);
var
  Name, Form: string;
  I: Integer;
  Ask: TJSONData;
  Saved, Answer: TJSONObject;
  Handled: Boolean;
begin
  Name := Msg.Get('handler', '');
  Form := Msg.Get('form', '');
  Ask := Msg.Find('ask');
  Saved := FReply;
  if Ask <> nil then
    FReply := TJSONObject.Create
  else
    FReply := nil;
  try
    try
      { O form com classe registrada: o método da instância dele. }
      Handled := DispatchToForm(Msg);
      if not Handled then
      begin
        I := FHandlers.IndexOf(Form + '.' + Name);
        if I < 0 then
          I := FHandlers.IndexOf(Name);
        if I >= 0 then
          THandlerBox(FHandlers.Objects[I]).Handler(Self, Msg)
        else
          AylisLog('evento sem handler: ' + Name);
      end;
      if Ask <> nil then
      begin
        Answer := TJSONObject.Create(['ask', Ask.AsInt64]);
        Answer.Add('values', FReply);
        FReply := nil;
        Send('answer', Answer);
      end;
    except
      on E: Exception do
      begin
        if Ask <> nil then
          { Recusado: a mensagem vai para a tela (vazia, recusa calado). }
          Send('answer', TJSONObject.Create(['ask', Ask.AsInt64, 'error', E.Message]))
        else
        begin
          AylisLog('erro no handler ' + Name + ': ' + E.Message);
          { A tela mostra o erro ao usuário (como o Application.HandleException do Delphi):
            sem isto, o clique em Salvar falharia sem ninguém saber. }
          Send('exception', TJSONObject.Create(['message', E.Message, 'path', Form, 'handler', Name]));
        end;
      end;
    end;
  finally
    FReply.Free;
    FReply := Saved;
    { O handler terminou: a tela deixa de mostrar o backend ocupado. }
    if Msg.Find('seq') <> nil then
      Send('done', TJSONObject.Create(['seq', Msg.Find('seq').AsInt64]));
  end;
end;

end.
