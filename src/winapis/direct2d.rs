use windows::{
    core::Interface,
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::{
        Direct2D::*,
        Direct3D::*,
        Direct3D11::*,
        Dxgi::{Common::*, *},
    },
    Win32::System::Com::*,
};

/// Struct to reference Direct2D objects
pub struct D2DApplication {
    context: ID2D1DeviceContext,
    swapchain: IDXGISwapChain1,
}

impl D2DApplication {
    /// Create D2DApplication struct that is only way to use Direct2D.
    pub fn new(winapp: &super::winapi::WindowsApplication) -> Result<Self, String> {
        unsafe {
            CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED).map_err(|e| e.to_string())?
        };
        // Create factory
        let factory: ID2D1Factory1 = unsafe {
            let mut ppifactory = None;
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &ID2D1Factory1::IID,
                &D2D1_FACTORY_OPTIONS::default(),
                std::mem::transmute(&mut ppifactory),
            )
            .map_err(|e| e.to_string())?;
            ppifactory.ok_or("The Option of D2D1CreateFactry is None.")?
        };
        // Create D3D11Device
        let d3d11device = unsafe {
            let mut ppdevice = None;
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HINSTANCE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                &D3D_FEATURE_LEVEL_11_1,
                1,
                D3D11_SDK_VERSION,
                &mut ppdevice,
                std::ptr::null_mut(),
                &mut None,
            )
            .map_err(|e| e.to_string() + "\nFailed to create D3D11Device.")?;
            ppdevice.ok_or("Failed to create D3D11Device.")?
        };
        // Create device context
        let context = unsafe {
            let dxdevice = d3d11device
                .cast::<IDXGIDevice>()
                .map_err(|e| e.to_string() + "\nFailed to cast D3D11Device to IDXGIDevice.")?;
            factory
                .CreateDevice(dxdevice)
                .map_err(|e| e.to_string() + "\nFailed to create device ID2D1Device.")?
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)
                .map_err(|e| e.to_string() + "\nFailed to create device context.")?
        };
        unsafe { context.SetUnitMode(D2D1_UNIT_MODE_DIPS) };
        // Create swapchain
        let swapchain = unsafe {
            let dxdevice = d3d11device
                .cast::<IDXGIDevice>()
                .map_err(|e| e.to_string() + "\nFailed to cast D3D11Device to IDXGIDevice.")?;
            let dxfactory: IDXGIFactory2 = dxdevice
                .GetAdapter()
                .map_err(|e| e.to_string() + "\nFailed to get IDXGIAdapter.")?
                .GetParent()
                .map_err(|e| e.to_string() + "\nFailed to IDXGIAdapter::GetParent().")?;
            let pdesc = DXGI_SWAP_CHAIN_DESC1 {
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
                ..Default::default()
            };
            dxfactory
                .CreateSwapChainForHwnd(d3d11device, winapp.hwnd, &pdesc, std::ptr::null(), None)
                .map_err(|e| e.to_string() + "\nFailed to create swapchain.")?
        };
        // Get dpi
        let mut dpi = 0.0;
        let mut dpiy = 0.0;
        unsafe { factory.GetDesktopDpi(&mut dpi, &mut dpiy) };
        // Create bitmap
        let bitmap = unsafe {
            let backbuffer: IDXGISurface = swapchain
                .GetBuffer(0)
                .map_err(|e| e.to_string() + "\nFailed to get backbuffer.")?;
            let bitmapproperties = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: dpi,
                dpiY: dpi,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                colorContext: None,
            };
            context
                .CreateBitmapFromDxgiSurface(backbuffer, &bitmapproperties)
                .map_err(|e| e.to_string() + "\nFailed to create bitmap from dxgisurface.")?
        };
        unsafe { context.SetTarget(bitmap) };
        // Finish
        Ok(D2DApplication { context, swapchain })
    }

    /// Call this at the first of drawing
    pub fn begin_draw(&self) {
        unsafe { self.context.BeginDraw() };
    }

    /// Call this at the end of drawing
    pub fn end_draw(&self) -> Result<(), String> {
        unsafe {
            self.context
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
                .map_err(|e| e.to_string() + "\nFailed to end draw.")?
        };
        Ok(())
    }

    /// Wait VSync
    pub fn present(&self, sync: u32, flag: u32) -> Result<(), String> {
        unsafe {
            self.swapchain
                .Present(sync, flag)
                .map_err(|e| e.to_string() + "\nFailed to wait vsync.")?
        };
        Ok(())
    }

    /// Clear screen black.
    pub fn clear_screen(&self) {}
}
