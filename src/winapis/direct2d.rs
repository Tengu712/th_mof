use std::ops::Mul;
use windows::{
    core::Interface,
    Foundation::Numerics::*,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct2D::{Common::*, *},
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
            Imaging::*,
        },
        System::{Com::*, SystemServices::*},
    },
};

/// Struct to reference Direct2D objects.
pub struct D2DApplication {
    context: ID2D1DeviceContext,
    swapchain: IDXGISwapChain1,
}

/// Struct to reference image data.
pub struct Image {
    bitmap: ID2D1Bitmap,
    pub width: u32,
    pub height: u32,
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
            .map_err(|e| e.to_string() + "\nFailed to create D2D1CreateFactory.")?;
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
        Ok(Self { context, swapchain })
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
    pub fn clear_screen(&self, r: f32, g: f32, b: f32) {
        unsafe { self.context.Clear(&D2D1_COLOR_F { r, g, b, a: 1.0 }) };
    }

    /// Set matrix to reverse.
    pub fn reverse(&self, onoff: bool, screen_width: f32) {
        let mut mat_scl = Matrix3x2::identity();
        let mut mat_trs = Matrix3x2::identity();
        if onoff {
            mat_scl.M11 = -1.0;
            mat_trs.M31 = screen_width;
        }
        unsafe { self.context.SetTransform(&mat_scl.mul(mat_trs)) };
    }

    /// Draw Image.
    pub fn draw_image(
        &self,
        image: &Image,
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        uv_left: f32,
        uv_top: f32,
        uv_width: f32,
        uv_height: f32,
        alpha: f32,
        center: bool,
    ) {
        let (left, top) = if center {
            (left - width / 2.0, top - height / 2.0)
        } else {
            (left, top)
        };
        let dst_rect = D2D_RECT_F {
            left,
            top,
            right: left + width,
            bottom: top + height,
        };
        let src_rect = D2D_RECT_F {
            left: uv_left,
            top: uv_top,
            right: uv_left + uv_width,
            bottom: uv_top + uv_height,
        };
        unsafe {
            self.context.DrawBitmap(
                &image.bitmap,
                &dst_rect,
                alpha,
                D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
                &src_rect,
            )
        };
    }

    /// Create image
    pub fn create_image_from_file(&self, filename: &str) -> Result<Image, String> {
        let factory: IWICImagingFactory = unsafe {
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_SERVER).map_err(|e| {
                e.to_string() + "\nFailed to create IWICImagingFactory. : " + filename
            })?
        };
        let decoder = unsafe {
            factory
                .CreateDecoderFromFilename(
                    filename,
                    std::ptr::null(),
                    GENERIC_READ,
                    WICDecodeMetadataCacheOnLoad,
                )
                .map_err(|e| e.to_string() + "\nFailed to create decoder. : " + filename)?
        };
        let frame = unsafe {
            decoder
                .GetFrame(0)
                .map_err(|e| e.to_string() + "\nFailed to get frame. : " + filename)?
        };
        let converter = unsafe {
            factory
                .CreateFormatConverter()
                .map_err(|e| e.to_string() + "\nFailed to create format converter. : " + filename)?
        };
        unsafe {
            converter
                .Initialize(
                    frame,
                    &GUID_WICPixelFormat32bppPBGRA,
                    WICBitmapDitherTypeNone,
                    None,
                    1.0,
                    WICBitmapPaletteTypeMedianCut,
                )
                .map_err(|e| e.to_string() + "\nFailed to initialize converter. : " + filename)?
        };
        let mut width = 0;
        let mut height = 0;
        unsafe {
            converter
                .GetSize(&mut width, &mut height)
                .map_err(|e| e.to_string() + "\nFailed to get image size. : " + filename)?
        };
        let bitmap = unsafe {
            self.context
                .CreateBitmapFromWicBitmap(converter, std::ptr::null())
                .map_err(|e| e.to_string() + "\nFailed to create bitmap. : " + filename)?
        };
        Ok(Image {
            bitmap,
            width,
            height,
        })
    }
}
