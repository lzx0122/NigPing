fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=/MANIFESTUAC:level='requireAdministrator'");
        println!("cargo:rustc-link-arg=/MANIFESTUAC:uiAccess='false'");
    }
    tauri_build::build()
}
