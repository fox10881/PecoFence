//! Composition-hosted WebView2 (`ICoreWebView2CompositionController`) on a
//! DirectComposition visual tree — the only hosting mode where a WebView can
//! composite with real per-pixel alpha, so the DWM material (Mica/acrylic) on
//! our top-level HWND shows through transparent page regions.
//!
//! `windows-webview` exposes only the HWND controller (which composites an
//! opaque redirection surface), so the composition interfaces are bound here
//! by hand in the crate's own generated style. Input has no child HWND: mouse
//! events are forwarded with `SendMouseInput`, and `MoveFocus` on
//! `WM_SETFOCUS` puts the WebView2 input site in charge of the keyboard.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_transmute_annotations)]
#![allow(clippy::upper_case_acronyms)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use coredesk_platform::{HWND, POINT};
use windows::Win32::Foundation::HWND as DCompHwnd;
use windows::Win32::Graphics::DirectComposition::{
    DCompositionCreateDevice, IDCompositionDevice, IDCompositionTarget, IDCompositionVisual,
};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows_core::{
    HRESULT, HSTRING, IUnknown, IUnknown_Vtbl, Interface, PCWSTR, PWSTR, Ref, Result, implement,
};

windows_core::link!("webview2loader.dll" "system" fn CreateCoreWebView2EnvironmentWithOptions(browserExecutableFolder: PCWSTR, userDataFolder: PCWSTR, environmentOptions: *mut core::ffi::c_void, environmentCreatedHandler: *mut core::ffi::c_void) -> HRESULT);
windows_core::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void);
windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv: *mut core::ffi::c_void));
windows_core::link!("user32.dll" "system" fn GetMessageW(msg: *mut MSG, hwnd: *mut core::ffi::c_void, min: u32, max: u32) -> i32);
windows_core::link!("user32.dll" "system" fn TranslateMessage(msg: *const MSG) -> i32);
windows_core::link!("user32.dll" "system" fn DispatchMessageW(msg: *const MSG) -> isize);

#[repr(C)]
#[derive(Default)]
struct MSG {
    hwnd: *mut core::ffi::c_void,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    pt: POINT,
}

// ---------------------------------------------------------------------------
// Raw COM interfaces, vtable layouts copied slot-for-slot from WebView2.idl
// (SDK 1.0.2903.40). Every interface below is flattened: ancestor methods are
// repeated in declaration order; slots we never call stay `usize`.
// ---------------------------------------------------------------------------

type CF = *mut core::ffi::c_void;

/// The `windows` meta-crate's (windows-core 0.62) `Error` → our 0.100 `Error`:
/// both carry the same `HRESULT`, so only the shell differs.
fn w62(e: windows::core::Error) -> windows_core::Error {
    windows_core::Error::from_hresult(windows_core::HRESULT(e.code().0))
}

windows_core::imp::define_interface!(
    IWv2Environment,
    IWv2Environment_Vtbl,
    0xb96d755e_0319_4e92_a296_23436f46a1fc
);
windows_core::imp::interface_hierarchy!(IWv2Environment, windows_core::IUnknown);
#[repr(C)]
pub struct IWv2Environment_Vtbl {
    pub base__: IUnknown_Vtbl,
    CreateCoreWebView2Controller: usize,
    CreateWebResourceResponse: usize,
    BrowserVersionString: usize,
    add_NewBrowserVersionAvailable: usize,
    remove_NewBrowserVersionAvailable: usize,
}
impl windows_core::RuntimeName for IWv2Environment {}

