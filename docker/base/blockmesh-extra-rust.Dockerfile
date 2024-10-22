FROM blockmesh/blockmesh-ubuntu-base AS base
ARG DEBIAN_FRONTEND=noninteractive
ARG SQLX_VERSION=0.7.3
ARG LEPTOS_VERSION=0.2.20
ARG RUSTC_VERSION=1.77.0
ARG WASM_PACK=0.12.1
RUN cargo install cargo-leptos --version=$LEPTOS_VERSION
RUN cargo install sqlx-cli --version=$SQLX_VERSION --no-default-features --features postgres,rustls
RUN cargo install wasm-pack --version=$WASM_PACK
RUN cargo install bunyan