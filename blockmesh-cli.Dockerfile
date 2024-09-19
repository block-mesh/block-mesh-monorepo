FROM blockmesh/blockmesh-ubuntu-base:latest-amd64
ARG DEBIAN_FRONTEND=noninteractive
ARG LEPTOS_HASH_FILES=true
ARG LEPTOS_HASH_FILE_NAME=hash.txt
WORKDIR /opt/

RUN curl -sLO https://github.com/block-mesh/block-mesh-monorepo/releases/latest/download/blockmesh-cli-aarch64-unknown-linux-gnu.tar.gz \
  && tar -xvf blockmesh-cli-aarch64-unknown-linux-gnu.tar.gz \
  && mv target/aarch64-unknown-linux-gnu/release/blockmesh-cli blockmesh-cli \
  && chmod +x blockmesh-cli
RUN echo '#!/bin/bash\n\
exec /opt/blockmesh-cli --email "$EMAIL" --password "$PASSWORD"' > /usr/local/bin/entrypoint.sh && \
chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]