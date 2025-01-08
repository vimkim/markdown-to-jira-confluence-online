_default:
    just --choose

build-run:
    wasm-pack build --target web --out-dir ./dist/pkg/
    simple-http-server dist

cargo-run-try-conversion:
    cargo run --bin try_conversion

wasm-build:
    wasm-pack build --target web --out-dir ./dist/pkg/

dist-clean:
    /bin/rm -rf ./dist/

dist-clean-wasm-build: dist-clean
    mkdir -p ./dist/
    cp -R www/* dist
    just wasm-build

serve-dev: wasm-build
    python -m http.server 8000 -d dist

gh-deploy: dist-clean-wasm-build
    npx gh-pages -d dist

test:
    cargo test
