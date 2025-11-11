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
dev:
  @cargo leptos watch --split

# Run release server
run:
  @cargo leptos serve --release --split

# Run audit
audit:
  @cargo deny check

# Run tests
test:
  @cargo leptos test
  @cargo nextest run

# Build artifacts
build:
  @cargo leptos build --release --split
