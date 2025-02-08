alias b := build
alias t := test

build:
    cargo build --release

lint:
    cargo fmt -- --check
    cargo clippy

test: build
    cargo test
    for x in examples/*; do cargo run --release -- $x; done

ci: test

coverage:
    cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml
