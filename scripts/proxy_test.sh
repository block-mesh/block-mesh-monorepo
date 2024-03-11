#!/usr/bin/env bash
curl \
  --proxy-header 'api_token: 123' \
  -x \
  "127.0.0.1:3000" \
  "https://example.com"
