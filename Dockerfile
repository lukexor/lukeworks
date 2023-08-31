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

ENV SCCACHE_DIR=/tmp/sccache
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=$SCCACHE_DIR cargo chef cook --clippy --all-features
RUN --mount=type=cache,target=$SCCACHE_DIR cargo chef cook --all-targets
RUN --mount=type=cache,target=$SCCACHE_DIR cargo chef cook --release \
  --target-dir=target/server \
  --no-default-features \
  --features ssr
RUN --mount=type=cache,target=$SCCACHE_DIR cargo chef cook --release \
  --target-dir=target/front \
  --target=wasm32-unknown-unknown \
  --no-default-features \
  --features hydrate

COPY . .

RUN --mount=type=cache,target=$SCCACHE_DIR cargo clippy --no-deps --all-features -- -D warnings
RUN --mount=type=cache,target=$SCCACHE_DIR cargo leptos test -v
RUN --mount=type=cache,target=$SCCACHE_DIR cargo leptos build -v --release
RUN sccache --show-stats

# Runtime
# -------------------------------------
FROM gcr.io/distroless/cc AS runtime

ENV RUST_LOG="info"
USER nobody

WORKDIR /app
# Server binary
COPY --from=builder --chown=nobody:nobody /app/target/server/release/lukeworks /app
# Static assets
COPY --from=builder --chown=nobody:nobody /app/target/site /app/site

EXPOSE 3000

CMD ["/app/lukeworks"]
