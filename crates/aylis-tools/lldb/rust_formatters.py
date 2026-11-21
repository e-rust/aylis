# Os formatadores do Rust mais rápidos no LLDB (carregado pelo depurador da IDE).
#
# HashMap e HashSet: o formatador do toolchain (lldb_providers.StdHashMapSyntheticProvider)
# varre todos os baldes da tabela, um valor do LLDB por byte, cada vez que a variável
# aparece: um mapa de 5 mil itens custa 0,3 s por passo e um de 1 milhão, mais de um
# minuto. Aqui o tamanho vem do contador `items` da tabela, e os baldes só são lidos (de
# uma vez) quando a variável é aberta.
#
# Structs e tuplas: o resumo do toolchain (StructSummaryProvider) junta o de todos os campos,
# recursivamente e sem limite; no Linux (DWARF), o `ui: &mut Ui` de um handler levava
# minutos, as variáveis não vinham e o F8 não voltava (Fedora, 21/11). Aqui o resumo mostra
# só os primeiros campos de um nível; os de dentro aparecem ao abrir a variável.

import lldb

# Quantos campos o resumo de uma struct mostra (o resto vira "...").
_MAX_FIELDS = 6
# O resumo de uma struct (de mais de um campo) dentro do resumo de outra: só "{...}". As de um
# campo só (o AtomicUsize, o Cell, o UnsafeCell) não contam: o valor de dentro aparece.
_depth = 0
# O limite de structs uma dentro da outra, contando as de um campo (uma lista encadeada).
_nesting = 0


def _bounded(valobj, names):
    global _depth, _nesting
    opening, closing = ("{", "}") if names else ("(", ")")
    count = valobj.GetNumChildren()
    wrapper = count == 1
    if (_depth >= 1 and not wrapper) or _nesting >= 8:
        return opening + "..." + closing
    _depth += 0 if wrapper else 1
    _nesting += 1
    try:
        parts = []
        for i in range(min(count, _MAX_FIELDS)):
            child = valobj.GetChildAtIndex(i)
            text = child.GetSummary() or child.GetValue() or "{...}"
            parts.append(f"{child.GetName()}:{text}" if names else text)
        if count > _MAX_FIELDS:
            parts.append("...")
        return opening + ", ".join(parts) + closing
    finally:
        _depth -= 0 if wrapper else 1
        _nesting -= 1


def _struct_summary(valobj, _dict):
    return _bounded(valobj, True)


def _tuple_summary(valobj, _dict):
    return _bounded(valobj, False)


def _setup(self):
    table = self.table()
    inner = table.GetChildMemberWithName("table")
    self._aylis_capacity = inner.GetChildMemberWithName("bucket_mask").GetValueAsUnsigned() + 1
    ctrl = inner.GetChildMemberWithName("ctrl").GetChildAtIndex(0)
    self._aylis_ctrl = ctrl.GetValueAsUnsigned()
    self._aylis_valid = None
    self.size = inner.GetChildMemberWithName("items").GetValueAsUnsigned()

    self.pair_type = table.GetType().GetTemplateArgumentType(0)
    if not self.pair_type.IsValid():
        import lldb_providers as p
        arg_name = next(p.get_template_args(table.GetTypeName()))
        self.pair_type = p.resolve_msvc_template_arg(arg_name, self.valobj.target)
    if self.pair_type.IsTypedefType():
        self.pair_type = self.pair_type.GetTypedefedType()
    self.pair_type_size = self.pair_type.GetByteSize()

    self.new_layout = not inner.GetChildMemberWithName("data").IsValid()
    if self.new_layout:
        self.data_ptr = ctrl.Cast(self.pair_type.GetPointerType())
    else:
        self.data_ptr = inner.GetChildMemberWithName("data").GetChildAtIndex(0)


def _valid_indices(self):
    # Os baldes ocupados (o bit alto do byte de controle zerado), lidos de uma vez.
    if self._aylis_valid is None:
        valid = []
        if self.size > 0 and self._aylis_capacity > 0:
            error = lldb.SBError()
            data = self.valobj.GetProcess().ReadMemory(self._aylis_ctrl, self._aylis_capacity, error)
            if error.Success() and data:
                valid = [i for i, b in enumerate(data) if b & 128 == 0]
        self._aylis_valid = valid
    return self._aylis_valid


def _num_children(self):
    return self.size


def _get_child_at_index(self, index):
    valid = _valid_indices(self)
    if index < 0 or index >= len(valid):
        return None
    return _ORIGINAL_GET_CHILD(self, index)


_ORIGINAL_GET_CHILD = None


def __lldb_init_module(debugger, internal_dict):
    global _ORIGINAL_GET_CHILD
    try:
        import lldb_providers
    except ImportError:
        return  # sem os formatadores do Rust: nada a acelerar
    # O LLDB chama o resumo pelo nome (`lldb_lookup.StructSummaryProvider`), e os de enum e
    # de campo chamam o do `lldb_providers`: troca nos dois.
    modules = [lldb_providers]
    try:
        import lldb_lookup
        modules.append(lldb_lookup)
    except ImportError:
        pass
    for module in modules:
        if hasattr(module, "StructSummaryProvider"):
            module.StructSummaryProvider = _struct_summary
        if hasattr(module, "TupleSummaryProvider"):
            module.TupleSummaryProvider = _tuple_summary
    cls = lldb_providers.StdHashMapSyntheticProvider
    if getattr(cls, "_aylis_fast", False):
        return
    _ORIGINAL_GET_CHILD = cls.get_child_at_index
    cls.update = _setup
    cls.num_children = _num_children
    cls.get_child_at_index = _get_child_at_index
    # O original lê `self.valid_indices[index]`.
    cls.valid_indices = property(_valid_indices)
    cls._aylis_fast = True
