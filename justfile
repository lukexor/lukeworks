VERSION := `awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml`

CARGO := `command -v cargo 2> /dev/null`
CARGO_WATCH := `command -v cargo-watch 2> /dev/null`

default:
  @just --list

get-version:
  @echo {{VERSION}}

clean:
  cargo clean
  rm -rf .vercel/output

build:
  cargo build --release
  cargo leptos build --release
  mkdir -p .vercel/output/static .vercel/output/functions
  cp -f target/site/pkg/* .vercel/output/static/
  cp -f .vc-config.json .vercel/output/functions/
  cp -f target/release/handler .vercel/output/functions/

run:
  cargo leptos serve --release

test:
  cargo leptos test

dev:
  cargo leptos watch

