# The deployed image: the server binary, the site directory it serves, and the
# hash file that names the fingerprinted assets inside it.
#
# The build stage carries a Rust toolchain, cargo-leptos and mold; the runtime
# stage carries none of them. `ldd` on the binary wants only libc, libm and
# libgcc, so a slim Debian is enough and there is nothing to install into it.

FROM rust:1.91-bookworm AS builder

# `.cargo/config.toml` links with mold on Linux, so every link fails without it.
RUN apt-get update \
  && apt-get install -y --no-install-recommends mold \
  && rm -rf /var/lib/apt/lists/*

# cargo-leptos drives the two-target build and runs tailwind. binstall fetches a
# prebuilt one; building it from source here would be several minutes of the
# image build spent on a tool.
RUN curl -fsSLO https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && tar -xf cargo-binstall-x86_64-unknown-linux-musl.tgz -C "$CARGO_HOME/bin" \
  && rm cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && cargo binstall --locked -y cargo-leptos

WORKDIR /build
COPY . .

# Installs 1.91.1 and the wasm32 target named by `rust-toolchain.toml` as their
# own layer, so a toolchain download is not repeated on every source change.
RUN rustup show

# Fingerprints the JS, WASM and CSS filenames and writes the `hash.txt` the
# server reads to resolve them. `RootStylesheet` switches on the same flag, and
# `cache_control_middleware` serves hashed names as immutable, so a release
# built without this serves assets that browsers must revalidate.
ENV LEPTOS_HASH_FILES=true

# `--split` is required, not a size flag. The `#[lazy_route]` pages compile to a
# module import of `__wasm_split_placeholder__`, which only a split build
# rewrites; without it hydration dies on `Failed to resolve module specifier`
# and the app is inert. What it buys is 48KB gzipped off the landing page.
#
# `LEPTOS_HASH_FILES` above is load-bearing here: the split glue and every chunk
# keep one name per build otherwise, and a cached glue paired with a fresh
# bundle fails to instantiate.
RUN cargo leptos build --release --split

FROM debian:bookworm-slim AS runtime

# The binary resolves `LEPTOS_SITE_ROOT` and `hash.txt` against the working
# directory, so both live beside it here.
WORKDIR /app

COPY --from=builder /build/target/release/lukeworks ./lukeworks
COPY --from=builder /build/target/release/hash.txt ./hash.txt
COPY --from=builder /build/target/site ./site

ENV LEPTOS_SITE_ROOT=site \
  LEPTOS_SITE_ADDR=0.0.0.0:8080 \
  LEPTOS_ENV=PROD \
  LEPTOS_HASH_FILES=true \
  RUST_LOG=info

# Nothing here writes to disk or binds a privileged port.
USER nobody

EXPOSE 8080
CMD ["./lukeworks"]
