#!/bin/bash
#docker buildx build --platform linux/amd64 -t bmesh -f block-mesh-manager.Dockerfile --load .
docker run \
-t blockmesh/block-mesh-manager:latest-amd64 \
-e APP_ENVIRONMENT=local \
-e REDIS_URL="redis://127.0.0.1:6379" \
-e DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh" \
-e MAILGUN_SEND_KEY="" \
--platform linux/amd64 -t bmesh