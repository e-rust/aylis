# Segundo plano: o `BackgroundService`

A lógica que precisa continuar com a tela fechada (sincronizar, acompanhar a localização,
tocar áudio, falar com um aparelho Bluetooth) fica numa unidade sem tela do projeto e roda no
componente `BackgroundService`.

## Como usar

1. Ponha um `BackgroundService` num form (categoria Device) e escolha:
   - `kind`, o tipo que o Android 14+ exige: `dataSync` (o padrão), `location`,
     `mediaPlayback`, `connectedDevice`, `microphone` ou `camera`;
   - `title` e `text`, o texto da notificação fixa;
   - `keep_alive`, para o serviço sobreviver ao sistema.
2. "Gerar código" cria `src/background.rs`, com `pub fn run(service: aylis::background::Service)`,
   e liga a unidade no `lib.rs`. O corpo é seu:
   - `service.recv(tempo)` recebe o que a tela mandou;
   - `service.send(texto)` manda para a tela;
   - `service.stopped()` diz quando parar.
3. Na tela:
   - `active = true` liga o serviço;
   - `send(texto)` fala com a lógica;
   - `on_message` recebe o que ela mandou. O que ela mandou com a tela fechada fica guardado
     e chega todo quando a tela volta.

## O que acontece em cada sistema

| Sistema | Com a tela fechada |
|---|---|
| Android | Um serviço em primeiro plano, com a notificação fixa. O processo não é encerrado. |
| Android com `keep_alive` | O sistema reinicia o serviço se matar o processo (`START_STICKY`) e ele volta no boot. Nos dois casos ele sobe sozinho, sem a tela. O app também pede para sair da otimização de bateria. |
| Windows, macOS, Linux | A lógica roda enquanto o app está aberto. |
| iOS | Não há serviço assim. A lógica roda com o app aberto; para o resto, os modos permitidos pela Apple (áudio, localização, `BGTaskScheduler`). |

Para cada tipo, a publicação declara no manifesto:
- `FOREGROUND_SERVICE` e a permissão do tipo (`FOREGROUND_SERVICE_LOCATION`...);
- `POST_NOTIFICATIONS`;
- com `keep_alive`, `RECEIVE_BOOT_COMPLETED` e `REQUEST_IGNORE_BATTERY_OPTIMIZATIONS`.

O tipo `location` só funciona com a permissão de localização já concedida (o componente
`Permissions`). A Play Store pede a justificativa de cada tipo no formulário do app.

## Fabricantes com listas próprias

Alguns fabricantes encerram apps em segundo plano mesmo com o serviço em primeiro plano e a
isenção da bateria. O usuário precisa liberar o app numa tela própria do aparelho; os nomes
mudam de versão para versão:

| Fabricante | Onde liberar |
|---|---|
| ASUS (ZenUI) | Gerenciador móvel → Gerenciador de inicialização automática: permitir o app; e Economia de energia → otimização: desligar para o app |
| Xiaomi (MIUI/HyperOS) | Configurações → Apps → o app → Inicialização automática: ligar; Economia de bateria: "Sem restrições" |
| Samsung (One UI) | Configurações → Bateria → Limites de uso em segundo plano: tirar o app de "Apps em suspensão" e de "Suspensão profunda" |
| Huawei (EMUI) | Configurações → Bateria → Inicialização de apps: gerenciar manualmente, com as três opções ligadas |
| OnePlus / Oppo / Realme (ColorOS) | Configurações → Bateria → Otimização de bateria: "Não otimizar"; e Inicialização automática: ligar |
| Motorola | Configurações → Apps → o app → Bateria: "Sem restrições" |

O site [dontkillmyapp.com](https://dontkillmyapp.com) mantém a lista atualizada, com as telas.
Vale mostrar ao usuário, na primeira vez que ele liga o serviço, um aviso com esse endereço
(o `AppLink.open_url` abre a página do fabricante dele).

## E notificações push (FCM/APNs)?

O push é a reserva quando o serviço não pode ficar ligado. O servidor manda uma mensagem e o
sistema acorda o app mesmo encerrado; no iOS é o único jeito. Avaliado em 01/11; fica para um
componente próprio, porque pede:

- no Android, o Firebase (`google-services.json`) e o FCM (a biblioteca Java do Google, que o
  Aylis não usa hoje);
- no iOS, o certificado ou a chave APNs e a capacidade de push no Xcode;
- um servidor que guarde os tokens dos aparelhos.

Para apps que só precisam acordar de tempos em tempos, o serviço com `keep_alive` resolve sem
nada disso.
