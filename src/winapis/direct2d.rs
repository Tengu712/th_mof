use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*,
    Win32::System::Com::*, Win32::System::LibraryLoader::*, Win32::System::Performance::*,
    Win32::System::SystemInformation::GetLocalTime, Win32::UI::Animation::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct D2DApplication {
    factory: ID2D1Factory1,
    target: ID2D1HwndRenderTarget,
}

impl D2DApplication {
    /// Create D2DApplication struct that is only way to use Direct2D.
    pub fn new(winapp: &super::winapi::WindowsApplication) -> Result<Self> {
        unsafe { CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)? };
        // Create factory
        let factory: ID2D1Factory1 = {
            let mut result = None;
            unsafe {
                D2D1CreateFactory(
                    D2D1_FACTORY_TYPE_SINGLE_THREADED,
                    &ID2D1Factory1::IID,
                    &D2D1_FACTORY_OPTIONS::default(),
                    std::mem::transmute(&mut result),
                )
                .map(|()| result.unwrap())
            }
        }?;
        // Create render target
        let target = unsafe {
            factory.CreateHwndRenderTarget(
                &D2D1_RENDER_TARGET_PROPERTIES::default(),
                &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd: winapp.hwnd,
                    pixelSize: D2D_SIZE_U {
                        width: winapp.width,
                        height: winapp.height,
                    },
                    presentOptions: D2D1_PRESENT_OPTIONS_NONE,
                },
            )
        }?;
        // Finish
        Ok(D2DApplication { factory, target })
    }

    /// Clear screen black.
    pub fn clear_screen(&self) {
        unsafe {
            self.target.BeginDraw();
            self.target.Clear(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            });
            self.target
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
                .unwrap();
        }
    }
}
