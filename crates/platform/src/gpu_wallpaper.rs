//! Windows Graphics Capture reads the compositor's GPU surface, including DX11 scene
//! wallpapers which cannot paint into a PrintWindow DC. Targets a validated WPE window
//! or its clean Explorer host, never a monitor or a foreground application.
use crate::{bindings::HWND, capture_bindings as b, live_wallpaper::CapturedWallpaper};
use windows_core::{Interface, Result};

windows_core::link!("kernel32.dll" "system" fn GetModuleHandleExW(flags: u32,
    name: windows_core::PCWSTR, module: *mut crate::bindings::HMODULE) -> windows_core::BOOL);

fn pin_capture_runtime() -> Result<()> {
    // Windows 11 24H2 GraphicsCapture.dll can unload at the last worker apartment exit
    // while its internal asynchronous callback is still returning (WER reports an AV
    // in GraphicsCapture.dll_unloaded). Pin the activated runtime until process exit.
    // Sessions/pools are still explicitly closed; this retains code, not capture objects.
    let name = crate::wide::to_wide("GraphicsCapture.dll");
    let mut module = crate::bindings::HMODULE::default();
    unsafe {
        if !GetModuleHandleExW(1, windows_core::PCWSTR(name.as_ptr()), &mut module).as_bool() {
            return Err(windows_core::Error::from_thread());
        }
    }
    Ok(())
}

pub(crate) struct GpuSurface {
    pool: b::Direct3D11CaptureFramePool,
    session: b::GraphicsCaptureSession,
    device: b::ID3D11Device,
    context: b::ID3D11DeviceContext,
    staging: Option<b::ID3D11Texture2D>,
    dimensions: (u32, u32),
    last: Option<Vec<u8>>,
}

impl GpuSurface {
    pub(crate) fn new(hwnd: HWND) -> Result<Self> {
        let interop: b::IGraphicsCaptureItemInterop =
            windows_core::factory::<b::GraphicsCaptureItem, b::IGraphicsCaptureItemInterop>()?;
        pin_capture_runtime()?;
        // SAFETY: caller validated this WPE window or clean desktop host. No monitor capture.
        let item: b::GraphicsCaptureItem = unsafe { interop.CreateForWindow(b::HWND(hwnd.0))? };
        let size = item.Size()?;
        let (mut device, mut context) = (None, None);
        unsafe {
            b::D3D11CreateDevice(
                None,
                b::D3D_DRIVER_TYPE_HARDWARE,
                b::HMODULE::default(),
                b::D3D11_CREATE_DEVICE_BGRA_SUPPORT as u32,
                Some(&[b::D3D_FEATURE_LEVEL_11_0]),
                b::D3D11_SDK_VERSION as u32,
                Some(&mut device),
                None,
                Some(&mut context),
            )
            .ok()?;
        }
        let device = device.unwrap();
        let context = context.unwrap();
        let dxgi: b::IDXGIDevice = device.cast()?;
        let winrt: b::IDirect3DDevice =
            unsafe { b::CreateDirect3D11DeviceFromDXGIDevice(&dxgi)?.cast()? };
        let pool = b::Direct3D11CaptureFramePool::CreateFreeThreaded(
            &winrt,
            b::DirectXPixelFormat::B8G8R8A8UIntNormalized,
            2,
            size,
        )?;
        let session = pool.CreateCaptureSession(&item)?;
        session.SetIsCursorCaptureEnabled(false)?;
        session.SetIsBorderRequired(false)?;
        session.StartCapture()?;
        Ok(Self {
            pool,
            session,
            device,
            context,
            staging: None,
            dimensions: (size.Width as u32, size.Height as u32),
            last: None,
        })
    }

    pub(crate) fn capture(&mut self, left: i32, top: i32) -> Result<Option<CapturedWallpaper>> {
        // Drain the bounded pool and use its newest frame. Paused wallpapers retain their
        // last valid GPU frame; absence of a new frame is not a capture failure.
        let mut newest: Option<b::Direct3D11CaptureFrame> = None;
        for _ in 0..2 {
            let Ok(frame) = self.pool.TryGetNextFrame() else {
                break;
            };
            if let Some(old) = newest.replace(frame) {
                let _ = old.cast::<b::IClosable>()?.Close();
            }
        }
        // Old queued frames are never displayed. Read back only the newest texture,
        // avoiding a second full-screen GPU stall and copy after a late wake-up.
        if let Some(frame) = newest {
            let result = self.read_frame(&frame);
            let _ = frame.cast::<b::IClosable>()?.Close();
            result?;
        }
        Ok(self.last.as_ref().map(|pixels| CapturedWallpaper {
            left,
            top,
            width: self.dimensions.0,
            height: self.dimensions.1,
            bgra: pixels.clone(),
        }))
    }

    fn read_frame(&mut self, frame: &b::Direct3D11CaptureFrame) -> Result<()> {
        let access: b::IDirect3DDxgiInterfaceAccess = frame.Surface()?.cast()?;
        let texture: b::ID3D11Texture2D = unsafe { access.GetInterface()? };
        let mut desc = b::D3D11_TEXTURE2D_DESC::default();
        unsafe { texture.GetDesc(&mut desc) };
        let size = frame.ContentSize()?;
        if size.Width <= 0
            || size.Height <= 0
            || size.Width as u32 > desc.Width
            || size.Height as u32 > desc.Height
            || u64::from(desc.Width) * u64::from(desc.Height) > 33_177_600
        {
            return Ok(());
        }
        let dimensions = (desc.Width, desc.Height);
        if self.staging.is_none() || self.dimensions != dimensions {
            desc.Usage = b::D3D11_USAGE_STAGING;
            desc.BindFlags = 0;
            desc.CPUAccessFlags = b::D3D11_CPU_ACCESS_READ as u32;
            desc.MiscFlags = 0;
            let mut staging = None;
            unsafe {
                self.device
                    .CreateTexture2D(&desc, None, Some(&mut staging))
                    .ok()?
            };
            self.staging = staging;
            self.dimensions = dimensions;
        }
        let staging = self.staging.as_ref().unwrap();
        let mut mapped = b::D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            self.context.CopyResource(staging, &texture);
            self.context
                .Map(staging, 0, b::D3D11_MAP_READ, 0, Some(&mut mapped))
                .ok()?;
        }
        let row_bytes = dimensions.0 as usize * 4;
        let mut pixels = vec![0u8; row_bytes * dimensions.1 as usize];
        // SAFETY: Map succeeded; each row is within the staging texture. Unmap before
        // closing the frame, even for a black frame (a valid scene transition).
        unsafe {
            for y in 0..dimensions.1 as usize {
                std::ptr::copy_nonoverlapping(
                    (mapped.pData as *const u8).add(y * mapped.RowPitch as usize),
                    pixels.as_mut_ptr().add(y * row_bytes),
                    row_bytes,
                );
            }
            self.context.Unmap(staging, 0);
        }
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel[3] = 255;
        }
        self.last = Some(pixels);
        Ok(())
    }
}

impl Drop for GpuSurface {
    fn drop(&mut self) {
        if let Ok(closable) = self.session.cast::<b::IClosable>() {
            let _ = closable.Close();
        }
        if let Ok(closable) = self.pool.cast::<b::IClosable>() {
            let _ = closable.Close();
        }
    }
}
