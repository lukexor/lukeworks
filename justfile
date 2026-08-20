set positional-arguments

# Cleaner output when using justfile - run cargo manually for backtraces
export RUST_BACKTRACE := env("RUST_BACKTRACE", "0")
export RUST_LOG := env("RUST_LOG", "debug")

CARGO_TARGET_DIR := env("CARGO_TARGET_DIR", "target")
VERSION := `grep '^version' Cargo.toml | sed -E 's/version = "(.*)"/\1/'`
GIT_SHA := `git rev-parse --short HEAD`

# List commands
default:
  @just --list

# Current version
version:
  @echo {{VERSION}}

# Install dev tools
install:
  @cargo install cargo-binstall
  @cargo bininstall --locked -y \
    just \
    cargo-deny \
    cargo-leptos \
    cargo-nextest

# Run development server
#
# No `--split` here. Under the watcher a rebuild fails in cargo-leptos with
# `Could not rename target/site/pkg/lukeworks_bg.wasm`, taking the server down
# on the first edit. Nothing is `#[lazy]` yet, so splitting buys nothing until
# the sketches land (Phase 6) and can be revisited then.
dev:
  @cargo leptos watch

# Run release server
run:
  @cargo leptos serve --release --split

# Run audit
audit:
  @cargo deny check

# Run tests
#
# `--features ssr` matters: the generated post table is compiled into the ssr
# build only, so without it the content tests run against an empty table.
test:
  @cargo leptos test
  @cargo nextest run --features ssr

# Build artifacts
build:
  @cargo leptos build --release --split