windows_core::imp::define_interface!(
    IWv2Environment3,
    IWv2Environment3_Vtbl,
    0x80a22ae3_be7c_4ce2_afe1_5a50056cdeeb
);
windows_core::imp::interface_hierarchy!(IWv2Environment3, windows_core::IUnknown);
impl IWv2Environment3 {
    /// `ICoreWebView2Environment3::CreateCoreWebView2CompositionController`.
    pub unsafe fn CreateCoreWebView2CompositionController(
        &self,
        parent: CF,
        handler: *const IWv2CompControllerCompleted,
    ) -> Result<()> {
        unsafe {
            (Interface::vtable(self).CreateCoreWebView2CompositionController)(
                Interface::as_raw(self),
                parent,
                handler,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWv2Environment3_Vtbl {
    pub base__: IUnknown_Vtbl,
    // ICoreWebView2Environment
    CreateCoreWebView2Controller: usize,
    CreateWebResourceResponse: usize,
    BrowserVersionString: usize,
    add_NewBrowserVersionAvailable: usize,
    remove_NewBrowserVersionAvailable: usize,
    // ICoreWebView2Environment2
    CreateWebResourceRequest: usize,
    // ICoreWebView2Environment3
    CreateCoreWebView2CompositionController:
        unsafe extern "system" fn(CF, CF, *const IWv2CompControllerCompleted) -> HRESULT,
    CreateCoreWebView2PointerInfo: usize,
}
impl windows_core::RuntimeName for IWv2Environment3 {}

windows_core::imp::define_interface!(
    IWv2CompositionController,
    IWv2CompositionController_Vtbl,
    0x3df9b733_b9ae_4a15_86b4_eb9ee9826469
);
windows_core::imp::interface_hierarchy!(IWv2CompositionController, windows_core::IUnknown);
impl IWv2CompositionController {
    pub unsafe fn put_RootVisualTarget(&self, target: CF) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_RootVisualTarget)(Interface::as_raw(self), target).ok()
        }
    }
    pub unsafe fn SendMouseInput(
        &self,
        kind: i32,
        virtual_keys: i32,
        mouse_data: u32,
        point: POINT,
    ) -> Result<()> {
        unsafe {
            (Interface::vtable(self).SendMouseInput)(
                Interface::as_raw(self),
                kind,
                virtual_keys,
                mouse_data,
                point,
            )
            .ok()
        }
    }
    pub unsafe fn Cursor(&self) -> Result<HCURSOR> {
        let mut cursor = std::ptr::null_mut();
        unsafe {
            (Interface::vtable(self).get_Cursor)(Interface::as_raw(self), &mut cursor).ok()?;
        }
        Ok(cursor)
    }
    pub unsafe fn SystemCursorId(&self) -> Result<u32> {
        let mut id = 0;
        unsafe {
            (Interface::vtable(self).get_SystemCursorId)(Interface::as_raw(self), &mut id).ok()?;
        }
        Ok(id)
    }
    pub unsafe fn add_CursorChanged(&self, handler: *const IWv2CursorChanged) -> Result<i64> {
        let mut token = 0;
        unsafe {
            (Interface::vtable(self).add_CursorChanged)(
                Interface::as_raw(self),
                handler,
                &mut token,
            )
            .ok()?;
        }
        Ok(token)
    }
}
pub type HCURSOR = *mut core::ffi::c_void;
#[repr(C)]
pub struct IWv2CompositionController_Vtbl {
    pub base__: IUnknown_Vtbl,
    get_RootVisualTarget: usize,
    put_RootVisualTarget: unsafe extern "system" fn(CF, CF) -> HRESULT,
    SendMouseInput: unsafe extern "system" fn(CF, i32, i32, u32, POINT) -> HRESULT,
    SendPointerInput: usize,
    get_Cursor: unsafe extern "system" fn(CF, *mut HCURSOR) -> HRESULT,
    get_SystemCursorId: unsafe extern "system" fn(CF, *mut u32) -> HRESULT,
    add_CursorChanged: unsafe extern "system" fn(CF, *const IWv2CursorChanged, *mut i64) -> HRESULT,
    remove_CursorChanged: usize,
}
impl windows_core::RuntimeName for IWv2CompositionController {}

windows_core::imp::define_interface!(
    IWv2Controller3,
    IWv2Controller3_Vtbl,
    0xf9614724_5d2b_41dc_aef7_73d62b51543b
);
windows_core::imp::interface_hierarchy!(IWv2Controller3, windows_core::IUnknown);
impl IWv2Controller3 {
    pub unsafe fn put_IsVisible(&self, visible: bool) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_IsVisible)(Interface::as_raw(self), visible as i32).ok()
        }
    }
    pub unsafe fn put_Bounds(&self, bounds: RECT) -> Result<()> {
        unsafe { (Interface::vtable(self).put_Bounds)(Interface::as_raw(self), bounds).ok() }
    }
    pub unsafe fn MoveFocus(&self, reason: i32) -> Result<()> {
        unsafe { (Interface::vtable(self).MoveFocus)(Interface::as_raw(self), reason).ok() }
    }
    pub unsafe fn NotifyParentWindowPositionChanged(&self) -> Result<()> {
        unsafe {
            (Interface::vtable(self).NotifyParentWindowPositionChanged)(Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn Close(&self) -> Result<()> {
        unsafe { (Interface::vtable(self).Close)(Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CoreWebView2(&self) -> Result<IWv2> {
        let mut webview = None;
        unsafe {
            (Interface::vtable(self).get_CoreWebView2)(Interface::as_raw(self), &mut webview)
                .ok()?;
        }
        webview.ok_or_else(windows_core::Error::empty)
    }
    pub unsafe fn put_DefaultBackgroundColor(&self, color: Wv2Color) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_DefaultBackgroundColor)(Interface::as_raw(self), color)
                .ok()
        }
    }
    pub unsafe fn put_RasterizationScale(&self, scale: f64) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_RasterizationScale)(Interface::as_raw(self), scale).ok()
        }
    }
    pub unsafe fn put_ShouldDetectMonitorScaleChanges(&self, detect: bool) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_ShouldDetectMonitorScaleChanges)(
                Interface::as_raw(self),
                detect as i32,
            )
            .ok()
        }
    }
    pub unsafe fn put_BoundsMode(&self, mode: i32) -> Result<()> {
        unsafe { (Interface::vtable(self).put_BoundsMode)(Interface::as_raw(self), mode).ok() }
    }
}
/// `COREWEBVIEW2_COLOR`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Wv2Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
pub struct IWv2Controller3_Vtbl {
    pub base__: IUnknown_Vtbl,
    // ICoreWebView2Controller
    get_IsVisible: usize,
    put_IsVisible: unsafe extern "system" fn(CF, i32) -> HRESULT,
    get_Bounds: usize,
    put_Bounds: unsafe extern "system" fn(CF, RECT) -> HRESULT,
    get_ZoomFactor: usize,
    put_ZoomFactor: usize,
    add_ZoomFactorChanged: usize,
    remove_ZoomFactorChanged: usize,
    SetBoundsAndZoomFactor: usize,
    MoveFocus: unsafe extern "system" fn(CF, i32) -> HRESULT,
    add_MoveFocusRequested: usize,
    remove_MoveFocusRequested: usize,
    add_GotFocus: usize,
    remove_GotFocus: usize,
    add_LostFocus: usize,
    remove_LostFocus: usize,
    add_AcceleratorKeyPressed: usize,
    remove_AcceleratorKeyPressed: usize,
    get_ParentWindow: usize,
    put_ParentWindow: usize,
    NotifyParentWindowPositionChanged: unsafe extern "system" fn(CF) -> HRESULT,
    Close: unsafe extern "system" fn(CF) -> HRESULT,
    get_CoreWebView2: unsafe extern "system" fn(CF, *mut Option<IWv2>) -> HRESULT,
    // ICoreWebView2Controller2
    get_DefaultBackgroundColor: usize,
    put_DefaultBackgroundColor: unsafe extern "system" fn(CF, Wv2Color) -> HRESULT,
    // ICoreWebView2Controller3
    get_RasterizationScale: usize,
    put_RasterizationScale: unsafe extern "system" fn(CF, f64) -> HRESULT,
    get_ShouldDetectMonitorScaleChanges: usize,
    put_ShouldDetectMonitorScaleChanges: unsafe extern "system" fn(CF, i32) -> HRESULT,
    add_RasterizationScaleChanged: usize,
    remove_RasterizationScaleChanged: usize,
    get_BoundsMode: usize,
    put_BoundsMode: unsafe extern "system" fn(CF, i32) -> HRESULT,
}
impl windows_core::RuntimeName for IWv2Controller3 {}

