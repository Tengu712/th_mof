use windows::{
    core::PCSTR, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

/// Show a message box.
pub fn show_messagebox(message: String, title: String) {
    unsafe { MessageBoxW(None, message, title, MB_OK | MB_ICONERROR) };
}

/// Show a yes-no message box. The answer is yes, it returns true.
pub fn ask_yesno(message: String, title: String) -> bool {
    unsafe { MessageBoxW(None, message, title, MB_YESNO) == IDNO }
}

/// Private. Window procedure.
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

/// Struct to reference basic objects
pub struct WindowsApplication {
    pub hwnd: HWND,
    pub width: u32,
    pub height: u32,
    pub windowed: bool,
}

impl WindowsApplication {
    /// Create WindowsApplication struct that is only way to use WindowsAPI.
    pub fn new(title: &str, width: u32, height: u32, windowed: bool) -> Result<Self, String> {
        let (window_style, window_show) = if windowed {
            (WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX, SW_SHOW)
        } else {
            (WS_POPUP, SW_SHOWMAXIMIZED)
        };
        // Get instance handle
        let instance = unsafe { GetModuleHandleA(None) };
        if instance.0 == 0 {
            return Err("Failed to get instance handle.".to_owned());
        }
        // Register window class
        let wcex = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance,
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW) },
            lpszClassName: PCSTR(b"RustWindowClass\0".as_ptr()),
            ..Default::default()
        };
        if unsafe { RegisterClassExA(&wcex) == 0 } {
            return Err("Failed to register window class.".to_owned());
        }
        // Adjust window size
        let mut window_rect = RECT {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };
        unsafe { AdjustWindowRect(&mut window_rect, window_style, false) };
        // Create window and get window handle
        let hwnd = unsafe {
            CreateWindowExA(
                Default::default(),
                "RustWindowClass",
                title,
                window_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                window_rect.right - window_rect.left,
                window_rect.bottom - window_rect.top,
                None,
                None,
                instance,
                std::ptr::null(),
            )
        };
        if hwnd.is_invalid() {
            return Err("Failed to create window.".to_owned());
        }
        unsafe { ShowWindow(hwnd, window_show) };
        // Finish
        Ok(WindowsApplication {
            hwnd,
            width,
            height,
            windowed,
        })
    }

    /// Process all window events.
    /// If return value is true, window has closed. Otherwise, it is deadtime.
    pub fn do_event(&self) -> bool {
        let mut message = MSG::default();
        loop {
            if unsafe { PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() } {
                if message.message == WM_QUIT {
                    return true;
                }
                unsafe {
                    TranslateMessage(&message);
                    DispatchMessageA(&message);
                }
                continue;
            }
            break;
        }
        false
    }
}
