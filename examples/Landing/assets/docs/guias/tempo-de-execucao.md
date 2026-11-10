# Em tempo de execução: componentes, propriedades, eventos e dados

Tudo passa pelo `Ui`. Um componente é identificado por um `ComponentId` (um
"ponteiro seguro": depois de liberado, o id antigo não aponta para mais nada e as
consultas devolvem `None` ou erro).

## Achar componentes

```rust
// Pelo nome, dentro de um form (os nomes são únicos por form).
let form = ui.find("frmClientes").unwrap();
let edt = ui.find_in(form, "edtNome").unwrap();

// Por caminho: form.componente
let btn = ui.find_path("frmClientes.btnSalvar").unwrap();

// Navegar na árvore
let pai = ui.parent(edt);
for &filho in ui.children(form) { /* ... */ }
let form_do_controle = ui.form_of(btn);
let classe = ui.class_name(btn);            // Some("Button")
```

Dentro de um handler de evento, o `sender` é o id de quem disparou, e o form vem de
`ui.form_of(sender)`.

## Ler e trocar propriedades

Há dois jeitos, e os dois valem para qualquer componente.

### Pelo nome (como o Object Inspector, o JSON e o protocolo remoto)

```rust
ui.set_prop(edt, "text", "Maria")?;
ui.set_prop(btn, "enabled", false)?;
ui.set_prop(painel, "align", PropValue::Enum("left".into()))?;  // enum: pelo nome
ui.set_prop(lbl, "color", Color::rgb(220, 38, 38))?;
ui.set_prop(btn, "style", "danger")?;                            // subtipo (variante)
ui.set_prop(painel, "theme", "Aylis Dark")?;                       // tema do painel e dos filhos
ui.set_prop(btn, "hint", "Salva o cadastro")?;                    // dica

let texto = ui.get_prop(edt, "text")?;          // PropValue::String("Maria")
let lista = ui.prop_infos(edt);                 // nome, tipo e categoria de cada propriedade
```

`set_prop` valida o tipo (`Error::TypeMismatch`) e o nome (`Error::UnknownProperty`),
repinta e refaz o layout quando precisa, e avisa o componente (`property_changed`).
Números convertem entre si (`10`, `10.0`, `10i64`).

### Pelo tipo (Rust, com o compilador conferindo)

```rust
if let Some(e) = ui.get_mut::<Edit>(edt) {
    e.text = "Maria".into();
    e.placeholder = "Nome completo".into();
}
let nota = ui.get::<NumberBox>(nb).map(|n| n.value);

// Posição, tamanho, alinhamento, visibilidade... (o ControlBase de qualquer controle)
ui.control_mut(btn).unwrap().set_visible(false).set_width(120.0);
ui.invalidate();      // depois de mudar campos que afetam o layout direto na struct
```

Mudando os campos direto, o `Ui` não fica sabendo. Chame `ui.invalidate()` para refazer o
layout ou `ui.request_repaint()` para só repintar. O `set_prop` faz isso sozinho.

## Criar e liberar em tempo de execução

```rust
let b = ui.add(Button::new("Novo"), painel);                 // pelo tipo
let c = ui.create("Button.icon", Some(painel))?;             // pelo nome (com subtipo)
ui.control_mut(b).unwrap().set_align(Align::Left).set_width(100.0);
ui.set_name(b, "btnNovo")?;
ui.set_parent(b, Some(outro_painel))?;                       // mudar de pai
ui.free(b);                                                  // libera ele e os filhos
```

## Eventos

```rust
// Clique
ui.on_click(btn, |ui, sender| {
    let form = ui.form_of(sender);
    let nome = ui.find_in(form, "edtNome").and_then(|e| ui.get::<Edit>(e)).map(|e| e.text.clone());
    toast(ui, &format!("Salvo: {}", nome.unwrap_or_default()));
});

// Qualquer evento, com os argumentos
ui.on(edt, "on_change", |ui, sender, args| {
    if let EventArgs::Value(PropValue::String(texto)) = args {
        let _ = ui.set_prop(sender, "hint", texto.as_str());
    }
});

// Pelo nome do handler: o form diz "on_click": "btn_salvar_click" e o código registra.
ui.register_handler("btn_salvar_click", |ui, sender, _args| { /* ... */ });

// Disparar à mão, como se o usuário tivesse clicado
ui.click(btn);
```

