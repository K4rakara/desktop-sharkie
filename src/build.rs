fn env(var: &'static str) -> String {
    std::env::var(var).unwrap_or("".to_owned())
}

fn main() {
    if env("CARGO_CFG_TARGET_FAMILY") == "unix"
    && env("CARGO_CFG_TARGET_OS") != "macos"
    && env("CARGO_CFG_TARGET_OS") != "android"
    && env("CARGO_CFG_TARGET_OS") != "ios" {
        // Set the cfg flag.
        println!("cargo:rustc-cfg=platform_unix");
    } else if env("CARGO_CFG_TARGET_FAMILY") == "windows" {
        #[cfg(not(target_family = "windows"))]
        println!("cargo:warning=Building for Windows on non-windows platforms \
                    is problematic. If you run into any issues, please try \
                    building in a windows VM.");
        
        println!("cargo:warning=Hi");

        // Set the cfg flag.
        println!("cargo:rustc-cfg=platform_windows");
        
        // Add windows resources.
        // This is the stuff that is shown when you right click an executable
        // and hit properties (iirc).
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/assets/icon.ico")
            .set("ProductName", "Desktop Sharkie")
            .set("FileDescription", "Desktop Sharkie")
            .set("LegalCopyright", "Copyleft © 2021")
            .set("OriginalFilename", "desktop-sharkie.exe")
            .compile()
            .unwrap();
    } else if env("CARGO_CFG_TARGET_OS") == "macos" {
        // Set the cfg flag.
        println!("cargo:rustc-cfg=platform_macos");
    } else {
        // Exit; other platforms aren't supported in the slightest.
        eprintln!("Unsupported platform!");
        std::process::exit(1);
    }
}

