#!/bin/bash
docker buildx build --platform linux/arm64 -t bmesh -f docker/blockmesh-cli.Dockerfile --load .
# docker run --platform linux/arm64 --entrypoint /bin/bash -it -t bmesh
# docker run --platform linux/arm64 --entrypoint /bin/bash -it -t blockmesh/blockmesh-cli:latest

function run() {
  #--label com.centurylinklabs.watchtower.enable=true
  # docker run -d  \
  docker run \
  -e EMAIL=${EMAIL} -e PASSWORD=${PASSWORD} \
  --restart=unless-stopped blockmesh/blockmesh-cli:latest-arm64
}
