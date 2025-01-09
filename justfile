_default:
    just --choose

dist-clean-wasm-build-run: dist-clean-wasm-build
    simple-http-server dist

cargo-run-try-conversion:
    cargo run --bin try_conversion

wasm-build-debug:
    wasm-pack build --debug --target web --out-dir ./dist/pkg/

wasm-build-release:
    wasm-pack build --release --target web --out-dir ./dist/pkg/

dist-clean:
    /bin/rm -rf ./dist/

dist-clean-wasm-build: dist-clean
    mkdir -p ./dist/
    cp -R www/* dist
    just wasm-build-debug

dist-clean-wasm-build-release: dist-clean
    mkdir -p ./dist/
    cp -R www/* dist
    just wasm-build-release

watch:
    cargo watch -w src -s "just dist-clean-wasm-build-run"

serve-dev: wasm-build-debug
    python -m http.server 8000 -d dist

gh-deploy: dist-clean-wasm-build-release
    npx gh-pages -d dist

test:
    cargo test
