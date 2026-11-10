/*
 * aylis.h — C ABI do aylis (aylis.dll / libaylis.so / libaylis.dylib).
 *
 * Tudo passa pelo protocolo JSON do aylis-remote (comandos load, create, set, get,
 * data...). Respostas: {"result": ...} ou {"error": "..."}.
 * Toda string devolvida pelo aylis deve ser liberada com aylis_string_free.
 */
#ifndef AYLIS_H
#define AYLIS_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct AylisApp AylisApp;
/* Contexto recebido nos callbacks de evento; só vale durante a chamada. */
typedef struct AylisCtx AylisCtx;

/* Evento com handler por nome, em JSON:
 * {"event": "on_click", "handler": "...", "sender": {"id", "name", "class"}, "args": ...}
 * Um evento disparado de dentro do callback (aylis_ctx_execute com "click") chega a ele na
 * hora, como no Delphi. Trate as exceções (C++, Delphi) dentro do callback: uma que escape
 * encerra o processo. */
typedef void (*AylisEventFn)(AylisCtx *ctx, const char *event_json, void *user);

/* Cria um aplicativo (classes padrão registradas, idioma pt-BR). */
AylisApp *aylis_new(void);
void aylis_free(AylisApp *app);
void aylis_string_free(char *text);

/* Executa um comando JSON. Vale antes de aylis_run. */
char *aylis_execute(AylisApp *app, const char *command_json);
/* Executa um comando de dentro de um callback de evento. */
char *aylis_ctx_execute(AylisCtx *ctx, const char *command_json);

/* Define quem recebe os eventos. Devolve 0 se ok. */
int aylis_set_event_callback(AylisApp *app, AylisEventFn callback, void *user);

/* Agenda um comando para a thread da tela (qualquer thread, a qualquer momento). */
int aylis_post(const AylisApp *app, const char *command_json);

/* Abre o form e bloqueia até a janela fechar. Devolve 0 se ok. Depois disso o
 * aplicativo só pode ser liberado. */
int aylis_run(AylisApp *app, const char *form_name);

/* Desenha o form num PNG, sem janela. Devolve 0 se ok. */
int aylis_render_png(AylisApp *app, const char *form_name, const char *path);

#ifdef __cplusplus
}
#endif

#endif /* AYLIS_H */