windows_core::imp::define_interface!(IWv2, IWv2_Vtbl, 0x76eceacb_0461_4d94_ac83_423a6793775e);
windows_core::imp::interface_hierarchy!(IWv2, windows_core::IUnknown);
impl IWv2 {
    pub unsafe fn Settings(&self) -> Result<IWv2Settings> {
        let mut settings = None;
        unsafe {
            (Interface::vtable(self).Settings)(Interface::as_raw(self), &mut settings).ok()?;
        }
        settings.ok_or_else(windows_core::Error::empty)
    }
    pub unsafe fn NavigateToString(&self, html: &HSTRING) -> Result<()> {
        unsafe {
            (Interface::vtable(self).NavigateToString)(
                Interface::as_raw(self),
                PCWSTR::from_raw(html.as_ptr()),
            )
            .ok()
        }
    }
    pub unsafe fn PostWebMessageAsJson(&self, json: &HSTRING) -> Result<()> {
        unsafe {
            (Interface::vtable(self).PostWebMessageAsJson)(
                Interface::as_raw(self),
                PCWSTR::from_raw(json.as_ptr()),
            )
            .ok()
        }
    }
    pub unsafe fn add_WebMessageReceived(
        &self,
        handler: *const IWv2WebMessageReceived,
    ) -> Result<i64> {
        let mut token = 0;
        unsafe {
            (Interface::vtable(self).add_WebMessageReceived)(
                Interface::as_raw(self),
                handler,
                &mut token,
            )
            .ok()?;
        }
        Ok(token)
    }
}
#[repr(C)]
pub struct IWv2_Vtbl {
    pub base__: IUnknown_Vtbl,
    Settings: unsafe extern "system" fn(CF, *mut Option<IWv2Settings>) -> HRESULT,
    Source: usize,
    Navigate: usize,
    NavigateToString: unsafe extern "system" fn(CF, PCWSTR) -> HRESULT,
    add_NavigationStarting: usize,
    remove_NavigationStarting: usize,
    add_ContentLoading: usize,
    remove_ContentLoading: usize,
    add_SourceChanged: usize,
    remove_SourceChanged: usize,
    add_HistoryChanged: usize,
    remove_HistoryChanged: usize,
    add_NavigationCompleted: usize,
    remove_NavigationCompleted: usize,
    add_FrameNavigationStarting: usize,
    remove_FrameNavigationStarting: usize,
    add_FrameNavigationCompleted: usize,
    remove_FrameNavigationCompleted: usize,
    add_ScriptDialogOpening: usize,
    remove_ScriptDialogOpening: usize,
    add_PermissionRequested: usize,
    remove_PermissionRequested: usize,
    add_ProcessFailed: usize,
    remove_ProcessFailed: usize,
    AddScriptToExecuteOnDocumentCreated: usize,
    RemoveScriptToExecuteOnDocumentCreated: usize,
    ExecuteScript: usize,
    CapturePreview: usize,
    Reload: usize,
    PostWebMessageAsJson: unsafe extern "system" fn(CF, PCWSTR) -> HRESULT,
    PostWebMessageAsString: usize,
    add_WebMessageReceived:
        unsafe extern "system" fn(CF, *const IWv2WebMessageReceived, *mut i64) -> HRESULT,
    remove_WebMessageReceived: usize,
    CallDevToolsProtocolMethod: usize,
    BrowserProcessId: usize,
    CanGoBack: usize,
    CanGoForward: usize,
    GoBack: usize,
    GoForward: usize,
    GetDevToolsProtocolEventReceiver: usize,
    Stop: usize,
    add_NewWindowRequested: usize,
    remove_NewWindowRequested: usize,
    add_DocumentTitleChanged: usize,
    remove_DocumentTitleChanged: usize,
    DocumentTitle: usize,
    AddHostObjectToScript: usize,
    RemoveHostObjectFromScript: usize,
    OpenDevToolsWindow: usize,
    add_ContainsFullScreenElementChanged: usize,
    remove_ContainsFullScreenElementChanged: usize,
    ContainsFullScreenElement: usize,
    add_WebResourceRequested: usize,
    remove_WebResourceRequested: usize,
    AddWebResourceRequestedFilter: usize,
    RemoveWebResourceRequestedFilter: usize,
    add_WindowCloseRequested: usize,
    remove_WindowCloseRequested: usize,
}
impl windows_core::RuntimeName for IWv2 {}

