FROM ubuntu:22.04 AS base
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG DEBIAN_FRONTEND=noninteractive
RUN echo "BUILDPLATFORM = $BUILDPLATFORM"
RUN echo "TARGETPLATFORM = $TARGETPLATFORM"
RUN apt-get update
RUN apt-get install curl gzip git-all -y
FROM base AS build
WORKDIR /opt/
RUN git clone --depth 1 https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/feature-flags-server/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/feature-flags-server-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf feature-flags-server-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/feature-flags-server feature-flags-server \
  && chmod +x feature-flags-server
CMD ["/opt/feature-flags-server"]