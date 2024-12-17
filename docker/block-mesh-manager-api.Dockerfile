FROM ubuntu:22.04 AS base
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
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-api-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf block-mesh-manager-api-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/block-mesh-manager-api block-mesh-manager-api \
  && chmod +x block-mesh-manager-api
CMD ["/opt/block-mesh-manager-api"]