use winapi;
use crate::misc;

use std::env;
use std::mem::{ transmute as tm, zeroed };
use std::ptr::null_mut;
use winapi::um::shellapi::*;
use winapi::um::winuser::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::*;
use misc::fatal;

static mut MENU_OPEN: bool = false;

struct Tray;

impl Tray {
    const INTERACTION: UINT = WM_APP + 1;

    const BUTTON_ABOUT: UINT = WM_APP + 2;
    const BUTTON_CLOSE_MENU: UINT = WM_APP + 3;
    const BUTTON_CLOSE_GURA: UINT = WM_APP + 4;

    unsafe fn hinstance() -> HINSTANCE {
        GetModuleHandleA(null_mut())
    }

    unsafe fn hwnd(hinstance: HINSTANCE) -> HWND {
        // Create a dummy window class with a custom callback.
        let dummy_window_class = (&mut {
            let mut dummy_window_class: WNDCLASSA = zeroed();
            
            dummy_window_class.hInstance = tm(hinstance);
            dummy_window_class.lpfnWndProc = Some(__raw_callback__);
            dummy_window_class.lpszClassName = "dummy\0".as_ptr() as *mut i8;

            dummy_window_class
        }) as *mut WNDCLASSA;

        // Register the dummy window class.
        RegisterClassA(dummy_window_class);

        // Spawn the dummy window.
        let hwnd = CreateWindowExA(
            0,
            "dummy\0".as_ptr() as *mut i8,
            "dummy\0".as_ptr() as *mut i8,
            0,
            0, 0,
            10, 10,
            null_mut() as _,
            null_mut() as _,
            tm(hinstance),
            null_mut() as _);

        if hwnd.is_null() {
            fatal("Unexpected runtime error.", "Failed to create a dummy window.");
        }

        hwnd
    }

    unsafe fn callback(
        hwnd: HWND,
        umsg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match umsg {
            WM_CREATE => {
                let data = (&mut {
                    let mut data: NOTIFYICONDATAA = zeroed();
                    
                    data.uID = 1;
                    data.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;

                    // Associate the tray icon with this window.
                    data.hWnd = tm(hwnd);

                    // Set the message that Windows will send when our tray
                    // icon is interacted with.
                    data.uCallbackMessage = Tray::INTERACTION;
                    
                    // Extract the programs icon for use in the tray.
                    ExtractIconExA(
                        env::current_exe()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .as_ptr() as *mut i8,
                        0,
                        null_mut(),
                        &mut data.hIcon as *mut HICON,
                        1);

                    // Instead of using a `*mut i8` like the other APIs,
                    // this one uses `[i8; 128]`, so we need to copy our
                    // `&'static str` to an `[i8; 128]` to pass it.
                    //
                    // Gotta love the consistency of Windows :^)
                    data.szTip = {
                        let mut sz_tip: [i8; 128] = [0; 128];
                        "Desktop Sharkie\0"
                            .bytes()
                            .zip(sz_tip.iter_mut())
                            .for_each(|(byte, ptr)| *ptr = tm(byte));
                        sz_tip
                    };
                    
                    data
                }) as *mut NOTIFYICONDATAA;

                Shell_NotifyIconA(NIM_ADD, data);
            },
            WM_CLOSE => {
                let data = (&mut {
                    let mut data: NOTIFYICONDATAA = zeroed();
                    data.uID = 1;
                    data.hWnd = tm(hwnd);
                    data
                }) as *mut NOTIFYICONDATAA;

                Shell_NotifyIconA(NIM_DELETE, data);

                std::process::exit(0);
            },
            WM_COMMAND => match wparam as UINT {
                Tray::BUTTON_ABOUT => {
                    todo!();
                },
                Tray::BUTTON_CLOSE_MENU => {
                    MENU_OPEN = false;
                    SetFocus(null_mut());
                },
                Tray::BUTTON_CLOSE_GURA => {
                    SendMessageA(hwnd, WM_CLOSE, 0, 0);
                },
                _ => (),
            },
            Tray::INTERACTION => match lparam as UINT {
                WM_RBUTTONUP => {
                    if MENU_OPEN {
                        return (&0 as *const i32) as LRESULT;
                    }

                    let hmenu = {
                        let hmenu = CreatePopupMenu();
                        /*InsertMenuA(
                            hmenu,
                            0,
                            MF_BYPOSITION | MF_STRING,
                            Tray::BUTTON_ABOUT as usize,
                            "About...\0".as_ptr() as *mut i8);*/
                        InsertMenuA(
                            hmenu,
                            1,
                            MF_BYPOSITION | MF_STRING,
                            Tray::BUTTON_CLOSE_MENU as usize,
                            "Close this menu\0".as_ptr() as *mut i8);
                        InsertMenuA(
                            hmenu,
                            2,
                            MF_BYPOSITION | MF_STRING,
                            Tray::BUTTON_CLOSE_GURA as usize,
                            "Close Goomba Roomba\0".as_ptr() as *mut i8);
                        hmenu
                    };

                    MENU_OPEN = true;

                    let cursor_pos = {
                        let mut cursor_pos: POINT = zeroed();
                        GetCursorPos(&mut cursor_pos as *mut POINT);
                        cursor_pos
                    };

                    TrackPopupMenu(
                        hmenu,
                        TPM_LEFTALIGN,
                        cursor_pos.x,
                        cursor_pos.y,
                        0,
                        hwnd,
                        null_mut());

                    DestroyMenu(hmenu);
                },
                _ => (),
            },
            _ => return DefWindowProcA(hwnd, umsg, wparam, lparam),
        }
        (&0 as *const i32) as LRESULT
    }
}

pub fn configure_tray() {
    unsafe {
        let hinstance = Tray::hinstance();
        Tray::hwnd(hinstance);
    }
}

unsafe extern "system" fn __raw_callback__(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    Tray::callback(hwnd, umsg, wparam, lparam)
}
