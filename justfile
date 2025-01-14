alias b := build
alias t := test

build:
    cargo build --release

lint:
    cargo fmt -- --check
    cargo clippy

test: build
    cargo test
    cargo run --release -- examples/*
