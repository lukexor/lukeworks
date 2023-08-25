VERSION := `awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml`

CARGO := `command -v cargo 2> /dev/null`
CARGO_WATCH := `command -v cargo-watch 2> /dev/null`

default:
  @just --list

get-version:
  @echo {{VERSION}}

clean:
  cargo clean

build:
  cargo leptos build -r

run:
  cargo leptos serve -r

test:
  cargo leptos test

dev:
  cargo leptos watch