Os handlers gerados pela IDE ficam em `src/forms/<form>.rs`. A IDE cria a função na
primeira vez e depois só refaz o bloco de registro; o corpo da função é seu.

## Forms

```rust
let f = ui.create_form("frmClientes")?;     // cria a partir de forms/clientes.json
ui.show(f)?;
ui.show_modal(f, |ui, result| {             // modal com retorno (como ShowModal)
    if result == ModalResult::Ok { /* ... */ }
})?;
ui.close(f);
```

## Tema

```rust
ui.set_theme(Theme::dark());                       // troca tudo
ui.set_prop(barra, "theme", "Aylis Dark")?;         // só a barra e os filhos
ui.theme_mut().mix("Button", "Marca");             // todos os botões com o tema "Marca"
ui.register_theme(meu_tema);                       // acrescenta à biblioteca
let cor = ui.theme_of(btn).color("primary");       // com qual tema o controle é pintado
```

## Dados

Os dados ficam num `ClientDataSet` (em memória). Os controles DB* se ligam a ele por um
`DataSource`. Para ler e alterar, pegue um `DataSet`, que é um "cursor" sobre os registros:

```rust
use aylis::prelude::*;

let cds = ui.find("cdsClientes").unwrap();
let mut ds = data_set(&mut ui, cds);

// Carregar (do backend: banco, API, arquivo)
ds.load_rows(vec![
    vec![1.into(), "Maria".into(), true.into()],
    vec![2.into(), "João".into(), false.into()],
])?;
// ou uma lista de objetos JSON: ds.load_json(&serde_json::from_str(&texto)?)?

// Navegar e ler
ds.first()?;
while !ds.eof() {
    let nome = ds.text("nome")?;           // formatado (máscara, display_format)
    let ativo = ds.value("ativo")?;        // Value::Bool(true)
    ds.next()?;
}
ds.locate("id", 2)?;                       // posiciona

// Alterar (os controles ligados da tela acompanham)
ds.edit()?;
ds.set_text("nome", "Maria da Silva")?;    // como se o usuário digitasse (valida)
ds.set_value("ativo", false)?;
ds.post()?;

ds.append()?;
ds.set_value("nome", "Ana")?;
ds.post()?;

// Filtro e ordem
ds.set_filter("ativo and nome like 'M%'")?;
ds.set_index("nome")?;

// Mandar as alterações ao backend (delta) e aplicar a resposta
let alteracoes = ds.delta()?;              // inclusões, alterações e exclusões
ds.on_apply(|changes| {
    // gravar no banco e responder registro a registro
    changes.iter().map(|c| ApplyResult::ok(c.reference)).collect()
});
ds.apply_updates()?;
```

Para só ler, sem precisar de `&mut Ui`, use `view(&ui, cds)`.

Eventos do conjunto de dados, como `BeforePost`, `AfterScroll` e `OnNewRecord`:

```rust
data_set(&mut ui, cds)
    .on_before_post(|ui, cds| {
        let nome = view(ui, cds).text("nome").unwrap_or_default();
        if nome.trim().is_empty() { Err("Informe o nome".into()) } else { Ok(()) }
    })
    .on_after_scroll(|_ui, _cds| Ok(()));
```

Os campos (`Field`) são componentes filhos do `ClientDataSet`, com propriedades como
`label`, `required`, `display_format`, `edit_mask`, `read_only`, `lookup` e `calculated`.
Eles também são lidos e trocados com `ui.set_prop(campo, "label", "Nome")`.

## Python e outras linguagens

O mesmo modelo funciona por protocolo: o processo da tela (`aylis-host`) mostra os forms e
o seu código em Python (ou qualquer linguagem que fale JSON por pipe ou WebSocket) comanda
(`clients/python/aylis_client.py`):

```python
from aylis_client import Aylis

aylis = Aylis()

@aylis.on("btn_salvar_click")                     # o handler pelo nome usado no form
def salvar(event):
    nome = aylis.call("get", target="edtNome", prop="text")
    aylis.send("set", target="lblStatus", props={"text": f"Salvo: {nome}"})

aylis.run("frmClientes")                          # abre o form e trata os eventos
```
