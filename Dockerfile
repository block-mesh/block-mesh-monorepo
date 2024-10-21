FROM blockmesh/blockmesh-ubuntu-base:latest-amd64 AS base
ARG DEBIAN_FRONTEND=noninteractive
#FROM ubuntu:latest
#RUN apt-get update
#RUN apt-get install curl gzip git-all -y

COPY . /code
WORKDIR /code

FROM base AS copy

RUN cargo fetch
#RUN --mount=type=bind,source=.,target=/code
RUN cargo build --features hydrate,ssr

#RUN cargo install sqlx-cli --no-default-features -F postgres,rustls
#RUN sqlx migrate run --source libs/block-mesh-manager/migrations --ignore-missing --database-url="postgresql://postgres:password@localhost:5559/block-mesh"

#FROM copy AS development
#
#EXPOSE 8000
#
#CMD ["cargo", "run", "-p", "block-mesh-manager"]
#
#FROM copy AS production
#EXPOSE 8000
#CMD ["cargo", "run", "-p", "block-mesh-manager", "--release"]