windows_core::imp::define_interface!(
    IWv2Settings,
    IWv2Settings_Vtbl,
    0xe566e521_c5d0_4c92_9b17_5e7d028e0dbe
);
windows_core::imp::interface_hierarchy!(IWv2Settings, windows_core::IUnknown);
impl IWv2Settings {
    pub unsafe fn put_IsStatusBarEnabled(&self, enabled: bool) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_IsStatusBarEnabled)(
                Interface::as_raw(self),
                enabled as i32,
            )
            .ok()
        }
    }
    pub unsafe fn put_AreDefaultContextMenusEnabled(&self, enabled: bool) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_AreDefaultContextMenusEnabled)(
                Interface::as_raw(self),
                enabled as i32,
            )
            .ok()
        }
    }
    pub unsafe fn put_IsZoomControlEnabled(&self, enabled: bool) -> Result<()> {
        unsafe {
            (Interface::vtable(self).put_IsZoomControlEnabled)(
                Interface::as_raw(self),
                enabled as i32,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IWv2Settings_Vtbl {
    pub base__: IUnknown_Vtbl,
    get_IsScriptEnabled: usize,
    put_IsScriptEnabled: usize,
    get_IsWebMessageEnabled: usize,
    put_IsWebMessageEnabled: usize,
    get_AreDefaultScriptDialogsEnabled: usize,
    put_AreDefaultScriptDialogsEnabled: usize,
    get_IsStatusBarEnabled: usize,
    put_IsStatusBarEnabled: unsafe extern "system" fn(CF, i32) -> HRESULT,
    get_AreDevToolsEnabled: usize,
    put_AreDevToolsEnabled: usize,
    get_AreDefaultContextMenusEnabled: usize,
    put_AreDefaultContextMenusEnabled: unsafe extern "system" fn(CF, i32) -> HRESULT,
    get_AreHostObjectsAllowed: usize,
    put_AreHostObjectsAllowed: usize,
    get_IsZoomControlEnabled: usize,
    put_IsZoomControlEnabled: unsafe extern "system" fn(CF, i32) -> HRESULT,
    get_IsBuiltInErrorPageEnabled: usize,
    put_IsBuiltInErrorPageEnabled: usize,
}
impl windows_core::RuntimeName for IWv2Settings {}

windows_core::imp::define_interface!(
    IWv2WebMessageReceivedEventArgs,
    IWv2WebMessageReceivedEventArgs_Vtbl,
    0x0f99a40c_e962_4207_9e92_e3d542eff849
);
windows_core::imp::interface_hierarchy!(IWv2WebMessageReceivedEventArgs, windows_core::IUnknown);
impl IWv2WebMessageReceivedEventArgs {
    /// JSON text of the posted message; freed with `CoTaskMemFree`.
    pub unsafe fn WebMessageAsJson(&self) -> Result<String> {
        let mut raw = PWSTR::null();
        unsafe {
            (Interface::vtable(self).get_WebMessageAsJson)(Interface::as_raw(self), &mut raw)
                .ok()?;
        }
        let text = if raw.is_null() {
            String::new()
        } else {
            unsafe { PCWSTR::from_raw(raw.0).to_string().unwrap_or_default() }
        };
        unsafe { CoTaskMemFree(raw.0.cast()) };
        Ok(text)
    }
}
#[repr(C)]
pub struct IWv2WebMessageReceivedEventArgs_Vtbl {
    pub base__: IUnknown_Vtbl,
    get_Source: usize,
    get_WebMessageAsJson: unsafe extern "system" fn(CF, *mut PWSTR) -> HRESULT,
    TryGetWebMessageAsString: usize,
}
impl windows_core::RuntimeName for IWv2WebMessageReceivedEventArgs {}

// ---------------------------------------------------------------------------
// Handler interfaces (implemented by us through `implement`).
// ---------------------------------------------------------------------------

windows_core::imp::define_interface!(
    IWv2CompControllerCompleted,
    IWv2CompControllerCompleted_Vtbl,
    0x02fab84b_1428_4fb7_ad45_1b2e64736184
);
windows_core::imp::interface_hierarchy!(IWv2CompControllerCompleted, windows_core::IUnknown);
#[repr(C)]
pub struct IWv2CompControllerCompleted_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(CF, HRESULT, CF) -> HRESULT,
}
pub trait IWv2CompControllerCompleted_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<IWv2CompositionController>) -> Result<()>;
}
impl IWv2CompControllerCompleted_Vtbl {
    pub const fn new<Identity: IWv2CompControllerCompleted_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: IWv2CompControllerCompleted_Impl,
            const OFFSET: isize,
        >(
            this: CF,
            errorcode: HRESULT,
            result: CF,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWv2CompControllerCompleted_Impl::Invoke(
                    this,
                    errorcode,
                    core::mem::transmute_copy(&result),
                )
                .into()
            }
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWv2CompControllerCompleted as Interface>::IID
    }
}
impl windows_core::RuntimeName for IWv2CompControllerCompleted {}

windows_core::imp::define_interface!(
    IWv2CursorChanged,
    IWv2CursorChanged_Vtbl,
    0x9da43ccc_26e1_4dad_b56c_d8961c94c571
);
windows_core::imp::interface_hierarchy!(IWv2CursorChanged, windows_core::IUnknown);
#[repr(C)]
pub struct IWv2CursorChanged_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(CF, CF, CF) -> HRESULT,
}
pub trait IWv2CursorChanged_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, sender: Ref<IWv2CompositionController>, args: Ref<IUnknown>) -> Result<()>;
}
impl IWv2CursorChanged_Vtbl {
    pub const fn new<Identity: IWv2CursorChanged_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IWv2CursorChanged_Impl, const OFFSET: isize>(
            this: CF,
            sender: CF,
            args: CF,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWv2CursorChanged_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWv2CursorChanged as Interface>::IID
    }
}
impl windows_core::RuntimeName for IWv2CursorChanged {}

windows_core::imp::define_interface!(
    IWv2WebMessageReceived,
    IWv2WebMessageReceived_Vtbl,
    0x57213f19_00e6_49fa_8e07_898ea01ecbd2
);
windows_core::imp::interface_hierarchy!(IWv2WebMessageReceived, windows_core::IUnknown);
#[repr(C)]
pub struct IWv2WebMessageReceived_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(CF, CF, CF) -> HRESULT,
}
pub trait IWv2WebMessageReceived_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, sender: Ref<IWv2>, args: Ref<IWv2WebMessageReceivedEventArgs>) -> Result<()>;
}
impl IWv2WebMessageReceived_Vtbl {
    pub const fn new<Identity: IWv2WebMessageReceived_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: IWv2WebMessageReceived_Impl,
            const OFFSET: isize,
        >(
            this: CF,
            sender: CF,
            args: CF,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWv2WebMessageReceived_Impl::Invoke(
                    this,
                    core::mem::transmute_copy(&sender),
                    core::mem::transmute_copy(&args),
                )
                .into()
            }
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWv2WebMessageReceived as Interface>::IID
    }
}
impl windows_core::RuntimeName for IWv2WebMessageReceived {}

windows_core::imp::define_interface!(
    IWv2EnvCompleted,
    IWv2EnvCompleted_Vtbl,
    0x4a5c436e_a9e3_4a2e_89c3_910d3513f5cc
);
windows_core::imp::interface_hierarchy!(IWv2EnvCompleted, windows_core::IUnknown);
#[repr(C)]
pub struct IWv2EnvCompleted_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(CF, HRESULT, CF) -> HRESULT,
}
pub trait IWv2EnvCompleted_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<IWv2Environment>) -> Result<()>;
}
impl IWv2EnvCompleted_Vtbl {
    pub const fn new<Identity: IWv2EnvCompleted_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IWv2EnvCompleted_Impl, const OFFSET: isize>(
            this: CF,
            errorcode: HRESULT,
            result: CF,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWv2EnvCompleted_Impl::Invoke(this, errorcode, core::mem::transmute_copy(&result))
                    .into()
            }
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWv2EnvCompleted as Interface>::IID
    }
}
impl windows_core::RuntimeName for IWv2EnvCompleted {}

// ---------------------------------------------------------------------------
// Handler implementations.
// ---------------------------------------------------------------------------

/// Stores the async result so the nested pump can return it.
type Slot<T> = Rc<Cell<Option<Result<T>>>>;

