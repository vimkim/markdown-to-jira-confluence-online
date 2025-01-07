_default:
    just --choose

build-run:
    wasm-pack build --target web
    simple-http-server .

cargo-run-try-conversion:
    cargo run --bin try_conversion

wasm-build:
    wasm-pack build --target web --out-dir ./public/pkg/

build:
    /bin/rm -rf ./public/
    mkdir -p ./public/
    cp ./index.html ./public/
    just wasm-build

serve-dev: build
    python -m http.server 8000 -d public

gh-deploy: build
    npx gh-pages -d public

test:
    cargo test
