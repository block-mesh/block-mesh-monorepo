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
RUN cp -fr block-mesh-monorepo/libs/block-mesh-manager/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
    && tar -xvf block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz
RUN mv target/* . \
  && mv release/hash.txt . \
  && mv release/block-mesh-manager block-mesh-manager \
  && chmod +x block-mesh-manager
CMD ["/opt/block-mesh-manager"]