_default:
    just --choose

build-run:
    wasm-pack build --target web
    simple-http-server .

cargo-run-try-conversion:
    cargo run --bin try_conversion
