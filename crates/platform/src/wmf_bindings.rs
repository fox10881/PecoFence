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
pub unsafe fn D3D11CreateDevice<P0>(
    padapter: P0,
    drivertype: D3D_DRIVER_TYPE,
    software: HMODULE,
    flags: u32,
    pfeaturelevels: Option<&[D3D_FEATURE_LEVEL]>,
    sdkversion: u32,
    ppdevice: Option<*mut Option<ID3D11Device>>,
    pfeaturelevel: Option<*mut D3D_FEATURE_LEVEL>,
    ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>,
) -> windows_core::HRESULT
where
    P0: windows_core::Param<IDXGIAdapter>,
{
    windows_core::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : *mut core::ffi::c_void, drivertype : D3D_DRIVER_TYPE, software : HMODULE, flags : u32, pfeaturelevels : *const D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        D3D11CreateDevice(
            padapter.param().abi(),
            drivertype,
            software,
            flags,
            pfeaturelevels.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            pfeaturelevels.map_or(0, |slice| slice.len().try_into().unwrap()),
            sdkversion,
            ppdevice.unwrap_or(core::mem::zeroed()) as _,
            pfeaturelevel.unwrap_or(core::mem::zeroed()) as _,
            ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn DCompositionCreateDevice2<P0, T>(renderingdevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("dcomp.dll" "system" fn DCompositionCreateDevice2(renderingdevice : *mut core::ffi::c_void, iid : *const windows_core::GUID, dcompositiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        DCompositionCreateDevice2(renderingdevice.param().abi(), &T::IID, &mut result__)
            .and_then(|| windows_core::imp::Type::from_abi(result__))
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
pub unsafe fn MFCreateDXGIDeviceManager(
    resettoken: *mut u32,
    ppdevicemanager: *mut Option<IMFDXGIDeviceManager>,
) -> windows_core::HRESULT {
    windows_core::link!("mfplat.dll" "system" fn MFCreateDXGIDeviceManager(resettoken : *mut u32, ppdevicemanager : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCreateDXGIDeviceManager(resettoken as _, core::mem::transmute(ppdevicemanager)) }
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct APP_DEPRECATED_HRESULT(pub windows_core::HRESULT);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type D3D11_AUTHENTICATED_CHANNEL_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
pub type D3D11_BLEND = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub IndependentBlendEnable: windows_core::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}
impl Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_BLEND_OP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BUFFEREX_SRV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub StructureByteStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_RTV {
    pub Anonymous: D3D11_BUFFER_RTV_0,
    pub Anonymous2: D3D11_BUFFER_RTV_1,
}
impl Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D11_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D11_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_SRV {
    pub Anonymous: D3D11_BUFFER_SRV_0,
    pub Anonymous2: D3D11_BUFFER_SRV_1,
}
impl Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D11_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D11_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BUFFER_UAV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
pub type D3D11_COMPARISON_FUNC = i32;
pub type D3D11_COUNTER = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: D3D11_COUNTER,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D11_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
pub type D3D11_COUNTER_TYPE = i32;
pub type D3D11_CPU_ACCESS_FLAG = i32;
pub const D3D11_CPU_ACCESS_READ: D3D11_CPU_ACCESS_FLAG = 131072;
pub const D3D11_CPU_ACCESS_WRITE: D3D11_CPU_ACCESS_FLAG = 65536;
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 32;
pub const D3D11_CREATE_DEVICE_DEBUG: D3D11_CREATE_DEVICE_FLAG = 2;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: D3D11_CREATE_DEVICE_FLAG = 64;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: D3D11_CREATE_DEVICE_FLAG = 256;
pub type D3D11_CREATE_DEVICE_FLAG = i32;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY:
    D3D11_CREATE_DEVICE_FLAG = 128;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D11_CREATE_DEVICE_FLAG =
    8;
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: D3D11_CREATE_DEVICE_FLAG = 1;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: D3D11_CREATE_DEVICE_FLAG = 4;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 2048;
pub type D3D11_CULL_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_core::BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: windows_core::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D11_TEX1D_DSV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D11_TEX2D_DSV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D11_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_DSV,
}
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_DEPTH_WRITE_MASK = i32;
pub type D3D11_DEVICE_CONTEXT_TYPE = i32;
pub type D3D11_DSV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_ENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
pub type D3D11_FEATURE = i32;
pub type D3D11_FILL_MODE = i32;
pub type D3D11_FILTER = i32;
pub type D3D11_INPUT_CLASSIFICATION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub Format: DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
pub type D3D11_MAP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
pub const D3D11_MAP_READ: D3D11_MAP = 1;
pub const D3D11_MAP_READ_WRITE: D3D11_MAP = 3;
pub const D3D11_MAP_WRITE: D3D11_MAP = 2;
pub const D3D11_MAP_WRITE_DISCARD: D3D11_MAP = 4;
pub const D3D11_MAP_WRITE_NO_OVERWRITE: D3D11_MAP = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_OMAC {
    pub Omac: [u8; 16],
}
impl Default for D3D11_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_PRIMITIVE_TOPOLOGY(pub D3D_PRIMITIVE_TOPOLOGY);
pub type D3D11_QUERY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_QUERY_DESC {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_core::BOOL,
    pub ScissorEnable: windows_core::BOOL,
    pub MultisampleEnable: windows_core::BOOL,
    pub AntialiasedLineEnable: windows_core::BOOL,
}
pub type D3D11_RECT = RECT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: windows_core::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
impl Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
impl Default for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_RESOURCE_DIMENSION = i32;
pub type D3D11_RTV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SDK_VERSION: i32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: windows_core::PCSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3D11_SRV_DIMENSION(pub D3D_SRV_DIMENSION);
pub type D3D11_STENCIL_OP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_VPOV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_VDOV {
    pub ArraySlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_VPIV {
    pub MipSlice: u32,
    pub ArraySlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_VPOV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
pub type D3D11_TEXTURE_ADDRESS_MODE = i32;
pub type D3D11_UAV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D11_TEX3D_UAV,
}
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_USAGE = i32;
pub const D3D11_USAGE_DEFAULT: D3D11_USAGE = 0;
pub const D3D11_USAGE_DYNAMIC: D3D11_USAGE = 2;
pub const D3D11_USAGE_IMMUTABLE: D3D11_USAGE = 1;
pub const D3D11_USAGE_STAGING: D3D11_USAGE = 3;
pub type D3D11_VDOV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_COLOR {
    pub Anonymous: D3D11_VIDEO_COLOR_0,
}
impl Default for D3D11_VIDEO_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_COLOR_0 {
    pub YCbCr: D3D11_VIDEO_COLOR_YCbCrA,
    pub RGBA: D3D11_VIDEO_COLOR_RGBA,
}
impl Default for D3D11_VIDEO_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_COLOR_RGBA {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_COLOR_YCbCrA {
    pub Y: f32,
    pub Cb: f32,
    pub Cr: f32,
    pub A: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    pub Caps: u32,
    pub KeyExchangeTypeCount: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub BufferIndex: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub FirstMBaddress: u32,
    pub NumMBsInBuffer: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub ReservedBits: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub PartialEncryption: windows_core::BOOL,
    pub EncryptedBlockInfo: D3D11_ENCRYPTED_BLOCK_INFO,
}
pub type D3D11_VIDEO_DECODER_BUFFER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_CONFIG {
    pub guidConfigBitstreamEncryption: windows_core::GUID,
    pub guidConfigMBcontrolEncryption: windows_core::GUID,
    pub guidConfigResidDiffEncryption: windows_core::GUID,
    pub ConfigBitstreamRaw: u32,
    pub ConfigMBcontrolRasterOrder: u32,
    pub ConfigResidDiffHost: u32,
    pub ConfigSpatialResid8: u32,
    pub ConfigResid8Subtraction: u32,
    pub ConfigSpatialHost8or9Clipping: u32,
    pub ConfigSpatialResidInterleaved: u32,
    pub ConfigIntraResidUnsigned: u32,
    pub ConfigResidDiffAccelerator: u32,
    pub ConfigHostInverseScan: u32,
    pub ConfigSpecificIDCT: u32,
    pub Config4GroupedCoefs: u32,
    pub ConfigMinRenderTargetBuffCount: u16,
    pub ConfigDecoderSpecific: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_DESC {
    pub Guid: windows_core::GUID,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub OutputFormat: DXGI_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_DECODER_EXTENSION {
    pub Function: u32,
    pub pPrivateInputData: *mut core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
    pub ResourceCount: u32,
    pub ppResourceList: *mut Option<ID3D11Resource>,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    pub DecodeProfile: windows_core::GUID,
    pub ViewDimension: D3D11_VDOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0,
}
impl Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VDOV,
}
impl Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_FRAME_FORMAT = i32;
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: D3D11_VIDEO_FRAME_FORMAT = 2;
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: D3D11_VIDEO_FRAME_FORMAT = 1;
pub const D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE: D3D11_VIDEO_FRAME_FORMAT = 0;
pub type D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CAPS {
    pub DeviceCaps: u32,
    pub FeatureCaps: u32,
    pub FilterCaps: u32,
    pub InputFormatCaps: u32,
    pub AutoStreamCaps: u32,
    pub StereoCaps: u32,
    pub RateConversionCapsCount: u32,
    pub MaxInputStreams: u32,
    pub MaxStreamStates: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    pub _bitfield: u32,
}
impl D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    pub fn Usage(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Usage(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn RGB_Range(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_RGB_Range(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn YCbCr_Matrix(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_YCbCr_Matrix(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn YCbCr_xvYCC(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_YCbCr_xvYCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Nominal_Range(&self) -> u32 {
        (self._bitfield << 26) >> 30
    }
    pub fn set_Nominal_Range(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    pub InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT,
    pub InputFrameRate: DXGI_RATIONAL,
    pub InputWidth: u32,
    pub InputHeight: u32,
    pub OutputFrameRate: DXGI_RATIONAL,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
    pub Usage: D3D11_VIDEO_USAGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    pub CustomRate: DXGI_RATIONAL,
    pub OutputFrames: u32,
    pub InputInterlaced: windows_core::BOOL,
    pub InputFramesOrFields: u32,
}
pub type D3D11_VIDEO_PROCESSOR_FILTER = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
pub type D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_INPUT: D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = 1;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_OUTPUT: D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    pub FourCC: u32,
    pub ViewDimension: D3D11_VPIV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0,
}
impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPIV,
}
impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_PROCESSOR_OUTPUT_RATE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    pub ViewDimension: D3D11_VPOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0,
}
impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPOV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_VPOV,
}
impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ProcessorCaps: u32,
    pub ITelecineCaps: u32,
    pub CustomRateCount: u32,
}
pub type D3D11_VIDEO_PROCESSOR_ROTATION = i32;
pub type D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE = i32;
pub type D3D11_VIDEO_PROCESSOR_STEREO_FORMAT = i32;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM {
    pub Enable: windows_core::BOOL,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ppPastSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
    pub pInputSurface: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
    pub ppFutureSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
    pub ppPastSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
    pub pInputSurfaceRight: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
    pub ppFutureSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
}
pub type D3D11_VIDEO_USAGE = i32;
pub const D3D11_VIDEO_USAGE_OPTIMAL_QUALITY: D3D11_VIDEO_USAGE = 2;
pub const D3D11_VIDEO_USAGE_OPTIMAL_SPEED: D3D11_VIDEO_USAGE = 1;
pub const D3D11_VIDEO_USAGE_PLAYBACK_NORMAL: D3D11_VIDEO_USAGE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
pub type D3D11_VPIV_DIMENSION = i32;
pub const D3D11_VPIV_DIMENSION_TEXTURE2D: D3D11_VPIV_DIMENSION = 1;
pub const D3D11_VPIV_DIMENSION_UNKNOWN: D3D11_VPIV_DIMENSION = 0;
pub type D3D11_VPOV_DIMENSION = i32;
pub const D3D11_VPOV_DIMENSION_TEXTURE2D: D3D11_VPOV_DIMENSION = 1;
pub const D3D11_VPOV_DIMENSION_TEXTURE2DARRAY: D3D11_VPOV_DIMENSION = 2;
pub const D3D11_VPOV_DIMENSION_UNKNOWN: D3D11_VPOV_DIMENSION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub type D3D_DRIVER_TYPE = i32;
pub const D3D_DRIVER_TYPE_HARDWARE: D3D_DRIVER_TYPE = 1;
pub const D3D_DRIVER_TYPE_NULL: D3D_DRIVER_TYPE = 3;
pub const D3D_DRIVER_TYPE_REFERENCE: D3D_DRIVER_TYPE = 2;
pub const D3D_DRIVER_TYPE_SOFTWARE: D3D_DRIVER_TYPE = 4;
pub const D3D_DRIVER_TYPE_UNKNOWN: D3D_DRIVER_TYPE = 0;
pub const D3D_DRIVER_TYPE_WARP: D3D_DRIVER_TYPE = 5;
pub type D3D_FEATURE_LEVEL = i32;
pub const D3D_FEATURE_LEVEL_10_0: D3D_FEATURE_LEVEL = 40960;
pub const D3D_FEATURE_LEVEL_10_1: D3D_FEATURE_LEVEL = 41216;
pub const D3D_FEATURE_LEVEL_11_0: D3D_FEATURE_LEVEL = 45056;
pub const D3D_FEATURE_LEVEL_11_1: D3D_FEATURE_LEVEL = 45312;
pub const D3D_FEATURE_LEVEL_12_0: D3D_FEATURE_LEVEL = 49152;
pub const D3D_FEATURE_LEVEL_12_1: D3D_FEATURE_LEVEL = 49408;
pub const D3D_FEATURE_LEVEL_12_2: D3D_FEATURE_LEVEL = 49664;
pub const D3D_FEATURE_LEVEL_1_0_CORE: D3D_FEATURE_LEVEL = 4096;
pub const D3D_FEATURE_LEVEL_1_0_GENERIC: D3D_FEATURE_LEVEL = 256;
pub const D3D_FEATURE_LEVEL_9_1: D3D_FEATURE_LEVEL = 37120;
pub const D3D_FEATURE_LEVEL_9_2: D3D_FEATURE_LEVEL = 37376;
pub const D3D_FEATURE_LEVEL_9_3: D3D_FEATURE_LEVEL = 37632;
pub type D3D_PRIMITIVE_TOPOLOGY = i32;
pub type D3D_SRV_DIMENSION = i32;
pub type DCOMPOSITION_BITMAP_INTERPOLATION_MODE = i32;
pub type DCOMPOSITION_BORDER_MODE = i32;
pub type DCOMPOSITION_COMPOSITE_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: LUID,
}
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ALPHA_MODE = i32;
pub const DXGI_ALPHA_MODE_FORCE_DWORD: DXGI_ALPHA_MODE = -1;
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = 3;
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = 1;
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = 2;
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = 0;
pub type DXGI_FORMAT = i32;
pub const DXGI_FORMAT_420_OPAQUE: DXGI_FORMAT = 106;
pub const DXGI_FORMAT_A4B4G4R4_UNORM: DXGI_FORMAT = 191;
pub const DXGI_FORMAT_A8P8: DXGI_FORMAT = 114;
pub const DXGI_FORMAT_A8_UNORM: DXGI_FORMAT = 65;
pub const DXGI_FORMAT_AI44: DXGI_FORMAT = 111;
pub const DXGI_FORMAT_AYUV: DXGI_FORMAT = 100;
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DXGI_FORMAT = 115;
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DXGI_FORMAT = 86;
pub const DXGI_FORMAT_B5G6R5_UNORM: DXGI_FORMAT = 85;
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DXGI_FORMAT = 90;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = 87;
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DXGI_FORMAT = 91;
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DXGI_FORMAT = 92;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DXGI_FORMAT = 88;
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DXGI_FORMAT = 93;
pub const DXGI_FORMAT_BC1_TYPELESS: DXGI_FORMAT = 70;
pub const DXGI_FORMAT_BC1_UNORM: DXGI_FORMAT = 71;
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DXGI_FORMAT = 72;
pub const DXGI_FORMAT_BC2_TYPELESS: DXGI_FORMAT = 73;
pub const DXGI_FORMAT_BC2_UNORM: DXGI_FORMAT = 74;
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DXGI_FORMAT = 75;
pub const DXGI_FORMAT_BC3_TYPELESS: DXGI_FORMAT = 76;
pub const DXGI_FORMAT_BC3_UNORM: DXGI_FORMAT = 77;
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DXGI_FORMAT = 78;
pub const DXGI_FORMAT_BC4_SNORM: DXGI_FORMAT = 81;
pub const DXGI_FORMAT_BC4_TYPELESS: DXGI_FORMAT = 79;
pub const DXGI_FORMAT_BC4_UNORM: DXGI_FORMAT = 80;
pub const DXGI_FORMAT_BC5_SNORM: DXGI_FORMAT = 84;
pub const DXGI_FORMAT_BC5_TYPELESS: DXGI_FORMAT = 82;
pub const DXGI_FORMAT_BC5_UNORM: DXGI_FORMAT = 83;
pub const DXGI_FORMAT_BC6H_SF16: DXGI_FORMAT = 96;
pub const DXGI_FORMAT_BC6H_TYPELESS: DXGI_FORMAT = 94;
pub const DXGI_FORMAT_BC6H_UF16: DXGI_FORMAT = 95;
pub const DXGI_FORMAT_BC7_TYPELESS: DXGI_FORMAT = 97;
pub const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT = 98;
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DXGI_FORMAT = 99;
pub const DXGI_FORMAT_D16_UNORM: DXGI_FORMAT = 55;
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DXGI_FORMAT = 45;
pub const DXGI_FORMAT_D32_FLOAT: DXGI_FORMAT = 40;
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DXGI_FORMAT = 20;
pub const DXGI_FORMAT_FORCE_UINT: DXGI_FORMAT = -1;
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DXGI_FORMAT = 69;
pub const DXGI_FORMAT_IA44: DXGI_FORMAT = 112;
pub const DXGI_FORMAT_NV11: DXGI_FORMAT = 110;
pub const DXGI_FORMAT_NV12: DXGI_FORMAT = 103;
pub const DXGI_FORMAT_P010: DXGI_FORMAT = 104;
pub const DXGI_FORMAT_P016: DXGI_FORMAT = 105;
pub const DXGI_FORMAT_P208: DXGI_FORMAT = 130;
pub const DXGI_FORMAT_P8: DXGI_FORMAT = 113;
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DXGI_FORMAT = 23;
pub const DXGI_FORMAT_R10G10B10A2_UINT: DXGI_FORMAT = 25;
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DXGI_FORMAT = 24;
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DXGI_FORMAT = 89;
pub const DXGI_FORMAT_R11G11B10_FLOAT: DXGI_FORMAT = 26;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DXGI_FORMAT = 10;
pub const DXGI_FORMAT_R16G16B16A16_SINT: DXGI_FORMAT = 14;
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DXGI_FORMAT = 13;
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DXGI_FORMAT = 9;
pub const DXGI_FORMAT_R16G16B16A16_UINT: DXGI_FORMAT = 12;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DXGI_FORMAT = 11;
pub const DXGI_FORMAT_R16G16_FLOAT: DXGI_FORMAT = 34;
pub const DXGI_FORMAT_R16G16_SINT: DXGI_FORMAT = 38;
pub const DXGI_FORMAT_R16G16_SNORM: DXGI_FORMAT = 37;
pub const DXGI_FORMAT_R16G16_TYPELESS: DXGI_FORMAT = 33;
pub const DXGI_FORMAT_R16G16_UINT: DXGI_FORMAT = 36;
pub const DXGI_FORMAT_R16G16_UNORM: DXGI_FORMAT = 35;
pub const DXGI_FORMAT_R16_FLOAT: DXGI_FORMAT = 54;
pub const DXGI_FORMAT_R16_SINT: DXGI_FORMAT = 59;
pub const DXGI_FORMAT_R16_SNORM: DXGI_FORMAT = 58;
pub const DXGI_FORMAT_R16_TYPELESS: DXGI_FORMAT = 53;
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = 57;
pub const DXGI_FORMAT_R16_UNORM: DXGI_FORMAT = 56;
pub const DXGI_FORMAT_R1_UNORM: DXGI_FORMAT = 66;
pub const DXGI_FORMAT_R24G8_TYPELESS: DXGI_FORMAT = 44;
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DXGI_FORMAT = 46;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = 2;
pub const DXGI_FORMAT_R32G32B32A32_SINT: DXGI_FORMAT = 4;
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DXGI_FORMAT = 1;
pub const DXGI_FORMAT_R32G32B32A32_UINT: DXGI_FORMAT = 3;
pub const DXGI_FORMAT_R32G32B32_FLOAT: DXGI_FORMAT = 6;
pub const DXGI_FORMAT_R32G32B32_SINT: DXGI_FORMAT = 8;
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DXGI_FORMAT = 5;
pub const DXGI_FORMAT_R32G32B32_UINT: DXGI_FORMAT = 7;
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = 16;
pub const DXGI_FORMAT_R32G32_SINT: DXGI_FORMAT = 18;
pub const DXGI_FORMAT_R32G32_TYPELESS: DXGI_FORMAT = 15;
pub const DXGI_FORMAT_R32G32_UINT: DXGI_FORMAT = 17;
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DXGI_FORMAT = 19;
pub const DXGI_FORMAT_R32_FLOAT: DXGI_FORMAT = 41;
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DXGI_FORMAT = 21;
pub const DXGI_FORMAT_R32_SINT: DXGI_FORMAT = 43;
pub const DXGI_FORMAT_R32_TYPELESS: DXGI_FORMAT = 39;
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = 42;
pub const DXGI_FORMAT_R8G8B8A8_SINT: DXGI_FORMAT = 32;
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DXGI_FORMAT = 31;
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DXGI_FORMAT = 27;
pub const DXGI_FORMAT_R8G8B8A8_UINT: DXGI_FORMAT = 30;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = 28;
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DXGI_FORMAT = 29;
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DXGI_FORMAT = 68;
pub const DXGI_FORMAT_R8G8_SINT: DXGI_FORMAT = 52;
pub const DXGI_FORMAT_R8G8_SNORM: DXGI_FORMAT = 51;
pub const DXGI_FORMAT_R8G8_TYPELESS: DXGI_FORMAT = 48;
pub const DXGI_FORMAT_R8G8_UINT: DXGI_FORMAT = 50;
pub const DXGI_FORMAT_R8G8_UNORM: DXGI_FORMAT = 49;
pub const DXGI_FORMAT_R8_SINT: DXGI_FORMAT = 64;
pub const DXGI_FORMAT_R8_SNORM: DXGI_FORMAT = 63;
pub const DXGI_FORMAT_R8_TYPELESS: DXGI_FORMAT = 60;
pub const DXGI_FORMAT_R8_UINT: DXGI_FORMAT = 62;
pub const DXGI_FORMAT_R8_UNORM: DXGI_FORMAT = 61;
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DXGI_FORMAT = 67;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: DXGI_FORMAT = 189;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: DXGI_FORMAT = 190;
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = 0;
pub const DXGI_FORMAT_V208: DXGI_FORMAT = 131;
pub const DXGI_FORMAT_V408: DXGI_FORMAT = 132;
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DXGI_FORMAT = 47;
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DXGI_FORMAT = 22;
pub const DXGI_FORMAT_Y210: DXGI_FORMAT = 108;
pub const DXGI_FORMAT_Y216: DXGI_FORMAT = 109;
pub const DXGI_FORMAT_Y410: DXGI_FORMAT = 101;
pub const DXGI_FORMAT_Y416: DXGI_FORMAT = 102;
pub const DXGI_FORMAT_YUY2: DXGI_FORMAT = 107;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: windows_core::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
impl Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
pub type DXGI_MODE_ROTATION = i32;
pub type DXGI_MODE_SCALING = i32;
pub type DXGI_MODE_SCANLINE_ORDER = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: DXGI_MODE_ROTATION,
    pub Monitor: HMONITOR,
}
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut RECT,
    pub pScrollRect: *mut RECT,
    pub pScrollOffset: *mut POINT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub type DXGI_RESIDENCY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
pub type DXGI_RGBA = D3DCOLORVALUE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: HWND,
    pub Windowed: windows_core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub Stereo: windows_core::BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: windows_core::BOOL,
}
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_USAGE(pub u32);
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
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HDC(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HINSTANCE(pub *mut core::ffi::c_void);
pub type HMODULE = HINSTANCE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMONITOR(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWND(pub *mut core::ffi::c_void);
windows_core::imp::define_interface!(
    ID3D11Asynchronous,
    ID3D11Asynchronous_Vtbl,
    0x4b35d0cd_1e15_4258_9c98_1b1333f6dd3b
);
impl core::ops::Deref for ID3D11Asynchronous {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Asynchronous,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11Asynchronous_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDataSize: usize,
}
impl windows_core::RuntimeName for ID3D11Asynchronous {}
windows_core::imp::define_interface!(
    ID3D11AuthenticatedChannel,
    ID3D11AuthenticatedChannel_Vtbl,
    0x3015a308_dcbd_47aa_a747_192486d14d4a
);
impl core::ops::Deref for ID3D11AuthenticatedChannel {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11AuthenticatedChannel,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11AuthenticatedChannel_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetCertificateSize: usize,
    GetCertificate: usize,
    GetChannelHandle: usize,
}
impl windows_core::RuntimeName for ID3D11AuthenticatedChannel {}
windows_core::imp::define_interface!(
    ID3D11BlendState,
    ID3D11BlendState_Vtbl,
    0x75b68faa_347d_4159_8f45_a0640f01cd9a
);
impl core::ops::Deref for ID3D11BlendState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11BlendState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11BlendState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11BlendState {}
windows_core::imp::define_interface!(
    ID3D11Buffer,
    ID3D11Buffer_Vtbl,
    0x48570b85_d1ee_4fcd_a250_eb350722b037
);
impl core::ops::Deref for ID3D11Buffer {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Buffer,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Buffer_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11Buffer {}
windows_core::imp::define_interface!(
    ID3D11ClassInstance,
    ID3D11ClassInstance_Vtbl,
    0xa6cd7faa_b0b7_4a2f_9436_8662a65797cb
);
impl core::ops::Deref for ID3D11ClassInstance {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ClassInstance,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11ClassInstance_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetClassLinkage: usize,
    GetDesc: usize,
    GetInstanceName: usize,
    GetTypeName: usize,
}
impl windows_core::RuntimeName for ID3D11ClassInstance {}
windows_core::imp::define_interface!(
    ID3D11ClassLinkage,
    ID3D11ClassLinkage_Vtbl,
    0xddf57cba_9543_46e4_a12b_f207a0fe7fed
);
impl core::ops::Deref for ID3D11ClassLinkage {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ClassLinkage,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11ClassLinkage_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetClassInstance: usize,
    CreateClassInstance: usize,
}
impl windows_core::RuntimeName for ID3D11ClassLinkage {}
windows_core::imp::define_interface!(
    ID3D11CommandList,
    ID3D11CommandList_Vtbl,
    0xa24bc4d1_769e_43f7_8013_98ff566c18e2
);
impl core::ops::Deref for ID3D11CommandList {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11CommandList,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11CommandList_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetContextFlags: usize,
}
impl windows_core::RuntimeName for ID3D11CommandList {}
windows_core::imp::define_interface!(
    ID3D11ComputeShader,
    ID3D11ComputeShader_Vtbl,
    0x4f5b196e_c2bd_495e_bd01_1fded38e4969
);
impl core::ops::Deref for ID3D11ComputeShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ComputeShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11ComputeShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11ComputeShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11ComputeShader_Vtbl {
    pub const fn new<Identity: ID3D11ComputeShader_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ComputeShader as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11ComputeShader {}
windows_core::imp::define_interface!(
    ID3D11Counter,
    ID3D11Counter_Vtbl,
    0x6e8c49fb_a371_4770_b440_29086022b741
);
impl core::ops::Deref for ID3D11Counter {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Counter,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Asynchronous
);
#[repr(C)]
pub struct ID3D11Counter_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11Counter {}
windows_core::imp::define_interface!(
    ID3D11CryptoSession,
    ID3D11CryptoSession_Vtbl,
    0x9b32f9ad_bdcc_40a6_a39d_d5c865845720
);
impl core::ops::Deref for ID3D11CryptoSession {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11CryptoSession,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11CryptoSession_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetCryptoType: usize,
    GetDecoderProfile: usize,
    GetCertificateSize: usize,
    GetCertificate: usize,
    GetCryptoSessionHandle: usize,
}
impl windows_core::RuntimeName for ID3D11CryptoSession {}
windows_core::imp::define_interface!(
    ID3D11DepthStencilState,
    ID3D11DepthStencilState_Vtbl,
    0x03823efb_8d8f_4e1c_9aa2_f64bb2cbfdf1
);
impl core::ops::Deref for ID3D11DepthStencilState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DepthStencilState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11DepthStencilState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11DepthStencilState {}
windows_core::imp::define_interface!(
    ID3D11DepthStencilView,
    ID3D11DepthStencilView_Vtbl,
    0x9fdac92a_1876_48c3_afad_25b94f84a9b6
);
impl core::ops::Deref for ID3D11DepthStencilView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DepthStencilView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11DepthStencilView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11DepthStencilView {}
windows_core::imp::define_interface!(
    ID3D11Device,
    ID3D11Device_Vtbl,
    0xdb6f6ddb_ac77_4e88_8253_819df9bbf140
);
windows_core::imp::interface_hierarchy!(ID3D11Device, windows_core::IUnknown);
impl ID3D11Device {
    pub(crate) unsafe fn CreateBuffer(
        &self,
        pdesc: *const D3D11_BUFFER_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        ppbuffer: Option<*mut Option<ID3D11Buffer>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateBuffer)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                ppbuffer.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateTexture1D(
        &self,
        pdesc: *const D3D11_TEXTURE1D_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        pptexture1d: Option<*mut Option<ID3D11Texture1D>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateTexture1D)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                pptexture1d.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateTexture2D(
        &self,
        pdesc: *const D3D11_TEXTURE2D_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        pptexture2d: Option<*mut Option<ID3D11Texture2D>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateTexture2D)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                pptexture2d.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateTexture3D(
        &self,
        pdesc: *const D3D11_TEXTURE3D_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        pptexture3d: Option<*mut Option<ID3D11Texture3D>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateTexture3D)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                pptexture3d.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateShaderResourceView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>,
        ppsrview: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateShaderResourceView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                ppsrview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateUnorderedAccessView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>,
        ppuaview: Option<*mut Option<ID3D11UnorderedAccessView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateUnorderedAccessView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                ppuaview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateRenderTargetView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_RENDER_TARGET_VIEW_DESC>,
        pprtview: Option<*mut Option<ID3D11RenderTargetView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateRenderTargetView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                pprtview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateDepthStencilView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>,
        ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDepthStencilView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateInputLayout(
        &self,
        pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC],
        pshaderbytecodewithinputsignature: &[u8],
        ppinputlayout: Option<*mut Option<ID3D11InputLayout>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateInputLayout)(
                windows_core::Interface::as_raw(self),
                pinputelementdescs.as_ptr(),
                pinputelementdescs.len().try_into().unwrap(),
                core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()),
                pshaderbytecodewithinputsignature.len().try_into().unwrap(),
                ppinputlayout.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateVertexShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppvertexshader: Option<*mut Option<ID3D11VertexShader>>,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateVertexShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppvertexshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateGeometryShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateGeometryShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppgeometryshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateGeometryShaderWithStreamOutput<P7>(
        &self,
        pshaderbytecode: &[u8],
        psodeclaration: Option<&[D3D11_SO_DECLARATION_ENTRY]>,
        pbufferstrides: Option<&[u32]>,
        rasterizedstream: u32,
        pclasslinkage: P7,
        ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>,
    ) -> windows_core::HRESULT
    where
        P7: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                psodeclaration.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                psodeclaration.map_or(0, |slice| slice.len().try_into().unwrap()),
                pbufferstrides.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                pbufferstrides.map_or(0, |slice| slice.len().try_into().unwrap()),
                rasterizedstream,
                pclasslinkage.param().abi(),
                ppgeometryshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreatePixelShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        pppixelshader: Option<*mut Option<ID3D11PixelShader>>,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreatePixelShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                pppixelshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateHullShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        pphullshader: Option<*mut Option<ID3D11HullShader>>,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateHullShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                pphullshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateDomainShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppdomainshader: Option<*mut Option<ID3D11DomainShader>>,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDomainShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppdomainshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateComputeShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppcomputeshader: Option<*mut Option<ID3D11ComputeShader>>,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateComputeShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppcomputeshader.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClassLinkage)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateBlendState(
        &self,
        pblendstatedesc: *const D3D11_BLEND_DESC,
        ppblendstate: Option<*mut Option<ID3D11BlendState>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateBlendState)(
                windows_core::Interface::as_raw(self),
                pblendstatedesc,
                ppblendstate.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateDepthStencilState(
        &self,
        pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC,
        ppdepthstencilstate: Option<*mut Option<ID3D11DepthStencilState>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDepthStencilState)(
                windows_core::Interface::as_raw(self),
                pdepthstencildesc,
                ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateRasterizerState(
        &self,
        prasterizerdesc: *const D3D11_RASTERIZER_DESC,
        pprasterizerstate: Option<*mut Option<ID3D11RasterizerState>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateRasterizerState)(
                windows_core::Interface::as_raw(self),
                prasterizerdesc,
                pprasterizerstate.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateSamplerState(
        &self,
        psamplerdesc: *const D3D11_SAMPLER_DESC,
        ppsamplerstate: Option<*mut Option<ID3D11SamplerState>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateSamplerState)(
                windows_core::Interface::as_raw(self),
                psamplerdesc,
                ppsamplerstate.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateQuery(
        &self,
        pquerydesc: *const D3D11_QUERY_DESC,
        ppquery: Option<*mut Option<ID3D11Query>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateQuery)(
                windows_core::Interface::as_raw(self),
                pquerydesc,
                ppquery.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreatePredicate(
        &self,
        ppredicatedesc: *const D3D11_QUERY_DESC,
        pppredicate: Option<*mut Option<ID3D11Predicate>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreatePredicate)(
                windows_core::Interface::as_raw(self),
                ppredicatedesc,
                pppredicate.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateCounter(
        &self,
        pcounterdesc: *const D3D11_COUNTER_DESC,
        ppcounter: Option<*mut Option<ID3D11Counter>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateCounter)(
                windows_core::Interface::as_raw(self),
                pcounterdesc,
                ppcounter.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateDeferredContext(
        &self,
        contextflags: u32,
        ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDeferredContext)(
                windows_core::Interface::as_raw(self),
                contextflags,
                ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn OpenSharedResource<T>(
        &self,
        hresource: HANDLE,
        result__: *mut Option<T>,
    ) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OpenSharedResource)(
                windows_core::Interface::as_raw(self),
                hresource,
                &T::IID,
                result__ as *mut _ as *mut _,
            )
            .ok()
        }
    }
    pub(crate) unsafe fn CheckFormatSupport(
        &self,
        format: DXGI_FORMAT,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckFormatSupport)(
                windows_core::Interface::as_raw(self),
                format,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn CheckMultisampleQualityLevels(
        &self,
        format: DXGI_FORMAT,
        samplecount: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(
                windows_core::Interface::as_raw(self),
                format,
                samplecount,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCounterInfo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn CheckCounter(
        &self,
        pdesc: *const D3D11_COUNTER_DESC,
        ptype: *mut D3D11_COUNTER_TYPE,
        pactivecounters: *mut u32,
        szname: Option<windows_core::PSTR>,
        pnamelength: Option<*mut u32>,
        szunits: Option<windows_core::PSTR>,
        punitslength: Option<*mut u32>,
        szdescription: Option<windows_core::PSTR>,
        pdescriptionlength: Option<*mut u32>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CheckCounter)(
                windows_core::Interface::as_raw(self),
                pdesc,
                ptype as _,
                pactivecounters as _,
                szname.unwrap_or(core::mem::zeroed()) as _,
                pnamelength.unwrap_or(core::mem::zeroed()) as _,
                szunits.unwrap_or(core::mem::zeroed()) as _,
                punitslength.unwrap_or(core::mem::zeroed()) as _,
                szdescription.unwrap_or(core::mem::zeroed()) as _,
                pdescriptionlength.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CheckFeatureSupport(
        &self,
        feature: D3D11_FEATURE,
        pfeaturesupportdata: *mut core::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CheckFeatureSupport)(
                windows_core::Interface::as_raw(self),
                feature,
                pfeaturesupportdata as _,
                featuresupportdatasize,
            )
        }
    }
    pub(crate) unsafe fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: Option<*mut core::ffi::c_void>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                pdatasize as _,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: Option<*const core::ffi::c_void>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                datasize,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn SetPrivateDataInterface<P1>(
        &self,
        guid: *const windows_core::GUID,
        pdata: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                guid,
                pdata.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetFeatureLevel(&self) -> D3D_FEATURE_LEVEL {
        unsafe {
            (windows_core::Interface::vtable(self).GetFeatureLevel)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetCreationFlags)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetDeviceRemovedReason(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetImmediateContext(&self) -> windows_core::Result<ID3D11DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub(crate) unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetExceptionMode)(
                windows_core::Interface::as_raw(self),
                raiseflags,
            )
        }
    }
    pub(crate) unsafe fn GetExceptionMode(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetExceptionMode)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct ID3D11Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_BUFFER_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTexture1D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_TEXTURE1D_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTexture2D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_TEXTURE2D_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTexture3D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_TEXTURE3D_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateShaderResourceView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateUnorderedAccessView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRenderTargetView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_RENDER_TARGET_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDepthStencilView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateInputLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_INPUT_ELEMENT_DESC,
        u32,
        *const core::ffi::c_void,
        usize,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *const D3D11_SO_DECLARATION_ENTRY,
        u32,
        *const u32,
        u32,
        u32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateHullShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDomainShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateComputeShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateClassLinkage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_BLEND_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_DEPTH_STENCIL_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_RASTERIZER_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_SAMPLER_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_QUERY_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_QUERY_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_COUNTER_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDeferredContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OpenSharedResource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CheckFormatSupport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_FORMAT,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_FORMAT,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub CheckCounterInfo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_COUNTER_DESC,
        *mut D3D11_COUNTER_TYPE,
        *mut u32,
        windows_core::PSTR,
        *mut u32,
        windows_core::PSTR,
        *mut u32,
        windows_core::PSTR,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D3D11_FEATURE,
        *mut core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D_FEATURE_LEVEL,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDeviceRemovedReason:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetExceptionMode:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D11Device_Impl: windows_core::IUnknownImpl {
    fn CreateBuffer(
        &self,
        pdesc: *const D3D11_BUFFER_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        ppbuffer: windows_core::OutRef<ID3D11Buffer>,
    ) -> windows_core::Result<()>;
    fn CreateTexture1D(
        &self,
        pdesc: *const D3D11_TEXTURE1D_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        pptexture1d: windows_core::OutRef<ID3D11Texture1D>,
    ) -> windows_core::Result<()>;
    fn CreateTexture2D(
        &self,
        pdesc: *const D3D11_TEXTURE2D_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        pptexture2d: windows_core::OutRef<ID3D11Texture2D>,
    ) -> windows_core::Result<()>;
    fn CreateTexture3D(
        &self,
        pdesc: *const D3D11_TEXTURE3D_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        pptexture3d: windows_core::OutRef<ID3D11Texture3D>,
    ) -> windows_core::Result<()>;
    fn CreateShaderResourceView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        ppsrview: windows_core::OutRef<ID3D11ShaderResourceView>,
    ) -> windows_core::Result<()>;
    fn CreateUnorderedAccessView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        ppuaview: windows_core::OutRef<ID3D11UnorderedAccessView>,
    ) -> windows_core::Result<()>;
    fn CreateRenderTargetView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        pprtview: windows_core::OutRef<ID3D11RenderTargetView>,
    ) -> windows_core::Result<()>;
    fn CreateDepthStencilView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>,
    ) -> windows_core::Result<()>;
    fn CreateInputLayout(
        &self,
        pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC,
        numelements: u32,
        pshaderbytecodewithinputsignature: *const core::ffi::c_void,
        bytecodelength: usize,
        ppinputlayout: windows_core::OutRef<ID3D11InputLayout>,
    ) -> windows_core::Result<()>;
    fn CreateVertexShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppvertexshader: windows_core::OutRef<ID3D11VertexShader>,
    ) -> windows_core::Result<()>;
    fn CreateGeometryShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>,
    ) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        psodeclaration: *const D3D11_SO_DECLARATION_ENTRY,
        numentries: u32,
        pbufferstrides: *const u32,
        numstrides: u32,
        rasterizedstream: u32,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>,
    ) -> windows_core::Result<()>;
    fn CreatePixelShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        pppixelshader: windows_core::OutRef<ID3D11PixelShader>,
    ) -> windows_core::Result<()>;
    fn CreateHullShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        pphullshader: windows_core::OutRef<ID3D11HullShader>,
    ) -> windows_core::Result<()>;
    fn CreateDomainShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppdomainshader: windows_core::OutRef<ID3D11DomainShader>,
    ) -> windows_core::Result<()>;
    fn CreateComputeShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppcomputeshader: windows_core::OutRef<ID3D11ComputeShader>,
    ) -> windows_core::Result<()>;
    fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(
        &self,
        pblendstatedesc: *const D3D11_BLEND_DESC,
        ppblendstate: windows_core::OutRef<ID3D11BlendState>,
    ) -> windows_core::Result<()>;
    fn CreateDepthStencilState(
        &self,
        pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC,
        ppdepthstencilstate: windows_core::OutRef<ID3D11DepthStencilState>,
    ) -> windows_core::Result<()>;
    fn CreateRasterizerState(
        &self,
        prasterizerdesc: *const D3D11_RASTERIZER_DESC,
        pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState>,
    ) -> windows_core::Result<()>;
    fn CreateSamplerState(
        &self,
        psamplerdesc: *const D3D11_SAMPLER_DESC,
        ppsamplerstate: windows_core::OutRef<ID3D11SamplerState>,
    ) -> windows_core::Result<()>;
    fn CreateQuery(
        &self,
        pquerydesc: *const D3D11_QUERY_DESC,
        ppquery: windows_core::OutRef<ID3D11Query>,
    ) -> windows_core::Result<()>;
    fn CreatePredicate(
        &self,
        ppredicatedesc: *const D3D11_QUERY_DESC,
        pppredicate: windows_core::OutRef<ID3D11Predicate>,
    ) -> windows_core::Result<()>;
    fn CreateCounter(
        &self,
        pcounterdesc: *const D3D11_COUNTER_DESC,
        ppcounter: windows_core::OutRef<ID3D11Counter>,
    ) -> windows_core::Result<()>;
    fn CreateDeferredContext(
        &self,
        contextflags: u32,
        ppdeferredcontext: windows_core::OutRef<ID3D11DeviceContext>,
    ) -> windows_core::Result<()>;
    fn OpenSharedResource(
        &self,
        hresource: HANDLE,
        returnedinterface: *const windows_core::GUID,
        ppresource: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(
        &self,
        format: DXGI_FORMAT,
        samplecount: u32,
    ) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(
        &self,
        pdesc: *const D3D11_COUNTER_DESC,
        ptype: *mut D3D11_COUNTER_TYPE,
        pactivecounters: *mut u32,
        szname: windows_core::PSTR,
        pnamelength: *mut u32,
        szunits: windows_core::PSTR,
        punitslength: *mut u32,
        szdescription: windows_core::PSTR,
        pdescriptionlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn CheckFeatureSupport(
        &self,
        feature: D3D11_FEATURE,
        pfeaturesupportdata: *mut core::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> windows_core::Result<()>;
    fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        guid: *const windows_core::GUID,
        pdata: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> D3D_FEATURE_LEVEL;
    fn GetCreationFlags(&self) -> u32;
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn GetImmediateContext(&self, ppimmediatecontext: windows_core::OutRef<ID3D11DeviceContext>);
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
}
impl ID3D11Device_Vtbl {
    pub const fn new<Identity: ID3D11Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBuffer<Identity: ID3D11Device_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_BUFFER_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            ppbuffer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateBuffer(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&ppbuffer),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTexture1D<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_TEXTURE1D_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            pptexture1d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture1D(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&pptexture1d),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTexture2D<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_TEXTURE2D_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            pptexture2d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture2D(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&pptexture2d),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTexture3D<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_TEXTURE3D_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            pptexture3d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture3D(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&pptexture3d),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
            ppsrview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateShaderResourceView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppsrview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
            ppuaview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateUnorderedAccessView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppuaview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC,
            pprtview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateRenderTargetView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pprtview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
            ppdepthstencilview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDepthStencilView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppdepthstencilview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateInputLayout<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC,
            numelements: u32,
            pshaderbytecodewithinputsignature: *const core::ffi::c_void,
            bytecodelength: usize,
            ppinputlayout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateInputLayout(
                    this,
                    core::mem::transmute_copy(&pinputelementdescs),
                    core::mem::transmute_copy(&numelements),
                    core::mem::transmute_copy(&pshaderbytecodewithinputsignature),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&ppinputlayout),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppvertexshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateVertexShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppvertexshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateGeometryShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppgeometryshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateGeometryShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppgeometryshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            psodeclaration: *const D3D11_SO_DECLARATION_ENTRY,
            numentries: u32,
            pbufferstrides: *const u32,
            numstrides: u32,
            rasterizedstream: u32,
            pclasslinkage: *mut core::ffi::c_void,
            ppgeometryshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateGeometryShaderWithStreamOutput(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&psodeclaration),
                    core::mem::transmute_copy(&numentries),
                    core::mem::transmute_copy(&pbufferstrides),
                    core::mem::transmute_copy(&numstrides),
                    core::mem::transmute_copy(&rasterizedstream),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppgeometryshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreatePixelShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            pppixelshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreatePixelShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&pppixelshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateHullShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            pphullshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateHullShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&pphullshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDomainShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppdomainshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDomainShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppdomainshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateComputeShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppcomputeshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateComputeShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppcomputeshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateClassLinkage<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pplinkage: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CreateClassLinkage(this) {
                    Ok(ok__) => {
                        pplinkage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlendState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pblendstatedesc: *const D3D11_BLEND_DESC,
            ppblendstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateBlendState(
                    this,
                    core::mem::transmute_copy(&pblendstatedesc),
                    core::mem::transmute_copy(&ppblendstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC,
            ppdepthstencilstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDepthStencilState(
                    this,
                    core::mem::transmute_copy(&pdepthstencildesc),
                    core::mem::transmute_copy(&ppdepthstencilstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            prasterizerdesc: *const D3D11_RASTERIZER_DESC,
            pprasterizerstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateRasterizerState(
                    this,
                    core::mem::transmute_copy(&prasterizerdesc),
                    core::mem::transmute_copy(&pprasterizerstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateSamplerState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psamplerdesc: *const D3D11_SAMPLER_DESC,
            ppsamplerstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateSamplerState(
                    this,
                    core::mem::transmute_copy(&psamplerdesc),
                    core::mem::transmute_copy(&ppsamplerstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ID3D11Device_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pquerydesc: *const D3D11_QUERY_DESC,
            ppquery: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateQuery(
                    this,
                    core::mem::transmute_copy(&pquerydesc),
                    core::mem::transmute_copy(&ppquery),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreatePredicate<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppredicatedesc: *const D3D11_QUERY_DESC,
            pppredicate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreatePredicate(
                    this,
                    core::mem::transmute_copy(&ppredicatedesc),
                    core::mem::transmute_copy(&pppredicate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateCounter<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcounterdesc: *const D3D11_COUNTER_DESC,
            ppcounter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateCounter(
                    this,
                    core::mem::transmute_copy(&pcounterdesc),
                    core::mem::transmute_copy(&ppcounter),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDeferredContext<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            contextflags: u32,
            ppdeferredcontext: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDeferredContext(
                    this,
                    core::mem::transmute_copy(&contextflags),
                    core::mem::transmute_copy(&ppdeferredcontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn OpenSharedResource<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hresource: HANDLE,
            returnedinterface: *const windows_core::GUID,
            ppresource: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::OpenSharedResource(
                    this,
                    core::mem::transmute_copy(&hresource),
                    core::mem::transmute_copy(&returnedinterface),
                    core::mem::transmute_copy(&ppresource),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckFormatSupport<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: DXGI_FORMAT,
            pformatsupport: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CheckFormatSupport(
                    this,
                    core::mem::transmute_copy(&format),
                ) {
                    Ok(ok__) => {
                        pformatsupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: DXGI_FORMAT,
            samplecount: u32,
            pnumqualitylevels: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CheckMultisampleQualityLevels(
                    this,
                    core::mem::transmute_copy(&format),
                    core::mem::transmute_copy(&samplecount),
                ) {
                    Ok(ok__) => {
                        pnumqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckCounterInfo<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcounterinfo: *mut D3D11_COUNTER_INFO,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo));
            }
        }
        unsafe extern "system" fn CheckCounter<Identity: ID3D11Device_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_COUNTER_DESC,
            ptype: *mut D3D11_COUNTER_TYPE,
            pactivecounters: *mut u32,
            szname: windows_core::PSTR,
            pnamelength: *mut u32,
            szunits: windows_core::PSTR,
            punitslength: *mut u32,
            szdescription: windows_core::PSTR,
            pdescriptionlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckCounter(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ptype),
                    core::mem::transmute_copy(&pactivecounters),
                    core::mem::transmute_copy(&szname),
                    core::mem::transmute_copy(&pnamelength),
                    core::mem::transmute_copy(&szunits),
                    core::mem::transmute_copy(&punitslength),
                    core::mem::transmute_copy(&szdescription),
                    core::mem::transmute_copy(&pdescriptionlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            feature: D3D11_FEATURE,
            pfeaturesupportdata: *mut core::ffi::c_void,
            featuresupportdatasize: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckFeatureSupport(
                    this,
                    core::mem::transmute_copy(&feature),
                    core::mem::transmute_copy(&pfeaturesupportdata),
                    core::mem::transmute_copy(&featuresupportdatasize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPrivateData<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdatasize: *mut u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdatasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateData<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFeatureLevel<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> D3D_FEATURE_LEVEL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetFeatureLevel(this)
            }
        }
        unsafe extern "system" fn GetCreationFlags<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetCreationFlags(this)
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetDeviceRemovedReason(this).into()
            }
        }
        unsafe extern "system" fn GetImmediateContext<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppimmediatecontext: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetImmediateContext(
                    this,
                    core::mem::transmute_copy(&ppimmediatecontext),
                );
            }
        }
        unsafe extern "system" fn SetExceptionMode<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            raiseflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags))
                    .into()
            }
        }
        unsafe extern "system" fn GetExceptionMode<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetExceptionMode(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBuffer: CreateBuffer::<Identity, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<
                Identity,
                OFFSET,
            >,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            CreateHullShader: CreateHullShader::<Identity, OFFSET>,
            CreateDomainShader: CreateDomainShader::<Identity, OFFSET>,
            CreateComputeShader: CreateComputeShader::<Identity, OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Identity, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, OFFSET>,
            CreateCounter: CreateCounter::<Identity, OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Identity, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, OFFSET>,
            CheckCounter: CheckCounter::<Identity, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            GetImmediateContext: GetImmediateContext::<Identity, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Device {}
windows_core::imp::define_interface!(
    ID3D11DeviceChild,
    ID3D11DeviceChild_Vtbl,
    0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae
);
windows_core::imp::interface_hierarchy!(ID3D11DeviceChild, windows_core::IUnknown);
impl ID3D11DeviceChild {
    pub(crate) unsafe fn GetDevice(&self) -> windows_core::Result<ID3D11Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub(crate) unsafe fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: Option<*mut core::ffi::c_void>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                pdatasize as _,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: Option<*const core::ffi::c_void>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                datasize,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn SetPrivateDataInterface<P1>(
        &self,
        guid: *const windows_core::GUID,
        pdata: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                guid,
                pdata.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ID3D11DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ID3D11DeviceChild_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: windows_core::OutRef<ID3D11Device>);
    fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        guid: *const windows_core::GUID,
        pdata: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ID3D11DeviceChild_Vtbl {
    pub const fn new<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<
            Identity: ID3D11DeviceChild_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppdevice: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice));
            }
        }
        unsafe extern "system" fn GetPrivateData<
            Identity: ID3D11DeviceChild_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdatasize: *mut u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::GetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdatasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateData<
            Identity: ID3D11DeviceChild_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: ID3D11DeviceChild_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DeviceChild {}
windows_core::imp::define_interface!(
    ID3D11DeviceContext,
    ID3D11DeviceContext_Vtbl,
    0xc0bfa96c_e089_44fb_8eaf_26f8796190da
);
impl core::ops::Deref for ID3D11DeviceContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DeviceContext,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
impl ID3D11DeviceContext {
    pub(crate) unsafe fn VSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn PSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn PSSetShader<P0>(
        &self,
        ppixelshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11PixelShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShader)(
                windows_core::Interface::as_raw(self),
                ppixelshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub(crate) unsafe fn PSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub(crate) unsafe fn VSSetShader<P0>(
        &self,
        pvertexshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11VertexShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShader)(
                windows_core::Interface::as_raw(self),
                pvertexshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub(crate) unsafe fn DrawIndexed(
        &self,
        indexcount: u32,
        startindexlocation: u32,
        basevertexlocation: i32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexed)(
                windows_core::Interface::as_raw(self),
                indexcount,
                startindexlocation,
                basevertexlocation,
            );
        }
    }
    pub(crate) unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(
                windows_core::Interface::as_raw(self),
                vertexcount,
                startvertexlocation,
            );
        }
    }
    pub(crate) unsafe fn Map<P0>(
        &self,
        presource: P0,
        subresource: u32,
        maptype: D3D11_MAP,
        mapflags: u32,
        pmappedresource: Option<*mut D3D11_MAPPED_SUBRESOURCE>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Map)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                subresource,
                maptype,
                mapflags,
                pmappedresource.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                subresource,
            );
        }
    }
    pub(crate) unsafe fn PSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D11InputLayout>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetInputLayout)(
                windows_core::Interface::as_raw(self),
                pinputlayout.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn IASetVertexBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppvertexbuffers: Option<*const Option<ID3D11Buffer>>,
        pstrides: Option<*const u32>,
        poffsets: Option<*const u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetVertexBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _,
                pstrides.unwrap_or(core::mem::zeroed()) as _,
                poffsets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn IASetIndexBuffer<P0>(
        &self,
        pindexbuffer: P0,
        format: DXGI_FORMAT,
        offset: u32,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetIndexBuffer)(
                windows_core::Interface::as_raw(self),
                pindexbuffer.param().abi(),
                format,
                offset,
            );
        }
    }
    pub(crate) unsafe fn DrawIndexedInstanced(
        &self,
        indexcountperinstance: u32,
        instancecount: u32,
        startindexlocation: u32,
        basevertexlocation: i32,
        startinstancelocation: u32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstanced)(
                windows_core::Interface::as_raw(self),
                indexcountperinstance,
                instancecount,
                startindexlocation,
                basevertexlocation,
                startinstancelocation,
            );
        }
    }
    pub(crate) unsafe fn DrawInstanced(
        &self,
        vertexcountperinstance: u32,
        instancecount: u32,
        startvertexlocation: u32,
        startinstancelocation: u32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstanced)(
                windows_core::Interface::as_raw(self),
                vertexcountperinstance,
                instancecount,
                startvertexlocation,
                startinstancelocation,
            );
        }
    }
    pub(crate) unsafe fn GSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn GSSetShader<P0>(
        &self,
        pshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11GeometryShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShader)(
                windows_core::Interface::as_raw(self),
                pshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub(crate) unsafe fn IASetPrimitiveTopology(&self, topology: D3D11_PRIMITIVE_TOPOLOGY) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(
                windows_core::Interface::as_raw(self),
                topology,
            );
        }
    }
    pub(crate) unsafe fn VSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn VSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub(crate) unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Begin)(
                windows_core::Interface::as_raw(self),
                pasync.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).End)(
                windows_core::Interface::as_raw(self),
                pasync.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn GetData<P0>(
        &self,
        pasync: P0,
        pdata: Option<*mut core::ffi::c_void>,
        datasize: u32,
        getdataflags: u32,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetData)(
                windows_core::Interface::as_raw(self),
                pasync.param().abi(),
                pdata.unwrap_or(core::mem::zeroed()) as _,
                datasize,
                getdataflags,
            )
        }
    }
    pub(crate) unsafe fn SetPredication<P0>(&self, ppredicate: P0, predicatevalue: bool)
    where
        P0: windows_core::Param<ID3D11Predicate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(
                windows_core::Interface::as_raw(self),
                ppredicate.param().abi(),
                predicatevalue.into(),
            );
        }
    }
    pub(crate) unsafe fn GSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn GSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub(crate) unsafe fn OMSetRenderTargets<P2>(
        &self,
        pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>,
        pdepthstencilview: P2,
    ) where
        P2: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargets)(
                windows_core::Interface::as_raw(self),
                pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                pdepthstencilview.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P2>(
        &self,
        pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>,
        pdepthstencilview: P2,
        uavstartslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>,
        puavinitialcounts: Option<*const u32>,
    ) where
        P2: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargetsAndUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                pdepthstencilview.param().abi(),
                uavstartslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
                puavinitialcounts.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn OMSetBlendState<P0>(
        &self,
        pblendstate: P0,
        blendfactor: Option<&[f32; 4]>,
        samplemask: u32,
    ) where
        P0: windows_core::Param<ID3D11BlendState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetBlendState)(
                windows_core::Interface::as_raw(self),
                pblendstate.param().abi(),
                blendfactor.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                samplemask,
            );
        }
    }
    pub(crate) unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D11DepthStencilState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetDepthStencilState)(
                windows_core::Interface::as_raw(self),
                pdepthstencilstate.param().abi(),
                stencilref,
            );
        }
    }
    pub(crate) unsafe fn SOSetTargets(
        &self,
        numbuffers: u32,
        ppsotargets: Option<*const Option<ID3D11Buffer>>,
        poffsets: Option<*const u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).SOSetTargets)(
                windows_core::Interface::as_raw(self),
                numbuffers,
                ppsotargets.unwrap_or(core::mem::zeroed()) as _,
                poffsets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn DrawAuto(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self));
        }
    }
    pub(crate) unsafe fn DrawIndexedInstancedIndirect<P0>(
        &self,
        pbufferforargs: P0,
        alignedbyteoffsetforargs: u32,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstancedIndirect)(
                windows_core::Interface::as_raw(self),
                pbufferforargs.param().abi(),
                alignedbyteoffsetforargs,
            );
        }
    }
    pub(crate) unsafe fn DrawInstancedIndirect<P0>(
        &self,
        pbufferforargs: P0,
        alignedbyteoffsetforargs: u32,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstancedIndirect)(
                windows_core::Interface::as_raw(self),
                pbufferforargs.param().abi(),
                alignedbyteoffsetforargs,
            );
        }
    }
    pub(crate) unsafe fn Dispatch(
        &self,
        threadgroupcountx: u32,
        threadgroupcounty: u32,
        threadgroupcountz: u32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).Dispatch)(
                windows_core::Interface::as_raw(self),
                threadgroupcountx,
                threadgroupcounty,
                threadgroupcountz,
            );
        }
    }
    pub(crate) unsafe fn DispatchIndirect<P0>(
        &self,
        pbufferforargs: P0,
        alignedbyteoffsetforargs: u32,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DispatchIndirect)(
                windows_core::Interface::as_raw(self),
                pbufferforargs.param().abi(),
                alignedbyteoffsetforargs,
            );
        }
    }
    pub(crate) unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D11RasterizerState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetState)(
                windows_core::Interface::as_raw(self),
                prasterizerstate.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D11_VIEWPORT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetViewports)(
                windows_core::Interface::as_raw(self),
                pviewports.map_or(0, |slice| slice.len().try_into().unwrap()),
                pviewports.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            );
        }
    }
    pub(crate) unsafe fn RSSetScissorRects(&self, prects: Option<&[D3D11_RECT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetScissorRects)(
                windows_core::Interface::as_raw(self),
                prects.map_or(0, |slice| slice.len().try_into().unwrap()),
                prects.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            );
        }
    }
    pub(crate) unsafe fn CopySubresourceRegion<P0, P5>(
        &self,
        pdstresource: P0,
        dstsubresource: u32,
        dstx: u32,
        dsty: u32,
        dstz: u32,
        psrcresource: P5,
        srcsubresource: u32,
        psrcbox: Option<*const D3D11_BOX>,
    ) where
        P0: windows_core::Param<ID3D11Resource>,
        P5: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopySubresourceRegion)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                dstsubresource,
                dstx,
                dsty,
                dstz,
                psrcresource.param().abi(),
                srcsubresource,
                psrcbox.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyResource)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                psrcresource.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn UpdateSubresource<P0>(
        &self,
        pdstresource: P0,
        dstsubresource: u32,
        pdstbox: Option<*const D3D11_BOX>,
        psrcdata: *const core::ffi::c_void,
        srcrowpitch: u32,
        srcdepthpitch: u32,
    ) where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateSubresource)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                dstsubresource,
                pdstbox.unwrap_or(core::mem::zeroed()) as _,
                psrcdata,
                srcrowpitch,
                srcdepthpitch,
            );
        }
    }
    pub(crate) unsafe fn CopyStructureCount<P0, P2>(
        &self,
        pdstbuffer: P0,
        dstalignedbyteoffset: u32,
        psrcview: P2,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
        P2: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyStructureCount)(
                windows_core::Interface::as_raw(self),
                pdstbuffer.param().abi(),
                dstalignedbyteoffset,
                psrcview.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn ClearRenderTargetView<P0>(
        &self,
        prendertargetview: P0,
        colorrgba: &[f32; 4],
    ) where
        P0: windows_core::Param<ID3D11RenderTargetView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRenderTargetView)(
                windows_core::Interface::as_raw(self),
                prendertargetview.param().abi(),
                colorrgba.as_ptr(),
            );
        }
    }
    pub(crate) unsafe fn ClearUnorderedAccessViewUint<P0>(
        &self,
        punorderedaccessview: P0,
        values: &[u32; 4],
    ) where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUnorderedAccessViewUint)(
                windows_core::Interface::as_raw(self),
                punorderedaccessview.param().abi(),
                values.as_ptr(),
            );
        }
    }
    pub(crate) unsafe fn ClearUnorderedAccessViewFloat<P0>(
        &self,
        punorderedaccessview: P0,
        values: &[f32; 4],
    ) where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUnorderedAccessViewFloat)(
                windows_core::Interface::as_raw(self),
                punorderedaccessview.param().abi(),
                values.as_ptr(),
            );
        }
    }
    pub(crate) unsafe fn ClearDepthStencilView<P0>(
        &self,
        pdepthstencilview: P0,
        clearflags: u32,
        depth: f32,
        stencil: u8,
    ) where
        P0: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearDepthStencilView)(
                windows_core::Interface::as_raw(self),
                pdepthstencilview.param().abi(),
                clearflags,
                depth,
                stencil,
            );
        }
    }
    pub(crate) unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D11ShaderResourceView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMips)(
                windows_core::Interface::as_raw(self),
                pshaderresourceview.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResourceMinLOD)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                minlod,
            );
        }
    }
    pub(crate) unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetResourceMinLOD)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn ResolveSubresource<P0, P2>(
        &self,
        pdstresource: P0,
        dstsubresource: u32,
        psrcresource: P2,
        srcsubresource: u32,
        format: DXGI_FORMAT,
    ) where
        P0: windows_core::Param<ID3D11Resource>,
        P2: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveSubresource)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                dstsubresource,
                psrcresource.param().abi(),
                srcsubresource,
                format,
            );
        }
    }
    pub(crate) unsafe fn ExecuteCommandList<P0>(&self, pcommandlist: P0, restorecontextstate: bool)
    where
        P0: windows_core::Param<ID3D11CommandList>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteCommandList)(
                windows_core::Interface::as_raw(self),
                pcommandlist.param().abi(),
                restorecontextstate.into(),
            );
        }
    }
    pub(crate) unsafe fn HSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn HSSetShader<P0>(
        &self,
        phullshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11HullShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetShader)(
                windows_core::Interface::as_raw(self),
                phullshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub(crate) unsafe fn HSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub(crate) unsafe fn HSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn DSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn DSSetShader<P0>(
        &self,
        pdomainshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11DomainShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetShader)(
                windows_core::Interface::as_raw(self),
                pdomainshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub(crate) unsafe fn DSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub(crate) unsafe fn DSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn CSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn CSSetUnorderedAccessViews(
        &self,
        startslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>,
        puavinitialcounts: Option<*const u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                startslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
                puavinitialcounts.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CSSetShader<P0>(
        &self,
        pcomputeshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11ComputeShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetShader)(
                windows_core::Interface::as_raw(self),
                pcomputeshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub(crate) unsafe fn CSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub(crate) unsafe fn CSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub(crate) unsafe fn VSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn PSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn PSGetShader(
        &self,
        pppixelshader: *mut Option<ID3D11PixelShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pppixelshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn PSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VSGetShader(
        &self,
        ppvertexshader: *mut Option<ID3D11VertexShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppvertexshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn PSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D11InputLayout> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetInputLayout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub(crate) unsafe fn IAGetVertexBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppvertexbuffers: Option<*mut Option<ID3D11Buffer>>,
        pstrides: Option<*mut u32>,
        poffsets: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetVertexBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _,
                pstrides.unwrap_or(core::mem::zeroed()) as _,
                poffsets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn IAGetIndexBuffer(
        &self,
        pindexbuffer: *mut Option<ID3D11Buffer>,
        format: Option<*mut DXGI_FORMAT>,
        offset: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetIndexBuffer)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pindexbuffer),
                format.unwrap_or(core::mem::zeroed()) as _,
                offset.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn GSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn GSGetShader(
        &self,
        ppgeometryshader: *mut Option<ID3D11GeometryShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppgeometryshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn IAGetPrimitiveTopology(&self) -> D3D11_PRIMITIVE_TOPOLOGY {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn VSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn GetPredication(
        &self,
        pppredicate: *mut Option<ID3D11Predicate>,
        ppredicatevalue: Option<*mut windows_core::BOOL>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GetPredication)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pppredicate),
                ppredicatevalue.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn GSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn GSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn OMGetRenderTargets(
        &self,
        numviews: u32,
        pprendertargetviews: Option<*mut Option<ID3D11RenderTargetView>>,
        ppdepthstencilview: *mut Option<ID3D11DepthStencilView>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargets)(
                windows_core::Interface::as_raw(self),
                numviews,
                pprendertargetviews.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(ppdepthstencilview),
            );
        }
    }
    pub(crate) unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(
        &self,
        numrtvs: u32,
        pprendertargetviews: Option<*mut Option<ID3D11RenderTargetView>>,
        ppdepthstencilview: *mut Option<ID3D11DepthStencilView>,
        uavstartslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*mut Option<ID3D11UnorderedAccessView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargetsAndUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                numrtvs,
                pprendertargetviews.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(ppdepthstencilview),
                uavstartslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn OMGetBlendState(
        &self,
        ppblendstate: *mut Option<ID3D11BlendState>,
        blendfactor: Option<*mut f32>,
        psamplemask: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetBlendState)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppblendstate),
                blendfactor.unwrap_or(core::mem::zeroed()) as _,
                psamplemask.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn OMGetDepthStencilState(
        &self,
        ppdepthstencilstate: *mut Option<ID3D11DepthStencilState>,
        pstencilref: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetDepthStencilState)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppdepthstencilstate),
                pstencilref.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn SOGetTargets(
        &self,
        numbuffers: u32,
        ppsotargets: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).SOGetTargets)(
                windows_core::Interface::as_raw(self),
                numbuffers,
                ppsotargets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn RSGetState(&self) -> windows_core::Result<ID3D11RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSGetState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub(crate) unsafe fn RSGetViewports(
        &self,
        pnumviewports: *mut u32,
        pviewports: Option<*mut D3D11_VIEWPORT>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetViewports)(
                windows_core::Interface::as_raw(self),
                pnumviewports as _,
                pviewports.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn RSGetScissorRects(
        &self,
        pnumrects: *mut u32,
        prects: Option<*mut D3D11_RECT>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetScissorRects)(
                windows_core::Interface::as_raw(self),
                pnumrects as _,
                prects.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn HSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn HSGetShader(
        &self,
        pphullshader: *mut Option<ID3D11HullShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pphullshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn HSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn HSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn DSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn DSGetShader(
        &self,
        ppdomainshader: *mut Option<ID3D11DomainShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppdomainshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn DSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn DSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CSGetUnorderedAccessViews(
        &self,
        startslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*mut Option<ID3D11UnorderedAccessView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                startslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CSGetShader(
        &self,
        ppcomputeshader: *mut Option<ID3D11ComputeShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppcomputeshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn CSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(
                self,
            ));
        }
    }
    pub(crate) unsafe fn Flush(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self));
        }
    }
    pub(crate) unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        unsafe {
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetContextFlags(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetContextFlags)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn FinishCommandList(
        &self,
        restoredeferredcontextstate: bool,
        ppcommandlist: Option<*mut Option<ID3D11CommandList>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FinishCommandList)(
                windows_core::Interface::as_raw(self),
                restoredeferredcontextstate.into(),
                ppcommandlist.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
}
#[repr(C)]
pub struct ID3D11DeviceContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub VSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub PSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub Map: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        D3D11_MAP,
        u32,
        *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub PSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub IASetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IASetVertexBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
        *const u32,
    ),
    pub IASetIndexBuffer:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DXGI_FORMAT, u32),
    pub DrawIndexedInstanced:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    pub GSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub IASetPrimitiveTopology:
        unsafe extern "system" fn(*mut core::ffi::c_void, D3D11_PRIMITIVE_TOPOLOGY),
    pub VSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
    ) -> windows_core::HRESULT,
    pub SetPredication: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ),
    pub GSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
    ),
    pub OMSetBlendState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, u32),
    pub OMSetDepthStencilState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub SOSetTargets: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
    ),
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DrawIndexedInstancedIndirect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub DrawInstancedIndirect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub DispatchIndirect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub RSSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RSSetViewports:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D11_VIEWPORT),
    pub RSSetScissorRects:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D11_RECT),
    pub CopySubresourceRegion: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        u32,
        *mut core::ffi::c_void,
        u32,
        *const D3D11_BOX,
    ),
    pub CopyResource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub UpdateSubresource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const D3D11_BOX,
        *const core::ffi::c_void,
        u32,
        u32,
    ),
    pub CopyStructureCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ),
    pub ClearRenderTargetView:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearUnorderedAccessViewUint:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u32),
    pub ClearUnorderedAccessViewFloat:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearDepthStencilView:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, f32, u8),
    pub GenerateMips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetResourceMinLOD:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32),
    pub GetResourceMinLOD:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> f32,
    pub ResolveSubresource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        u32,
        DXGI_FORMAT,
    ),
    pub ExecuteCommandList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ),
    pub HSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub HSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub DSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetUnorderedAccessViews: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
    ),
    pub CSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub CSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub PSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub PSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub IAGetInputLayout:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ),
    pub IAGetIndexBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut DXGI_FORMAT,
        *mut u32,
    ),
    pub GSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub IAGetPrimitiveTopology:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_PRIMITIVE_TOPOLOGY),
    pub VSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GetPredication: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ),
    pub GSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetRenderTargets: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ),
    pub OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ),
    pub OMGetBlendState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut f32,
        *mut u32,
    ),
    pub OMGetDepthStencilState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub SOGetTargets:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void),
    pub RSGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub RSGetViewports:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D11_VIEWPORT),
    pub RSGetScissorRects:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D11_RECT),
    pub HSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub HSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub DSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetUnorderedAccessViews:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub CSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub FinishCommandList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ID3D11DeviceContext_Impl: ID3D11DeviceChild_Impl {
    fn VSSetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *const Option<ID3D11Buffer>,
    );
    fn PSSetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>,
    );
    fn PSSetShader(
        &self,
        ppixelshader: windows_core::Ref<ID3D11PixelShader>,
        ppclassinstances: *const Option<ID3D11ClassInstance>,
        numclassinstances: u32,
    );
    fn PSSetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *const Option<ID3D11SamplerState>,
    );
    fn VSSetShader(
        &self,
        pvertexshader: windows_core::Ref<ID3D11VertexShader>,
        ppclassinstances: *const Option<ID3D11ClassInstance>,
        numclassinstances: u32,
    );
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn Map(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        subresource: u32,
        maptype: D3D11_MAP,
        mapflags: u32,
        pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> windows_core::Result<()>;
    fn Unmap(&self, presource: windows_core::Ref<ID3D11Resource>, subresource: u32);
    fn PSSetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *const Option<ID3D11Buffer>,
    );
    fn IASetInputLayout(&self, pinputlayout: windows_core::Ref<ID3D11InputLayout>);
    fn IASetVertexBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppvertexbuffers: *const Option<ID3D11Buffer>,
        pstrides: *const u32,
        poffsets: *const u32,
    );
    fn IASetIndexBuffer(
        &self,
        pindexbuffer: windows_core::Ref<ID3D11Buffer>,
        format: DXGI_FORMAT,
        offset: u32,
    );
    fn DrawIndexedInstanced(
        &self,
        indexcountperinstance: u32,
        instancecount: u32,
        startindexlocation: u32,
        basevertexlocation: i32,
        startinstancelocation: u32,
    );
    fn DrawInstanced(
        &self,
        vertexcountperinstance: u32,
        instancecount: u32,
        startvertexlocation: u32,
        startinstancelocation: u32,
    );
    fn GSSetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *const Option<ID3D11Buffer>,
    );
    fn GSSetShader(
        &self,
        pshader: windows_core::Ref<ID3D11GeometryShader>,
        ppclassinstances: *const Option<ID3D11ClassInstance>,
        numclassinstances: u32,
    );
    fn IASetPrimitiveTopology(&self, topology: D3D11_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>,
    );
    fn VSSetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *const Option<ID3D11SamplerState>,
    );
    fn Begin(&self, pasync: windows_core::Ref<ID3D11Asynchronous>);
    fn End(&self, pasync: windows_core::Ref<ID3D11Asynchronous>);
    fn GetData(
        &self,
        pasync: windows_core::Ref<ID3D11Asynchronous>,
        pdata: *mut core::ffi::c_void,
        datasize: u32,
        getdataflags: u32,
    ) -> windows_core::Result<()>;
    fn SetPredication(
        &self,
        ppredicate: windows_core::Ref<ID3D11Predicate>,
        predicatevalue: windows_core::BOOL,
    );
    fn GSSetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>,
    );
    fn GSSetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *const Option<ID3D11SamplerState>,
    );
    fn OMSetRenderTargets(
        &self,
        numviews: u32,
        pprendertargetviews: *const Option<ID3D11RenderTargetView>,
        pdepthstencilview: windows_core::Ref<ID3D11DepthStencilView>,
    );
    fn OMSetRenderTargetsAndUnorderedAccessViews(
        &self,
        numrtvs: u32,
        pprendertargetviews: *const Option<ID3D11RenderTargetView>,
        pdepthstencilview: windows_core::Ref<ID3D11DepthStencilView>,
        uavstartslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: *const Option<ID3D11UnorderedAccessView>,
        puavinitialcounts: *const u32,
    );
    fn OMSetBlendState(
        &self,
        pblendstate: windows_core::Ref<ID3D11BlendState>,
        blendfactor: *const f32,
        samplemask: u32,
    );
    fn OMSetDepthStencilState(
        &self,
        pdepthstencilstate: windows_core::Ref<ID3D11DepthStencilState>,
        stencilref: u32,
    );
    fn SOSetTargets(
        &self,
        numbuffers: u32,
        ppsotargets: *const Option<ID3D11Buffer>,
        poffsets: *const u32,
    );
    fn DrawAuto(&self);
    fn DrawIndexedInstancedIndirect(
        &self,
        pbufferforargs: windows_core::Ref<ID3D11Buffer>,
        alignedbyteoffsetforargs: u32,
    );
    fn DrawInstancedIndirect(
        &self,
        pbufferforargs: windows_core::Ref<ID3D11Buffer>,
        alignedbyteoffsetforargs: u32,
    );
    fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn DispatchIndirect(
        &self,
        pbufferforargs: windows_core::Ref<ID3D11Buffer>,
        alignedbyteoffsetforargs: u32,
    );
    fn RSSetState(&self, prasterizerstate: windows_core::Ref<ID3D11RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D11_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const D3D11_RECT);
    fn CopySubresourceRegion(
        &self,
        pdstresource: windows_core::Ref<ID3D11Resource>,
        dstsubresource: u32,
        dstx: u32,
        dsty: u32,
        dstz: u32,
        psrcresource: windows_core::Ref<ID3D11Resource>,
        srcsubresource: u32,
        psrcbox: *const D3D11_BOX,
    );
    fn CopyResource(
        &self,
        pdstresource: windows_core::Ref<ID3D11Resource>,
        psrcresource: windows_core::Ref<ID3D11Resource>,
    );
    fn UpdateSubresource(
        &self,
        pdstresource: windows_core::Ref<ID3D11Resource>,
        dstsubresource: u32,
        pdstbox: *const D3D11_BOX,
        psrcdata: *const core::ffi::c_void,
        srcrowpitch: u32,
        srcdepthpitch: u32,
    );
    fn CopyStructureCount(
        &self,
        pdstbuffer: windows_core::Ref<ID3D11Buffer>,
        dstalignedbyteoffset: u32,
        psrcview: windows_core::Ref<ID3D11UnorderedAccessView>,
    );
    fn ClearRenderTargetView(
        &self,
        prendertargetview: windows_core::Ref<ID3D11RenderTargetView>,
        colorrgba: *const f32,
    );
    fn ClearUnorderedAccessViewUint(
        &self,
        punorderedaccessview: windows_core::Ref<ID3D11UnorderedAccessView>,
        values: *const u32,
    );
    fn ClearUnorderedAccessViewFloat(
        &self,
        punorderedaccessview: windows_core::Ref<ID3D11UnorderedAccessView>,
        values: *const f32,
    );
    fn ClearDepthStencilView(
        &self,
        pdepthstencilview: windows_core::Ref<ID3D11DepthStencilView>,
        clearflags: u32,
        depth: f32,
        stencil: u8,
    );
    fn GenerateMips(&self, pshaderresourceview: windows_core::Ref<ID3D11ShaderResourceView>);
    fn SetResourceMinLOD(&self, presource: windows_core::Ref<ID3D11Resource>, minlod: f32);
    fn GetResourceMinLOD(&self, presource: windows_core::Ref<ID3D11Resource>) -> f32;
    fn ResolveSubresource(
        &self,
        pdstresource: windows_core::Ref<ID3D11Resource>,
        dstsubresource: u32,
        psrcresource: windows_core::Ref<ID3D11Resource>,
        srcsubresource: u32,
        format: DXGI_FORMAT,
    );
    fn ExecuteCommandList(
        &self,
        pcommandlist: windows_core::Ref<ID3D11CommandList>,
        restorecontextstate: windows_core::BOOL,
    );
    fn HSSetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>,
    );
    fn HSSetShader(
        &self,
        phullshader: windows_core::Ref<ID3D11HullShader>,
        ppclassinstances: *const Option<ID3D11ClassInstance>,
        numclassinstances: u32,
    );
    fn HSSetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *const Option<ID3D11SamplerState>,
    );
    fn HSSetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *const Option<ID3D11Buffer>,
    );
    fn DSSetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>,
    );
    fn DSSetShader(
        &self,
        pdomainshader: windows_core::Ref<ID3D11DomainShader>,
        ppclassinstances: *const Option<ID3D11ClassInstance>,
        numclassinstances: u32,
    );
    fn DSSetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *const Option<ID3D11SamplerState>,
    );
    fn DSSetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *const Option<ID3D11Buffer>,
    );
    fn CSSetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>,
    );
    fn CSSetUnorderedAccessViews(
        &self,
        startslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: *const Option<ID3D11UnorderedAccessView>,
        puavinitialcounts: *const u32,
    );
    fn CSSetShader(
        &self,
        pcomputeshader: windows_core::Ref<ID3D11ComputeShader>,
        ppclassinstances: *const Option<ID3D11ClassInstance>,
        numclassinstances: u32,
    );
    fn CSSetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *const Option<ID3D11SamplerState>,
    );
    fn CSSetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *const Option<ID3D11Buffer>,
    );
    fn VSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *mut Option<ID3D11Buffer>,
    );
    fn PSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>,
    );
    fn PSGetShader(
        &self,
        pppixelshader: windows_core::OutRef<ID3D11PixelShader>,
        ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>,
        pnumclassinstances: *mut u32,
    );
    fn PSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *mut Option<ID3D11SamplerState>,
    );
    fn VSGetShader(
        &self,
        ppvertexshader: windows_core::OutRef<ID3D11VertexShader>,
        ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>,
        pnumclassinstances: *mut u32,
    );
    fn PSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *mut Option<ID3D11Buffer>,
    );
    fn IAGetInputLayout(&self, ppinputlayout: windows_core::OutRef<ID3D11InputLayout>);
    fn IAGetVertexBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppvertexbuffers: *mut Option<ID3D11Buffer>,
        pstrides: *mut u32,
        poffsets: *mut u32,
    );
    fn IAGetIndexBuffer(
        &self,
        pindexbuffer: windows_core::OutRef<ID3D11Buffer>,
        format: *mut DXGI_FORMAT,
        offset: *mut u32,
    );
    fn GSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *mut Option<ID3D11Buffer>,
    );
    fn GSGetShader(
        &self,
        ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>,
        ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>,
        pnumclassinstances: *mut u32,
    );
    fn IAGetPrimitiveTopology(&self, ptopology: *mut D3D11_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>,
    );
    fn VSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *mut Option<ID3D11SamplerState>,
    );
    fn GetPredication(
        &self,
        pppredicate: windows_core::OutRef<ID3D11Predicate>,
        ppredicatevalue: *mut windows_core::BOOL,
    );
    fn GSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>,
    );
    fn GSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *mut Option<ID3D11SamplerState>,
    );
    fn OMGetRenderTargets(
        &self,
        numviews: u32,
        pprendertargetviews: *mut Option<ID3D11RenderTargetView>,
        ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>,
    );
    fn OMGetRenderTargetsAndUnorderedAccessViews(
        &self,
        numrtvs: u32,
        pprendertargetviews: *mut Option<ID3D11RenderTargetView>,
        ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>,
        uavstartslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: *mut Option<ID3D11UnorderedAccessView>,
    );
    fn OMGetBlendState(
        &self,
        ppblendstate: windows_core::OutRef<ID3D11BlendState>,
        blendfactor: *mut f32,
        psamplemask: *mut u32,
    );
    fn OMGetDepthStencilState(
        &self,
        ppdepthstencilstate: windows_core::OutRef<ID3D11DepthStencilState>,
        pstencilref: *mut u32,
    );
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut Option<ID3D11Buffer>);
    fn RSGetState(&self, pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState>);
    fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT);
    fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: *mut D3D11_RECT);
    fn HSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>,
    );
    fn HSGetShader(
        &self,
        pphullshader: windows_core::OutRef<ID3D11HullShader>,
        ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>,
        pnumclassinstances: *mut u32,
    );
    fn HSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *mut Option<ID3D11SamplerState>,
    );
    fn HSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *mut Option<ID3D11Buffer>,
    );
    fn DSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>,
    );
    fn DSGetShader(
        &self,
        ppdomainshader: windows_core::OutRef<ID3D11DomainShader>,
        ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>,
        pnumclassinstances: *mut u32,
    );
    fn DSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *mut Option<ID3D11SamplerState>,
    );
    fn DSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *mut Option<ID3D11Buffer>,
    );
    fn CSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>,
    );
    fn CSGetUnorderedAccessViews(
        &self,
        startslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: *mut Option<ID3D11UnorderedAccessView>,
    );
    fn CSGetShader(
        &self,
        ppcomputeshader: windows_core::OutRef<ID3D11ComputeShader>,
        ppclassinstances: windows_core::OutRef<ID3D11ClassInstance>,
        pnumclassinstances: *mut u32,
    );
    fn CSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: *mut Option<ID3D11SamplerState>,
    );
    fn CSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: *mut Option<ID3D11Buffer>,
    );
    fn ClearState(&self);
    fn Flush(&self);
    fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE;
    fn GetContextFlags(&self) -> u32;
    fn FinishCommandList(
        &self,
        restoredeferredcontextstate: windows_core::BOOL,
        ppcommandlist: windows_core::OutRef<ID3D11CommandList>,
    ) -> windows_core::Result<()>;
}
impl ID3D11DeviceContext_Vtbl {
    pub const fn new<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VSSetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn PSSetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn PSSetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppixelshader: *mut core::ffi::c_void,
            ppclassinstances: *const *mut core::ffi::c_void,
            numclassinstances: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetShader(
                    this,
                    core::mem::transmute_copy(&ppixelshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&numclassinstances),
                );
            }
        }
        unsafe extern "system" fn PSSetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn VSSetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvertexshader: *mut core::ffi::c_void,
            ppclassinstances: *const *mut core::ffi::c_void,
            numclassinstances: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetShader(
                    this,
                    core::mem::transmute_copy(&pvertexshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&numclassinstances),
                );
            }
        }
        unsafe extern "system" fn DrawIndexed<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            indexcount: u32,
            startindexlocation: u32,
            basevertexlocation: i32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawIndexed(
                    this,
                    core::mem::transmute_copy(&indexcount),
                    core::mem::transmute_copy(&startindexlocation),
                    core::mem::transmute_copy(&basevertexlocation),
                );
            }
        }
        unsafe extern "system" fn Draw<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            vertexcount: u32,
            startvertexlocation: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Draw(
                    this,
                    core::mem::transmute_copy(&vertexcount),
                    core::mem::transmute_copy(&startvertexlocation),
                );
            }
        }
        unsafe extern "system" fn Map<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            subresource: u32,
            maptype: D3D11_MAP,
            mapflags: u32,
            pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Map(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&subresource),
                    core::mem::transmute_copy(&maptype),
                    core::mem::transmute_copy(&mapflags),
                    core::mem::transmute_copy(&pmappedresource),
                )
                .into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            subresource: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Unmap(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&subresource),
                );
            }
        }
        unsafe extern "system" fn PSSetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSSetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn IASetInputLayout<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pinputlayout: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetInputLayout(
                    this,
                    core::mem::transmute_copy(&pinputlayout),
                );
            }
        }
        unsafe extern "system" fn IASetVertexBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppvertexbuffers: *const *mut core::ffi::c_void,
            pstrides: *const u32,
            poffsets: *const u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetVertexBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppvertexbuffers),
                    core::mem::transmute_copy(&pstrides),
                    core::mem::transmute_copy(&poffsets),
                );
            }
        }
        unsafe extern "system" fn IASetIndexBuffer<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pindexbuffer: *mut core::ffi::c_void,
            format: DXGI_FORMAT,
            offset: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetIndexBuffer(
                    this,
                    core::mem::transmute_copy(&pindexbuffer),
                    core::mem::transmute_copy(&format),
                    core::mem::transmute_copy(&offset),
                );
            }
        }
        unsafe extern "system" fn DrawIndexedInstanced<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            indexcountperinstance: u32,
            instancecount: u32,
            startindexlocation: u32,
            basevertexlocation: i32,
            startinstancelocation: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawIndexedInstanced(
                    this,
                    core::mem::transmute_copy(&indexcountperinstance),
                    core::mem::transmute_copy(&instancecount),
                    core::mem::transmute_copy(&startindexlocation),
                    core::mem::transmute_copy(&basevertexlocation),
                    core::mem::transmute_copy(&startinstancelocation),
                );
            }
        }
        unsafe extern "system" fn DrawInstanced<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vertexcountperinstance: u32,
            instancecount: u32,
            startvertexlocation: u32,
            startinstancelocation: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawInstanced(
                    this,
                    core::mem::transmute_copy(&vertexcountperinstance),
                    core::mem::transmute_copy(&instancecount),
                    core::mem::transmute_copy(&startvertexlocation),
                    core::mem::transmute_copy(&startinstancelocation),
                );
            }
        }
        unsafe extern "system" fn GSSetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn GSSetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshader: *mut core::ffi::c_void,
            ppclassinstances: *const *mut core::ffi::c_void,
            numclassinstances: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetShader(
                    this,
                    core::mem::transmute_copy(&pshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&numclassinstances),
                );
            }
        }
        unsafe extern "system" fn IASetPrimitiveTopology<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            topology: D3D11_PRIMITIVE_TOPOLOGY,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IASetPrimitiveTopology(
                    this,
                    core::mem::transmute_copy(&topology),
                );
            }
        }
        unsafe extern "system" fn VSSetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn VSSetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSSetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn Begin<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pasync: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Begin(this, core::mem::transmute_copy(&pasync));
            }
        }
        unsafe extern "system" fn End<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pasync: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::End(this, core::mem::transmute_copy(&pasync));
            }
        }
        unsafe extern "system" fn GetData<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pasync: *mut core::ffi::c_void,
            pdata: *mut core::ffi::c_void,
            datasize: u32,
            getdataflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetData(
                    this,
                    core::mem::transmute_copy(&pasync),
                    core::mem::transmute_copy(&pdata),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&getdataflags),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPredication<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppredicate: *mut core::ffi::c_void,
            predicatevalue: windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SetPredication(
                    this,
                    core::mem::transmute_copy(&ppredicate),
                    core::mem::transmute_copy(&predicatevalue),
                );
            }
        }
        unsafe extern "system" fn GSSetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn GSSetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSSetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn OMSetRenderTargets<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numviews: u32,
            pprendertargetviews: *const *mut core::ffi::c_void,
            pdepthstencilview: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetRenderTargets(
                    this,
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&pprendertargetviews),
                    core::mem::transmute_copy(&pdepthstencilview),
                );
            }
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numrtvs: u32,
            pprendertargetviews: *const *mut core::ffi::c_void,
            pdepthstencilview: *mut core::ffi::c_void,
            uavstartslot: u32,
            numuavs: u32,
            ppunorderedaccessviews: *const *mut core::ffi::c_void,
            puavinitialcounts: *const u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetRenderTargetsAndUnorderedAccessViews(
                    this,
                    core::mem::transmute_copy(&numrtvs),
                    core::mem::transmute_copy(&pprendertargetviews),
                    core::mem::transmute_copy(&pdepthstencilview),
                    core::mem::transmute_copy(&uavstartslot),
                    core::mem::transmute_copy(&numuavs),
                    core::mem::transmute_copy(&ppunorderedaccessviews),
                    core::mem::transmute_copy(&puavinitialcounts),
                );
            }
        }
        unsafe extern "system" fn OMSetBlendState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pblendstate: *mut core::ffi::c_void,
            blendfactor: *const f32,
            samplemask: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetBlendState(
                    this,
                    core::mem::transmute_copy(&pblendstate),
                    core::mem::transmute_copy(&blendfactor),
                    core::mem::transmute_copy(&samplemask),
                );
            }
        }
        unsafe extern "system" fn OMSetDepthStencilState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdepthstencilstate: *mut core::ffi::c_void,
            stencilref: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMSetDepthStencilState(
                    this,
                    core::mem::transmute_copy(&pdepthstencilstate),
                    core::mem::transmute_copy(&stencilref),
                );
            }
        }
        unsafe extern "system" fn SOSetTargets<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numbuffers: u32,
            ppsotargets: *const *mut core::ffi::c_void,
            poffsets: *const u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SOSetTargets(
                    this,
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppsotargets),
                    core::mem::transmute_copy(&poffsets),
                );
            }
        }
        unsafe extern "system" fn DrawAuto<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawAuto(this);
            }
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbufferforargs: *mut core::ffi::c_void,
            alignedbyteoffsetforargs: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawIndexedInstancedIndirect(
                    this,
                    core::mem::transmute_copy(&pbufferforargs),
                    core::mem::transmute_copy(&alignedbyteoffsetforargs),
                );
            }
        }
        unsafe extern "system" fn DrawInstancedIndirect<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbufferforargs: *mut core::ffi::c_void,
            alignedbyteoffsetforargs: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DrawInstancedIndirect(
                    this,
                    core::mem::transmute_copy(&pbufferforargs),
                    core::mem::transmute_copy(&alignedbyteoffsetforargs),
                );
            }
        }
        unsafe extern "system" fn Dispatch<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            threadgroupcountx: u32,
            threadgroupcounty: u32,
            threadgroupcountz: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Dispatch(
                    this,
                    core::mem::transmute_copy(&threadgroupcountx),
                    core::mem::transmute_copy(&threadgroupcounty),
                    core::mem::transmute_copy(&threadgroupcountz),
                );
            }
        }
        unsafe extern "system" fn DispatchIndirect<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbufferforargs: *mut core::ffi::c_void,
            alignedbyteoffsetforargs: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DispatchIndirect(
                    this,
                    core::mem::transmute_copy(&pbufferforargs),
                    core::mem::transmute_copy(&alignedbyteoffsetforargs),
                );
            }
        }
        unsafe extern "system" fn RSSetState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            prasterizerstate: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSSetState(
                    this,
                    core::mem::transmute_copy(&prasterizerstate),
                );
            }
        }
        unsafe extern "system" fn RSSetViewports<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numviewports: u32,
            pviewports: *const D3D11_VIEWPORT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSSetViewports(
                    this,
                    core::mem::transmute_copy(&numviewports),
                    core::mem::transmute_copy(&pviewports),
                );
            }
        }
        unsafe extern "system" fn RSSetScissorRects<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numrects: u32,
            prects: *const D3D11_RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSSetScissorRects(
                    this,
                    core::mem::transmute_copy(&numrects),
                    core::mem::transmute_copy(&prects),
                );
            }
        }
        unsafe extern "system" fn CopySubresourceRegion<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdstresource: *mut core::ffi::c_void,
            dstsubresource: u32,
            dstx: u32,
            dsty: u32,
            dstz: u32,
            psrcresource: *mut core::ffi::c_void,
            srcsubresource: u32,
            psrcbox: *const D3D11_BOX,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CopySubresourceRegion(
                    this,
                    core::mem::transmute_copy(&pdstresource),
                    core::mem::transmute_copy(&dstsubresource),
                    core::mem::transmute_copy(&dstx),
                    core::mem::transmute_copy(&dsty),
                    core::mem::transmute_copy(&dstz),
                    core::mem::transmute_copy(&psrcresource),
                    core::mem::transmute_copy(&srcsubresource),
                    core::mem::transmute_copy(&psrcbox),
                );
            }
        }
        unsafe extern "system" fn CopyResource<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdstresource: *mut core::ffi::c_void,
            psrcresource: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CopyResource(
                    this,
                    core::mem::transmute_copy(&pdstresource),
                    core::mem::transmute_copy(&psrcresource),
                );
            }
        }
        unsafe extern "system" fn UpdateSubresource<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdstresource: *mut core::ffi::c_void,
            dstsubresource: u32,
            pdstbox: *const D3D11_BOX,
            psrcdata: *const core::ffi::c_void,
            srcrowpitch: u32,
            srcdepthpitch: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::UpdateSubresource(
                    this,
                    core::mem::transmute_copy(&pdstresource),
                    core::mem::transmute_copy(&dstsubresource),
                    core::mem::transmute_copy(&pdstbox),
                    core::mem::transmute_copy(&psrcdata),
                    core::mem::transmute_copy(&srcrowpitch),
                    core::mem::transmute_copy(&srcdepthpitch),
                );
            }
        }
        unsafe extern "system" fn CopyStructureCount<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdstbuffer: *mut core::ffi::c_void,
            dstalignedbyteoffset: u32,
            psrcview: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CopyStructureCount(
                    this,
                    core::mem::transmute_copy(&pdstbuffer),
                    core::mem::transmute_copy(&dstalignedbyteoffset),
                    core::mem::transmute_copy(&psrcview),
                );
            }
        }
        unsafe extern "system" fn ClearRenderTargetView<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            prendertargetview: *mut core::ffi::c_void,
            colorrgba: *const f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearRenderTargetView(
                    this,
                    core::mem::transmute_copy(&prendertargetview),
                    core::mem::transmute_copy(&colorrgba),
                );
            }
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            punorderedaccessview: *mut core::ffi::c_void,
            values: *const u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearUnorderedAccessViewUint(
                    this,
                    core::mem::transmute_copy(&punorderedaccessview),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            punorderedaccessview: *mut core::ffi::c_void,
            values: *const f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearUnorderedAccessViewFloat(
                    this,
                    core::mem::transmute_copy(&punorderedaccessview),
                    core::mem::transmute_copy(&values),
                );
            }
        }
        unsafe extern "system" fn ClearDepthStencilView<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdepthstencilview: *mut core::ffi::c_void,
            clearflags: u32,
            depth: f32,
            stencil: u8,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearDepthStencilView(
                    this,
                    core::mem::transmute_copy(&pdepthstencilview),
                    core::mem::transmute_copy(&clearflags),
                    core::mem::transmute_copy(&depth),
                    core::mem::transmute_copy(&stencil),
                );
            }
        }
        unsafe extern "system" fn GenerateMips<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderresourceview: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GenerateMips(
                    this,
                    core::mem::transmute_copy(&pshaderresourceview),
                );
            }
        }
        unsafe extern "system" fn SetResourceMinLOD<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            minlod: f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SetResourceMinLOD(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&minlod),
                );
            }
        }
        unsafe extern "system" fn GetResourceMinLOD<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
        ) -> f32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetResourceMinLOD(
                    this,
                    core::mem::transmute_copy(&presource),
                )
            }
        }
        unsafe extern "system" fn ResolveSubresource<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdstresource: *mut core::ffi::c_void,
            dstsubresource: u32,
            psrcresource: *mut core::ffi::c_void,
            srcsubresource: u32,
            format: DXGI_FORMAT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ResolveSubresource(
                    this,
                    core::mem::transmute_copy(&pdstresource),
                    core::mem::transmute_copy(&dstsubresource),
                    core::mem::transmute_copy(&psrcresource),
                    core::mem::transmute_copy(&srcsubresource),
                    core::mem::transmute_copy(&format),
                );
            }
        }
        unsafe extern "system" fn ExecuteCommandList<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcommandlist: *mut core::ffi::c_void,
            restorecontextstate: windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ExecuteCommandList(
                    this,
                    core::mem::transmute_copy(&pcommandlist),
                    core::mem::transmute_copy(&restorecontextstate),
                );
            }
        }
        unsafe extern "system" fn HSSetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn HSSetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            phullshader: *mut core::ffi::c_void,
            ppclassinstances: *const *mut core::ffi::c_void,
            numclassinstances: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetShader(
                    this,
                    core::mem::transmute_copy(&phullshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&numclassinstances),
                );
            }
        }
        unsafe extern "system" fn HSSetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn HSSetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSSetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn DSSetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn DSSetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdomainshader: *mut core::ffi::c_void,
            ppclassinstances: *const *mut core::ffi::c_void,
            numclassinstances: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetShader(
                    this,
                    core::mem::transmute_copy(&pdomainshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&numclassinstances),
                );
            }
        }
        unsafe extern "system" fn DSSetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn DSSetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSSetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn CSSetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numuavs: u32,
            ppunorderedaccessviews: *const *mut core::ffi::c_void,
            puavinitialcounts: *const u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetUnorderedAccessViews(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numuavs),
                    core::mem::transmute_copy(&ppunorderedaccessviews),
                    core::mem::transmute_copy(&puavinitialcounts),
                );
            }
        }
        unsafe extern "system" fn CSSetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcomputeshader: *mut core::ffi::c_void,
            ppclassinstances: *const *mut core::ffi::c_void,
            numclassinstances: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetShader(
                    this,
                    core::mem::transmute_copy(&pcomputeshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&numclassinstances),
                );
            }
        }
        unsafe extern "system" fn CSSetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn CSSetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *const *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSSetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn VSGetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn PSGetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn PSGetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pppixelshader: *mut *mut core::ffi::c_void,
            ppclassinstances: *mut *mut core::ffi::c_void,
            pnumclassinstances: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetShader(
                    this,
                    core::mem::transmute_copy(&pppixelshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&pnumclassinstances),
                );
            }
        }
        unsafe extern "system" fn PSGetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn VSGetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppvertexshader: *mut *mut core::ffi::c_void,
            ppclassinstances: *mut *mut core::ffi::c_void,
            pnumclassinstances: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetShader(
                    this,
                    core::mem::transmute_copy(&ppvertexshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&pnumclassinstances),
                );
            }
        }
        unsafe extern "system" fn PSGetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::PSGetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn IAGetInputLayout<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppinputlayout: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetInputLayout(
                    this,
                    core::mem::transmute_copy(&ppinputlayout),
                );
            }
        }
        unsafe extern "system" fn IAGetVertexBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppvertexbuffers: *mut *mut core::ffi::c_void,
            pstrides: *mut u32,
            poffsets: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetVertexBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppvertexbuffers),
                    core::mem::transmute_copy(&pstrides),
                    core::mem::transmute_copy(&poffsets),
                );
            }
        }
        unsafe extern "system" fn IAGetIndexBuffer<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pindexbuffer: *mut *mut core::ffi::c_void,
            format: *mut DXGI_FORMAT,
            offset: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetIndexBuffer(
                    this,
                    core::mem::transmute_copy(&pindexbuffer),
                    core::mem::transmute_copy(&format),
                    core::mem::transmute_copy(&offset),
                );
            }
        }
        unsafe extern "system" fn GSGetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn GSGetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppgeometryshader: *mut *mut core::ffi::c_void,
            ppclassinstances: *mut *mut core::ffi::c_void,
            pnumclassinstances: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetShader(
                    this,
                    core::mem::transmute_copy(&ppgeometryshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&pnumclassinstances),
                );
            }
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ptopology: *mut D3D11_PRIMITIVE_TOPOLOGY,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::IAGetPrimitiveTopology(
                    this,
                    core::mem::transmute_copy(&ptopology),
                );
            }
        }
        unsafe extern "system" fn VSGetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn VSGetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::VSGetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn GetPredication<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pppredicate: *mut *mut core::ffi::c_void,
            ppredicatevalue: *mut windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetPredication(
                    this,
                    core::mem::transmute_copy(&pppredicate),
                    core::mem::transmute_copy(&ppredicatevalue),
                );
            }
        }
        unsafe extern "system" fn GSGetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn GSGetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GSGetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn OMGetRenderTargets<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numviews: u32,
            pprendertargetviews: *mut *mut core::ffi::c_void,
            ppdepthstencilview: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetRenderTargets(
                    this,
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&pprendertargetviews),
                    core::mem::transmute_copy(&ppdepthstencilview),
                );
            }
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numrtvs: u32,
            pprendertargetviews: *mut *mut core::ffi::c_void,
            ppdepthstencilview: *mut *mut core::ffi::c_void,
            uavstartslot: u32,
            numuavs: u32,
            ppunorderedaccessviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetRenderTargetsAndUnorderedAccessViews(
                    this,
                    core::mem::transmute_copy(&numrtvs),
                    core::mem::transmute_copy(&pprendertargetviews),
                    core::mem::transmute_copy(&ppdepthstencilview),
                    core::mem::transmute_copy(&uavstartslot),
                    core::mem::transmute_copy(&numuavs),
                    core::mem::transmute_copy(&ppunorderedaccessviews),
                );
            }
        }
        unsafe extern "system" fn OMGetBlendState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppblendstate: *mut *mut core::ffi::c_void,
            blendfactor: *mut f32,
            psamplemask: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetBlendState(
                    this,
                    core::mem::transmute_copy(&ppblendstate),
                    core::mem::transmute_copy(&blendfactor),
                    core::mem::transmute_copy(&psamplemask),
                );
            }
        }
        unsafe extern "system" fn OMGetDepthStencilState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppdepthstencilstate: *mut *mut core::ffi::c_void,
            pstencilref: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::OMGetDepthStencilState(
                    this,
                    core::mem::transmute_copy(&ppdepthstencilstate),
                    core::mem::transmute_copy(&pstencilref),
                );
            }
        }
        unsafe extern "system" fn SOGetTargets<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            numbuffers: u32,
            ppsotargets: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::SOGetTargets(
                    this,
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppsotargets),
                );
            }
        }
        unsafe extern "system" fn RSGetState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pprasterizerstate: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSGetState(
                    this,
                    core::mem::transmute_copy(&pprasterizerstate),
                );
            }
        }
        unsafe extern "system" fn RSGetViewports<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pnumviewports: *mut u32,
            pviewports: *mut D3D11_VIEWPORT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSGetViewports(
                    this,
                    core::mem::transmute_copy(&pnumviewports),
                    core::mem::transmute_copy(&pviewports),
                );
            }
        }
        unsafe extern "system" fn RSGetScissorRects<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pnumrects: *mut u32,
            prects: *mut D3D11_RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::RSGetScissorRects(
                    this,
                    core::mem::transmute_copy(&pnumrects),
                    core::mem::transmute_copy(&prects),
                );
            }
        }
        unsafe extern "system" fn HSGetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn HSGetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pphullshader: *mut *mut core::ffi::c_void,
            ppclassinstances: *mut *mut core::ffi::c_void,
            pnumclassinstances: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetShader(
                    this,
                    core::mem::transmute_copy(&pphullshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&pnumclassinstances),
                );
            }
        }
        unsafe extern "system" fn HSGetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn HSGetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::HSGetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn DSGetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn DSGetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppdomainshader: *mut *mut core::ffi::c_void,
            ppclassinstances: *mut *mut core::ffi::c_void,
            pnumclassinstances: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetShader(
                    this,
                    core::mem::transmute_copy(&ppdomainshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&pnumclassinstances),
                );
            }
        }
        unsafe extern "system" fn DSGetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn DSGetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::DSGetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn CSGetShaderResources<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numviews: u32,
            ppshaderresourceviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetShaderResources(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numviews),
                    core::mem::transmute_copy(&ppshaderresourceviews),
                );
            }
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numuavs: u32,
            ppunorderedaccessviews: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetUnorderedAccessViews(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numuavs),
                    core::mem::transmute_copy(&ppunorderedaccessviews),
                );
            }
        }
        unsafe extern "system" fn CSGetShader<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppcomputeshader: *mut *mut core::ffi::c_void,
            ppclassinstances: *mut *mut core::ffi::c_void,
            pnumclassinstances: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetShader(
                    this,
                    core::mem::transmute_copy(&ppcomputeshader),
                    core::mem::transmute_copy(&ppclassinstances),
                    core::mem::transmute_copy(&pnumclassinstances),
                );
            }
        }
        unsafe extern "system" fn CSGetSamplers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numsamplers: u32,
            ppsamplers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetSamplers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numsamplers),
                    core::mem::transmute_copy(&ppsamplers),
                );
            }
        }
        unsafe extern "system" fn CSGetConstantBuffers<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startslot: u32,
            numbuffers: u32,
            ppconstantbuffers: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::CSGetConstantBuffers(
                    this,
                    core::mem::transmute_copy(&startslot),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&ppconstantbuffers),
                );
            }
        }
        unsafe extern "system" fn ClearState<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::ClearState(this);
            }
        }
        unsafe extern "system" fn Flush<Identity: ID3D11DeviceContext_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::Flush(this);
            }
        }
        unsafe extern "system" fn GetType<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> D3D11_DEVICE_CONTEXT_TYPE {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetContextFlags<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::GetContextFlags(this)
            }
        }
        unsafe extern "system" fn FinishCommandList<
            Identity: ID3D11DeviceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            restoredeferredcontextstate: windows_core::BOOL,
            ppcommandlist: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext_Impl::FinishCommandList(
                    this,
                    core::mem::transmute_copy(&restoredeferredcontextstate),
                    core::mem::transmute_copy(&ppcommandlist),
                )
                .into()
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, OFFSET>,
            PSSetShader: PSSetShader::<Identity, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, OFFSET>,
            VSSetShader: VSSetShader::<Identity, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, OFFSET>,
            GSSetShader: GSSetShader::<Identity, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, OFFSET>,
            Begin: Begin::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews::<
                Identity,
                OFFSET,
            >,
            OMSetBlendState: OMSetBlendState::<Identity, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, OFFSET>,
            DrawAuto: DrawAuto::<Identity, OFFSET>,
            DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect::<Identity, OFFSET>,
            DrawInstancedIndirect: DrawInstancedIndirect::<Identity, OFFSET>,
            Dispatch: Dispatch::<Identity, OFFSET>,
            DispatchIndirect: DispatchIndirect::<Identity, OFFSET>,
            RSSetState: RSSetState::<Identity, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, OFFSET>,
            CopyResource: CopyResource::<Identity, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, OFFSET>,
            CopyStructureCount: CopyStructureCount::<Identity, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, OFFSET>,
            GenerateMips: GenerateMips::<Identity, OFFSET>,
            SetResourceMinLOD: SetResourceMinLOD::<Identity, OFFSET>,
            GetResourceMinLOD: GetResourceMinLOD::<Identity, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, OFFSET>,
            ExecuteCommandList: ExecuteCommandList::<Identity, OFFSET>,
            HSSetShaderResources: HSSetShaderResources::<Identity, OFFSET>,
            HSSetShader: HSSetShader::<Identity, OFFSET>,
            HSSetSamplers: HSSetSamplers::<Identity, OFFSET>,
            HSSetConstantBuffers: HSSetConstantBuffers::<Identity, OFFSET>,
            DSSetShaderResources: DSSetShaderResources::<Identity, OFFSET>,
            DSSetShader: DSSetShader::<Identity, OFFSET>,
            DSSetSamplers: DSSetSamplers::<Identity, OFFSET>,
            DSSetConstantBuffers: DSSetConstantBuffers::<Identity, OFFSET>,
            CSSetShaderResources: CSSetShaderResources::<Identity, OFFSET>,
            CSSetUnorderedAccessViews: CSSetUnorderedAccessViews::<Identity, OFFSET>,
            CSSetShader: CSSetShader::<Identity, OFFSET>,
            CSSetSamplers: CSSetSamplers::<Identity, OFFSET>,
            CSSetConstantBuffers: CSSetConstantBuffers::<Identity, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, OFFSET>,
            PSGetShader: PSGetShader::<Identity, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, OFFSET>,
            VSGetShader: VSGetShader::<Identity, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, OFFSET>,
            GSGetShader: GSGetShader::<Identity, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, OFFSET>,
            GetPredication: GetPredication::<Identity, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews::<
                Identity,
                OFFSET,
            >,
            OMGetBlendState: OMGetBlendState::<Identity, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, OFFSET>,
            RSGetState: RSGetState::<Identity, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, OFFSET>,
            HSGetShaderResources: HSGetShaderResources::<Identity, OFFSET>,
            HSGetShader: HSGetShader::<Identity, OFFSET>,
            HSGetSamplers: HSGetSamplers::<Identity, OFFSET>,
            HSGetConstantBuffers: HSGetConstantBuffers::<Identity, OFFSET>,
            DSGetShaderResources: DSGetShaderResources::<Identity, OFFSET>,
            DSGetShader: DSGetShader::<Identity, OFFSET>,
            DSGetSamplers: DSGetSamplers::<Identity, OFFSET>,
            DSGetConstantBuffers: DSGetConstantBuffers::<Identity, OFFSET>,
            CSGetShaderResources: CSGetShaderResources::<Identity, OFFSET>,
            CSGetUnorderedAccessViews: CSGetUnorderedAccessViews::<Identity, OFFSET>,
            CSGetShader: CSGetShader::<Identity, OFFSET>,
            CSGetSamplers: CSGetSamplers::<Identity, OFFSET>,
            CSGetConstantBuffers: CSGetConstantBuffers::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetContextFlags: GetContextFlags::<Identity, OFFSET>,
            FinishCommandList: FinishCommandList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DeviceContext {}
