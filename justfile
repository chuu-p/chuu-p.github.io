dev:
    live-server docs/ & cargo watch -i docs/ -x run

test:
    cargo test

coverage:
    cargo tarpaulin --doc

coverage-report:
    cargo tarpaulin --doc --out html
