//! `Playlist`: a lista de faixas de um `MediaPlayer`, com próxima, anterior, aleatório e
//! repetir, e a passagem de uma faixa para a outra sem silêncio (o `queue_next` do player).
//! Decidido em 21/11 (o relatório de um player de música portado para o Aylis): o que todo
//! player refaz igual sai do app, e o "sem silêncio" o app não alcança sozinho (precisa do fluxo
//! de áudio). Não visual; o `MediaPlayer` só toca.

use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropEnum, PropKind, PropValue, Ui, properties};

use crate::MediaPlayer;

/// O que acontece no fim da lista (ou da faixa).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RepeatMode {
    /// Para no fim da lista.
    #[default]
    Off,
    /// Volta ao começo da lista.
    All,
    /// Repete a faixa.
    One,
}

impl PropEnum for RepeatMode {
    const NAMES: &'static [&'static str] = &["off", "all", "one"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Off, Self::All, Self::One].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A lista de faixas de um player. Não visual.
pub struct Playlist {
    /// O `MediaPlayer` que toca a lista.
    pub player: Option<ComponentId>,
    /// Os arquivos (do projeto ou caminhos do disco), na ordem da lista.
    pub items: String,
    /// A faixa de agora (-1: nenhuma).
    pub index: i64,
    /// A ordem aleatória (sem repetir até passar por todas).
    pub shuffle: bool,
    /// No fim: `off` para, `all` volta ao começo da lista, `one` repete a faixa.
    pub repeat: RepeatMode,
    /// Emenda a próxima faixa sem silêncio (desligado: o player fecha uma e abre a outra).
    pub gapless: bool,
    /// A ordem de tocar (os índices de `items`); sem `shuffle`, a da lista.
    order: Vec<usize>,
    /// A faixa que o player tem na fila (`queue_next`), para avançar quando ela entrar.
    queued: Option<usize>,
    seed: u64,
}

impl Default for Playlist {
    fn default() -> Self {
        Self { player: None, items: String::new(), index: -1, shuffle: false, repeat: RepeatMode::Off, gapless: true, order: Vec::new(), queued: None, seed: 0x9E37_79B9_7F4A_7C15 }
    }
}

impl Playlist {
    /// Os arquivos da lista (separados por `;` ou por linha).
    pub fn files(&self) -> Vec<String> {
        self.items.split([';', '\n']).map(|s| s.trim()).filter(|s| !s.is_empty()).map(str::to_string).collect()
    }

    /// Refaz a ordem (a lista ou o aleatório mudaram), com a faixa de agora na frente no
    /// aleatório (o que toca continua).
    fn reorder(&mut self) {
        let n = self.files().len();
        self.order = (0..n).collect();
        if self.shuffle && n > 1 {
            // Fisher-Yates com um xorshift: sem dependência, igual em todas as plataformas.
            let mut s = self.seed ^ (n as u64).wrapping_mul(0x2545_F491_4F6C_DD1D);
            for i in (1..n).rev() {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                self.order.swap(i, (s % (i as u64 + 1)) as usize);
            }
            self.seed = s;
            if let Some(at) = usize::try_from(self.index).ok().and_then(|i| self.order.iter().position(|&o| o == i)) {
                let current = self.order.remove(at);
                self.order.insert(0, current);
            }
        }
        self.queued = None;
    }

    /// A faixa depois de `index` (ou antes, com `back`), pela ordem e pelo `repeat`.
    fn neighbor(&self, back: bool) -> Option<usize> {
        let n = self.order.len();
        if n == 0 {
            return None;
        }
        let Some(current) = usize::try_from(self.index).ok() else { return self.order.first().copied() };
        if self.repeat == RepeatMode::One && !back {
            return Some(current);
        }
        let at = self.order.iter().position(|&o| o == current).unwrap_or(0);
        let next = if back { at.checked_sub(1).or((self.repeat == RepeatMode::All).then(|| n - 1)) } else { Some(at + 1).filter(|&a| a < n).or((self.repeat == RepeatMode::All).then_some(0)) };
        next.map(|a| self.order[a])
    }

