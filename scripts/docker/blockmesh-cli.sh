#!/bin/bash
export ROOT="$(git rev-parse --show-toplevel)"
docker buildx build --platform linux/arm64 -t bmesh-cli-arm64 -f docker/blockmesh-cli.Dockerfile --load .
docker buildx build --platform linux/amd64 -t bmesh-cli-amd64 -f docker/blockmesh-cli.Dockerfile --load .

# docker run --platform linux/arm64 --entrypoint /bin/bash -it -t bmesh-cli-arm64
# docker run --platform linux/amd64 --entrypoint /bin/bash -it -t bmesh-cli-amd64
# docker run --platform linux/arm64 --entrypoint /bin/bash -it -t blockmesh/blockmesh-cli:latest

function run() {
  #--label com.centurylinklabs.watchtower.enable=true
  # docker run -d  \
  docker run \
  -e EMAIL=${EMAIL} -e PASSWORD=${PASSWORD} \
  --restart=unless-stopped blockmesh/blockmesh-cli:latest-arm64

  docker run -e EMAIL=${EMAIL} -e PASSWORD=${PASSWORD} blockmesh/blockmesh-cli:latest
}


function release() {
  docker buildx build --platform linux/amd64,linux/arm64 -t blockmesh/blockmesh-cli -f "${ROOT}/docker/blockmesh-cli.Dockerfile" --push .
}