windows_core::imp::define_interface!(
    ID3D11DomainShader,
    ID3D11DomainShader_Vtbl,
    0xf582c508_0f36_490c_9977_31eece268cfa
);
impl core::ops::Deref for ID3D11DomainShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DomainShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11DomainShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11DomainShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11DomainShader_Vtbl {
    pub const fn new<Identity: ID3D11DomainShader_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DomainShader as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DomainShader {}
windows_core::imp::define_interface!(
    ID3D11GeometryShader,
    ID3D11GeometryShader_Vtbl,
    0x38325b96_effb_4022_ba02_2e795b70275c
);
impl core::ops::Deref for ID3D11GeometryShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11GeometryShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11GeometryShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11GeometryShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11GeometryShader_Vtbl {
    pub const fn new<Identity: ID3D11GeometryShader_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11GeometryShader as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11GeometryShader {}
windows_core::imp::define_interface!(
    ID3D11HullShader,
    ID3D11HullShader_Vtbl,
    0x8e5c6061_628a_4c8e_8264_bbe45cb3d5dd
);
impl core::ops::Deref for ID3D11HullShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11HullShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11HullShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11HullShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11HullShader_Vtbl {
    pub const fn new<Identity: ID3D11HullShader_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11HullShader as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11HullShader {}
windows_core::imp::define_interface!(
    ID3D11InputLayout,
    ID3D11InputLayout_Vtbl,
    0xe4819ddc_4cf0_4025_bd26_5de82a3e07b7
);
impl core::ops::Deref for ID3D11InputLayout {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11InputLayout,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11InputLayout_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11InputLayout_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11InputLayout_Vtbl {
    pub const fn new<Identity: ID3D11InputLayout_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11InputLayout as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11InputLayout {}
windows_core::imp::define_interface!(
    ID3D11PixelShader,
    ID3D11PixelShader_Vtbl,
    0xea82e40d_51dc_4f33_93d4_db7c9125ae8c
);
impl core::ops::Deref for ID3D11PixelShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11PixelShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11PixelShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11PixelShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11PixelShader_Vtbl {
    pub const fn new<Identity: ID3D11PixelShader_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11PixelShader as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11PixelShader {}
windows_core::imp::define_interface!(
    ID3D11Predicate,
    ID3D11Predicate_Vtbl,
    0x9eb576dd_9f77_4d86_81aa_8bab5fe490e2
);
impl core::ops::Deref for ID3D11Predicate {
    type Target = ID3D11Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Predicate,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Asynchronous,
    ID3D11Query
);
#[repr(C)]
pub struct ID3D11Predicate_Vtbl {
    pub base__: ID3D11Query_Vtbl,
}
impl windows_core::RuntimeName for ID3D11Predicate {}
windows_core::imp::define_interface!(
    ID3D11Query,
    ID3D11Query_Vtbl,
    0xd6c00747_87b7_425e_b84d_44d108560afd
);
impl core::ops::Deref for ID3D11Query {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Query,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Asynchronous
);
#[repr(C)]
pub struct ID3D11Query_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11Query {}
windows_core::imp::define_interface!(
    ID3D11RasterizerState,
    ID3D11RasterizerState_Vtbl,
    0x9bb4ab81_ab1a_4d8f_b506_fc04200b6ee7
);
impl core::ops::Deref for ID3D11RasterizerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11RasterizerState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11RasterizerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11RasterizerState {}
windows_core::imp::define_interface!(
    ID3D11RenderTargetView,
    ID3D11RenderTargetView_Vtbl,
    0xdfdba067_0b8d_4865_875b_d7b4516cc164
);
impl core::ops::Deref for ID3D11RenderTargetView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11RenderTargetView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11RenderTargetView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11RenderTargetView {}
windows_core::imp::define_interface!(
    ID3D11Resource,
    ID3D11Resource_Vtbl,
    0xdc8e63f3_d12b_4952_b47b_5e45026a862d
);
impl core::ops::Deref for ID3D11Resource {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Resource, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11Resource {
    pub(crate) unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetEvictionPriority)(
                windows_core::Interface::as_raw(self),
                evictionpriority,
            );
        }
    }
    pub(crate) unsafe fn GetEvictionPriority(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetEvictionPriority)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct ID3D11Resource_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_RESOURCE_DIMENSION),
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait ID3D11Resource_Impl: ID3D11DeviceChild_Impl {
    fn GetType(&self, presourcedimension: *mut D3D11_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl ID3D11Resource_Vtbl {
    pub const fn new<Identity: ID3D11Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ID3D11Resource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            presourcedimension: *mut D3D11_RESOURCE_DIMENSION,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Resource_Impl::GetType(this, core::mem::transmute_copy(&presourcedimension));
            }
        }
        unsafe extern "system" fn SetEvictionPriority<
            Identity: ID3D11Resource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            evictionpriority: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Resource_Impl::SetEvictionPriority(
                    this,
                    core::mem::transmute_copy(&evictionpriority),
                );
            }
        }
        unsafe extern "system" fn GetEvictionPriority<
            Identity: ID3D11Resource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Resource_Impl::GetEvictionPriority(this)
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Resource as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Resource {}
windows_core::imp::define_interface!(
    ID3D11SamplerState,
    ID3D11SamplerState_Vtbl,
    0xda6fea51_564c_4487_9810_f0d0f9b4e3a5
);
impl core::ops::Deref for ID3D11SamplerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11SamplerState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11SamplerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11SamplerState {}
windows_core::imp::define_interface!(
    ID3D11ShaderResourceView,
    ID3D11ShaderResourceView_Vtbl,
    0xb0e06fe0_8192_4e1a_b1ca_36d7414710b2
);
impl core::ops::Deref for ID3D11ShaderResourceView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ShaderResourceView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11ShaderResourceView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11ShaderResourceView {}
windows_core::imp::define_interface!(
    ID3D11Texture1D,
    ID3D11Texture1D_Vtbl,
    0xf8fb5c27_c6b3_4f75_a4c8_439af2ef564c
);
impl core::ops::Deref for ID3D11Texture1D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Texture1D,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Texture1D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11Texture1D {}
windows_core::imp::define_interface!(
    ID3D11Texture2D,
    ID3D11Texture2D_Vtbl,
    0x6f15aaf2_d208_4e89_9ab4_489535d34f9c
);
impl core::ops::Deref for ID3D11Texture2D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Texture2D,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
impl ID3D11Texture2D {
    pub(crate) unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            );
        }
    }
}
#[repr(C)]
pub struct ID3D11Texture2D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE2D_DESC),
}
pub trait ID3D11Texture2D_Impl: ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC);
}
impl ID3D11Texture2D_Vtbl {
    pub const fn new<Identity: ID3D11Texture2D_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D11Texture2D_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *mut D3D11_TEXTURE2D_DESC,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Texture2D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        Self {
            base__: ID3D11Resource_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture2D as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
            || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Texture2D {}
windows_core::imp::define_interface!(
    ID3D11Texture3D,
    ID3D11Texture3D_Vtbl,
    0x037e866e_f56d_4357_a8af_9dabbe6e250e
);
impl core::ops::Deref for ID3D11Texture3D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Texture3D,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Texture3D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11Texture3D {}
windows_core::imp::define_interface!(
    ID3D11UnorderedAccessView,
    ID3D11UnorderedAccessView_Vtbl,
    0x28acf509_7f5c_48f6_8611_f316010a6380
);
impl core::ops::Deref for ID3D11UnorderedAccessView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11UnorderedAccessView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11UnorderedAccessView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11UnorderedAccessView {}
windows_core::imp::define_interface!(
    ID3D11VertexShader,
    ID3D11VertexShader_Vtbl,
    0x3b301d64_d678_4289_8897_22f8928b72f3
);
impl core::ops::Deref for ID3D11VertexShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VertexShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11VertexShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
pub trait ID3D11VertexShader_Impl: ID3D11DeviceChild_Impl {}
impl ID3D11VertexShader_Vtbl {
    pub const fn new<Identity: ID3D11VertexShader_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VertexShader as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VertexShader {}
windows_core::imp::define_interface!(
    ID3D11VideoContext,
    ID3D11VideoContext_Vtbl,
    0x61f21c45_3c0e_4a74_9cea_67100d9ad5e4
);
impl core::ops::Deref for ID3D11VideoContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoContext,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
impl ID3D11VideoContext {
    pub(crate) unsafe fn GetDecoderBuffer<P0>(
        &self,
        pdecoder: P0,
        r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE,
        pbuffersize: *mut u32,
        ppbuffer: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetDecoderBuffer)(
                windows_core::Interface::as_raw(self),
                pdecoder.param().abi(),
                r#type,
                pbuffersize as _,
                ppbuffer as _,
            )
        }
    }
    pub(crate) unsafe fn ReleaseDecoderBuffer<P0>(
        &self,
        pdecoder: P0,
        r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseDecoderBuffer)(
                windows_core::Interface::as_raw(self),
                pdecoder.param().abi(),
                r#type,
            )
        }
    }
    pub(crate) unsafe fn DecoderBeginFrame<P0, P1>(
        &self,
        pdecoder: P0,
        pview: P1,
        contentkeysize: u32,
        pcontentkey: Option<*const core::ffi::c_void>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
        P1: windows_core::Param<ID3D11VideoDecoderOutputView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecoderBeginFrame)(
                windows_core::Interface::as_raw(self),
                pdecoder.param().abi(),
                pview.param().abi(),
                contentkeysize,
                pcontentkey.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn DecoderEndFrame<P0>(&self, pdecoder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecoderEndFrame)(
                windows_core::Interface::as_raw(self),
                pdecoder.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SubmitDecoderBuffers<P0>(
        &self,
        pdecoder: P0,
        pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC],
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubmitDecoderBuffers)(
                windows_core::Interface::as_raw(self),
                pdecoder.param().abi(),
                pbufferdesc.len().try_into().unwrap(),
                pbufferdesc.as_ptr(),
            )
        }
    }
    pub(crate) unsafe fn DecoderExtension<P0>(
        &self,
        pdecoder: P0,
        pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION,
    ) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecoderExtension)(
                windows_core::Interface::as_raw(self),
                pdecoder.param().abi(),
                pextensiondata,
            )
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputTargetRect<P0>(
        &self,
        pvideoprocessor: P0,
        enable: bool,
        prect: Option<*const RECT>,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputTargetRect)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                enable.into(),
                prect.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputBackgroundColor<P0>(
        &self,
        pvideoprocessor: P0,
        ycbcr: bool,
        pcolor: *const D3D11_VIDEO_COLOR,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputBackgroundColor)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                ycbcr.into(),
                pcolor,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputColorSpace<P0>(
        &self,
        pvideoprocessor: P0,
        pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputColorSpace)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                pcolorspace,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputAlphaFillMode<P0>(
        &self,
        pvideoprocessor: P0,
        alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
        streamindex: u32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputAlphaFillMode)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                alphafillmode,
                streamindex,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputConstriction<P0>(
        &self,
        pvideoprocessor: P0,
        enable: bool,
        size: SIZE,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputConstriction)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                enable.into(),
                size,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputStereoMode<P0>(
        &self,
        pvideoprocessor: P0,
        enable: bool,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputStereoMode)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                enable.into(),
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetOutputExtension<P0>(
        &self,
        pvideoprocessor: P0,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetOutputExtension)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                pextensionguid,
                datasize,
                pdata,
            )
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputTargetRect<P0>(
        &self,
        pvideoprocessor: P0,
        enabled: *mut windows_core::BOOL,
        prect: *mut RECT,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputTargetRect)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                enabled as _,
                prect as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputBackgroundColor<P0>(
        &self,
        pvideoprocessor: P0,
        pycbcr: *mut windows_core::BOOL,
        pcolor: *mut D3D11_VIDEO_COLOR,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputBackgroundColor)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                pycbcr as _,
                pcolor as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputColorSpace<P0>(
        &self,
        pvideoprocessor: P0,
    ) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputColorSpace)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputAlphaFillMode<P0>(
        &self,
        pvideoprocessor: P0,
        palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
        pstreamindex: *mut u32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputAlphaFillMode)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                palphafillmode as _,
                pstreamindex as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputConstriction<P0>(
        &self,
        pvideoprocessor: P0,
        penabled: *mut windows_core::BOOL,
        psize: *mut SIZE,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputConstriction)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                penabled as _,
                psize as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputStereoMode<P0>(
        &self,
        pvideoprocessor: P0,
    ) -> windows_core::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputStereoMode)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn VideoProcessorGetOutputExtension<P0>(
        &self,
        pvideoprocessor: P0,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetOutputExtension)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                pextensionguid,
                datasize,
                pdata as _,
            )
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamFrameFormat<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        frameformat: D3D11_VIDEO_FRAME_FORMAT,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamFrameFormat)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                frameformat,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamColorSpace<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamColorSpace)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                pcolorspace,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamOutputRate<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
        repeatframe: bool,
        pcustomrate: Option<*const DXGI_RATIONAL>,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamOutputRate)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                outputrate,
                repeatframe.into(),
                pcustomrate.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamSourceRect<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        prect: Option<*const RECT>,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamSourceRect)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                prect.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamDestRect<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        prect: Option<*const RECT>,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamDestRect)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                prect.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamAlpha<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        alpha: f32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamAlpha)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                alpha,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamPalette<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        pentries: Option<&[u32]>,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamPalette)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                pentries.map_or(0, |slice| slice.len().try_into().unwrap()),
                pentries.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamPixelAspectRatio<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        psourceaspectratio: Option<*const DXGI_RATIONAL>,
        pdestinationaspectratio: Option<*const DXGI_RATIONAL>,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamPixelAspectRatio)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                psourceaspectratio.unwrap_or(core::mem::zeroed()) as _,
                pdestinationaspectratio.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamLumaKey<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        lower: f32,
        upper: f32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamLumaKey)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                lower,
                upper,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamStereoFormat<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
        leftviewframe0: bool,
        baseviewframe0: bool,
        flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
        monooffset: i32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamStereoFormat)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                format,
                leftviewframe0.into(),
                baseviewframe0.into(),
                flipmode,
                monooffset,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamAutoProcessingMode<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamAutoProcessingMode)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamFilter<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        filter: D3D11_VIDEO_PROCESSOR_FILTER,
        enable: bool,
        level: i32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamFilter)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                filter,
                enable.into(),
                level,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamExtension<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamExtension)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                pextensionguid,
                datasize,
                pdata,
            )
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamFrameFormat<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
    ) -> D3D11_VIDEO_FRAME_FORMAT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamFrameFormat)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamColorSpace<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
    ) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamColorSpace)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamOutputRate<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
        prepeatframe: *mut windows_core::BOOL,
        pcustomrate: *mut DXGI_RATIONAL,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamOutputRate)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                poutputrate as _,
                prepeatframe as _,
                pcustomrate as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamSourceRect<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        prect: *mut RECT,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamSourceRect)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penabled as _,
                prect as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamDestRect<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        prect: *mut RECT,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamDestRect)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penabled as _,
                prect as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamAlpha<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        palpha: *mut f32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamAlpha)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penabled as _,
                palpha as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamPalette<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        count: u32,
        pentries: *mut u32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamPalette)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                count,
                pentries as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamPixelAspectRatio<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        psourceaspectratio: *mut DXGI_RATIONAL,
        pdestinationaspectratio: *mut DXGI_RATIONAL,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamPixelAspectRatio)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penabled as _,
                psourceaspectratio as _,
                pdestinationaspectratio as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamLumaKey<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        plower: *mut f32,
        pupper: *mut f32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamLumaKey)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penabled as _,
                plower as _,
                pupper as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamStereoFormat<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penable: *mut windows_core::BOOL,
        pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
        pleftviewframe0: *mut windows_core::BOOL,
        pbaseviewframe0: *mut windows_core::BOOL,
        pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
        monooffset: *mut i32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamStereoFormat)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penable as _,
                pformat as _,
                pleftviewframe0 as _,
                pbaseviewframe0 as _,
                pflipmode as _,
                monooffset as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamAutoProcessingMode<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
    ) -> windows_core::BOOL
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamAutoProcessingMode)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                &mut result__,
            );
            result__
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamFilter<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        filter: D3D11_VIDEO_PROCESSOR_FILTER,
        penabled: *mut windows_core::BOOL,
        plevel: *mut i32,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamFilter)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                filter,
                penabled as _,
                plevel as _,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamExtension<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamExtension)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                pextensionguid,
                datasize,
                pdata as _,
            )
        }
    }
    pub(crate) unsafe fn VideoProcessorBlt<P0, P1>(
        &self,
        pvideoprocessor: P0,
        pview: P1,
        outputframe: u32,
        pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM],
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11VideoProcessor>,
        P1: windows_core::Param<ID3D11VideoProcessorOutputView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorBlt)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                pview.param().abi(),
                outputframe,
                pstreams.len().try_into().unwrap(),
                pstreams.as_ptr(),
            )
        }
    }
    pub(crate) unsafe fn NegotiateCryptoSessionKeyExchange<P0>(
        &self,
        pcryptosession: P0,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NegotiateCryptoSessionKeyExchange)(
                windows_core::Interface::as_raw(self),
                pcryptosession.param().abi(),
                datasize,
                pdata as _,
            )
        }
    }
    pub(crate) unsafe fn EncryptionBlt<P0, P1, P2>(
        &self,
        pcryptosession: P0,
        psrcsurface: P1,
        pdstsurface: P2,
        ivsize: u32,
        piv: *mut core::ffi::c_void,
    ) where
        P0: windows_core::Param<ID3D11CryptoSession>,
        P1: windows_core::Param<ID3D11Texture2D>,
        P2: windows_core::Param<ID3D11Texture2D>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EncryptionBlt)(
                windows_core::Interface::as_raw(self),
                pcryptosession.param().abi(),
                psrcsurface.param().abi(),
                pdstsurface.param().abi(),
                ivsize,
                piv as _,
            );
        }
    }
    pub(crate) unsafe fn DecryptionBlt<P0, P1, P2>(
        &self,
        pcryptosession: P0,
        psrcsurface: P1,
        pdstsurface: P2,
        pencryptedblockinfo: Option<*const D3D11_ENCRYPTED_BLOCK_INFO>,
        contentkeysize: u32,
        pcontentkey: Option<*const core::ffi::c_void>,
        ivsize: u32,
        piv: *mut core::ffi::c_void,
    ) where
        P0: windows_core::Param<ID3D11CryptoSession>,
        P1: windows_core::Param<ID3D11Texture2D>,
        P2: windows_core::Param<ID3D11Texture2D>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecryptionBlt)(
                windows_core::Interface::as_raw(self),
                pcryptosession.param().abi(),
                psrcsurface.param().abi(),
                pdstsurface.param().abi(),
                pencryptedblockinfo.unwrap_or(core::mem::zeroed()) as _,
                contentkeysize,
                pcontentkey.unwrap_or(core::mem::zeroed()) as _,
                ivsize,
                piv as _,
            );
        }
    }
    pub(crate) unsafe fn StartSessionKeyRefresh<P0>(
        &self,
        pcryptosession: P0,
        randomnumbersize: u32,
        prandomnumber: *mut core::ffi::c_void,
    ) where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).StartSessionKeyRefresh)(
                windows_core::Interface::as_raw(self),
                pcryptosession.param().abi(),
                randomnumbersize,
                prandomnumber as _,
            );
        }
    }
    pub(crate) unsafe fn FinishSessionKeyRefresh<P0>(&self, pcryptosession: P0)
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FinishSessionKeyRefresh)(
                windows_core::Interface::as_raw(self),
                pcryptosession.param().abi(),
            );
        }
    }
    pub(crate) unsafe fn GetEncryptionBltKey<P0>(
        &self,
        pcryptosession: P0,
        keysize: u32,
        preadbackkey: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11CryptoSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetEncryptionBltKey)(
                windows_core::Interface::as_raw(self),
                pcryptosession.param().abi(),
                keysize,
                preadbackkey as _,
            )
        }
    }
    pub(crate) unsafe fn NegotiateAuthenticatedChannelKeyExchange<P0>(
        &self,
        pchannel: P0,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NegotiateAuthenticatedChannelKeyExchange)(
                windows_core::Interface::as_raw(self),
                pchannel.param().abi(),
                datasize,
                pdata as _,
            )
        }
    }
    pub(crate) unsafe fn QueryAuthenticatedChannel<P0>(
        &self,
        pchannel: P0,
        inputsize: u32,
        pinput: *const core::ffi::c_void,
        outputsize: u32,
        poutput: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).QueryAuthenticatedChannel)(
                windows_core::Interface::as_raw(self),
                pchannel.param().abi(),
                inputsize,
                pinput,
                outputsize,
                poutput as _,
            )
        }
    }
    pub(crate) unsafe fn ConfigureAuthenticatedChannel<P0>(
        &self,
        pchannel: P0,
        inputsize: u32,
        pinput: *const core::ffi::c_void,
        poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11AuthenticatedChannel>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ConfigureAuthenticatedChannel)(
                windows_core::Interface::as_raw(self),
                pchannel.param().abi(),
                inputsize,
                pinput,
                poutput as _,
            )
        }
    }
    pub(crate) unsafe fn VideoProcessorSetStreamRotation<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        enable: bool,
        rotation: D3D11_VIDEO_PROCESSOR_ROTATION,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorSetStreamRotation)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                enable.into(),
                rotation,
            );
        }
    }
    pub(crate) unsafe fn VideoProcessorGetStreamRotation<P0>(
        &self,
        pvideoprocessor: P0,
        streamindex: u32,
        penable: *mut windows_core::BOOL,
        protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION,
    ) where
        P0: windows_core::Param<ID3D11VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VideoProcessorGetStreamRotation)(
                windows_core::Interface::as_raw(self),
                pvideoprocessor.param().abi(),
                streamindex,
                penable as _,
                protation as _,
            );
        }
    }
}
#[repr(C)]
pub struct ID3D11VideoContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetDecoderBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D3D11_VIDEO_DECODER_BUFFER_TYPE,
        *mut u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReleaseDecoderBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D3D11_VIDEO_DECODER_BUFFER_TYPE,
    ) -> windows_core::HRESULT,
    pub DecoderBeginFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DecoderEndFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SubmitDecoderBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const D3D11_VIDEO_DECODER_BUFFER_DESC,
    ) -> windows_core::HRESULT,
    pub DecoderExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_DECODER_EXTENSION,
    ) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorSetOutputTargetRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *const RECT,
    ),
    pub VideoProcessorSetOutputBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *const D3D11_VIDEO_COLOR,
    ),
    pub VideoProcessorSetOutputColorSpace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    ),
    pub VideoProcessorSetOutputAlphaFillMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
        u32,
    ),
    pub VideoProcessorSetOutputConstriction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
        SIZE,
    ),
    pub VideoProcessorSetOutputStereoMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ),
    pub VideoProcessorSetOutputExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorGetOutputTargetRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
        *mut RECT,
    ),
    pub VideoProcessorGetOutputBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
        *mut D3D11_VIDEO_COLOR,
    ),
    pub VideoProcessorGetOutputColorSpace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    ),
    pub VideoProcessorGetOutputAlphaFillMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
        *mut u32,
    ),
    pub VideoProcessorGetOutputConstriction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
        *mut SIZE,
    ),
    pub VideoProcessorGetOutputStereoMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ),
    pub VideoProcessorGetOutputExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *mut core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorSetStreamFrameFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        D3D11_VIDEO_FRAME_FORMAT,
    ),
    pub VideoProcessorSetStreamColorSpace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    ),
    pub VideoProcessorSetStreamOutputRate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
        windows_core::BOOL,
        *const DXGI_RATIONAL,
    ),
    pub VideoProcessorSetStreamSourceRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        *const RECT,
    ),
    pub VideoProcessorSetStreamDestRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        *const RECT,
    ),
    pub VideoProcessorSetStreamAlpha: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        f32,
    ),
    pub VideoProcessorSetStreamPalette: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *const u32,
    ),
    pub VideoProcessorSetStreamPixelAspectRatio: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        *const DXGI_RATIONAL,
        *const DXGI_RATIONAL,
    ),
    pub VideoProcessorSetStreamLumaKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        f32,
        f32,
    ),
    pub VideoProcessorSetStreamStereoFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
        windows_core::BOOL,
        windows_core::BOOL,
        D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
        i32,
    ),
    pub VideoProcessorSetStreamAutoProcessingMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
    ),
    pub VideoProcessorSetStreamFilter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        D3D11_VIDEO_PROCESSOR_FILTER,
        windows_core::BOOL,
        i32,
    ),
    pub VideoProcessorSetStreamExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorGetStreamFrameFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut D3D11_VIDEO_FRAME_FORMAT,
    ),
    pub VideoProcessorGetStreamColorSpace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    ),
    pub VideoProcessorGetStreamOutputRate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
        *mut windows_core::BOOL,
        *mut DXGI_RATIONAL,
    ),
    pub VideoProcessorGetStreamSourceRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut RECT,
    ),
    pub VideoProcessorGetStreamDestRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut RECT,
    ),
    pub VideoProcessorGetStreamAlpha: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut f32,
    ),
    pub VideoProcessorGetStreamPalette: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
    ),
    pub VideoProcessorGetStreamPixelAspectRatio: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut DXGI_RATIONAL,
        *mut DXGI_RATIONAL,
    ),
    pub VideoProcessorGetStreamLumaKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut f32,
        *mut f32,
    ),
    pub VideoProcessorGetStreamStereoFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
        *mut windows_core::BOOL,
        *mut windows_core::BOOL,
        *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
        *mut i32,
    ),
    pub VideoProcessorGetStreamAutoProcessingMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
    ),
    pub VideoProcessorGetStreamFilter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        D3D11_VIDEO_PROCESSOR_FILTER,
        *mut windows_core::BOOL,
        *mut i32,
    ),
    pub VideoProcessorGetStreamExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        u32,
        *mut core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT,
    pub VideoProcessorBlt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *const D3D11_VIDEO_PROCESSOR_STREAM,
    ) -> windows_core::HRESULT,
    pub NegotiateCryptoSessionKeyExchange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EncryptionBlt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ),
    pub DecryptionBlt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_ENCRYPTED_BLOCK_INFO,
        u32,
        *const core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ),
    pub StartSessionKeyRefresh: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ),
    pub FinishSessionKeyRefresh:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetEncryptionBltKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NegotiateAuthenticatedChannelKeyExchange:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            u32,
            *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    pub QueryAuthenticatedChannel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ConfigureAuthenticatedChannel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const core::ffi::c_void,
        *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT,
    ) -> windows_core::HRESULT,
    pub VideoProcessorSetStreamRotation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        D3D11_VIDEO_PROCESSOR_ROTATION,
    ),
    pub VideoProcessorGetStreamRotation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut D3D11_VIDEO_PROCESSOR_ROTATION,
    ),
}
pub trait ID3D11VideoContext_Impl: ID3D11DeviceChild_Impl {
    fn GetDecoderBuffer(
        &self,
        pdecoder: windows_core::Ref<ID3D11VideoDecoder>,
        r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE,
        pbuffersize: *mut u32,
        ppbuffer: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn ReleaseDecoderBuffer(
        &self,
        pdecoder: windows_core::Ref<ID3D11VideoDecoder>,
        r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    ) -> windows_core::Result<()>;
    fn DecoderBeginFrame(
        &self,
        pdecoder: windows_core::Ref<ID3D11VideoDecoder>,
        pview: windows_core::Ref<ID3D11VideoDecoderOutputView>,
        contentkeysize: u32,
        pcontentkey: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn DecoderEndFrame(
        &self,
        pdecoder: windows_core::Ref<ID3D11VideoDecoder>,
    ) -> windows_core::Result<()>;
    fn SubmitDecoderBuffers(
        &self,
        pdecoder: windows_core::Ref<ID3D11VideoDecoder>,
        numbuffers: u32,
        pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC,
    ) -> windows_core::Result<()>;
    fn DecoderExtension(
        &self,
        pdecoder: windows_core::Ref<ID3D11VideoDecoder>,
        pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION,
    ) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorSetOutputTargetRect(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        enable: windows_core::BOOL,
        prect: *const RECT,
    );
    fn VideoProcessorSetOutputBackgroundColor(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        ycbcr: windows_core::BOOL,
        pcolor: *const D3D11_VIDEO_COLOR,
    );
    fn VideoProcessorSetOutputColorSpace(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    );
    fn VideoProcessorSetOutputAlphaFillMode(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
        streamindex: u32,
    );
    fn VideoProcessorSetOutputConstriction(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        enable: windows_core::BOOL,
        size: &SIZE,
    );
    fn VideoProcessorSetOutputStereoMode(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        enable: windows_core::BOOL,
    );
    fn VideoProcessorSetOutputExtension(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorGetOutputTargetRect(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        enabled: *mut windows_core::BOOL,
        prect: *mut RECT,
    );
    fn VideoProcessorGetOutputBackgroundColor(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        pycbcr: *mut windows_core::BOOL,
        pcolor: *mut D3D11_VIDEO_COLOR,
    );
    fn VideoProcessorGetOutputColorSpace(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    );
    fn VideoProcessorGetOutputAlphaFillMode(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
        pstreamindex: *mut u32,
    );
    fn VideoProcessorGetOutputConstriction(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        penabled: *mut windows_core::BOOL,
        psize: *mut SIZE,
    );
    fn VideoProcessorGetOutputStereoMode(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        penabled: *mut windows_core::BOOL,
    );
    fn VideoProcessorGetOutputExtension(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorSetStreamFrameFormat(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        frameformat: D3D11_VIDEO_FRAME_FORMAT,
    );
    fn VideoProcessorSetStreamColorSpace(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    );
    fn VideoProcessorSetStreamOutputRate(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
        repeatframe: windows_core::BOOL,
        pcustomrate: *const DXGI_RATIONAL,
    );
    fn VideoProcessorSetStreamSourceRect(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        prect: *const RECT,
    );
    fn VideoProcessorSetStreamDestRect(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        prect: *const RECT,
    );
    fn VideoProcessorSetStreamAlpha(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        alpha: f32,
    );
    fn VideoProcessorSetStreamPalette(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        count: u32,
        pentries: *const u32,
    );
    fn VideoProcessorSetStreamPixelAspectRatio(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        psourceaspectratio: *const DXGI_RATIONAL,
        pdestinationaspectratio: *const DXGI_RATIONAL,
    );
    fn VideoProcessorSetStreamLumaKey(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        lower: f32,
        upper: f32,
    );
    fn VideoProcessorSetStreamStereoFormat(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
        leftviewframe0: windows_core::BOOL,
        baseviewframe0: windows_core::BOOL,
        flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
        monooffset: i32,
    );
    fn VideoProcessorSetStreamAutoProcessingMode(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
    );
    fn VideoProcessorSetStreamFilter(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        filter: D3D11_VIDEO_PROCESSOR_FILTER,
        enable: windows_core::BOOL,
        level: i32,
    );
    fn VideoProcessorSetStreamExtension(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorGetStreamFrameFormat(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT,
    );
    fn VideoProcessorGetStreamColorSpace(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
    );
    fn VideoProcessorGetStreamOutputRate(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
        prepeatframe: *mut windows_core::BOOL,
        pcustomrate: *mut DXGI_RATIONAL,
    );
    fn VideoProcessorGetStreamSourceRect(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        prect: *mut RECT,
    );
    fn VideoProcessorGetStreamDestRect(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        prect: *mut RECT,
    );
    fn VideoProcessorGetStreamAlpha(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        palpha: *mut f32,
    );
    fn VideoProcessorGetStreamPalette(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        count: u32,
        pentries: *mut u32,
    );
    fn VideoProcessorGetStreamPixelAspectRatio(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        psourceaspectratio: *mut DXGI_RATIONAL,
        pdestinationaspectratio: *mut DXGI_RATIONAL,
    );
    fn VideoProcessorGetStreamLumaKey(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
        plower: *mut f32,
        pupper: *mut f32,
    );
    fn VideoProcessorGetStreamStereoFormat(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penable: *mut windows_core::BOOL,
        pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
        pleftviewframe0: *mut windows_core::BOOL,
        pbaseviewframe0: *mut windows_core::BOOL,
        pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
        monooffset: *mut i32,
    );
    fn VideoProcessorGetStreamAutoProcessingMode(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penabled: *mut windows_core::BOOL,
    );
    fn VideoProcessorGetStreamFilter(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        filter: D3D11_VIDEO_PROCESSOR_FILTER,
        penabled: *mut windows_core::BOOL,
        plevel: *mut i32,
    );
    fn VideoProcessorGetStreamExtension(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        pextensionguid: *const windows_core::GUID,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> APP_DEPRECATED_HRESULT;
    fn VideoProcessorBlt(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        pview: windows_core::Ref<ID3D11VideoProcessorOutputView>,
        outputframe: u32,
        streamcount: u32,
        pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM,
    ) -> windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchange(
        &self,
        pcryptosession: windows_core::Ref<ID3D11CryptoSession>,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn EncryptionBlt(
        &self,
        pcryptosession: windows_core::Ref<ID3D11CryptoSession>,
        psrcsurface: windows_core::Ref<ID3D11Texture2D>,
        pdstsurface: windows_core::Ref<ID3D11Texture2D>,
        ivsize: u32,
        piv: *mut core::ffi::c_void,
    );
    fn DecryptionBlt(
        &self,
        pcryptosession: windows_core::Ref<ID3D11CryptoSession>,
        psrcsurface: windows_core::Ref<ID3D11Texture2D>,
        pdstsurface: windows_core::Ref<ID3D11Texture2D>,
        pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO,
        contentkeysize: u32,
        pcontentkey: *const core::ffi::c_void,
        ivsize: u32,
        piv: *mut core::ffi::c_void,
    );
    fn StartSessionKeyRefresh(
        &self,
        pcryptosession: windows_core::Ref<ID3D11CryptoSession>,
        randomnumbersize: u32,
        prandomnumber: *mut core::ffi::c_void,
    );
    fn FinishSessionKeyRefresh(&self, pcryptosession: windows_core::Ref<ID3D11CryptoSession>);
    fn GetEncryptionBltKey(
        &self,
        pcryptosession: windows_core::Ref<ID3D11CryptoSession>,
        keysize: u32,
        preadbackkey: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn NegotiateAuthenticatedChannelKeyExchange(
        &self,
        pchannel: windows_core::Ref<ID3D11AuthenticatedChannel>,
        datasize: u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn QueryAuthenticatedChannel(
        &self,
        pchannel: windows_core::Ref<ID3D11AuthenticatedChannel>,
        inputsize: u32,
        pinput: *const core::ffi::c_void,
        outputsize: u32,
        poutput: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn ConfigureAuthenticatedChannel(
        &self,
        pchannel: windows_core::Ref<ID3D11AuthenticatedChannel>,
        inputsize: u32,
        pinput: *const core::ffi::c_void,
        poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT,
    ) -> windows_core::Result<()>;
    fn VideoProcessorSetStreamRotation(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        enable: windows_core::BOOL,
        rotation: D3D11_VIDEO_PROCESSOR_ROTATION,
    );
    fn VideoProcessorGetStreamRotation(
        &self,
        pvideoprocessor: windows_core::Ref<ID3D11VideoProcessor>,
        streamindex: u32,
        penable: *mut windows_core::BOOL,
        protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION,
    );
}
impl ID3D11VideoContext_Vtbl {
    pub const fn new<Identity: ID3D11VideoContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDecoderBuffer<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoder: *mut core::ffi::c_void,
            r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE,
            pbuffersize: *mut u32,
            ppbuffer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::GetDecoderBuffer(
                    this,
                    core::mem::transmute_copy(&pdecoder),
                    core::mem::transmute_copy(&r#type),
                    core::mem::transmute_copy(&pbuffersize),
                    core::mem::transmute_copy(&ppbuffer),
                )
                .into()
            }
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoder: *mut core::ffi::c_void,
            r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::ReleaseDecoderBuffer(
                    this,
                    core::mem::transmute_copy(&pdecoder),
                    core::mem::transmute_copy(&r#type),
                )
                .into()
            }
        }
        unsafe extern "system" fn DecoderBeginFrame<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoder: *mut core::ffi::c_void,
            pview: *mut core::ffi::c_void,
            contentkeysize: u32,
            pcontentkey: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecoderBeginFrame(
                    this,
                    core::mem::transmute_copy(&pdecoder),
                    core::mem::transmute_copy(&pview),
                    core::mem::transmute_copy(&contentkeysize),
                    core::mem::transmute_copy(&pcontentkey),
                )
                .into()
            }
        }
        unsafe extern "system" fn DecoderEndFrame<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoder: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecoderEndFrame(this, core::mem::transmute_copy(&pdecoder))
                    .into()
            }
        }
        unsafe extern "system" fn SubmitDecoderBuffers<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoder: *mut core::ffi::c_void,
            numbuffers: u32,
            pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::SubmitDecoderBuffers(
                    this,
                    core::mem::transmute_copy(&pdecoder),
                    core::mem::transmute_copy(&numbuffers),
                    core::mem::transmute_copy(&pbufferdesc),
                )
                .into()
            }
        }
        unsafe extern "system" fn DecoderExtension<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoder: *mut core::ffi::c_void,
            pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION,
        ) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecoderExtension(
                    this,
                    core::mem::transmute_copy(&pdecoder),
                    core::mem::transmute_copy(&pextensiondata),
                )
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            enable: windows_core::BOOL,
            prect: *const RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputTargetRect(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&prect),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            ycbcr: windows_core::BOOL,
            pcolor: *const D3D11_VIDEO_COLOR,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputBackgroundColor(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&ycbcr),
                    core::mem::transmute_copy(&pcolor),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputColorSpace(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&pcolorspace),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
            streamindex: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputAlphaFillMode(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&alphafillmode),
                    core::mem::transmute_copy(&streamindex),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            enable: windows_core::BOOL,
            size: SIZE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputConstriction(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute(&size),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            enable: windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputStereoMode(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&enable),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            pextensionguid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetOutputExtension(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&pextensionguid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            enabled: *mut windows_core::BOOL,
            prect: *mut RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputTargetRect(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&enabled),
                    core::mem::transmute_copy(&prect),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            pycbcr: *mut windows_core::BOOL,
            pcolor: *mut D3D11_VIDEO_COLOR,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputBackgroundColor(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&pycbcr),
                    core::mem::transmute_copy(&pcolor),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputColorSpace(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&pcolorspace),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE,
            pstreamindex: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputAlphaFillMode(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&palphafillmode),
                    core::mem::transmute_copy(&pstreamindex),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            penabled: *mut windows_core::BOOL,
            psize: *mut SIZE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputConstriction(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&psize),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            penabled: *mut windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputStereoMode(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&penabled),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            pextensionguid: *const windows_core::GUID,
            datasize: u32,
            pdata: *mut core::ffi::c_void,
        ) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetOutputExtension(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&pextensionguid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            frameformat: D3D11_VIDEO_FRAME_FORMAT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamFrameFormat(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&frameformat),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamColorSpace(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&pcolorspace),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
            repeatframe: windows_core::BOOL,
            pcustomrate: *const DXGI_RATIONAL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamOutputRate(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&outputrate),
                    core::mem::transmute_copy(&repeatframe),
                    core::mem::transmute_copy(&pcustomrate),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            prect: *const RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamSourceRect(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&prect),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            prect: *const RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamDestRect(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&prect),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            alpha: f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamAlpha(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&alpha),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            count: u32,
            pentries: *const u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamPalette(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&pentries),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            psourceaspectratio: *const DXGI_RATIONAL,
            pdestinationaspectratio: *const DXGI_RATIONAL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamPixelAspectRatio(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&psourceaspectratio),
                    core::mem::transmute_copy(&pdestinationaspectratio),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            lower: f32,
            upper: f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamLumaKey(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&lower),
                    core::mem::transmute_copy(&upper),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
            leftviewframe0: windows_core::BOOL,
            baseviewframe0: windows_core::BOOL,
            flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
            monooffset: i32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamStereoFormat(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&format),
                    core::mem::transmute_copy(&leftviewframe0),
                    core::mem::transmute_copy(&baseviewframe0),
                    core::mem::transmute_copy(&flipmode),
                    core::mem::transmute_copy(&monooffset),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamAutoProcessingMode(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            filter: D3D11_VIDEO_PROCESSOR_FILTER,
            enable: windows_core::BOOL,
            level: i32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamFilter(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&filter),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&level),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            pextensionguid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamExtension(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&pextensionguid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamFrameFormat(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&pframeformat),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamColorSpace(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&pcolorspace),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE,
            prepeatframe: *mut windows_core::BOOL,
            pcustomrate: *mut DXGI_RATIONAL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamOutputRate(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&poutputrate),
                    core::mem::transmute_copy(&prepeatframe),
                    core::mem::transmute_copy(&pcustomrate),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penabled: *mut windows_core::BOOL,
            prect: *mut RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamSourceRect(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&prect),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penabled: *mut windows_core::BOOL,
            prect: *mut RECT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamDestRect(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&prect),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penabled: *mut windows_core::BOOL,
            palpha: *mut f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamAlpha(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&palpha),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            count: u32,
            pentries: *mut u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamPalette(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&count),
                    core::mem::transmute_copy(&pentries),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penabled: *mut windows_core::BOOL,
            psourceaspectratio: *mut DXGI_RATIONAL,
            pdestinationaspectratio: *mut DXGI_RATIONAL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamPixelAspectRatio(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&psourceaspectratio),
                    core::mem::transmute_copy(&pdestinationaspectratio),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penabled: *mut windows_core::BOOL,
            plower: *mut f32,
            pupper: *mut f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamLumaKey(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&plower),
                    core::mem::transmute_copy(&pupper),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penable: *mut windows_core::BOOL,
            pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT,
            pleftviewframe0: *mut windows_core::BOOL,
            pbaseviewframe0: *mut windows_core::BOOL,
            pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE,
            monooffset: *mut i32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamStereoFormat(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penable),
                    core::mem::transmute_copy(&pformat),
                    core::mem::transmute_copy(&pleftviewframe0),
                    core::mem::transmute_copy(&pbaseviewframe0),
                    core::mem::transmute_copy(&pflipmode),
                    core::mem::transmute_copy(&monooffset),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penabled: *mut windows_core::BOOL,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamAutoProcessingMode(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penabled),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            filter: D3D11_VIDEO_PROCESSOR_FILTER,
            penabled: *mut windows_core::BOOL,
            plevel: *mut i32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamFilter(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&filter),
                    core::mem::transmute_copy(&penabled),
                    core::mem::transmute_copy(&plevel),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            pextensionguid: *const windows_core::GUID,
            datasize: u32,
            pdata: *mut core::ffi::c_void,
        ) -> APP_DEPRECATED_HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamExtension(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&pextensionguid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
            }
        }
        unsafe extern "system" fn VideoProcessorBlt<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            pview: *mut core::ffi::c_void,
            outputframe: u32,
            streamcount: u32,
            pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorBlt(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&pview),
                    core::mem::transmute_copy(&outputframe),
                    core::mem::transmute_copy(&streamcount),
                    core::mem::transmute_copy(&pstreams),
                )
                .into()
            }
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptosession: *mut core::ffi::c_void,
            datasize: u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::NegotiateCryptoSessionKeyExchange(
                    this,
                    core::mem::transmute_copy(&pcryptosession),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn EncryptionBlt<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptosession: *mut core::ffi::c_void,
            psrcsurface: *mut core::ffi::c_void,
            pdstsurface: *mut core::ffi::c_void,
            ivsize: u32,
            piv: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::EncryptionBlt(
                    this,
                    core::mem::transmute_copy(&pcryptosession),
                    core::mem::transmute_copy(&psrcsurface),
                    core::mem::transmute_copy(&pdstsurface),
                    core::mem::transmute_copy(&ivsize),
                    core::mem::transmute_copy(&piv),
                );
            }
        }
        unsafe extern "system" fn DecryptionBlt<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptosession: *mut core::ffi::c_void,
            psrcsurface: *mut core::ffi::c_void,
            pdstsurface: *mut core::ffi::c_void,
            pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO,
            contentkeysize: u32,
            pcontentkey: *const core::ffi::c_void,
            ivsize: u32,
            piv: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::DecryptionBlt(
                    this,
                    core::mem::transmute_copy(&pcryptosession),
                    core::mem::transmute_copy(&psrcsurface),
                    core::mem::transmute_copy(&pdstsurface),
                    core::mem::transmute_copy(&pencryptedblockinfo),
                    core::mem::transmute_copy(&contentkeysize),
                    core::mem::transmute_copy(&pcontentkey),
                    core::mem::transmute_copy(&ivsize),
                    core::mem::transmute_copy(&piv),
                );
            }
        }
        unsafe extern "system" fn StartSessionKeyRefresh<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptosession: *mut core::ffi::c_void,
            randomnumbersize: u32,
            prandomnumber: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::StartSessionKeyRefresh(
                    this,
                    core::mem::transmute_copy(&pcryptosession),
                    core::mem::transmute_copy(&randomnumbersize),
                    core::mem::transmute_copy(&prandomnumber),
                );
            }
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptosession: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::FinishSessionKeyRefresh(
                    this,
                    core::mem::transmute_copy(&pcryptosession),
                );
            }
        }
        unsafe extern "system" fn GetEncryptionBltKey<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptosession: *mut core::ffi::c_void,
            keysize: u32,
            preadbackkey: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::GetEncryptionBltKey(
                    this,
                    core::mem::transmute_copy(&pcryptosession),
                    core::mem::transmute_copy(&keysize),
                    core::mem::transmute_copy(&preadbackkey),
                )
                .into()
            }
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pchannel: *mut core::ffi::c_void,
            datasize: u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::NegotiateAuthenticatedChannelKeyExchange(
                    this,
                    core::mem::transmute_copy(&pchannel),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pchannel: *mut core::ffi::c_void,
            inputsize: u32,
            pinput: *const core::ffi::c_void,
            outputsize: u32,
            poutput: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::QueryAuthenticatedChannel(
                    this,
                    core::mem::transmute_copy(&pchannel),
                    core::mem::transmute_copy(&inputsize),
                    core::mem::transmute_copy(&pinput),
                    core::mem::transmute_copy(&outputsize),
                    core::mem::transmute_copy(&poutput),
                )
                .into()
            }
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pchannel: *mut core::ffi::c_void,
            inputsize: u32,
            pinput: *const core::ffi::c_void,
            poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::ConfigureAuthenticatedChannel(
                    this,
                    core::mem::transmute_copy(&pchannel),
                    core::mem::transmute_copy(&inputsize),
                    core::mem::transmute_copy(&pinput),
                    core::mem::transmute_copy(&poutput),
                )
                .into()
            }
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            enable: windows_core::BOOL,
            rotation: D3D11_VIDEO_PROCESSOR_ROTATION,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorSetStreamRotation(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&enable),
                    core::mem::transmute_copy(&rotation),
                );
            }
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<
            Identity: ID3D11VideoContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideoprocessor: *mut core::ffi::c_void,
            streamindex: u32,
            penable: *mut windows_core::BOOL,
            protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoContext_Impl::VideoProcessorGetStreamRotation(
                    this,
                    core::mem::transmute_copy(&pvideoprocessor),
                    core::mem::transmute_copy(&streamindex),
                    core::mem::transmute_copy(&penable),
                    core::mem::transmute_copy(&protation),
                );
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetDecoderBuffer: GetDecoderBuffer::<Identity, OFFSET>,
            ReleaseDecoderBuffer: ReleaseDecoderBuffer::<Identity, OFFSET>,
            DecoderBeginFrame: DecoderBeginFrame::<Identity, OFFSET>,
            DecoderEndFrame: DecoderEndFrame::<Identity, OFFSET>,
            SubmitDecoderBuffers: SubmitDecoderBuffers::<Identity, OFFSET>,
            DecoderExtension: DecoderExtension::<Identity, OFFSET>,
            VideoProcessorSetOutputTargetRect: VideoProcessorSetOutputTargetRect::<Identity, OFFSET>,
            VideoProcessorSetOutputBackgroundColor: VideoProcessorSetOutputBackgroundColor::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetOutputColorSpace: VideoProcessorSetOutputColorSpace::<Identity, OFFSET>,
            VideoProcessorSetOutputAlphaFillMode: VideoProcessorSetOutputAlphaFillMode::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetOutputConstriction: VideoProcessorSetOutputConstriction::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetOutputStereoMode: VideoProcessorSetOutputStereoMode::<Identity, OFFSET>,
            VideoProcessorSetOutputExtension: VideoProcessorSetOutputExtension::<Identity, OFFSET>,
            VideoProcessorGetOutputTargetRect: VideoProcessorGetOutputTargetRect::<Identity, OFFSET>,
            VideoProcessorGetOutputBackgroundColor: VideoProcessorGetOutputBackgroundColor::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetOutputColorSpace: VideoProcessorGetOutputColorSpace::<Identity, OFFSET>,
            VideoProcessorGetOutputAlphaFillMode: VideoProcessorGetOutputAlphaFillMode::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetOutputConstriction: VideoProcessorGetOutputConstriction::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetOutputStereoMode: VideoProcessorGetOutputStereoMode::<Identity, OFFSET>,
            VideoProcessorGetOutputExtension: VideoProcessorGetOutputExtension::<Identity, OFFSET>,
            VideoProcessorSetStreamFrameFormat: VideoProcessorSetStreamFrameFormat::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetStreamColorSpace: VideoProcessorSetStreamColorSpace::<Identity, OFFSET>,
            VideoProcessorSetStreamOutputRate: VideoProcessorSetStreamOutputRate::<Identity, OFFSET>,
            VideoProcessorSetStreamSourceRect: VideoProcessorSetStreamSourceRect::<Identity, OFFSET>,
            VideoProcessorSetStreamDestRect: VideoProcessorSetStreamDestRect::<Identity, OFFSET>,
            VideoProcessorSetStreamAlpha: VideoProcessorSetStreamAlpha::<Identity, OFFSET>,
            VideoProcessorSetStreamPalette: VideoProcessorSetStreamPalette::<Identity, OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio: VideoProcessorSetStreamPixelAspectRatio::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetStreamLumaKey: VideoProcessorSetStreamLumaKey::<Identity, OFFSET>,
            VideoProcessorSetStreamStereoFormat: VideoProcessorSetStreamStereoFormat::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetStreamAutoProcessingMode: VideoProcessorSetStreamAutoProcessingMode::<
                Identity,
                OFFSET,
            >,
            VideoProcessorSetStreamFilter: VideoProcessorSetStreamFilter::<Identity, OFFSET>,
            VideoProcessorSetStreamExtension: VideoProcessorSetStreamExtension::<Identity, OFFSET>,
            VideoProcessorGetStreamFrameFormat: VideoProcessorGetStreamFrameFormat::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetStreamColorSpace: VideoProcessorGetStreamColorSpace::<Identity, OFFSET>,
            VideoProcessorGetStreamOutputRate: VideoProcessorGetStreamOutputRate::<Identity, OFFSET>,
            VideoProcessorGetStreamSourceRect: VideoProcessorGetStreamSourceRect::<Identity, OFFSET>,
            VideoProcessorGetStreamDestRect: VideoProcessorGetStreamDestRect::<Identity, OFFSET>,
            VideoProcessorGetStreamAlpha: VideoProcessorGetStreamAlpha::<Identity, OFFSET>,
            VideoProcessorGetStreamPalette: VideoProcessorGetStreamPalette::<Identity, OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio: VideoProcessorGetStreamPixelAspectRatio::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetStreamLumaKey: VideoProcessorGetStreamLumaKey::<Identity, OFFSET>,
            VideoProcessorGetStreamStereoFormat: VideoProcessorGetStreamStereoFormat::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetStreamAutoProcessingMode: VideoProcessorGetStreamAutoProcessingMode::<
                Identity,
                OFFSET,
            >,
            VideoProcessorGetStreamFilter: VideoProcessorGetStreamFilter::<Identity, OFFSET>,
            VideoProcessorGetStreamExtension: VideoProcessorGetStreamExtension::<Identity, OFFSET>,
            VideoProcessorBlt: VideoProcessorBlt::<Identity, OFFSET>,
            NegotiateCryptoSessionKeyExchange: NegotiateCryptoSessionKeyExchange::<Identity, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange: NegotiateAuthenticatedChannelKeyExchange::<
                Identity,
                OFFSET,
            >,
            QueryAuthenticatedChannel: QueryAuthenticatedChannel::<Identity, OFFSET>,
            ConfigureAuthenticatedChannel: ConfigureAuthenticatedChannel::<Identity, OFFSET>,
            VideoProcessorSetStreamRotation: VideoProcessorSetStreamRotation::<Identity, OFFSET>,
            VideoProcessorGetStreamRotation: VideoProcessorGetStreamRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoContext {}
