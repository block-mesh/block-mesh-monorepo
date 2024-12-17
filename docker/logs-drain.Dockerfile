FROM ubuntu:24.04 AS base
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG DEBIAN_FRONTEND=noninteractive
RUN echo "BUILDPLATFORM = $BUILDPLATFORM"
RUN echo "TARGETPLATFORM = $TARGETPLATFORM"
RUN apt-get update
RUN apt-get install curl gzip git-all -y
RUN apt-get install libc6 -y
FROM base AS build
WORKDIR /opt/
RUN git clone --depth 1 https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/logs-drain/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/logs-drain-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf logs-drain-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/logs-drain logs-drain \
  && chmod +x logs-drain
CMD ["/opt/logs-drain"]