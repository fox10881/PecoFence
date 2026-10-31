//! Port of TranslucentTB's `ExplorerTAP/XamlBlurBrush.cpp` + `effects/*`.
//!
//! TTB's Win 11 Blur is NOT an `AcrylicBrush` and NOT a DWM accent — it is a
//! `XamlCompositionBrushBase` whose `CompositionBrush` is a real Composition
//! effect: `CreateBackdropBrush()` -> `GaussianBlur` -> `Flood` tint, joined
//! by `Composite` in `SOURCE_OVER` (see `XamlBlurBrush::OnConnected`). The
//! D2D effect descriptions are custom COM objects implementing
//! `IGraphicsEffect` + `IGraphicsEffectSource` + `IGraphicsEffectD2DInterop`,
//! ported 1:1 from `effects/GaussianBlurEffect.cpp`, `FloodEffect.cpp` and
//! `CompositeEffect.cpp`.

use crate::bindings::*;
use crate::fx_bindings::{
    IGraphicsEffect, IGraphicsEffect_Impl, IGraphicsEffectSource, IGraphicsEffectSource_Impl,
    IXamlCompositionBrushBaseOverrides, IXamlCompositionBrushBaseOverrides_Vtbl,
    IXamlCompositionBrushBaseProtected,
};
use windows_core::implement;

// `IGraphicsEffectD2DInterop` is header-only (`windows.graphics.effects.
// interop.h`) and absent from the winmd, so it is hand-declared here in the
// same shape windows-bindgen emits (see the `IClassFactory` block).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING =
    GRAPHICS_EFFECT_PROPERTY_MAPPING(1);

windows_core::imp::define_interface!(
    IGraphicsEffectD2DInterop,
    IGraphicsEffectD2DInterop_Vtbl,
    0x2fc57384_a068_44d7_a331_30982fcf7177
);
windows_core::imp::interface_hierarchy!(IGraphicsEffectD2DInterop, windows_core::IUnknown);

pub trait IGraphicsEffectD2DInterop_Impl: windows_core::IUnknownImpl {
    fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetNamedPropertyMapping(
        &self,
        name: &windows_core::PCWSTR,
        index: *mut u32,
        mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
    ) -> windows_core::Result<()>;
    fn GetPropertyCount(&self) -> windows_core::Result<u32>;
    fn GetProperty(&self, index: u32) -> windows_core::Result<windows_core::IInspectable>;
    fn GetSource(&self, index: u32) -> windows_core::Result<IGraphicsEffectSource>;
    fn GetSourceCount(&self) -> windows_core::Result<u32>;
}

#[repr(C)]
pub struct IGraphicsEffectD2DInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEffectId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetNamedPropertyMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut u32,
        *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
    ) -> windows_core::HRESULT,
    pub GetPropertyCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSourceCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}

