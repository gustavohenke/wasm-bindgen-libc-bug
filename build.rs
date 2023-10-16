pub fn main() {
    println!("cargo:rustc-link-search=emscripten/cache/sysroot/lib/wasm32-unknown-unknown");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=standalonewasm");
}