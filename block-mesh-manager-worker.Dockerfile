FROM ubuntu:22.04 AS builder
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /opt/
RUN apt-get update
RUN apt-get install curl gzip git-all -y

RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-worker-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf block-mesh-manager-worker-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/block-mesh-manager-worker block-mesh-manager-worker \
  && chmod +x block-mesh-manager-worker

CMD ["/opt/block-mesh-manager-worker"]