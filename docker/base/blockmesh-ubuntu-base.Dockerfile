FROM --platform=$BUILDPLATFORM ubuntu:22.04
ARG TARGETPLATFORM
ARG DEBIAN_FRONTEND=noninteractive
ARG SQLX_VERSION=0.7.3
ARG LEPTOS_VERSION=0.2.20
ARG RUSTC_VERSION=1.77.0
ARG WASM_PACK=0.12.1
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
RUN apt-get install -y --no-install-recommends openssl ca-certificates
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu
