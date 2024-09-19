#!/bin/bash
docker buildx build --platform linux/amd64 -t block-mesh-manager-worker -f block-mesh-manager-worker.Dockerfile --load .
docker run \
--network=blockmesh_network \
-e APP_ENVIRONMENT=local \
-e REDIS_URL="redis://127.0.0.1:6379" \
-e DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh" \
-e MAILGUN_SEND_KEY="" \
--platform linux/amd64 -t block-mesh-manager-worker