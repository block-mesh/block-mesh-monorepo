FROM rust:buster AS base

WORKDIR /code

VOLUME .
#COPY . /code

RUN cargo fetch
RUN cargo install sqlx-cli --no-default-features -F postgres,rustls
RUN sqlx migrate run --source libs/block-mesh-manager/migrations --ignore-missing --database-url="postgresql://postgres:password@localhost:5559/block-mesh"

FROM base AS development

EXPOSE 8000

CMD ["cargo", "run", "-p", "block-mesh-manager"]

FROM base AS production
EXPOSE 8000
CMD ["cargo", "run", "-p", "block-mesh-manager", "--release"]