    /// Toca a faixa `index` da lista (a ordem da lista, não a do aleatório).
    pub fn play_index(ui: &mut Ui, id: ComponentId, index: usize) -> Result<(), String> {
        let (player, file) = {
            let p = ui.get_mut::<Playlist>(id).ok_or("não é uma Playlist")?;
            let file = p.files().get(index).cloned().ok_or_else(|| format!("a lista não tem a faixa {index}"))?;
            if p.order.len() != p.files().len() {
                p.reorder();
            }
            p.index = index as i64;
            p.queued = None;
            (p.player.ok_or("ligue o player da Playlist")?, file)
        };
        MediaPlayer::clear_next(ui, player);
        ui.set_prop(player, "file", file.as_str()).map_err(|e| e.to_string())?;
        MediaPlayer::play(ui, player)?;
        ui.fire(id, "on_track_change", &EventArgs::Value(PropValue::Int(index as i64)));
        Self::queue_following(ui, id);
        Ok(())
    }

    /// A próxima faixa (pelo aleatório e pelo `repeat`); no fim sem `repeat`, para.
    pub fn next(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let next = ui.get_mut::<Playlist>(id).and_then(|p| {
            if p.order.len() != p.files().len() {
                p.reorder();
            }
            // O `repeat: one` repete sozinho no fim da faixa; o "próxima" do usuário avança.
            let saved = p.repeat;
            if saved == RepeatMode::One {
                p.repeat = RepeatMode::All;
            }
            let n = p.neighbor(false);
            p.repeat = saved;
            n
        });
        match next {
            Some(i) => Self::play_index(ui, id, i),
            None => {
                if let Some(player) = ui.get::<Playlist>(id).and_then(|p| p.player) {
                    MediaPlayer::stop(ui, player);
                }
                Ok(())
            }
        }
    }

    /// A faixa anterior; passados 3 s da faixa, volta ao começo dela (como os players).
    pub fn previous(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let Some(player) = ui.get::<Playlist>(id).and_then(|p| p.player) else { return Ok(()) };
        if MediaPlayer::position(ui, player) > 3.0 {
            MediaPlayer::seek(ui, player, 0.0);
            return Ok(());
        }
        let back = ui.get::<Playlist>(id).and_then(|p| p.neighbor(true));
        match back {
            Some(i) => Self::play_index(ui, id, i),
            None => {
                MediaPlayer::seek(ui, player, 0.0);
                Ok(())
            }
        }
    }

    /// Com `gapless`, deixa a próxima na fila do player.
    fn queue_following(ui: &mut Ui, id: ComponentId) {
        let Some((player, next, file)) = ui.get::<Playlist>(id).filter(|p| p.gapless).and_then(|p| {
            let next = p.neighbor(false)?;
            Some((p.player?, next, p.files().get(next)?.clone()))
        }) else {
            return;
        };
        // Abre no fim da faixa de agora (o player precisa estar aberto: o `play` abre em
        // segundo plano; aí o `on_ready` do player chega aqui de novo pelo `player_ready`).
        match MediaPlayer::queue_next(ui, player, &file) {
            Ok(()) if MediaPlayer::is_ready(ui, player) => {
                if let Some(p) = ui.get_mut::<Playlist>(id) {
                    p.queued = Some(next);
                }
            }
            Ok(()) => {}
            Err(e) => {
                ui.fire(id, "on_error", &EventArgs::Text(e));
            }
        }
    }

    /// As `Playlist` ligadas ao player `player`.
    fn of_player(ui: &Ui, player: ComponentId) -> Vec<ComponentId> {
        ui.components().filter(|&c| ui.get::<Playlist>(c).is_some_and(|p| p.player == Some(player))).collect()
    }

