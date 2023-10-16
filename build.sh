set -eu

main() {
    pushd emscripten
    git apply ../emscripten.patch
    npm i
    ./embuilder --force build libc
    ./embuilder --force build libstandalonewasm
    git reset --hard
    popd

    cargo clean
    cargo build --target=wasm32-unknown-unknown
    wasm-bindgen --out-dir . target/wasm32-unknown-unknown/debug/wasm_bindgen_bug.wasm
}

main