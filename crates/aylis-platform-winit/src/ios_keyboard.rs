//! iOS: a altura do teclado virtual, para a área segura descontar o que ele cobre (como o
//! `adjustResize` do Android). O UIKit avisa pelo `UIKeyboardWillChangeFrameNotification`
//! com o retângulo final do teclado; a parte dele que cobre a tela vira a faixa de baixo.

use std::sync::atomic::{AtomicU32, Ordering};

use block2::RcBlock;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use objc2_foundation::{CGRect, NSString};

/// Quanto o teclado cobre a tela por baixo, em pontos (bits de um `f32`).
static COVERED: AtomicU32 = AtomicU32::new(0);

/// O que o teclado cobre agora, em pontos (0 sem teclado).
pub(crate) fn covered() -> f32 {
    f32::from_bits(COVERED.load(Ordering::Relaxed))
}

/// Passa a ouvir o teclado; `wake` acorda o laço de eventos a cada mudança.
pub(crate) fn watch(wake: impl Fn() + 'static) {
    unsafe {
        let center: *mut AnyObject = msg_send![class!(NSNotificationCenter), defaultCenter];
        let name = NSString::from_str("UIKeyboardWillChangeFrameNotification");
        let block = RcBlock::new(move |note: *mut AnyObject| {
            let info: *mut AnyObject = msg_send![note, userInfo];
            if info.is_null() {
                return;
            }
            let key = NSString::from_str("UIKeyboardFrameEndUserInfoKey");
            let value: *mut AnyObject = msg_send![info, objectForKey: &*key];
            if value.is_null() {
                return;
            }
            let frame: CGRect = msg_send![value, CGRectValue];
            let screen: *mut AnyObject = msg_send![class!(UIScreen), mainScreen];
            let bounds: CGRect = msg_send![screen, bounds];
            let covered = (bounds.size.height - frame.origin.y).clamp(0.0, bounds.size.height) as f32;
            COVERED.store(covered.to_bits(), Ordering::Relaxed);
            wake();
        });
        let nil = std::ptr::null_mut::<AnyObject>();
        let _: *mut AnyObject = msg_send![center, addObserverForName: &*name, object: nil, queue: nil, usingBlock: &*block];
        // O observador vale enquanto o app existir.
        std::mem::forget(block);
    }
}
