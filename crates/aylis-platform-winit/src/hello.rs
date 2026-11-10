//! Windows Hello (rosto, digital ou o PIN do Windows) para o componente `Biometrics`, pelo
//! `UserConsentVerifier` do WinRT. O diálogo do sistema fica preso à janela ativa do app.

use aylis_core::platform::{BiometricResult, BiometricService, Biometry};
use windows::Security::Credentials::UI::{UserConsentVerificationResult, UserConsentVerifier, UserConsentVerifierAvailability};
use windows::Win32::System::WinRT::IUserConsentVerifierInterop;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow;
use windows::Win32::UI::WindowsAndMessaging::{EnumThreadWindows, GetForegroundWindow, IsWindowVisible};
use windows::core::{BOOL, HSTRING, factory};

pub(crate) struct WindowsHello;

/// A janela ativa desta thread (a da interface) ou, sem ela, a primeira visível dela; em
/// último caso a que está em primeiro plano.
fn app_window() -> HWND {
    unsafe extern "system" fn first_visible(hwnd: HWND, found: LPARAM) -> BOOL {
        // SAFETY: `found` aponta para o HWND da função abaixo, vivo durante a enumeração.
        unsafe {
            if IsWindowVisible(hwnd).as_bool() {
                *(found.0 as *mut HWND) = hwnd;
                return BOOL(0);
            }
        }
        BOOL(1)
    }
    // SAFETY: consultas de janelas da própria thread.
    unsafe {
        let active = GetActiveWindow();
        if !active.is_invalid() {
            return active;
        }
        let mut found = HWND::default();
        let _ = EnumThreadWindows(GetCurrentThreadId(), Some(first_visible), LPARAM(&mut found as *mut HWND as isize));
        if !found.is_invalid() { found } else { GetForegroundWindow() }
    }
}

impl BiometricService for WindowsHello {
    fn availability(&self) -> Biometry {
        let available = UserConsentVerifier::CheckAvailabilityAsync().and_then(|op| op.join());
        match available {
            Ok(UserConsentVerifierAvailability::Available) => Biometry::Any,
            _ => Biometry::None,
        }
    }

    fn authenticate(&mut self, reason: &str, _allow_device_credential: bool, done: Box<dyn FnOnce(BiometricResult) + Send>) {
        // A janela do app (o pedido sai da thread da interface): o diálogo fica por cima dela.
        let window = app_window().0 as isize;
        let reason = HSTRING::from(reason);
        // O resultado espera o usuário: fora da thread da interface.
        std::thread::spawn(move || {
            let result = (|| -> windows::core::Result<UserConsentVerificationResult> {
                let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>()?;
                let hwnd = HWND(window as *mut core::ffi::c_void);
                // SAFETY: a janela é do app; o WinRT devolve a operação assíncrona.
                let op: windows_future::IAsyncOperation<UserConsentVerificationResult> = unsafe { interop.RequestVerificationForWindowAsync(hwnd, &reason)? };
                op.join()
            })();
            done(match result {
                Ok(UserConsentVerificationResult::Verified) => BiometricResult::Success,
                Ok(UserConsentVerificationResult::Canceled) => BiometricResult::Cancelled,
                Ok(UserConsentVerificationResult::DeviceNotPresent | UserConsentVerificationResult::NotConfiguredForUser) => {
                    BiometricResult::Failed(aylis_core::messages::t("o Windows Hello não está configurado"))
                }
                Ok(UserConsentVerificationResult::DisabledByPolicy) => BiometricResult::Failed(aylis_core::messages::t("o Windows Hello está desligado por política")),
                Ok(UserConsentVerificationResult::DeviceBusy) => BiometricResult::Failed(aylis_core::messages::t("o leitor está ocupado")),
                Ok(UserConsentVerificationResult::RetriesExhausted) => BiometricResult::Failed(aylis_core::messages::t("tentativas esgotadas")),
                Ok(_) => BiometricResult::Failed(aylis_core::messages::t("não foi possível confirmar")),
                Err(e) => BiometricResult::Failed(e.message()),
            });
        });
    }
}
