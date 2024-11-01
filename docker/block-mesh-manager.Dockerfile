FROM --platform=$BUILDPLATFORM blockmesh/blockmesh-ubuntu-base:latest AS build
ARG DEBIAN_FRONTEND=noninteractive
ARG LEPTOS_HASH_FILES=true
ARG LEPTOS_HASH_FILE_NAME=hash.txt
WORKDIR /opt/
RUN git clone --depth 1 https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/block-mesh-manager/* .
RUN if [ "$BUILDPLATFORM" = "linux/amd64" ]; then \
      curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
      && tar -xzvf block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz; \
    else \
      curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-aarch64-unknown-linux-gnu.tar.gz \
      && tar -xzvf block-mesh-manager-aarch64-unknown-linux-gnu.tar.gz; \
    fi
RUN mv target/* . \
  && mv release/hash.txt . \
  && mv release/block-mesh-manager block-mesh-manager \
  && chmod +x block-mesh-manager
CMD ["/opt/block-mesh-manager"]