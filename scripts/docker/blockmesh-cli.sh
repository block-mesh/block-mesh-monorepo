#!/bin/bash
docker buildx build --platform linux/arm64 -t bmesh -f blockmesh-cli.Dockerfile --load .