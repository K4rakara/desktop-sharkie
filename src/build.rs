#[cfg(target_family = "windows")] use winres;

fn main() {
    #[cfg(all(
        target_family = "unix",
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))
    ))] {
        println!("cargo:rustc-cfg=platform_unix");
    } #[cfg(target_family = "windows")] {
        println!("cargo:rustc-cfg=platform_windows");
        
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/assets/icon.ico")
            .set("ProductName", "Desktop Sharkie")
            .set("FileDescription", "Desktop Sharkie")
            .set("LegalCopyright", "Copyleft © 2021")
            .set("OriginalFilename", "desktop-sharkie.exe")
            .compile()
            .unwrap();
    } #[cfg(target_os = "macos")] {
        println!("cargo:rustc-cfg=platform_macos");
    } #[cfg(any(target_os = "ios", target_os = "android"))] {
        std::process::exit(1);
    }
}

