FROM blockmesh/blockmesh-ubuntu-base:latest-arm64
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /opt/

RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/feature-flags-server-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf feature-flags-server-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/release/feature-flags-server feature-flags-server \
  && chmod +x feature-flags-server

CMD ["/opt/feature-flags-server"]