# list available recipes
default:
    @just --list --unsorted

# build for the web (into docs/)
[group('build')]
web:
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/rotchess-rust.wasm docs

# build workspace docs
[group('build')]
doc:
    cargo doc --workspace --no-deps

# build workspace docs with private items
[group('build')]
doc-private:
    cargo doc --workspace --no-deps --document-private-items

# build all docs
[group('build')]
doc-all:
    cargo doc

# serve whatever's been built into docs/
[group('run')]
serve:
    (cd docs && python -m http.server)

# locally run with fastest iteration speed
[group('run')]
run:
    cargo run