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
# `--split` is required, not an optimisation: the routes in `FlatRoutes` marked
# `#[lazy_route]` compile to a module import of `__wasm_split_placeholder__`,
# which a build without it never rewrites. Hydration then dies on
# `Failed to resolve module specifier` and the whole app stays inert.
dev:
  @cargo leptos watch --split

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
