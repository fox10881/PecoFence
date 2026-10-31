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

    include!("styler_engine.rs");
}

/// Microsoft.UI.Xaml engine — File Explorer (`PECO-FES1`) themes applied to
/// the WinUI3 islands inside `CabinetWClass`, driven by the second
/// diagnostics session that `Microsoft.Internal.FrameworkUdk.dll` hosts.
/// Same engine body; the bindings alias resolves Microsoft.UI.Xaml.* types.
pub mod mux {
    use crate::effects as fxe;
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

    include!("styler_engine.rs");
}
