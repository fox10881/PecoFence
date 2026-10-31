// The XAML styling engine — `styler_engine.rs` is instantiated once for
// `wux` (Windows.UI.Xaml — taskbar + Start Menu), driven by the
// `Windows.UI.Xaml.dll` diagnostics session.
//
// The module supplies the bindings alias (`xb`), effects module (`fxe`),
// dispatcher plumbing, wire-magic table, and shared-state accessors the
// engine body consumes.

/// Windows.UI.Xaml engine — taskbar (`PECO-TBS1`), Start Menu (`PECO-SMS1`)
/// and Notification Center (`PECO-NCS1`) themes.
pub mod wux {
    use crate::effects as fxe;
    use crate::styler_bindings as xb;

    type XDispatcher = xb::CoreDispatcher;

    /// The wux diagnostics service delivers `OnVisualTreeChange` on the
    /// element's own UI thread — direct styling is correct.
    const MARSHAL_ON_ELEMENT: bool = false;

    /// `IReference<Windows.UI.Text.FontWeight>` signature + runtime name for
    /// the boxed-FontWeight workaround (`Setter.Value` hands back bare ints).
    const FONTWEIGHT_IID: windows_core::GUID =
        windows_core::GUID::from_signature(windows_core::imp::ConstBuffer::from_slice(
            b"pinterface({faa439ea-bffd-4f5b-bb77-9db672644519};struct(Windows.UI.Text.FontWeight;u2))",
        ));
    const FONTWEIGHT_RUNTIME_NAME: &str =
        "Windows.Foundation.IReference`1<Windows.UI.Text.FontWeight>";

    const CONTROLS_NS: &str = "Windows.UI.Xaml.Controls.";
    const SHAPES_NS: &str = "Windows.UI.Xaml.Shapes.";
    const NS_FE: &str = "Windows.UI.Xaml.FrameworkElement";

    fn wire_kind(magic: &str) -> Option<WireKind> {
        Some(match magic {
            "PECO-SMS1" => WireKind::StartMenu,
            "PECO-TBS1" => WireKind::Taskbar,
            "PECO-NCS1" => WireKind::NotificationCenter,
            _ => return None,
        })
    }

    fn elem_dispatcher<T: windows_core::Interface>(el: &T) -> Option<XDispatcher> {
        windows_core::Interface::cast::<xb::DependencyObject>(el)
            .ok()?
            .Dispatcher()
            .ok()
    }

    fn post_ui(
        d: &XDispatcher,
        p: Prio,
        f: impl Fn() -> windows_core::Result<()> + Send + 'static,
    ) {
        let prio = match p {
            Prio::Low => xb::CoreDispatcherPriority::Low,
            Prio::Normal => xb::CoreDispatcherPriority::Normal,
            Prio::High => xb::CoreDispatcherPriority::High,
        };
        let _ = d.RunAsync(prio, &xb::DispatchedHandler::new(f));
    }

    fn mod_diagnostics() -> Option<crate::bindings::IXamlDiagnostics> {
        crate::shared::lock().diagnostics.clone()
    }

    fn mod_service_watcher() -> Option<(
        crate::bindings::IVisualTreeService3,
        crate::bindings::IVisualTreeServiceCallback2,
    )> {
        let s = crate::shared::lock();
        s.tree_service.clone().zip(s.watcher.clone())
    }

    /// `StyleVal::Blur` bridge — the compositor blur+tint brush from
    /// `effects`. Produces `Windows.UI.Xaml` types only; the mux engine
    /// must never route WinUI3 elements through here (`E_NOINTERFACE`
    /// on `SetValue`).
    fn blur_brush(
        ui: &xb::UIElement,
        p: &BlurParams,
    ) -> windows_core::Result<windows_core::IInspectable> {
        windows_core::Interface::cast(&fxe::make_blur_brush_any(ui, p.amount, p.tint)?)
    }

    include!("styler_engine.rs");
}

/// Microsoft.UI.Xaml engine — File Explorer (`PECO-FES1`) themes applied to
/// the WinUI3 islands inside `CabinetWClass`, driven by the second
/// diagnostics session that `Microsoft.Internal.FrameworkUdk.dll` hosts.
/// Same engine body; the bindings alias resolves Microsoft.UI.Xaml.* types.
pub mod mux {
    use crate::mux_bindings as xb;

    type XDispatcher = xb::DispatcherQueue;

