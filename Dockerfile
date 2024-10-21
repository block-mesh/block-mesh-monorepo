FROM blockmesh/blockmesh-ubuntu-base:latest-amd64 AS base
ARG DEBIAN_FRONTEND=noninteractive
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