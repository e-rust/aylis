//! Manter à vista: o teclado do celular (ou uma barra do sistema) cobre parte do form, e o
//! campo com foco tem de aparecer. [`Ui::reveal`] rola os contêineres em volta
//! (`ScrollBox`) e, se ainda não bastar, o próprio form; o form também rola com o dedo (ou a
//! roda) quando o conteúdo não cabe, como uma página. Nada a configurar: vale para todo
//! form.

use crate::component::ComponentId;
use crate::geometry::PointF;
use crate::ui::Ui;

/// Folga entre o controle mostrado e a borda da área visível.
const MARGIN: f32 = 8.0;

impl Ui {
    /// Quanto o form `root` pode rolar: o que o conteúdo passa da parte da janela que não
    /// está coberta (pelo teclado, pela barra de início...).
    pub(crate) fn form_scroll_max(&self, root: ComponentId) -> f32 {
        let Some(cb) = self.cb(root) else { return 0.0 };
        let visible = cb.bounds().height - cb.safe_area.bottom;
        let content = cb.content_size().height;
        // Passando, sobra a folga embaixo (o último campo não encosta no teclado).
        if content > visible { content + MARGIN - visible } else { 0.0 }
    }

    /// Rola o necessário (os contêineres que rolam em volta e, por último, o form) para o
    /// controle `id` ficar inteiro na área visível. O aylis chama sozinho quando o teclado
    /// do celular abre sobre o campo com foco.
    pub fn reveal(&mut self, id: ComponentId) {
        self.reveal_with(id, false);
    }

    /// Rola para o controle `id` ficar no alto da área visível (o menu de uma página que leva
    /// à seção), e não só o necessário para ele aparecer, como no [`Ui::reveal`].
    pub fn reveal_top(&mut self, id: ComponentId) {
        self.reveal_with(id, true);
    }

    fn reveal_with(&mut self, id: ComponentId, at_top: bool) {
        self.update_layout();
        let root = self.root_of(id);
        let mut current = self.parent(id);
        while let Some(a) = current {
            let scrolls = a == root || self.component(a).is_some_and(|c| c.scroll_axes().1);
            if scrolls {
                self.reveal_in(a, id, root, at_top);
            }
            if a == root {
                break;
            }
            current = self.parent(a);
        }
    }

    fn reveal_in(&mut self, container: ComponentId, id: ComponentId, root: ComponentId, at_top: bool) {
        let (Some(r), Some(area)) = (self.absolute_rect(id), self.absolute_rect(container)) else { return };
        let Some(form) = self.cb(root) else { return };
        let (safe, form_height) = (form.safe_area, form.bounds().height);
        let Some(cb) = self.cb(container) else { return };
        let top = area.y.max(safe.top) + MARGIN;
        let bottom = area.bottom().min(form_height - safe.bottom) - MARGIN;
        let max = if container == root { self.form_scroll_max(root) } else { (cb.content_size().height - cb.bounds().height).max(0.0) };
        let delta = if at_top || r.height > bottom - top || r.y < top {
            r.y - top // no alto (pedido, ou maior que a área: mostra o começo)
        } else if r.bottom() > bottom {
            r.bottom() - bottom
        } else {
            return;
        };
        let offset = cb.scroll_offset();
        let y = (offset.y + delta).clamp(0.0, max);
        if (y - offset.y).abs() < 0.5 {
            return;
        }
        if let Some(cb) = self.cb_mut(container) {
            cb.set_scroll_offset(PointF::new(offset.x, y));
        }
        self.invalidate();
        self.update_layout();
        self.request_repaint();
    }

    /// A roda (ou o dedo) que nenhum controle tratou rola o form, se o conteúdo não cabe.
    pub(crate) fn scroll_form(&mut self, root: ComponentId, delta: f32) -> bool {
        let max = self.form_scroll_max(root);
        let Some(cb) = self.cb(root) else { return false };
        let offset = cb.scroll_offset();
        let y = (offset.y - delta).clamp(0.0, max);
        if (y - offset.y).abs() < 0.01 {
            return false;
        }
        if let Some(cb) = self.cb_mut(root) {
            cb.set_scroll_offset(PointF::new(offset.x, y));
        }
        self.invalidate();
        self.request_repaint();
        true
    }
}
