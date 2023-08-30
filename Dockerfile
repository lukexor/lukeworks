# Builder
# -------------------------------------
# bullseye required due to cargo-leptos openssl binary dependency on
# libssl.so.1.1
FROM builder:0.1.0 AS builder

WORKDIR /app
COPY . .

RUN cargo leptos build -r

# Final
# -------------------------------------
FROM gcr.io/distroless/cc

ENV RUST_LOG="info"

WORKDIR /app
# Server binary
COPY --from=builder /app/target/server/release/lukeworks /app
# Static assets
COPY --from=builder /app/target/site /app/site
# COPY --from=builder /app/Cargo.toml /app/

EXPOSE 8080
USER nobody

CMD ["/app/lukeworks"]
