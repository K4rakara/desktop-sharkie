
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
    #[cfg(any(platform_unix, platform_macos))] {
        println!("\n{}\n\x1b[1m{}\x1b[0m\n",
            line_1.as_ref(),
            line_2.as_ref());
    } #[cfg(platform_windows)] {
        let formatted = format!("{}\n{}\0",
            line_1.as_ref(),
            line_2.as_ref());
        
        unsafe {
            use winapi::um::winuser::*;
            
            MessageBoxA(
                std::ptr::null_mut(),
                formatted.as_str().as_bytes().as_ptr() as *const i8,
                b"Fatal error\0".as_ptr() as *const i8,
                MB_ICONERROR);
        }
    }
    std::process::exit(1);
}