fn slot<T>() -> Slot<T> {
    Rc::new(Cell::new(None))
}

/// WebView2 completes handlers by posting to this thread's queue: pump until
/// the slot is filled. Mirrors windows-webview's own `pump::wait`.
fn pump<T>(slot: &Slot<T>) -> Result<T> {
    let mut message = MSG::default();
    loop {
        if let Some(value) = slot.take() {
            return value;
        }
        unsafe {
            match GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) {
                -1 => return Err(windows_core::Error::from_thread()),
                0 => return Err(windows_core::Error::empty()),
                _ => {
                    let _ = TranslateMessage(&message);
                    let _ = DispatchMessageW(&message);
                }
            }
        }
    }
}

#[implement(IWv2EnvCompleted)]
struct EnvCompleted(Cell<Option<Box<dyn FnOnce(Result<IWv2Environment>)>>>);

impl IWv2EnvCompleted_Impl for EnvCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<IWv2Environment>) -> Result<()> {
        let outcome = errorcode.ok().and_then(|()| result.ok().cloned());
        if let Some(handler) = self.0.take() {
            handler(outcome);
        }
        Ok(())
    }
}

#[implement(IWv2CompControllerCompleted)]
struct CompCompleted(Cell<Option<Box<dyn FnOnce(Result<IWv2CompositionController>)>>>);

impl IWv2CompControllerCompleted_Impl for CompCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<IWv2CompositionController>) -> Result<()> {
        let outcome = errorcode.ok().and_then(|()| result.ok().cloned());
        if let Some(handler) = self.0.take() {
            handler(outcome);
        }
        Ok(())
    }
}

#[implement(IWv2CursorChanged)]
struct CursorChanged(Rc<RefCell<dyn FnMut(&IWv2CompositionController)>>);

impl IWv2CursorChanged_Impl for CursorChanged_Impl {
    fn Invoke(&self, sender: Ref<IWv2CompositionController>, _args: Ref<IUnknown>) -> Result<()> {
        if let Ok(sender) = sender.ok() {
            (self.0.borrow_mut())(sender);
        }
        Ok(())
    }
}

#[implement(IWv2WebMessageReceived)]
struct WebMessageReceived(Rc<dyn Fn(String)>);

impl IWv2WebMessageReceived_Impl for WebMessageReceived_Impl {
    fn Invoke(&self, _sender: Ref<IWv2>, args: Ref<IWv2WebMessageReceivedEventArgs>) -> Result<()> {
        if let Ok(args) = args.ok()
            && let Ok(json) = unsafe { args.WebMessageAsJson() }
        {
            (self.0)(json);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The composition environment + hosted WebView pair.
// ---------------------------------------------------------------------------

// `ICoreWebView2EnvironmentOptions`, implemented by the caller and handed to
// `CreateCoreWebView2EnvironmentWithOptions`. Only the base interface is
// offered — the loader QIs for the newer option interfaces and skips them.
windows_core::imp::define_interface!(
    IWv2EnvironmentOptions,
    IWv2EnvironmentOptions_Vtbl,
    0x2fde08a8_1e9a_4766_8c05_95a9ceb9d1c5
);
windows_core::imp::interface_hierarchy!(IWv2EnvironmentOptions, windows_core::IUnknown);
#[repr(C)]
pub struct IWv2EnvironmentOptions_Vtbl {
    pub base__: IUnknown_Vtbl,
    get_AdditionalBrowserArguments: unsafe extern "system" fn(CF, *mut PWSTR) -> HRESULT,
    put_AdditionalBrowserArguments: unsafe extern "system" fn(CF, PCWSTR) -> HRESULT,
    get_Language: unsafe extern "system" fn(CF, *mut PWSTR) -> HRESULT,
    put_Language: unsafe extern "system" fn(CF, PCWSTR) -> HRESULT,
    get_TargetCompatibleBrowserVersion: unsafe extern "system" fn(CF, *mut PWSTR) -> HRESULT,
    put_TargetCompatibleBrowserVersion: unsafe extern "system" fn(CF, PCWSTR) -> HRESULT,
    get_AllowSingleSignOnUsingOSPrimaryAccount: unsafe extern "system" fn(CF, *mut i32) -> HRESULT,
    put_AllowSingleSignOnUsingOSPrimaryAccount: unsafe extern "system" fn(CF, i32) -> HRESULT,
}
pub trait IWv2EnvironmentOptions_Impl: windows_core::IUnknownImpl {
    fn get_AdditionalBrowserArguments(&self, value: *mut PWSTR) -> Result<()>;
    fn put_AdditionalBrowserArguments(&self, value: &PCWSTR) -> Result<()>;
    fn get_Language(&self, value: *mut PWSTR) -> Result<()>;
    fn put_Language(&self, value: &PCWSTR) -> Result<()>;
    fn get_TargetCompatibleBrowserVersion(&self, value: *mut PWSTR) -> Result<()>;
    fn put_TargetCompatibleBrowserVersion(&self, value: &PCWSTR) -> Result<()>;
    fn get_AllowSingleSignOnUsingOSPrimaryAccount(&self, allow: *mut i32) -> Result<()>;
    fn put_AllowSingleSignOnUsingOSPrimaryAccount(&self, allow: i32) -> Result<()>;
}
impl IWv2EnvironmentOptions_Vtbl {
    pub const fn new<Identity: IWv2EnvironmentOptions_Impl, const OFFSET: isize>() -> Self {
        macro_rules! getter {
            ($name:ident, $trait:ident) => {{
                unsafe extern "system" fn $name<
                    Identity: IWv2EnvironmentOptions_Impl,
                    const OFFSET: isize,
                >(
                    this: CF,
                    value: *mut PWSTR,
                ) -> HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IWv2EnvironmentOptions_Impl::$trait(this, value).into()
                    }
                }
                $name::<Identity, OFFSET>
            }};
        }
        Self {
            base__: IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AdditionalBrowserArguments: getter!(ga, get_AdditionalBrowserArguments),
            put_AdditionalBrowserArguments: {
                unsafe extern "system" fn f<
                    Identity: IWv2EnvironmentOptions_Impl,
                    const OFFSET: isize,
                >(
                    this: CF,
                    a: PCWSTR,
                ) -> HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IWv2EnvironmentOptions_Impl::put_AdditionalBrowserArguments(this, &a).into()
                    }
                }
                f::<Identity, OFFSET>
            },
            get_Language: getter!(gl, get_Language),
            put_Language: {
                unsafe extern "system" fn f<
                    Identity: IWv2EnvironmentOptions_Impl,
                    const OFFSET: isize,
                >(
                    this: CF,
                    a: PCWSTR,
                ) -> HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IWv2EnvironmentOptions_Impl::put_Language(this, &a).into()
                    }
                }
                f::<Identity, OFFSET>
            },
            get_TargetCompatibleBrowserVersion: getter!(gt, get_TargetCompatibleBrowserVersion),
            put_TargetCompatibleBrowserVersion: {
                unsafe extern "system" fn f<
                    Identity: IWv2EnvironmentOptions_Impl,
                    const OFFSET: isize,
                >(
                    this: CF,
                    a: PCWSTR,
                ) -> HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IWv2EnvironmentOptions_Impl::put_TargetCompatibleBrowserVersion(this, &a)
                            .into()
                    }
                }
                f::<Identity, OFFSET>
            },
            get_AllowSingleSignOnUsingOSPrimaryAccount: {
                unsafe extern "system" fn f<
                    Identity: IWv2EnvironmentOptions_Impl,
                    const OFFSET: isize,
                >(
                    this: CF,
                    a: *mut i32,
                ) -> HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IWv2EnvironmentOptions_Impl::get_AllowSingleSignOnUsingOSPrimaryAccount(
                            this, a,
                        )
                        .into()
                    }
                }
                f::<Identity, OFFSET>
            },
            put_AllowSingleSignOnUsingOSPrimaryAccount: {
                unsafe extern "system" fn f<
                    Identity: IWv2EnvironmentOptions_Impl,
                    const OFFSET: isize,
                >(
                    this: CF,
                    a: i32,
                ) -> HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IWv2EnvironmentOptions_Impl::put_AllowSingleSignOnUsingOSPrimaryAccount(
                            this, a,
                        )
                        .into()
                    }
                }
                f::<Identity, OFFSET>
            },
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWv2EnvironmentOptions as Interface>::IID
    }
}
impl windows_core::RuntimeName for IWv2EnvironmentOptions {}

