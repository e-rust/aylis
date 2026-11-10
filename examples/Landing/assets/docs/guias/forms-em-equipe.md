# Forms em equipe: diferenças e conflitos

Os forms são JSON (`forms/*.json`) e vão para o controle de versão como o código. A IDE grava
sempre no mesmo formato, para que a diferença entre duas versões mostre só o que mudou.

## O formato que a IDE grava

- uma chave por linha, dois espaços por nível;
- a ordem é fixa: `class`, `name`, as propriedades na ordem da classe, `bindings`, `views` e
  `children` por último;
- listas curtas numa linha só (`"margins": [8, 4, 8, 8]`, `"columns": ["nome", "saldo"]`);
- só o que é diferente do padrão da classe é gravado.

Por isso:

- abrir e salvar sem mexer não muda o arquivo;
- mudar uma propriedade muda uma linha;
- mover um componente muda as linhas dele de lugar, não o arquivo inteiro.

Um form escrito à mão ou por outra ferramenta passa para esse formato uma vez com:

```bash
aylis-ide --format-forms projeto.aylisproj
```

O comando só muda a forma, nunca o conteúdo. Depois dele, a IDE não mexe em mais nada além do
que você mudou.

## O `.aylisproj` não muda ao salvar um form

Salvar um form não regrava o `.aylisproj`: duas pessoas mexendo em forms diferentes não
conflitam nele. A IDE anota de cada form a hora do arquivo (para saber quando ler de novo se
ele é um DataModule e o que ele usa), mas essa anotação fica em `.aylis/form-stamps.json`, que é
desta máquina e fica fora do controle de versão (o `.aylis/.gitignore` já diz isso). O
`.aylisproj` só muda quando muda o que ele diz: um form novo, renomeado ou removido, o form
principal, as opções do projeto.

## Resolver um conflito

Duas pessoas mudaram o mesmo form e o `git merge` parou com conflito no `.json`. O conflito
fica restrito aos componentes que as duas mexeram:

1. **Abra o arquivo no editor de texto** (não no designer: com as marcas `<<<<<<<` o JSON não
   é válido e a IDE não abre).
2. **Cada bloco de conflito é uma propriedade ou um componente.**
   - Duas mudanças na mesma propriedade (`"width": 120` contra `"width": 140`): fique com uma.
   - Dois componentes novos no mesmo lugar dos `children`: fique com os dois, cuidando da
     vírgula entre eles. A ordem dos `children` é a ordem na tela, e no alinhamento por `align`
     ela decide quem fica por cima.
   - O mesmo `name` nos dois lados: renomeie um. Nomes repetidos no mesmo form não abrem.
3. **Confira o JSON:** abra o form na IDE. Se algo estiver errado, ela diz o componente e a
   propriedade.
4. **Salve na IDE uma vez:** o arquivo volta ao formato fixo, e o próximo merge sai limpo.
5. **Gere o código** (o "Gerar código" ou o F9), para os componentes novos aparecerem no acesso
   tipado (`self.btnNovo`).

## Dicas para evitar conflitos

- Faça commits pequenos: um form por assunto.
- Antes de começar num form que outra pessoa está mexendo, traga as mudanças dela
  (`git pull`).
- Prefira frames (um pedaço de tela num form próprio) para partes que várias pessoas mexem.
