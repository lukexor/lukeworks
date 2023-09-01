version := `awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml`

check-cargo:
  @if ! command -v cargo >/dev/null 2>&1; then \
    echo "\033[31m\`cargo\` is not available please install cargo (https://www.rust-lang.org/tools/install)\033[0m"; \
    exit 1; \
  fi

check-cargo-leptos: check-cargo
  @if ! cargo leptos --version >/dev/null 2>&1; then \
    echo "\033[31m\`cargo-leptos\` is not available please install cargo-leptos (https://crates.io/crates/cargo-leptos)\033[0m"; \
    exit 1; \
  fi

check-docker:
  @if ! command -v docker >/dev/null 2>&1; then \
    echo "\033[31m\`docker\` is not available please install docker (https://docs.docker.com/engine/install/)\033[0m"; \
    exit 1; \
  fi

default:
  @just --list

version:
  @echo {{version}}

clean: check-cargo
  cargo clean -p lukeworks

lint: check-cargo
  cargo clippy

image: check-docker
  sudo docker build --progress plain -t lukeworks-builder -f  builder.Dockerfile .
  sudo docker build --progress plain -t lukeworks:{{version}} .
  sudo docker system prune -f

build: check-cargo-leptos
  cargo leptos build --release

run: check-cargo-leptos
  cargo leptos serve --release

test: check-cargo-leptos
  cargo leptos test

dev: check-cargo-leptos
  cargo leptos watch
