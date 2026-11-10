//! Itens que um controle desenha sozinho (as abas de um TabControl, as linhas de um ListBox,
//! os itens de um menu, as células de um grid) descritos para quem não vê a tela: os
//! leitores de tela e a automação do sistema (UI Automation no Windows), o protocolo do
//! inspetor e os testes. Cada item tem papel, nome, posição e estado, e aceita clicar ou
//! selecionar — sem mandar evento de mouse para a janela.

use crate::geometry::RectF;

/// O papel de um item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemRole {
    Tab,
    ListItem,
    MenuItem,
    /// Item de menu que marca e desmarca.
    MenuItemCheck,
    /// Linha de um grid (as células dela vêm depois, com `parent` apontando para ela).
    Row,
    Cell,
    /// Cabeçalho de coluna de um grid.
    ColumnHeader,
    /// Nó de uma árvore (com `level` e, se tem filhos, `expanded`).
    TreeItem,
    /// Opção de um grupo de rádio.
    Radio,
    /// Botão desenhado (o mês anterior de um calendário).
    Button,
}

/// Um item desenhado pelo controle.
#[derive(Clone, Debug, PartialEq)]
pub struct AccessItem {
    pub role: ItemRole,
    /// O texto que a pessoa vê (o nome para o leitor de tela e para achar o item).
    pub name: String,
    /// Onde está, nas coordenadas do controle (`None`: fora da parte visível).
    pub rect: Option<RectF>,
    pub selected: bool,
    pub checked: Option<bool>,
    pub disabled: bool,
    /// Tem itens dentro (um item de menu com submenu).
    pub has_popup: bool,
    /// O item dono deste (a linha de uma célula), pelo índice na lista.
    pub parent: Option<usize>,
    /// Linha e coluna (grid).
    pub row: Option<usize>,
    pub column: Option<usize>,
    /// Profundidade na árvore (0 = raiz).
    pub level: Option<usize>,
    /// Aberto ou fechado (nó de árvore com filhos).
    pub expanded: Option<bool>,
}

impl AccessItem {
    pub fn new(role: ItemRole, name: impl Into<String>) -> Self {
        Self {
            role,
            name: name.into(),
            rect: None,
            selected: false,
            checked: None,
            disabled: false,
            has_popup: false,
            parent: None,
            row: None,
            column: None,
            level: None,
            expanded: None,
        }
    }

    pub fn rect(mut self, rect: RectF) -> Self {
        self.rect = Some(rect);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn has_popup(mut self, has_popup: bool) -> Self {
        self.has_popup = has_popup;
        self
    }

    pub fn parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn cell(mut self, row: usize, column: usize) -> Self {
        self.row = Some(row);
        self.column = Some(column);
        self
    }

    pub fn level(mut self, level: usize) -> Self {
        self.level = Some(level);
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }
}

/// O que fazer num item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemAction {
    /// Como o clique: ativa (um item de menu, uma aba, uma linha).
    Click,
    /// Só seleciona (a linha de uma lista sem ativar).
    Select,
    /// Abre (um nó de árvore com filhos).
    Expand,
    /// Fecha.
    Collapse,
}
