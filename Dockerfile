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

# No `--split`: nothing is `#[lazy]` yet, so there is nothing to split out, and
# whether a split bundle routes correctly in the browser is still untested. See
# the open question at the end of MIGRATION.md.
RUN cargo leptos build --release

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
