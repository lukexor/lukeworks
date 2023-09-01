# Base
# -------------------------------------
# bullseye required due to cargo-leptos openssl binary dependency on
# libssl.so.1.1
FROM rust:bullseye as base
WORKDIR /app

RUN curl -LO https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && mv cargo-binstall /usr/local/cargo/bin
RUN curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
  && chmod u+x tailwindcss-linux-x64 \
  && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy
RUN cargo binstall --locked -y --install-path /usr/local/cargo/bin \
  cargo-chef \
  cargo-leptos \
  wasm-opt
