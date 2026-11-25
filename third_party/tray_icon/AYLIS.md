# tray-icon (cópia do Aylis)

Cópia do `tray-icon` 0.25.1 (crates.io, licença MIT OR Apache-2.0, projeto Tauri:
https://github.com/tauri-apps/tray-icon, commit `b09ceda9f7357b4a1f6bc361fd912a7dfca077cc`) com
uma correção e um acréscimo, os dois só no Windows:

- `set_tooltip` com o GUID (`src/platform_impl/windows/mod.rs`): o original não põe o
  `NIF_GUID` ao trocar a dica (o `set_icon` e o `set_visible` põem), e num ícone criado com
  `with_guid` a dica nunca mudava. Achado ao conferir o `guid` do `TrayIcon` em 25/11/2026.
- `with_uid(u32)` no `TrayIconBuilder` (e o campo `uid` no `TrayIconAttributes`): no Windows, o
  `uID` do ícone passa a ser o número dado, no lugar do contador interno (`COUNTER`). Sem GUID,
  o Windows 11 lembra se o ícone fica na barra ou no "^" pelo caminho do exe mais o `uID`; com o
  contador, o `uID` depende de quantos identificadores já foram tirados antes (o próprio
  `TrayIconBuilder::new` tira um para o `TrayIconId`, e por isso o primeiro ícone sai com o 2),
  e um app que substitui outro no mesmo caminho não consegue herdar o lugar do ícone dele.
  Nas outras plataformas, ignorado.

O patch e o texto para oferecer ao projeto original (as duas mudanças) estão em `UPSTREAM.md`.
Voltar para a versão do crates.io quando as duas estiverem numa versão publicada.

No crates.io, esta cópia vai como `aylis-tray-icon` (como a `aylis-accesskit-android`): o nome
do original é do Tauri. A biblioteca continua `tray_icon`, e o `aylis-platform-winit` a usa com
`package = "aylis-tray-icon"`. As `dev-dependencies` do original saíram (os exemplos não vêm na
cópia, e elas puxariam o `eframe` e o `tao` para o `Cargo.lock` do workspace).

As licenças do original (MIT ou Apache-2.0, à escolha) estão em `LICENSE-MIT` e `LICENSE-APACHE`.
