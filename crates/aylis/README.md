# Aylis

O Aylis é um framework de interface em Rust: você desenha os formulários na IDE, ajusta as
propriedades e escreve só o código dos eventos. O mesmo app roda no Windows, no Linux, no
macOS, no Android, no iOS e no navegador, com a mesma aparência em todos.

Este é o crate que os apps usam: ele junta o núcleo, os controles, a camada de dados, as
plataformas e o projeto (`aylis::Application`, `aylis::prelude`).

## Como começar

O jeito mais simples é pela IDE do Aylis (o instalador está nas
[releases](https://github.com/e-rust/aylis/releases)): **Arquivo → Novo projeto** cria o projeto com os forms, o
`Cargo.toml` e o código dos eventos já ligados a este crate. A [documentação](https://e-rust.github.io/aylis/) tem
o tutorial do primeiro projeto, passo a passo, e a página de cada componente.

- Forms e temas em JSON, editados no designer da IDE.
- Centenas de componentes: formulário, grades de dados, gráficos, relatórios, mapas, vídeo,
  câmera, 3D, Markdown, documentos e controles industriais.
- Renderizadores Direct2D (Windows), Skia (CPU e GPU) e Canvas 2D (navegador).
- O código dos eventos também em Python, TypeScript, C#, Java, Go, C++ ou Free Pascal.

## Links

- Site e documentação: https://e-rust.github.io/aylis/
- Código e releases: https://github.com/e-rust/aylis

## Licença

MIT.
