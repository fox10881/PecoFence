#[inline]
pub unsafe fn BitBlt(
    hdc: HDC,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    hdcsrc: Option<HDC>,
    x1: i32,
    y1: i32,
    rop: u32,
) -> windows_core::BOOL {
    windows_core::link!("gdi32.dll" "system" fn BitBlt(hdc : HDC, x : i32, y : i32, cx : i32, cy : i32, hdcsrc : HDC, x1 : i32, y1 : i32, rop : u32) -> windows_core::BOOL);
    unsafe {
        BitBlt(
            hdc,
            x,
            y,
            cx,
            cy,
            hdcsrc.unwrap_or(core::mem::zeroed()) as _,
            x1,
            y1,
            rop,
        )
    }
}
#[inline]
pub unsafe fn GetParent(hwnd: HWND) -> HWND {
    windows_core::link!("user32.dll" "system" fn GetParent(hwnd : HWND) -> HWND);
    unsafe { GetParent(hwnd) }
}
#[inline]
pub unsafe fn MFCreateAttributes(
    ppmfattributes: *mut Option<IMFAttributes>,
    cinitialsize: u32,
) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCreateAttributes(ppmfattributes : *mut *mut core::ffi::c_void, cinitialsize : u32) -> windows_core::HRESULT);
    unsafe { MFCreateAttributes(core::mem::transmute(ppmfattributes), cinitialsize) }
}
#[inline]
pub unsafe fn MFCreateMediaType() -> windows_core::Result<IMFMediaType> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaType(ppmftype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaType(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSourceReaderFromURL<P0, P1>(
    pwszurl: P0,
    pattributes: P1,
) -> windows_core::Result<IMFSourceReader>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IMFAttributes>,
{
    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromURL(pwszurl : windows_core::PCWSTR, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceReaderFromURL(
            pwszurl.param().abi(),
            pattributes.param().abi(),
            &mut result__,
        )
        .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFShutdown() -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFShutdown() -> windows_core::HRESULT);
    unsafe { MFShutdown() }
}
#[inline]
pub unsafe fn MFStartup(version: u32, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFStartup(version : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { MFStartup(version, dwflags) }
}
#[inline]
pub unsafe fn SetParent(hwndchild: HWND, hwndnewparent: Option<HWND>) -> HWND {
    windows_core::link!("user32.dll" "system" fn SetParent(hwndchild : HWND, hwndnewparent : HWND) -> HWND);
    unsafe { SetParent(hwndchild, hwndnewparent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetStretchBltMode(hdc: HDC, mode: i32) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn SetStretchBltMode(hdc : HDC, mode : i32) -> i32);
    unsafe { SetStretchBltMode(hdc, mode) }
}
#[inline]
pub unsafe fn StretchDIBits(
    hdc: HDC,
    xdest: i32,
    ydest: i32,
    destwidth: i32,
    destheight: i32,
    xsrc: i32,
    ysrc: i32,
    srcwidth: i32,
    srcheight: i32,
    lpbits: Option<*const core::ffi::c_void>,
    lpbmi: *const BITMAPINFO,
    iusage: u32,
    rop: u32,
) -> i32 {
    windows_core::link!("gdi32.dll" "system" fn StretchDIBits(hdc : HDC, xdest : i32, ydest : i32, destwidth : i32, destheight : i32, xsrc : i32, ysrc : i32, srcwidth : i32, srcheight : i32, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, iusage : u32, rop : u32) -> i32);
    unsafe {
        StretchDIBits(
            hdc,
            xdest,
            ydest,
            destwidth,
            destheight,
            xsrc,
            ysrc,
            srcwidth,
            srcheight,
            lpbits.unwrap_or(core::mem::zeroed()) as _,
            lpbmi,
            iusage,
            rop,
        )
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
impl Default for BITMAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut VARIANT_BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::BSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: *mut i8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut CY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut SCODE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
pub const COLORONCOLOR: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
pub const GUID_ContainerFormatJpeg: windows_core::GUID =
    windows_core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
pub const GUID_WICPixelFormat24bppBGR: windows_core::GUID =
    windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
pub const GUID_WICPixelFormat32bppBGR: windows_core::GUID =
    windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
pub const HALFTONE: i32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDC(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWND(pub *mut core::ffi::c_void);
windows_core::imp::define_interface!(
    IDispatch,
    IDispatch_Vtbl,
    0x00020400_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetTypeInfoCount: usize,
    GetTypeInfo: usize,
    GetIDsOfNames: usize,
    Invoke: usize,
}
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(
    IMF2DBuffer,
    IMF2DBuffer_Vtbl,
    0x7dc9d5f9_9ed9_44ec_9bbf_0600bb589fbb
);
windows_core::imp::interface_hierarchy!(IMF2DBuffer, windows_core::IUnknown);
impl IMF2DBuffer {
    pub(crate) unsafe fn Lock2D(
        &self,
        ppbscanline0: *mut *mut u8,
        plpitch: *mut i32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Lock2D)(
                windows_core::Interface::as_raw(self),
                ppbscanline0 as _,
                plpitch as _,
            )
        }
    }
    pub(crate) unsafe fn Unlock2D(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Unlock2D)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetScanline0AndPitch(
        &self,
        pbscanline0: *mut *mut u8,
        plpitch: *mut i32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetScanline0AndPitch)(
                windows_core::Interface::as_raw(self),
                pbscanline0 as _,
                plpitch as _,
            )
        }
    }
    pub(crate) unsafe fn IsContiguousFormat(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsContiguousFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetContiguousLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContiguousLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn ContiguousCopyTo(
        &self,
        pbdestbuffer: *mut u8,
        cbdestbuffer: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ContiguousCopyTo)(
                windows_core::Interface::as_raw(self),
                pbdestbuffer as _,
                cbdestbuffer,
            )
        }
    }
    pub(crate) unsafe fn ContiguousCopyFrom(&self, pbsrcbuffer: &[u8]) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ContiguousCopyFrom)(
                windows_core::Interface::as_raw(self),
                pbsrcbuffer.as_ptr(),
                pbsrcbuffer.len().try_into().unwrap(),
            )
        }
    }
}
#[repr(C)]
pub struct IMF2DBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lock2D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut u8,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub Unlock2D: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScanline0AndPitch: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut u8,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub IsContiguousFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetContiguousLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ContiguousCopyTo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub ContiguousCopyFrom:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IMF2DBuffer_Impl: windows_core::IUnknownImpl {
    fn Lock2D(&self, ppbscanline0: *mut *mut u8, plpitch: *mut i32) -> windows_core::Result<()>;
    fn Unlock2D(&self) -> windows_core::Result<()>;
    fn GetScanline0AndPitch(
        &self,
        pbscanline0: *mut *mut u8,
        plpitch: *mut i32,
    ) -> windows_core::Result<()>;
    fn IsContiguousFormat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetContiguousLength(&self) -> windows_core::Result<u32>;
    fn ContiguousCopyTo(
        &self,
        pbdestbuffer: *mut u8,
        cbdestbuffer: u32,
    ) -> windows_core::Result<()>;
    fn ContiguousCopyFrom(
        &self,
        pbsrcbuffer: *const u8,
        cbsrcbuffer: u32,
    ) -> windows_core::Result<()>;
}
impl IMF2DBuffer_Vtbl {
    pub const fn new<Identity: IMF2DBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock2D<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppbscanline0: *mut *mut u8,
            plpitch: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::Lock2D(
                    this,
                    core::mem::transmute_copy(&ppbscanline0),
                    core::mem::transmute_copy(&plpitch),
                )
                .into()
            }
        }
        unsafe extern "system" fn Unlock2D<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::Unlock2D(this).into()
            }
        }
        unsafe extern "system" fn GetScanline0AndPitch<
            Identity: IMF2DBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbscanline0: *mut *mut u8,
            plpitch: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::GetScanline0AndPitch(
                    this,
                    core::mem::transmute_copy(&pbscanline0),
                    core::mem::transmute_copy(&plpitch),
                )
                .into()
            }
        }
        unsafe extern "system" fn IsContiguousFormat<
            Identity: IMF2DBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pfiscontiguous: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMF2DBuffer_Impl::IsContiguousFormat(this) {
                    Ok(ok__) => {
                        pfiscontiguous.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContiguousLength<
            Identity: IMF2DBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcblength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMF2DBuffer_Impl::GetContiguousLength(this) {
                    Ok(ok__) => {
                        pcblength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContiguousCopyTo<
            Identity: IMF2DBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbdestbuffer: *mut u8,
            cbdestbuffer: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::ContiguousCopyTo(
                    this,
                    core::mem::transmute_copy(&pbdestbuffer),
                    core::mem::transmute_copy(&cbdestbuffer),
                )
                .into()
            }
        }
        unsafe extern "system" fn ContiguousCopyFrom<
            Identity: IMF2DBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbsrcbuffer: *const u8,
            cbsrcbuffer: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::ContiguousCopyFrom(
                    this,
                    core::mem::transmute_copy(&pbsrcbuffer),
                    core::mem::transmute_copy(&cbsrcbuffer),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Lock2D: Lock2D::<Identity, OFFSET>,
            Unlock2D: Unlock2D::<Identity, OFFSET>,
            GetScanline0AndPitch: GetScanline0AndPitch::<Identity, OFFSET>,
            IsContiguousFormat: IsContiguousFormat::<Identity, OFFSET>,
            GetContiguousLength: GetContiguousLength::<Identity, OFFSET>,
            ContiguousCopyTo: ContiguousCopyTo::<Identity, OFFSET>,
            ContiguousCopyFrom: ContiguousCopyFrom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMF2DBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMF2DBuffer {}
windows_core::imp::define_interface!(
    IMFAttributes,
    IMFAttributes_Vtbl,
    0x2cd2d921_c447_44a7_a13c_4adabfc247e3
);
windows_core::imp::interface_hierarchy!(IMFAttributes, windows_core::IUnknown);
impl IMFAttributes {
    pub(crate) unsafe fn GetItem(
        &self,
        guidkey: *const windows_core::GUID,
        pvalue: *mut PROPVARIANT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pvalue,
            )
        }
    }
    pub(crate) unsafe fn GetItemType(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<MF_ATTRIBUTE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn CompareItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn Compare<P0>(
        &self,
        ptheirs: P0,
        matchtype: MF_ATTRIBUTES_MATCH_TYPE,
    ) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(
                windows_core::Interface::as_raw(self),
                ptheirs.param().abi(),
                matchtype,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetUINT32(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUINT32)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetUINT64(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUINT64)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetDouble(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDouble)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetGUID(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetStringLength(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringLength)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetString(
        &self,
        guidkey: *const windows_core::GUID,
        pwszvalue: windows_core::PWSTR,
        cchbufsize: u32,
        pcchlength: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetString)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pwszvalue,
                cchbufsize,
                pcchlength as _,
            )
        }
    }
    pub(crate) unsafe fn GetAllocatedString(
        &self,
        guidkey: *const windows_core::GUID,
        ppwszvalue: *mut windows_core::PWSTR,
        pcchlength: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetAllocatedString)(
                windows_core::Interface::as_raw(self),
                guidkey,
                ppwszvalue as _,
                pcchlength as _,
            )
        }
    }
    pub(crate) unsafe fn GetBlobSize(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobSize)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *mut u8,
        cbbufsize: u32,
        pcbblobsize: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetBlob)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pbuf as _,
                cbbufsize,
                pcbblobsize as _,
            )
        }
    }
    pub(crate) unsafe fn GetAllocatedBlob(
        &self,
        guidkey: *const windows_core::GUID,
        ppbuf: *mut *mut u8,
        pcbsize: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetAllocatedBlob)(
                windows_core::Interface::as_raw(self),
                guidkey,
                ppbuf as _,
                pcbsize as _,
            )
        }
    }
    pub(crate) unsafe fn GetUnknown<T>(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetUnknown)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
                value,
            )
        }
    }
    pub(crate) unsafe fn DeleteItem(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
            )
        }
    }
    pub(crate) unsafe fn DeleteAllItems(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteAllItems)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub(crate) unsafe fn SetUINT32(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetUINT32)(
                windows_core::Interface::as_raw(self),
                guidkey,
                unvalue,
            )
        }
    }
    pub(crate) unsafe fn SetUINT64(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u64,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetUINT64)(
                windows_core::Interface::as_raw(self),
                guidkey,
                unvalue,
            )
        }
    }
    pub(crate) unsafe fn SetDouble(
        &self,
        guidkey: *const windows_core::GUID,
        fvalue: f64,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetDouble)(
                windows_core::Interface::as_raw(self),
                guidkey,
                fvalue,
            )
        }
    }
    pub(crate) unsafe fn SetGUID(
        &self,
        guidkey: *const windows_core::GUID,
        guidvalue: *const windows_core::GUID,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetGUID)(
                windows_core::Interface::as_raw(self),
                guidkey,
                guidvalue,
            )
        }
    }
    pub(crate) unsafe fn SetString<P1>(
        &self,
        guidkey: *const windows_core::GUID,
        wszvalue: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetString)(
                windows_core::Interface::as_raw(self),
                guidkey,
                wszvalue.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *const u8,
        cbbufsize: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetBlob)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pbuf,
                cbbufsize,
            )
        }
    }
    pub(crate) unsafe fn SetUnknown<P1>(
        &self,
        guidkey: *const windows_core::GUID,
        punknown: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnknown)(
                windows_core::Interface::as_raw(self),
                guidkey,
                punknown.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn LockStore(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).LockStore)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn UnlockStore(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).UnlockStore)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub(crate) unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetItemByIndex(
        &self,
        unindex: u32,
        pguidkey: *mut windows_core::GUID,
        pvalue: *mut PROPVARIANT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetItemByIndex)(
                windows_core::Interface::as_raw(self),
                unindex,
                pguidkey as _,
                pvalue,
            )
        }
    }
    pub(crate) unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyAllItems)(
                windows_core::Interface::as_raw(self),
                pdest.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IMFAttributes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut MF_ATTRIBUTE_TYPE,
    ) -> windows_core::HRESULT,
    pub CompareItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const PROPVARIANT,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        MF_ATTRIBUTES_MATCH_TYPE,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetUINT32: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetUINT64: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut f64,
    ) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        windows_core::PWSTR,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetAllocatedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut windows_core::PWSTR,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetBlobSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetBlob: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u8,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetAllocatedBlob: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut u8,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetUnknown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub DeleteAllItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUINT32: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
    ) -> windows_core::HRESULT,
    pub SetUINT64: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u64,
    ) -> windows_core::HRESULT,
    pub SetDouble: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        f64,
    ) -> windows_core::HRESULT,
    pub SetGUID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetBlob: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const u8,
        u32,
    ) -> windows_core::HRESULT,
    pub SetUnknown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemByIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub CopyAllItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IMFAttributes_Impl: windows_core::IUnknownImpl {
    fn GetItem(
        &self,
        guidkey: *const windows_core::GUID,
        pvalue: *mut PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn GetItemType(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<MF_ATTRIBUTE_TYPE>;
    fn CompareItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn Compare(
        &self,
        ptheirs: windows_core::Ref<IMFAttributes>,
        matchtype: MF_ATTRIBUTES_MATCH_TYPE,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn GetUINT32(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetUINT64(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u64>;
    fn GetDouble(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<f64>;
    fn GetGUID(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<windows_core::GUID>;
    fn GetStringLength(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetString(
        &self,
        guidkey: *const windows_core::GUID,
        pwszvalue: windows_core::PWSTR,
        cchbufsize: u32,
        pcchlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetAllocatedString(
        &self,
        guidkey: *const windows_core::GUID,
        ppwszvalue: *mut windows_core::PWSTR,
        pcchlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetBlobSize(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *mut u8,
        cbbufsize: u32,
        pcbblobsize: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetAllocatedBlob(
        &self,
        guidkey: *const windows_core::GUID,
        ppbuf: *mut *mut u8,
        pcbsize: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetUnknown(
        &self,
        guidkey: *const windows_core::GUID,
        riid: *const windows_core::GUID,
        ppv: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn DeleteItem(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteAllItems(&self) -> windows_core::Result<()>;
    fn SetUINT32(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u32,
    ) -> windows_core::Result<()>;
    fn SetUINT64(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u64,
    ) -> windows_core::Result<()>;
    fn SetDouble(
        &self,
        guidkey: *const windows_core::GUID,
        fvalue: f64,
    ) -> windows_core::Result<()>;
    fn SetGUID(
        &self,
        guidkey: *const windows_core::GUID,
        guidvalue: *const windows_core::GUID,
    ) -> windows_core::Result<()>;
    fn SetString(
        &self,
        guidkey: *const windows_core::GUID,
        wszvalue: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *const u8,
        cbbufsize: u32,
    ) -> windows_core::Result<()>;
    fn SetUnknown(
        &self,
        guidkey: *const windows_core::GUID,
        punknown: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn LockStore(&self) -> windows_core::Result<()>;
    fn UnlockStore(&self) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetItemByIndex(
        &self,
        unindex: u32,
        pguidkey: *mut windows_core::GUID,
        pvalue: *mut PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn CopyAllItems(&self, pdest: windows_core::Ref<IMFAttributes>) -> windows_core::Result<()>;
}
impl IMFAttributes_Vtbl {
    pub const fn new<Identity: IMFAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pvalue: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItem(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetItemType<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            ptype: *mut MF_ATTRIBUTE_TYPE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetItemType(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            value: *const PROPVARIANT,
            pbresult: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::CompareItem(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&value),
                ) {
                    Ok(ok__) => {
                        pbresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ptheirs: *mut core::ffi::c_void,
            matchtype: MF_ATTRIBUTES_MATCH_TYPE,
            pbresult: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::Compare(
                    this,
                    core::mem::transmute_copy(&ptheirs),
                    core::mem::transmute_copy(&matchtype),
                ) {
                    Ok(ok__) => {
                        pbresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUINT32<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            punvalue: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetUINT32(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        punvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUINT64<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            punvalue: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetUINT64(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        punvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDouble<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pfvalue: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetDouble(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        pfvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGUID<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pguidvalue: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetGUID(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        pguidvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringLength<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pcchlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetStringLength(this, core::mem::transmute_copy(&guidkey))
                {
                    Ok(ok__) => {
                        pcchlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pwszvalue: windows_core::PWSTR,
            cchbufsize: u32,
            pcchlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetString(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pwszvalue),
                    core::mem::transmute_copy(&cchbufsize),
                    core::mem::transmute_copy(&pcchlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAllocatedString<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            ppwszvalue: *mut windows_core::PWSTR,
            pcchlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetAllocatedString(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&ppwszvalue),
                    core::mem::transmute_copy(&pcchlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetBlobSize<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pcbblobsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetBlobSize(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        pcbblobsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pbuf: *mut u8,
            cbbufsize: u32,
            pcbblobsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetBlob(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pbuf),
                    core::mem::transmute_copy(&cbbufsize),
                    core::mem::transmute_copy(&pcbblobsize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAllocatedBlob<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            ppbuf: *mut *mut u8,
            pcbsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetAllocatedBlob(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&ppbuf),
                    core::mem::transmute_copy(&pcbsize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetUnknown<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppv: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetUnknown(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppv),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            value: *const PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetItem(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::DeleteItem(this, core::mem::transmute_copy(&guidkey)).into()
            }
        }
        unsafe extern "system" fn DeleteAllItems<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::DeleteAllItems(this).into()
            }
        }
        unsafe extern "system" fn SetUINT32<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            unvalue: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUINT32(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&unvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetUINT64<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            unvalue: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUINT64(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&unvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetDouble<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            fvalue: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetDouble(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&fvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetGUID<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            guidvalue: *const windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetGUID(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&guidvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetString<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            wszvalue: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetString(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute(&wszvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pbuf: *const u8,
            cbbufsize: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetBlob(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pbuf),
                    core::mem::transmute_copy(&cbbufsize),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetUnknown<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            punknown: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUnknown(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&punknown),
                )
                .into()
            }
        }
        unsafe extern "system" fn LockStore<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::LockStore(this).into()
            }
        }
        unsafe extern "system" fn UnlockStore<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::UnlockStore(this).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcitems: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcitems.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemByIndex<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            unindex: u32,
            pguidkey: *mut windows_core::GUID,
            pvalue: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItemByIndex(
                    this,
                    core::mem::transmute_copy(&unindex),
                    core::mem::transmute_copy(&pguidkey),
                    core::mem::transmute_copy(&pvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn CopyAllItems<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdest: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::CopyAllItems(this, core::mem::transmute_copy(&pdest)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, OFFSET>,
            GetItemType: GetItemType::<Identity, OFFSET>,
            CompareItem: CompareItem::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            GetUINT32: GetUINT32::<Identity, OFFSET>,
            GetUINT64: GetUINT64::<Identity, OFFSET>,
            GetDouble: GetDouble::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetAllocatedString: GetAllocatedString::<Identity, OFFSET>,
            GetBlobSize: GetBlobSize::<Identity, OFFSET>,
            GetBlob: GetBlob::<Identity, OFFSET>,
            GetAllocatedBlob: GetAllocatedBlob::<Identity, OFFSET>,
            GetUnknown: GetUnknown::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            DeleteAllItems: DeleteAllItems::<Identity, OFFSET>,
            SetUINT32: SetUINT32::<Identity, OFFSET>,
            SetUINT64: SetUINT64::<Identity, OFFSET>,
            SetDouble: SetDouble::<Identity, OFFSET>,
            SetGUID: SetGUID::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            SetBlob: SetBlob::<Identity, OFFSET>,
            SetUnknown: SetUnknown::<Identity, OFFSET>,
            LockStore: LockStore::<Identity, OFFSET>,
            UnlockStore: UnlockStore::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetItemByIndex: GetItemByIndex::<Identity, OFFSET>,
            CopyAllItems: CopyAllItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFAttributes {}
windows_core::imp::define_interface!(
    IMFCollection,
    IMFCollection_Vtbl,
    0x5bc8a76b_869a_46a3_9b03_fa218a66aebe
);
windows_core::imp::interface_hierarchy!(IMFCollection, windows_core::IUnknown);
#[repr(C)]
pub struct IMFCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetElementCount: usize,
    GetElement: usize,
    AddElement: usize,
    RemoveElement: usize,
    InsertElementAt: usize,
    RemoveAllElements: usize,
}
impl windows_core::RuntimeName for IMFCollection {}
windows_core::imp::define_interface!(
    IMFMediaBuffer,
    IMFMediaBuffer_Vtbl,
    0x045fa593_8799_42b8_bc8d_8968c6453507
);
windows_core::imp::interface_hierarchy!(IMFMediaBuffer, windows_core::IUnknown);
impl IMFMediaBuffer {
    pub(crate) unsafe fn Lock(
        &self,
        ppbbuffer: *mut *mut u8,
        pcbmaxlength: Option<*mut u32>,
        pcbcurrentlength: Option<*mut u32>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Lock)(
                windows_core::Interface::as_raw(self),
                ppbbuffer as _,
                pcbmaxlength.unwrap_or(core::mem::zeroed()) as _,
                pcbcurrentlength.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetCurrentLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetCurrentLength(&self, cbcurrentlength: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetCurrentLength)(
                windows_core::Interface::as_raw(self),
                cbcurrentlength,
            )
        }
    }
    pub(crate) unsafe fn GetMaxLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IMFMediaBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lock: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut u8,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCurrentLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMFMediaBuffer_Impl: windows_core::IUnknownImpl {
    fn Lock(
        &self,
        ppbbuffer: *mut *mut u8,
        pcbmaxlength: *mut u32,
        pcbcurrentlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetCurrentLength(&self) -> windows_core::Result<u32>;
    fn SetCurrentLength(&self, cbcurrentlength: u32) -> windows_core::Result<()>;
    fn GetMaxLength(&self) -> windows_core::Result<u32>;
}
impl IMFMediaBuffer_Vtbl {
    pub const fn new<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppbbuffer: *mut *mut u8,
            pcbmaxlength: *mut u32,
            pcbcurrentlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::Lock(
                    this,
                    core::mem::transmute_copy(&ppbbuffer),
                    core::mem::transmute_copy(&pcbmaxlength),
                    core::mem::transmute_copy(&pcbcurrentlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetCurrentLength<
            Identity: IMFMediaBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcbcurrentlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaBuffer_Impl::GetCurrentLength(this) {
                    Ok(ok__) => {
                        pcbcurrentlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentLength<
            Identity: IMFMediaBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cbcurrentlength: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::SetCurrentLength(
                    this,
                    core::mem::transmute_copy(&cbcurrentlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetMaxLength<
            Identity: IMFMediaBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcbmaxlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaBuffer_Impl::GetMaxLength(this) {
                    Ok(ok__) => {
                        pcbmaxlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetCurrentLength: GetCurrentLength::<Identity, OFFSET>,
            SetCurrentLength: SetCurrentLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaBuffer {}
windows_core::imp::define_interface!(
    IMFMediaEvent,
    IMFMediaEvent_Vtbl,
    0xdf598932_f10c_4e39_bba2_c308f101daa3
);
impl core::ops::Deref for IMFMediaEvent {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEvent, windows_core::IUnknown, IMFAttributes);
#[repr(C)]
pub struct IMFMediaEvent_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    GetType: usize,
    GetExtendedType: usize,
    GetStatus: usize,
    GetValue: usize,
}
impl windows_core::RuntimeName for IMFMediaEvent {}
windows_core::imp::define_interface!(
    IMFMediaType,
    IMFMediaType_Vtbl,
    0x44ae0fa8_ea31_4109_8d2e_4cae4997c555
);
impl core::ops::Deref for IMFMediaType {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaType, windows_core::IUnknown, IMFAttributes);
impl IMFMediaType {
    pub(crate) unsafe fn GetMajorType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMajorType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsCompressedFormat(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCompressedFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn IsEqual<P0>(&self, pimediatype: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(
                windows_core::Interface::as_raw(self),
                pimediatype.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetRepresentation(
        &self,
        guidrepresentation: windows_core::GUID,
        ppvrepresentation: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetRepresentation)(
                windows_core::Interface::as_raw(self),
                guidrepresentation,
                ppvrepresentation as _,
            )
        }
    }
    pub(crate) unsafe fn FreeRepresentation(
        &self,
        guidrepresentation: windows_core::GUID,
        pvrepresentation: *const core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FreeRepresentation)(
                windows_core::Interface::as_raw(self),
                guidrepresentation,
                pvrepresentation,
            )
        }
    }
}
#[repr(C)]
pub struct IMFMediaType_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    pub GetMajorType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub IsCompressedFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetRepresentation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FreeRepresentation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::GUID,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IMFMediaType_Impl: IMFAttributes_Impl {
    fn GetMajorType(&self) -> windows_core::Result<windows_core::GUID>;
    fn IsCompressedFormat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsEqual(&self, pimediatype: windows_core::Ref<IMFMediaType>) -> windows_core::Result<u32>;
    fn GetRepresentation(
        &self,
        guidrepresentation: &windows_core::GUID,
        ppvrepresentation: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn FreeRepresentation(
        &self,
        guidrepresentation: &windows_core::GUID,
        pvrepresentation: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl IMFMediaType_Vtbl {
    pub const fn new<Identity: IMFMediaType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMajorType<Identity: IMFMediaType_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pguidmajortype: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaType_Impl::GetMajorType(this) {
                    Ok(ok__) => {
                        pguidmajortype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCompressedFormat<
            Identity: IMFMediaType_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pfcompressed: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaType_Impl::IsCompressedFormat(this) {
                    Ok(ok__) => {
                        pfcompressed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IMFMediaType_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pimediatype: *mut core::ffi::c_void,
            pdwflags: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaType_Impl::IsEqual(this, core::mem::transmute_copy(&pimediatype)) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRepresentation<
            Identity: IMFMediaType_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidrepresentation: windows_core::GUID,
            ppvrepresentation: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::GetRepresentation(
                    this,
                    core::mem::transmute(&guidrepresentation),
                    core::mem::transmute_copy(&ppvrepresentation),
                )
                .into()
            }
        }
        unsafe extern "system" fn FreeRepresentation<
            Identity: IMFMediaType_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidrepresentation: windows_core::GUID,
            pvrepresentation: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::FreeRepresentation(
                    this,
                    core::mem::transmute(&guidrepresentation),
                    core::mem::transmute_copy(&pvrepresentation),
                )
                .into()
            }
        }
        Self {
            base__: IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetMajorType: GetMajorType::<Identity, OFFSET>,
            IsCompressedFormat: IsCompressedFormat::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            GetRepresentation: GetRepresentation::<Identity, OFFSET>,
            FreeRepresentation: FreeRepresentation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaType as windows_core::Interface>::IID
            || iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaType {}
windows_core::imp::define_interface!(
    IMFSample,
    IMFSample_Vtbl,
    0xc40a00f2_b93a_4d80_ae8c_5a1c634f58e4
);
impl core::ops::Deref for IMFSample {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSample, windows_core::IUnknown, IMFAttributes);
impl IMFSample {
    pub(crate) unsafe fn GetSampleFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampleFlags)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetSampleFlags(&self, dwsampleflags: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetSampleFlags)(
                windows_core::Interface::as_raw(self),
                dwsampleflags,
            )
        }
    }
    pub(crate) unsafe fn GetSampleTime(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampleTime)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetSampleTime(&self, hnssampletime: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetSampleTime)(
                windows_core::Interface::as_raw(self),
                hnssampletime,
            )
        }
    }
    pub(crate) unsafe fn GetSampleDuration(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSampleDuration)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetSampleDuration(&self, hnssampleduration: i64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetSampleDuration)(
                windows_core::Interface::as_raw(self),
                hnssampleduration,
            )
        }
    }
    pub(crate) unsafe fn GetBufferCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetBufferByIndex(
        &self,
        dwindex: u32,
    ) -> windows_core::Result<IMFMediaBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferByIndex)(
                windows_core::Interface::as_raw(self),
                dwindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn ConvertToContiguousBuffer(&self) -> windows_core::Result<IMFMediaBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConvertToContiguousBuffer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn AddBuffer<P0>(&self, pbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaBuffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddBuffer)(
                windows_core::Interface::as_raw(self),
                pbuffer.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn RemoveBufferByIndex(&self, dwindex: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveBufferByIndex)(
                windows_core::Interface::as_raw(self),
                dwindex,
            )
        }
    }
    pub(crate) unsafe fn RemoveAllBuffers(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAllBuffers)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetTotalLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn CopyToBuffer<P0>(&self, pbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaBuffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyToBuffer)(
                windows_core::Interface::as_raw(self),
                pbuffer.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IMFSample_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    pub GetSampleFlags:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleFlags:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSampleTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetSampleTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetSampleDuration:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetSampleDuration:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetBufferCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBufferByIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ConvertToContiguousBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveBufferByIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTotalLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CopyToBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IMFSample_Impl: IMFAttributes_Impl {
    fn GetSampleFlags(&self) -> windows_core::Result<u32>;
    fn SetSampleFlags(&self, dwsampleflags: u32) -> windows_core::Result<()>;
    fn GetSampleTime(&self) -> windows_core::Result<i64>;
    fn SetSampleTime(&self, hnssampletime: i64) -> windows_core::Result<()>;
    fn GetSampleDuration(&self) -> windows_core::Result<i64>;
    fn SetSampleDuration(&self, hnssampleduration: i64) -> windows_core::Result<()>;
    fn GetBufferCount(&self) -> windows_core::Result<u32>;
    fn GetBufferByIndex(&self, dwindex: u32) -> windows_core::Result<IMFMediaBuffer>;
    fn ConvertToContiguousBuffer(&self) -> windows_core::Result<IMFMediaBuffer>;
    fn AddBuffer(&self, pbuffer: windows_core::Ref<IMFMediaBuffer>) -> windows_core::Result<()>;
    fn RemoveBufferByIndex(&self, dwindex: u32) -> windows_core::Result<()>;
    fn RemoveAllBuffers(&self) -> windows_core::Result<()>;
    fn GetTotalLength(&self) -> windows_core::Result<u32>;
    fn CopyToBuffer(&self, pbuffer: windows_core::Ref<IMFMediaBuffer>) -> windows_core::Result<()>;
}
impl IMFSample_Vtbl {
    pub const fn new<Identity: IMFSample_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSampleFlags<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdwsampleflags: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::GetSampleFlags(this) {
                    Ok(ok__) => {
                        pdwsampleflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleFlags<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwsampleflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::SetSampleFlags(this, core::mem::transmute_copy(&dwsampleflags))
                    .into()
            }
        }
        unsafe extern "system" fn GetSampleTime<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            phnssampletime: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::GetSampleTime(this) {
                    Ok(ok__) => {
                        phnssampletime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleTime<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            hnssampletime: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::SetSampleTime(this, core::mem::transmute_copy(&hnssampletime))
                    .into()
            }
        }
        unsafe extern "system" fn GetSampleDuration<
            Identity: IMFSample_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            phnssampleduration: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::GetSampleDuration(this) {
                    Ok(ok__) => {
                        phnssampleduration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleDuration<
            Identity: IMFSample_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hnssampleduration: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::SetSampleDuration(
                    this,
                    core::mem::transmute_copy(&hnssampleduration),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetBufferCount<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdwbuffercount: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::GetBufferCount(this) {
                    Ok(ok__) => {
                        pdwbuffercount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBufferByIndex<
            Identity: IMFSample_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwindex: u32,
            ppbuffer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::GetBufferByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConvertToContiguousBuffer<
            Identity: IMFSample_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppbuffer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::ConvertToContiguousBuffer(this) {
                    Ok(ok__) => {
                        ppbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddBuffer<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pbuffer: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::AddBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn RemoveBufferByIndex<
            Identity: IMFSample_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwindex: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::RemoveBufferByIndex(this, core::mem::transmute_copy(&dwindex))
                    .into()
            }
        }
        unsafe extern "system" fn RemoveAllBuffers<
            Identity: IMFSample_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::RemoveAllBuffers(this).into()
            }
        }
        unsafe extern "system" fn GetTotalLength<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcbtotallength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSample_Impl::GetTotalLength(this) {
                    Ok(ok__) => {
                        pcbtotallength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyToBuffer<Identity: IMFSample_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pbuffer: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::CopyToBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetSampleFlags: GetSampleFlags::<Identity, OFFSET>,
            SetSampleFlags: SetSampleFlags::<Identity, OFFSET>,
            GetSampleTime: GetSampleTime::<Identity, OFFSET>,
            SetSampleTime: SetSampleTime::<Identity, OFFSET>,
            GetSampleDuration: GetSampleDuration::<Identity, OFFSET>,
            SetSampleDuration: SetSampleDuration::<Identity, OFFSET>,
            GetBufferCount: GetBufferCount::<Identity, OFFSET>,
            GetBufferByIndex: GetBufferByIndex::<Identity, OFFSET>,
            ConvertToContiguousBuffer: ConvertToContiguousBuffer::<Identity, OFFSET>,
            AddBuffer: AddBuffer::<Identity, OFFSET>,
            RemoveBufferByIndex: RemoveBufferByIndex::<Identity, OFFSET>,
            RemoveAllBuffers: RemoveAllBuffers::<Identity, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, OFFSET>,
            CopyToBuffer: CopyToBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSample as windows_core::Interface>::IID
            || iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSample {}
windows_core::imp::define_interface!(
    IMFSourceReader,
    IMFSourceReader_Vtbl,
    0x70ae66f2_c809_4e4f_8915_bdcb406b7993
);
windows_core::imp::interface_hierarchy!(IMFSourceReader, windows_core::IUnknown);
impl IMFSourceReader {
    pub(crate) unsafe fn GetStreamSelection(
        &self,
        dwstreamindex: u32,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetStreamSelection(
        &self,
        dwstreamindex: u32,
        fselected: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetStreamSelection)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                fselected.into(),
            )
        }
    }
    pub(crate) unsafe fn GetNativeMediaType(
        &self,
        dwstreamindex: u32,
        dwmediatypeindex: u32,
    ) -> windows_core::Result<IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNativeMediaType)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                dwmediatypeindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetCurrentMediaType(
        &self,
        dwstreamindex: u32,
    ) -> windows_core::Result<IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMediaType)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetCurrentMediaType<P2>(
        &self,
        dwstreamindex: u32,
        pdwreserved: Option<*const u32>,
        pmediatype: P2,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IMFMediaType>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCurrentMediaType)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                pdwreserved.unwrap_or(core::mem::zeroed()) as _,
                pmediatype.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetCurrentPosition(
        &self,
        guidtimeformat: *const windows_core::GUID,
        varposition: *const PROPVARIANT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetCurrentPosition)(
                windows_core::Interface::as_raw(self),
                guidtimeformat,
                varposition,
            )
        }
    }
    pub(crate) unsafe fn ReadSample(
        &self,
        dwstreamindex: u32,
        dwcontrolflags: u32,
        pdwactualstreamindex: Option<*mut u32>,
        pdwstreamflags: Option<*mut u32>,
        plltimestamp: Option<*mut i64>,
        ppsample: Option<*mut Option<IMFSample>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ReadSample)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                dwcontrolflags,
                pdwactualstreamindex.unwrap_or(core::mem::zeroed()) as _,
                pdwstreamflags.unwrap_or(core::mem::zeroed()) as _,
                plltimestamp.unwrap_or(core::mem::zeroed()) as _,
                ppsample.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn Flush(&self, dwstreamindex: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
            )
        }
    }
    pub(crate) unsafe fn GetServiceForStream<T>(
        &self,
        dwstreamindex: u32,
        guidservice: *const windows_core::GUID,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetServiceForStream)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                guidservice,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetPresentationAttribute(
        &self,
        dwstreamindex: u32,
        guidattribute: *const windows_core::GUID,
    ) -> windows_core::Result<PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationAttribute)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                guidattribute,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
pub struct IMFSourceReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamSelection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetStreamSelection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetNativeMediaType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCurrentMediaType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCurrentMediaType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCurrentPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub ReadSample: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
        *mut u32,
        *mut i64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetServiceForStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPresentationAttribute: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
}
pub trait IMFSourceReader_Impl: windows_core::IUnknownImpl {
    fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamSelection(
        &self,
        dwstreamindex: u32,
        fselected: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn GetNativeMediaType(
        &self,
        dwstreamindex: u32,
        dwmediatypeindex: u32,
    ) -> windows_core::Result<IMFMediaType>;
    fn GetCurrentMediaType(&self, dwstreamindex: u32) -> windows_core::Result<IMFMediaType>;
    fn SetCurrentMediaType(
        &self,
        dwstreamindex: u32,
        pdwreserved: *const u32,
        pmediatype: windows_core::Ref<IMFMediaType>,
    ) -> windows_core::Result<()>;
    fn SetCurrentPosition(
        &self,
        guidtimeformat: *const windows_core::GUID,
        varposition: *const PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn ReadSample(
        &self,
        dwstreamindex: u32,
        dwcontrolflags: u32,
        pdwactualstreamindex: *mut u32,
        pdwstreamflags: *mut u32,
        plltimestamp: *mut i64,
        ppsample: windows_core::OutRef<IMFSample>,
    ) -> windows_core::Result<()>;
    fn Flush(&self, dwstreamindex: u32) -> windows_core::Result<()>;
    fn GetServiceForStream(
        &self,
        dwstreamindex: u32,
        guidservice: *const windows_core::GUID,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetPresentationAttribute(
        &self,
        dwstreamindex: u32,
        guidattribute: *const windows_core::GUID,
    ) -> windows_core::Result<PROPVARIANT>;
}
impl IMFSourceReader_Vtbl {
    pub const fn new<Identity: IMFSourceReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamSelection<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            pfselected: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetStreamSelection(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                ) {
                    Ok(ok__) => {
                        pfselected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamSelection<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            fselected: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetStreamSelection(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&fselected),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetNativeMediaType<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            dwmediatypeindex: u32,
            ppmediatype: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetNativeMediaType(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&dwmediatypeindex),
                ) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentMediaType<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            ppmediatype: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetCurrentMediaType(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                ) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentMediaType<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            pdwreserved: *const u32,
            pmediatype: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetCurrentMediaType(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&pdwreserved),
                    core::mem::transmute_copy(&pmediatype),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetCurrentPosition<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidtimeformat: *const windows_core::GUID,
            varposition: *const PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::SetCurrentPosition(
                    this,
                    core::mem::transmute_copy(&guidtimeformat),
                    core::mem::transmute_copy(&varposition),
                )
                .into()
            }
        }
        unsafe extern "system" fn ReadSample<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            dwcontrolflags: u32,
            pdwactualstreamindex: *mut u32,
            pdwstreamflags: *mut u32,
            plltimestamp: *mut i64,
            ppsample: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::ReadSample(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&dwcontrolflags),
                    core::mem::transmute_copy(&pdwactualstreamindex),
                    core::mem::transmute_copy(&pdwstreamflags),
                    core::mem::transmute_copy(&plltimestamp),
                    core::mem::transmute_copy(&ppsample),
                )
                .into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFSourceReader_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::Flush(this, core::mem::transmute_copy(&dwstreamindex)).into()
            }
        }
        unsafe extern "system" fn GetServiceForStream<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            guidservice: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppvobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceReader_Impl::GetServiceForStream(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&guidservice),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPresentationAttribute<
            Identity: IMFSourceReader_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            guidattribute: *const windows_core::GUID,
            pvarattribute: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceReader_Impl::GetPresentationAttribute(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&guidattribute),
                ) {
                    Ok(ok__) => {
                        pvarattribute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamSelection: GetStreamSelection::<Identity, OFFSET>,
            SetStreamSelection: SetStreamSelection::<Identity, OFFSET>,
            GetNativeMediaType: GetNativeMediaType::<Identity, OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Identity, OFFSET>,
            SetCurrentMediaType: SetCurrentMediaType::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            ReadSample: ReadSample::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            GetServiceForStream: GetServiceForStream::<Identity, OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSourceReader {}
windows_core::imp::define_interface!(
    IMFTransform,
    IMFTransform_Vtbl,
    0xbf94c121_5b05_4e6f_8000_ba598961414d
);
windows_core::imp::interface_hierarchy!(IMFTransform, windows_core::IUnknown);
impl IMFTransform {
    pub(crate) unsafe fn GetStreamLimits(
        &self,
        pdwinputminimum: *mut u32,
        pdwinputmaximum: *mut u32,
        pdwoutputminimum: *mut u32,
        pdwoutputmaximum: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetStreamLimits)(
                windows_core::Interface::as_raw(self),
                pdwinputminimum as _,
                pdwinputmaximum as _,
                pdwoutputminimum as _,
                pdwoutputmaximum as _,
            )
        }
    }
    pub(crate) unsafe fn GetStreamCount(
        &self,
        pcinputstreams: *mut u32,
        pcoutputstreams: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetStreamCount)(
                windows_core::Interface::as_raw(self),
                pcinputstreams as _,
                pcoutputstreams as _,
            )
        }
    }
    pub(crate) unsafe fn GetStreamIDs(
        &self,
        dwinputidarraysize: u32,
        pdwinputids: *mut u32,
        dwoutputidarraysize: u32,
        pdwoutputids: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetStreamIDs)(
                windows_core::Interface::as_raw(self),
                dwinputidarraysize,
                pdwinputids as _,
                dwoutputidarraysize,
                pdwoutputids as _,
            )
        }
    }
    pub(crate) unsafe fn GetInputStreamInfo(
        &self,
        dwinputstreamid: u32,
        pstreaminfo: *mut MFT_INPUT_STREAM_INFO,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetInputStreamInfo)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                pstreaminfo as _,
            )
        }
    }
    pub(crate) unsafe fn GetOutputStreamInfo(
        &self,
        dwoutputstreamid: u32,
    ) -> windows_core::Result<MFT_OUTPUT_STREAM_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamInfo)(
                windows_core::Interface::as_raw(self),
                dwoutputstreamid,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetAttributes(&self) -> windows_core::Result<IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetInputStreamAttributes(
        &self,
        dwinputstreamid: u32,
    ) -> windows_core::Result<IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAttributes)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetOutputStreamAttributes(
        &self,
        dwoutputstreamid: u32,
    ) -> windows_core::Result<IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAttributes)(
                windows_core::Interface::as_raw(self),
                dwoutputstreamid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn DeleteInputStream(&self, dwstreamid: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteInputStream)(
                windows_core::Interface::as_raw(self),
                dwstreamid,
            )
        }
    }
    pub(crate) unsafe fn AddInputStreams(
        &self,
        cstreams: u32,
        adwstreamids: *const u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).AddInputStreams)(
                windows_core::Interface::as_raw(self),
                cstreams,
                adwstreamids,
            )
        }
    }
    pub(crate) unsafe fn GetInputAvailableType(
        &self,
        dwinputstreamid: u32,
        dwtypeindex: u32,
    ) -> windows_core::Result<IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputAvailableType)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                dwtypeindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetOutputAvailableType(
        &self,
        dwoutputstreamid: u32,
        dwtypeindex: u32,
    ) -> windows_core::Result<IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputAvailableType)(
                windows_core::Interface::as_raw(self),
                dwoutputstreamid,
                dwtypeindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetInputType<P1>(
        &self,
        dwinputstreamid: u32,
        ptype: P1,
        dwflags: u32,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFMediaType>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetInputType)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                ptype.param().abi(),
                dwflags,
            )
        }
    }
    pub(crate) unsafe fn SetOutputType<P1>(
        &self,
        dwoutputstreamid: u32,
        ptype: P1,
        dwflags: u32,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFMediaType>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOutputType)(
                windows_core::Interface::as_raw(self),
                dwoutputstreamid,
                ptype.param().abi(),
                dwflags,
            )
        }
    }
    pub(crate) unsafe fn GetInputCurrentType(
        &self,
        dwinputstreamid: u32,
    ) -> windows_core::Result<IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCurrentType)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetOutputCurrentType(
        &self,
        dwoutputstreamid: u32,
    ) -> windows_core::Result<IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCurrentType)(
                windows_core::Interface::as_raw(self),
                dwoutputstreamid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetInputStatus(&self, dwinputstreamid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStatus)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetOutputStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStatus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetOutputBounds(
        &self,
        hnslowerbound: i64,
        hnsupperbound: i64,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetOutputBounds)(
                windows_core::Interface::as_raw(self),
                hnslowerbound,
                hnsupperbound,
            )
        }
    }
    pub(crate) unsafe fn ProcessEvent<P1>(
        &self,
        dwinputstreamid: u32,
        pevent: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFMediaEvent>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessEvent)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                pevent.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn ProcessMessage(
        &self,
        emessage: MFT_MESSAGE_TYPE,
        ulparam: usize,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessMessage)(
                windows_core::Interface::as_raw(self),
                emessage,
                ulparam,
            )
        }
    }
    pub(crate) unsafe fn ProcessInput<P1>(
        &self,
        dwinputstreamid: u32,
        psample: P1,
        dwflags: u32,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFSample>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessInput)(
                windows_core::Interface::as_raw(self),
                dwinputstreamid,
                psample.param().abi(),
                dwflags,
            )
        }
    }
    pub(crate) unsafe fn ProcessOutput(
        &self,
        dwflags: u32,
        coutputbuffercount: u32,
        poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER,
        pdwstatus: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessOutput)(
                windows_core::Interface::as_raw(self),
                dwflags,
                coutputbuffercount,
                poutputsamples,
                pdwstatus as _,
            )
        }
    }
}
#[repr(C)]
pub struct IMFTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStreamLimits: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetStreamIDs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut MFT_INPUT_STREAM_INFO,
    ) -> windows_core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut MFT_OUTPUT_STREAM_INFO,
    ) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetInputStreamAttributes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetOutputStreamAttributes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeleteInputStream:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AddInputStreams:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub GetInputAvailableType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetOutputAvailableType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetInputType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
    pub SetOutputType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
    pub GetInputCurrentType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetOutputCurrentType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetInputStatus:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetOutputStatus:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOutputBounds:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64, i64) -> windows_core::HRESULT,
    pub ProcessEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ProcessMessage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MFT_MESSAGE_TYPE,
        usize,
    ) -> windows_core::HRESULT,
    pub ProcessInput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
    pub ProcessOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut MFT_OUTPUT_DATA_BUFFER,
        *mut u32,
    ) -> windows_core::HRESULT,
}
pub trait IMFTransform_Impl: windows_core::IUnknownImpl {
    fn GetStreamLimits(
        &self,
        pdwinputminimum: *mut u32,
        pdwinputmaximum: *mut u32,
        pdwoutputminimum: *mut u32,
        pdwoutputmaximum: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetStreamCount(
        &self,
        pcinputstreams: *mut u32,
        pcoutputstreams: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetStreamIDs(
        &self,
        dwinputidarraysize: u32,
        pdwinputids: *mut u32,
        dwoutputidarraysize: u32,
        pdwoutputids: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetInputStreamInfo(
        &self,
        dwinputstreamid: u32,
        pstreaminfo: *mut MFT_INPUT_STREAM_INFO,
    ) -> windows_core::Result<()>;
    fn GetOutputStreamInfo(
        &self,
        dwoutputstreamid: u32,
    ) -> windows_core::Result<MFT_OUTPUT_STREAM_INFO>;
    fn GetAttributes(&self) -> windows_core::Result<IMFAttributes>;
    fn GetInputStreamAttributes(&self, dwinputstreamid: u32)
    -> windows_core::Result<IMFAttributes>;
    fn GetOutputStreamAttributes(
        &self,
        dwoutputstreamid: u32,
    ) -> windows_core::Result<IMFAttributes>;
    fn DeleteInputStream(&self, dwstreamid: u32) -> windows_core::Result<()>;
    fn AddInputStreams(&self, cstreams: u32, adwstreamids: *const u32) -> windows_core::Result<()>;
    fn GetInputAvailableType(
        &self,
        dwinputstreamid: u32,
        dwtypeindex: u32,
    ) -> windows_core::Result<IMFMediaType>;
    fn GetOutputAvailableType(
        &self,
        dwoutputstreamid: u32,
        dwtypeindex: u32,
    ) -> windows_core::Result<IMFMediaType>;
    fn SetInputType(
        &self,
        dwinputstreamid: u32,
        ptype: windows_core::Ref<IMFMediaType>,
        dwflags: u32,
    ) -> windows_core::Result<()>;
    fn SetOutputType(
        &self,
        dwoutputstreamid: u32,
        ptype: windows_core::Ref<IMFMediaType>,
        dwflags: u32,
    ) -> windows_core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamid: u32) -> windows_core::Result<IMFMediaType>;
    fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> windows_core::Result<IMFMediaType>;
    fn GetInputStatus(&self, dwinputstreamid: u32) -> windows_core::Result<u32>;
    fn GetOutputStatus(&self) -> windows_core::Result<u32>;
    fn SetOutputBounds(&self, hnslowerbound: i64, hnsupperbound: i64) -> windows_core::Result<()>;
    fn ProcessEvent(
        &self,
        dwinputstreamid: u32,
        pevent: windows_core::Ref<IMFMediaEvent>,
    ) -> windows_core::Result<()>;
    fn ProcessMessage(
        &self,
        emessage: MFT_MESSAGE_TYPE,
        ulparam: usize,
    ) -> windows_core::Result<()>;
    fn ProcessInput(
        &self,
        dwinputstreamid: u32,
        psample: windows_core::Ref<IMFSample>,
        dwflags: u32,
    ) -> windows_core::Result<()>;
    fn ProcessOutput(
        &self,
        dwflags: u32,
        coutputbuffercount: u32,
        poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER,
        pdwstatus: *mut u32,
    ) -> windows_core::Result<()>;
}
impl IMFTransform_Vtbl {
    pub const fn new<Identity: IMFTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamLimits<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdwinputminimum: *mut u32,
            pdwinputmaximum: *mut u32,
            pdwoutputminimum: *mut u32,
            pdwoutputmaximum: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamLimits(
                    this,
                    core::mem::transmute_copy(&pdwinputminimum),
                    core::mem::transmute_copy(&pdwinputmaximum),
                    core::mem::transmute_copy(&pdwoutputminimum),
                    core::mem::transmute_copy(&pdwoutputmaximum),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetStreamCount<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcinputstreams: *mut u32,
            pcoutputstreams: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamCount(
                    this,
                    core::mem::transmute_copy(&pcinputstreams),
                    core::mem::transmute_copy(&pcoutputstreams),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetStreamIDs<Identity: IMFTransform_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwinputidarraysize: u32,
            pdwinputids: *mut u32,
            dwoutputidarraysize: u32,
            pdwoutputids: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetStreamIDs(
                    this,
                    core::mem::transmute_copy(&dwinputidarraysize),
                    core::mem::transmute_copy(&pdwinputids),
                    core::mem::transmute_copy(&dwoutputidarraysize),
                    core::mem::transmute_copy(&pdwoutputids),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInputStreamInfo<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            pstreaminfo: *mut MFT_INPUT_STREAM_INFO,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::GetInputStreamInfo(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                    core::mem::transmute_copy(&pstreaminfo),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwoutputstreamid: u32,
            pstreaminfo: *mut MFT_OUTPUT_STREAM_INFO,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputStreamInfo(
                    this,
                    core::mem::transmute_copy(&dwoutputstreamid),
                ) {
                    Ok(ok__) => {
                        pstreaminfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributes<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pattributes: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetAttributes(this) {
                    Ok(ok__) => {
                        pattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            pattributes: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputStreamAttributes(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                ) {
                    Ok(ok__) => {
                        pattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwoutputstreamid: u32,
            pattributes: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputStreamAttributes(
                    this,
                    core::mem::transmute_copy(&dwoutputstreamid),
                ) {
                    Ok(ok__) => {
                        pattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteInputStream<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamid: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::DeleteInputStream(this, core::mem::transmute_copy(&dwstreamid))
                    .into()
            }
        }
        unsafe extern "system" fn AddInputStreams<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cstreams: u32,
            adwstreamids: *const u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::AddInputStreams(
                    this,
                    core::mem::transmute_copy(&cstreams),
                    core::mem::transmute_copy(&adwstreamids),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInputAvailableType<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            dwtypeindex: u32,
            pptype: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputAvailableType(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                    core::mem::transmute_copy(&dwtypeindex),
                ) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwoutputstreamid: u32,
            dwtypeindex: u32,
            pptype: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputAvailableType(
                    this,
                    core::mem::transmute_copy(&dwoutputstreamid),
                    core::mem::transmute_copy(&dwtypeindex),
                ) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputType<Identity: IMFTransform_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            ptype: *mut core::ffi::c_void,
            dwflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetInputType(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                    core::mem::transmute_copy(&ptype),
                    core::mem::transmute_copy(&dwflags),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetOutputType<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwoutputstreamid: u32,
            ptype: *mut core::ffi::c_void,
            dwflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetOutputType(
                    this,
                    core::mem::transmute_copy(&dwoutputstreamid),
                    core::mem::transmute_copy(&ptype),
                    core::mem::transmute_copy(&dwflags),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInputCurrentType<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            pptype: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputCurrentType(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                ) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwoutputstreamid: u32,
            pptype: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputCurrentType(
                    this,
                    core::mem::transmute_copy(&dwoutputstreamid),
                ) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputStatus<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            pdwflags: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetInputStatus(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                ) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputStatus<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdwflags: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTransform_Impl::GetOutputStatus(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputBounds<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hnslowerbound: i64,
            hnsupperbound: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::SetOutputBounds(
                    this,
                    core::mem::transmute_copy(&hnslowerbound),
                    core::mem::transmute_copy(&hnsupperbound),
                )
                .into()
            }
        }
        unsafe extern "system" fn ProcessEvent<Identity: IMFTransform_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            pevent: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessEvent(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                    core::mem::transmute_copy(&pevent),
                )
                .into()
            }
        }
        unsafe extern "system" fn ProcessMessage<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            emessage: MFT_MESSAGE_TYPE,
            ulparam: usize,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessMessage(
                    this,
                    core::mem::transmute_copy(&emessage),
                    core::mem::transmute_copy(&ulparam),
                )
                .into()
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IMFTransform_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwinputstreamid: u32,
            psample: *mut core::ffi::c_void,
            dwflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessInput(
                    this,
                    core::mem::transmute_copy(&dwinputstreamid),
                    core::mem::transmute_copy(&psample),
                    core::mem::transmute_copy(&dwflags),
                )
                .into()
            }
        }
        unsafe extern "system" fn ProcessOutput<
            Identity: IMFTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwflags: u32,
            coutputbuffercount: u32,
            poutputsamples: *mut MFT_OUTPUT_DATA_BUFFER,
            pdwstatus: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTransform_Impl::ProcessOutput(
                    this,
                    core::mem::transmute_copy(&dwflags),
                    core::mem::transmute_copy(&coutputbuffercount),
                    core::mem::transmute_copy(&poutputsamples),
                    core::mem::transmute_copy(&pdwstatus),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamLimits: GetStreamLimits::<Identity, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetStreamIDs: GetStreamIDs::<Identity, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            GetInputStreamAttributes: GetInputStreamAttributes::<Identity, OFFSET>,
            GetOutputStreamAttributes: GetOutputStreamAttributes::<Identity, OFFSET>,
            DeleteInputStream: DeleteInputStream::<Identity, OFFSET>,
            AddInputStreams: AddInputStreams::<Identity, OFFSET>,
            GetInputAvailableType: GetInputAvailableType::<Identity, OFFSET>,
            GetOutputAvailableType: GetOutputAvailableType::<Identity, OFFSET>,
            SetInputType: SetInputType::<Identity, OFFSET>,
            SetOutputType: SetOutputType::<Identity, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, OFFSET>,
            GetOutputStatus: GetOutputStatus::<Identity, OFFSET>,
            SetOutputBounds: SetOutputBounds::<Identity, OFFSET>,
            ProcessEvent: ProcessEvent::<Identity, OFFSET>,
            ProcessMessage: ProcessMessage::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTransform {}
windows_core::imp::define_interface!(
    IPropertyBag2,
    IPropertyBag2_Vtbl,
    0x22f55882_280b_11d0_a8a9_00a0c90c2004
);
windows_core::imp::interface_hierarchy!(IPropertyBag2, windows_core::IUnknown);
#[repr(C)]
pub struct IPropertyBag2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Read: usize,
    Write: usize,
    CountProperties: usize,
    GetPropertyInfo: usize,
    LoadObject: usize,
}
impl windows_core::RuntimeName for IPropertyBag2 {}
windows_core::imp::define_interface!(
    ISequentialStream,
    ISequentialStream_Vtbl,
    0x0c733a30_2a1c_11ce_ade5_00aa0044773d
);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
#[repr(C)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Read: usize,
    Write: usize,
}
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(
    IStorage,
    IStorage_Vtbl,
    0x0000000b_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
#[repr(C)]
pub struct IStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateStream: usize,
    OpenStream: usize,
    CreateStorage: usize,
    OpenStorage: usize,
    CopyTo: usize,
    MoveElementTo: usize,
    Commit: usize,
    Revert: usize,
    EnumElements: usize,
    DestroyElement: usize,
    RenameElement: usize,
    SetElementTimes: usize,
    SetClass: usize,
    SetStateBits: usize,
    Stat: usize,
}
impl windows_core::RuntimeName for IStorage {}
windows_core::imp::define_interface!(
    IStream,
    IStream_Vtbl,
    0x0000000c_0000_0000_c000_000000000046
);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
#[repr(C)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    Seek: usize,
    SetSize: usize,
    CopyTo: usize,
    Commit: usize,
    Revert: usize,
    LockRegion: usize,
    UnlockRegion: usize,
    Stat: usize,
    Clone: usize,
}
impl windows_core::RuntimeName for IStream {}
windows_core::imp::define_interface!(
    IWICBitmapCodecInfo,
    IWICBitmapCodecInfo_Vtbl,
    0xe87a44c4_b76e_4c47_8b09_298eb12a2714
);
impl core::ops::Deref for IWICBitmapCodecInfo {
    type Target = IWICComponentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IWICBitmapCodecInfo,
    windows_core::IUnknown,
    IWICComponentInfo
);
#[repr(C)]
pub struct IWICBitmapCodecInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    GetContainerFormat: usize,
    GetPixelFormats: usize,
    GetColorManagementVersion: usize,
    GetDeviceManufacturer: usize,
    GetDeviceModels: usize,
    GetMimeTypes: usize,
    GetFileExtensions: usize,
    DoesSupportAnimation: usize,
    DoesSupportChromakey: usize,
    DoesSupportLossless: usize,
    DoesSupportMultiframe: usize,
    MatchesMimeType: usize,
}
impl windows_core::RuntimeName for IWICBitmapCodecInfo {}
windows_core::imp::define_interface!(
    IWICBitmapEncoder,
    IWICBitmapEncoder_Vtbl,
    0x00000103_a8f2_4877_ba0a_fd2b6645fb94
);
windows_core::imp::interface_hierarchy!(IWICBitmapEncoder, windows_core::IUnknown);
impl IWICBitmapEncoder {
    pub(crate) unsafe fn Initialize<P0>(
        &self,
        pistream: P0,
        cacheoption: WICBitmapEncoderCacheOption,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Initialize)(
                windows_core::Interface::as_raw(self),
                pistream.param().abi(),
                cacheoption,
            )
        }
    }
    pub(crate) unsafe fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetEncoderInfo(&self) -> windows_core::Result<IWICBitmapEncoderInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEncoderInfo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetColorContexts(
        &self,
        ccount: u32,
        ppicolorcontext: *const Option<IWICColorContext>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetColorContexts)(
                windows_core::Interface::as_raw(self),
                ccount,
                core::mem::transmute(ppicolorcontext),
            )
        }
    }
    pub(crate) unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPalette)(
                windows_core::Interface::as_raw(self),
                pipalette.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetThumbnail)(
                windows_core::Interface::as_raw(self),
                pithumbnail.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetPreview<P0>(&self, pipreview: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreview)(
                windows_core::Interface::as_raw(self),
                pipreview.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn CreateNewFrame(
        &self,
        ppiframeencode: *mut Option<IWICBitmapFrameEncode>,
        ppiencoderoptions: *mut Option<IPropertyBag2>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateNewFrame)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppiframeencode),
                core::mem::transmute(ppiencoderoptions),
            )
        }
    }
    pub(crate) unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetMetadataQueryWriter(
        &self,
    ) -> windows_core::Result<IWICMetadataQueryWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IWICBitmapEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        WICBitmapEncoderCacheOption,
    ) -> windows_core::HRESULT,
    pub GetContainerFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetEncoderInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateNewFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IWICBitmapEncoder_Impl: windows_core::IUnknownImpl {
    fn Initialize(
        &self,
        pistream: windows_core::Ref<IStream>,
        cacheoption: WICBitmapEncoderCacheOption,
    ) -> windows_core::Result<()>;
    fn GetContainerFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetEncoderInfo(&self) -> windows_core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(
        &self,
        ccount: u32,
        ppicolorcontext: *const Option<IWICColorContext>,
    ) -> windows_core::Result<()>;
    fn SetPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn SetThumbnail(
        &self,
        pithumbnail: windows_core::Ref<IWICBitmapSource>,
    ) -> windows_core::Result<()>;
    fn SetPreview(
        &self,
        pipreview: windows_core::Ref<IWICBitmapSource>,
    ) -> windows_core::Result<()>;
    fn CreateNewFrame(
        &self,
        ppiframeencode: windows_core::OutRef<IWICBitmapFrameEncode>,
        ppiencoderoptions: windows_core::OutRef<IPropertyBag2>,
    ) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
impl IWICBitmapEncoder_Vtbl {
    pub const fn new<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pistream: *mut core::ffi::c_void,
            cacheoption: WICBitmapEncoderCacheOption,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::Initialize(
                    this,
                    core::mem::transmute_copy(&pistream),
                    core::mem::transmute_copy(&cacheoption),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetContainerFormat<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pguidcontainerformat: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoder_Impl::GetContainerFormat(this) {
                    Ok(ok__) => {
                        pguidcontainerformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEncoderInfo<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppiencoderinfo: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoder_Impl::GetEncoderInfo(this) {
                    Ok(ok__) => {
                        ppiencoderinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorContexts<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ccount: u32,
            ppicolorcontext: *const *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetColorContexts(
                    this,
                    core::mem::transmute_copy(&ccount),
                    core::mem::transmute_copy(&ppicolorcontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPalette<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pipalette: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetPalette(this, core::mem::transmute_copy(&pipalette))
                    .into()
            }
        }
        unsafe extern "system" fn SetThumbnail<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pithumbnail: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetThumbnail(this, core::mem::transmute_copy(&pithumbnail))
                    .into()
            }
        }
        unsafe extern "system" fn SetPreview<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pipreview: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::SetPreview(this, core::mem::transmute_copy(&pipreview))
                    .into()
            }
        }
        unsafe extern "system" fn CreateNewFrame<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppiframeencode: *mut *mut core::ffi::c_void,
            ppiencoderoptions: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::CreateNewFrame(
                    this,
                    core::mem::transmute_copy(&ppiframeencode),
                    core::mem::transmute_copy(&ppiencoderoptions),
                )
                .into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWICBitmapEncoder_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapEncoder_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<
            Identity: IWICBitmapEncoder_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppimetadataquerywriter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapEncoder_Impl::GetMetadataQueryWriter(this) {
                    Ok(ok__) => {
                        ppimetadataquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, OFFSET>,
            GetEncoderInfo: GetEncoderInfo::<Identity, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
            SetPreview: SetPreview::<Identity, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapEncoder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapEncoder {}
windows_core::imp::define_interface!(
    IWICBitmapEncoderInfo,
    IWICBitmapEncoderInfo_Vtbl,
    0x94c9b4ee_a09f_4f92_8a1e_4a9bce7e76fb
);
impl core::ops::Deref for IWICBitmapEncoderInfo {
    type Target = IWICBitmapCodecInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IWICBitmapEncoderInfo,
    windows_core::IUnknown,
    IWICComponentInfo,
    IWICBitmapCodecInfo
);
#[repr(C)]
pub struct IWICBitmapEncoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    CreateInstance: usize,
}
impl windows_core::RuntimeName for IWICBitmapEncoderInfo {}
windows_core::imp::define_interface!(
    IWICBitmapFrameEncode,
    IWICBitmapFrameEncode_Vtbl,
    0x00000105_a8f2_4877_ba0a_fd2b6645fb94
);
windows_core::imp::interface_hierarchy!(IWICBitmapFrameEncode, windows_core::IUnknown);
impl IWICBitmapFrameEncode {
    pub(crate) unsafe fn Initialize<P0>(&self, piencoderoptions: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyBag2>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Initialize)(
                windows_core::Interface::as_raw(self),
                piencoderoptions.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetSize)(
                windows_core::Interface::as_raw(self),
                uiwidth,
                uiheight,
            )
        }
    }
    pub(crate) unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetResolution)(
                windows_core::Interface::as_raw(self),
                dpix,
                dpiy,
            )
        }
    }
    pub(crate) unsafe fn SetPixelFormat(
        &self,
        ppixelformat: *mut WICPixelFormatGUID,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetPixelFormat)(
                windows_core::Interface::as_raw(self),
                ppixelformat as _,
            )
        }
    }
    pub(crate) unsafe fn SetColorContexts(
        &self,
        ccount: u32,
        ppicolorcontext: *const Option<IWICColorContext>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetColorContexts)(
                windows_core::Interface::as_raw(self),
                ccount,
                core::mem::transmute(ppicolorcontext),
            )
        }
    }
    pub(crate) unsafe fn SetPalette<P0>(&self, pipalette: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICPalette>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPalette)(
                windows_core::Interface::as_raw(self),
                pipalette.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetThumbnail)(
                windows_core::Interface::as_raw(self),
                pithumbnail.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn WritePixels(
        &self,
        linecount: u32,
        cbstride: u32,
        cbbuffersize: u32,
        pbpixels: *const u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).WritePixels)(
                windows_core::Interface::as_raw(self),
                linecount,
                cbstride,
                cbbuffersize,
                pbpixels,
            )
        }
    }
    pub(crate) unsafe fn WriteSource<P0>(
        &self,
        pibitmapsource: P0,
        prc: *const WICRect,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).WriteSource)(
                windows_core::Interface::as_raw(self),
                pibitmapsource.param().abi(),
                prc,
            )
        }
    }
    pub(crate) unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetMetadataQueryWriter(
        &self,
    ) -> windows_core::Result<IWICMetadataQueryWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataQueryWriter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IWICBitmapFrameEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetResolution:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub SetPixelFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut WICPixelFormatGUID,
    ) -> windows_core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub WritePixels: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        *const u8,
    ) -> windows_core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const WICRect,
    ) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IWICBitmapFrameEncode_Impl: windows_core::IUnknownImpl {
    fn Initialize(
        &self,
        piencoderoptions: windows_core::Ref<IPropertyBag2>,
    ) -> windows_core::Result<()>;
    fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::Result<()>;
    fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()>;
    fn SetPixelFormat(&self, ppixelformat: *mut WICPixelFormatGUID) -> windows_core::Result<()>;
    fn SetColorContexts(
        &self,
        ccount: u32,
        ppicolorcontext: *const Option<IWICColorContext>,
    ) -> windows_core::Result<()>;
    fn SetPalette(&self, pipalette: windows_core::Ref<IWICPalette>) -> windows_core::Result<()>;
    fn SetThumbnail(
        &self,
        pithumbnail: windows_core::Ref<IWICBitmapSource>,
    ) -> windows_core::Result<()>;
    fn WritePixels(
        &self,
        linecount: u32,
        cbstride: u32,
        cbbuffersize: u32,
        pbpixels: *const u8,
    ) -> windows_core::Result<()>;
    fn WriteSource(
        &self,
        pibitmapsource: windows_core::Ref<IWICBitmapSource>,
        prc: *const WICRect,
    ) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn GetMetadataQueryWriter(&self) -> windows_core::Result<IWICMetadataQueryWriter>;
}
impl IWICBitmapFrameEncode_Vtbl {
    pub const fn new<Identity: IWICBitmapFrameEncode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            piencoderoptions: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::Initialize(
                    this,
                    core::mem::transmute_copy(&piencoderoptions),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetSize<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            uiwidth: u32,
            uiheight: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetSize(
                    this,
                    core::mem::transmute_copy(&uiwidth),
                    core::mem::transmute_copy(&uiheight),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetResolution<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dpix: f64,
            dpiy: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetResolution(
                    this,
                    core::mem::transmute_copy(&dpix),
                    core::mem::transmute_copy(&dpiy),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPixelFormat<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppixelformat: *mut WICPixelFormatGUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetPixelFormat(
                    this,
                    core::mem::transmute_copy(&ppixelformat),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetColorContexts<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ccount: u32,
            ppicolorcontext: *const *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetColorContexts(
                    this,
                    core::mem::transmute_copy(&ccount),
                    core::mem::transmute_copy(&ppicolorcontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPalette<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pipalette: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetPalette(this, core::mem::transmute_copy(&pipalette))
                    .into()
            }
        }
        unsafe extern "system" fn SetThumbnail<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pithumbnail: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::SetThumbnail(
                    this,
                    core::mem::transmute_copy(&pithumbnail),
                )
                .into()
            }
        }
        unsafe extern "system" fn WritePixels<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            linecount: u32,
            cbstride: u32,
            cbbuffersize: u32,
            pbpixels: *const u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::WritePixels(
                    this,
                    core::mem::transmute_copy(&linecount),
                    core::mem::transmute_copy(&cbstride),
                    core::mem::transmute_copy(&cbbuffersize),
                    core::mem::transmute_copy(&pbpixels),
                )
                .into()
            }
        }
        unsafe extern "system" fn WriteSource<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pibitmapsource: *mut core::ffi::c_void,
            prc: *const WICRect,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::WriteSource(
                    this,
                    core::mem::transmute_copy(&pibitmapsource),
                    core::mem::transmute_copy(&prc),
                )
                .into()
            }
        }
        unsafe extern "system" fn Commit<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapFrameEncode_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn GetMetadataQueryWriter<
            Identity: IWICBitmapFrameEncode_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppimetadataquerywriter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapFrameEncode_Impl::GetMetadataQueryWriter(this) {
                    Ok(ok__) => {
                        ppimetadataquerywriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SetResolution: SetResolution::<Identity, OFFSET>,
            SetPixelFormat: SetPixelFormat::<Identity, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, OFFSET>,
            WritePixels: WritePixels::<Identity, OFFSET>,
            WriteSource: WriteSource::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapFrameEncode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapFrameEncode {}
windows_core::imp::define_interface!(
    IWICBitmapSource,
    IWICBitmapSource_Vtbl,
    0x00000120_a8f2_4877_ba0a_fd2b6645fb94
);
windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
#[repr(C)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetSize: usize,
    GetPixelFormat: usize,
    GetResolution: usize,
    CopyPalette: usize,
    CopyPixels: usize,
}
impl windows_core::RuntimeName for IWICBitmapSource {}
windows_core::imp::define_interface!(
    IWICColorContext,
    IWICColorContext_Vtbl,
    0x3c613a02_34b2_44ea_9a7c_45aea9c6fd6d
);
windows_core::imp::interface_hierarchy!(IWICColorContext, windows_core::IUnknown);
#[repr(C)]
pub struct IWICColorContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    InitializeFromFilename: usize,
    InitializeFromMemory: usize,
    InitializeFromExifColorSpace: usize,
    GetType: usize,
    GetProfileBytes: usize,
    GetExifColorSpace: usize,
}
impl windows_core::RuntimeName for IWICColorContext {}
windows_core::imp::define_interface!(
    IWICComponentInfo,
    IWICComponentInfo_Vtbl,
    0x23bc3f0a_698b_4357_886b_f24d50671334
);
windows_core::imp::interface_hierarchy!(IWICComponentInfo, windows_core::IUnknown);
#[repr(C)]
pub struct IWICComponentInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetComponentType: usize,
    GetCLSID: usize,
    GetSigningStatus: usize,
    GetAuthor: usize,
    GetVendorGUID: usize,
    GetVersion: usize,
    GetSpecVersion: usize,
    GetFriendlyName: usize,
}
impl windows_core::RuntimeName for IWICComponentInfo {}
windows_core::imp::define_interface!(
    IWICMetadataQueryReader,
    IWICMetadataQueryReader_Vtbl,
    0x30989668_e1c9_4597_b395_458eedb808df
);
windows_core::imp::interface_hierarchy!(IWICMetadataQueryReader, windows_core::IUnknown);
#[repr(C)]
pub struct IWICMetadataQueryReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetContainerFormat: usize,
    GetLocation: usize,
    GetMetadataByName: usize,
    GetEnumerator: usize,
}
impl windows_core::RuntimeName for IWICMetadataQueryReader {}
windows_core::imp::define_interface!(
    IWICMetadataQueryWriter,
    IWICMetadataQueryWriter_Vtbl,
    0xa721791a_0def_4d06_bd91_2118bf1db10b
);
impl core::ops::Deref for IWICMetadataQueryWriter {
    type Target = IWICMetadataQueryReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IWICMetadataQueryWriter,
    windows_core::IUnknown,
    IWICMetadataQueryReader
);
#[repr(C)]
pub struct IWICMetadataQueryWriter_Vtbl {
    pub base__: IWICMetadataQueryReader_Vtbl,
    SetMetadataByName: usize,
    RemoveMetadataByName: usize,
}
impl windows_core::RuntimeName for IWICMetadataQueryWriter {}
windows_core::imp::define_interface!(
    IWICPalette,
    IWICPalette_Vtbl,
    0x00000040_a8f2_4877_ba0a_fd2b6645fb94
);
windows_core::imp::interface_hierarchy!(IWICPalette, windows_core::IUnknown);
#[repr(C)]
pub struct IWICPalette_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    InitializePredefined: usize,
    InitializeCustom: usize,
    InitializeFromBitmap: usize,
    InitializeFromPalette: usize,
    GetType: usize,
    GetColorCount: usize,
    GetColors: usize,
    IsBlackWhite: usize,
    IsGrayscale: usize,
    HasAlpha: usize,
}
impl windows_core::RuntimeName for IWICPalette {}
windows_core::imp::define_interface!(
    IWICStream,
    IWICStream_Vtbl,
    0x135ff860_22b7_4ddf_b0f6_218f4f299a43
);
impl core::ops::Deref for IWICStream {
    type Target = IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IWICStream,
    windows_core::IUnknown,
    ISequentialStream,
    IStream
);
impl IWICStream {
    pub(crate) unsafe fn InitializeFromIStream<P0>(&self, pistream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeFromIStream)(
                windows_core::Interface::as_raw(self),
                pistream.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn InitializeFromFilename<P0>(
        &self,
        wzfilename: P0,
        dwdesiredaccess: u32,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeFromFilename)(
                windows_core::Interface::as_raw(self),
                wzfilename.param().abi(),
                dwdesiredaccess,
            )
        }
    }
    pub(crate) unsafe fn InitializeFromMemory(
        &self,
        pbbuffer: *const u8,
        cbbuffersize: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeFromMemory)(
                windows_core::Interface::as_raw(self),
                pbbuffer,
                cbbuffersize,
            )
        }
    }
    pub(crate) unsafe fn InitializeFromIStreamRegion<P0>(
        &self,
        pistream: P0,
        uloffset: u64,
        ulmaxsize: u64,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeFromIStreamRegion)(
                windows_core::Interface::as_raw(self),
                pistream.param().abi(),
                uloffset,
                ulmaxsize,
            )
        }
    }
}
#[repr(C)]
pub struct IWICStream_Vtbl {
    pub base__: IStream_Vtbl,
    pub InitializeFromIStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InitializeFromFilename: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
    ) -> windows_core::HRESULT,
    pub InitializeFromMemory:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub InitializeFromIStreamRegion: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        u64,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IWICStream {}
