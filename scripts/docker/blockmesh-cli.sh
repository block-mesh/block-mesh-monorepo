#!/bin/bash
docker buildx build --platform linux/arm64 -t bmesh -f docker/blockmesh-cli.Dockerfile --load .
# docker run --platform linux/arm64 --entrypoint /bin/bash -it -t bmesh
