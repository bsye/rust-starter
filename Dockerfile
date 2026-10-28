ARG RUST_VERSION=1.99
FROM rust:${RUST_VERSION}-trixie AS base
ARG BINSTALL_VERSION=1.26.1
ARG DIOXUS_CLI_VERSION=0.7.10
RUN rustup target add wasm32-unknown-unknown \
    && rustup component add rustfmt clippy
RUN curl -fsSL "https://github.com/cargo-bins/cargo-binstall/releases/download/v${BINSTALL_VERSION}/cargo-binstall-$(uname -m)-unknown-linux-musl.tgz" \
    | tar -xz -C /usr/local/cargo/bin \
    && cargo binstall --no-confirm "dioxus-cli@${DIOXUS_CLI_VERSION}"
WORKDIR /app

FROM base AS dev
EXPOSE 8080
CMD ["dx", "serve", "--addr", "0.0.0.0", "--port", "8080"]