pub type LPSAFEARRAY = *mut SAFEARRAY;
pub type LPVERSIONEDSTREAM = *mut VERSIONEDSTREAM;
pub const MFMediaType_Video: windows_core::GUID =
    windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
pub const MFSTARTUP_NOSOCKET: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_INPUT_STREAM_INFO {
    pub hnsMaxLatency: i64,
    pub dwFlags: u32,
    pub cbSize: u32,
    pub cbMaxLookahead: u32,
    pub cbAlignment: u32,
}
pub const MFT_MESSAGE_COMMAND_DRAIN: MFT_MESSAGE_TYPE = 1;
pub const MFT_MESSAGE_COMMAND_FLUSH: MFT_MESSAGE_TYPE = 0;
pub const MFT_MESSAGE_COMMAND_FLUSH_OUTPUT_STREAM: MFT_MESSAGE_TYPE = 268435464;
pub const MFT_MESSAGE_COMMAND_MARKER: MFT_MESSAGE_TYPE = 536870912;
pub const MFT_MESSAGE_COMMAND_SET_OUTPUT_STREAM_STATE: MFT_MESSAGE_TYPE = 268435463;
pub const MFT_MESSAGE_COMMAND_TICK: MFT_MESSAGE_TYPE = 4;
pub const MFT_MESSAGE_DROP_SAMPLES: MFT_MESSAGE_TYPE = 3;
pub const MFT_MESSAGE_NOTIFY_BEGIN_STREAMING: MFT_MESSAGE_TYPE = 268435456;
pub const MFT_MESSAGE_NOTIFY_END_OF_STREAM: MFT_MESSAGE_TYPE = 268435458;
pub const MFT_MESSAGE_NOTIFY_END_STREAMING: MFT_MESSAGE_TYPE = 268435457;
pub const MFT_MESSAGE_NOTIFY_EVENT: MFT_MESSAGE_TYPE = 268435462;
pub const MFT_MESSAGE_NOTIFY_REACQUIRE_RESOURCES: MFT_MESSAGE_TYPE = 268435461;
pub const MFT_MESSAGE_NOTIFY_RELEASE_RESOURCES: MFT_MESSAGE_TYPE = 268435460;
pub const MFT_MESSAGE_NOTIFY_START_OF_STREAM: MFT_MESSAGE_TYPE = 268435459;
pub const MFT_MESSAGE_SET_D3D_MANAGER: MFT_MESSAGE_TYPE = 2;
pub type MFT_MESSAGE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFT_OUTPUT_DATA_BUFFER {
    pub dwStreamID: u32,
    pub pSample: core::mem::ManuallyDrop<Option<IMFSample>>,
    pub dwStatus: u32,
    pub pEvents: core::mem::ManuallyDrop<Option<IMFCollection>>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_OUTPUT_STREAM_INFO {
    pub dwFlags: u32,
    pub cbSize: u32,
    pub cbAlignment: u32,
}
pub type MF_ATTRIBUTES_MATCH_TYPE = i32;
pub type MF_ATTRIBUTE_TYPE = i32;
pub const MF_MT_DEFAULT_STRIDE: windows_core::GUID =
    windows_core::GUID::from_u128(0x644b4e48_1e02_4516_b0eb_c01ca9d49ac6);
pub const MF_MT_FRAME_RATE: windows_core::GUID =
    windows_core::GUID::from_u128(0xc459a2e8_3d2c_4e44_b132_fee5156c7bb0);
pub const MF_MT_FRAME_SIZE: windows_core::GUID =
    windows_core::GUID::from_u128(0x1652c33d_d6b2_4012_b834_72030849a37d);
pub const MF_MT_MAJOR_TYPE: windows_core::GUID =
    windows_core::GUID::from_u128(0x48eba18e_f8c9_4687_bf11_0a74c9f96a8f);
pub const MF_MT_SUBTYPE: windows_core::GUID =
    windows_core::GUID::from_u128(0xf7e34c9a_42e8_4714_b74b_cb29d72c35e5);
pub const MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 32;
pub const MF_SOURCE_READERF_ENDOFSTREAM: MF_SOURCE_READER_FLAG = 2;
pub const MF_SOURCE_READERF_ERROR: MF_SOURCE_READER_FLAG = 1;
pub const MF_SOURCE_READER_ALL_STREAMS: i32 = -2;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: i32 = -4;
pub type MF_SOURCE_READER_FLAG = u32;
pub const MF_VERSION: i32 = 131184;
#[repr(C)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
impl Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: DECIMAL,
}
impl Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PROPVARIANT_0_0 {
    pub vt: VARTYPE,
    pub wReserved1: PROPVAR_PAD1,
    pub wReserved2: PROPVAR_PAD2,
    pub wReserved3: PROPVAR_PAD3,
    pub Anonymous: PROPVARIANT_0_0_0,
}
impl Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPVARIANT_0_0_0 {
    pub cVal: i8,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: SCODE,
    pub cyVal: CY,
    pub date: f64,
    pub filetime: FILETIME,
    pub puuid: *mut windows_core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: BLOB,
    pub pszVal: windows_core::PSTR,
    pub pwszVal: windows_core::PWSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub pStream: core::mem::ManuallyDrop<Option<IStream>>,
    pub pStorage: core::mem::ManuallyDrop<Option<IStorage>>,
    pub pVersionedStream: LPVERSIONEDSTREAM,
    pub parray: LPSAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: *mut i8,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub pdecVal: *mut DECIMAL,
    pub pscode: *mut SCODE,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut LPSAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
impl Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_PAD1(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_PAD2(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_PAD3(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCODE(pub i32);
pub const SRCCOPY: u32 = 13369376;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARIANT_BOOL(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARTYPE(pub u16);
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_core::GUID,
    pub pStream: core::mem::ManuallyDrop<Option<IStream>>,
}
pub type WICBitmapEncoderCacheOption = i32;
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = 2;
pub type WICPixelFormatGUID = windows_core::GUID;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
