_default:
    just --choose

build-run:
    wasm-pack build --target web
    simple-http-server .

cargo-run-try-conversion:
    cargo run --bin try_conversion

wasm-build:
    wasm-pack build --target web --out-dir ./dist/pkg/

dist-clean:
    /bin/rm -rf ./dist/

dist-clean-wasm-build: dist-clean
    mkdir -p ./dist/
    cp -R www/* dist/*
    just wasm-build

serve-dev: build
    python -m http.server 8000 -d dist

gh-deploy: build
    npx gh-pages -d dist

test:
    cargo test
