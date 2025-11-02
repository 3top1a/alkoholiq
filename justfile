alias b := build
alias t := test

build:
    cargo build --release

lint:
    cargo fmt -- --check
    cargo clippy

test:
    cargo test
    cargo build --release
    for x in examples/lir/*; do cargo run --release < $x; done

ci: test

coverage:
    cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml
