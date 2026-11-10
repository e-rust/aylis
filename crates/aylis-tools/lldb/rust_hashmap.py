# HashMap e HashSet do Rust mais rápidos no LLDB (carregado pelo depurador da IDE).
#
# O formatador do toolchain (lldb_providers.StdHashMapSyntheticProvider) varre todos os
# baldes da tabela, um valor do LLDB por byte, cada vez que a variável aparece: um mapa de
# 5 mil itens custa 0,3 s por passo e um de 1 milhão, mais de um minuto. Aqui o tamanho
# vem do contador `items` da tabela, e os baldes só são lidos (de uma vez) quando a
# variável é aberta.

import lldb


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