impl IGraphicsEffectD2DInterop_Vtbl {
    pub const fn new<Identity: IGraphicsEffectD2DInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEffectId<
            Identity: IGraphicsEffectD2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            id: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2DInterop_Impl::GetEffectId(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNamedPropertyMapping<
            Identity: IGraphicsEffectD2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: windows_core::PCWSTR,
            index: *mut u32,
            mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsEffectD2DInterop_Impl::GetNamedPropertyMapping(
                    this,
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&mapping),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPropertyCount<
            Identity: IGraphicsEffectD2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            count: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2DInterop_Impl::GetPropertyCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<
            Identity: IGraphicsEffectD2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            value: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2DInterop_Impl::GetProperty(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<
            Identity: IGraphicsEffectD2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            source: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2DInterop_Impl::GetSource(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        source.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceCount<
            Identity: IGraphicsEffectD2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            count: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2DInterop_Impl::GetSourceCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEffectId: GetEffectId::<Identity, OFFSET>,
            GetNamedPropertyMapping: GetNamedPropertyMapping::<Identity, OFFSET>,
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsEffectD2DInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGraphicsEffectD2DInterop {}

// `IXamlCompositionBrushBaseOverrides` landed in fx_bindings.rs without an
// `_Impl` trait (bindgen only emits call-site surface for it), so the
// implementation side is supplied here — needed to derive from
// `XamlCompositionBrushBase` like TTB's `XamlBlurBrush`.
pub trait IXamlCompositionBrushBaseOverrides_Impl: windows_core::IUnknownImpl {
    fn OnConnected(&self) -> windows_core::Result<()>;
    fn OnDisconnected(&self) -> windows_core::Result<()>;
}

impl IXamlCompositionBrushBaseOverrides_Vtbl {
    pub const fn new<Identity: IXamlCompositionBrushBaseOverrides_Impl, const OFFSET: isize>()
    -> Self {
        unsafe extern "system" fn OnConnected<
            Identity: IXamlCompositionBrushBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlCompositionBrushBaseOverrides_Impl::OnConnected(this).into()
            }
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: IXamlCompositionBrushBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlCompositionBrushBaseOverrides_Impl::OnDisconnected(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IXamlCompositionBrushBaseOverrides,
                OFFSET,
            >(),
            OnConnected: OnConnected::<Identity, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseOverrides as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXamlCompositionBrushBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.IXamlCompositionBrushBaseOverrides";
}

// ---- D2D constants (d2d1effects.h / d2d1_1.h), verbatim from TTB ----
const CLSID_D2D1_GAUSSIAN_BLUR: windows_core::GUID =
    windows_core::GUID::from_u128(0x1feb6d69_2fe6_4ac9_8c58_1d7f93e7a6a5);
const CLSID_D2D1_FLOOD: windows_core::GUID =
    windows_core::GUID::from_u128(0x61c23c20_ae69_4d8e_94cf_50078df638f2);
const CLSID_D2D1_COMPOSITE: windows_core::GUID =
    windows_core::GUID::from_u128(0x48fc9f51_f6ac_48f1_8b58_3b28ac46f76d);

const D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION: u32 = 0;
const D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION: u32 = 1;
const D2D1_GAUSSIANBLUR_PROP_BORDER_MODE: u32 = 2;
const D2D1_FLOOD_PROP_COLOR: u32 = 0;
const D2D1_COMPOSITE_PROP_MODE: u32 = 0;

const D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED: u32 = 1;
const D2D1_BORDER_MODE_SOFT: u32 = 0;
const D2D1_COMPOSITE_MODE_SOURCE_OVER: u32 = 0;

const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057u32 as _);
const E_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x8000000Bu32 as _);

fn wide_eq(name: &windows_core::PCWSTR, lit: &str) -> bool {
    unsafe { name.to_string().map(|s| s == lit).unwrap_or(false) }
}

/// `GaussianBlurEffect` — TTB `effects/GaussianBlurEffect.h`.
#[implement(IGraphicsEffect, IGraphicsEffectSource, IGraphicsEffectD2DInterop)]
struct GaussianBlurEffect {
    name: windows_core::HSTRING,
    source: IGraphicsEffectSource,
    blur_amount: f32,
}

impl IGraphicsEffect_Impl for GaussianBlurEffect_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.name.clone())
    }
    fn SetName(&self, _value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Ok(())
    }
}

impl IGraphicsEffectSource_Impl for GaussianBlurEffect_Impl {}

