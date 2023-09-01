# Planner
# -------------------------------------
FROM lukeworks-builder AS planner
WORKDIR /app

COPY . .
RUN cargo chef prepare

# Builder
# -------------------------------------
FROM lukeworks-builder AS builder
WORKDIR /app

ENV LEPTOS_OUTPUT_NAME=lukeworks
ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=127.0.0.1:3000
ENV LEPTOS_RELOAD_PORT=3001

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook \
  --target-dir target/front --target wasm32-unknown-unknown \
  --no-default-features --features hydrate \
  --profile wasm-release
RUN cargo chef cook \
  --bin lukeworks \
  --target-dir target/server \
  --no-default-features --features ssr \
  --release

COPY . .
RUN cargo leptos build -vv --release

# Runtime
# -------------------------------------
FROM gcr.io/distroless/cc AS runtime
WORKDIR /app

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

# Server binary
COPY --from=builder --chown=nobody:nobody /app/target/server/release/lukeworks /app
# Static assets
COPY --from=builder --chown=nobody:nobody /app/target/site /app/site
COPY --from=builder --chown=nobody:nobody /app/Cargo.toml /app/

EXPOSE 3000
USER nobody

CMD ["/app/lukeworks"]
