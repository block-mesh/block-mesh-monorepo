FROM blockmesh/blockmesh-ubuntu-base:latest-amd64
ARG DEBIAN_FRONTEND=noninteractive
ARG LEPTOS_HASH_FILES=true
ARG LEPTOS_HASH_FILE_NAME=hash.txt
WORKDIR /opt/

RUN git clone --depth 1 https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/block-mesh-manager/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/* . \
  && mv release/hash.txt . \
  && mv release/block-mesh-manager block-mesh-manager \
  && chmod +x block-mesh-manager

CMD ["/opt/block-mesh-manager"]