    /// O player emendou a faixa da fila (`queue_next`): a lista avança e põe a seguinte.
    pub(crate) fn player_advanced(ui: &mut Ui, player: ComponentId) {
        for id in Self::of_player(ui, player) {
            let Some(index) = ui.get_mut::<Playlist>(id).and_then(|p| {
                let i = p.queued.take()?;
                p.index = i as i64;
                Some(i)
            }) else {
                continue;
            };
            ui.fire(id, "on_track_change", &EventArgs::Value(PropValue::Int(index as i64)));
            Self::queue_following(ui, id);
        }
    }

    /// O player abriu o arquivo (o `play` em segundo plano): agora dá para pôr a próxima na fila.
    pub(crate) fn player_ready(ui: &mut Ui, player: ComponentId) {
        for id in Self::of_player(ui, player) {
            if ui.get::<Playlist>(id).is_some_and(|p| p.queued.is_none() && p.index >= 0) {
                Self::queue_following(ui, id);
            }
        }
    }

    /// O player terminou sem emendar (sem `gapless`, ou a próxima não abriu): a próxima faixa.
    pub(crate) fn player_ended(ui: &mut Ui, player: ComponentId) {
        for id in Self::of_player(ui, player) {
            // Só a lista que está tocando (o fim de um arquivo avulso não começa a lista).
            if ui.get::<Playlist>(id).is_none_or(|p| p.index < 0) {
                continue;
            }
            let next = ui.get::<Playlist>(id).and_then(|p| p.neighbor(false));
            match next {
                Some(i) => {
                    if let Err(e) = Self::play_index(ui, id, i) {
                        ui.fire(id, "on_error", &EventArgs::Text(e));
                    }
                }
                None => {
                    if let Some(p) = ui.get_mut::<Playlist>(id) {
                        p.index = -1;
                    }
                    ui.fire(id, "on_track_change", &EventArgs::Value(PropValue::Int(-1)));
                }
            }
        }
    }
}

impl Component for Playlist {
    fn class_name(&self) -> &'static str {
        "Playlist"
    }

    properties! {
        player: Option<ComponentId> => "Behavior",
        items: String as List => "Data",
        index: i64 => "Data",
        shuffle: bool => "Behavior",
        repeat: RepeatMode => "Behavior",
        gapless: bool => "Behavior";
        events: on_track_change: i64, on_error: String
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("next", &[], None), MethodInfo::new("previous", &[], None), MethodInfo::new("play_index", &[PropKind::Int], None).params(&["index"])];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let id = ctx.id;
        let index = args.first().and_then(|v| v.to_i64().ok());
        let method = method.to_string();
        // Os métodos mexem no player: depois deste evento (o `invoke` tem o componente emprestado).
        match method.as_str() {
            "next" | "previous" | "play_index" => {
                if method == "play_index" && index.is_none_or(|i| i < 0) {
                    return Err("play_index: informe o índice".into());
                }
                ctx.ui.poster().post_to(id, move |ui| {
                    let r = match method.as_str() {
                        "next" => Playlist::next(ui, id),
                        "previous" => Playlist::previous(ui, id),
                        _ => Playlist::play_index(ui, id, index.unwrap_or(0) as usize),
                    };
                    if let Err(e) = r {
                        ui.fire(id, "on_error", &EventArgs::Text(e));
                    }
                });
                Ok(PropValue::Null)
            }
            _ => Err(format!("Playlist não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            "items" | "shuffle" => {
                self.reorder();
                // A próxima da fila pode ter mudado.
                let id = ctx.id;
                ctx.ui.poster().post_to(id, move |ui| {
                    if let Some(player) = ui.get::<Playlist>(id).and_then(|p| p.player) {
                        MediaPlayer::clear_next(ui, player);
                    }
                    Playlist::queue_following(ui, id);
                });
            }
            "repeat" | "gapless" => {
                self.queued = None;
                let id = ctx.id;
                ctx.ui.poster().post_to(id, move |ui| {
                    if let Some(player) = ui.get::<Playlist>(id).and_then(|p| p.player) {
                        MediaPlayer::clear_next(ui, player);
                    }
                    Playlist::queue_following(ui, id);
                });
            }
            _ => {}
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.player == Some(freed) {
            self.player = None;
        }
    }
}
