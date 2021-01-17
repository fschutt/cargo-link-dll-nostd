fn main() {
    // dynamically link azul.dll - fails because there is no azul.lib file, only azul.dll.lib
    println!("cargo:rustc-link-search=dylib={}", concat!(env!("CARGO_MANIFEST_DIR"), "/../target/release"));
}