/// App-side options object: only `AdditionalBrowserArguments` carries a
/// payload (debug/test CDP flag); every other getter yields empty/false.
/// CoTaskMem-allocated NUL-terminated UTF-16 copy — ownership transfers to
/// the runtime, which frees it with `CoTaskMemFree`.
unsafe fn alloc_lpwstr(value: &str) -> Result<PWSTR> {
    let encoded: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        let p = CoTaskMemAlloc(encoded.len() * 2) as *mut u16;
        if p.is_null() {
            return Err(windows_core::Error::from_hresult(HRESULT(-2147024882)));
        }
        std::ptr::copy_nonoverlapping(encoded.as_ptr(), p, encoded.len());
        Ok(PWSTR(p))
    }
}

#[implement(IWv2EnvironmentOptions)]
struct EnvOptions {
    browser_args: String,
}

impl IWv2EnvironmentOptions_Impl for EnvOptions_Impl {
    fn get_AdditionalBrowserArguments(&self, value: *mut PWSTR) -> Result<()> {
        // Out strings from caller-implemented options are CoTaskMem-owned by
        // the runtime once read.
        unsafe { *value = alloc_lpwstr(&self.browser_args)? };
        Ok(())
    }
    fn put_AdditionalBrowserArguments(&self, _v: &PCWSTR) -> Result<()> {
        Ok(())
    }
    fn get_Language(&self, value: *mut PWSTR) -> Result<()> {
        unsafe { *value = alloc_lpwstr("")? };
        Ok(())
    }
    fn put_Language(&self, _v: &PCWSTR) -> Result<()> {
        Ok(())
    }
    fn get_TargetCompatibleBrowserVersion(&self, value: *mut PWSTR) -> Result<()> {
        // WebView2 rejects an empty value — the GA baseline accepts every
        // supported runtime (mirrors windows-webview's own default).
        unsafe { *value = alloc_lpwstr("86.0.616.0")? };
        Ok(())
    }
    fn put_TargetCompatibleBrowserVersion(&self, _v: &PCWSTR) -> Result<()> {
        Ok(())
    }
    fn get_AllowSingleSignOnUsingOSPrimaryAccount(&self, allow: *mut i32) -> Result<()> {
        unsafe { *allow = 0 };
        Ok(())
    }
    fn put_AllowSingleSignOnUsingOSPrimaryAccount(&self, _allow: i32) -> Result<()> {
        Ok(())
    }
}

/// The shared WebView2 environment — one for the whole process, kept alive
/// for the lifetime of the app (the browser processes exit when the last
/// controller closes).
pub struct CompEnvironment {
    env3: IWv2Environment3,
}

impl CompEnvironment {
    /// Creates the environment, pumping the UI thread until the loader calls
    /// back. `browser_args` feeds `AdditionalBrowserArguments` (debug CDP).
    pub fn create(user_data: &std::path::Path, browser_args: &str) -> Result<Self> {
        let _ = std::fs::create_dir_all(user_data);
        let udf = HSTRING::from(user_data.to_string_lossy().as_ref());
        let options: IWv2EnvironmentOptions = EnvOptions {
            browser_args: browser_args.to_string(),
        }
        .into();
        let slot: Slot<IWv2Environment> = slot();
        {
            let sink = slot.clone();
            let handler: IWv2EnvCompleted = EnvCompleted(Cell::new(Some(Box::new(move |r| {
                sink.set(Some(r));
            }))))
            .into();
            let mut hr = unsafe {
                CreateCoreWebView2EnvironmentWithOptions(
                    PCWSTR::null(),
                    PCWSTR::from_raw(udf.as_ptr()),
                    options.as_raw() as CF,
                    handler.as_raw() as CF,
                )
            };
            tracing::debug!(?hr, "webview_comp: env create (with options)");
            if hr.is_err() {
                hr = unsafe {
                    CreateCoreWebView2EnvironmentWithOptions(
                        PCWSTR::null(),
                        PCWSTR::from_raw(udf.as_ptr()),
                        std::ptr::null_mut(),
                        handler.as_raw() as CF,
                    )
                };
                tracing::debug!(?hr, "webview_comp: env create (null options)");
            }
            hr.ok()?;
        }
        let env = pump(&slot)?;
        let env3 = env.cast()?;
        Ok(Self { env3 })
    }
}

