FROM ubuntu:22.04 AS builder
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /opt/
RUN apt-get update
RUN apt-get install curl gzip git-all -y

RUN git clone https://github.com/block-mesh/block-mesh-monorepo.git
RUN cp -fr block-mesh-monorepo/libs/block-mesh-manager/* .
RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
  && tar -xvf block-mesh-manager-x86_64-unknown-linux-gnu.tar.gz \
  && mv target/* . \
  && mv release/hash.txt . \
  && mv release/block-mesh-manager block-mesh-manager \
  && chmod +x block-mesh-manager

CMD ["/opt/block-mesh-manager"]