impl IGraphicsEffectD2DInterop_Impl for GaussianBlurEffect_Impl {
    fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID> {
        Ok(CLSID_D2D1_GAUSSIAN_BLUR)
    }
    fn GetNamedPropertyMapping(
        &self,
        name: &windows_core::PCWSTR,
        index: *mut u32,
        mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
    ) -> windows_core::Result<()> {
        if index.is_null() || mapping.is_null() {
            return Err(E_INVALIDARG.into());
        }
        let prop = if wide_eq(name, "BlurAmount") {
            D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION
        } else if wide_eq(name, "Optimization") {
            D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION
        } else if wide_eq(name, "BorderMode") {
            D2D1_GAUSSIANBLUR_PROP_BORDER_MODE
        } else {
            return Err(E_INVALIDARG.into());
        };
        unsafe {
            *index = prop;
            *mapping = GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT;
        }
        Ok(())
    }
    fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        Ok(3)
    }
    fn GetProperty(&self, index: u32) -> windows_core::Result<windows_core::IInspectable> {
        match index {
            D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION => {
                crate::fx_bindings::PropertyValue::CreateSingle(self.blur_amount)
            }
            D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION => crate::fx_bindings::PropertyValue::CreateUInt32(
                D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED,
            ),
            D2D1_GAUSSIANBLUR_PROP_BORDER_MODE => {
                crate::fx_bindings::PropertyValue::CreateUInt32(D2D1_BORDER_MODE_SOFT)
            }
            _ => Err(E_BOUNDS.into()),
        }
    }
    fn GetSource(&self, index: u32) -> windows_core::Result<IGraphicsEffectSource> {
        if index == 0 {
            Ok(self.source.clone())
        } else {
            Err(E_BOUNDS.into())
        }
    }
    fn GetSourceCount(&self) -> windows_core::Result<u32> {
        Ok(1)
    }
}

/// `FloodEffect` — TTB `effects/FloodEffect.h`.
#[implement(IGraphicsEffect, IGraphicsEffectSource, IGraphicsEffectD2DInterop)]
struct FloodEffect {
    name: windows_core::HSTRING,
    color: [f32; 4],
}

impl IGraphicsEffect_Impl for FloodEffect_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.name.clone())
    }
    fn SetName(&self, _value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Ok(())
    }
}

impl IGraphicsEffectSource_Impl for FloodEffect_Impl {}

impl IGraphicsEffectD2DInterop_Impl for FloodEffect_Impl {
    fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID> {
        Ok(CLSID_D2D1_FLOOD)
    }
    fn GetNamedPropertyMapping(
        &self,
        name: &windows_core::PCWSTR,
        index: *mut u32,
        mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
    ) -> windows_core::Result<()> {
        if index.is_null() || mapping.is_null() || !wide_eq(name, "Color") {
            return Err(E_INVALIDARG.into());
        }
        unsafe {
            *index = D2D1_FLOOD_PROP_COLOR;
            *mapping = GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT;
        }
        Ok(())
    }
    fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        Ok(1)
    }
    fn GetProperty(&self, index: u32) -> windows_core::Result<windows_core::IInspectable> {
        if index == D2D1_FLOOD_PROP_COLOR {
            crate::fx_bindings::PropertyValue::CreateSingleArray(&self.color)
        } else {
            Err(E_BOUNDS.into())
        }
    }
    fn GetSource(&self, _index: u32) -> windows_core::Result<IGraphicsEffectSource> {
        Err(E_BOUNDS.into())
    }
    fn GetSourceCount(&self) -> windows_core::Result<u32> {
        Ok(0)
    }
}

/// `CompositeEffect` — TTB `effects/CompositeEffect.h`.
#[implement(IGraphicsEffect, IGraphicsEffectSource, IGraphicsEffectD2DInterop)]
struct CompositeEffect {
    name: windows_core::HSTRING,
    sources: Vec<IGraphicsEffectSource>,
    mode: u32,
}

impl IGraphicsEffect_Impl for CompositeEffect_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.name.clone())
    }
    fn SetName(&self, _value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Ok(())
    }
}

impl IGraphicsEffectSource_Impl for CompositeEffect_Impl {}

