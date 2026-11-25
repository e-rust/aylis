# O que levar ao projeto original

O que oferecer ao `tray-icon` (https://github.com/tauri-apps/tray-icon), sem PR aberto: duas
mudanças independentes (a correção pode ir sozinha, antes), com o texto em inglês e o patch
contra a 0.25.1. Quando as duas estiverem numa versão publicada, a cópia em
`third_party/tray_icon` sai e o `aylis-platform-winit` volta ao crate do crates.io.

---

## 1. fix(windows): `set_tooltip` ignores the GUID of an icon built `with_guid`

`set_icon` and `set_visible` call `apply_guid` before `NIM_MODIFY`, but `set_tooltip` does not.
For an icon registered with `NIF_GUID`, the shell identifies it only by `guidItem`, so the
modify message without `NIF_GUID` addresses `(hWnd, uID)`, which the shell does not know: the
call fails silently (`Shell_NotifyIconW` returns 0, and the tooltip stays the initial one).

Fix: one `apply_guid(&mut nid, self.guid);` in `set_tooltip`, like the other two.

**Tested** on Windows 11 (26300): with a GUID, the tooltip read through UI Automation in the
overflow changes after `set_tooltip`; before the fix it kept the initial text.

## 2. feat(windows): allow a fixed `uID` with `TrayIconBuilder::with_uid`

**Why.** Without `NIF_GUID`, Windows 11 remembers whether a notification icon is shown on the
taskbar or hidden in the overflow ("^") keyed on the executable path plus the icon's `uID`
(`HKCU\Control Panel\NotifyIconSettings`, values `ExecutablePath` and `UID`). Today `uID`
comes from the crate-wide `COUNTER`, which is also used by `TrayIconId::new_unique` (called by
`TrayIconBuilder::new`), so the first icon of a process usually gets `uID = 2`, and the number
shifts with creation order or with any other id taken before. Two consequences:

- an application that replaces an older program installed at the same path (common in
  enterprise deployments, where users already pinned the old icon) cannot inherit the icon's
  slot, because the old program used a different `uID` (often `1`), and the icon lands in the
  overflow again;
- an app with several icons, or that creates them in a different order between versions,
  silently loses the user's choice.

`with_guid` solves this when a GUID can be used, but a GUID is bound to the binary's path
(unless signed) and cannot match a predecessor that registered by `uID`.

**What.** A new `uid: Option<u32>` attribute and `TrayIconBuilder::with_uid(u32)`. On Windows,
`TrayIcon::new` uses it instead of `COUNTER.next()`; other platforms ignore it. Each tray icon
has its own hidden window, so the `(hWnd, uID)` pair stays unique even if two icons share a
number. `TrayIconAttributes` gains a field, which breaks struct literals without
`..Default::default()` (the same as when `guid` was added).

**Tested** on Windows 11 (26300): an icon with `with_uid(1)` registers with `uID = 1`
(`Shell_NotifyIconGetRect` on `(hWnd, 1)` succeeds, and `NotifyIconSettings` records `UID = 1`),
changes icon and tooltip, and is removed on drop; the same with two icons in one process (one
with a fixed `uID` and one with a GUID; one with a fixed `uID` and one with the counter).

## The patch (both changes)

```diff
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -241,6 +241,25 @@
     ///
     /// - **Linux / macOS:** Ignored.
     pub guid: Option<u128>,
+
+    /// A fixed `uID` for the tray icon. **Windows only**.
+    ///
+    /// Without a GUID, Windows keys the per-icon user settings (pinned to the
+    /// taskbar or hidden in the overflow) on the executable path plus this
+    /// `uID`. By default the `uID` comes from a process-wide counter, so it
+    /// depends on how many ids were handed out before (the builder's own
+    /// [`TrayIconId`] takes one too). A fixed number keeps the setting across
+    /// runs regardless of creation order, and lets a program inherit the slot
+    /// of the one it replaces at the same path.
+    ///
+    /// The `uID` only needs to be unique per tray icon window, and every tray
+    /// icon has its own window, so two icons may share a number; Windows will
+    /// then share the remembered setting between them.
+    ///
+    /// ## Platform-specific:
+    ///
+    /// - **Linux / macOS:** Ignored.
+    pub uid: Option<u32>,
 }
 
 impl Default for TrayIconAttributes {
@@ -255,6 +274,7 @@
             menu_on_right_click: true,
             title: None,
             guid: None,
+            uid: None,
         }
     }
 }
@@ -373,6 +393,14 @@
         self
     }
 
+    /// Set a fixed `uID` for this tray icon instead of the internal counter. **Windows only**.
+    ///
+    /// See [`TrayIconAttributes::uid`] for why and how to pick one.
+    pub fn with_uid(mut self, uid: u32) -> Self {
+        self.attrs.uid = Some(uid);
+        self
+    }
+
     /// Access the unique id that will be assigned to the tray icon
     /// this builder will create.
     pub fn id(&self) -> &TrayIconId {
--- a/src/platform_impl/windows/mod.rs
+++ b/src/platform_impl/windows/mod.rs
@@ -94,7 +94,7 @@
 
 impl TrayIcon {
     pub fn new(id: TrayIconId, attrs: TrayIconAttributes) -> crate::Result<Self> {
-        let internal_id = COUNTER.next();
+        let internal_id = attrs.uid.unwrap_or_else(|| COUNTER.next());
         let guid = attrs.guid.map(GUID::from_u128);
 
         let class_name = util::encode_wide("tray_icon_app");
@@ -243,6 +243,7 @@
                     nid.szTip[i] = tip[i];
                 }
             }
+            apply_guid(&mut nid, self.guid);
 
             if Shell_NotifyIconW(NIM_MODIFY, &nid) == 0 {
                 return Err(crate::Error::OsError(std::io::Error::last_os_error()));
```
