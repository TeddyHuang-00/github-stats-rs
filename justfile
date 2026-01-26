set dotenv-load := true

format:
    cargo +nightly fmt --all
    cargo sort --workspace
    cargo sort-derives
    maudfmt src/
    just --fmt --unstable

check: format
    typos **/*.rs
    cargo check --all --all-targets --workspace
    cargo clippy --all-targets --all-features

fix: format && format
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

run:
    cargo run --release