impl IGraphicsEffectD2DInterop_Impl for CompositeEffect_Impl {
    fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID> {
        Ok(CLSID_D2D1_COMPOSITE)
    }
    fn GetNamedPropertyMapping(
        &self,
        name: &windows_core::PCWSTR,
        index: *mut u32,
        mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
    ) -> windows_core::Result<()> {
        if index.is_null() || mapping.is_null() || !wide_eq(name, "Mode") {
            return Err(E_INVALIDARG.into());
        }
        unsafe {
            *index = D2D1_COMPOSITE_PROP_MODE;
            *mapping = GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT;
        }
        Ok(())
    }
    fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        Ok(1)
    }
    fn GetProperty(&self, index: u32) -> windows_core::Result<windows_core::IInspectable> {
        if index == D2D1_COMPOSITE_PROP_MODE {
            crate::fx_bindings::PropertyValue::CreateUInt32(self.mode)
        } else {
            Err(E_BOUNDS.into())
        }
    }
    fn GetSource(&self, index: u32) -> windows_core::Result<IGraphicsEffectSource> {
        self.sources
            .get(index as usize)
            .cloned()
            .ok_or_else(|| E_BOUNDS.into())
    }
    fn GetSourceCount(&self) -> windows_core::Result<u32> {
        Ok(self.sources.len() as u32)
    }
}

/// The "derived" half of the composed `XamlCompositionBrushBase` — TTB's
/// `XamlBlurBrush` itself. The composition brush is attached eagerly via the
/// protected interface, so the overrides are inert; they exist because a
/// composable factory call requires a non-empty implementation part.
#[implement(IXamlCompositionBrushBaseOverrides)]
struct BlurBrushOuter;

impl IXamlCompositionBrushBaseOverrides_Impl for BlurBrushOuter_Impl {
    fn OnConnected(&self) -> windows_core::Result<()> {
        Ok(())
    }
    fn OnDisconnected(&self) -> windows_core::Result<()> {
        Ok(())
    }
}

/// `XamlBlurBrush::OnConnected`, verbatim: backdrop brush -> gaussian blur ->
/// flood tint -> composite SOURCE_OVER, handed to a composed
/// `XamlCompositionBrushBase` whose `CompositionBrush` is the effect brush.
/// `element` supplies the compositor via `ElementCompositionPreview` — the
/// same `GetElementVisual(control).Compositor()` TTB uses.
/// Must run on the XAML UI thread.
pub fn make_blur_brush(
    element: &Rectangle,
    blur_amount: f32,
    tint: [f32; 4],
) -> windows_core::Result<Brush> {
    let element = windows_core::Interface::cast::<UIElement>(element)?;
    make_blur_brush_any(&element, blur_amount, tint)
}

/// Same brush for any `UIElement` — the styler's `<WindhawkBlur/>` values
/// target elements of arbitrary type, not just taskbar `Rectangle`s.
/// Generic over the caller's projection type (`bindings`/`styler_bindings`
/// emit separate `UIElement`s over the same IID).
pub fn make_blur_brush_any<T: windows_core::Interface>(
    element: &T,
    blur_amount: f32,
    tint: [f32; 4],
) -> windows_core::Result<Brush> {
    let element: UIElement = windows_core::Interface::cast(element)?;
    let visual = ElementCompositionPreview::GetElementVisual(&element)?;
    let compositor = visual.Compositor()?;
    let comp_brush = make_effect_brush(&compositor, blur_amount, tint)?;

    let xcbb = XamlCompositionBrushBase::compose(BlurBrushOuter)?;
    let protected: IXamlCompositionBrushBaseProtected = windows_core::Interface::cast(&xcbb)?;
    unsafe {
        (windows_core::Interface::vtable(&protected).SetCompositionBrush)(
            windows_core::Interface::as_raw(&protected),
            windows_core::Interface::as_raw(&comp_brush),
        )
        .ok()?;
    }
    windows_core::Interface::cast(&xcbb)
}

