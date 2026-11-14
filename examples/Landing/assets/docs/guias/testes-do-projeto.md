# Testes do projeto

O `test` da IDE (e a ferramenta `test` do MCP) roda os testes do projeto na linguagem ativa e
devolve o resultado de cada um: os que passaram, os que falharam (com a mensagem) e os
pulados. Os testes ficam na pasta do código da linguagem (`<projeto>/<linguagem>`).

| Linguagem | Onde ficam | Como o Aylis roda |
|---|---|---|
| Rust | `tests/*.rs` e os `#[test]` do código | `cargo test --no-fail-fast` |
| Python | `test_*.py` | `python -m unittest discover -v` |
| Go | `*_test.go` | `go test -v ./...` |
| TypeScript | `*.test.ts` | `bun test` ou `node --test` |
| C# | um projeto de testes | `dotnet test` (só diz se passou) |
| Java | `tests/*.java` | o executor do Aylis, ou o JUnit |
| C++ | `tests/*.cpp` | o executor do Aylis |
| Free Pascal | `tests/*.pas` | o FPCUnit, com o executor do Aylis |

Java, C++ e Free Pascal não têm uma ferramenta de teste que venha junto. Para elas, o
cliente do Aylis traz um executor pequeno. Ele compila os testes junto com o código dos forms,
em `<projeto>/.aylis/tests`, e imprime uma linha por teste.

## Java

Marque os métodos de teste com `@dev.aylis.Test` e confira com `dev.aylis.Check`:

```java
import dev.aylis.Check;
import dev.aylis.Test;
import forms.Calculadora;

public class MotorTest {
    @Test
    public void soma() {
        Calculadora c = new Calculadora();
        for (String t : new String[] {"1", "2", "+", "3", "0", "="}) c.tecla(t);
        Check.equal("42", c.visor());
    }
}
```

`Check.equal`, `Check.near` (números com tolerância) e `Check.that` (uma condição). O método
pode ser estático ou não (cada teste roda num objeto novo).

Com o JUnit no projeto (o `junit-platform-console-standalone-*.jar` em `tests/lib`), o Aylis
roda o JUnit no lugar do executor próprio, e os testes usam as anotações dele.

## C++

Inclua `aylis_test.hpp` e registre cada teste com `AYLIS_TEST`:

```cpp
#include "aylis_test.hpp"
#include "forms/motor.hpp"

AYLIS_TEST(soma) {
    motor::Calculadora c;
    for (const char* t : {"1", "2", "+", "3", "0", "="}) c.tecla(t);
    AYLIS_CHECK_EQ(std::string("42"), c.visor());
}
```

`AYLIS_CHECK(condição)`, `AYLIS_CHECK_EQ(esperado, veio)` e
`AYLIS_CHECK_NEAR(esperado, veio, tolerância)`. O `main` dos testes é do Aylis: os arquivos de
`src/forms` entram na compilação, e o `src/main.cpp` do app não.

## Free Pascal

Os testes são do FPCUnit, que vem com o Free Pascal: uma classe `TTestCase` com os métodos de
teste em `published` e o `RegisterTest` na `initialization`:

```pascal
unit testmotor;

{$mode objfpc}{$H+}

interface

uses
  fpcunit, testregistry, motor;

type
  TTestMotor = class(TTestCase)
  published
    procedure Soma;
  end;

implementation

procedure TTestMotor.Soma;
begin
  AssertEquals('42', ...);
end;

initialization
  RegisterTest(TTestMotor);
end.
```

O Aylis gera o programa que roda todos os testes registrados (com a unit `aylistest` do
cliente) e lê o resultado de cada um.

## Exemplo

A Calculadora (`examples/Calculadora`) tem os mesmos testes do motor nas linguagens: o
`rust/tests/motor.rs`, o `java/tests/MotorTest.java`, o `cpp/tests/motor.cpp` e o
`pascal/tests/testmotor.pas`.
