use raw_window_handle;
use glium;
use winapi;
pub mod tray;

use std::mem::transmute as tm;
use raw_window_handle::*;
use glium::Display;
use winapi::um::winuser::*;

pub use tray::configure_tray;

/// Sets the attributes of the window associated with `display`.
/// 
/// In particular, this ensures that:
/// - The window is transparent.
/// - The window is not shown in the taskbar.
/// - The window does not receive user input -- it instead passes down to
///   whatever window is behind it.
pub fn configure_window(display: &Display) {
    match display.gl_window().window().raw_window_handle() {
        RawWindowHandle::Windows(window) => unsafe {
            // For whatever reason, changes to `WS_EX_TOOLWINDOW` and
            // `WS_EX_APPWINDOW` require the window to be hidden and reshown to
            // take effect.
            // 
            // So, hide the window.
            ShowWindow(tm(window.hwnd), SW_HIDE);               
            
            // Transmutation of `window.hwnd` is done because `GetWindowLongA`
            // expects a `*mut c_void`.
            // 
            // Transmutation of the return value is done because the `WS_EX_*`
            // constants are exposed as `u32`s, while `GetWindowLongA` returns
            // `i32`. We don't want a numeric cast, as that might change what
            // bits are set, so instead we transmute.
            let mut current_style: u32 = tm(GetWindowLongA(tm(window.hwnd), GWL_EXSTYLE));
            
            // Make the window transparent.
            current_style |= WS_EX_COMPOSITED;
            current_style |= WS_EX_TRANSPARENT;
            current_style |= WS_EX_LAYERED;

            // Pass user input to this window to the windows below it.
            current_style |= WS_EX_NOACTIVATE;

            // Hide the window from the taskbar.
            current_style |= WS_EX_TOOLWINDOW;
            current_style &= !WS_EX_APPWINDOW;
            
            // Again, `window.hwnd` is transmuted because this api expects a 
            // `*mut c_void`.
            // 
            // `current_style` is transmuted back to `i32` from a `u32`, for 
            // the same reasons as above.
            SetWindowLongA(tm(window.hwnd), GWL_EXSTYLE, tm(current_style));
            
            // Reshow the window, since we hid it earlier.
            ShowWindow(tm(window.hwnd), SW_SHOW);
        },
        _ => unreachable!(),
    }
}