/// The effect-graph half, separated so a test can drive it with a local
/// `Compositor` instead of the taskbar island's.
fn make_effect_brush(
    compositor: &Compositor,
    blur_amount: f32,
    tint: [f32; 4],
) -> windows_core::Result<CompositionBrush> {
    let backdrop = compositor.CreateBackdropBrush()?;
    let backdrop_brush: CompositionBrush = windows_core::Interface::cast(&backdrop)?;

    let source_param =
        CompositionEffectSourceParameter::Create(&windows_core::HSTRING::from("backdrop"))?;
    let source_param_src: IGraphicsEffectSource = windows_core::Interface::cast(&source_param)?;

    let blur: IGraphicsEffect = GaussianBlurEffect {
        name: "GaussianBlurEffect".into(),
        source: source_param_src,
        blur_amount,
    }
    .into();
    let flood: IGraphicsEffect = FloodEffect {
        name: "FloodEffect".into(),
        color: tint,
    }
    .into();
    let blur_src: IGraphicsEffectSource = windows_core::Interface::cast(&blur)?;
    let flood_src: IGraphicsEffectSource = windows_core::Interface::cast(&flood)?;
    let composite: IGraphicsEffect = CompositeEffect {
        name: "CompositeEffect".into(),
        sources: vec![blur_src, flood_src],
        mode: D2D1_COMPOSITE_MODE_SOURCE_OVER,
    }
    .into();

    // `bindings::IGraphicsEffect` (the call-site shell) and
    // `fx_bindings::IGraphicsEffect` (the implementable def) share an IID —
    // a QI cast moves between them.
    let composite_shell: crate::bindings::IGraphicsEffect =
        windows_core::Interface::cast(&composite)?;
    let factory = compositor.CreateEffectFactory(&composite_shell)?;
    let effect_brush = factory.CreateBrush()?;
    effect_brush.SetSourceParameter(&windows_core::HSTRING::from("backdrop"), &backdrop_brush)?;
    windows_core::Interface::cast(&effect_brush)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Same prototype as the vendored windows-composition bindings — kept local
    // so the repro test needs no extra crate.
    windows_core::link!("coremessaging.dll" "system" fn CreateDispatcherQueueController(options: *const core::ffi::c_void, controller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);

    /// Drives the exact CreateEffectFactory path with a local Compositor to
    /// isolate whether 0x80070057 comes from our effect objects or from the
    /// taskbar island's compositor.
    #[test]
    fn effect_factory_repro() {
        unsafe {
            let _ = crate::win32::RoInitialize(0);
        }
        // DispatcherQueueOptions { dwSize, threadType=CURRENT(2), apartmentType=ASTA(2) }
        let options = [12u32, 2u32, 2u32];
        let mut controller = core::ptr::null_mut();
        let hr = unsafe {
            CreateDispatcherQueueController(options.as_ptr() as *const _, &mut controller)
        };
        assert!(hr.is_ok(), "dispatcher queue hr={hr:?}");
        let compositor = Compositor::new().expect("compositor");

        // Bisect: leaf flood first, then blur alone, then the full graph.
        let flood: IGraphicsEffect = FloodEffect {
            name: "FloodEffect".into(),
            color: [0.0, 0.0, 0.0, 0.0],
        }
        .into();
        let flood_shell: crate::bindings::IGraphicsEffect =
            windows_core::Interface::cast(&flood).unwrap();
        match compositor.CreateEffectFactory(&flood_shell) {
            Ok(_) => eprintln!("flood alone: OK"),
            Err(e) => eprintln!("flood alone: {e:?}"),
        }

        let source_param =
            CompositionEffectSourceParameter::Create(&windows_core::HSTRING::from("backdrop"))
                .unwrap();
        let source_param_src: IGraphicsEffectSource =
            windows_core::Interface::cast(&source_param).unwrap();
        let blur: IGraphicsEffect = GaussianBlurEffect {
            name: "GaussianBlurEffect".into(),
            source: source_param_src,
            blur_amount: 3.0,
        }
        .into();
        let blur_shell: crate::bindings::IGraphicsEffect =
            windows_core::Interface::cast(&blur).unwrap();
        match compositor.CreateEffectFactory(&blur_shell) {
            Ok(_) => eprintln!("blur alone: OK"),
            Err(e) => eprintln!("blur alone: {e:?}"),
        }

        match make_effect_brush(&compositor, 3.0, [0.0, 0.0, 0.0, 0.0]) {
            Ok(_) => eprintln!("effect_factory_repro: OK"),
            Err(e) => panic!("effect_factory_repro failed: {e:?}"),
        }
    }
}
