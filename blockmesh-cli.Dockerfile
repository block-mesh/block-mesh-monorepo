FROM ubuntu:22.04 AS builder
ARG DEBIAN_FRONTEND=noninteractive
ARG LEPTOS_HASH_FILES=true
ARG LEPTOS_HASH_FILE_NAME=hash.txt
WORKDIR /opt/
RUN apt-get update
RUN apt-get install curl gzip git-all -y

RUN git clone https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/block-mesh-manager/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/blockmesh-cli-aarch64-unknown-linux-gnu.tar.gz \
  && tar -xvf block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/aarch64-unknown-linux-gnu/release/blockmesh-cli blockmesh-cli \
  && chmod +x block-mesh-manager
RUN echo '#!/bin/bash\n\
exec /opt/local/bin/blockmesh-cli --email "$EMAIL" --password "$PASSWORD"' > /usr/local/bin/entrypoint.sh && \
chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]