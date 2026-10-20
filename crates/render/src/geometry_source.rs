//! Documented IGeometrySource2DInterop bridge for a Direct2D clip path.
//! windows-bindgen 0.100 does not include windows.graphics.interop.h.
#![allow(non_snake_case, non_camel_case_types)]
use crate::comp;
use windows_core::{Interface, Result};

windows_core::imp::define_interface!(
    IGeometrySource2DInterop,
    IGeometrySource2DInterop_Vtbl,
    0x0657af73_53fd_47cf_84ff_c8492d2a80a3
);
windows_core::imp::interface_hierarchy!(IGeometrySource2DInterop, windows_core::IUnknown);
impl windows_core::RuntimeName for IGeometrySource2DInterop {}

pub trait IGeometrySource2DInterop_Impl: windows_core::IUnknownImpl {
    fn GetGeometry(&self) -> Result<comp::ID2D1Geometry>;
}
#[repr(C)]
pub struct IGeometrySource2DInterop_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    GetGeometry: unsafe extern "system" fn(
        *mut std::ffi::c_void,
        *mut *mut std::ffi::c_void,
    ) -> windows_core::HRESULT,
    TryGetGeometryUsingFactory: unsafe extern "system" fn(
        *mut std::ffi::c_void,
        *mut std::ffi::c_void,
        *mut *mut std::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl IGeometrySource2DInterop_Vtbl {
    pub const fn new<Identity: IGeometrySource2DInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get<
            Identity: IGeometrySource2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut std::ffi::c_void,
            value: *mut *mut std::ffi::c_void,
        ) -> windows_core::HRESULT {
            if value.is_null() {
                return windows_core::HRESULT(0x80004003u32 as i32);
            }
            unsafe {
                let this = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match this.GetGeometry() {
                    Ok(geometry) => {
                        value.write(geometry.into_raw());
                        windows_core::HRESULT(0)
                    }
                    Err(error) => {
                        value.write(std::ptr::null_mut());
                        error.into()
                    }
                }
            }
        }
        unsafe extern "system" fn try_get<
            Identity: IGeometrySource2DInterop_Impl,
            const OFFSET: isize,
        >(
            this: *mut std::ffi::c_void,
            _factory: *mut std::ffi::c_void,
            value: *mut *mut std::ffi::c_void,
        ) -> windows_core::HRESULT {
            // Returning null is the documented "not from this factory" response.
            // Composition can then call GetGeometry on our original factory's path.
            let _ = this;
            if value.is_null() {
                return windows_core::HRESULT(0x80004003u32 as i32);
            }
            unsafe {
                value.write(std::ptr::null_mut());
            }
            windows_core::HRESULT(0)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeometry: get::<Identity, OFFSET>,
            TryGetGeometryUsingFactory: try_get::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &IGeometrySource2DInterop::IID
    }
}

#[windows_core::implement(comp::IGeometrySource2D, IGeometrySource2DInterop)]
struct GeometrySource {
    path: windows_canvas::Path,
}
impl comp::IGeometrySource2D_Impl for GeometrySource_Impl {}
impl IGeometrySource2DInterop_Impl for GeometrySource_Impl {
    fn GetGeometry(&self) -> Result<comp::ID2D1Geometry> {
        self.path.raw().cast()
    }
}
pub(crate) fn composition_path(path: windows_canvas::Path) -> Result<comp::CompositionPath> {
    let source: comp::IGeometrySource2D = GeometrySource { path }.into();
    comp::CompositionPath::Create(&source)
}