windows_core::imp::define_interface!(
    ID3D11VideoDecoder,
    ID3D11VideoDecoder_Vtbl,
    0x3c9c5b51_995d_48d1_9b8d_fa5caeded65c
);
impl core::ops::Deref for ID3D11VideoDecoder {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoDecoder,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11VideoDecoder_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetCreationParameters: usize,
    GetDriverHandle: usize,
}
impl windows_core::RuntimeName for ID3D11VideoDecoder {}
windows_core::imp::define_interface!(
    ID3D11VideoDecoderOutputView,
    ID3D11VideoDecoderOutputView_Vtbl,
    0xc2931aea_2a85_4f20_860f_fba1fd256e18
);
impl core::ops::Deref for ID3D11VideoDecoderOutputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoDecoderOutputView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11VideoDecoderOutputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
impl windows_core::RuntimeName for ID3D11VideoDecoderOutputView {}
windows_core::imp::define_interface!(
    ID3D11VideoDevice,
    ID3D11VideoDevice_Vtbl,
    0x10ec4d5b_975a_4689_b9e4_d0aac30fe333
);
windows_core::imp::interface_hierarchy!(ID3D11VideoDevice, windows_core::IUnknown);
impl ID3D11VideoDevice {
    pub(crate) unsafe fn CreateVideoDecoder(
        &self,
        pvideodesc: *const D3D11_VIDEO_DECODER_DESC,
        pconfig: *const D3D11_VIDEO_DECODER_CONFIG,
    ) -> windows_core::Result<ID3D11VideoDecoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoDecoder)(
                windows_core::Interface::as_raw(self),
                pvideodesc,
                pconfig,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateVideoProcessor<P0>(
        &self,
        penum: P0,
        rateconversionindex: u32,
    ) -> windows_core::Result<ID3D11VideoProcessor>
    where
        P0: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoProcessor)(
                windows_core::Interface::as_raw(self),
                penum.param().abi(),
                rateconversionindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateAuthenticatedChannel(
        &self,
        channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE,
    ) -> windows_core::Result<ID3D11AuthenticatedChannel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAuthenticatedChannel)(
                windows_core::Interface::as_raw(self),
                channeltype,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateCryptoSession(
        &self,
        pcryptotype: *const windows_core::GUID,
        pdecoderprofile: Option<*const windows_core::GUID>,
        pkeyexchangetype: *const windows_core::GUID,
    ) -> windows_core::Result<ID3D11CryptoSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCryptoSession)(
                windows_core::Interface::as_raw(self),
                pcryptotype,
                pdecoderprofile.unwrap_or(core::mem::zeroed()) as _,
                pkeyexchangetype,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateVideoDecoderOutputView<P0>(
        &self,
        presource: P0,
        pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC,
        ppvdovview: Option<*mut Option<ID3D11VideoDecoderOutputView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateVideoDecoderOutputView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc,
                ppvdovview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateVideoProcessorInputView<P0, P1>(
        &self,
        presource: P0,
        penum: P1,
        pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
        ppvpiview: Option<*mut Option<ID3D11VideoProcessorInputView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateVideoProcessorInputView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                penum.param().abi(),
                pdesc,
                ppvpiview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateVideoProcessorOutputView<P0, P1>(
        &self,
        presource: P0,
        penum: P1,
        pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
        ppvpoview: Option<*mut Option<ID3D11VideoProcessorOutputView>>,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateVideoProcessorOutputView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                penum.param().abi(),
                pdesc,
                ppvpoview.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn CreateVideoProcessorEnumerator(
        &self,
        pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
    ) -> windows_core::Result<ID3D11VideoProcessorEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoProcessorEnumerator)(
                windows_core::Interface::as_raw(self),
                pdesc,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetVideoDecoderProfileCount(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoDecoderProfileCount)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetVideoDecoderProfile(
        &self,
        index: u32,
    ) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoDecoderProfile)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn CheckVideoDecoderFormat(
        &self,
        pdecoderprofile: *const windows_core::GUID,
        format: DXGI_FORMAT,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckVideoDecoderFormat)(
                windows_core::Interface::as_raw(self),
                pdecoderprofile,
                format,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetVideoDecoderConfigCount(
        &self,
        pdesc: *const D3D11_VIDEO_DECODER_DESC,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoDecoderConfigCount)(
                windows_core::Interface::as_raw(self),
                pdesc,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetVideoDecoderConfig(
        &self,
        pdesc: *const D3D11_VIDEO_DECODER_DESC,
        index: u32,
        pconfig: *mut D3D11_VIDEO_DECODER_CONFIG,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoDecoderConfig)(
                windows_core::Interface::as_raw(self),
                pdesc,
                index,
                pconfig as _,
            )
        }
    }
    pub(crate) unsafe fn GetContentProtectionCaps(
        &self,
        pcryptotype: Option<*const windows_core::GUID>,
        pdecoderprofile: Option<*const windows_core::GUID>,
        pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetContentProtectionCaps)(
                windows_core::Interface::as_raw(self),
                pcryptotype.unwrap_or(core::mem::zeroed()) as _,
                pdecoderprofile.unwrap_or(core::mem::zeroed()) as _,
                pcaps as _,
            )
        }
    }
    pub(crate) unsafe fn CheckCryptoKeyExchange(
        &self,
        pcryptotype: *const windows_core::GUID,
        pdecoderprofile: Option<*const windows_core::GUID>,
        index: u32,
    ) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCryptoKeyExchange)(
                windows_core::Interface::as_raw(self),
                pcryptotype,
                pdecoderprofile.unwrap_or(core::mem::zeroed()) as _,
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: Option<*const core::ffi::c_void>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                datasize,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn SetPrivateDataInterface<P1>(
        &self,
        guid: *const windows_core::GUID,
        pdata: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                guid,
                pdata.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct ID3D11VideoDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateVideoDecoder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_DECODER_DESC,
        *const D3D11_VIDEO_DECODER_CONFIG,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVideoProcessor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateAuthenticatedChannel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D3D11_AUTHENTICATED_CHANNEL_TYPE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCryptoSession: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVideoDecoderOutputView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVideoProcessorInputView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVideoProcessorOutputView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVideoProcessorEnumerator: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetVideoDecoderProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetVideoDecoderProfile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub CheckVideoDecoderFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        DXGI_FORMAT,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetVideoDecoderConfigCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_DECODER_DESC,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetVideoDecoderConfig: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_VIDEO_DECODER_DESC,
        u32,
        *mut D3D11_VIDEO_DECODER_CONFIG,
    ) -> windows_core::HRESULT,
    pub GetContentProtectionCaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS,
    ) -> windows_core::HRESULT,
    pub CheckCryptoKeyExchange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        u32,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ID3D11VideoDevice_Impl: windows_core::IUnknownImpl {
    fn CreateVideoDecoder(
        &self,
        pvideodesc: *const D3D11_VIDEO_DECODER_DESC,
        pconfig: *const D3D11_VIDEO_DECODER_CONFIG,
    ) -> windows_core::Result<ID3D11VideoDecoder>;
    fn CreateVideoProcessor(
        &self,
        penum: windows_core::Ref<ID3D11VideoProcessorEnumerator>,
        rateconversionindex: u32,
    ) -> windows_core::Result<ID3D11VideoProcessor>;
    fn CreateAuthenticatedChannel(
        &self,
        channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE,
    ) -> windows_core::Result<ID3D11AuthenticatedChannel>;
    fn CreateCryptoSession(
        &self,
        pcryptotype: *const windows_core::GUID,
        pdecoderprofile: *const windows_core::GUID,
        pkeyexchangetype: *const windows_core::GUID,
    ) -> windows_core::Result<ID3D11CryptoSession>;
    fn CreateVideoDecoderOutputView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC,
        ppvdovview: windows_core::OutRef<ID3D11VideoDecoderOutputView>,
    ) -> windows_core::Result<()>;
    fn CreateVideoProcessorInputView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        penum: windows_core::Ref<ID3D11VideoProcessorEnumerator>,
        pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
        ppvpiview: windows_core::OutRef<ID3D11VideoProcessorInputView>,
    ) -> windows_core::Result<()>;
    fn CreateVideoProcessorOutputView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        penum: windows_core::Ref<ID3D11VideoProcessorEnumerator>,
        pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
        ppvpoview: windows_core::OutRef<ID3D11VideoProcessorOutputView>,
    ) -> windows_core::Result<()>;
    fn CreateVideoProcessorEnumerator(
        &self,
        pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
    ) -> windows_core::Result<ID3D11VideoProcessorEnumerator>;
    fn GetVideoDecoderProfileCount(&self) -> u32;
    fn GetVideoDecoderProfile(&self, index: u32) -> windows_core::Result<windows_core::GUID>;
    fn CheckVideoDecoderFormat(
        &self,
        pdecoderprofile: *const windows_core::GUID,
        format: DXGI_FORMAT,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn GetVideoDecoderConfigCount(
        &self,
        pdesc: *const D3D11_VIDEO_DECODER_DESC,
    ) -> windows_core::Result<u32>;
    fn GetVideoDecoderConfig(
        &self,
        pdesc: *const D3D11_VIDEO_DECODER_DESC,
        index: u32,
        pconfig: *mut D3D11_VIDEO_DECODER_CONFIG,
    ) -> windows_core::Result<()>;
    fn GetContentProtectionCaps(
        &self,
        pcryptotype: *const windows_core::GUID,
        pdecoderprofile: *const windows_core::GUID,
        pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS,
    ) -> windows_core::Result<()>;
    fn CheckCryptoKeyExchange(
        &self,
        pcryptotype: *const windows_core::GUID,
        pdecoderprofile: *const windows_core::GUID,
        index: u32,
    ) -> windows_core::Result<windows_core::GUID>;
    fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        guid: *const windows_core::GUID,
        pdata: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl ID3D11VideoDevice_Vtbl {
    pub const fn new<Identity: ID3D11VideoDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVideoDecoder<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pvideodesc: *const D3D11_VIDEO_DECODER_DESC,
            pconfig: *const D3D11_VIDEO_DECODER_CONFIG,
            ppdecoder: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateVideoDecoder(
                    this,
                    core::mem::transmute_copy(&pvideodesc),
                    core::mem::transmute_copy(&pconfig),
                ) {
                    Ok(ok__) => {
                        ppdecoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            penum: *mut core::ffi::c_void,
            rateconversionindex: u32,
            ppvideoprocessor: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateVideoProcessor(
                    this,
                    core::mem::transmute_copy(&penum),
                    core::mem::transmute_copy(&rateconversionindex),
                ) {
                    Ok(ok__) => {
                        ppvideoprocessor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE,
            ppauthenticatedchannel: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateAuthenticatedChannel(
                    this,
                    core::mem::transmute_copy(&channeltype),
                ) {
                    Ok(ok__) => {
                        ppauthenticatedchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCryptoSession<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptotype: *const windows_core::GUID,
            pdecoderprofile: *const windows_core::GUID,
            pkeyexchangetype: *const windows_core::GUID,
            ppcryptosession: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateCryptoSession(
                    this,
                    core::mem::transmute_copy(&pcryptotype),
                    core::mem::transmute_copy(&pdecoderprofile),
                    core::mem::transmute_copy(&pkeyexchangetype),
                ) {
                    Ok(ok__) => {
                        ppcryptosession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC,
            ppvdovview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::CreateVideoDecoderOutputView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppvdovview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            penum: *mut core::ffi::c_void,
            pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
            ppvpiview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::CreateVideoProcessorInputView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&penum),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppvpiview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            penum: *mut core::ffi::c_void,
            pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
            ppvpoview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::CreateVideoProcessorOutputView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&penum),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppvpoview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CreateVideoProcessorEnumerator(
                    this,
                    core::mem::transmute_copy(&pdesc),
                ) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::GetVideoDecoderProfileCount(this)
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfile<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            pdecoderprofile: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::GetVideoDecoderProfile(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        pdecoderprofile.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdecoderprofile: *const windows_core::GUID,
            format: DXGI_FORMAT,
            psupported: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CheckVideoDecoderFormat(
                    this,
                    core::mem::transmute_copy(&pdecoderprofile),
                    core::mem::transmute_copy(&format),
                ) {
                    Ok(ok__) => {
                        psupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_VIDEO_DECODER_DESC,
            pcount: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::GetVideoDecoderConfigCount(
                    this,
                    core::mem::transmute_copy(&pdesc),
                ) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfig<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_VIDEO_DECODER_DESC,
            index: u32,
            pconfig: *mut D3D11_VIDEO_DECODER_CONFIG,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::GetVideoDecoderConfig(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&pconfig),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetContentProtectionCaps<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptotype: *const windows_core::GUID,
            pdecoderprofile: *const windows_core::GUID,
            pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::GetContentProtectionCaps(
                    this,
                    core::mem::transmute_copy(&pcryptotype),
                    core::mem::transmute_copy(&pdecoderprofile),
                    core::mem::transmute_copy(&pcaps),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcryptotype: *const windows_core::GUID,
            pdecoderprofile: *const windows_core::GUID,
            index: u32,
            pkeyexchangetype: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoDevice_Impl::CheckCryptoKeyExchange(
                    this,
                    core::mem::transmute_copy(&pcryptotype),
                    core::mem::transmute_copy(&pdecoderprofile),
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        pkeyexchangetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: ID3D11VideoDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoDevice_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoDecoder: CreateVideoDecoder::<Identity, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, OFFSET>,
            CreateVideoDecoderOutputView: CreateVideoDecoderOutputView::<Identity, OFFSET>,
            CreateVideoProcessorInputView: CreateVideoProcessorInputView::<Identity, OFFSET>,
            CreateVideoProcessorOutputView: CreateVideoProcessorOutputView::<Identity, OFFSET>,
            CreateVideoProcessorEnumerator: CreateVideoProcessorEnumerator::<Identity, OFFSET>,
            GetVideoDecoderProfileCount: GetVideoDecoderProfileCount::<Identity, OFFSET>,
            GetVideoDecoderProfile: GetVideoDecoderProfile::<Identity, OFFSET>,
            CheckVideoDecoderFormat: CheckVideoDecoderFormat::<Identity, OFFSET>,
            GetVideoDecoderConfigCount: GetVideoDecoderConfigCount::<Identity, OFFSET>,
            GetVideoDecoderConfig: GetVideoDecoderConfig::<Identity, OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, OFFSET>,
            CheckCryptoKeyExchange: CheckCryptoKeyExchange::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoDevice {}
windows_core::imp::define_interface!(
    ID3D11VideoProcessor,
    ID3D11VideoProcessor_Vtbl,
    0x1d7b0652_185f_41c6_85ce_0c5be3d4ae6c
);
impl core::ops::Deref for ID3D11VideoProcessor {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoProcessor,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
impl ID3D11VideoProcessor {
    pub(crate) unsafe fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
        unsafe {
            (windows_core::Interface::vtable(self).GetContentDesc)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            );
        }
    }
    pub(crate) unsafe fn GetRateConversionCaps(
        &self,
        pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GetRateConversionCaps)(
                windows_core::Interface::as_raw(self),
                pcaps as _,
            );
        }
    }
}
#[repr(C)]
pub struct ID3D11VideoProcessor_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetContentDesc:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC),
    pub GetRateConversionCaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
    ),
}
pub trait ID3D11VideoProcessor_Impl: ID3D11DeviceChild_Impl {
    fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC);
    fn GetRateConversionCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS);
}
impl ID3D11VideoProcessor_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContentDesc<
            Identity: ID3D11VideoProcessor_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessor_Impl::GetContentDesc(this, core::mem::transmute_copy(&pdesc));
            }
        }
        unsafe extern "system" fn GetRateConversionCaps<
            Identity: ID3D11VideoProcessor_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessor_Impl::GetRateConversionCaps(
                    this,
                    core::mem::transmute_copy(&pcaps),
                );
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetContentDesc: GetContentDesc::<Identity, OFFSET>,
            GetRateConversionCaps: GetRateConversionCaps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessor as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoProcessor {}
windows_core::imp::define_interface!(
    ID3D11VideoProcessorEnumerator,
    ID3D11VideoProcessorEnumerator_Vtbl,
    0x31627037_53ab_4200_9061_05faa9ab45f9
);
impl core::ops::Deref for ID3D11VideoProcessorEnumerator {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoProcessorEnumerator,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
impl ID3D11VideoProcessorEnumerator {
    pub(crate) unsafe fn GetVideoProcessorContentDesc(
        &self,
        pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoProcessorContentDesc)(
                windows_core::Interface::as_raw(self),
                pcontentdesc as _,
            )
        }
    }
    pub(crate) unsafe fn CheckVideoProcessorFormat(
        &self,
        format: DXGI_FORMAT,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckVideoProcessorFormat)(
                windows_core::Interface::as_raw(self),
                format,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetVideoProcessorCaps(
        &self,
        pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoProcessorCaps)(
                windows_core::Interface::as_raw(self),
                pcaps as _,
            )
        }
    }
    pub(crate) unsafe fn GetVideoProcessorRateConversionCaps(
        &self,
        typeindex: u32,
        pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoProcessorRateConversionCaps)(
                windows_core::Interface::as_raw(self),
                typeindex,
                pcaps as _,
            )
        }
    }
    pub(crate) unsafe fn GetVideoProcessorCustomRate(
        &self,
        typeindex: u32,
        customrateindex: u32,
        prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoProcessorCustomRate)(
                windows_core::Interface::as_raw(self),
                typeindex,
                customrateindex,
                prate as _,
            )
        }
    }
    pub(crate) unsafe fn GetVideoProcessorFilterRange(
        &self,
        filter: D3D11_VIDEO_PROCESSOR_FILTER,
    ) -> windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoProcessorFilterRange)(
                windows_core::Interface::as_raw(self),
                filter,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ID3D11VideoProcessorEnumerator_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub GetVideoProcessorContentDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
    ) -> windows_core::HRESULT,
    pub CheckVideoProcessorFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_FORMAT,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetVideoProcessorCaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_CAPS,
    ) -> windows_core::HRESULT,
    pub GetVideoProcessorRateConversionCaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
    )
        -> windows_core::HRESULT,
    pub GetVideoProcessorCustomRate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE,
    ) -> windows_core::HRESULT,
    pub GetVideoProcessorFilterRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D3D11_VIDEO_PROCESSOR_FILTER,
        *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE,
    ) -> windows_core::HRESULT,
}
pub trait ID3D11VideoProcessorEnumerator_Impl: ID3D11DeviceChild_Impl {
    fn GetVideoProcessorContentDesc(
        &self,
        pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
    ) -> windows_core::Result<()>;
    fn CheckVideoProcessorFormat(&self, format: DXGI_FORMAT) -> windows_core::Result<u32>;
    fn GetVideoProcessorCaps(
        &self,
        pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS,
    ) -> windows_core::Result<()>;
    fn GetVideoProcessorRateConversionCaps(
        &self,
        typeindex: u32,
        pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
    ) -> windows_core::Result<()>;
    fn GetVideoProcessorCustomRate(
        &self,
        typeindex: u32,
        customrateindex: u32,
        prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE,
    ) -> windows_core::Result<()>;
    fn GetVideoProcessorFilterRange(
        &self,
        filter: D3D11_VIDEO_PROCESSOR_FILTER,
    ) -> windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>;
}
impl ID3D11VideoProcessorEnumerator_Vtbl {
    pub const fn new<Identity: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVideoProcessorContentDesc<
            Identity: ID3D11VideoProcessorEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorContentDesc(
                    this,
                    core::mem::transmute_copy(&pcontentdesc),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<
            Identity: ID3D11VideoProcessorEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: DXGI_FORMAT,
            pflags: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoProcessorEnumerator_Impl::CheckVideoProcessorFormat(
                    this,
                    core::mem::transmute_copy(&format),
                ) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<
            Identity: ID3D11VideoProcessorEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorCaps(
                    this,
                    core::mem::transmute_copy(&pcaps),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<
            Identity: ID3D11VideoProcessorEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            typeindex: u32,
            pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorRateConversionCaps(
                    this,
                    core::mem::transmute_copy(&typeindex),
                    core::mem::transmute_copy(&pcaps),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<
            Identity: ID3D11VideoProcessorEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            typeindex: u32,
            customrateindex: u32,
            prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorCustomRate(
                    this,
                    core::mem::transmute_copy(&typeindex),
                    core::mem::transmute_copy(&customrateindex),
                    core::mem::transmute_copy(&prate),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<
            Identity: ID3D11VideoProcessorEnumerator_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            filter: D3D11_VIDEO_PROCESSOR_FILTER,
            prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorFilterRange(
                    this,
                    core::mem::transmute_copy(&filter),
                ) {
                    Ok(ok__) => {
                        prange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetVideoProcessorContentDesc: GetVideoProcessorContentDesc::<Identity, OFFSET>,
            CheckVideoProcessorFormat: CheckVideoProcessorFormat::<Identity, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, OFFSET>,
            GetVideoProcessorRateConversionCaps: GetVideoProcessorRateConversionCaps::<
                Identity,
                OFFSET,
            >,
            GetVideoProcessorCustomRate: GetVideoProcessorCustomRate::<Identity, OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator as windows_core::Interface>::IID
            || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11VideoProcessorEnumerator {}
windows_core::imp::define_interface!(
    ID3D11VideoProcessorInputView,
    ID3D11VideoProcessorInputView_Vtbl,
    0x11ec5a5f_51dc_4945_ab34_6e8c21300ea5
);
impl core::ops::Deref for ID3D11VideoProcessorInputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoProcessorInputView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
impl ID3D11VideoProcessorInputView {
    pub(crate) unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
}
#[repr(C)]
pub struct ID3D11VideoProcessorInputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
    ),
}
impl windows_core::RuntimeName for ID3D11VideoProcessorInputView {}
windows_core::imp::define_interface!(
    ID3D11VideoProcessorOutputView,
    ID3D11VideoProcessorOutputView_Vtbl,
    0xa048285e_25a9_4527_bd93_d68b68c44254
);
impl core::ops::Deref for ID3D11VideoProcessorOutputView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VideoProcessorOutputView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
impl ID3D11VideoProcessorOutputView {
    pub(crate) unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
}
#[repr(C)]
pub struct ID3D11VideoProcessorOutputView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
    ),
}
impl windows_core::RuntimeName for ID3D11VideoProcessorOutputView {}
windows_core::imp::define_interface!(
    ID3D11View,
    ID3D11View_Vtbl,
    0x839d1216_bb2e_412b_b7f4_a9dbebe08ed1
);
impl core::ops::Deref for ID3D11View {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11View, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
pub struct ID3D11View_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetResource: usize,
}
impl windows_core::RuntimeName for ID3D11View {}
windows_core::imp::define_interface!(
    IDCompositionAnimation,
    IDCompositionAnimation_Vtbl,
    0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5
);
windows_core::imp::interface_hierarchy!(IDCompositionAnimation, windows_core::IUnknown);
#[repr(C)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Reset: usize,
    SetAbsoluteBeginTime: usize,
    AddCubic: usize,
    AddSinusoidal: usize,
    AddRepeat: usize,
    End: usize,
}
impl windows_core::RuntimeName for IDCompositionAnimation {}
windows_core::imp::define_interface!(
    IDCompositionClip,
    IDCompositionClip_Vtbl,
    0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7
);
windows_core::imp::interface_hierarchy!(IDCompositionClip, windows_core::IUnknown);
#[repr(C)]
pub struct IDCompositionClip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDCompositionClip_Impl: windows_core::IUnknownImpl {}
impl IDCompositionClip_Vtbl {
    pub const fn new<Identity: IDCompositionClip_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionClip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionClip {}
windows_core::imp::define_interface!(
    IDCompositionDesktopDevice,
    IDCompositionDesktopDevice_Vtbl,
    0x5f4633fe_1e08_4cb8_8c75_ce24333f5602
);
impl core::ops::Deref for IDCompositionDesktopDevice {
    type Target = IDCompositionDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionDesktopDevice,
    windows_core::IUnknown,
    IDCompositionDevice2
);
impl IDCompositionDesktopDevice {
    pub(crate) unsafe fn CreateTargetForHwnd(
        &self,
        hwnd: HWND,
        topmost: bool,
    ) -> windows_core::Result<IDCompositionTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTargetForHwnd)(
                windows_core::Interface::as_raw(self),
                hwnd,
                topmost.into(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSurfaceFromHandle(
        &self,
        handle: HANDLE,
    ) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFromHandle)(
                windows_core::Interface::as_raw(self),
                handle,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSurfaceFromHwnd(
        &self,
        hwnd: HWND,
    ) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFromHwnd)(
                windows_core::Interface::as_raw(self),
                hwnd,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateTargetForHwnd: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDCompositionDesktopDevice_Impl: IDCompositionDevice2_Impl {
    fn CreateTargetForHwnd(
        &self,
        hwnd: HWND,
        topmost: windows_core::BOOL,
    ) -> windows_core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(
        &self,
        handle: HANDLE,
    ) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(&self, hwnd: HWND) -> windows_core::Result<windows_core::IUnknown>;
}
impl IDCompositionDesktopDevice_Vtbl {
    pub const fn new<Identity: IDCompositionDesktopDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTargetForHwnd<
            Identity: IDCompositionDesktopDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hwnd: HWND,
            topmost: windows_core::BOOL,
            target: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDesktopDevice_Impl::CreateTargetForHwnd(
                    this,
                    core::mem::transmute_copy(&hwnd),
                    core::mem::transmute_copy(&topmost),
                ) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<
            Identity: IDCompositionDesktopDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handle: HANDLE,
            surface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDesktopDevice_Impl::CreateSurfaceFromHandle(
                    this,
                    core::mem::transmute_copy(&handle),
                ) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<
            Identity: IDCompositionDesktopDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hwnd: HWND,
            surface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDesktopDevice_Impl::CreateSurfaceFromHwnd(
                    this,
                    core::mem::transmute_copy(&hwnd),
                ) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDCompositionDevice2_Vtbl::new::<Identity, OFFSET>(),
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDesktopDevice as windows_core::Interface>::IID
            || iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionDesktopDevice {}
windows_core::imp::define_interface!(
    IDCompositionDevice2,
    IDCompositionDevice2_Vtbl,
    0x75f6468d_1b8e_447c_9bc6_75fea80b5b25
);
windows_core::imp::interface_hierarchy!(IDCompositionDevice2, windows_core::IUnknown);
impl IDCompositionDevice2 {
    pub(crate) unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn WaitForCommitCompletion(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).WaitForCommitCompletion)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetFrameStatistics(
        &self,
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFrameStatistics)(
                windows_core::Interface::as_raw(self),
                statistics as _,
            )
        }
    }
    pub(crate) unsafe fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVisual)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSurfaceFactory<P0>(
        &self,
        renderingdevice: P0,
    ) -> windows_core::Result<IDCompositionSurfaceFactory>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurfaceFactory)(
                windows_core::Interface::as_raw(self),
                renderingdevice.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: DXGI_FORMAT,
        alphamode: DXGI_ALPHA_MODE,
    ) -> windows_core::Result<IDCompositionSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSurface)(
                windows_core::Interface::as_raw(self),
                width,
                height,
                pixelformat,
                alphamode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: DXGI_FORMAT,
        alphamode: DXGI_ALPHA_MODE,
    ) -> windows_core::Result<IDCompositionVirtualSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualSurface)(
                windows_core::Interface::as_raw(self),
                initialwidth,
                initialheight,
                pixelformat,
                alphamode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTranslateTransform(
        &self,
    ) -> windows_core::Result<IDCompositionTranslateTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranslateTransform)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateScaleTransform(
        &self,
    ) -> windows_core::Result<IDCompositionScaleTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScaleTransform)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateRotateTransform(
        &self,
    ) -> windows_core::Result<IDCompositionRotateTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRotateTransform)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSkewTransform(
        &self,
    ) -> windows_core::Result<IDCompositionSkewTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSkewTransform)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateMatrixTransform(
        &self,
    ) -> windows_core::Result<IDCompositionMatrixTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTransformGroup(
        &self,
        transforms: &[Option<IDCompositionTransform>],
    ) -> windows_core::Result<IDCompositionTransform> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformGroup)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(transforms.as_ptr()),
                transforms.len().try_into().unwrap(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTranslateTransform3D(
        &self,
    ) -> windows_core::Result<IDCompositionTranslateTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTranslateTransform3D)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateScaleTransform3D(
        &self,
    ) -> windows_core::Result<IDCompositionScaleTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScaleTransform3D)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateRotateTransform3D(
        &self,
    ) -> windows_core::Result<IDCompositionRotateTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRotateTransform3D)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateMatrixTransform3D(
        &self,
    ) -> windows_core::Result<IDCompositionMatrixTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMatrixTransform3D)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTransform3DGroup(
        &self,
        transforms3d: &[Option<IDCompositionTransform3D>],
    ) -> windows_core::Result<IDCompositionTransform3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransform3DGroup)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(transforms3d.as_ptr()),
                transforms3d.len().try_into().unwrap(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateEffectGroup(
        &self,
    ) -> windows_core::Result<IDCompositionEffectGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffectGroup)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateRectangleClip(
        &self,
    ) -> windows_core::Result<IDCompositionRectangleClip> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRectangleClip)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAnimation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDCompositionDevice2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForCommitCompletion:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> windows_core::HRESULT,
    pub CreateVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        DXGI_FORMAT,
        DXGI_ALPHA_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVirtualSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        DXGI_FORMAT,
        DXGI_ALPHA_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTranslateTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDCompositionDevice2_Impl: windows_core::IUnknownImpl {
    fn Commit(&self) -> windows_core::Result<()>;
    fn WaitForCommitCompletion(&self) -> windows_core::Result<()>;
    fn GetFrameStatistics(
        &self,
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> windows_core::Result<()>;
    fn CreateVisual(&self) -> windows_core::Result<IDCompositionVisual2>;
    fn CreateSurfaceFactory(
        &self,
        renderingdevice: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<IDCompositionSurfaceFactory>;
    fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: DXGI_FORMAT,
        alphamode: DXGI_ALPHA_MODE,
    ) -> windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: DXGI_FORMAT,
        alphamode: DXGI_ALPHA_MODE,
    ) -> windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateTranslateTransform(&self) -> windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(&self) -> windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(&self) -> windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(&self) -> windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(&self) -> windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(
        &self,
        transforms: *const Option<IDCompositionTransform>,
        elements: u32,
    ) -> windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(&self)
    -> windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(&self) -> windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(&self) -> windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(&self) -> windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(
        &self,
        transforms3d: *const Option<IDCompositionTransform3D>,
        elements: u32,
    ) -> windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(&self) -> windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(&self) -> windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(&self) -> windows_core::Result<IDCompositionAnimation>;
}
impl IDCompositionDevice2_Vtbl {
    pub const fn new<Identity: IDCompositionDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice2_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn WaitForCommitCompletion<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice2_Impl::WaitForCommitCompletion(this).into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionDevice2_Impl::GetFrameStatistics(
                    this,
                    core::mem::transmute_copy(&statistics),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateVisual<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            visual: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateVisual(this) {
                    Ok(ok__) => {
                        visual.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurfaceFactory<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            renderingdevice: *mut core::ffi::c_void,
            surfacefactory: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateSurfaceFactory(
                    this,
                    core::mem::transmute_copy(&renderingdevice),
                ) {
                    Ok(ok__) => {
                        surfacefactory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            width: u32,
            height: u32,
            pixelformat: DXGI_FORMAT,
            alphamode: DXGI_ALPHA_MODE,
            surface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateSurface(
                    this,
                    core::mem::transmute_copy(&width),
                    core::mem::transmute_copy(&height),
                    core::mem::transmute_copy(&pixelformat),
                    core::mem::transmute_copy(&alphamode),
                ) {
                    Ok(ok__) => {
                        surface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVirtualSurface<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            initialwidth: u32,
            initialheight: u32,
            pixelformat: DXGI_FORMAT,
            alphamode: DXGI_ALPHA_MODE,
            virtualsurface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateVirtualSurface(
                    this,
                    core::mem::transmute_copy(&initialwidth),
                    core::mem::transmute_copy(&initialheight),
                    core::mem::transmute_copy(&pixelformat),
                    core::mem::transmute_copy(&alphamode),
                ) {
                    Ok(ok__) => {
                        virtualsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTranslateTransform<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            translatetransform: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTranslateTransform(this) {
                    Ok(ok__) => {
                        translatetransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScaleTransform<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            scaletransform: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateScaleTransform(this) {
                    Ok(ok__) => {
                        scaletransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRotateTransform<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rotatetransform: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateRotateTransform(this) {
                    Ok(ok__) => {
                        rotatetransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSkewTransform<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            skewtransform: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateSkewTransform(this) {
                    Ok(ok__) => {
                        skewtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            matrixtransform: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateMatrixTransform(this) {
                    Ok(ok__) => {
                        matrixtransform.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransformGroup<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            transforms: *const *mut core::ffi::c_void,
            elements: u32,
            transformgroup: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTransformGroup(
                    this,
                    core::mem::transmute_copy(&transforms),
                    core::mem::transmute_copy(&elements),
                ) {
                    Ok(ok__) => {
                        transformgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTranslateTransform3D<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            translatetransform3d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTranslateTransform3D(this) {
                    Ok(ok__) => {
                        translatetransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateScaleTransform3D<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            scaletransform3d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateScaleTransform3D(this) {
                    Ok(ok__) => {
                        scaletransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRotateTransform3D<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rotatetransform3d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateRotateTransform3D(this) {
                    Ok(ok__) => {
                        rotatetransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMatrixTransform3D<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            matrixtransform3d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateMatrixTransform3D(this) {
                    Ok(ok__) => {
                        matrixtransform3d.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransform3DGroup<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            transforms3d: *const *mut core::ffi::c_void,
            elements: u32,
            transform3dgroup: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateTransform3DGroup(
                    this,
                    core::mem::transmute_copy(&transforms3d),
                    core::mem::transmute_copy(&elements),
                ) {
                    Ok(ok__) => {
                        transform3dgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEffectGroup<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            effectgroup: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateEffectGroup(this) {
                    Ok(ok__) => {
                        effectgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRectangleClip<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clip: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateRectangleClip(this) {
                    Ok(ok__) => {
                        clip.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAnimation<
            Identity: IDCompositionDevice2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            animation: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDCompositionDevice2_Impl::CreateAnimation(this) {
                    Ok(ok__) => {
                        animation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            CreateVisual: CreateVisual::<Identity, OFFSET>,
            CreateSurfaceFactory: CreateSurfaceFactory::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionDevice2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionDevice2 {}
windows_core::imp::define_interface!(
    IDCompositionEffect,
    IDCompositionEffect_Vtbl,
    0xec81b08f_bfcb_4e8d_b193_a915587999e8
);
windows_core::imp::interface_hierarchy!(IDCompositionEffect, windows_core::IUnknown);
#[repr(C)]
pub struct IDCompositionEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDCompositionEffect_Impl: windows_core::IUnknownImpl {}
impl IDCompositionEffect_Vtbl {
    pub const fn new<Identity: IDCompositionEffect_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionEffect {}
windows_core::imp::define_interface!(
    IDCompositionEffectGroup,
    IDCompositionEffectGroup_Vtbl,
    0xa7929a74_e6b2_4bd6_8b95_4040119ca34d
);
impl core::ops::Deref for IDCompositionEffectGroup {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionEffectGroup,
    windows_core::IUnknown,
    IDCompositionEffect
);
#[repr(C)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    SetOpacity: usize,
    SetOpacity2: usize,
    SetTransform3D: usize,
}
impl windows_core::RuntimeName for IDCompositionEffectGroup {}
windows_core::imp::define_interface!(
    IDCompositionMatrixTransform,
    IDCompositionMatrixTransform_Vtbl,
    0x16cdff07_c503_419c_83f2_0965c7af1fa6
);
impl core::ops::Deref for IDCompositionMatrixTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionMatrixTransform,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D,
    IDCompositionTransform
);
#[repr(C)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    SetMatrix: usize,
    SetMatrixElement: usize,
    SetMatrixElement2: usize,
}
impl windows_core::RuntimeName for IDCompositionMatrixTransform {}
windows_core::imp::define_interface!(
    IDCompositionMatrixTransform3D,
    IDCompositionMatrixTransform3D_Vtbl,
    0x4b3363f0_643b_41b7_b6e0_ccf22d34467c
);
impl core::ops::Deref for IDCompositionMatrixTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionMatrixTransform3D,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D
);
#[repr(C)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    SetMatrix: usize,
    SetMatrixElement: usize,
    SetMatrixElement2: usize,
}
impl windows_core::RuntimeName for IDCompositionMatrixTransform3D {}
windows_core::imp::define_interface!(
    IDCompositionRectangleClip,
    IDCompositionRectangleClip_Vtbl,
    0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2
);
impl core::ops::Deref for IDCompositionRectangleClip {
    type Target = IDCompositionClip;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionRectangleClip,
    windows_core::IUnknown,
    IDCompositionClip
);
#[repr(C)]
pub struct IDCompositionRectangleClip_Vtbl {
    pub base__: IDCompositionClip_Vtbl,
    SetLeft: usize,
    SetLeft2: usize,
    SetTop: usize,
    SetTop2: usize,
    SetRight: usize,
    SetRight2: usize,
    SetBottom: usize,
    SetBottom2: usize,
    SetTopLeftRadiusX: usize,
    SetTopLeftRadiusX2: usize,
    SetTopLeftRadiusY: usize,
    SetTopLeftRadiusY2: usize,
    SetTopRightRadiusX: usize,
    SetTopRightRadiusX2: usize,
    SetTopRightRadiusY: usize,
    SetTopRightRadiusY2: usize,
    SetBottomLeftRadiusX: usize,
    SetBottomLeftRadiusX2: usize,
    SetBottomLeftRadiusY: usize,
    SetBottomLeftRadiusY2: usize,
    SetBottomRightRadiusX: usize,
    SetBottomRightRadiusX2: usize,
    SetBottomRightRadiusY: usize,
    SetBottomRightRadiusY2: usize,
}
impl windows_core::RuntimeName for IDCompositionRectangleClip {}
windows_core::imp::define_interface!(
    IDCompositionRotateTransform,
    IDCompositionRotateTransform_Vtbl,
    0x641ed83c_ae96_46c5_90dc_32774cc5c6d5
);
impl core::ops::Deref for IDCompositionRotateTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionRotateTransform,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D,
    IDCompositionTransform
);
#[repr(C)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    SetAngle: usize,
    SetAngle2: usize,
    SetCenterX: usize,
    SetCenterX2: usize,
    SetCenterY: usize,
    SetCenterY2: usize,
}
impl windows_core::RuntimeName for IDCompositionRotateTransform {}
windows_core::imp::define_interface!(
    IDCompositionRotateTransform3D,
    IDCompositionRotateTransform3D_Vtbl,
    0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18
);
impl core::ops::Deref for IDCompositionRotateTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionRotateTransform3D,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D
);
#[repr(C)]
pub struct IDCompositionRotateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    SetAngle: usize,
    SetAngle2: usize,
    SetAxisX: usize,
    SetAxisX2: usize,
    SetAxisY: usize,
    SetAxisY2: usize,
    SetAxisZ: usize,
    SetAxisZ2: usize,
    SetCenterX: usize,
    SetCenterX2: usize,
    SetCenterY: usize,
    SetCenterY2: usize,
    SetCenterZ: usize,
    SetCenterZ2: usize,
}
impl windows_core::RuntimeName for IDCompositionRotateTransform3D {}
windows_core::imp::define_interface!(
    IDCompositionScaleTransform,
    IDCompositionScaleTransform_Vtbl,
    0x71fde914_40ef_45ef_bd51_68b037c339f9
);
impl core::ops::Deref for IDCompositionScaleTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionScaleTransform,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D,
    IDCompositionTransform
);
#[repr(C)]
pub struct IDCompositionScaleTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    SetScaleX: usize,
    SetScaleX2: usize,
    SetScaleY: usize,
    SetScaleY2: usize,
    SetCenterX: usize,
    SetCenterX2: usize,
    SetCenterY: usize,
    SetCenterY2: usize,
}
impl windows_core::RuntimeName for IDCompositionScaleTransform {}
windows_core::imp::define_interface!(
    IDCompositionScaleTransform3D,
    IDCompositionScaleTransform3D_Vtbl,
    0x2a9e9ead_364b_4b15_a7c4_a1997f78b389
);
impl core::ops::Deref for IDCompositionScaleTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionScaleTransform3D,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D
);
#[repr(C)]
pub struct IDCompositionScaleTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    SetScaleX: usize,
    SetScaleX2: usize,
    SetScaleY: usize,
    SetScaleY2: usize,
    SetScaleZ: usize,
    SetScaleZ2: usize,
    SetCenterX: usize,
    SetCenterX2: usize,
    SetCenterY: usize,
    SetCenterY2: usize,
    SetCenterZ: usize,
    SetCenterZ2: usize,
}
impl windows_core::RuntimeName for IDCompositionScaleTransform3D {}
windows_core::imp::define_interface!(
    IDCompositionSkewTransform,
    IDCompositionSkewTransform_Vtbl,
    0xe57aa735_dcdb_4c72_9c61_0591f58889ee
);
impl core::ops::Deref for IDCompositionSkewTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionSkewTransform,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D,
    IDCompositionTransform
);
#[repr(C)]
pub struct IDCompositionSkewTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    SetAngleX: usize,
    SetAngleX2: usize,
    SetAngleY: usize,
    SetAngleY2: usize,
    SetCenterX: usize,
    SetCenterX2: usize,
    SetCenterY: usize,
    SetCenterY2: usize,
}
impl windows_core::RuntimeName for IDCompositionSkewTransform {}
windows_core::imp::define_interface!(
    IDCompositionSurface,
    IDCompositionSurface_Vtbl,
    0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac
);
windows_core::imp::interface_hierarchy!(IDCompositionSurface, windows_core::IUnknown);
#[repr(C)]
pub struct IDCompositionSurface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    BeginDraw: usize,
    EndDraw: usize,
    SuspendDraw: usize,
    ResumeDraw: usize,
    Scroll: usize,
}
impl windows_core::RuntimeName for IDCompositionSurface {}
windows_core::imp::define_interface!(
    IDCompositionSurfaceFactory,
    IDCompositionSurfaceFactory_Vtbl,
    0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8
);
windows_core::imp::interface_hierarchy!(IDCompositionSurfaceFactory, windows_core::IUnknown);
#[repr(C)]
pub struct IDCompositionSurfaceFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateSurface: usize,
    CreateVirtualSurface: usize,
}
impl windows_core::RuntimeName for IDCompositionSurfaceFactory {}
windows_core::imp::define_interface!(
    IDCompositionTarget,
    IDCompositionTarget_Vtbl,
    0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89
);
windows_core::imp::interface_hierarchy!(IDCompositionTarget, windows_core::IUnknown);
impl IDCompositionTarget {
    pub(crate) unsafe fn SetRoot<P0>(&self, visual: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionVisual>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetRoot)(
                windows_core::Interface::as_raw(self),
                visual.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IDCompositionTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRoot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDCompositionTarget_Impl: windows_core::IUnknownImpl {
    fn SetRoot(&self, visual: windows_core::Ref<IDCompositionVisual>) -> windows_core::Result<()>;
}
impl IDCompositionTarget_Vtbl {
    pub const fn new<Identity: IDCompositionTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRoot<
            Identity: IDCompositionTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            visual: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionTarget_Impl::SetRoot(this, core::mem::transmute_copy(&visual)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetRoot: SetRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTarget {}
windows_core::imp::define_interface!(
    IDCompositionTransform,
    IDCompositionTransform_Vtbl,
    0xfd55faa7_37e0_4c20_95d2_9be45bc33f55
);
impl core::ops::Deref for IDCompositionTransform {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionTransform,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D
);
#[repr(C)]
pub struct IDCompositionTransform_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
}
pub trait IDCompositionTransform_Impl: IDCompositionTransform3D_Impl {}
impl IDCompositionTransform_Vtbl {
    pub const fn new<Identity: IDCompositionTransform_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: IDCompositionTransform3D_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTransform as windows_core::Interface>::IID
            || iid == &<IDCompositionEffect as windows_core::Interface>::IID
            || iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTransform {}
windows_core::imp::define_interface!(
    IDCompositionTransform3D,
    IDCompositionTransform3D_Vtbl,
    0x71185722_246b_41f2_aad1_0443f7f4bfc2
);
impl core::ops::Deref for IDCompositionTransform3D {
    type Target = IDCompositionEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionTransform3D,
    windows_core::IUnknown,
    IDCompositionEffect
);
#[repr(C)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
}
pub trait IDCompositionTransform3D_Impl: IDCompositionEffect_Impl {}
impl IDCompositionTransform3D_Vtbl {
    pub const fn new<Identity: IDCompositionTransform3D_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: IDCompositionEffect_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionTransform3D as windows_core::Interface>::IID
            || iid == &<IDCompositionEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionTransform3D {}
windows_core::imp::define_interface!(
    IDCompositionTranslateTransform,
    IDCompositionTranslateTransform_Vtbl,
    0x06791122_c6f0_417d_8323_269e987f5954
);
impl core::ops::Deref for IDCompositionTranslateTransform {
    type Target = IDCompositionTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionTranslateTransform,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D,
    IDCompositionTransform
);
#[repr(C)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    SetOffsetX: usize,
    SetOffsetX2: usize,
    SetOffsetY: usize,
    SetOffsetY2: usize,
}
impl windows_core::RuntimeName for IDCompositionTranslateTransform {}
windows_core::imp::define_interface!(
    IDCompositionTranslateTransform3D,
    IDCompositionTranslateTransform3D_Vtbl,
    0x91636d4b_9ba1_4532_aaf7_e3344994d788
);
impl core::ops::Deref for IDCompositionTranslateTransform3D {
    type Target = IDCompositionTransform3D;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionTranslateTransform3D,
    windows_core::IUnknown,
    IDCompositionEffect,
    IDCompositionTransform3D
);
#[repr(C)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    SetOffsetX: usize,
    SetOffsetX2: usize,
    SetOffsetY: usize,
    SetOffsetY2: usize,
    SetOffsetZ: usize,
    SetOffsetZ2: usize,
}
impl windows_core::RuntimeName for IDCompositionTranslateTransform3D {}
windows_core::imp::define_interface!(
    IDCompositionVirtualSurface,
    IDCompositionVirtualSurface_Vtbl,
    0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0
);
impl core::ops::Deref for IDCompositionVirtualSurface {
    type Target = IDCompositionSurface;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionVirtualSurface,
    windows_core::IUnknown,
    IDCompositionSurface
);
#[repr(C)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base__: IDCompositionSurface_Vtbl,
    Resize: usize,
    Trim: usize,
}
impl windows_core::RuntimeName for IDCompositionVirtualSurface {}
windows_core::imp::define_interface!(
    IDCompositionVisual,
    IDCompositionVisual_Vtbl,
    0x4d93059d_097b_4651_9a60_f0f25116e2f3
);
windows_core::imp::interface_hierarchy!(IDCompositionVisual, windows_core::IUnknown);
impl IDCompositionVisual {
    pub(crate) unsafe fn SetOffsetX<P0>(&self, animation: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffsetX)(
                windows_core::Interface::as_raw(self),
                animation.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetOffsetX2(&self, offsetx: f32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffsetX2)(
                windows_core::Interface::as_raw(self),
                offsetx,
            )
        }
    }
    pub(crate) unsafe fn SetOffsetY<P0>(&self, animation: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionAnimation>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffsetY)(
                windows_core::Interface::as_raw(self),
                animation.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetOffsetY2(&self, offsety: f32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetOffsetY2)(
                windows_core::Interface::as_raw(self),
                offsety,
            )
        }
    }
    pub(crate) unsafe fn SetTransform<P0>(&self, transform: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionTransform>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(
                windows_core::Interface::as_raw(self),
                transform.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetTransform2(
        &self,
        matrix: *const windows_numerics::Matrix3x2,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform2)(
                windows_core::Interface::as_raw(self),
                matrix,
            )
        }
    }
    pub(crate) unsafe fn SetTransformParent<P0>(&self, visual: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransformParent)(
                windows_core::Interface::as_raw(self),
                visual.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetEffect<P0>(&self, effect: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionEffect>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetEffect)(
                windows_core::Interface::as_raw(self),
                effect.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetBitmapInterpolationMode(
        &self,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetBitmapInterpolationMode)(
                windows_core::Interface::as_raw(self),
                interpolationmode,
            )
        }
    }
    pub(crate) unsafe fn SetBorderMode(
        &self,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetBorderMode)(
                windows_core::Interface::as_raw(self),
                bordermode,
            )
        }
    }
    pub(crate) unsafe fn SetClip<P0>(&self, clip: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDCompositionClip>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetClip)(
                windows_core::Interface::as_raw(self),
                clip.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn SetClip2(&self, rect: *const D2D_RECT_F) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetClip2)(
                windows_core::Interface::as_raw(self),
                rect,
            )
        }
    }
    pub(crate) unsafe fn SetContent<P0>(&self, content: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContent)(
                windows_core::Interface::as_raw(self),
                content.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn AddVisual<P0, P2>(
        &self,
        visual: P0,
        insertabove: bool,
        referencevisual: P2,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
        P2: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddVisual)(
                windows_core::Interface::as_raw(self),
                visual.param().abi(),
                insertabove.into(),
                referencevisual.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn RemoveVisual<P0>(&self, visual: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveVisual)(
                windows_core::Interface::as_raw(self),
                visual.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn RemoveAllVisuals(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAllVisuals)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn SetCompositeMode(
        &self,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetCompositeMode)(
                windows_core::Interface::as_raw(self),
                compositemode,
            )
        }
    }
}
#[repr(C)]
pub struct IDCompositionVisual_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetOffsetX2:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetOffsetY2:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTransform2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_numerics::Matrix3x2,
    ) -> windows_core::HRESULT,
    pub SetTransformParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> windows_core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DCOMPOSITION_BORDER_MODE,
    ) -> windows_core::HRESULT,
    pub SetClip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetClip2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveVisual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveAllVisuals:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DCOMPOSITION_COMPOSITE_MODE,
    ) -> windows_core::HRESULT,
}
pub trait IDCompositionVisual_Impl: windows_core::IUnknownImpl {
    fn SetOffsetX(
        &self,
        animation: windows_core::Ref<IDCompositionAnimation>,
    ) -> windows_core::Result<()>;
    fn SetOffsetX2(&self, offsetx: f32) -> windows_core::Result<()>;
    fn SetOffsetY(
        &self,
        animation: windows_core::Ref<IDCompositionAnimation>,
    ) -> windows_core::Result<()>;
    fn SetOffsetY2(&self, offsety: f32) -> windows_core::Result<()>;
    fn SetTransform(
        &self,
        transform: windows_core::Ref<IDCompositionTransform>,
    ) -> windows_core::Result<()>;
    fn SetTransform2(&self, matrix: *const windows_numerics::Matrix3x2)
    -> windows_core::Result<()>;
    fn SetTransformParent(
        &self,
        visual: windows_core::Ref<IDCompositionVisual>,
    ) -> windows_core::Result<()>;
    fn SetEffect(&self, effect: windows_core::Ref<IDCompositionEffect>)
    -> windows_core::Result<()>;
    fn SetBitmapInterpolationMode(
        &self,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> windows_core::Result<()>;
    fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> windows_core::Result<()>;
    fn SetClip(&self, clip: windows_core::Ref<IDCompositionClip>) -> windows_core::Result<()>;
    fn SetClip2(&self, rect: *const D2D_RECT_F) -> windows_core::Result<()>;
    fn SetContent(
        &self,
        content: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn AddVisual(
        &self,
        visual: windows_core::Ref<IDCompositionVisual>,
        insertabove: windows_core::BOOL,
        referencevisual: windows_core::Ref<IDCompositionVisual>,
    ) -> windows_core::Result<()>;
    fn RemoveVisual(
        &self,
        visual: windows_core::Ref<IDCompositionVisual>,
    ) -> windows_core::Result<()>;
    fn RemoveAllVisuals(&self) -> windows_core::Result<()>;
    fn SetCompositeMode(
        &self,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> windows_core::Result<()>;
}
impl IDCompositionVisual_Vtbl {
    pub const fn new<Identity: IDCompositionVisual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOffsetX<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            animation: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetX(this, core::mem::transmute_copy(&animation))
                    .into()
            }
        }
        unsafe extern "system" fn SetOffsetX2<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            offsetx: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetX2(this, core::mem::transmute_copy(&offsetx))
                    .into()
            }
        }
        unsafe extern "system" fn SetOffsetY<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            animation: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetY(this, core::mem::transmute_copy(&animation))
                    .into()
            }
        }
        unsafe extern "system" fn SetOffsetY2<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            offsety: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetOffsetY2(this, core::mem::transmute_copy(&offsety))
                    .into()
            }
        }
        unsafe extern "system" fn SetTransform<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            transform: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetTransform(this, core::mem::transmute_copy(&transform))
                    .into()
            }
        }
        unsafe extern "system" fn SetTransform2<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            matrix: *const windows_numerics::Matrix3x2,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetTransform2(this, core::mem::transmute_copy(&matrix))
                    .into()
            }
        }
        unsafe extern "system" fn SetTransformParent<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            visual: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetTransformParent(
                    this,
                    core::mem::transmute_copy(&visual),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetEffect<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            effect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetEffect(this, core::mem::transmute_copy(&effect)).into()
            }
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetBitmapInterpolationMode(
                    this,
                    core::mem::transmute_copy(&interpolationmode),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetBorderMode<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bordermode: DCOMPOSITION_BORDER_MODE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetBorderMode(
                    this,
                    core::mem::transmute_copy(&bordermode),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetClip<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clip: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetClip(this, core::mem::transmute_copy(&clip)).into()
            }
        }
        unsafe extern "system" fn SetClip2<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rect: *const D2D_RECT_F,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetClip2(this, core::mem::transmute_copy(&rect)).into()
            }
        }
        unsafe extern "system" fn SetContent<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            content: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetContent(this, core::mem::transmute_copy(&content))
                    .into()
            }
        }
        unsafe extern "system" fn AddVisual<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            visual: *mut core::ffi::c_void,
            insertabove: windows_core::BOOL,
            referencevisual: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::AddVisual(
                    this,
                    core::mem::transmute_copy(&visual),
                    core::mem::transmute_copy(&insertabove),
                    core::mem::transmute_copy(&referencevisual),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveVisual<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            visual: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::RemoveVisual(this, core::mem::transmute_copy(&visual))
                    .into()
            }
        }
        unsafe extern "system" fn RemoveAllVisuals<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::RemoveAllVisuals(this).into()
            }
        }
        unsafe extern "system" fn SetCompositeMode<
            Identity: IDCompositionVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            compositemode: DCOMPOSITION_COMPOSITE_MODE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionVisual_Impl::SetCompositeMode(
                    this,
                    core::mem::transmute_copy(&compositemode),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOffsetX: SetOffsetX::<Identity, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            SetTransform2: SetTransform2::<Identity, OFFSET>,
            SetTransformParent: SetTransformParent::<Identity, OFFSET>,
            SetEffect: SetEffect::<Identity, OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Identity, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, OFFSET>,
            SetClip: SetClip::<Identity, OFFSET>,
            SetClip2: SetClip2::<Identity, OFFSET>,
            SetContent: SetContent::<Identity, OFFSET>,
            AddVisual: AddVisual::<Identity, OFFSET>,
            RemoveVisual: RemoveVisual::<Identity, OFFSET>,
            RemoveAllVisuals: RemoveAllVisuals::<Identity, OFFSET>,
            SetCompositeMode: SetCompositeMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionVisual as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionVisual {}
windows_core::imp::define_interface!(
    IDCompositionVisual2,
    IDCompositionVisual2_Vtbl,
    0xe8de1639_4331_4b26_bc5f_6a321d347a85
);
impl core::ops::Deref for IDCompositionVisual2 {
    type Target = IDCompositionVisual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDCompositionVisual2,
    windows_core::IUnknown,
    IDCompositionVisual
);
#[repr(C)]
pub struct IDCompositionVisual2_Vtbl {
    pub base__: IDCompositionVisual_Vtbl,
    SetOpacityMode: usize,
    SetBackFaceVisibility: usize,
}
impl windows_core::RuntimeName for IDCompositionVisual2 {}
windows_core::imp::define_interface!(
    IDXGIAdapter,
    IDXGIAdapter_Vtbl,
    0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0
);
impl core::ops::Deref for IDXGIAdapter {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub(crate) unsafe fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: *mut Option<IDXGIOutput>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EnumOutputs)(
                windows_core::Interface::as_raw(self),
                output,
                core::mem::transmute(ppoutput),
            )
        }
    }
    pub(crate) unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            )
        }
    }
    pub(crate) unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const windows_core::GUID,
    ) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckInterfaceSupport)(
                windows_core::Interface::as_raw(self),
                interfacename,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_ADAPTER_DESC,
    ) -> windows_core::HRESULT,
    pub CheckInterfaceSupport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut i64,
    ) -> windows_core::HRESULT,
}
pub trait IDXGIAdapter_Impl: IDXGIObject_Impl {
    fn EnumOutputs(
        &self,
        output: u32,
        ppoutput: windows_core::OutRef<IDXGIOutput>,
    ) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::Result<()>;
    fn CheckInterfaceSupport(
        &self,
        interfacename: *const windows_core::GUID,
    ) -> windows_core::Result<i64>;
}
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: IDXGIAdapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumOutputs<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            output: u32,
            ppoutput: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter_Impl::EnumOutputs(
                    this,
                    core::mem::transmute_copy(&output),
                    core::mem::transmute_copy(&ppoutput),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *mut DXGI_ADAPTER_DESC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<
            Identity: IDXGIAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            interfacename: *const windows_core::GUID,
            pumdversion: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::CheckInterfaceSupport(
                    this,
                    core::mem::transmute_copy(&interfacename),
                ) {
                    Ok(ok__) => {
                        pumdversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID
            || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter {}
windows_core::imp::define_interface!(
    IDXGIDevice,
    IDXGIDevice_Vtbl,
    0x54ec77fa_1377_44e6_8c32_88fd5f44c84c
);
impl core::ops::Deref for IDXGIDevice {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub(crate) unsafe fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: DXGI_USAGE,
        psharedresource: Option<*const DXGI_SHARED_RESOURCE>,
        ppsurface: *mut Option<IDXGISurface>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateSurface)(
                windows_core::Interface::as_raw(self),
                pdesc,
                numsurfaces,
                usage,
                psharedresource.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(ppsurface),
            )
        }
    }
    pub(crate) unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const Option<windows_core::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).QueryResourceResidency)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppresources),
                presidencystatus as _,
                numresources,
            )
        }
    }
    pub(crate) unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetGPUThreadPriority)(
                windows_core::Interface::as_raw(self),
                priority,
            )
        }
    }
    pub(crate) unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_SURFACE_DESC,
        u32,
        DXGI_USAGE,
        *const DXGI_SHARED_RESOURCE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub QueryResourceResidency: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        *mut DXGI_RESIDENCY,
        u32,
    ) -> windows_core::HRESULT,
    pub SetGPUThreadPriority:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetGPUThreadPriority:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IDXGIDevice_Impl: IDXGIObject_Impl {
    fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: DXGI_USAGE,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut Option<IDXGISurface>,
    ) -> windows_core::Result<()>;
    fn QueryResourceResidency(
        &self,
        ppresources: *const Option<windows_core::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
}
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: IDXGIDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapter<Identity: IDXGIDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            padapter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetAdapter(this) {
                    Ok(ok__) => {
                        padapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDXGIDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *const DXGI_SURFACE_DESC,
            numsurfaces: u32,
            usage: DXGI_USAGE,
            psharedresource: *const DXGI_SHARED_RESOURCE,
            ppsurface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::CreateSurface(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&numsurfaces),
                    core::mem::transmute_copy(&usage),
                    core::mem::transmute_copy(&psharedresource),
                    core::mem::transmute_copy(&ppsurface),
                )
                .into()
            }
        }
        unsafe extern "system" fn QueryResourceResidency<
            Identity: IDXGIDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppresources: *const *mut core::ffi::c_void,
            presidencystatus: *mut DXGI_RESIDENCY,
            numresources: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::QueryResourceResidency(
                    this,
                    core::mem::transmute_copy(&ppresources),
                    core::mem::transmute_copy(&presidencystatus),
                    core::mem::transmute_copy(&numresources),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<
            Identity: IDXGIDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            priority: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority))
                    .into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<
            Identity: IDXGIDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppriority: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice as windows_core::Interface>::IID
            || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDevice {}