/// WebView2's default background: fully transparent so the host window's DWM
/// material shows through wherever the page does not paint.
const TRANSPARENT: Wv2Color = Wv2Color {
    a: 0,
    r: 0,
    g: 0,
    b: 0,
};

/// A WebView hosted by `ICoreWebView2CompositionController` inside our own
/// DirectComposition visual tree. `close()` must run before the host HWND
/// dies (call it from `WM_DESTROY`).
pub struct CompWebview {
    comp: IWv2CompositionController,
    ctl3: IWv2Controller3,
    webview: IWv2,
    device: IDCompositionDevice,
    target: IDCompositionTarget,
    /// Kept for lifetime only — `RootVisualTarget` owns the on-screen role.
    _visual: IDCompositionVisual,
    cursor: Rc<Cell<HCURSOR>>,
    _env: IWv2Environment3,
    _registrations: Vec<i64>,
    _handlers: Vec<IUnknown>,
}

impl CompWebview {
    /// Creates the DComp tree + composition controller, wires messaging and
    /// input, and navigates to `html`. `on_message` receives each posted JSON.
    pub fn create(
        env: &CompEnvironment,
        hwnd: HWND,
        dpi: u32,
        client: (i32, i32),
        html: &str,
        on_message: impl Fn(String) + 'static,
    ) -> Result<Self> {
        // 1. DirectComposition device + target + root visual.
        let device: IDCompositionDevice =
            unsafe { DCompositionCreateDevice(Option::<&IDXGIDevice>::None).map_err(w62)? };
        let target = unsafe {
            device
                .CreateTargetForHwnd(DCompHwnd(hwnd.0), true)
                .map_err(w62)?
        };
        let visual = unsafe { device.CreateVisual().map_err(w62)? };
        unsafe { target.SetRoot(&visual).map_err(w62)? };

        // 2. The composition controller (nested pump until the handler fires).
        let slot: Slot<IWv2CompositionController> = slot();
        {
            let slot = slot.clone();
            let handler: IWv2CompControllerCompleted =
                CompCompleted(Cell::new(Some(Box::new(move |r| {
                    slot.set(Some(r));
                }))))
                .into();
            unsafe {
                env.env3.CreateCoreWebView2CompositionController(
                    hwnd.0,
                    Interface::as_raw(&handler) as *const IWv2CompControllerCompleted,
                )?;
            }
        }
        let comp = pump(&slot)?;
        let ctl3: IWv2Controller3 = comp.cast()?;
        let webview = unsafe { ctl3.CoreWebView2()? };

        // 3. Page hosting switches: transparent surface + raw-pixel bounds.
        unsafe {
            ctl3.put_DefaultBackgroundColor(TRANSPARENT)?;
            ctl3.put_BoundsMode(0)?; // COREWEBVIEW2_BOUNDS_MODE_USE_RAW_PIXELS
            let (w, h) = client;
            ctl3.put_Bounds(RECT {
                left: 0,
                top: 0,
                right: w,
                bottom: h,
            })?;
            ctl3.put_RasterizationScale(f64::from(dpi) / 96.0)?;
            ctl3.put_ShouldDetectMonitorScaleChanges(true)?;
            if let Ok(settings) = webview.Settings() {
                let _ = settings.put_AreDefaultContextMenusEnabled(false);
                let _ = settings.put_IsStatusBarEnabled(false);
                let _ = settings.put_IsZoomControlEnabled(false);
            }
        }

        // 4. Page→host JSON and the cursor feed.
        let mut handlers: Vec<IUnknown> = Vec::new();
        let mut tokens = Vec::new();
        {
            let on_message: Rc<dyn Fn(String)> = Rc::new(on_message);
            let h: IWv2WebMessageReceived = WebMessageReceived(on_message).into();
            if let Ok(t) = unsafe {
                webview
                    .add_WebMessageReceived(Interface::as_raw(&h) as *const IWv2WebMessageReceived)
            } {
                tokens.push(t);
                handlers.push(h.cast()?);
            }
        }
        let cursor = Rc::new(Cell::new(std::ptr::null_mut()));
        {
            let comp_rc = comp.clone();
            let cursor_rc = cursor.clone();
            let update: Rc<RefCell<dyn FnMut(&IWv2CompositionController)>> =
                Rc::new(RefCell::new(move |sender: &IWv2CompositionController| {
                    let _ = sender;
                    let c = unsafe { comp_rc.Cursor().unwrap_or(std::ptr::null_mut()) };
                    cursor_rc.set(c);
                }));
            let h: IWv2CursorChanged = CursorChanged(update).into();
            if let Ok(t) =
                unsafe { comp.add_CursorChanged(Interface::as_raw(&h) as *const IWv2CursorChanged) }
            {
                tokens.push(t);
                handlers.push(h.cast()?);
            }
        }

        // 5. Root visual target → visible → commit.
        unsafe {
            comp.put_RootVisualTarget(windows::core::Interface::as_raw(&visual) as CF)?;
            ctl3.put_IsVisible(true)?;
            device.Commit().map_err(w62)?;
        }

        // 6. Load the page.
        unsafe {
            webview.NavigateToString(&HSTRING::from(html))?;
        }

        Ok(Self {
            comp,
            ctl3,
            webview,
            device,
            target,
            _visual: visual,
            cursor,
            _env: env.env3.clone(),
            _registrations: tokens,
            _handlers: handlers,
        })
    }

    /// Resize the WebView to the raw-pixel client rect (`WM_SIZE`).
    pub fn set_bounds(&self, w: i32, h: i32) {
        unsafe {
            let _ = self.ctl3.put_Bounds(RECT {
                left: 0,
                top: 0,
                right: w,
                bottom: h,
            });
        }
    }

    pub fn set_rasterization_scale(&self, dpi: u32) {
        unsafe {
            let _ = self.ctl3.put_RasterizationScale(f64::from(dpi) / 96.0);
        }
    }

