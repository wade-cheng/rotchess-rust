# list available recipes
default:
    just --list --unsorted

# build for the web (into docs/)
[group('build')]
web:
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/rotchess-rust.wasm docs

# serve whatever's been built into docs/
[group('run')]
serve:
    (cd docs && python -m http.server)

# locally run with fastest iteration speed
[group('run')]
run:
    cargo run