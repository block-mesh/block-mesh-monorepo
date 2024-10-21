FROM rust:buster AS base
#FROM ubuntu:latest
#ARG DEBIAN_FRONTEND=noninteractive
#RUN apt-get update
#RUN apt-get install curl gzip git-all -y

COPY . /code
WORKDIR /code
#RUN --mount=type=bind,source=.,target=/code

#RUN cargo build --features hydrate,ssr
CMD ["bash"]
#RUN cargo fetch
#RUN cargo install sqlx-cli --no-default-features -F postgres,rustls
#RUN sqlx migrate run --source libs/block-mesh-manager/migrations --ignore-missing --database-url="postgresql://postgres:password@localhost:5559/block-mesh"

#FROM base AS development
#
#EXPOSE 8000
#
#CMD ["cargo", "run", "-p", "block-mesh-manager"]
#
#FROM base AS production
#EXPOSE 8000
#CMD ["cargo", "run", "-p", "block-mesh-manager", "--release"]