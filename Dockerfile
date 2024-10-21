FROM blockmesh/blockmesh-ubuntu-base:latest-amd64 AS base
ARG DEBIAN_FRONTEND=noninteractive
ARG SCCACHE_BUCKET=blockmesh-sccache
ARG SCCACHE_ENDPOINT=https://66f69a5836ae4963ba7e97414a2c17d9.r2.cloudflarestorage.com
ARG SCCACHE_REGION=auto
ARG AWS_ACCESS_KEY_ID=91ec8f700f4f652cee43bd1d46753fb5
ARG AWS_SECRET_ACCESS_KEY=ee229e8956ae7d8c752c42ffcff2f14a997f46707d3ed30cd677bc10a1b51ea7
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /code
FROM base AS planner
COPY . /code
RUN echo 'rustc-wrapper = "sccache"' >> .cargo/config.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /code/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
FROM builder AS copy
RUN cargo fetch

FROM copy AS prebuild
RUN cargo build -p block-mesh-manager --features hydrate,ssr

FROM prebuild AS migrate
#RUN sqlx migrate run --source libs/block-mesh-manager/migrations --ignore-missing --database-url="postgresql://postgres:password@localhost:5559/block-mesh"

FROM migrate AS development
EXPOSE 8000
CMD ["cargo", "run", "-p", "block-mesh-manager"]

FROM migrate AS production
EXPOSE 8000
CMD ["cargo", "run", "-p", "block-mesh-manager", "--release"]