windows_core::imp::define_interface!(
    IDXGIDeviceSubObject,
    IDXGIDeviceSubObject_Vtbl,
    0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6
);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
#[repr(C)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    GetDevice: usize,
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(
    IDXGIFactory,
    IDXGIFactory_Vtbl,
    0x7b7166ec_21c7_44ae_b21a_c9ae321ae369
);
impl core::ops::Deref for IDXGIFactory {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory, windows_core::IUnknown, IDXGIObject);
#[repr(C)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    EnumAdapters: usize,
    MakeWindowAssociation: usize,
    GetWindowAssociation: usize,
    CreateSwapChain: usize,
    CreateSoftwareAdapter: usize,
}
impl windows_core::RuntimeName for IDXGIFactory {}
windows_core::imp::define_interface!(
    IDXGIFactory1,
    IDXGIFactory1_Vtbl,
    0x770aae78_f26f_4dba_a829_253c83d1b387
);
impl core::ops::Deref for IDXGIFactory1 {
    type Target = IDXGIFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGIFactory1,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIFactory
);
#[repr(C)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    EnumAdapters1: usize,
    IsCurrent: usize,
}
impl windows_core::RuntimeName for IDXGIFactory1 {}
windows_core::imp::define_interface!(
    IDXGIFactory2,
    IDXGIFactory2_Vtbl,
    0x50c83a1c_e072_4c48_87b0_3630fa36a6d0
);
impl core::ops::Deref for IDXGIFactory2 {
    type Target = IDXGIFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGIFactory2,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIFactory,
    IDXGIFactory1
);
impl IDXGIFactory2 {
    pub(crate) unsafe fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn CreateSwapChainForHwnd<P0, P4>(
        &self,
        pdevice: P0,
        hwnd: HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>,
        prestricttooutput: P4,
    ) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForHwnd)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                hwnd,
                pdesc,
                pfullscreendesc.unwrap_or(core::mem::zeroed()) as _,
                prestricttooutput.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateSwapChainForCoreWindow<P0, P1, P3>(
        &self,
        pdevice: P0,
        pwindow: P1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: P3,
    ) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCoreWindow)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                pwindow.param().abi(),
                pdesc,
                prestricttooutput.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetSharedResourceAdapterLuid(
        &self,
        hresource: HANDLE,
    ) -> windows_core::Result<LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedResourceAdapterLuid)(
                windows_core::Interface::as_raw(self),
                hresource,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn RegisterStereoStatusWindow(
        &self,
        windowhandle: HWND,
        wmsg: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(
                windows_core::Interface::as_raw(self),
                windowhandle,
                wmsg,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn RegisterStereoStatusEvent(
        &self,
        hevent: HANDLE,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(
                windows_core::Interface::as_raw(self),
                hevent,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterStereoStatus)(
                windows_core::Interface::as_raw(self),
                dwcookie,
            );
        }
    }
    pub(crate) unsafe fn RegisterOcclusionStatusWindow(
        &self,
        windowhandle: HWND,
        wmsg: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(
                windows_core::Interface::as_raw(self),
                windowhandle,
                wmsg,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn RegisterOcclusionStatusEvent(
        &self,
        hevent: HANDLE,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(
                windows_core::Interface::as_raw(self),
                hevent,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(
                windows_core::Interface::as_raw(self),
                dwcookie,
            );
        }
    }
    pub(crate) unsafe fn CreateSwapChainForComposition<P0, P2>(
        &self,
        pdevice: P0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: P2,
    ) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForComposition)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                pdesc,
                prestricttooutput.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        HWND,
        *const DXGI_SWAP_CHAIN_DESC1,
        *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const DXGI_SWAP_CHAIN_DESC1,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut LUID,
    ) -> windows_core::HRESULT,
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub UnregisterStereoStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub CreateSwapChainForComposition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const DXGI_SWAP_CHAIN_DESC1,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDXGIFactory2 {}
windows_core::imp::define_interface!(
    IDXGIObject,
    IDXGIObject_Vtbl,
    0xaec22fb8_76f3_4639_9be0_28eb43a67a2e
);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
impl IDXGIObject {
    pub(crate) unsafe fn SetPrivateData(
        &self,
        name: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                name,
                datasize,
                pdata,
            )
        }
    }
    pub(crate) unsafe fn SetPrivateDataInterface<P1>(
        &self,
        name: *const windows_core::GUID,
        punknown: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                name,
                punknown.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetPrivateData(
        &self,
        name: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrivateData)(
                windows_core::Interface::as_raw(self),
                name,
                pdatasize as _,
                pdata as _,
            )
        }
    }
    pub(crate) unsafe fn GetParent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetParent)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDXGIObject_Impl: windows_core::IUnknownImpl {
    fn SetPrivateData(
        &self,
        name: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        name: *const windows_core::GUID,
        punknown: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn GetPrivateData(
        &self,
        name: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetParent(
        &self,
        riid: *const windows_core::GUID,
        ppparent: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: IDXGIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrivateData<
            Identity: IDXGIObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&name),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: IDXGIObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *const windows_core::GUID,
            punknown: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&name),
                    core::mem::transmute_copy(&punknown),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPrivateData<
            Identity: IDXGIObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *const windows_core::GUID,
            pdatasize: *mut u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetPrivateData(
                    this,
                    core::mem::transmute_copy(&name),
                    core::mem::transmute_copy(&pdatasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDXGIObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppparent: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetParent(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppparent),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(
    IDXGIOutput,
    IDXGIOutput_Vtbl,
    0xae02eedb_c735_4690_8d52_5a8dc20213aa
);
impl core::ops::Deref for IDXGIOutput {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput, windows_core::IUnknown, IDXGIObject);
impl IDXGIOutput {
    pub(crate) unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            )
        }
    }
    pub(crate) unsafe fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: Option<*mut DXGI_MODE_DESC>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDisplayModeList)(
                windows_core::Interface::as_raw(self),
                enumformat,
                flags,
                pnummodes as _,
                pdesc.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn FindClosestMatchingMode<P2>(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: P2,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FindClosestMatchingMode)(
                windows_core::Interface::as_raw(self),
                pmodetomatch,
                pclosestmatch as _,
                pconcerneddevice.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn WaitForVBlank(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub(crate) unsafe fn TakeOwnership<P0>(
        &self,
        pdevice: P0,
        exclusive: bool,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).TakeOwnership)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                exclusive.into(),
            )
        }
    }
    pub(crate) unsafe fn ReleaseOwnership(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseOwnership)(
                windows_core::Interface::as_raw(self),
            );
        }
    }
    pub(crate) unsafe fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetGammaControlCapabilities)(
                windows_core::Interface::as_raw(self),
                pgammacaps as _,
            )
        }
    }
    pub(crate) unsafe fn SetGammaControl(
        &self,
        parray: *const DXGI_GAMMA_CONTROL,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetGammaControl)(
                windows_core::Interface::as_raw(self),
                parray,
            )
        }
    }
    pub(crate) unsafe fn GetGammaControl(
        &self,
        parray: *mut DXGI_GAMMA_CONTROL,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetGammaControl)(
                windows_core::Interface::as_raw(self),
                parray as _,
            )
        }
    }
    pub(crate) unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDisplaySurface)(
                windows_core::Interface::as_raw(self),
                pscanoutsurface.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetDisplaySurfaceData)(
                windows_core::Interface::as_raw(self),
                pdestination.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFrameStatistics)(
                windows_core::Interface::as_raw(self),
                pstats as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_OUTPUT_DESC,
    ) -> windows_core::HRESULT,
    pub GetDisplayModeList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_FORMAT,
        u32,
        *mut u32,
        *mut DXGI_MODE_DESC,
    ) -> windows_core::HRESULT,
    pub FindClosestMatchingMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_MODE_DESC,
        *mut DXGI_MODE_DESC,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub ReleaseOwnership: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetGammaControlCapabilities: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> windows_core::HRESULT,
    pub SetGammaControl: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_GAMMA_CONTROL,
    ) -> windows_core::HRESULT,
    pub GetGammaControl: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_GAMMA_CONTROL,
    ) -> windows_core::HRESULT,
    pub SetDisplaySurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_FRAME_STATISTICS,
    ) -> windows_core::HRESULT,
}
pub trait IDXGIOutput_Impl: IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::Result<()>;
    fn GetDisplayModeList(
        &self,
        enumformat: DXGI_FORMAT,
        flags: u32,
        pnummodes: *mut u32,
        pdesc: *mut DXGI_MODE_DESC,
    ) -> windows_core::Result<()>;
    fn FindClosestMatchingMode(
        &self,
        pmodetomatch: *const DXGI_MODE_DESC,
        pclosestmatch: *mut DXGI_MODE_DESC,
        pconcerneddevice: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn WaitForVBlank(&self) -> windows_core::Result<()>;
    fn TakeOwnership(
        &self,
        pdevice: windows_core::Ref<windows_core::IUnknown>,
        exclusive: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn ReleaseOwnership(&self);
    fn GetGammaControlCapabilities(
        &self,
        pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> windows_core::Result<()>;
    fn SetGammaControl(&self, parray: *const DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn GetGammaControl(&self, parray: *mut DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn SetDisplaySurface(
        &self,
        pscanoutsurface: windows_core::Ref<IDXGISurface>,
    ) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData(
        &self,
        pdestination: windows_core::Ref<IDXGISurface>,
    ) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
}
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: IDXGIOutput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutput_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *mut DXGI_OUTPUT_DESC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeList<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            enumformat: DXGI_FORMAT,
            flags: u32,
            pnummodes: *mut u32,
            pdesc: *mut DXGI_MODE_DESC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplayModeList(
                    this,
                    core::mem::transmute_copy(&enumformat),
                    core::mem::transmute_copy(&flags),
                    core::mem::transmute_copy(&pnummodes),
                    core::mem::transmute_copy(&pdesc),
                )
                .into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pmodetomatch: *const DXGI_MODE_DESC,
            pclosestmatch: *mut DXGI_MODE_DESC,
            pconcerneddevice: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::FindClosestMatchingMode(
                    this,
                    core::mem::transmute_copy(&pmodetomatch),
                    core::mem::transmute_copy(&pclosestmatch),
                    core::mem::transmute_copy(&pconcerneddevice),
                )
                .into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDXGIOutput_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::WaitForVBlank(this).into()
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdevice: *mut core::ffi::c_void,
            exclusive: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::TakeOwnership(
                    this,
                    core::mem::transmute_copy(&pdevice),
                    core::mem::transmute_copy(&exclusive),
                )
                .into()
            }
        }
        unsafe extern "system" fn ReleaseOwnership<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::ReleaseOwnership(this);
            }
        }
        unsafe extern "system" fn GetGammaControlCapabilities<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pgammacaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControlCapabilities(
                    this,
                    core::mem::transmute_copy(&pgammacaps),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetGammaControl<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parray: *const DXGI_GAMMA_CONTROL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn GetGammaControl<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parray: *mut DXGI_GAMMA_CONTROL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn SetDisplaySurface<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pscanoutsurface: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetDisplaySurface(
                    this,
                    core::mem::transmute_copy(&pscanoutsurface),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdestination: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplaySurfaceData(
                    this,
                    core::mem::transmute_copy(&pdestination),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<
            Identity: IDXGIOutput_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pstats: *mut DXGI_FRAME_STATISTICS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats))
                    .into()
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput as windows_core::Interface>::IID
            || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIOutput {}
windows_core::imp::define_interface!(
    IDXGISurface,
    IDXGISurface_Vtbl,
    0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec
);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISurface,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject
);
#[repr(C)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    GetDesc: usize,
    Map: usize,
    Unmap: usize,
}
impl windows_core::RuntimeName for IDXGISurface {}
windows_core::imp::define_interface!(
    IDXGISwapChain,
    IDXGISwapChain_Vtbl,
    0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a
);
impl core::ops::Deref for IDXGISwapChain {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISwapChain,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject
);
impl IDXGISwapChain {
    pub(crate) unsafe fn Present(&self, syncinterval: u32, flags: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Present)(
                windows_core::Interface::as_raw(self),
                syncinterval,
                flags,
            )
        }
    }
    pub(crate) unsafe fn GetBuffer<T>(&self, buffer: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetBuffer)(
                windows_core::Interface::as_raw(self),
                buffer,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetFullscreenState<P1>(
        &self,
        fullscreen: bool,
        ptarget: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFullscreenState)(
                windows_core::Interface::as_raw(self),
                fullscreen.into(),
                ptarget.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetFullscreenState(
        &self,
        pfullscreen: Option<*mut windows_core::BOOL>,
        pptarget: Option<*mut Option<IDXGIOutput>>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFullscreenState)(
                windows_core::Interface::as_raw(self),
                pfullscreen.unwrap_or(core::mem::zeroed()) as _,
                pptarget.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub(crate) unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            )
        }
    }
    pub(crate) unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResizeBuffers)(
                windows_core::Interface::as_raw(self),
                buffercount,
                width,
                height,
                newformat,
                swapchainflags,
            )
        }
    }
    pub(crate) unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ResizeTarget)(
                windows_core::Interface::as_raw(self),
                pnewtargetparameters,
            )
        }
    }
    pub(crate) unsafe fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainingOutput)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFrameStatistics)(
                windows_core::Interface::as_raw(self),
                pstats as _,
            )
        }
    }
    pub(crate) unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SWAP_CHAIN_DESC,
    ) -> windows_core::HRESULT,
    pub ResizeBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        DXGI_FORMAT,
        u32,
    ) -> windows_core::HRESULT,
    pub ResizeTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_MODE_DESC,
    ) -> windows_core::HRESULT,
    pub GetContainingOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_FRAME_STATISTICS,
    ) -> windows_core::HRESULT,
    pub GetLastPresentCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDXGISwapChain {}