    /// Re-asserts the transparent surface after a desktop composition-band
    /// rebuild (theme apply re-creates the wallpaper + effects surfaces and
    /// the WebView's companion window can lose alpha to the churn):
    /// transparent background, the visual → target attach, a bounds/dpi
    /// re-sync, and a `Commit` so a detached tree re-binds. Idempotent.
    pub fn rearm_transparency(&self, dpi: u32, client: (i32, i32)) {
        unsafe {
            let _ = self.ctl3.put_DefaultBackgroundColor(TRANSPARENT);
            let _ = self.target.SetRoot(&self._visual);
            let _ = self
                .comp
                .put_RootVisualTarget(windows::core::Interface::as_raw(&self._visual) as CF);
            self.set_bounds(client.0, client.1);
            self.set_rasterization_scale(dpi);
            self.notify_parent_window_position_changed();
            let _ = self.device.Commit();
        }
    }

    /// `WM_SETFOCUS` — hands the keyboard to the WebView2 input site; without
    /// it a composition-hosted page never sees a key.
    pub fn move_focus(&self) {
        unsafe {
            let _ = self.ctl3.MoveFocus(0); // PROGRAMMATIC
        }
    }

    pub fn notify_parent_window_position_changed(&self) {
        unsafe {
            let _ = self.ctl3.NotifyParentWindowPositionChanged();
        }
    }

    /// `WM_MOUSEMOVE`/buttons/wheel/leave — forwarded as `SendMouseInput`.
    /// `vk` is the message's `MK_*` low word (values match the
    /// `COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS` bit order).
    pub fn send_mouse(&self, kind: i32, vk: i32, mouse_data: u32, x: i32, y: i32) {
        unsafe {
            let _ = self
                .comp
                .SendMouseInput(kind, vk, mouse_data, POINT { x, y });
        }
    }

    /// The cursor the page last asked for (`WM_SETCURSOR` uses it; null →
    /// fall back to `SystemCursorId`'s stock cursor).
    pub fn cursor(&self) -> HCURSOR {
        self.cursor.get()
    }

    pub fn system_cursor_id(&self) -> u32 {
        unsafe { self.comp.SystemCursorId().unwrap_or(0) }
    }

    /// Forwards a host wndproc message to `SendMouseInput`. Returns `true`
    /// when the message was consumed (host returns 0). `kind` values match
    /// `COREWEBVIEW2_MOUSE_EVENT_KIND`, which mirrors the Win32 message ids
    /// for the same events.
    ///
    /// `track_mouse_leave`/`SetCapture`/`ReleaseCapture` live here so both
    /// hosts behave identically.
    pub fn forward_wm(
        &self,
        hwnd: HWND,
        message: u32,
        wparam: usize,
        lparam: isize,
        tracked: &std::cell::Cell<bool>,
    ) -> bool {
        use coredesk_platform::{msg, window};
        const MOUSE_KIND: &[(u32, i32)] = &[
            (0x0200, 512), // WM_MOUSEMOVE
            (0x0201, 513), // WM_LBUTTONDOWN
            (0x0202, 514), // WM_LBUTTONUP
            (0x0203, 515), // WM_LBUTTONDBLCLK
            (0x0204, 516), // WM_RBUTTONDOWN
            (0x0205, 517), // WM_RBUTTONUP
            (0x0206, 518), // WM_RBUTTONDBLCLK
            (0x0207, 519), // WM_MBUTTONDOWN
            (0x0208, 520), // WM_MBUTTONUP
            (0x0209, 521), // WM_MBUTTONDBLCLK
            (0x020B, 523), // WM_XBUTTONDOWN
            (0x020C, 524), // WM_XBUTTONUP
            (0x020D, 525), // WM_XBUTTONDBLCLK
            (0x020A, 522), // WM_MOUSEWHEEL
            (0x020E, 526), // WM_MOUSEHWHEEL
        ];
        match message {
            msg::WM_MOUSELEAVE => {
                tracked.set(false);
                self.send_mouse(675, 0, 0, -1, -1);
                true
            }
            msg::WM_SETCURSOR => true, // handled by the host (needs the cursor cell)
            _ => {
                let Some(&(_, kind)) = MOUSE_KIND.iter().find(|(m, _)| *m == message) else {
                    return false;
                };
                // MOUSEMOVE: arm leave tracking once; click: capture so the
                // page still sees the matching up event off-client.
                if message == msg::WM_MOUSEMOVE && !tracked.replace(true) {
                    window::track_mouse_leave(hwnd);
                }
                if matches!(message, 0x0201 | 0x0204 | 0x0207 | 0x020B) {
                    window::set_capture(hwnd);
                } else if matches!(message, 0x0202 | 0x0205 | 0x0208 | 0x020C) {
                    window::release_capture();
                }
                // Wheel lparam is screen coords; X data rides in wparam hi.
                let (x, y) = if matches!(message, 0x020A | 0x020E) {
                    let pt = window::screen_to_client(
                        hwnd,
                        POINT {
                            x: msg::lo_i16(lparam),
                            y: msg::hi_i16(lparam),
                        },
                    );
                    (pt.x, pt.y)
                } else {
                    (msg::lo_i16(lparam), msg::hi_i16(lparam))
                };
                let mouse_data = match message {
                    0x020A | 0x020E => (msg::hi_u16(wparam) as u16 as i16) as i32 as u32,
                    0x020B..=0x020D => msg::hi_u16(wparam),
                    _ => 0,
                };
                self.send_mouse(kind, msg::lo_u16(wparam) as i32, mouse_data, x, y);
                true
            }
        }
    }

    /// JSON to the page (`post_json` in the old hosts).
    pub fn post_json(&self, json: &str) {
        unsafe {
            let _ = self.webview.PostWebMessageAsJson(&HSTRING::from(json));
        }
    }

    /// `WM_DESTROY`: close the WebView first, then detach the root visual and
    /// commit — reversing the attach order keeps DComp from holding a dead
    /// WebView visual.
    pub fn close(self) {
        unsafe {
            let _ = self.ctl3.Close();
            let _ = self
                .target
                .SetRoot(Option::<&IDCompositionVisual>::None)
                .map_err(w62);
            let _ = self.device.Commit().map_err(w62);
        }
    }
}
