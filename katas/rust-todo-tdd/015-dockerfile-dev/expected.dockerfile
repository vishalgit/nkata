# syntax=docker/dockerfile:1
FROM rust:latest

ARG USERNAME=vishal
ARG UID=1000
ARG GID=1000

# The only root step: create your user and give it the toolchain directories.
RUN groupadd --gid ${GID} ${USERNAME} \
    && useradd --uid ${UID} --gid ${GID} --create-home --shell /bin/bash ${USERNAME} \
    && mkdir -p /workspace /home/${USERNAME}/.cache \
    && chown -R ${UID}:${GID} /usr/local/cargo /usr/local/rustup /workspace /home/${USERNAME}

# Everything below — every install and every compile — runs as uid 1000.
USER ${UID}:${GID}
ENV LANG=C.UTF-8
WORKDIR /workspace

RUN rustup component add rustfmt clippy \
    && rustup target add wasm32-unknown-unknown x86_64-unknown-linux-musl

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall -y --locked cargo-leptos bacon \
    && cargo install --locked sqlx-cli --no-default-features --features rustls,postgres
