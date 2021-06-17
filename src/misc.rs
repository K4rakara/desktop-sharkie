#[cfg(platform_windows)] use winapi;

#[derive(Debug, Copy, Clone)]
pub enum UserEvent {
    Move,
    Frame,
}

pub fn fatal<A, B>(line_1: A, line_2: B) -> !
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    #[cfg(platform_windows)] {
        // Allocate and format a string in safe rust.
        let formatted = format!("{}\n{}\0",
            line_1.as_ref(),
            line_2.as_ref());
        
        // ... Then, pass it down to unsafe rust as a pointer.
        unsafe {
            use winapi::um::winuser::*;
            
            MessageBoxA(
                std::ptr::null_mut(),
                formatted.as_str().as_bytes().as_ptr() as *const i8,
                b"Desktop Sharkie - Fatal error\0".as_ptr() as *const i8,
                MB_ICONERROR);
        }
    } #[cfg(any(platform_unix, platform_macos))] {
        println!("\n{}\n\x1b[1m{}\x1b[0m\n",
            line_1.as_ref(),
            line_2.as_ref());
    }

    std::process::exit(1);
}


