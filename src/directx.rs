use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

/// 
pub fn ask_yn(message: &str, title: &str) -> bool {
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

pub struct DXApplication {}

impl DXApplication {
    /// Create DXApplication struct that is only way to use WindowsAPI.
    pub fn new(title: &str, width: i32, height: i32, windowed: bool) -> Self {
        let (window_style, window_show) = if windowed {
            (WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX, SW_SHOW)
        } else {
            (WS_POPUP, SW_SHOWMAXIMIZED)
        };
        // Get instance handle
        let instance = unsafe { GetModuleHandleA(None) };
        debug_assert!(instance.0 != 0);
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
        debug_assert!(unsafe { RegisterClassExA(&wcex) != 0 });
        // Adjust window size
        let mut window_rect = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
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
        debug_assert!(!hwnd.is_invalid());
        // Finish
        unsafe { ShowWindow(hwnd, window_show) };
        DXApplication {}
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
