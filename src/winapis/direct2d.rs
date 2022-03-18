use windows::{
    core::Interface, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*,
    Win32::System::Com::*,
};

pub struct D2DApplication {
    target: ID2D1HwndRenderTarget,
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
        // Create render target
        let target = unsafe {
            factory
                .CreateHwndRenderTarget(
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
                .map_err(|e| e.to_string())?
        };
        // Finish
        Ok(D2DApplication { target })
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
