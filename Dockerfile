# Planner
# -------------------------------------
# bullseye required due to cargo-leptos openssl binary dependency on
# libssl.so.1.1
FROM rust:bullseye as planner
WORKDIR /app

RUN curl -LO https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && tar xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && mv cargo-binstall /usr/local/cargo/bin
RUN curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
  && chmod u+x tailwindcss-linux-x64 \
  && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy
RUN cargo binstall -y --locked --install-path /usr/local/cargo/bin \
  cargo-chef \
  cargo-leptos \
  wasm-opt

COPY . .
RUN cargo chef prepare

# Builder
# -------------------------------------
FROM planner AS builder
WORKDIR /app

COPY --from=planner /app/recipe.json .
RUN cargo chef cook \
  --target-dir target/front \
  --target wasm32-unknown-unknown \
  --no-default-features --features hydrate \
  --profile wasm-release
RUN cargo chef cook \
  --bin lukeworks \
  --target-dir target/server \
  --no-default-features --features ssr \
  --release
RUN strip target/server/release/lukeworks

COPY . .
RUN cargo leptos build -vv --release

# Runtime
# -------------------------------------
FROM gcr.io/distroless/cc:debug AS runtime
WORKDIR /app

ENV RUST_LOG "info,lukeworks=debug,tower_http=debug,axum::rejection=trace"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT "site"

# Server
COPY --from=builder --chown=nobody:nobody /app/target/server/release/lukeworks /app/Cargo.toml .
# Assets
COPY --from=builder --chown=nobody:nobody /app/target/site site

EXPOSE 3000
USER nobody

ENTRYPOINT ["/app/lukeworks"]
