dev:
    live-server docs/ & cargo watch -i docs/ -x run

ci: test fmt clippy # audit # coverage

all: audit test fmt clippy coverage unused outdated 

audit:
  cargo deny check advisories

test:
  cargo test --all-features

fmt:
  cargo fmt --all -- --check

clippy:
  cargo clippy -- -D warnings

coverage:
  cargo tarpaulin --ignore-tests --doc

coverage-report:
    cargo tarpaulin --ignore-tests --doc --out html

unused:
  cargo +nightly udeps --workspace

outdated:
  cargo outdated --root-deps-only --workspace

fmtwrite:
  cargo fmt