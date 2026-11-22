### `set_value` pelo código

Mudar o `value` pelo código também dispara o `on_change` (como o usuário arrastando). Dentro
de um handler do mesmo form, o evento é adiado para depois dele (o log mostra
`evento ... adiado`). Para mudar sem reagir, guarde no estado do form que a mudança veio do
código e ignore no `on_change`.