windows_core::imp::define_interface!(
    IDXGISwapChain1,
    IDXGISwapChain1_Vtbl,
    0x790a45f7_0d42_4876_983a_0a55cfe6f4aa
);
impl core::ops::Deref for IDXGISwapChain1 {
    type Target = IDXGISwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISwapChain1,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject,
    IDXGISwapChain
);
impl IDXGISwapChain1 {
    pub(crate) unsafe fn GetDesc1(
        &self,
        pdesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesc1)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            )
        }
    }
    pub(crate) unsafe fn GetFullscreenDesc(
        &self,
        pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFullscreenDesc)(
                windows_core::Interface::as_raw(self),
                pdesc as _,
            )
        }
    }
    pub(crate) unsafe fn GetHwnd(&self) -> windows_core::Result<HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwnd)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetCoreWindow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetCoreWindow)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: u32,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Present1)(
                windows_core::Interface::as_raw(self),
                syncinterval,
                presentflags,
                ppresentparameters,
            )
        }
    }
    pub(crate) unsafe fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictToOutput)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetBackgroundColor(
        &self,
        pcolor: *const DXGI_RGBA,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackgroundColor)(
                windows_core::Interface::as_raw(self),
                pcolor,
            )
        }
    }
    pub(crate) unsafe fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetRotation)(
                windows_core::Interface::as_raw(self),
                rotation,
            )
        }
    }
    pub(crate) unsafe fn GetRotation(&self) -> windows_core::Result<DXGI_MODE_ROTATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> windows_core::HRESULT,
    pub GetFullscreenDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> windows_core::HRESULT,
    pub GetHwnd:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut HWND) -> windows_core::HRESULT,
    pub GetCoreWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Present1: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const DXGI_PRESENT_PARAMETERS,
    ) -> windows_core::HRESULT,
    pub IsTemporaryMonoSupported:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetRestrictToOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_RGBA,
    ) -> windows_core::HRESULT,
    pub GetBackgroundColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_RGBA) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_MODE_ROTATION,
    ) -> windows_core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_MODE_ROTATION,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDXGISwapChain1 {}
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
    IMFDXGIBuffer,
    IMFDXGIBuffer_Vtbl,
    0xe7174cfa_1c9e_48b1_8866_626226bfc258
);
windows_core::imp::interface_hierarchy!(IMFDXGIBuffer, windows_core::IUnknown);
impl IMFDXGIBuffer {
    pub(crate) unsafe fn GetResource<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetResource)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetSubresourceIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubresourceIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetUnknown<T>(
        &self,
        guid: *const windows_core::GUID,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetUnknown)(
                windows_core::Interface::as_raw(self),
                guid,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn SetUnknown<P1>(
        &self,
        guid: *const windows_core::GUID,
        punkdata: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnknown)(
                windows_core::Interface::as_raw(self),
                guid,
                punkdata.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IMFDXGIBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSubresourceIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetUnknown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetUnknown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IMFDXGIBuffer_Impl: windows_core::IUnknownImpl {
    fn GetResource(
        &self,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetSubresourceIndex(&self) -> windows_core::Result<u32>;
    fn GetUnknown(
        &self,
        guid: *const windows_core::GUID,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetUnknown(
        &self,
        guid: *const windows_core::GUID,
        punkdata: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
}
impl IMFDXGIBuffer_Vtbl {
    pub const fn new<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppvobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::GetResource(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetSubresourceIndex<
            Identity: IMFDXGIBuffer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pusubresource: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDXGIBuffer_Impl::GetSubresourceIndex(this) {
                    Ok(ok__) => {
                        pusubresource.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUnknown<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppvobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::GetUnknown(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetUnknown<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            punkdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::SetUnknown(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&punkdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResource: GetResource::<Identity, OFFSET>,
            GetSubresourceIndex: GetSubresourceIndex::<Identity, OFFSET>,
            GetUnknown: GetUnknown::<Identity, OFFSET>,
            SetUnknown: SetUnknown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDXGIBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFDXGIBuffer {}
windows_core::imp::define_interface!(
    IMFDXGIDeviceManager,
    IMFDXGIDeviceManager_Vtbl,
    0xeb533d5d_2db6_40f8_97a9_494692014f07
);
windows_core::imp::interface_hierarchy!(IMFDXGIDeviceManager, windows_core::IUnknown);
impl IMFDXGIDeviceManager {
    pub(crate) unsafe fn CloseDeviceHandle(&self, hdevice: HANDLE) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CloseDeviceHandle)(
                windows_core::Interface::as_raw(self),
                hdevice,
            )
        }
    }
    pub(crate) unsafe fn GetVideoService<T>(&self, hdevice: HANDLE) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoService)(
                windows_core::Interface::as_raw(self),
                hdevice,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn LockDevice<T>(
        &self,
        hdevice: HANDLE,
        fblock: bool,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).LockDevice)(
                windows_core::Interface::as_raw(self),
                hdevice,
                &T::IID,
                &mut result__,
                fblock.into(),
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn OpenDeviceHandle(&self) -> windows_core::Result<HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDeviceHandle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn ResetDevice<P0>(
        &self,
        punkdevice: P0,
        resettoken: u32,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResetDevice)(
                windows_core::Interface::as_raw(self),
                punkdevice.param().abi(),
                resettoken,
            )
        }
    }
    pub(crate) unsafe fn TestDevice(&self, hdevice: HANDLE) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).TestDevice)(
                windows_core::Interface::as_raw(self),
                hdevice,
            )
        }
    }
    pub(crate) unsafe fn UnlockDevice(
        &self,
        hdevice: HANDLE,
        fsavestate: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).UnlockDevice)(
                windows_core::Interface::as_raw(self),
                hdevice,
                fsavestate.into(),
            )
        }
    }
}
#[repr(C)]
pub struct IMFDXGIDeviceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CloseDeviceHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, HANDLE) -> windows_core::HRESULT,
    pub GetVideoService: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub OpenDeviceHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut HANDLE) -> windows_core::HRESULT,
    pub ResetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
    pub TestDevice:
        unsafe extern "system" fn(*mut core::ffi::c_void, HANDLE) -> windows_core::HRESULT,
    pub UnlockDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IMFDXGIDeviceManager_Impl: windows_core::IUnknownImpl {
    fn CloseDeviceHandle(&self, hdevice: HANDLE) -> windows_core::Result<()>;
    fn GetVideoService(
        &self,
        hdevice: HANDLE,
        riid: *const windows_core::GUID,
        ppservice: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn LockDevice(
        &self,
        hdevice: HANDLE,
        riid: *const windows_core::GUID,
        ppunkdevice: *mut *mut core::ffi::c_void,
        fblock: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn OpenDeviceHandle(&self) -> windows_core::Result<HANDLE>;
    fn ResetDevice(
        &self,
        punkdevice: windows_core::Ref<windows_core::IUnknown>,
        resettoken: u32,
    ) -> windows_core::Result<()>;
    fn TestDevice(&self, hdevice: HANDLE) -> windows_core::Result<()>;
    fn UnlockDevice(
        &self,
        hdevice: HANDLE,
        fsavestate: windows_core::BOOL,
    ) -> windows_core::Result<()>;
}
impl IMFDXGIDeviceManager_Vtbl {
    pub const fn new<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloseDeviceHandle<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hdevice: HANDLE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::CloseDeviceHandle(
                    this,
                    core::mem::transmute_copy(&hdevice),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetVideoService<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hdevice: HANDLE,
            riid: *const windows_core::GUID,
            ppservice: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::GetVideoService(
                    this,
                    core::mem::transmute_copy(&hdevice),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppservice),
                )
                .into()
            }
        }
        unsafe extern "system" fn LockDevice<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hdevice: HANDLE,
            riid: *const windows_core::GUID,
            ppunkdevice: *mut *mut core::ffi::c_void,
            fblock: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::LockDevice(
                    this,
                    core::mem::transmute_copy(&hdevice),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppunkdevice),
                    core::mem::transmute_copy(&fblock),
                )
                .into()
            }
        }
        unsafe extern "system" fn OpenDeviceHandle<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            phdevice: *mut HANDLE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDXGIDeviceManager_Impl::OpenDeviceHandle(this) {
                    Ok(ok__) => {
                        phdevice.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetDevice<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            punkdevice: *mut core::ffi::c_void,
            resettoken: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::ResetDevice(
                    this,
                    core::mem::transmute_copy(&punkdevice),
                    core::mem::transmute_copy(&resettoken),
                )
                .into()
            }
        }
        unsafe extern "system" fn TestDevice<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hdevice: HANDLE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::TestDevice(this, core::mem::transmute_copy(&hdevice))
                    .into()
            }
        }
        unsafe extern "system" fn UnlockDevice<
            Identity: IMFDXGIDeviceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hdevice: HANDLE,
            fsavestate: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::UnlockDevice(
                    this,
                    core::mem::transmute_copy(&hdevice),
                    core::mem::transmute_copy(&fsavestate),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseDeviceHandle: CloseDeviceHandle::<Identity, OFFSET>,
            GetVideoService: GetVideoService::<Identity, OFFSET>,
            LockDevice: LockDevice::<Identity, OFFSET>,
            OpenDeviceHandle: OpenDeviceHandle::<Identity, OFFSET>,
            ResetDevice: ResetDevice::<Identity, OFFSET>,
            TestDevice: TestDevice::<Identity, OFFSET>,
            UnlockDevice: UnlockDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDXGIDeviceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFDXGIDeviceManager {}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
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
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}
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
