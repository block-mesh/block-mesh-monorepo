#!/bin/bash
docker buildx build --platform linux/amd64 -t bmesh -f block-mesh-manager.Dockerfile --load .
docker run \
-e APP_ENVIRONMENT=local \
-e DATABASE_URL="postgres://postgres:password@localhost:5559/block-mesh" \
-e MAILGUN_SEND_KEY="" \
--platform linux/amd64 -t bmesh