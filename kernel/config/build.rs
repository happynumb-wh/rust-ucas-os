fn main() {
    println!("cargo:rerun-if-env-changed=UC_CONFIG_PATH");
    if let Ok(config_path) = std::env::var("UC_CONFIG_PATH") {
        println!("cargo:rerun-if-changed={config_path}");
    }
}
