use std::env;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut icon_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
        icon_path.push("app_icon.ico");

        println!("cargo:rerun-if-changed=app_icon.ico");

        winresource::WindowsResource::new()
            .set_icon(icon_path.to_str().unwrap())
            .compile()
            .unwrap();
    }
}