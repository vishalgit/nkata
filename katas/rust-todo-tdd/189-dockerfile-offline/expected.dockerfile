# syntax=docker/dockerfile:1

# ---------- build (as uid 1000) ----------
FROM rust:latest AS builder
RUN groupadd --gid 1000 vishal \
    && useradd --uid 1000 --gid 1000 --create-home vishal \
    && chown -R 1000:1000 /usr/local/cargo /usr/local/rustup
USER 1000:1000
RUN rustup target add wasm32-unknown-unknown x86_64-unknown-linux-musl \
    && curl -L --proto '=https' --tlsv1.2 -sSf \
       https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall -y --locked cargo-leptos
WORKDIR /home/vishal/app
COPY --chown=1000:1000 . .
ENV SQLX_OFFLINE=true \
    LEPTOS_BIN_TARGET_TRIPLE=x86_64-unknown-linux-musl
RUN cargo leptos build --release

# ---------- run: no distro at all ----------
FROM scratch
WORKDIR /app
COPY --from=builder --chown=1000:1000 /home/vishal/app/target/x86_64-unknown-linux-musl/release/server /app/server
COPY --from=builder --chown=1000:1000 /home/vishal/app/target/site /app/site
ENV RUST_LOG=info \
    LEPTOS_OUTPUT_NAME=todo \
    LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:8080 \
    LEPTOS_RELOAD_PORT=3001 \
    LEPTOS_ENV=PROD
ENV LOG_FORMAT=json \
    METRICS_ADDR=0.0.0.0:9000
EXPOSE 9000
USER 1000:1000
EXPOSE 8080
ENTRYPOINT ["/app/server"]
