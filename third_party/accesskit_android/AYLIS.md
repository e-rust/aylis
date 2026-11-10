# accesskit_android (cópia do aylis)

Cópia do `accesskit_android` 0.9.1 (crates.io, licença MIT OR Apache-2.0, projeto AccessKit:
https://github.com/AccessKit/accesskit) com uma correção:

- `src/inject.rs`: o `InMemoryDexClassLoader` da classe `Delegate` recebe o carregador do
  sistema como pai em vez de `null`. Com `null`, no Android 9 (API 28) e anteriores, a classe
  não é definida (ela estende `android.view.View$AccessibilityDelegate`, que esse carregador
  não enxerga) e o app cai ao abrir. Achado no ASUS ZenFone 5 (Android 9) em 01/11/2026.

Voltar para a versão do crates.io quando a correção estiver numa versão publicada.

As licenças do original (MIT ou Apache-2.0, à escolha) estão em `LICENSE-MIT` e `LICENSE-APACHE`,
copiadas do repositório do AccessKit.
