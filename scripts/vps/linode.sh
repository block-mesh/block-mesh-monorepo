#!/usr/bin/env bash

# Deploy a new VPS on linode

curl -H "Content-Type: application/json" \
-H "Authorization: Bearer ${LINODE_ACCESS_TOKEN}" \
-X POST -d '{
    "authorized_users": [],
    "backups_enabled": false,
    "booted": true,
    "image": "linode/ubuntu22.04",
    "label": "ubuntu-us-southeast",
    "private_ip": false,
    "region": "us-southeast",
    "root_pass": "Fc8FJA78x6qdivzku2Ly",
    "tags": [],
    "type": "g6-nanode-1"
}' https://api.linode.com/v4/linode/instances