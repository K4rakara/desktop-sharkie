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
        
        // Set the cfg flag.
        println!("cargo:rustc-cfg=platform_windows");
        
        // Add windows resources.
        embed_resource::compile("src/resource.rc");
    } else if env("CARGO_CFG_TARGET_OS") == "macos" {
        // Set the cfg flag.
        println!("cargo:rustc-cfg=platform_macos");
    } else {
        // Exit; other platforms aren't supported in the slightest.
        eprintln!("Unsupported platform!");
        std::process::exit(1);
    }
}

