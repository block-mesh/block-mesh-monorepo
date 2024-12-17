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
RUN git clone --depth 1 https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/emails/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/emails-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf emails-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/emails emails \
  && chmod +x emails
CMD ["/opt/emails"]