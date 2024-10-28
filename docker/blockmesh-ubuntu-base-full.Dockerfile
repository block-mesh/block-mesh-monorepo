FROM --platform=$BUILDPLATFORM ubuntu:22.04 AS build
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
RUN cargo install cargo-leptos --features no_downloads --version=$LEPTOS_VERSION
RUN cargo install sqlx-cli --version=$SQLX_VERSION --no-default-features --features postgres,rustls
RUN cargo install wasm-pack --version=$WASM_PACK
RUN cargo install bunyan
#RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install --locked cargo-zigbuild
RUN apt-get install nodejs npm -y
RUN npm install -g sass
RUN cargo install cross
RUN apt-get install ca-certificates curl
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
RUN chmod a+r /etc/apt/keyrings/docker.asc
# Add the repository to Apt sources:
RUN echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null
RUN apt-get update
RUN apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
RUN rustup target add aarch64-unknown-linux-musl
RUN rustup component add rust-std --target x86_64-unknown-linux-gnu
RUN rustup component add rust-src --target x86_64-unknown-linux-gnu
RUN rustup component add rust-std --target aarch64-unknown-linux-gnu
RUN rustup component add rust-src --target aarch64-unknown-linux-gnu
RUN rustup component add rust-std --target aarch64-unknown-linux-musl
RUN rustup component add rust-src --target aarch64-unknown-linux-musl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
RUN export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")" && \
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && nvm install 18.17.1 && npm install -g sass
#FROM build AS run
#WORKDIR /code
#EXPOSE 8000
#COPY ./file.sh /
#RUN chmod 755 /file.sh
#CMD ["/bin/bash", "-c", "$COMMAND"]
#ENTRYPOINT [ "/code/file.sh" ]
