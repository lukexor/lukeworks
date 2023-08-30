# Builder
# -------------------------------------
FROM rust:slim-bullseye as builder

RUN apt-get update && apt-get install -y \
  wget \
  && rm -rf /var/lib/apt/lists/* \
  && wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && cp cargo-binstall /usr/local/cargo/bin \
  && cargo binstall cargo-leptos -y \
  && rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cargo leptos build -r
