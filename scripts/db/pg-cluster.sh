#!/usr/bin/env bash
# https://postgresql-cluster.org/docs

# https://pelle.io/posts/hetzner-rds-postgres/

docker run -d --name pg-console \
  --publish 80:80 \
  --publish 8080:8080 \
  --env PG_CONSOLE_API_URL=http://localhost:8080/api/v1 \
  --env PG_CONSOLE_AUTHORIZATION_TOKEN=secret_token \
  --env PG_CONSOLE_DOCKER_IMAGE=vitabaks/postgresql_cluster:latest \
  --volume console_postgres:/var/lib/postgresql \
  --volume /var/run/docker.sock:/var/run/docker.sock \
  --volume /tmp/ansible:/tmp/ansible \
  --restart=unless-stopped \
  vitabaks/postgresql_cluster_console:2.0.0