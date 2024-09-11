FROM ubuntu:22.04 AS builder
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /opt/
RUN apt-get update
RUN apt-get install curl gzip git-all -y

RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/feature-flags-server-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf feature-flags-server-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/feature-flags-server feature-flags-server \
  && chmod +x feature-flags-server

CMD ["/opt/feature-flags-server"]