    /// The mux (WinUI3) diagnostics service can deliver `OnVisualTreeChange`
    /// on a non-UI thread — `SetValue` there throws RPC_E_WRONG_THREAD, so
    /// element styling is marshalled onto each element's DispatcherQueue.
    const MARSHAL_ON_ELEMENT: bool = true;

    /// `IReference<Windows.UI.Text.FontWeight>` signature + runtime name for
    /// the boxed-FontWeight workaround — WinUI3 still boxes
    /// `Windows.UI.Text.FontWeight`, same IID as wux.
    const FONTWEIGHT_IID: windows_core::GUID =
        windows_core::GUID::from_signature(windows_core::imp::ConstBuffer::from_slice(
            b"pinterface({faa439ea-bffd-4f5b-bb77-9db672644519};struct(Windows.UI.Text.FontWeight;u2))",
        ));
    const FONTWEIGHT_RUNTIME_NAME: &str =
        "Windows.Foundation.IReference`1<Windows.UI.Text.FontWeight>";

    const CONTROLS_NS: &str = "Microsoft.UI.Xaml.Controls.";
    const SHAPES_NS: &str = "Microsoft.UI.Xaml.Shapes.";
    const NS_FE: &str = "Microsoft.UI.Xaml.FrameworkElement";

    fn wire_kind(magic: &str) -> Option<WireKind> {
        Some(match magic {
            "PECO-FES1" => WireKind::FileExplorer,
            _ => return None,
        })
    }

    fn elem_dispatcher<T: windows_core::Interface>(el: &T) -> Option<XDispatcher> {
        windows_core::Interface::cast::<xb::DependencyObject>(el)
            .ok()?
            .DispatcherQueue()
            .ok()
    }

    fn post_ui(
        d: &XDispatcher,
        p: Prio,
        f: impl Fn() -> windows_core::Result<()> + Send + 'static,
    ) {
        let prio = match p {
            Prio::Low => xb::DispatcherQueuePriority::Low,
            Prio::Normal => xb::DispatcherQueuePriority::Normal,
            Prio::High => xb::DispatcherQueuePriority::High,
        };
        let _ = d.TryEnqueueWithPriority(prio, &xb::DispatcherQueueHandler::new(f));
    }

    fn mod_diagnostics() -> Option<crate::bindings::IXamlDiagnostics> {
        crate::shared::lock().diagnostics_winui3.clone()
    }

    fn mod_service_watcher() -> Option<(
        crate::bindings::IVisualTreeService3,
        crate::bindings::IVisualTreeServiceCallback2,
    )> {
        let s = crate::shared::lock();
        s.tree_service_winui3.clone().zip(s.watcher_winui3.clone())
    }

    /// `StyleVal::Blur` bridge for **WinUI3** — `fxe::make_blur_brush_any`
    /// builds a `Windows.UI.Xaml` compositor brush, which `SetValue` rejects
    /// on `Microsoft.UI.Xaml` elements. Resolve a native
    /// `Microsoft.UI.Xaml.Media.AcrylicBrush` through the mux `XamlReader`
    /// instead (WinUI3 `AcrylicBrush` has no `BackgroundSource` — it is
    /// always in-app acrylic). Fallback: a translucent `SolidColorBrush`
    /// whose alpha is clamped to `0x25..=0x40` — never opaque.
    fn blur_brush(
        _ui: &xb::UIElement,
        p: &BlurParams,
    ) -> windows_core::Result<windows_core::IInspectable> {
        let [r, g, b, a] = p.tint;
        let byte = |v: f32| -> u32 { (v.clamp(0.0, 1.0) * 255.0).round() as u32 };
        let tint_rgb = format!("{:02X}{:02X}{:02X}", byte(r), byte(g), byte(b));
        let tint_opacity = a.clamp(0.0, 0.95);
        let alpha = byte(a).clamp(0x25, 0x40);
        let xaml = format!(
            "<AcrylicBrush \
             xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\" \
             TintColor=\"#{tint_rgb}\" TintOpacity=\"{tint_opacity:.2}\" \
             Opacity=\"0.95\" FallbackColor=\"#{alpha:02X}{tint_rgb}\"/>"
        );
        match xb::XamlReader::Load(&windows_core::HSTRING::from(&xaml)) {
            ok @ Ok(_) => ok,
            Err(_) => {
                let xaml = format!(
                    "<SolidColorBrush \
                     xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\" \
                     Color=\"#{alpha:02X}{tint_rgb}\"/>"
                );
                xb::XamlReader::Load(&windows_core::HSTRING::from(&xaml))
            }
        }
    }

    include!("styler_engine.rs");
}
