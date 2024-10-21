FROM blockmesh/blockmesh-ubuntu-base:latest-amd64 AS base
ARG DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /code
COPY . /code
RUN cargo fetch

FROM base AS development
EXPOSE 8000
CMD ["cargo", "run", "-p", "block-mesh-manager"]

FROM base AS production
EXPOSE 8000
CMD ["cargo", "run", "-p", "block-mesh-manager", "--release"]