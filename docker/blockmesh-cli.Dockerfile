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
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/blockmesh-cli-x86_64-unknown-linux-gnu.tar.gz \
      && tar -xvf blockmesh-cli-x86_64-unknown-linux-gnu.tar.gz \
      && mv target/x86_64-unknown-linux-gnu/release/blockmesh-cli blockmesh-cli-amd64 \
      && chmod +x blockmesh-cli-amd64
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/blockmesh-cli-aarch64-unknown-linux-gnu.tar.gz \
      && tar -xvf blockmesh-cli-aarch64-unknown-linux-gnu.tar.gz \
      && mv target/aarch64-unknown-linux-gnu/release/blockmesh-cli blockmesh-cli-arm64 \
      && chmod +x blockmesh-cli-arm64
RUN echo '#!/bin/bash\n\
set -x\n\
if [ "$(arch)" = "x86_64" ] ; then\n\
exec /opt/blockmesh-cli-amd64 --email "$EMAIL" --password "$PASSWORD"\n\
else\n\
exec /opt/blockmesh-cli-arm64 --email "$EMAIL" --password "$PASSWORD"\n\
fi\n'  > /usr/local/bin/entrypoint.sh && \
chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]