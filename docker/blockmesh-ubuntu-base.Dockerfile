FROM ubuntu:22.04
ARG DEBIAN_FRONTEND=noninteractive
ARG SQLX_VERSION=0.7.3
ARG LEPTOS_VERSION=0.2.20
ARG RUSTC_VERSION=1.77.0
RUN apt-get update
RUN apt-get install curl gzip git-all -y
RUN apt-get install build-essential -y
RUN apt-get install -y pkg-config openssl libssl-dev
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install $RUSTC_VERSION
RUN rustup default $RUSTC_VERSION
RUN rustup component add rustfmt
RUN rustup component add rustc
RUN rustup component add cargo
RUN rustup component add rust-std
RUN rustup component add rust-docs
RUN rustup component add rust-analyzer
RUN rustup component add clippy
RUN rustup component add rust-src
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --version=$LEPTOS_VERSION
RUN cargo install sqlx-cli --version=$SQLX_VERSION --no-default-features --features postgres,rustls
RUN cargo install cargo-chef
RUN cargo install sccache
